// Copyright 2024 Karpeles Lab Inc.
// Property-based tests using proptest

use minlz::{decode, encode, encode_best, encode_better, Reader, Writer};
use proptest::prelude::*;
use std::io::{Read, Write as _};

proptest! {
    #[test]
    fn prop_roundtrip_standard(data: Vec<u8>) {
        // Skip very large inputs
        prop_assume!(data.len() <= 100_000);

        let compressed = encode(&data);
        let decompressed = decode(&compressed).expect("decode failed");
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_roundtrip_better(data: Vec<u8>) {
        prop_assume!(data.len() <= 100_000);

        let compressed = encode_better(&data);
        let decompressed = decode(&compressed).expect("decode failed");
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_roundtrip_best(data: Vec<u8>) {
        prop_assume!(data.len() <= 50_000); // Smaller due to larger hash tables

        let compressed = encode_best(&data);
        let decompressed = decode(&compressed).expect("decode failed");
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_stream_roundtrip(data: Vec<u8>) {
        prop_assume!(data.len() <= 100_000);
        prop_assume!(!data.is_empty()); // Skip empty data for stream test

        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(&data).expect("write failed");
            writer.flush().expect("flush failed");
        }

        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        reader.read_to_end(&mut decompressed).expect("read failed");

        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_compression_improves_or_equal(data in prop::collection::vec(any::<u8>(), 100..1000)) {
        // For repeated data, compression should be better
        let repeated = data.repeat(10);
        let compressed = encode(&repeated);

        // Should compress to less than 50% for repeated data
        prop_assert!(compressed.len() < repeated.len() / 2);
    }

    #[test]
    fn prop_decode_never_panics(data: Vec<u8>) {
        prop_assume!(data.len() <= 10_000);

        // Decoding arbitrary data should never panic - just return error or success
        let _ = decode(&data);
    }

    #[test]
    fn prop_empty_and_small(size in 0usize..100) {
        let data = vec![b'x'; size];
        let compressed = encode(&data);
        let decompressed = decode(&compressed).expect("decode failed");
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn prop_all_same_byte(byte: u8, size in 1usize..10000) {
        let data = vec![byte; size];
        let compressed = encode(&data);
        let decompressed = decode(&compressed).expect("decode failed");
        prop_assert_eq!(data, decompressed);

        // Should achieve good compression on repeated bytes
        if size > 100 {
            prop_assert!(compressed.len() < size / 5);
        }
    }

    #[test]
    fn prop_compression_levels_compatible(data in prop::collection::vec(any::<u8>(), 100..1000)) {
        // All three levels should produce output that decompresses to the same data
        let compressed_std = encode(&data);
        let compressed_better = encode_better(&data);
        let compressed_best = encode_best(&data);

        let decompressed_std = decode(&compressed_std).expect("std decode failed");
        let decompressed_better = decode(&compressed_better).expect("better decode failed");
        let decompressed_best = decode(&compressed_best).expect("best decode failed");

        prop_assert_eq!(&data, &decompressed_std);
        prop_assert_eq!(&data, &decompressed_better);
        prop_assert_eq!(&data, &decompressed_best);
    }

    #[test]
    fn prop_stream_incremental_read(data: Vec<u8>, chunk_size in 1usize..1000) {
        prop_assume!(data.len() <= 10_000);
        prop_assume!(chunk_size > 0);

        // Compress with stream
        let mut compressed = Vec::new();
        {
            let mut writer = Writer::new(&mut compressed);
            writer.write_all(&data).expect("write failed");
            writer.flush().expect("flush failed");
        }

        // Read back incrementally
        let mut reader = Reader::new(&compressed[..]);
        let mut decompressed = Vec::new();
        let mut buffer = vec![0u8; chunk_size];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => decompressed.extend_from_slice(&buffer[..n]),
                Err(_) => break,
            }
        }

        prop_assert_eq!(data, decompressed);
    }
}
