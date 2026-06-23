// Copyright 2024 Karpeles Lab Inc.
// MinLZ block codec tests.

use super::{
    compress, compress_level, compress_with_dict, decompress, decompress_into,
    decompress_with_dict, decompressed_len, Dict, Level, MAX_BLOCK_SIZE,
};
use crate::error::Error;
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

/// Small deterministic PRNG (xorshift64*) — keeps the stress test reproducible
/// and dependency-free.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Build inputs that stress copy offset/length boundaries: periodic data at
/// telling periods, RLE runs, random noise, and concatenations.
fn craft(rng: &mut Rng, kind: usize) -> Vec<u8> {
    match kind % 5 {
        0 => (0..rng.below(900)).map(|_| rng.next() as u8).collect(),
        1 => {
            // Periodic -> forces matches at a chosen offset.
            let periods = [1, 2, 3, 17, 63, 64, 65, 255, 256, 1023, 1024, 1025, 4096];
            let p = periods[rng.below(periods.len())];
            let chunk: Vec<u8> = (0..p).map(|_| rng.next() as u8).collect();
            let n = p + rng.below(p * 3 + 300);
            (0..n).map(|i| chunk[i % p]).collect()
        }
        2 => {
            // Match-length boundaries around 18 / 64 / 273.
            let lens = [4, 17, 18, 19, 63, 64, 65, 272, 273, 274, 600];
            let l = lens[rng.below(lens.len())];
            let unit: Vec<u8> = (0..4 + rng.below(36)).map(|_| rng.next() as u8).collect();
            let body: Vec<u8> = (0..l).map(|i| unit[i % unit.len()]).collect();
            let pre: Vec<u8> = (0..rng.below(40)).map(|_| rng.next() as u8).collect();
            [pre.clone(), body.clone(), body, pre].concat()
        }
        3 => vec![rng.next() as u8; 1 + rng.below(5000)],
        _ => {
            // Long-distance duplicate -> copy3 (offset >= 65536).
            let head: Vec<u8> = (0..10 + rng.below(2000))
                .map(|_| rng.next() as u8)
                .collect();
            let gap = vec![0u8; 66000 + rng.below(8000)];
            [head.clone(), gap, head].concat()
        }
    }
}

#[test]
fn all_levels_roundtrip() {
    let mut rng = Rng(0xcafef00dd00dfeed);
    for kind in 0..40 {
        let data = craft(&mut rng, kind);
        let mut sizes = Vec::new();
        for level in [Level::Fastest, Level::Balanced, Level::Smallest] {
            let comp = compress_level(&data, level).expect("compress");
            let got = decompress(&comp).expect("decompress");
            assert_eq!(got, data, "level {level:?} mismatch, len {}", data.len());
            sizes.push(comp.len());
        }
        // Higher levels should never be dramatically worse than Fastest; on a
        // clearly compressible input they should do at least as well.
        if data.len() > 2000 && sizes[0] * 2 < data.len() {
            assert!(
                sizes[2] <= sizes[0],
                "Smallest ({}) worse than Fastest ({}) on len {}",
                sizes[2],
                sizes[0],
                data.len()
            );
        }
    }
}

#[test]
fn dict_roundtrip() {
    let dict = Dict::new(b"the quick brown fox jumps over the lazy dog, repeatedly and often. ");
    let mut rng = Rng(0xd1c7_0000_1234_5678);
    for kind in 0..30 {
        let data = craft(&mut rng, kind);
        let comp = compress_with_dict(&data, &dict).expect("compress_with_dict");
        let got = decompress_with_dict(&comp, &dict).expect("decompress_with_dict");
        assert_eq!(got, data, "dict roundtrip mismatch, len {}", data.len());
    }
}

#[test]
fn dict_helps_similar_data() {
    // A block very similar to the dictionary should compress much better with
    // the dictionary than without.
    let base = b"GET /api/v1/users/12345/profile HTTP/1.1\r\nHost: example.com\r\n\r\n";
    let dict = Dict::new(base);
    let msg = b"GET /api/v1/users/67890/profile HTTP/1.1\r\nHost: example.com\r\n\r\n";

    let with = compress_with_dict(msg, &dict).unwrap();
    let without = compress(msg).unwrap();
    assert_eq!(decompress_with_dict(&with, &dict).unwrap(), msg);
    assert!(
        with.len() < without.len(),
        "dict ({}) should beat no-dict ({})",
        with.len(),
        without.len()
    );
}

#[test]
fn dict_empty_and_edge() {
    let dict = Dict::new(b"some shared dictionary context bytes here");
    for data in [&b""[..], b"x", b"short", &[0u8; 500][..]] {
        let comp = compress_with_dict(data, &dict).unwrap();
        assert_eq!(decompress_with_dict(&comp, &dict).unwrap(), data);
    }
    // An empty dictionary behaves like ordinary (indicator-less) compression.
    let empty = Dict::new(b"");
    assert!(empty.is_empty());
    let data = b"hello hello hello hello world world world";
    let comp = compress_with_dict(data, &empty).unwrap();
    assert_eq!(decompress_with_dict(&comp, &empty).unwrap(), data);
}

#[test]
fn roundtrip_stress() {
    let mut rng = Rng(0x1234_5678_9abc_def0);
    for i in 0..400 {
        let data = craft(&mut rng, i);
        let comp = compress(&data).expect("compress");
        // Never larger than the documented bound.
        assert!(comp.len() <= super::max_compressed_len(data.len()).unwrap());
        let got = decompress(&comp).expect("decompress");
        assert_eq!(got, data, "stress mismatch at iter {i}, len {}", data.len());
    }
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
}

#[cfg(feature = "s2")]
#[test]
fn snappy_s2_fallback() {
    // A non-zero indicator byte means the block is actually an S2/Snappy block;
    // the MinLZ decoder transparently falls back to the S2 decoder.
    let data = b"fallback: an S2 block decoded through the MinLZ entry point";
    let s2_block = crate::encode::encode(data);
    assert_ne!(
        s2_block[0], 0,
        "S2 block should not start with a 0 indicator"
    );
    assert_eq!(decompress(&s2_block).unwrap(), data);
    assert_eq!(decompressed_len(&s2_block).unwrap(), data.len());
}
