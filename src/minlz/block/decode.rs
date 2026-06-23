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
    /// A non-zero indicator byte: not a MinLZ block, decode as Snappy/S2.
    Fallback(&'a [u8]),
}

/// Parse the MinLZ block header (indicator byte + uncompressed length varint).
///
/// A non-zero indicator byte selects seamless Snappy/S2 fallback (the block is
/// an S2 or Snappy block); with the `s2` feature it is decoded by the S2
/// decoder, otherwise it reports [`Error::Unsupported`].
fn parse_header(src: &[u8]) -> Result<Header<'_>> {
    if src.is_empty() {
        return Err(Error::Corrupt);
    }
    if src[0] != 0 {
        return Ok(Header::Fallback(src));
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
///
/// For a Snappy/S2 fallback block this returns the S2 decoded length (requires
/// the `s2` feature).
pub fn decompressed_len(src: &[u8]) -> Result<usize> {
    match parse_header(src)? {
        Header::Literals(lits) => Ok(lits.len()),
        Header::Compressed { dlen, .. } => Ok(dlen),
        Header::Fallback(block) => fallback_len(block),
    }
}

/// Decompress a MinLZ block, returning the original bytes.
///
/// If the block is actually a Snappy or S2 block (non-zero indicator byte) it is
/// decoded transparently when the `s2` feature is enabled.
pub fn decompress(src: &[u8]) -> Result<Vec<u8>> {
    match parse_header(src)? {
        Header::Literals(lits) => Ok(lits.to_vec()),
        Header::Compressed { dlen, tokens } => {
            let mut dst = vec![0u8; dlen];
            decode_block(&mut dst, tokens)?;
            Ok(dst)
        }
        Header::Fallback(block) => fallback_decode(block),
    }
}

#[cfg(feature = "s2")]
fn fallback_decode(block: &[u8]) -> Result<Vec<u8>> {
    crate::decode::decode(block)
}

#[cfg(not(feature = "s2"))]
fn fallback_decode(_block: &[u8]) -> Result<Vec<u8>> {
    Err(Error::Unsupported)
}

#[cfg(feature = "s2")]
fn fallback_len(block: &[u8]) -> Result<usize> {
    crate::decode::decode_len(block).map(|(n, _)| n)
}

#[cfg(not(feature = "s2"))]
fn fallback_len(_block: &[u8]) -> Result<usize> {
    Err(Error::Unsupported)
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
        Header::Fallback(block) => {
            let decoded = fallback_decode(block)?;
            if dst.len() != decoded.len() {
                return Err(Error::BufferTooSmall);
            }
            dst.copy_from_slice(&decoded);
            Ok(decoded.len())
        }
    }
}

/// Decode a stream block body — `[uvarint(len)][tokens]` with no indicator byte
/// (stream chunk type 0x02/0x03) — into a freshly allocated `Vec`.
///
/// `max_block` bounds the decoded length. Empty blocks and blocks whose token
/// stream is larger than their output are rejected, matching the reference.
#[cfg(feature = "std")]
pub(crate) fn decompress_body(body: &[u8], max_block: usize) -> Result<Vec<u8>> {
    let (v, hlen) = decode_varint(body)?;
    if v > max_block as u64 {
        return Err(Error::TooLarge);
    }
    let tokens = &body[hlen..];
    let n = v as usize;
    if n == 0 || n < tokens.len() {
        return Err(Error::Corrupt);
    }
    let mut dst = vec![0u8; n];
    decode_block(&mut dst, tokens)?;
    Ok(dst)
}

/// Decompress a dictionary block body `[uvarint(len)][tokens]` (no indicator)
/// against `prefix`, returning the original bytes.
pub(crate) fn decompress_with_prefix(block: &[u8], prefix: &[u8]) -> Result<Vec<u8>> {
    let (v, hlen) = decode_varint(block)?;
    if v > MAX_BLOCK_SIZE as u64 {
        return Err(Error::TooLarge);
    }
    let n = v as usize;
    let tokens = &block[hlen..];
    let mut dst = vec![0u8; prefix.len() + n];
    dst[..prefix.len()].copy_from_slice(prefix);
    decode_block_from(&mut dst, tokens, prefix.len())?;
    Ok(dst[prefix.len()..].to_vec())
}

/// Decode a MinLZ token stream into `dst`, which must already be sized to the
/// exact decompressed length. Mirrors `minLZDecodeGo`.
fn decode_block(dst: &mut [u8], src: &[u8]) -> Result<()> {
    decode_block_from(dst, src, 0)
}

/// Decode a MinLZ token stream into `dst[d_start..]`, with `dst[..d_start]`
/// pre-filled (a dictionary prefix). Backreferences may reach into the prefix.
/// On success `d` reaches `dst.len()`.
pub(crate) fn decode_block_from(dst: &mut [u8], src: &[u8], d_start: usize) -> Result<()> {
    let dlen = dst.len();
    let slen = src.len();
    let mut d = d_start;
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
