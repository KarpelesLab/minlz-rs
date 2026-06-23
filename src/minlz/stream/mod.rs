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

use crate::minlz::{Index, MAX_BLOCK_SIZE};
use alloc::vec::Vec;
use std::io::{self, Read};

/// Decompress a complete MinLZ stream starting at uncompressed byte
/// `uncompressed_offset`, using `index` to seek directly to the containing
/// block rather than decoding from the start.
///
/// `stream` must be the entire stream bytes. Returns everything from
/// `uncompressed_offset` to the end of the stream.
///
/// ```
/// use std::io::Write;
/// let data: Vec<u8> = (0..200_000u32).map(|i| (i / 7) as u8).collect();
/// let mut w = minlz::minlz::Writer::new(Vec::new()).with_index();
/// w.write_all(&data).unwrap();
/// let stream = w.finish().unwrap();
///
/// let index = minlz::minlz::Index::load(&stream).unwrap();
/// let tail = minlz::minlz::seek_decompress(&stream, &index, 150_000).unwrap();
/// assert_eq!(tail, &data[150_000..]);
/// ```
pub fn seek_decompress(
    stream: &[u8],
    index: &Index,
    uncompressed_offset: u64,
) -> io::Result<Vec<u8>> {
    // Recover the block size from the stream header (10-byte identifier).
    let max_block = if stream.len() >= 10 && stream[0] == CHUNK_STREAM_ID {
        let indicator = (stream[9] & 0x0f) as u32;
        (1usize << (indicator + 10)).min(MAX_BLOCK_SIZE)
    } else {
        MAX_BLOCK_SIZE
    };

    let (comp_off, block_uoff) = index.find(uncompressed_offset);
    let comp_off = comp_off as usize;
    if comp_off > stream.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "index offset past end of stream",
        ));
    }

    let mut out = Vec::new();
    Reader::new_midstream(&stream[comp_off..], max_block).read_to_end(&mut out)?;

    // Drop the bytes between the block boundary and the requested offset.
    let skip = (uncompressed_offset - block_uoff) as usize;
    if skip > out.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "offset past decoded data",
        ));
    }
    out.drain(..skip);
    Ok(out)
}

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
