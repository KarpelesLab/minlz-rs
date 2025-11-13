// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::error::{Error, Result};

/// Decode a varint from the beginning of the slice.
/// Returns (value, bytes_read)
pub fn decode_varint(src: &[u8]) -> Result<(u64, usize)> {
    let mut value: u64 = 0;
    let mut shift = 0;

    for (i, &byte) in src.iter().enumerate() {
        if i >= 10 {
            return Err(Error::Corrupt);
        }

        if byte < 0x80 {
            if i == 9 && byte > 1 {
                return Err(Error::Corrupt);
            }
            value |= (byte as u64) << shift;
            return Ok((value, i + 1));
        }

        value |= ((byte & 0x7f) as u64) << shift;
        shift += 7;
    }

    Err(Error::Corrupt)
}

/// Encode a varint into the buffer.
/// Returns the number of bytes written.
pub fn encode_varint(dst: &mut [u8], mut value: u64) -> usize {
    let mut i = 0;

    while value >= 0x80 {
        dst[i] = (value as u8) | 0x80;
        value >>= 7;
        i += 1;
    }

    dst[i] = value as u8;
    i + 1
}

/// Returns the number of bytes needed to encode this value as a varint
pub fn varint_size(mut value: u64) -> usize {
    let mut n = 1;
    while value >= 0x80 {
        value >>= 7;
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_roundtrip() {
        let test_values = vec![0, 1, 127, 128, 255, 256, 65535, 65536, 0xffffffff];

        for &val in &test_values {
            let mut buf = [0u8; 10];
            let n = encode_varint(&mut buf, val);
            let (decoded, bytes_read) = decode_varint(&buf).unwrap();
            assert_eq!(val, decoded);
            assert_eq!(n, bytes_read);
        }
    }
}
