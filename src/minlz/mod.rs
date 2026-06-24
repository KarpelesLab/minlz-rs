// Copyright 2024 Karpeles Lab Inc.
// MinLZ codec support.
//
// The MinLZ block/stream format and the reference implementation it is ported
// from are:
//
//     MinLZ — https://github.com/minio/minlz
//     Copyright 2025 MinIO Inc., licensed under the Apache License, Version 2.0.
//     Based on code from snappy-go (Google, BSD-style license).
//
// This is an independent Rust implementation of the MinLZ specification v1.0
// (see https://github.com/minio/minlz/blob/main/SPEC.md). The encoder output
// is implementation-defined and need not match the reference byte-for-byte; the
// decoder follows the reference decoder, which is authoritative on ambiguity.

//! # MinLZ
//!
//! [MinLZ](https://github.com/minio/minlz) is an LZ77-style, byte-aligned
//! compression format in the Snappy/S2 family (same lineage, different author
//! goals). It is a **distinct format** from S2 — MinLZ can decode Snappy/S2
//! data, but MinLZ output cannot be read by Snappy or S2 decoders.
//!
//! Compared to S2's block format, MinLZ reassigns the tag scheme and adds
//! repeat (last-offset) copies, fused literal+copy operations, three copy
//! offset ranges with minimum offsets, and an 8 MiB maximum block size.
//!
//! This module currently implements the **block** format
//! ([`compress`]/[`decompress`]). The streaming format, index, and dictionary
//! support are planned.
//!
//! ```rust
//! let data = b"MinLZ MinLZ MinLZ block compression";
//! let compressed = minlz::minlz::compress(data).expect("compress");
//! let restored = minlz::minlz::decompress(&compressed).expect("decompress");
//! assert_eq!(&restored[..], data);
//! ```

mod block;
mod index;

pub use block::{
    compress, compress_into, compress_level, compress_with_dict, decompress, decompress_into,
    decompress_with_dict, decompressed_len, max_compressed_len, Dict, Level, MAX_BLOCK_SIZE,
    MAX_DICT_SIZE,
};
pub use index::Index;

#[cfg(feature = "std")]
mod stream;
#[cfg(feature = "std")]
pub use stream::{seek_decompress, Reader, Writer};
