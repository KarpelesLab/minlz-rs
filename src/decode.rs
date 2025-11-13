// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::constants::*;
use crate::error::{Error, Result};
use crate::varint::decode_varint;

/// Decoder for S2 and Snappy compression
pub struct Decoder {
    /// Whether to allow Snappy format (no repeat offsets)
    #[allow(dead_code)]
    allow_snappy: bool,
}

impl Decoder {
    /// Create a new decoder that accepts both S2 and Snappy formats
    pub fn new() -> Self {
        Decoder { allow_snappy: true }
    }

    /// Create a decoder that only accepts S2 format
    pub fn new_s2_only() -> Self {
        Decoder {
            allow_snappy: false,
        }
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode returns the decoded form of src. The returned slice may be a sub-slice
/// of dst if dst was large enough to hold the entire decoded block.
/// Otherwise, a newly allocated Vec will be returned.
///
/// This function accepts both S2 and Snappy format.
/// The dst and src must not overlap. It is valid to pass an empty dst.
pub fn decode(src: &[u8]) -> Result<Vec<u8>> {
    let (dlen, header_len) = decode_len(src)?;

    let mut dst = vec![0u8; dlen];
    s2_decode(&mut dst, &src[header_len..])?;

    Ok(dst)
}

/// Decode Snappy format data
/// This is an alias for decode() since S2 decoder handles Snappy format
pub fn decode_snappy(src: &[u8]) -> Result<Vec<u8>> {
    decode(src)
}

/// Decode into a pre-allocated destination buffer.
/// Returns the number of bytes written to dst.
#[allow(dead_code)]
pub fn decode_into(dst: &mut [u8], src: &[u8]) -> Result<usize> {
    let (dlen, header_len) = decode_len(src)?;

    if dst.len() < dlen {
        return Err(Error::BufferTooSmall);
    }

    s2_decode(&mut dst[..dlen], &src[header_len..])?;

    Ok(dlen)
}

/// Returns the length of the decoded block and the number of bytes
/// that the length header occupied.
pub fn decode_len(src: &[u8]) -> Result<(usize, usize)> {
    let (v, n) = decode_varint(src)?;

    if v > 0xffffffff {
        return Err(Error::Corrupt);
    }

    // Check for 32-bit overflow on 32-bit systems
    #[cfg(target_pointer_width = "32")]
    {
        if v > 0x7fffffff {
            return Err(Error::TooLarge);
        }
    }

    Ok((v as usize, n))
}

/// Core S2 decoding function
fn s2_decode(dst: &mut [u8], src: &[u8]) -> Result<()> {
    let mut d = 0; // destination index
    let mut s = 0; // source index
    let mut offset = 0; // last copy offset

    // Fast path: process as long as we can read at least 5 bytes
    while s < src.len().saturating_sub(5) {
        let tag = src[s] & 0x03;

        match tag {
            TAG_LITERAL => {
                let (length, bytes_consumed) = decode_literal_length(&src[s..])?;
                s += bytes_consumed;

                // Bounds check
                if length > dst.len() - d || length > src.len() - s {
                    return Err(Error::Corrupt);
                }

                // Copy literal bytes
                dst[d..d + length].copy_from_slice(&src[s..s + length]);
                d += length;
                s += length;
            }
            TAG_COPY1 => {
                let (new_offset, length, bytes_consumed) = decode_copy1(&src[s..], offset)?;
                s += bytes_consumed;
                offset = new_offset;

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            TAG_COPY2 => {
                if s + 3 > src.len() {
                    return Err(Error::Corrupt);
                }

                offset = u16::from_le_bytes([src[s + 1], src[s + 2]]) as usize;
                let length = 1 + ((src[s] >> 2) as usize);
                s += 3;

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            TAG_COPY4 => {
                if s + 5 > src.len() {
                    return Err(Error::Corrupt);
                }

                offset =
                    u32::from_le_bytes([src[s + 1], src[s + 2], src[s + 3], src[s + 4]]) as usize;
                let length = 1 + ((src[s] >> 2) as usize);
                s += 5;

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            _ => unreachable!(),
        }
    }

    // Slow path: process remaining bytes with extra bounds checking
    while s < src.len() {
        let tag = src[s] & 0x03;

        match tag {
            TAG_LITERAL => {
                let (length, bytes_consumed) = decode_literal_length(&src[s..])?;
                s += bytes_consumed;

                // Bounds check
                if s > src.len() || length > dst.len() - d || length > src.len() - s {
                    return Err(Error::Corrupt);
                }

                // Copy literal bytes
                dst[d..d + length].copy_from_slice(&src[s..s + length]);
                d += length;
                s += length;
            }
            TAG_COPY1 => {
                let (new_offset, length, bytes_consumed) = decode_copy1(&src[s..], offset)?;
                s += bytes_consumed;

                if s > src.len() {
                    return Err(Error::Corrupt);
                }

                offset = new_offset;

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            TAG_COPY2 => {
                s += 3;
                if s > src.len() {
                    return Err(Error::Corrupt);
                }

                offset = u16::from_le_bytes([src[s - 2], src[s - 1]]) as usize;
                let length = 1 + ((src[s - 3] >> 2) as usize);

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            TAG_COPY4 => {
                s += 5;
                if s > src.len() {
                    return Err(Error::Corrupt);
                }

                offset =
                    u32::from_le_bytes([src[s - 4], src[s - 3], src[s - 2], src[s - 1]]) as usize;
                let length = 1 + ((src[s - 5] >> 2) as usize);

                // Bounds check
                if offset == 0 || d < offset || length > dst.len() - d {
                    return Err(Error::Corrupt);
                }

                // Copy from earlier in dst
                copy_within(dst, d, offset, length);
                d += length;
            }
            _ => unreachable!(),
        }
    }

    // Verify we decoded exactly the right amount
    if d != dst.len() {
        return Err(Error::Corrupt);
    }

    Ok(())
}

/// Decode the length of a literal chunk
/// Returns (length, bytes_consumed)
fn decode_literal_length(src: &[u8]) -> Result<(usize, usize)> {
    let x = (src[0] >> 2) as u32;

    match x {
        0..=59 => Ok((x as usize + 1, 1)),
        60 => {
            if src.len() < 2 {
                return Err(Error::Corrupt);
            }
            Ok((src[1] as usize + 1, 2))
        }
        61 => {
            if src.len() < 3 {
                return Err(Error::Corrupt);
            }
            let len = u16::from_le_bytes([src[1], src[2]]) as usize;
            Ok((len + 1, 3))
        }
        62 => {
            if src.len() < 4 {
                return Err(Error::Corrupt);
            }
            let len = u32::from_le_bytes([src[1], src[2], src[3], 0]) as usize;
            Ok((len + 1, 4))
        }
        63 => {
            if src.len() < 5 {
                return Err(Error::Corrupt);
            }
            let len = u32::from_le_bytes([src[1], src[2], src[3], src[4]]) as usize;
            Ok((len + 1, 5))
        }
        _ => Err(Error::Corrupt),
    }
}

/// Decode a COPY1 tag
/// Returns (offset, length, bytes_consumed)
fn decode_copy1(src: &[u8], last_offset: usize) -> Result<(usize, usize, usize)> {
    if src.len() < 2 {
        return Err(Error::Corrupt);
    }

    let toffset = ((src[0] as usize & 0xe0) << 3) | (src[1] as usize);
    let mut length = ((src[0] >> 2) & 0x7) as usize;

    if toffset == 0 {
        // Repeat offset - special encoding for length
        match length {
            5 => {
                if src.len() < 3 {
                    return Err(Error::Corrupt);
                }
                length = src[2] as usize + 4;
                Ok((last_offset, length + 4, 3))
            }
            6 => {
                if src.len() < 4 {
                    return Err(Error::Corrupt);
                }
                length = u16::from_le_bytes([src[2], src[3]]) as usize + (1 << 8);
                Ok((last_offset, length + 4, 4))
            }
            7 => {
                if src.len() < 5 {
                    return Err(Error::Corrupt);
                }
                length = u32::from_le_bytes([src[2], src[3], src[4], 0]) as usize + (1 << 16);
                Ok((last_offset, length + 4, 5))
            }
            _ => {
                // 0-4: use as-is
                Ok((last_offset, length + 4, 2))
            }
        }
    } else {
        Ok((toffset, length + 4, 2))
    }
}

/// Copy data within the same buffer, handling overlapping regions correctly.
/// This mimics the behavior of the Go implementation where overlapping copies
/// repeat the pattern.
#[inline]
fn copy_within(dst: &mut [u8], d: usize, offset: usize, length: usize) {
    let src_start = d - offset;

    // If no overlap, use the fast built-in copy
    if offset >= length {
        dst.copy_within(src_start..src_start + length, d);
    } else {
        // Overlapping copy - must be done byte by byte to get the repeating pattern
        for i in 0..length {
            dst[d + i] = dst[src_start + i];
        }
    }
}
