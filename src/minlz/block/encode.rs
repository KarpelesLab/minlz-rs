// Copyright 2024 Karpeles Lab Inc.
// MinLZ block encoder. See ../mod.rs for licensing/attribution.
//
// NOTE: this is currently a correct but non-compressing encoder — it emits a
// MinLZ "stored" (literals-only) block. A matching encoder replaces it in a
// follow-up. The MinLZ spec leaves encoder output implementation-defined, so
// any valid token stream the reference decoder accepts is conformant.

use super::MAX_BLOCK_SIZE;
use crate::error::{Error, Result};
use alloc::vec::Vec;

/// Maximum number of bytes [`compress`] can produce for `src_len` input bytes,
/// or `None` if `src_len` exceeds [`MAX_BLOCK_SIZE`].
pub fn max_compressed_len(src_len: usize) -> Option<usize> {
    if src_len > MAX_BLOCK_SIZE {
        return None;
    }
    if src_len == 0 {
        return Some(1);
    }
    // Indicator byte + a zero length varint + the literals.
    Some(src_len + 2)
}

/// Compress `src` into a single MinLZ block.
///
/// Returns [`Error::TooLarge`] if `src` exceeds [`MAX_BLOCK_SIZE`]; larger
/// inputs must be split across blocks (the streaming API does this).
pub fn compress(src: &[u8]) -> Result<Vec<u8>> {
    if src.len() > MAX_BLOCK_SIZE {
        return Err(Error::TooLarge);
    }
    // The empty block is a single zero indicator byte.
    if src.is_empty() {
        return Ok(alloc::vec![0u8]);
    }
    // Stored block: indicator 0x00, uncompressed length 0 (one varint byte),
    // then the raw bytes as literals.
    let mut out = Vec::with_capacity(src.len() + 2);
    out.push(0);
    out.push(0);
    out.extend_from_slice(src);
    Ok(out)
}
