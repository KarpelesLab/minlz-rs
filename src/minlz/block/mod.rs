// Copyright 2024 Karpeles Lab Inc.
// MinLZ block format. See ../mod.rs for licensing/attribution.

//! MinLZ block (frame-less) codec.
//!
//! A block carries no checksum and is self-describing only in its uncompressed
//! length. Use the (future) streaming API for integrity-checked data.

mod decode;
mod encode;

pub use decode::{decompress, decompress_into, decompressed_len};
pub use encode::{compress, compress_into, compress_level, max_compressed_len};

use crate::error::{Error, Result};
use alloc::vec::Vec;

/// Maximum dictionary window size: the most recent 64 KiB are used.
pub const MAX_DICT_SIZE: usize = 65536;

/// A compression dictionary — shared context that primes the encoder and
/// decoder so small, similar blocks compress better.
///
/// **Crate-local format.** MinLZ's block dictionary format is unspecified (the
/// spec marks it "TBD" and the reference exposes no public dictionary API), so
/// [`compress_with_dict`]/[`decompress_with_dict`] are interoperable only with
/// this crate, not with `github.com/minio/minlz`. The same `Dict` must be used
/// to compress and decompress.
#[derive(Debug, Clone, Default)]
pub struct Dict {
    window: Vec<u8>,
}

impl Dict {
    /// Build a dictionary from `data`. Only the most recent [`MAX_DICT_SIZE`]
    /// bytes are retained.
    pub fn new(data: &[u8]) -> Self {
        let start = data.len().saturating_sub(MAX_DICT_SIZE);
        Dict {
            window: data[start..].to_vec(),
        }
    }

    /// The dictionary window length in bytes.
    pub fn len(&self) -> usize {
        self.window.len()
    }

    /// Whether the dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.window.is_empty()
    }
}

/// Compress `src` using `dict` as shared context. Produces a crate-local
/// dictionary block (see [`Dict`]); decode it with [`decompress_with_dict`] and
/// the same dictionary.
pub fn compress_with_dict(src: &[u8], dict: &Dict) -> Result<Vec<u8>> {
    if src.len() > MAX_BLOCK_SIZE {
        return Err(Error::TooLarge);
    }
    Ok(encode::compress_body_dict(src, &dict.window))
}

/// Decompress a dictionary block produced by [`compress_with_dict`] using the
/// same `dict`.
pub fn decompress_with_dict(block: &[u8], dict: &Dict) -> Result<Vec<u8>> {
    decode::decompress_with_prefix(block, &dict.window)
}

// Indicator-less block helpers used by the stream format (chunk type 0x02).
#[cfg(feature = "std")]
pub(crate) use decode::decompress_body;
#[cfg(feature = "std")]
pub(crate) use encode::compress_body;

/// Compression level for the MinLZ encoder.
///
/// All levels produce valid MinLZ blocks; they trade encode time for ratio.
/// (The decoder is the same regardless of level.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    /// Greedy single-table matching. Fastest, lowest ratio. Used by
    /// [`compress`].
    #[default]
    Fastest,
    /// Hash-chain search with lazy matching — better ratio, slower.
    Balanced,
    /// Deeper hash-chain search. Best ratio this encoder offers, slowest.
    Smallest,
}

/// Maximum uncompressed size of a single MinLZ block: 8 MiB.
pub const MAX_BLOCK_SIZE: usize = 8 << 20;

// Copy offset ranges (see SPEC.md §2.3–2.5).
pub(crate) const MAX_COPY1_OFFSET: usize = 1024;
pub(crate) const MIN_COPY2_OFFSET: usize = 64;
pub(crate) const MAX_COPY2_OFFSET: usize = MIN_COPY2_OFFSET + 65535;
pub(crate) const MIN_COPY3_OFFSET: usize = 65536;
pub(crate) const MAX_COPY3_OFFSET: usize = (2 << 20) + 65535;

// Copy1 carries a length of at most 273 bytes; longer same-offset matches are
// continued with a repeat.
pub(crate) const MAX_COPY1_LENGTH: usize = 273;

#[cfg(all(test, feature = "std"))]
mod tests;
