// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::constants::*;
use crate::dict::Dict;
use crate::error::{Error, Result};
use crate::varint::encode_varint;

/// Encoder for S2 compression
pub struct Encoder;

impl Encoder {
    /// Create a new encoder
    pub fn new() -> Self {
        Encoder
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode returns the encoded form of src.
/// The encoding is compatible with the Go s2 implementation.
pub fn encode(src: &[u8]) -> Vec<u8> {
    let max_len = max_encoded_len(src.len()).expect("source too large");
    let mut dst = vec![0u8; max_len];

    // Write the varint-encoded length of the decompressed bytes
    let d = encode_varint(&mut dst, src.len() as u64);

    if src.is_empty() {
        dst.truncate(d);
        return dst;
    }

    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        let n = emit_literal(&mut dst[d..], src);
        dst.truncate(d + n);
        return dst;
    }

    let n = encode_block(&mut dst[d..], src);
    if n > 0 {
        dst.truncate(d + n);
        return dst;
    }

    // Not compressible
    let n = emit_literal(&mut dst[d..], src);
    dst.truncate(d + n);
    dst
}

/// EncodeBetter provides better compression than Encode but is slower
pub fn encode_better(src: &[u8]) -> Vec<u8> {
    let max_len = max_encoded_len(src.len()).expect("source too large");
    let mut dst = vec![0u8; max_len];

    // Write the varint-encoded length of the decompressed bytes
    let d = encode_varint(&mut dst, src.len() as u64);

    if src.is_empty() {
        dst.truncate(d);
        return dst;
    }

    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        let n = emit_literal(&mut dst[d..], src);
        dst.truncate(d + n);
        return dst;
    }

    let n = encode_block_better(&mut dst[d..], src);
    if n > 0 {
        dst.truncate(d + n);
        return dst;
    }

    // Not compressible
    let n = emit_literal(&mut dst[d..], src);
    dst.truncate(d + n);
    dst
}

/// Encode with dictionary support
///
/// Uses the dictionary to find matches and improve compression ratio.
/// The dictionary is pre-populated into the hash table, allowing matches
/// against common patterns that appear in the dictionary.
pub fn encode_with_dict(src: &[u8], dict: &Dict) -> Vec<u8> {
    let max_len = max_encoded_len(src.len()).expect("source too large");
    let mut dst = vec![0u8; max_len];

    // Write the varint-encoded length of the decompressed bytes
    let d = encode_varint(&mut dst, src.len() as u64);

    if src.is_empty() {
        dst.truncate(d);
        return dst;
    }

    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        let n = emit_literal(&mut dst[d..], src);
        dst.truncate(d + n);
        return dst;
    }

    let n = encode_block_dict(&mut dst[d..], src, dict);
    if n > 0 {
        dst.truncate(d + n);
        return dst;
    }

    // Fallback to literal encoding
    let n = emit_literal(&mut dst[d..], src);
    dst.truncate(d + n);
    dst
}

/// Encode better with dictionary support
///
/// NOTE: Current implementation falls back to standard better encoding.
/// Dictionary is used for decoding but not yet for encoding optimization.
pub fn encode_better_with_dict(src: &[u8], _dict: &Dict) -> Vec<u8> {
    // TODO: Implement full dictionary-aware encoding
    encode_better(src)
}

/// Encode best with dictionary support
///
/// NOTE: Current implementation falls back to standard best encoding.
/// Dictionary is used for decoding but not yet for encoding optimization.
pub fn encode_best_with_dict(src: &[u8], _dict: &Dict) -> Vec<u8> {
    // TODO: Implement full dictionary-aware encoding
    encode_best(src)
}

/// Encode using Snappy-compatible format (no repeat offsets)
///
/// This produces output compatible with the original Snappy format,
/// which can be decompressed by both S2 and Snappy decoders.
/// The encoding is less efficient than S2 as it doesn't use repeat offsets.
pub fn encode_snappy(src: &[u8]) -> Vec<u8> {
    let max_len = max_encoded_len(src.len()).expect("source too large");
    let mut dst = vec![0u8; max_len];

    // Write the varint-encoded length of the decompressed bytes
    let d = encode_varint(&mut dst, src.len() as u64);

    if src.is_empty() {
        dst.truncate(d);
        return dst;
    }

    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        let n = emit_literal(&mut dst[d..], src);
        dst.truncate(d + n);
        return dst;
    }

    let n = encode_block_snappy(&mut dst[d..], src);
    if n > 0 {
        dst.truncate(d + n);
        return dst;
    }

    // Not compressible
    let n = emit_literal(&mut dst[d..], src);
    dst.truncate(d + n);
    dst
}

/// EncodeBest provides the best compression but is the slowest
pub fn encode_best(src: &[u8]) -> Vec<u8> {
    let max_len = max_encoded_len(src.len()).expect("source too large");
    let mut dst = vec![0u8; max_len];

    // Write the varint-encoded length of the decompressed bytes
    let d = encode_varint(&mut dst, src.len() as u64);

    if src.is_empty() {
        dst.truncate(d);
        return dst;
    }

    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        let n = emit_literal(&mut dst[d..], src);
        dst.truncate(d + n);
        return dst;
    }

    let n = encode_block_best(&mut dst[d..], src);
    if n > 0 {
        dst.truncate(d + n);
        return dst;
    }

    // Not compressible
    let n = emit_literal(&mut dst[d..], src);
    dst.truncate(d + n);
    dst
}

/// Returns the maximum length of an encoded block
pub fn max_encoded_len(src_len: usize) -> Result<usize> {
    if src_len > 0xffffffff {
        return Err(Error::TooLarge);
    }

    #[cfg(target_pointer_width = "32")]
    {
        if src_len > 0x7fffffff {
            return Err(Error::TooLarge);
        }
    }

    // Size of the varint encoded block size
    // Use Go's formula: (bits.Len64(n) + 7) / 7
    // This is slightly conservative (over-estimates at boundaries like 127, 16383)
    let bits_needed = if src_len == 0 {
        0
    } else {
        64 - (src_len as u64).leading_zeros() as usize
    };
    let varint_extra = (bits_needed + 7) / 7;
    let mut n = src_len + varint_extra;

    // Add maximum size of encoding block as literals
    let literal_extra = literal_extra_size(src_len as i64) as usize;
    n += literal_extra;

    // Add safety margin for compression metadata overhead.
    // During compression attempts, multiple literals and copy operations
    // may be emitted before deciding compression isn't worthwhile,
    // consuming extra space beyond the single-literal assumption.
    // We add src_len/32 bytes to account for this overhead, which aligns
    // with the compression worthiness threshold in encode_block.
    let safety_margin = src_len / 32 + 1;
    n += safety_margin;

    #[cfg(target_pointer_width = "32")]
    {
        if n > 0x7fffffff {
            return Err(Error::TooLarge);
        }
    }

    if n > 0xffffffff {
        return Err(Error::TooLarge);
    }

    Ok(n)
}

/// Calculate the extra size needed for literal encoding
fn literal_extra_size(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    // Matches Go's literalExtraSize function in s2.go
    // These ranges are conservative (slightly over-estimate) for MaxEncodedLen
    // Note: n is the literal LENGTH. The boundaries are conservative, returning
    // the next header size one position early to ensure sufficient buffer space.
    match n {
        ..60 => 1,            // 0-59: header 1 byte (emitLiteral uses 1 byte for len 1-60)
        60..256 => 2,         // 60-255: header 2 bytes (emitLiteral uses 2 bytes for len 61-256)
        256..65536 => 3, // 256-65535: header 3 bytes (emitLiteral uses 3 bytes for len 257-65536)
        65536..16777216 => 4, // 65536-16777215: header 4 bytes (emitLiteral uses 4 bytes for len 65537-16777216)
        _ => 5,               // >= 16777216: header 5 bytes
    }
}

/// Emit a literal chunk and return the number of bytes written
fn emit_literal(dst: &mut [u8], lit: &[u8]) -> usize {
    if lit.is_empty() {
        return 0;
    }

    let n = lit.len() - 1;
    let i = match n {
        0..=59 => {
            dst[0] = ((n as u8) << 2) | TAG_LITERAL;
            1
        }
        60..=255 => {
            dst[0] = (60 << 2) | TAG_LITERAL;
            dst[1] = n as u8;
            2
        }
        256..=65535 => {
            dst[0] = (61 << 2) | TAG_LITERAL;
            let bytes = (n as u16).to_le_bytes();
            dst[1] = bytes[0];
            dst[2] = bytes[1];
            3
        }
        65536..=16777215 => {
            dst[0] = (62 << 2) | TAG_LITERAL;
            dst[1] = n as u8;
            dst[2] = (n >> 8) as u8;
            dst[3] = (n >> 16) as u8;
            4
        }
        _ => {
            dst[0] = (63 << 2) | TAG_LITERAL;
            let bytes = (n as u32).to_le_bytes();
            dst[1] = bytes[0];
            dst[2] = bytes[1];
            dst[3] = bytes[2];
            dst[4] = bytes[3];
            5
        }
    };

    // Bounds check before copying
    if i + lit.len() > dst.len() {
        panic!(
            "emit_literal: insufficient dst space: need {}, have {}",
            i + lit.len(),
            dst.len()
        );
    }

    dst[i..i + lit.len()].copy_from_slice(lit);
    i + lit.len()
}

/// Emit a copy chunk without repeat optimization and return the number of bytes written
fn emit_copy_no_repeat(dst: &mut [u8], offset: usize, length: usize) -> usize {
    if offset >= 65536 {
        let mut i = 0;
        let mut remaining = length;

        if remaining > 64 {
            dst[0] = ((63 << 2) | TAG_COPY4 as usize) as u8;
            let bytes = (offset as u32).to_le_bytes();
            dst[1] = bytes[0];
            dst[2] = bytes[1];
            dst[3] = bytes[2];
            dst[4] = bytes[3];
            remaining -= 64;
            i = 5;

            if remaining >= 4 {
                return i + emit_copy_no_repeat(&mut dst[i..], offset, remaining);
            }
        }

        if remaining == 0 {
            return i;
        }

        dst[i] = (((remaining - 1) << 2) | TAG_COPY4 as usize) as u8;
        let bytes = (offset as u32).to_le_bytes();
        dst[i + 1] = bytes[0];
        dst[i + 2] = bytes[1];
        dst[i + 3] = bytes[2];
        dst[i + 4] = bytes[3];
        return i + 5;
    }

    // Offset no more than 2 bytes
    if length > 64 {
        // Emit a length 60 copy, encoded as 3 bytes
        dst[2] = (offset >> 8) as u8;
        dst[1] = offset as u8;
        dst[0] = ((59 << 2) | TAG_COPY2 as usize) as u8;
        let remaining = length - 60;
        // Emit remaining, at least 4 bytes remain
        return 3 + emit_copy_no_repeat(&mut dst[3..], offset, remaining);
    }

    if length >= 12 || offset >= 2048 {
        // Emit the remaining copy, encoded as 3 bytes
        dst[2] = (offset >> 8) as u8;
        dst[1] = offset as u8;
        dst[0] = (((length - 1) << 2) | TAG_COPY2 as usize) as u8;
        return 3;
    }

    // Emit the remaining copy, encoded as 2 bytes
    dst[1] = offset as u8;
    dst[0] = ((offset >> 8) << 5 | ((length - 4) << 2) | TAG_COPY1 as usize) as u8;
    2
}

/// Emit a COPY1 tag (11-bit offset)
#[allow(dead_code)]
fn emit_copy1(dst: &mut [u8], offset: usize, length: usize) -> usize {
    dst[0] = ((offset >> 8) << 5 | ((length - 4) << 2) | TAG_COPY1 as usize) as u8;
    dst[1] = offset as u8;
    2
}

/// Emit a COPY2 tag (16-bit offset)
#[allow(dead_code)]
fn emit_copy2(dst: &mut [u8], offset: usize, length: usize) -> usize {
    dst[0] = (((length - 1) << 2) | TAG_COPY2 as usize) as u8;
    let bytes = (offset as u16).to_le_bytes();
    dst[1] = bytes[0];
    dst[2] = bytes[1];
    3
}

/// Emit a COPY4 tag (32-bit offset)
fn emit_copy4(dst: &mut [u8], offset: usize, length: usize) -> usize {
    let mut i = 0;

    // If length > 64, split into multiple copies
    let mut remaining = length;
    if remaining > 64 {
        dst[0] = ((63 << 2) | TAG_COPY4 as usize) as u8;
        let bytes = (offset as u32).to_le_bytes();
        dst[1] = bytes[0];
        dst[2] = bytes[1];
        dst[3] = bytes[2];
        dst[4] = bytes[3];
        remaining -= 64;
        i = 5;

        if remaining >= 4 {
            // Emit remaining as repeat
            return i + emit_repeat(&mut dst[i..], offset, remaining);
        }
    }

    if remaining == 0 {
        return i;
    }

    dst[i] = (((remaining - 1) << 2) | TAG_COPY4 as usize) as u8;
    let bytes = (offset as u32).to_le_bytes();
    dst[i + 1] = bytes[0];
    dst[i + 2] = bytes[1];
    dst[i + 3] = bytes[2];
    dst[i + 4] = bytes[3];
    i + 5
}

/// Emit a repeat (reuse of the last offset)
fn emit_repeat(dst: &mut [u8], offset: usize, length: usize) -> usize {
    let mut len = length - 4;

    if len <= 4 {
        dst[0] = ((len << 2) | TAG_COPY1 as usize) as u8;
        dst[1] = 0;
        return 2;
    }

    if len < 8 && offset < 2048 {
        // Encode with offset
        dst[0] = (((offset >> 8) << 5) | (len << 2) | TAG_COPY1 as usize) as u8;
        dst[1] = offset as u8;
        return 2;
    }

    if len < (1 << 8) + 4 {
        len -= 4;
        dst[0] = ((5 << 2) | TAG_COPY1 as usize) as u8;
        dst[1] = 0;
        dst[2] = len as u8;
        return 3;
    }

    if len < (1 << 16) + (1 << 8) {
        len -= 1 << 8;
        dst[0] = ((6 << 2) | TAG_COPY1 as usize) as u8;
        dst[1] = 0;
        let bytes = (len as u16).to_le_bytes();
        dst[2] = bytes[0];
        dst[3] = bytes[1];
        return 4;
    }

    len -= 1 << 16;
    dst[0] = ((7 << 2) | TAG_COPY1 as usize) as u8;
    dst[1] = 0;
    dst[2] = len as u8;
    dst[3] = (len >> 8) as u8;
    dst[4] = (len >> 16) as u8;
    5
}

/// Hash function for matching
#[inline]
fn hash(data: &[u8], shift: u32) -> usize {
    let val = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    ((val.wrapping_mul(0x1e35a7bd)) >> shift) as usize
}

/// Hash function for 4 bytes (Better algorithm)
#[inline]
fn hash4(u: u64, h: u8) -> u32 {
    const PRIME_4_BYTES: u32 = 2654435761;
    ((u as u32).wrapping_mul(PRIME_4_BYTES)) >> ((32 - h) & 31)
}

/// Hash function for 5 bytes (Better algorithm)
#[allow(dead_code)]
#[inline]
fn hash5(u: u64, h: u8) -> u32 {
    const PRIME_5_BYTES: u64 = 889523592379;
    (((u << (64 - 40)).wrapping_mul(PRIME_5_BYTES)) >> ((64 - h) & 63)) as u32
}

/// Hash function for 6 bytes (Better algorithm)
#[inline]
fn hash6(u: u64, h: u32) -> u32 {
    const PRIME_6_BYTES: u64 = 0xcf1bbcdcb7a56463;
    ((u.wrapping_mul(PRIME_6_BYTES)) >> (64 - h)) as u32
}

/// Hash function for 7 bytes (Better algorithm)
#[inline]
fn hash7(u: u64, h: u8) -> u32 {
    const PRIME_7_BYTES: u64 = 58295818150454627;
    (((u << (64 - 56)).wrapping_mul(PRIME_7_BYTES)) >> ((64 - h) & 63)) as u32
}

/// Hash function for 8 bytes (Better algorithm)
#[inline]
fn hash8(u: u64, h: u8) -> u32 {
    const PRIME_8_BYTES: u64 = 0xcf1bbcdcb7a56463;
    ((u.wrapping_mul(PRIME_8_BYTES)) >> ((64 - h) & 63)) as u32
}

/// Load a u32 from the slice at the given offset
#[inline]
fn load32(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

/// Load a u64 from the slice at the given offset
#[inline]
fn load64(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

/// Encode a block using the S2 algorithm
fn encode_block(dst: &mut [u8], src: &[u8]) -> usize {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return 0;
    }

    // Hash table size - use 14 bits for blocks up to 64KB, otherwise 17 bits
    let table_bits = if src.len() <= 64 * 1024 { 14 } else { 17 };
    let table_size = 1 << table_bits;
    let shift = 32 - table_bits;

    let mut table = vec![0u32; table_size];

    let s_limit = src.len() - INPUT_MARGIN;
    let mut next_emit = 0;
    let mut s = 1;
    let mut d = 0;

    #[allow(unused_variables)]
    let cv = load64(src, s);

    'outer: loop {
        let mut candidate;
        let mut skip = 32;

        // Search for next match (with skipping)
        'search: loop {
            let next_s = s + (skip >> 5);
            skip += 1;

            if next_s > s_limit {
                break 'outer;
            }

            let h = hash(&src[s..], shift);
            candidate = table[h] as usize;
            table[h] = s as u32;

            if load32(src, s) == load32(src, candidate) {
                break 'search;
            }

            s = next_s;
        }

        // Inner loop: emit matches as long as there are immediate matches
        'emit_copies: loop {
            // Extend backwards
            while candidate > 0 && s > next_emit && src[candidate - 1] == src[s - 1] {
                candidate -= 1;
                s -= 1;
            }

            // Emit literal
            if s > next_emit {
                d += emit_literal(&mut dst[d..], &src[next_emit..s]);
            }

            // Extend the match forward
            let base = s;
            let offset = base - candidate;
            s += 4;
            candidate += 4;

            while s <= src.len() - 8 {
                if load64(src, s) != load64(src, candidate) {
                    let diff = (load64(src, s) ^ load64(src, candidate)).trailing_zeros() / 8;
                    s += diff as usize;
                    break;
                }
                s += 8;
                candidate += 8;
            }

            // Check remaining bytes (0-7 bytes) after the 8-byte aligned loop
            while s < src.len() && candidate < s && src[s] == src[candidate] {
                s += 1;
                candidate += 1;
            }

            // Emit copy (emit_copy handles repeat optimization internally)
            d += emit_copy(&mut dst[d..], offset, s - base);
            next_emit = s;

            // IMPORTANT: Check for immediate matches BEFORE checking s_limit
            // This matches Go's behavior where immediate match check comes first

            if s >= s_limit {
                // At or past limit, emit remaining and exit
                break 'outer;
            }

            // Check for immediate match at current position (like Go's encodeBlockGo)
            // Update hash table for s-2 and s
            if s >= 2 {
                let h_back = hash(&src[s - 2..], shift);
                table[h_back] = (s - 2) as u32;
            }

            let h_curr = hash(&src[s..], shift);
            candidate = table[h_curr] as usize;
            table[h_curr] = s as u32;

            // Check if there's an immediate match (with safety check)
            if candidate < s && s + 4 <= src.len() && load32(src, s) == load32(src, candidate) {
                // Continue emitting copies
                continue 'emit_copies;
            }

            // No immediate match, advance to next position and search again
            s += 1;
            break 'emit_copies;
        }
    }

    // Emit remaining
    if next_emit < src.len() {
        d += emit_literal(&mut dst[d..], &src[next_emit..]);
    }

    // Check if compression was worthwhile
    if d >= src.len() - src.len() / 32 {
        return 0;
    }

    d
}

/// Encode a block using the Better S2 algorithm with dual hash tables
fn encode_block_better(dst: &mut [u8], src: &[u8]) -> usize {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return 0;
    }

    // Initialize the hash tables.
    const L_TABLE_BITS: u8 = 17; // Long hash matches
    const S_TABLE_BITS: u8 = 14; // Short hash matches
    const L_TABLE_SIZE: usize = 1 << L_TABLE_BITS;
    const S_TABLE_SIZE: usize = 1 << S_TABLE_BITS;

    let mut l_table = vec![0u32; L_TABLE_SIZE];
    let mut s_table = vec![0u32; S_TABLE_SIZE];

    // Bail if we can't compress to at least this.
    let dst_limit = src.len() - src.len() / 32 - 6;

    let s_limit = src.len() - INPUT_MARGIN;
    let mut next_emit = 0;
    let mut s = 1;
    let mut d = 0;
    let mut repeat = 0;

    if src.len() < 8 {
        // Too small for Better algorithm, fallback to simple literal
        return 0;
    }

    let mut cv = load64(src, s);

    'outer: loop {
        let mut candidate_l;
        let mut next_s;

        // Find a match
        loop {
            // Next src position to check
            next_s = s + (s - next_emit) / 128 + 1;
            if next_s > s_limit {
                break 'outer;
            }

            let hash_l = hash7(cv, L_TABLE_BITS) as usize;
            let hash_s = hash4(cv, S_TABLE_BITS) as usize;
            candidate_l = l_table[hash_l] as usize;
            let candidate_s = s_table[hash_s] as usize;
            l_table[hash_l] = s as u32;
            s_table[hash_s] = s as u32;

            let val_long = if candidate_l > 0 && candidate_l < src.len() - 8 {
                load64(src, candidate_l)
            } else {
                0
            };
            let val_short = if candidate_s > 0 && candidate_s < src.len() - 8 {
                load64(src, candidate_s)
            } else {
                0
            };

            // If long matches at least 8 bytes, use that.
            if cv == val_long {
                break;
            }
            if cv == val_short {
                candidate_l = candidate_s;
                break;
            }

            // Long likely matches 7, so take that.
            if (cv as u32) == (val_long as u32) {
                break;
            }

            // Check our short candidate
            if (cv as u32) == (val_short as u32) {
                // Try a long candidate at s+1
                let hash_l = hash7(cv >> 8, L_TABLE_BITS) as usize;
                let candidate_l_next = l_table[hash_l] as usize;
                l_table[hash_l] = (s + 1) as u32;
                if candidate_l_next > 0
                    && candidate_l_next < src.len() - 4
                    && (cv >> 8) as u32 == load32(src, candidate_l_next)
                {
                    s += 1;
                    candidate_l = candidate_l_next;
                    break;
                }
                // Use our short candidate.
                candidate_l = candidate_s;
                break;
            }

            if next_s + 8 <= src.len() {
                cv = load64(src, next_s);
            }
            s = next_s;
        }

        // Extend backwards
        while candidate_l > 0 && s > next_emit && src[candidate_l - 1] == src[s - 1] {
            candidate_l -= 1;
            s -= 1;
        }

        // Bail if we exceed the maximum size.
        if d + (s - next_emit) > dst_limit {
            return 0;
        }

        let base = s;
        let offset = base - candidate_l;

        // Extend the 4-byte match as long as possible.
        s += 4;
        let mut candidate = candidate_l + 4;
        while s < src.len() {
            if src.len() - s < 8 {
                if s < src.len() && candidate < src.len() && src[s] == src[candidate] {
                    s += 1;
                    candidate += 1;
                    continue;
                }
                break;
            }
            if candidate + 8 > src.len() {
                break;
            }
            let diff = load64(src, s) ^ load64(src, candidate);
            if diff != 0 {
                s += (diff.trailing_zeros() / 8) as usize;
                break;
            }
            s += 8;
            candidate += 8;
        }

        // Bail if the match is equal or worse to the encoding for large offsets.
        if offset > 65535 && s - base <= 5 && repeat != offset {
            s = next_s + 1;
            if s >= s_limit {
                break;
            }
            if s + 8 <= src.len() {
                cv = load64(src, s);
            }
            continue;
        }

        // Emit literal
        d += emit_literal(&mut dst[d..], &src[next_emit..base]);

        // Emit copy
        if repeat == offset {
            d += emit_repeat(&mut dst[d..], offset, s - base);
        } else {
            d += emit_copy(&mut dst[d..], offset, s - base);
            repeat = offset;
        }

        next_emit = s;
        if s >= s_limit {
            break;
        }

        if d > dst_limit {
            // Do we have space for more, if not bail.
            return 0;
        }

        // Index short & long
        let index0 = base + 1;
        let index1 = s - 2;

        if index0 < src.len() - 8 {
            let cv0 = load64(src, index0);
            l_table[hash7(cv0, L_TABLE_BITS) as usize] = index0 as u32;
            if index0 + 1 < src.len() - 8 {
                s_table[hash4(cv0 >> 8, S_TABLE_BITS) as usize] = (index0 + 1) as u32;
            }
        }

        if index1 > 0 && index1 < src.len() - 8 {
            let cv1 = load64(src, index1);
            l_table[hash7(cv1, L_TABLE_BITS) as usize] = index1 as u32;
            if index1 + 1 < src.len() - 8 {
                s_table[hash4(cv1 >> 8, S_TABLE_BITS) as usize] = (index1 + 1) as u32;
            }
        }

        // Index large values sparsely in between.
        let mut index0 = index0 + 1;
        let mut index2 = (index0 + index1).div_ceil(2);
        while index2 < index1 {
            if index0 < src.len() - 8 {
                l_table[hash7(load64(src, index0), L_TABLE_BITS) as usize] = index0 as u32;
            }
            if index2 < src.len() - 8 {
                l_table[hash7(load64(src, index2), L_TABLE_BITS) as usize] = index2 as u32;
            }
            index0 += 2;
            index2 += 2;
        }

        if s + 8 <= src.len() {
            cv = load64(src, s);
        }
    }

    // Emit remaining
    if next_emit < src.len() {
        // Bail if we exceed the maximum size.
        if d + src.len() - next_emit > dst_limit {
            return 0;
        }
        d += emit_literal(&mut dst[d..], &src[next_emit..]);
    }

    d
}

/// Emit a copy with potential repeat optimization
fn emit_copy(dst: &mut [u8], offset: usize, length: usize) -> usize {
    if offset >= 65536 {
        return emit_copy4(dst, offset, length);
    }

    // Offset no more than 2 bytes
    if length > 64 {
        let off;
        let remaining_length;
        if offset < 2048 {
            // emit 8 bytes as tagCopy1, rest as repeats.
            dst[0] = (((offset >> 8) << 5) | ((8 - 4) << 2) | TAG_COPY1 as usize) as u8;
            dst[1] = offset as u8;
            remaining_length = length - 8;
            off = 2;
        } else {
            // Emit a length 60 copy, encoded as 3 bytes.
            // Emit remaining as repeat value (minimum 4 bytes).
            dst[0] = ((59 << 2) | TAG_COPY2 as usize) as u8;
            dst[1] = offset as u8;
            dst[2] = (offset >> 8) as u8;
            remaining_length = length - 60;
            off = 3;
        }
        // Emit remaining as repeats, at least 4 bytes remain.
        return off + emit_repeat(&mut dst[off..], offset, remaining_length);
    }

    if length >= 12 || offset >= 2048 {
        // Emit the remaining copy, encoded as 3 bytes
        dst[0] = (((length - 1) << 2) | TAG_COPY2 as usize) as u8;
        dst[1] = offset as u8;
        dst[2] = (offset >> 8) as u8;
        return 3;
    }

    // Emit the remaining copy, encoded as 2 bytes
    dst[0] = (((offset >> 8) << 5) | ((length - 4) << 2) | TAG_COPY1 as usize) as u8;
    dst[1] = offset as u8;
    2
}

/// Calculate the size needed to encode a copy (without actually encoding)
/// Used by encode_block_best for scoring matches
fn emit_copy_size(offset: usize, length: usize) -> usize {
    if offset >= 65536 {
        let mut i = 0;
        let mut remaining = length;
        if remaining > 64 {
            remaining -= 64;
            if remaining >= 4 {
                // Emit remaining as repeats
                return 5 + emit_repeat_size(offset, remaining);
            }
            i = 5;
        }
        if remaining == 0 {
            return i;
        }
        return i + 5;
    }

    // Offset no more than 2 bytes
    if length > 64 {
        if offset < 2048 {
            // Emit 8 bytes, then rest as repeats
            return 2 + emit_repeat_size(offset, length - 8);
        }
        // Emit remaining as repeats, at least 4 bytes remain
        return 3 + emit_repeat_size(offset, length - 60);
    }
    if length >= 12 || offset >= 2048 {
        return 3;
    }
    // Emit the remaining copy, encoded as 2 bytes
    2
}

/// Calculate the size needed to encode a repeat (without actually encoding)
/// Used by encode_block_best for scoring matches
fn emit_repeat_size(offset: usize, length: usize) -> usize {
    // Repeat offset, make length cheaper
    if length <= 4 + 4 || (length < 8 + 4 && offset < 2048) {
        return 2;
    }
    if length < (1 << 8) + 4 + 4 {
        return 3;
    }
    if length < (1 << 16) + (1 << 8) + 4 {
        return 4;
    }
    const MAX_REPEAT: usize = (1 << 24) - 1;
    let remaining = length - ((1 << 16) - 4);
    let left = if remaining > MAX_REPEAT {
        remaining - MAX_REPEAT + 4
    } else {
        0
    };
    if left > 0 {
        return 5 + emit_repeat_size(offset, left);
    }
    5
}

/// Encode a block using the Best S2 algorithm - port of Go's encodeBlockBest
/// This finds the best matches by evaluating multiple candidates with a scoring system
fn encode_block_best(dst: &mut [u8], src: &[u8]) -> usize {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return 0;
    }

    // Hash table configuration - matches Go's constants
    const L_TABLE_BITS: u8 = 19;
    const S_TABLE_BITS: u8 = 16;
    const L_TABLE_SIZE: usize = 1 << L_TABLE_BITS;
    const S_TABLE_SIZE: usize = 1 << S_TABLE_BITS;
    const MAX_SKIP: usize = 64;

    // Hash tables store uint64: current position in lower 32 bits, previous in upper 32 bits
    let mut l_table = vec![0u64; L_TABLE_SIZE];
    let mut s_table = vec![0u64; S_TABLE_SIZE];

    let dst_limit = src.len() - 5;
    let s_limit = src.len() - INPUT_MARGIN;

    let mut next_emit = 0;
    let mut s = 1;
    let mut d = 0;
    let mut repeat = 1; // Start at 1 like Go

    if src.len() < 8 {
        return 0;
    }

    #[derive(Clone, Copy, Debug)]
    struct Match {
        offset: usize,
        s: usize,
        length: usize,
        score: i32,
        rep: bool,
    }

    impl Match {
        fn empty() -> Self {
            Match {
                offset: 0,
                s: 0,
                length: 0,
                score: 0,
                rep: false,
            }
        }
    }

    // Helper functions matching Go's implementation
    let get_cur = |x: u64| (x & 0xffffffff) as usize;
    let get_prev = |x: u64| (x >> 32) as usize;

    // Calculate the score for a match (higher is better)
    let score_match = |m: &Match, next_emit: usize| -> i32 {
        // Matches that are longer forward are penalized since we must emit literals before it
        let mut score = (m.length as i32) - (m.s as i32);
        if next_emit == m.s {
            // If we don't need to emit literals, we save 1 byte
            score += 1;
        }
        let offset = m.s - m.offset;
        if m.rep {
            score - emit_repeat_size(offset, m.length) as i32
        } else {
            score - emit_copy_size(offset, m.length) as i32
        }
    };

    // Try to match at a given position and return scored match
    let match_at = |offset: usize,
                    s: usize,
                    first: u32,
                    rep: bool,
                    best: &Match,
                    src: &[u8],
                    next_emit: usize|
     -> Match {
        if best.length != 0 && best.s.wrapping_sub(best.offset) == s.wrapping_sub(offset) {
            // Don't retest if we have the same offset
            return Match::empty();
        }
        if offset >= src.len() || load32(src, offset) != first {
            return Match::empty();
        }

        let mut m = Match {
            offset,
            s,
            length: 4 + offset,
            score: 0,
            rep,
        };

        let mut s_pos = s + 4;
        while s_pos < src.len() {
            if src.len() - s_pos < 8 {
                if src[s_pos] == src[m.length] {
                    m.length += 1;
                    s_pos += 1;
                    continue;
                }
                break;
            }
            let diff = load64(src, s_pos) ^ load64(src, m.length);
            if diff != 0 {
                m.length += (diff.trailing_zeros() / 8) as usize;
                break;
            }
            s_pos += 8;
            m.length += 8;
        }
        m.length -= offset;
        m.score = score_match(&m, next_emit);

        if m.score <= -(m.s as i32) {
            // Eliminate if no savings
            return Match::empty();
        }
        m
    };

    // Compare two matches and return the better one
    let best_of = |a: Match, b: Match| -> Match {
        if b.length == 0 {
            return a;
        }
        if a.length == 0 {
            return b;
        }
        let a_score = a.score + (b.s as i32);
        let b_score = b.score + (a.s as i32);
        if a_score >= b_score {
            a
        } else {
            b
        }
    };

    let mut cv = load64(src, s);

    loop {
        // Find best match by checking multiple candidates
        let mut best = Match::empty();

        loop {
            let mut next_s = (s - next_emit) / 256 + 1;
            if next_s > MAX_SKIP {
                next_s = s + MAX_SKIP;
            } else {
                next_s += s;
            }
            if next_s > s_limit {
                break;
            }

            let hash_l = hash8(cv, L_TABLE_BITS) as usize;
            let hash_s = hash4(cv, S_TABLE_BITS) as usize;
            let candidate_l = l_table[hash_l];
            let candidate_s = s_table[hash_s];

            // Check candidates from hash tables (current and previous positions)
            if s > 0 {
                best = best_of(
                    best,
                    match_at(
                        get_cur(candidate_l),
                        s,
                        cv as u32,
                        false,
                        &best,
                        src,
                        next_emit,
                    ),
                );
                best = best_of(
                    best,
                    match_at(
                        get_prev(candidate_l),
                        s,
                        cv as u32,
                        false,
                        &best,
                        src,
                        next_emit,
                    ),
                );
                best = best_of(
                    best,
                    match_at(
                        get_cur(candidate_s),
                        s,
                        cv as u32,
                        false,
                        &best,
                        src,
                        next_emit,
                    ),
                );
                best = best_of(
                    best,
                    match_at(
                        get_prev(candidate_s),
                        s,
                        cv as u32,
                        false,
                        &best,
                        src,
                        next_emit,
                    ),
                );
            }

            // Check repeat offset
            if repeat <= s && repeat > 0 {
                best = best_of(
                    best,
                    match_at(
                        s - repeat + 1,
                        s + 1,
                        (cv >> 8) as u32,
                        true,
                        &best,
                        src,
                        next_emit,
                    ),
                );
            }

            // If we found a match, check positions s+1 and s+2 for better matches
            if best.length > 0 {
                // Check s+1
                let hash_s = hash4(cv >> 8, S_TABLE_BITS) as usize;
                let next_short = s_table[hash_s];
                let s1 = s + 1;
                if s1 < src.len() - 8 {
                    let cv1 = load64(src, s1);
                    let hash_l = hash8(cv1, L_TABLE_BITS) as usize;
                    let next_long = l_table[hash_l];

                    best = best_of(
                        best,
                        match_at(
                            get_cur(next_short),
                            s1,
                            cv1 as u32,
                            false,
                            &best,
                            src,
                            next_emit,
                        ),
                    );
                    best = best_of(
                        best,
                        match_at(
                            get_prev(next_short),
                            s1,
                            cv1 as u32,
                            false,
                            &best,
                            src,
                            next_emit,
                        ),
                    );
                    best = best_of(
                        best,
                        match_at(
                            get_cur(next_long),
                            s1,
                            cv1 as u32,
                            false,
                            &best,
                            src,
                            next_emit,
                        ),
                    );
                    best = best_of(
                        best,
                        match_at(
                            get_prev(next_long),
                            s1,
                            cv1 as u32,
                            false,
                            &best,
                            src,
                            next_emit,
                        ),
                    );

                    // Check s+2
                    let hash_s2 = hash4(cv1 >> 8, S_TABLE_BITS) as usize;
                    let next_short2 = s_table[hash_s2];
                    let s2 = s + 2;
                    if s2 < src.len() - 8 {
                        let cv2 = load64(src, s2);
                        let hash_l2 = hash8(cv2, L_TABLE_BITS) as usize;
                        let next_long2 = l_table[hash_l2];

                        if repeat <= s2 && repeat > 0 {
                            best = best_of(
                                best,
                                match_at(s2 - repeat, s2, cv2 as u32, true, &best, src, next_emit),
                            );
                        }
                        best = best_of(
                            best,
                            match_at(
                                get_cur(next_short2),
                                s2,
                                cv2 as u32,
                                false,
                                &best,
                                src,
                                next_emit,
                            ),
                        );
                        best = best_of(
                            best,
                            match_at(
                                get_prev(next_short2),
                                s2,
                                cv2 as u32,
                                false,
                                &best,
                                src,
                                next_emit,
                            ),
                        );
                        best = best_of(
                            best,
                            match_at(
                                get_cur(next_long2),
                                s2,
                                cv2 as u32,
                                false,
                                &best,
                                src,
                                next_emit,
                            ),
                        );
                        best = best_of(
                            best,
                            match_at(
                                get_prev(next_long2),
                                s2,
                                cv2 as u32,
                                false,
                                &best,
                                src,
                                next_emit,
                            ),
                        );
                    }
                }

                // Check at end of match for even better matches
                const SKIP_BEGINNING: usize = 2;
                const SKIP_END: usize = 1;
                let s_at = best.s + best.length - SKIP_END;
                if s_at < s_limit {
                    let s_back = best.s + SKIP_BEGINNING - SKIP_END;
                    let back_l = best.length - SKIP_BEGINNING;
                    if s_back < src.len() - 8 && s_at < src.len() - 8 {
                        let cv_back = load64(src, s_back);
                        let next = l_table[hash8(load64(src, s_at), L_TABLE_BITS) as usize];

                        if get_cur(next) > back_l {
                            let check_at = get_cur(next) - back_l;
                            if check_at > 0 {
                                best = best_of(
                                    best,
                                    match_at(
                                        check_at,
                                        s_back,
                                        cv_back as u32,
                                        false,
                                        &best,
                                        src,
                                        next_emit,
                                    ),
                                );
                            }
                        }
                        if get_prev(next) > back_l {
                            let check_at = get_prev(next) - back_l;
                            if check_at > 0 {
                                best = best_of(
                                    best,
                                    match_at(
                                        check_at,
                                        s_back,
                                        cv_back as u32,
                                        false,
                                        &best,
                                        src,
                                        next_emit,
                                    ),
                                );
                            }
                        }
                    }
                }
            }

            // Update hash tables with current and previous positions
            l_table[hash_l] = (s as u64) | (candidate_l << 32);
            s_table[hash_s] = (s as u64) | (candidate_s << 32);

            if best.length > 0 {
                break;
            }

            if next_s >= src.len() - 8 {
                break;
            }
            cv = load64(src, next_s);
            s = next_s;
        }

        if best.length == 0 {
            break; // No more matches found
        }

        // Extend backwards if not a repeat
        s = best.s;
        if !best.rep {
            while best.offset > 0 && s > next_emit && src[best.offset - 1] == src[s - 1] {
                best.offset -= 1;
                best.length += 1;
                s -= 1;
            }
        }

        // Bail if we exceed the maximum size
        if d + (s - next_emit) > dst_limit {
            return 0;
        }

        let base = s;
        let offset = s - best.offset;
        s += best.length;

        // Bail if the match is equal or worse to the encoding for large offsets
        if offset > 65535 && s - base <= 5 && !best.rep {
            s = best.s + 1;
            if s >= s_limit {
                break;
            }
            if s < src.len() - 8 {
                cv = load64(src, s);
            }
            continue;
        }

        // Emit literal before the match
        d += emit_literal(&mut dst[d..], &src[next_emit..base]);

        // Emit the match
        if best.rep {
            if next_emit > 0 {
                d += emit_repeat(&mut dst[d..], offset, best.length);
            } else {
                // First match cannot be a repeat
                d += emit_copy(&mut dst[d..], offset, best.length);
            }
        } else {
            d += emit_copy(&mut dst[d..], offset, best.length);
        }
        repeat = offset;

        next_emit = s;
        if s >= s_limit {
            break;
        }

        if d > dst_limit {
            return 0;
        }

        // Fill hash tables for all positions between best.s and s
        let mut i = best.s + 1;
        while i < s {
            if i < src.len() - 8 {
                let cv0 = load64(src, i);
                let long0 = hash8(cv0, L_TABLE_BITS) as usize;
                let short0 = hash4(cv0, S_TABLE_BITS) as usize;
                l_table[long0] = (i as u64) | (l_table[long0] << 32);
                s_table[short0] = (i as u64) | (s_table[short0] << 32);
            }
            i += 1;
        }

        if s < src.len() - 8 {
            cv = load64(src, s);
        }
    }

    // Emit remaining literals
    if next_emit < src.len() {
        if d + src.len() - next_emit > dst_limit {
            return 0;
        }
        d += emit_literal(&mut dst[d..], &src[next_emit..]);
    }

    d
}

/// Encode a block using Snappy-compatible algorithm (no repeat offsets)
///
/// This is similar to encode_block but uses emit_copy_no_repeat instead of
/// the S2 repeat offset optimization, making it compatible with Snappy decoders.
fn encode_block_snappy(dst: &mut [u8], src: &[u8]) -> usize {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return 0;
    }

    // Hash table size - use 14 bits like Snappy
    const TABLE_BITS: u32 = 14;
    const TABLE_SIZE: usize = 1 << TABLE_BITS;
    let shift = 32 - TABLE_BITS;

    let mut table = vec![0u32; TABLE_SIZE];

    let s_limit = src.len() - INPUT_MARGIN;
    let mut next_emit = 0;
    let mut s = 1;
    let mut d = 0;
    #[allow(unused_assignments)]
    let mut repeat = 1;

    #[allow(unused_variables)]
    let cv = load64(src, s);

    'outer: loop {
        let mut candidate;
        let mut skip = 32;

        loop {
            let next_s = s + (skip >> 5);
            skip += 1;

            if next_s > s_limit {
                break 'outer;
            }

            let h = hash(&src[s..], shift);
            candidate = table[h] as usize;
            table[h] = s as u32;

            if load32(src, s) == load32(src, candidate) {
                break;
            }

            s = next_s;
        }

        // Extend backwards
        while candidate > 0 && s > next_emit && src[candidate - 1] == src[s - 1] {
            candidate -= 1;
            s -= 1;
        }

        // Emit literal
        if s > next_emit {
            d += emit_literal(&mut dst[d..], &src[next_emit..s]);
        }

        // Extend the match forward
        let base = s;
        repeat = base - candidate;
        s += 4;
        candidate += 4;

        while s <= src.len() - 8 {
            if load64(src, s) != load64(src, candidate) {
                let diff = (load64(src, s) ^ load64(src, candidate)).trailing_zeros() / 8;
                s += diff as usize;
                break;
            }
            s += 8;
            candidate += 8;
        }

        // Use emit_copy_no_repeat for Snappy compatibility (no repeat offset optimization)
        d += emit_copy_no_repeat(&mut dst[d..], repeat, s - base);
        next_emit = s;

        if s >= s_limit {
            break;
        }

        // Update hash table
        let h1 = hash(&src[s - 1..], shift);
        table[h1] = (s - 1) as u32;

        s += 1;
    }

    // Emit remaining
    if next_emit < src.len() {
        d += emit_literal(&mut dst[d..], &src[next_emit..]);
    }

    // Check if compression was worthwhile
    if d >= src.len() - src.len() / 32 {
        return 0;
    }

    d
}

/// Encode a block using dictionary for better compression
fn encode_block_dict(dst: &mut [u8], src: &[u8], dict: &Dict) -> usize {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return 0;
    }

    const TABLE_BITS: u32 = 14;
    const TABLE_SIZE: usize = 1 << TABLE_BITS;
    let shift = 32 - TABLE_BITS;

    // Initialize hash table
    let mut table = vec![0u32; TABLE_SIZE];

    // Pre-populate table with dictionary entries
    let dict_data = dict.data();
    let dict_len = dict_data.len();

    // Hash dictionary entries - mark as negative offsets to distinguish from source
    let mut i = 0;
    while i < dict_len.saturating_sub(8) {
        let cv = load64(dict_data, i);
        let h = hash6(cv, TABLE_BITS) as usize;
        // Store as negative offset: -(dict_len - i)
        // This allows us to distinguish dictionary matches from source matches
        table[h] = (dict_len - i) as u32 | 0x80000000;
        i += 1;
    }

    let s_limit = src.len() - INPUT_MARGIN;
    let mut next_emit = 0;
    let mut s = 1;
    let mut d = 0;
    let mut repeat = dict_len - dict.repeat(); // Initialize repeat from dictionary

    if src.len() < 8 {
        return 0;
    }

    let mut cv = load64(src, s);

    'outer: loop {
        let mut candidate_pos: usize;
        let mut next_s;
        let mut is_dict_match;

        // Find a match
        loop {
            // Next src position to check
            next_s = s + (s - next_emit) / 128 + 1;
            if next_s > s_limit {
                break 'outer;
            }

            let h = hash(&src[s..], shift);
            let table_val = table[h];
            table[h] = s as u32;

            // Check if candidate is from dictionary or source
            if table_val & 0x80000000 != 0 {
                // Dictionary match
                is_dict_match = true;
                let dict_offset = (table_val & 0x7fffffff) as usize;
                if dict_offset > dict_len {
                    s = next_s;
                    cv = load64(src, s);
                    continue;
                }
                candidate_pos = dict_len - dict_offset;

                // Verify match in dictionary
                if candidate_pos < dict_len.saturating_sub(8) {
                    let dict_cv = load64(dict_data, candidate_pos);
                    if cv == dict_cv {
                        break;
                    }
                }
            } else {
                // Source match
                is_dict_match = false;
                candidate_pos = table_val as usize;
                if candidate_pos > 0 && candidate_pos < s && candidate_pos < src.len() - 8 {
                    let candidate_cv = load64(src, candidate_pos);
                    if cv == candidate_cv {
                        break;
                    }
                }
            }

            s = next_s;
            cv = load64(src, s);
        }

        // Emit literals up to this match
        if s > next_emit {
            d += emit_literal(&mut dst[d..], &src[next_emit..s]);
        }

        // Extend the match
        let mut length;

        if is_dict_match {
            // Match is in dictionary
            // Calculate actual match length between dictionary and source
            length = 4;
            let dict_remain = dict_len - candidate_pos;
            let src_remain = src.len() - s;
            let max_len = dict_remain.min(src_remain);

            while length < max_len && dict_data[candidate_pos + length] == src[s + length] {
                length += 1;
            }

            // Calculate offset for dictionary match
            // When decoding: dict_start = dict.data().len() - offset + d
            // So: offset = dict.data().len() - dict_start + d
            // Where dict_start is candidate_pos and d is s (current output position)
            let offset = dict_len - candidate_pos + s;

            // Emit the copy operation
            if offset == repeat {
                d += emit_repeat(&mut dst[d..], offset, length);
            } else {
                d += emit_copy(&mut dst[d..], offset, length);
                repeat = offset;
            }
        } else {
            // Match is in source
            length = 4;
            let remain = src.len() - s;

            // Extend forward
            while length < remain && src[candidate_pos + length] == src[s + length] {
                length += 1;
            }

            let offset = s - candidate_pos;

            // Emit the copy operation
            if offset == repeat {
                d += emit_repeat(&mut dst[d..], offset, length);
            } else {
                d += emit_copy(&mut dst[d..], offset, length);
                repeat = offset;
            }
        }

        next_emit = s + length;
        s += length;

        // Check dst limit
        if d >= src.len() - src.len() / 32 - 6 {
            break;
        }

        if s >= s_limit {
            break;
        }

        // Update hash table with positions we skipped
        let mut prev_s = s - 1;
        while prev_s > next_emit && s - prev_s < 10 {
            let h = hash(&src[prev_s..], shift);
            table[h] = prev_s as u32;
            prev_s -= 1;
        }

        cv = load64(src, s);
    }

    // Emit remaining literals
    if next_emit < src.len() {
        d += emit_literal(&mut dst[d..], &src[next_emit..]);
    }

    // Check if compression was worthwhile
    if d >= src.len() - src.len() / 32 {
        return 0;
    }

    d
}

// Test helpers - expose internal functions for testing
#[cfg(test)]
pub mod test_helpers {
    use super::*;

    /// Test wrapper for emit_literal
    pub fn test_emit_literal(dst: &mut [u8], lit: &[u8]) -> usize {
        emit_literal(dst, lit)
    }

    /// Test wrapper for emit_copy
    pub fn test_emit_copy(dst: &mut [u8], offset: usize, length: usize) -> usize {
        emit_copy(dst, offset, length)
    }

    /// Match length function for testing
    /// Counts the number of matching bytes at the beginning of two slices
    pub fn test_match_len(a: &[u8], b: &[u8]) -> usize {
        let len = a.len().min(b.len());
        let mut n = 0;
        while n < len && a[n] == b[n] {
            n += 1;
        }
        n
    }
}
