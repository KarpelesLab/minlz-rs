// Copyright 2024 Karpeles Lab Inc.
// MinLZ block format. See ../mod.rs for licensing/attribution.

//! MinLZ block (frame-less) codec.
//!
//! A block carries no checksum and is self-describing only in its uncompressed
//! length. Use the (future) streaming API for integrity-checked data.

mod decode;
mod encode;

pub use decode::{decompress, decompress_into, decompressed_len};
pub use encode::{compress, max_compressed_len};

/// Maximum uncompressed size of a single MinLZ block: 8 MiB.
pub const MAX_BLOCK_SIZE: usize = 8 << 20;

// Copy offset ranges used by the decoder (see SPEC.md §2.4–2.5). The matching
// encoder (added separately) introduces the remaining range constants.
pub(crate) const MIN_COPY2_OFFSET: usize = 64;
pub(crate) const MIN_COPY3_OFFSET: usize = 65536;

#[cfg(all(test, feature = "std"))]
mod tests;
