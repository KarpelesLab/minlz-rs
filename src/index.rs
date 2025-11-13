// Copyright 2024 Karpeles Lab Inc.
// S2 Index support for seeking in compressed streams

use crate::constants::CHUNK_TYPE_INDEX;
use crate::error::{Error, Result};

/// S2 Index header and trailer constants
const S2_INDEX_HEADER: &[u8] = b"s2idx\x00";
const S2_INDEX_TRAILER: &[u8] = b"\x00xdi2s";
const MAX_INDEX_ENTRIES: usize = 1 << 16;
const MIN_INDEX_DIST: i64 = 1 << 20; // 1MB minimum distance between entries
const SKIPPABLE_FRAME_HEADER: usize = 4;

/// Entry in the index mapping compressed to uncompressed offsets
#[derive(Debug, Clone, Copy)]
struct IndexEntry {
    compressed_offset: i64,
    uncompressed_offset: i64,
}

/// Index represents an S2/Snappy index for seeking support.
///
/// An index allows random access to compressed data by storing
/// offset pairs (compressed, uncompressed) at regular intervals.
#[derive(Debug, Clone)]
pub struct Index {
    /// Total uncompressed size if known. Will be -1 if unknown.
    pub total_uncompressed: i64,
    /// Total compressed size if known. Will be -1 if unknown.
    pub total_compressed: i64,
    /// Offset entries
    info: Vec<IndexEntry>,
    /// Estimated block uncompressed size
    est_block_uncomp: i64,
}

impl Index {
    /// Create a new empty index
    pub fn new() -> Self {
        Index {
            total_uncompressed: -1,
            total_compressed: -1,
            info: Vec::new(),
            est_block_uncomp: 0,
        }
    }

    /// Reset the index with a maximum block size hint
    pub fn reset(&mut self, max_block: i64) {
        self.est_block_uncomp = max_block;
        self.total_compressed = -1;
        self.total_uncompressed = -1;
        self.info.clear();
    }

    /// Add a compressed/uncompressed offset pair.
    /// Entries must be added in order.
    pub fn add(&mut self, compressed_offset: i64, uncompressed_offset: i64) -> Result<()> {
        if let Some(&last) = self.info.last() {
            // Check if uncompressed offset didn't change
            if last.uncompressed_offset == uncompressed_offset {
                // Update the compressed offset only
                if let Some(entry) = self.info.last_mut() {
                    entry.compressed_offset = compressed_offset;
                }
                return Ok(());
            }

            // Validate ordering
            if last.uncompressed_offset > uncompressed_offset {
                return Err(Error::Corrupt);
            }
            if last.compressed_offset > compressed_offset {
                return Err(Error::Corrupt);
            }

            // Only add entry if distance is large enough
            if last.uncompressed_offset + MIN_INDEX_DIST > uncompressed_offset {
                return Ok(());
            }
        }

        self.info.push(IndexEntry {
            compressed_offset,
            uncompressed_offset,
        });
        Ok(())
    }

    /// Find the offset at or before the wanted (uncompressed) offset.
    ///
    /// If offset is 0 or positive it is the offset from the beginning of the file.
    /// If the offset is negative, it is interpreted as the distance from the end of the file,
    /// where -1 represents the last byte.
    ///
    /// Returns (compressed_offset, uncompressed_offset) tuple.
    pub fn find(&self, offset: i64) -> Result<(i64, i64)> {
        if self.total_uncompressed < 0 {
            return Err(Error::Corrupt);
        }

        let offset = if offset < 0 {
            let offset = self.total_uncompressed + offset;
            if offset < 0 {
                return Err(Error::InvalidInput("offset before start".to_string()));
            }
            offset
        } else {
            offset
        };

        if offset > self.total_uncompressed {
            return Err(Error::InvalidInput("offset beyond end".to_string()));
        }

        // For large indexes, use binary search
        if self.info.len() > 200 {
            let idx = match self
                .info
                .binary_search_by_key(&offset, |e| e.uncompressed_offset)
            {
                Ok(i) => i,
                Err(i) => {
                    if i == 0 {
                        0
                    } else {
                        i - 1
                    }
                }
            };
            let entry = self.info[idx];
            return Ok((entry.compressed_offset, entry.uncompressed_offset));
        }

        // For small indexes, linear search
        let mut compressed_off = 0;
        let mut uncompressed_off = 0;
        for entry in &self.info {
            if entry.uncompressed_offset > offset {
                break;
            }
            compressed_off = entry.compressed_offset;
            uncompressed_off = entry.uncompressed_offset;
        }
        Ok((compressed_off, uncompressed_off))
    }

    /// Reduce index size to stay below MAX_INDEX_ENTRIES
    fn reduce(&mut self) {
        if self.info.len() < MAX_INDEX_ENTRIES && self.est_block_uncomp >= MIN_INDEX_DIST {
            return;
        }

        // Keep 1, remove removeN entries
        let mut remove_n = (self.info.len() + 1) / MAX_INDEX_ENTRIES;

        // Each block should be at least 1MB, but don't reduce below 1000 entries
        while self.est_block_uncomp * (remove_n as i64 + 1) < MIN_INDEX_DIST
            && self.info.len() / (remove_n + 1) > 1000
        {
            remove_n += 1;
        }

        let mut j = 0;
        let mut idx = 0;
        while idx < self.info.len() {
            self.info[j] = self.info[idx];
            j += 1;
            idx += remove_n + 1;
        }
        self.info.truncate(j);

        // Update maxblock estimate
        self.est_block_uncomp += self.est_block_uncomp * remove_n as i64;
    }

    /// Serialize index to bytes and append to buffer
    pub fn append_to(
        &mut self,
        buf: &mut Vec<u8>,
        uncomp_total: i64,
        comp_total: i64,
    ) -> Result<()> {
        self.reduce();

        let init_size = buf.len();

        // Skippable frame header
        buf.push(CHUNK_TYPE_INDEX);
        buf.extend_from_slice(&[0, 0, 0]); // Placeholder for chunk length

        // Header
        buf.extend_from_slice(S2_INDEX_HEADER);

        // Total uncompressed size
        encode_varint(buf, uncomp_total);

        // Total compressed size
        encode_varint(buf, comp_total);

        // Estimated block uncompressed size
        encode_varint(buf, self.est_block_uncomp);

        // Number of entries
        encode_varint(buf, self.info.len() as i64);

        // Check if we need to store uncompressed offsets
        let mut has_uncompressed = 0u8;
        for (idx, entry) in self.info.iter().enumerate() {
            if idx == 0 {
                if entry.uncompressed_offset != 0 {
                    has_uncompressed = 1;
                    break;
                }
                continue;
            }
            let expected = self.info[idx - 1].uncompressed_offset + self.est_block_uncomp;
            if entry.uncompressed_offset != expected {
                has_uncompressed = 1;
                break;
            }
        }
        buf.push(has_uncompressed);

        // Add uncompressed offsets if needed (delta encoded)
        if has_uncompressed == 1 {
            for (idx, entry) in self.info.iter().enumerate() {
                let u_off = if idx == 0 {
                    entry.uncompressed_offset
                } else {
                    let prev = self.info[idx - 1];
                    entry.uncompressed_offset - prev.uncompressed_offset - self.est_block_uncomp
                };
                encode_varint(buf, u_off);
            }
        }

        // Add compressed offsets (delta encoded with prediction)
        let mut c_predict = self.est_block_uncomp / 2;
        for (idx, entry) in self.info.iter().enumerate() {
            let c_off = if idx == 0 {
                entry.compressed_offset
            } else {
                let prev = self.info[idx - 1];
                let off = entry.compressed_offset - prev.compressed_offset - c_predict;
                // Update prediction with half the error
                c_predict += off / 2;
                off
            };
            encode_varint(buf, c_off);
        }

        // Add total size (fixed size for easier reading)
        let total_size = (buf.len() - init_size + 4 + S2_INDEX_TRAILER.len()) as u32;
        buf.extend_from_slice(&total_size.to_le_bytes());

        // Trailer
        buf.extend_from_slice(S2_INDEX_TRAILER);

        // Update chunk length in header
        let chunk_len = buf.len() - init_size - SKIPPABLE_FRAME_HEADER;
        buf[init_size + 1] = (chunk_len & 0xff) as u8;
        buf[init_size + 2] = ((chunk_len >> 8) & 0xff) as u8;
        buf[init_size + 3] = ((chunk_len >> 16) & 0xff) as u8;

        Ok(())
    }

    /// Load index from bytes
    pub fn load<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8]> {
        if data.len() <= 4 + S2_INDEX_HEADER.len() + S2_INDEX_TRAILER.len() {
            return Err(Error::BufferTooSmall);
        }

        if data[0] != CHUNK_TYPE_INDEX {
            return Err(Error::Corrupt);
        }

        let chunk_len = data[1] as usize | (data[2] as usize) << 8 | (data[3] as usize) << 16;
        let mut b = &data[4..];

        if b.len() < chunk_len {
            return Err(Error::BufferTooSmall);
        }

        // Validate header
        if !b.starts_with(S2_INDEX_HEADER) {
            return Err(Error::Unsupported);
        }
        b = &b[S2_INDEX_HEADER.len()..];

        // Read total uncompressed
        let (v, n) = decode_varint(b)?;
        if v < 0 {
            return Err(Error::Corrupt);
        }
        self.total_uncompressed = v;
        b = &b[n..];

        // Read total compressed
        let (v, n) = decode_varint(b)?;
        self.total_compressed = v;
        b = &b[n..];

        // Read estimated block size
        let (v, n) = decode_varint(b)?;
        if v < 0 {
            return Err(Error::Corrupt);
        }
        self.est_block_uncomp = v;
        b = &b[n..];

        // Read number of entries
        let (v, n) = decode_varint(b)?;
        if v < 0 || v > MAX_INDEX_ENTRIES as i64 {
            return Err(Error::Corrupt);
        }
        let entries = v as usize;
        b = &b[n..];

        // Allocate space for entries
        self.info.clear();
        self.info.reserve(entries);

        // Read hasUncompressed flag
        if b.is_empty() {
            return Err(Error::Corrupt);
        }
        let has_uncompressed = b[0];
        b = &b[1..];

        // Read uncompressed offsets if present
        let mut uncomp_offsets = Vec::with_capacity(entries);
        if has_uncompressed == 1 {
            for i in 0..entries {
                let (v, n) = decode_varint(b)?;
                b = &b[n..];
                let uncomp_off = if i == 0 {
                    v
                } else {
                    uncomp_offsets[i - 1] + self.est_block_uncomp + v
                };
                uncomp_offsets.push(uncomp_off);
            }
        } else {
            // Generate expected offsets
            for i in 0..entries {
                uncomp_offsets.push(i as i64 * self.est_block_uncomp);
            }
        }

        // Read compressed offsets
        let mut c_predict = self.est_block_uncomp / 2;
        for i in 0..entries {
            let (v, n) = decode_varint(b)?;
            b = &b[n..];
            let comp_off = if i == 0 {
                v
            } else {
                let prev_comp = self.info[i - 1].compressed_offset;
                c_predict += v / 2;
                prev_comp + c_predict + v
            };

            self.info.push(IndexEntry {
                compressed_offset: comp_off,
                uncompressed_offset: uncomp_offsets[i],
            });
        }

        // Validate trailer
        if b.len() < 4 + S2_INDEX_TRAILER.len() {
            return Err(Error::Corrupt);
        }

        let total_size_pos = b.len() - S2_INDEX_TRAILER.len() - 4;
        let trailer_pos = b.len() - S2_INDEX_TRAILER.len();

        if &b[trailer_pos..trailer_pos + S2_INDEX_TRAILER.len()] != S2_INDEX_TRAILER {
            return Err(Error::Corrupt);
        }

        let remaining = &b[total_size_pos + 4 + S2_INDEX_TRAILER.len()..];
        Ok(remaining)
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode a signed varint
fn encode_varint(buf: &mut Vec<u8>, value: i64) {
    // Zig-zag encoding for signed values
    let uvalue = ((value << 1) ^ (value >> 63)) as u64;

    let mut v = uvalue;
    while v >= 0x80 {
        buf.push((v as u8) | 0x80);
        v >>= 7;
    }
    buf.push(v as u8);
}

/// Decode a signed varint, returns (value, bytes_read)
fn decode_varint(data: &[u8]) -> Result<(i64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    let mut i = 0;

    while i < data.len() && i < 10 {
        let byte = data[i];
        result |= ((byte & 0x7f) as u64) << shift;
        i += 1;

        if byte < 0x80 {
            // Zig-zag decode
            let value = ((result >> 1) as i64) ^ -((result & 1) as i64);
            return Ok((value, i));
        }
        shift += 7;
    }

    Err(Error::Corrupt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_basic() {
        let mut index = Index::new();
        index.reset(1024 * 1024); // 1MB blocks
        index.total_uncompressed = 10 * 1024 * 1024; // 10MB total

        // Add entries with proper spacing (>= MIN_INDEX_DIST = 1MB)
        assert!(index.add(0, 0).is_ok());
        assert!(index.add(500_000, 1024 * 1024).is_ok()); // 1MB uncompressed
        assert!(index.add(1_000_000, 2 * 1024 * 1024).is_ok()); // 2MB uncompressed
        assert!(index.add(1_500_000, 3 * 1024 * 1024).is_ok()); // 3MB uncompressed

        // Test find at start
        let (c, u) = index.find(0).unwrap();
        assert_eq!(c, 0);
        assert_eq!(u, 0);

        // Test finding in second block
        let (c, u) = index.find(1024 * 1024).unwrap();
        assert_eq!(c, 500_000);
        assert_eq!(u, 1024 * 1024);

        // Test finding between blocks (should return earlier block)
        let (c, u) = index.find(1024 * 1024 + 500_000).unwrap();
        assert_eq!(c, 500_000);
        assert_eq!(u, 1024 * 1024);
    }

    #[test]
    fn test_varint_roundtrip() {
        let test_values = vec![0, 1, -1, 127, -127, 128, -128, 65535, -65535];

        for &val in &test_values {
            let mut buf = Vec::new();
            encode_varint(&mut buf, val);
            let (decoded, n) = decode_varint(&buf).unwrap();
            assert_eq!(decoded, val, "Failed for value {}", val);
            assert_eq!(n, buf.len());
        }
    }
}
