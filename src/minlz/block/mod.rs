// Copyright 2024 Karpeles Lab Inc.
// MinLZ block format. See ../mod.rs for licensing/attribution.

//! MinLZ block (frame-less) codec.
//!
//! A block carries no checksum and is self-describing only in its uncompressed
//! length. Use the (future) streaming API for integrity-checked data.

mod decode;
mod encode;

pub use decode::{decompress, decompress_into, decompressed_len};
pub use encode::{compress, compress_level, max_compressed_len};

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
