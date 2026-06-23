// Copyright 2024 Karpeles Lab Inc.
// MinLZ block decoder. See ../mod.rs for licensing/attribution.
//
// This is a safe, bounds-checked port of the reference pure-Go decoder
// (minLZDecodeGo in github.com/minio/minlz/decode.go), which is authoritative
// for the format. The reference splits decoding into a fast wide-load loop and
// a checked tail loop; here a single checked loop covers both, trading some
// speed for safety and simplicity.

use super::{MAX_BLOCK_SIZE, MIN_COPY2_OFFSET, MIN_COPY3_OFFSET};
use crate::error::{Error, Result};
use crate::varint::decode_varint;
use alloc::vec;
use alloc::vec::Vec;

#[inline(always)]
fn u16le(src: &[u8], i: usize) -> usize {
    src[i] as usize | (src[i + 1] as usize) << 8
}

#[inline(always)]
fn u24le(src: &[u8], i: usize) -> usize {
    src[i] as usize | (src[i + 1] as usize) << 8 | (src[i + 2] as usize) << 16
}

#[inline(always)]
fn u32le(src: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([src[i], src[i + 1], src[i + 2], src[i + 3]])
}

/// Outcome of parsing a block header.
enum Header<'a> {
    /// Output is exactly these literal bytes (empty or `block size == 0`).
    Literals(&'a [u8]),
    /// A compressed token stream decoding to `dlen` bytes.
    Compressed { dlen: usize, tokens: &'a [u8] },
}

/// Parse the MinLZ block header (indicator byte + uncompressed length varint).
///
/// Snappy/S2 fallback (a non-zero indicator byte) is not implemented and is
/// reported as [`Error::Unsupported`].
fn parse_header(src: &[u8]) -> Result<Header<'_>> {
    if src.is_empty() {
        return Err(Error::Corrupt);
    }
    if src[0] != 0 {
        // Non-zero indicator selects seamless Snappy/S2 fallback, which this
        // decoder does not (yet) implement.
        return Err(Error::Unsupported);
    }
    let src = &src[1..];
    // A lone indicator byte (`[0x00]`) is the canonical empty block.
    if src.is_empty() {
        return Ok(Header::Literals(&[]));
    }
    let (v, hlen) = decode_varint(src)?;
    if v > MAX_BLOCK_SIZE as u64 {
        return Err(Error::TooLarge);
    }
    let src = &src[hlen..];
    if src.is_empty() {
        return Err(Error::Corrupt);
    }
    let dlen = v as usize;
    if dlen == 0 {
        // Block size 0: the remainder is emitted verbatim as literals.
        return Ok(Header::Literals(src));
    }
    // A compressed block may not be larger than its decompressed output.
    if dlen < src.len() {
        return Err(Error::Corrupt);
    }
    Ok(Header::Compressed { dlen, tokens: src })
}

/// Return the decompressed length of a MinLZ block without decoding it.
pub fn decompressed_len(src: &[u8]) -> Result<usize> {
    match parse_header(src)? {
        Header::Literals(lits) => Ok(lits.len()),
        Header::Compressed { dlen, .. } => Ok(dlen),
    }
}

/// Decompress a MinLZ block, returning the original bytes.
pub fn decompress(src: &[u8]) -> Result<Vec<u8>> {
    match parse_header(src)? {
        Header::Literals(lits) => Ok(lits.to_vec()),
        Header::Compressed { dlen, tokens } => {
            let mut dst = vec![0u8; dlen];
            decode_block(&mut dst, tokens)?;
            Ok(dst)
        }
    }
}

/// Decompress a MinLZ block into `dst`, which must have exactly the
/// decompressed length (see [`decompressed_len`]). Returns the number of bytes
/// written (always `dst.len()` on success).
pub fn decompress_into(dst: &mut [u8], src: &[u8]) -> Result<usize> {
    match parse_header(src)? {
        Header::Literals(lits) => {
            if dst.len() != lits.len() {
                return Err(Error::BufferTooSmall);
            }
            dst.copy_from_slice(lits);
            Ok(lits.len())
        }
        Header::Compressed { dlen, tokens } => {
            if dst.len() != dlen {
                return Err(Error::BufferTooSmall);
            }
            decode_block(dst, tokens)?;
            Ok(dlen)
        }
    }
}

/// Decode a MinLZ token stream into `dst`, which must already be sized to the
/// exact decompressed length. Mirrors `minLZDecodeGo`.
fn decode_block(dst: &mut [u8], src: &[u8]) -> Result<()> {
    let dlen = dst.len();
    let slen = src.len();
    let mut d = 0usize;
    let mut s = 0usize;
    let mut offset: usize = 1;

    while s < slen {
        let length: usize;
        match src[s] & 0x03 {
            0 => {
                // Literal(s) or repeat copy.
                let v = src[s];
                let x = v >> 3;
                let len = if x < 29 {
                    s += 1;
                    (x as usize) + 1
                } else if x == 29 {
                    if s + 2 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = 30 + src[s + 1] as usize;
                    s += 2;
                    l
                } else if x == 30 {
                    if s + 3 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = 30 + u16le(src, s + 1);
                    s += 3;
                    l
                } else {
                    if s + 4 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = 30 + u24le(src, s + 1);
                    s += 4;
                    l
                };
                if v & 4 != 0 {
                    // Repeat: copy `len` bytes from the previous offset.
                    length = len;
                } else {
                    // Literals: copy `len` bytes from the input.
                    if len > dlen - d || len > slen - s {
                        return Err(Error::Corrupt);
                    }
                    dst[d..d + len].copy_from_slice(&src[s..s + len]);
                    d += len;
                    s += len;
                    continue;
                }
            }
            1 => {
                // Copy1: 10-bit offset [1..1024], length [4..273].
                if s + 2 > slen {
                    return Err(Error::Corrupt);
                }
                let mut len = ((src[s] >> 2) & 15) as usize;
                offset = (u16le(src, s) >> 6) + 1;
                if len == 15 {
                    if s + 3 > slen {
                        return Err(Error::Corrupt);
                    }
                    len = src[s + 2] as usize + 18;
                    s += 3;
                } else {
                    len += 4;
                    s += 2;
                }
                length = len;
            }
            2 => {
                // Copy2: 16-bit offset (+64), length [4..].
                if s + 3 > slen {
                    return Err(Error::Corrupt);
                }
                let lc = (src[s] >> 2) as usize;
                offset = u16le(src, s + 1);
                let len = if lc <= 60 {
                    s += 3;
                    lc + 4
                } else if lc == 61 {
                    if s + 4 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = src[s + 3] as usize + 64;
                    s += 4;
                    l
                } else if lc == 62 {
                    if s + 5 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = u16le(src, s + 3) + 64;
                    s += 5;
                    l
                } else {
                    if s + 6 > slen {
                        return Err(Error::Corrupt);
                    }
                    let l = u24le(src, s + 3) + 64;
                    s += 6;
                    l
                };
                offset += MIN_COPY2_OFFSET;
                length = len;
            }
            _ => {
                // Tag 3: fused Copy2 or Copy3.
                if s + 4 > slen {
                    return Err(Error::Corrupt);
                }
                let val = u32le(src, s);
                let is_copy3 = val & 4 != 0;
                let mut lit_len = ((val >> 3) & 3) as usize;
                let len: usize;
                if !is_copy3 {
                    // Fused Copy2: 16-bit offset (+64), length [4..11], 1..4 lits.
                    len = 4 + ((val >> 5) & 7) as usize;
                    offset = ((val >> 8) & 0xffff) as usize + MIN_COPY2_OFFSET;
                    s += 3;
                    lit_len += 1;
                } else {
                    // Copy3: 21-bit offset (+65536), 0..3 fused lits.
                    let lc = (val >> 5) & 63;
                    offset = (val >> 11) as usize + MIN_COPY3_OFFSET;
                    if lc < 61 {
                        len = lc as usize + 4;
                        s += 4;
                    } else if lc == 61 {
                        if s + 5 > slen {
                            return Err(Error::Corrupt);
                        }
                        len = src[s + 4] as usize + 64;
                        s += 5;
                    } else if lc == 62 {
                        if s + 6 > slen {
                            return Err(Error::Corrupt);
                        }
                        len = u16le(src, s + 4) + 64;
                        s += 6;
                    } else {
                        if s + 7 > slen {
                            return Err(Error::Corrupt);
                        }
                        len = u24le(src, s + 4) + 64;
                        s += 7;
                    }
                }
                if lit_len > 0 {
                    // Fused literals are emitted *before* the copy.
                    if lit_len > dlen - d || s + lit_len > slen {
                        return Err(Error::Corrupt);
                    }
                    dst[d..d + lit_len].copy_from_slice(&src[s..s + lit_len]);
                    d += lit_len;
                    s += lit_len;
                }
                length = len;
            }
        }

        // Perform the copy (also the repeat path).
        if offset == 0 || d < offset || length > dlen - d {
            return Err(Error::Corrupt);
        }
        if offset >= length {
            // Source and destination do not overlap.
            dst.copy_within(d - offset..d - offset + length, d);
        } else {
            // Overlapping run (RLE): must copy forward, byte by byte.
            let from = d - offset;
            for i in 0..length {
                dst[d + i] = dst[from + i];
            }
        }
        d += length;
    }

    if d != dlen {
        return Err(Error::Corrupt);
    }
    Ok(())
}
