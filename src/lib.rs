// Copyright 2024 Karpeles Lab Inc.
// Based on the S2 compression format by Klaus Post
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

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

mod constants;
mod crc;
mod decode;
mod dict;
mod encode;
mod error;
mod index;
mod reader;
mod varint;
mod writer;

#[cfg(feature = "concurrent")]
mod concurrent;

pub use decode::{decode, decode_len, decode_snappy, decode_with_dict, Decoder};
pub use dict::{make_dict, make_dict_manual, Dict, MAX_DICT_SIZE, MAX_DICT_SRC_OFFSET, MIN_DICT_SIZE};
pub use encode::{
    encode, encode_best, encode_best_with_dict, encode_better, encode_better_with_dict,
    encode_with_dict, max_encoded_len, Encoder,
};
pub use error::{Error, Result};
pub use index::Index;
pub use reader::Reader;
pub use writer::Writer;

#[cfg(feature = "concurrent")]
pub use concurrent::ConcurrentWriter;

#[cfg(test)]
mod tests;
