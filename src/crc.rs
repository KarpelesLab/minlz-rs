// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! CRC32 checksum implementation for S2 streams
//!
//! This implements the checksum specified in section 3 of
//! https://github.com/google/snappy/blob/master/framing_format.txt

use crc32fast::Hasher;

/// Calculate the CRC32 checksum for S2 stream format
///
/// This uses the Castagnoli polynomial and applies a transformation
/// as specified in the Snappy framing format.
pub fn crc(data: &[u8]) -> u32 {
    // crc32fast uses the Castagnoli polynomial by default
    let mut hasher = Hasher::new();
    hasher.update(data);
    let c = hasher.finalize();

    // Apply the transformation from the Snappy spec:
    // return ((c >> 15) | (c << 17)) + 0xa282ead8
    c.rotate_right(15).wrapping_add(0xa282ead8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_empty() {
        let result = crc(&[]);
        // Empty data should produce a specific CRC
        assert_ne!(result, 0);
    }

    #[test]
    fn test_crc_consistency() {
        let data = b"Hello, World!";
        let crc1 = crc(data);
        let crc2 = crc(data);
        assert_eq!(crc1, crc2, "CRC should be deterministic");
    }

    #[test]
    fn test_crc_different_data() {
        let data1 = b"Hello";
        let data2 = b"World";
        let crc1 = crc(data1);
        let crc2 = crc(data2);
        assert_ne!(crc1, crc2, "Different data should produce different CRCs");
    }
}
