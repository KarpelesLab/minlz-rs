// Copyright 2024 Karpeles Lab Inc.
// MinLZ block codec tests.

use super::{compress, decompress, decompress_into, decompressed_len, MAX_BLOCK_SIZE};
use crate::error::Error;
use alloc::vec;
use alloc::vec::Vec;

fn roundtrip(data: &[u8]) {
    let comp = compress(data).expect("compress");
    assert_eq!(decompressed_len(&comp).unwrap(), data.len());
    let got = decompress(&comp).expect("decompress");
    assert_eq!(got, data, "roundtrip mismatch for {} bytes", data.len());
}

#[test]
fn empty_block() {
    let comp = compress(b"").unwrap();
    assert_eq!(comp, vec![0u8]); // canonical empty block
    assert_eq!(decompressed_len(&comp).unwrap(), 0);
    assert_eq!(decompress(&comp).unwrap(), Vec::<u8>::new());
}

#[test]
fn roundtrips() {
    roundtrip(b"a");
    roundtrip(b"hello world");
    roundtrip(b"the quick brown fox jumps over the lazy dog");
    roundtrip(&[0u8; 1000]);
    let pattern: Vec<u8> = (0..4096).map(|i| (i * 31 % 256) as u8).collect();
    roundtrip(&pattern);
}

#[test]
fn decompress_into_exact() {
    let data = b"some bytes to store and read back";
    let comp = compress(data).unwrap();
    let mut buf = vec![0u8; data.len()];
    let n = decompress_into(&mut buf, &comp).unwrap();
    assert_eq!(n, data.len());
    assert_eq!(&buf, data);

    // Wrong-sized destination is rejected.
    let mut small = vec![0u8; data.len() - 1];
    assert_eq!(
        decompress_into(&mut small, &comp),
        Err(Error::BufferTooSmall)
    );
}

#[test]
fn rejects_oversized_input() {
    // We cannot allocate 8 MiB+1 cheaply here; just check the length guard.
    assert_eq!(super::max_compressed_len(MAX_BLOCK_SIZE + 1), None);
    assert_eq!(super::max_compressed_len(0), Some(1));
    assert_eq!(super::max_compressed_len(100), Some(102));
}

#[test]
fn rejects_corrupt() {
    assert_eq!(decompress(&[]), Err(Error::Corrupt));
    // Indicator + varint len but no token bytes.
    assert_eq!(decompress(&[0u8, 5u8]), Err(Error::Corrupt));
    // Non-zero indicator -> Snappy/S2 fallback (unimplemented).
    assert_eq!(decompress(&[1u8, 2u8, 3u8]), Err(Error::Unsupported));
}
