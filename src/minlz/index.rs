// Copyright 2024 Karpeles Lab Inc.
// MinLZ stream index (chunk 0x40). See mod.rs for licensing/attribution.
//
// Byte-compatible with the reference index (SPEC §4.12): an `s2idx\0`-headed
// skippable chunk mapping uncompressed offsets to compressed (stream) offsets,
// enabling random access without decoding from the start.

use crate::error::{Error, Result};
use alloc::vec::Vec;

pub(crate) const INDEX_CHUNK: u8 = 0x40;
const INDEX_HEADER: &[u8] = b"s2idx\x00";
const INDEX_TRAILER: &[u8] = b"\x00xdi2s";
const MAX_INDEX_ENTRIES: usize = 1 << 16;

/// A parsed MinLZ stream index: a sorted list of `(compressed_offset,
/// uncompressed_offset)` block boundaries usable for seeking.
#[derive(Debug, Clone, Default)]
pub struct Index {
    /// `(compressed_offset, uncompressed_offset)` pairs, ascending.
    entries: Vec<(u64, u64)>,
    /// Estimated uncompressed bytes per block (the writer's block size).
    est_block: u64,
    total_uncompressed: u64,
    /// Total compressed size, or `-1` if unknown.
    total_compressed: i64,
}

impl Index {
    // Construction is used by the (std-only) stream writer.
    #[cfg(feature = "std")]
    pub(crate) fn new(est_block: u64) -> Self {
        Index {
            entries: Vec::new(),
            est_block,
            total_uncompressed: 0,
            total_compressed: -1,
        }
    }

    #[cfg(feature = "std")]
    pub(crate) fn push(&mut self, compressed_offset: u64, uncompressed_offset: u64) {
        self.entries.push((compressed_offset, uncompressed_offset));
    }

    /// Number of indexed block boundaries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the index has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Total uncompressed size of the indexed stream.
    pub fn total_uncompressed(&self) -> u64 {
        self.total_uncompressed
    }

    /// Find the indexed block at or before `uncompressed_offset`. Returns
    /// `(compressed_offset, block_uncompressed_offset)`: seek the stream to
    /// `compressed_offset`, decode, then discard `uncompressed_offset -
    /// block_uncompressed_offset` bytes.
    pub fn find(&self, uncompressed_offset: u64) -> (u64, u64) {
        let mut best = (0u64, 0u64);
        for &(c, u) in &self.entries {
            if u > uncompressed_offset {
                break;
            }
            best = (c, u);
        }
        best
    }

    /// Encode the index as a `0x40` skippable chunk, byte-compatible with the
    /// reference. `total_compressed` is the stream size excluding the index, or
    /// `-1` if unknown.
    pub fn encode(&self, total_uncompressed: u64, total_compressed: i64) -> Vec<u8> {
        let mut b = Vec::with_capacity(32 + self.entries.len() * 4);
        b.extend_from_slice(&[INDEX_CHUNK, 0, 0, 0]); // length patched below
        b.extend_from_slice(INDEX_HEADER);
        put_svarint(&mut b, total_uncompressed as i64);
        put_svarint(&mut b, total_compressed);
        put_svarint(&mut b, self.est_block as i64);
        put_svarint(&mut b, self.entries.len() as i64);

        // Are uncompressed offsets exactly uniform (prev + est_block)? If so we
        // can omit them.
        let mut has_uncompressed = 0u8;
        for (idx, &(_, u)) in self.entries.iter().enumerate() {
            if idx == 0 {
                if u != 0 {
                    has_uncompressed = 1;
                    break;
                }
            } else if u != self.entries[idx - 1].1 + self.est_block {
                has_uncompressed = 1;
                break;
            }
        }
        b.push(has_uncompressed);

        if has_uncompressed == 1 {
            for (idx, &(_, u)) in self.entries.iter().enumerate() {
                let mut delta = u as i64;
                if idx > 0 {
                    delta -= (self.entries[idx - 1].1 + self.est_block) as i64;
                }
                put_svarint(&mut b, delta);
            }
        }

        let mut c_predict = (self.est_block / 2) as i64;
        for (idx, &(c, _)) in self.entries.iter().enumerate() {
            let mut delta = c as i64;
            if idx > 0 {
                delta -= (self.entries[idx - 1].0 as i64) + c_predict;
                c_predict += delta / 2;
            }
            put_svarint(&mut b, delta);
        }

        let total = (b.len() + 4 + INDEX_TRAILER.len()) as u32;
        b.extend_from_slice(&total.to_le_bytes());
        b.extend_from_slice(INDEX_TRAILER);

        let chunk_len = b.len() - 4;
        b[1] = chunk_len as u8;
        b[2] = (chunk_len >> 8) as u8;
        b[3] = (chunk_len >> 16) as u8;
        b
    }

    /// Parse an index from a full stream by locating the index chunk via its
    /// trailer at the end of the stream.
    pub fn load(stream: &[u8]) -> Result<Index> {
        if stream.len() < 4 + INDEX_HEADER.len() + 4 + INDEX_TRAILER.len() {
            return Err(Error::Corrupt);
        }
        let tlen = INDEX_TRAILER.len();
        if &stream[stream.len() - tlen..] != INDEX_TRAILER {
            return Err(Error::Unsupported); // no index at end of stream
        }
        let size_pos = stream.len() - tlen - 4;
        let size = u32::from_le_bytes([
            stream[size_pos],
            stream[size_pos + 1],
            stream[size_pos + 2],
            stream[size_pos + 3],
        ]) as usize;
        if size > stream.len() {
            return Err(Error::Corrupt);
        }
        Index::decode_chunk(&stream[stream.len() - size..])
    }

    /// Parse an index from a `0x40` chunk (starting at the chunk-type byte).
    pub fn decode_chunk(b: &[u8]) -> Result<Index> {
        if b.len() <= 4 + INDEX_HEADER.len() + INDEX_TRAILER.len() {
            return Err(Error::Corrupt);
        }
        if b[0] != INDEX_CHUNK {
            return Err(Error::Corrupt);
        }
        let chunk_len = b[1] as usize | (b[2] as usize) << 8 | (b[3] as usize) << 16;
        let mut p = &b[4..];
        if p.len() < chunk_len {
            return Err(Error::Corrupt);
        }
        if &p[..INDEX_HEADER.len()] != INDEX_HEADER {
            return Err(Error::Unsupported);
        }
        p = &p[INDEX_HEADER.len()..];

        let mut idx = Index::default();
        let (tu, n) = read_svarint(p).ok_or(Error::Corrupt)?;
        if tu < 0 {
            return Err(Error::Corrupt);
        }
        idx.total_uncompressed = tu as u64;
        p = &p[n..];

        let (tc, n) = read_svarint(p).ok_or(Error::Corrupt)?;
        idx.total_compressed = tc;
        p = &p[n..];

        let (eb, n) = read_svarint(p).ok_or(Error::Corrupt)?;
        if eb < 0 {
            return Err(Error::Corrupt);
        }
        idx.est_block = eb as u64;
        p = &p[n..];

        let (entries, n) = read_svarint(p).ok_or(Error::Corrupt)?;
        if entries < 0 || entries as usize > MAX_INDEX_ENTRIES {
            return Err(Error::Corrupt);
        }
        let entries = entries as usize;
        p = &p[n..];

        if p.is_empty() {
            return Err(Error::Corrupt);
        }
        let has_uncompressed = p[0];
        if has_uncompressed & 1 != has_uncompressed {
            return Err(Error::Corrupt);
        }
        p = &p[1..];

        idx.entries = Vec::with_capacity(entries);
        let mut prev_u = 0u64;
        for i in 0..entries {
            let mut u = 0i64;
            if has_uncompressed != 0 {
                let (v, n) = read_svarint(p).ok_or(Error::Corrupt)?;
                u = v;
                p = &p[n..];
            }
            if i > 0 {
                u += prev_u as i64 + idx.est_block as i64;
                if u as u64 <= prev_u {
                    return Err(Error::Corrupt);
                }
            }
            if u < 0 {
                return Err(Error::Corrupt);
            }
            prev_u = u as u64;
            idx.entries.push((0, prev_u));
        }

        let mut c_predict = (idx.est_block / 2) as i64;
        let mut prev_c = 0i64;
        for (i, entry) in idx.entries.iter_mut().enumerate() {
            let (mut c, n) = read_svarint(p).ok_or(Error::Corrupt)?;
            p = &p[n..];
            if i > 0 {
                let c_predict_new = c_predict + c / 2;
                c += prev_c + c_predict;
                if c <= prev_c {
                    return Err(Error::Corrupt);
                }
                c_predict = c_predict_new;
            }
            if c < 0 {
                return Err(Error::Corrupt);
            }
            prev_c = c;
            entry.0 = c as u64;
        }
        Ok(idx)
    }
}

/// Append a signed zigzag LEB128 varint (matches Go `binary.PutVarint`).
fn put_svarint(buf: &mut Vec<u8>, v: i64) {
    let mut ux = ((v << 1) ^ (v >> 63)) as u64;
    loop {
        let mut byte = (ux & 0x7f) as u8;
        ux >>= 7;
        if ux != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if ux == 0 {
            break;
        }
    }
}

/// Decode a signed zigzag LEB128 varint (matches Go `binary.Varint`).
fn read_svarint(b: &[u8]) -> Option<(i64, usize)> {
    let mut ux: u64 = 0;
    let mut shift = 0u32;
    for (i, &byte) in b.iter().enumerate() {
        if shift >= 64 {
            return None;
        }
        ux |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            let x = (ux >> 1) as i64 ^ -((ux & 1) as i64);
            return Some((x, i + 1));
        }
        shift += 7;
    }
    None
}
