// Copyright 2024 Karpeles Lab Inc.
// MinLZ stream (framing) format. See ../mod.rs for licensing/attribution.

//! MinLZ stream format — a Snappy-derived framing of length-prefixed,
//! independently-checksummed blocks.
//!
//! [`Writer`] wraps a [`std::io::Write`] and frames data into CRC-32C-protected
//! MinLZ blocks; [`Reader`] wraps a [`std::io::Read`] and decodes such a stream.
//! The format is interoperable with the reference implementation at
//! `github.com/minio/minlz` (`.mz` files).

mod reader;
mod writer;

pub use reader::Reader;
pub use writer::Writer;

/// Stream identifier chunk prefix: type `0xff`, length 6, body `"MinLz"`.
/// One block-size indicator byte follows to complete the 10-byte header.
pub(crate) const STREAM_MAGIC: &[u8] = b"\xff\x06\x00\x00MinLz";

pub(crate) const CHUNK_STREAM_ID: u8 = 0xff;
pub(crate) const CHUNK_UNCOMPRESSED: u8 = 0x01;
pub(crate) const CHUNK_COMPRESSED: u8 = 0x02; // CRC of uncompressed data
pub(crate) const CHUNK_COMPRESSED_COMP_CRC: u8 = 0x03; // CRC of compressed data
pub(crate) const CHUNK_EOF: u8 = 0x20;
/// Chunk ids `<= 0x3f` that are not otherwise handled are reserved and
/// non-skippable; ids `> 0x3f` are skippable unless recognized.
pub(crate) const MAX_NON_SKIPPABLE_CHUNK: u8 = 0x3f;
/// User-defined non-skippable chunk range `[0xc0, 0xfd]`.
pub(crate) const MIN_USER_NON_SKIPPABLE: u8 = 0xc0;
pub(crate) const MAX_USER_NON_SKIPPABLE: u8 = 0xfd;

/// Default uncompressed block size used by [`Writer`]: 1 MiB (a power of two, so
/// the header block-size indicator is exact).
pub(crate) const DEFAULT_BLOCK_SIZE: usize = 1 << 20;

/// The 4-byte chunk length cannot exceed this (3 little-endian bytes).
pub(crate) const MAX_CHUNK_LEN: usize = 0xff_ffff;

#[cfg(test)]
mod tests;
