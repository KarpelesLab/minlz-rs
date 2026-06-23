// Copyright 2024 Karpeles Lab Inc.
// MinLZ stream writer. See ../mod.rs for licensing/attribution.

use super::{
    CHUNK_COMPRESSED, CHUNK_EOF, CHUNK_UNCOMPRESSED, DEFAULT_BLOCK_SIZE, MAX_CHUNK_LEN,
    STREAM_MAGIC,
};
use crate::crc::crc;
use crate::minlz::block::compress_body;
use crate::minlz::index::Index;
use crate::minlz::Level;
use alloc::vec::Vec;
use std::io::{self, Write};

/// A writer that compresses data into the MinLZ stream format.
///
/// Data written is buffered into blocks (default 1 MiB); each block is
/// compressed (or stored, if incompressible) and framed with a masked CRC-32C.
/// Call [`finish`](Writer::finish) to flush the final block and write the
/// end-of-stream marker; [`Drop`] does the same on a best-effort basis.
///
/// ```
/// use std::io::Write;
/// let mut w = minlz::minlz::Writer::new(Vec::new());
/// w.write_all(b"hello hello hello hello hello").unwrap();
/// let compressed = w.finish().unwrap();
///
/// let mut out = Vec::new();
/// std::io::copy(&mut minlz::minlz::Reader::new(&compressed[..]), &mut out).unwrap();
/// assert_eq!(out, b"hello hello hello hello hello");
/// ```
pub struct Writer<W: Write> {
    /// `None` after `finish` has taken the inner writer.
    inner: Option<W>,
    level: Level,
    block_size: usize,
    ibuf: Vec<u8>,
    wrote_header: bool,
    uncompressed_written: u64,
    /// Compressed bytes written to the inner writer (for index offsets).
    compressed_written: u64,
    /// Index being built, if requested via [`Writer::with_index`].
    index: Option<Index>,
    finished: bool,
    /// Sticky error: once writing fails, every later call returns it.
    err: Option<io::ErrorKind>,
}

impl<W: Write> Writer<W> {
    /// Create a writer at the default ([`Level::Fastest`]) level.
    pub fn new(w: W) -> Self {
        Self::with_level(w, Level::default())
    }

    /// Create a writer at the given compression [`Level`].
    pub fn with_level(w: W, level: Level) -> Self {
        Writer {
            inner: Some(w),
            level,
            block_size: DEFAULT_BLOCK_SIZE,
            ibuf: Vec::with_capacity(DEFAULT_BLOCK_SIZE),
            wrote_header: false,
            uncompressed_written: 0,
            compressed_written: 0,
            index: None,
            finished: false,
            err: None,
        }
    }

    /// Enable building and appending a seek [`Index`](crate::minlz::Index) to
    /// the end of the stream (after the end-of-stream marker). The index is a
    /// skippable chunk, so plain readers ignore it.
    pub fn with_index(mut self) -> Self {
        self.index = Some(Index::new(self.block_size as u64));
        self
    }

    /// Write `b` to the inner writer, counting the bytes for index offsets.
    fn write_raw(&mut self, b: &[u8]) -> io::Result<()> {
        self.inner
            .as_mut()
            .ok_or_else(|| io::Error::other("minlz writer already finished"))?
            .write_all(b)?;
        self.compressed_written += b.len() as u64;
        Ok(())
    }

    fn header() -> [u8; 10] {
        // Block-size indicator = log2(block_size) - 10; for 1 MiB that is 10.
        let mut hdr = [0u8; 10];
        hdr[..STREAM_MAGIC.len()].copy_from_slice(STREAM_MAGIC);
        hdr[9] = (DEFAULT_BLOCK_SIZE.trailing_zeros() - 10) as u8;
        hdr
    }

    fn check(&self) -> io::Result<()> {
        match self.err {
            Some(kind) => Err(io::Error::new(kind, "minlz writer is in an error state")),
            None => Ok(()),
        }
    }

    fn w(&mut self) -> io::Result<&mut W> {
        self.inner
            .as_mut()
            .ok_or_else(|| io::Error::other("minlz writer already finished"))
    }

    fn write_header_if_needed(&mut self) -> io::Result<()> {
        if !self.wrote_header {
            let hdr = Self::header();
            self.write_raw(&hdr)?;
            self.wrote_header = true;
        }
        Ok(())
    }

    /// Frame and write one block of uncompressed data (`block.len() <=
    /// block_size`). Emits a compressed (0x02) or stored (0x01) chunk.
    fn write_block(&mut self, block: &[u8]) -> io::Result<()> {
        debug_assert!(!block.is_empty() && block.len() <= self.block_size);
        self.write_header_if_needed()?;
        // Record this block's start offsets for the index (before writing it).
        if let Some(index) = &mut self.index {
            index.push(self.compressed_written, self.uncompressed_written);
        }
        let checksum = crc(block);

        let body = compress_body(block, self.level);
        let (chunk_type, payload): (u8, &[u8]) = match &body {
            Some(b) if b.len() < block.len() => (CHUNK_COMPRESSED, b.as_slice()),
            _ => (CHUNK_UNCOMPRESSED, block),
        };
        let chunk_len = 4 + payload.len();
        if chunk_len > MAX_CHUNK_LEN {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "minlz chunk exceeds maximum length",
            ));
        }

        let mut hdr = [0u8; 8];
        hdr[0] = chunk_type;
        hdr[1] = chunk_len as u8;
        hdr[2] = (chunk_len >> 8) as u8;
        hdr[3] = (chunk_len >> 16) as u8;
        hdr[4..8].copy_from_slice(&checksum.to_le_bytes());
        self.write_raw(&hdr)?;
        self.write_raw(payload)?;
        self.uncompressed_written += block.len() as u64;
        Ok(())
    }

    /// Emit any fully-buffered blocks (used while accepting `write`s).
    fn drain_full_blocks(&mut self) -> io::Result<()> {
        while self.ibuf.len() >= self.block_size {
            let block: Vec<u8> = self.ibuf.drain(..self.block_size).collect();
            self.write_block(&block)?;
        }
        Ok(())
    }

    /// Emit whatever is buffered (a final, possibly partial, block).
    fn flush_buffer(&mut self) -> io::Result<()> {
        if !self.ibuf.is_empty() {
            let block = core::mem::take(&mut self.ibuf);
            self.write_block(&block)?;
            self.ibuf = Vec::with_capacity(self.block_size);
        }
        Ok(())
    }

    fn write_eof(&mut self) -> io::Result<()> {
        self.write_header_if_needed()?;
        let mut tmp = [0u8; 4 + 10];
        tmp[0] = CHUNK_EOF;
        let mut n = 0usize;
        let mut v = self.uncompressed_written;
        loop {
            let mut byte = (v & 0x7f) as u8;
            v >>= 7;
            if v != 0 {
                byte |= 0x80;
            }
            tmp[4 + n] = byte;
            n += 1;
            if v == 0 {
                break;
            }
        }
        tmp[1] = n as u8; // 3-byte length; n <= 10 fits in one byte
        let len = 4 + n;
        let bytes = tmp;
        self.write_raw(&bytes[..len])
    }

    /// Append the seek index chunk after the end-of-stream marker.
    fn write_index(&mut self) -> io::Result<()> {
        if let Some(index) = self.index.take() {
            let chunk = index.encode(self.uncompressed_written, self.compressed_written as i64);
            self.write_raw(&chunk)?;
        }
        Ok(())
    }

    /// Flush the final block, write the end-of-stream marker, and return the
    /// inner writer. Consumes the writer.
    pub fn finish(mut self) -> io::Result<W> {
        self.check()?;
        self.do_finish()?;
        Ok(self.inner.take().expect("inner present before finish"))
    }

    fn do_finish(&mut self) -> io::Result<()> {
        if self.finished {
            return Ok(());
        }
        self.finished = true;
        self.flush_buffer()?;
        self.write_eof()?;
        self.write_index()?;
        self.w()?.flush()
    }

    /// Record a sticky error on failure.
    fn guard<T>(&mut self, r: io::Result<T>) -> io::Result<T> {
        if let Err(e) = &r {
            self.err = Some(e.kind());
        }
        r
    }
}

impl<W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.check()?;
        if self.finished {
            return Err(io::Error::other("write after minlz stream finished"));
        }
        self.ibuf.extend_from_slice(buf);
        let r = self.drain_full_blocks();
        self.guard(r)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.check()?;
        let r = self
            .flush_buffer()
            .and_then(|()| self.w().and_then(|w| w.flush()));
        self.guard(r)
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        if !self.finished && self.err.is_none() && self.inner.is_some() {
            let _ = self.do_finish();
        }
    }
}
