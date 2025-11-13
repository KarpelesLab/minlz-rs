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
mod decode;
mod encode;
mod error;
mod varint;

pub use decode::{decode, decode_len, Decoder};
pub use encode::{encode, encode_better, encode_best, max_encoded_len, Encoder};
pub use error::{Error, Result};

#[cfg(test)]
mod tests;
