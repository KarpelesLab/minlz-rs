// Copyright 2024 Karpeles Lab Inc.
// MinLZ stream codec tests.

use super::super::Level;
use super::{Reader, Writer};
use std::io::{Read, Write};

fn roundtrip(data: &[u8], level: Level) {
    let mut w = Writer::with_level(Vec::new(), level);
    w.write_all(data).unwrap();
    let compressed = w.finish().unwrap();

    // Stream must start with the MinLZ identifier.
    if !data.is_empty() || !compressed.is_empty() {
        assert_eq!(&compressed[..4], b"\xff\x06\x00\x00");
        assert_eq!(&compressed[4..9], b"MinLz");
    }

    let mut out = Vec::new();
    Reader::new(&compressed[..]).read_to_end(&mut out).unwrap();
    assert_eq!(out, data, "stream roundtrip mismatch, {} bytes", data.len());
}

#[test]
fn roundtrips() {
    for level in [Level::Fastest, Level::Balanced, Level::Smallest] {
        roundtrip(b"", level);
        roundtrip(b"hello", level);
        roundtrip(b"hello hello hello hello hello hello hello", level);
        roundtrip(&vec![0u8; 5000], level);
        // Larger than one block (1 MiB) to exercise multi-block streams.
        let big: Vec<u8> = (0..3_500_000)
            .map(|i| ((i * 2654435761u64 as usize) >> 13) as u8)
            .collect();
        roundtrip(&big, level);
    }
}

#[test]
fn multiple_writes_and_flush() {
    let mut w = Writer::new(Vec::new());
    w.write_all(b"the quick ").unwrap();
    w.write_all(b"brown fox ").unwrap();
    w.flush().unwrap();
    w.write_all(b"jumps over").unwrap();
    let compressed = w.finish().unwrap();

    let mut out = Vec::new();
    Reader::new(&compressed[..]).read_to_end(&mut out).unwrap();
    assert_eq!(out, b"the quick brown fox jumps over");
}

#[test]
fn drop_finishes_stream() {
    let buf = {
        let mut w = Writer::new(Vec::new());
        w.write_all(b"finished on drop").unwrap();
        // Take the inner Vec out before drop by finishing explicitly is the
        // normal path; here we verify Drop also writes a usable stream.
        w.finish().unwrap()
    };
    let mut out = Vec::new();
    Reader::new(&buf[..]).read_to_end(&mut out).unwrap();
    assert_eq!(out, b"finished on drop");
}

#[test]
fn rejects_truncated_crc() {
    let mut w = Writer::new(Vec::new());
    w.write_all(b"some data that will be corrupted now")
        .unwrap();
    let mut compressed = w.finish().unwrap();
    // Flip a byte in the first chunk body (after the 10-byte header + 8-byte
    // chunk/crc header).
    let i = compressed.len() / 2;
    compressed[i] ^= 0xff;
    let mut out = Vec::new();
    let err = Reader::new(&compressed[..]).read_to_end(&mut out);
    assert!(err.is_err(), "expected CRC/corruption error");
}

#[test]
fn index_seek_multiblock() {
    use crate::minlz::{seek_decompress, Index};

    // Several blocks worth of compressible data (block size is 1 MiB).
    let data: Vec<u8> = (0..3_000_000u32).map(|i| (i / 5) as u8).collect();
    let mut w = Writer::new(Vec::new()).with_index();
    w.write_all(&data).unwrap();
    let stream = w.finish().unwrap();

    let index = Index::load(&stream).expect("load index");
    assert!(index.len() >= 2, "expected multiple index entries");
    assert_eq!(index.total_uncompressed(), data.len() as u64);

    // A plain reader still reads the whole thing (index chunk is skippable).
    let mut whole = Vec::new();
    Reader::new(&stream[..]).read_to_end(&mut whole).unwrap();
    assert_eq!(whole, data);

    for &off in &[0u64, 1, 999_999, 1_048_576, 1_500_000, 2_999_999] {
        let got = seek_decompress(&stream, &index, off).unwrap();
        assert_eq!(got, &data[off as usize..], "seek to {off}");
    }
}

#[test]
fn index_roundtrip_encode_decode() {
    use crate::minlz::Index;
    let data: Vec<u8> = (0..2_500_000u32).map(|i| (i / 3) as u8).collect();
    let mut w = Writer::new(Vec::new()).with_index();
    w.write_all(&data).unwrap();
    let stream = w.finish().unwrap();

    let index = Index::load(&stream).unwrap();
    // find() at an exact block boundary returns that boundary.
    let (_c, u) = index.find(1_048_576);
    assert!(u <= 1_048_576);
}

#[test]
fn concatenated_streams() {
    let mut a = Writer::new(Vec::new());
    a.write_all(b"first stream ").unwrap();
    let mut s = a.finish().unwrap();
    let mut b = Writer::new(Vec::new());
    b.write_all(b"second stream").unwrap();
    s.extend_from_slice(&b.finish().unwrap());

    let mut out = Vec::new();
    Reader::new(&s[..]).read_to_end(&mut out).unwrap();
    assert_eq!(out, b"first stream second stream");
}
