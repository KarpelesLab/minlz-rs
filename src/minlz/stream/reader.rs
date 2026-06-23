// Copyright 2024 Karpeles Lab Inc.
// MinLZ stream reader. See ../mod.rs for licensing/attribution.

use super::{
    CHUNK_COMPRESSED, CHUNK_COMPRESSED_COMP_CRC, CHUNK_EOF, CHUNK_STREAM_ID, CHUNK_UNCOMPRESSED,
    MAX_NON_SKIPPABLE_CHUNK, MAX_USER_NON_SKIPPABLE, MIN_USER_NON_SKIPPABLE, STREAM_MAGIC,
};
use crate::crc::crc;
use crate::minlz::block::decompress_body;
use crate::minlz::MAX_BLOCK_SIZE;
use alloc::vec;
use alloc::vec::Vec;
use std::io::{self, Read};

fn corrupt(msg: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

/// A reader that decompresses data from the MinLZ stream format.
///
/// Implements [`std::io::Read`]; each decoded block is verified against its
/// masked CRC-32C. Concatenated streams (multiple stream identifiers) and
/// skippable chunks (padding, index, user-defined) are handled transparently.
pub struct Reader<R: Read> {
    r: R,
    decoded: Vec<u8>,
    pos: usize,
    max_block: usize,
    saw_header: bool,
    /// Uncompressed bytes decoded since the current stream identifier (for EOF
    /// length validation).
    uncompressed_in_stream: u64,
    /// Validate the EOF chunk's total-size field. Disabled when starting
    /// mid-stream (after a seek), where the count is partial.
    validate_eof: bool,
    done: bool,
}

impl<R: Read> Reader<R> {
    /// Create a reader over a MinLZ stream.
    pub fn new(r: R) -> Self {
        Reader {
            r,
            decoded: Vec::new(),
            pos: 0,
            max_block: MAX_BLOCK_SIZE,
            saw_header: false,
            uncompressed_in_stream: 0,
            validate_eof: true,
            done: false,
        }
    }

    /// Create a reader positioned at a block boundary mid-stream (used for
    /// seeking via an [`Index`](crate::minlz::Index)). The stream header has
    /// already been parsed elsewhere, so `max_block` is supplied and EOF size
    /// validation is skipped.
    pub(crate) fn new_midstream(r: R, max_block: usize) -> Self {
        Reader {
            r,
            decoded: Vec::new(),
            pos: 0,
            max_block,
            saw_header: true,
            uncompressed_in_stream: 0,
            validate_eof: false,
            done: false,
        }
    }

    /// Read a 4-byte chunk header, or `None` at a clean end of stream.
    fn read_chunk_header(&mut self) -> io::Result<Option<[u8; 4]>> {
        let mut h = [0u8; 4];
        let mut got = 0;
        while got < 4 {
            let n = self.r.read(&mut h[got..])?;
            if n == 0 {
                if got == 0 {
                    return Ok(None);
                }
                return Err(corrupt("truncated chunk header"));
            }
            got += n;
        }
        Ok(Some(h))
    }

    fn read_exact_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.r.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn skip(&mut self, mut len: usize) -> io::Result<()> {
        let mut scratch = [0u8; 4096];
        while len > 0 {
            let want = len.min(scratch.len());
            self.r.read_exact(&mut scratch[..want])?;
            len -= want;
        }
        Ok(())
    }

    fn read_stream_id(&mut self, clen: usize) -> io::Result<()> {
        // STREAM_MAGIC already consumed "ff 06 00 00"; the chunk body is the
        // 5-byte "MinLz" plus a 1-byte block-size indicator (length 6).
        if clen != 6 {
            return Err(corrupt("bad stream identifier length"));
        }
        let body = self.read_exact_vec(clen)?;
        if body[..5] != STREAM_MAGIC[4..] {
            return Err(corrupt("not a MinLZ stream (bad magic body)"));
        }
        let indicator = body[5] & 0x0f;
        if indicator > 13 {
            return Err(corrupt("block-size indicator out of range"));
        }
        self.max_block = (1usize << (indicator as u32 + 10)).min(MAX_BLOCK_SIZE);
        self.saw_header = true;
        self.uncompressed_in_stream = 0;
        Ok(())
    }

    fn set_block(&mut self, block: Vec<u8>) {
        self.uncompressed_in_stream += block.len() as u64;
        self.decoded = block;
        self.pos = 0;
    }

    /// Decode the next data block into `self.decoded`. Returns `Ok(false)` at
    /// the end of the stream.
    fn fill(&mut self) -> io::Result<bool> {
        loop {
            let hdr = match self.read_chunk_header()? {
                Some(h) => h,
                None => return Ok(false),
            };
            let ctype = hdr[0];
            let clen = hdr[1] as usize | (hdr[2] as usize) << 8 | (hdr[3] as usize) << 16;

            // Before a stream identifier, a (non-skippable) data chunk is
            // invalid — but skippable chunks (index, padding) and a lone EOF
            // (an empty stream, spec §4.6) are fine. The stream identifier and
            // skippable chunks are all > MAX_NON_SKIPPABLE_CHUNK.
            if !self.saw_header && ctype <= MAX_NON_SKIPPABLE_CHUNK && ctype != CHUNK_EOF {
                return Err(corrupt("data chunk before stream identifier"));
            }

            match ctype {
                CHUNK_STREAM_ID => self.read_stream_id(clen)?,
                CHUNK_COMPRESSED | CHUNK_COMPRESSED_COMP_CRC => {
                    if clen < 4 {
                        return Err(corrupt("compressed chunk too short"));
                    }
                    let buf = self.read_exact_vec(clen)?;
                    let checksum = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                    let body = &buf[4..];
                    let decoded = decompress_body(body, self.max_block)
                        .map_err(|e| corrupt(stream_err(e)))?;
                    let to_crc: &[u8] = if ctype == CHUNK_COMPRESSED_COMP_CRC {
                        body
                    } else {
                        &decoded
                    };
                    if crc(to_crc) != checksum {
                        return Err(corrupt("CRC mismatch"));
                    }
                    self.set_block(decoded);
                    return Ok(true);
                }
                CHUNK_UNCOMPRESSED => {
                    if clen < 4 {
                        return Err(corrupt("uncompressed chunk too short"));
                    }
                    let buf = self.read_exact_vec(clen)?;
                    let checksum = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                    let data = buf[4..].to_vec();
                    if data.len() > self.max_block {
                        return Err(corrupt("uncompressed block exceeds max block size"));
                    }
                    if crc(&data) != checksum {
                        return Err(corrupt("CRC mismatch"));
                    }
                    self.set_block(data);
                    return Ok(true);
                }
                CHUNK_EOF => {
                    if clen > 10 {
                        return Err(corrupt("oversized EOF chunk"));
                    }
                    if clen != 0 {
                        let buf = self.read_exact_vec(clen)?;
                        let (want, n) =
                            read_uvarint(&buf).ok_or_else(|| corrupt("bad EOF size"))?;
                        if n != clen {
                            return Err(corrupt("EOF length mismatch"));
                        }
                        if self.validate_eof && want != self.uncompressed_in_stream {
                            return Err(corrupt("EOF size does not match decoded length"));
                        }
                    }
                    // Allow a following concatenated stream; otherwise the next
                    // read_chunk_header returns None (clean end).
                    self.saw_header = false;
                }
                _ => {
                    if ctype <= MAX_NON_SKIPPABLE_CHUNK
                        || (MIN_USER_NON_SKIPPABLE..=MAX_USER_NON_SKIPPABLE).contains(&ctype)
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "unsupported non-skippable chunk",
                        ));
                    }
                    // Skippable (padding, index, reserved/user skippable): ignore.
                    self.skip(clen)?;
                }
            }
        }
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() {
            return Ok(0);
        }
        loop {
            if self.pos < self.decoded.len() {
                let n = (self.decoded.len() - self.pos).min(out.len());
                out[..n].copy_from_slice(&self.decoded[self.pos..self.pos + n]);
                self.pos += n;
                return Ok(n);
            }
            if self.done {
                return Ok(0);
            }
            if !self.fill()? {
                self.done = true;
                return Ok(0);
            }
        }
    }
}

/// Map a block-decode error to a stream-reader message.
fn stream_err(e: crate::error::Error) -> &'static str {
    use crate::error::Error::*;
    match e {
        TooLarge => "block exceeds max block size",
        Corrupt | CrcMismatch | BufferTooSmall | Unsupported | InvalidInput(_) => {
            "corrupt compressed block"
        }
    }
}

/// Minimal unsigned-LEB128 decode for the EOF size field.
fn read_uvarint(buf: &[u8]) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    for (i, &b) in buf.iter().enumerate() {
        if shift >= 64 {
            return None;
        }
        result |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return Some((result, i + 1));
        }
        shift += 7;
    }
    None
}
