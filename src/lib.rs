// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#![cfg_attr(not(feature = "std"), no_std)]

//! # S2 Compression
//!
//! This library implements the S2 compression format, which is an extension of Snappy.
//! It is binary compatible with the Go implementation at github.com/klauspost/compress/s2
//!
//! S2 provides:
//! - Better compression than Snappy
//! - Faster decompression
//! - Block and stream formats
//! - CRC validation for streams
//!
//! ## Block Format Example
//!
//! ```rust
//! use minlz::{encode, decode};
//!
//! let data = b"Hello, World! This is a test of S2 compression.";
//! let compressed = encode(data);
//! let decompressed = decode(&compressed).expect("decompression failed");
//! assert_eq!(data, &decompressed[..]);
//! ```

#[macro_use]
extern crate alloc;

// Shared infrastructure (used by both codecs).
mod error;
pub use error::{Error, Result};

#[cfg(any(feature = "s2", feature = "minlz"))]
mod varint;

// CRC-32C (masked, Castagnoli) — the stream checksum shared by S2 and MinLZ.
#[cfg(any(feature = "s2", feature = "minlz"))]
pub mod crc;

// ----------------------------------------------------------------------------
// S2 codec (Snappy-compatible). Lives at the crate root for backwards
// compatibility; also re-exported under the `s2` module below.
// ----------------------------------------------------------------------------

#[cfg(feature = "s2")]
mod constants;
#[cfg(feature = "s2")]
mod decode;
#[cfg(feature = "s2")]
mod dict;
#[cfg(feature = "s2")]
mod encode;
#[cfg(feature = "s2")]
mod index;

#[cfg(all(feature = "std", feature = "s2"))]
mod reader;
#[cfg(all(feature = "std", feature = "s2"))]
mod writer;

#[cfg(feature = "concurrent")]
mod concurrent;

#[cfg(feature = "s2")]
pub use decode::{
    decode, decode_into, decode_len, decode_snappy, decode_with_dict, Decoder, MAX_DECODE_DST_SIZE,
};
#[cfg(feature = "s2")]
pub use dict::{
    make_dict, make_dict_manual, Dict, MAX_DICT_SIZE, MAX_DICT_SRC_OFFSET, MIN_DICT_SIZE,
};
#[cfg(feature = "s2")]
pub use encode::{
    encode, encode_best, encode_best_with_dict, encode_better, encode_better_with_dict,
    encode_snappy, encode_with_dict, max_encoded_len, Encoder,
};
#[cfg(feature = "s2")]
pub use index::Index;

#[cfg(all(feature = "std", feature = "s2"))]
pub use reader::Reader;
#[cfg(all(feature = "std", feature = "s2"))]
pub use writer::Writer;

#[cfg(feature = "concurrent")]
pub use concurrent::ConcurrentWriter;

/// The S2 codec (Snappy-compatible), namespaced.
///
/// Every item here is also re-exported at the crate root, so `minlz::encode`
/// and `minlz::s2::encode` are the same function. The `s2` module exists to
/// make the format explicit now that the crate also ships a [`minlz`] codec.
#[cfg(feature = "s2")]
pub mod s2 {
    pub use crate::decode::{
        decode, decode_into, decode_len, decode_snappy, decode_with_dict, Decoder,
        MAX_DECODE_DST_SIZE,
    };
    pub use crate::dict::{
        make_dict, make_dict_manual, Dict, MAX_DICT_SIZE, MAX_DICT_SRC_OFFSET, MIN_DICT_SIZE,
    };
    pub use crate::encode::{
        encode, encode_best, encode_best_with_dict, encode_better, encode_better_with_dict,
        encode_snappy, encode_with_dict, max_encoded_len, Encoder,
    };
    pub use crate::index::Index;

    #[cfg(feature = "std")]
    pub use crate::reader::Reader;
    #[cfg(feature = "std")]
    pub use crate::writer::Writer;

    #[cfg(feature = "concurrent")]
    pub use crate::concurrent::ConcurrentWriter;
}

// ----------------------------------------------------------------------------
// MinLZ codec (github.com/minio/minlz). A distinct format — see the module docs.
// ----------------------------------------------------------------------------

#[cfg(feature = "minlz")]
pub mod minlz;

#[cfg(all(test, feature = "std", feature = "s2"))]
mod tests;

#[cfg(all(test, feature = "std", feature = "s2"))]
mod snappy_tests;
