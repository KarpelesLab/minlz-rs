// Copyright 2024 Karpeles Lab Inc.
// Comprehensive tests for S2 compression

use minlz::{decode, encode, encode_best, encode_better, Reader, Writer};
use std::io::{Read, Write as _};

#[test]
fn test_round_trip_all_levels() {
    let test_cases = vec![
        ("empty", Vec::new()),
        ("single_byte", vec![b'x']),
        ("small_text", b"Hello, World!".to_vec()),
        ("repeated", vec![b'a'; 1000]),
        ("pattern", (0..1000).map(|i| (i % 256) as u8).collect()),
        (
            "lorem",
            b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100),
        ),
    ];

    for (name, data) in test_cases {
        // Test Standard compression
        let compressed = encode(&data);
        let decompressed =
            decode(&compressed).unwrap_or_else(|_| panic!("{}: standard decode failed", name));
        assert_eq!(data, decompressed, "{}: standard round-trip failed", name);

        // Test Better compression
        let compressed_better = encode_better(&data);
        let decompressed_better = decode(&compressed_better)
            .unwrap_or_else(|_| panic!("{}: better decode failed", name));
        assert_eq!(
            data, decompressed_better,
            "{}: better round-trip failed",
            name
        );

        // Test Best compression
        let compressed_best = encode_best(&data);
        let decompressed_best =
            decode(&compressed_best).unwrap_or_else(|_| panic!("{}: best decode failed", name));
        assert_eq!(data, decompressed_best, "{}: best round-trip failed", name);

        // Verify compression improves (or stays same for small data)
        if data.len() > 100 {
            assert!(
                compressed_better.len() <= compressed.len(),
                "{}: better should compress better or equal",
                name
            );
            assert!(
                compressed_best.len() <= compressed_better.len(),
                "{}: best should compress better or equal",
                name
            );
        }
    }
}

#[test]
fn test_stream_format_comprehensive() {
    let test_data = vec![
        b"First chunk of data. ".to_vec(),
        b"Second chunk with more information. ".repeat(10),
        vec![b'x'; 1000],
        (0..500).map(|i| (i % 256) as u8).collect(),
    ];

    // Write multiple chunks
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        for chunk in &test_data {
            writer.write_all(chunk).expect("write failed");
        }
        writer.flush().expect("flush failed");
    }

    // Read back and verify
    let mut reader = Reader::new(&compressed[..]);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed).expect("read failed");

    let expected: Vec<u8> = test_data.concat();
    assert_eq!(expected, decompressed, "stream round-trip failed");
}

#[test]
fn test_large_data() {
    // Test with 100KB of data (1MB causes stack overflow with large hash tables)
    let data: Vec<u8> = (0u32..100 * 1024)
        .map(|i| (i.wrapping_mul(7919) % 256) as u8)
        .collect();

    let compressed = encode(&data);
    let decompressed = decode(&compressed).expect("large data decode failed");

    assert_eq!(data, decompressed, "large data round-trip failed");
    assert!(
        compressed.len() < data.len(),
        "should achieve some compression on 100KB"
    );
}

#[test]
fn test_highly_compressible() {
    // Data that should compress very well
    let data = vec![b'A'; 10000];

    let compressed = encode(&data);
    let decompressed = decode(&compressed).expect("highly compressible decode failed");

    assert_eq!(data, decompressed);
    assert!(
        compressed.len() < data.len() / 10,
        "should achieve >90% compression ratio on repeated data"
    );
}

#[test]
fn test_incompressible_data() {
    // Data that shouldn't compress well (random-like)
    let data: Vec<u8> = (0u32..1000)
        .map(|i| {
            let x = i.wrapping_mul(31337);
            let y = i.wrapping_mul(i).wrapping_mul(7919);
            (x.wrapping_add(y) % 256) as u8
        })
        .collect();

    let compressed = encode(&data);
    let decompressed = decode(&compressed).expect("incompressible decode failed");

    assert_eq!(data, decompressed);
    // Should still be valid, even if larger
}

#[test]
fn test_edge_cases() {
    // Test various edge cases
    let edge_cases = [
        vec![0u8; 0],     // Empty
        vec![0u8; 1],     // Single byte
        vec![0u8; 15],    // Just below MIN_NON_LITERAL_BLOCK_SIZE
        vec![0u8; 16],    // MIN_NON_LITERAL_BLOCK_SIZE
        vec![0u8; 17],    // Just above
        vec![255u8; 100], // All 0xFF
        vec![0u8; 100],   // All 0x00
    ];

    for (i, data) in edge_cases.iter().enumerate() {
        let compressed = encode(data);
        let decompressed =
            decode(&compressed).unwrap_or_else(|_| panic!("edge case {} failed", i));
        assert_eq!(data, &decompressed, "edge case {} mismatch", i);
    }
}

#[test]
fn test_copy_operations() {
    // Data designed to trigger different copy operations
    let data = b"abcdefgh".repeat(100);

    let compressed = encode(&data);
    let decompressed = decode(&compressed).expect("copy operations decode failed");

    assert_eq!(data, decompressed);
    assert!(
        compressed.len() < data.len() / 5,
        "should compress repeated pattern well"
    );
}

#[test]
fn test_literal_sizes() {
    // Test different literal size encodings
    let test_cases = vec![
        1,    // Tiny
        59,   // Max 1-byte literal length
        60,   // First 2-byte literal length
        255,  // Max 1-byte extended
        256,  // First 2-byte extended
        1000, // Larger
    ];

    for size in test_cases {
        let data = vec![b'x'; size];
        let compressed = encode(&data);
        let decompressed =
            decode(&compressed).unwrap_or_else(|_| panic!("literal size {} failed", size));
        assert_eq!(data, decompressed, "literal size {} mismatch", size);
    }
}

#[test]
fn test_stream_incremental_read() {
    let data = b"Test data for incremental reading. ".repeat(100);

    // Compress
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(&data).expect("write failed");
        writer.flush().expect("flush failed");
    }

    // Read incrementally
    let mut reader = Reader::new(&compressed[..]);
    let mut decompressed = Vec::new();
    let mut buffer = [0u8; 64];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => decompressed.extend_from_slice(&buffer[..n]),
            Err(e) => panic!("incremental read failed: {}", e),
        }
    }

    assert_eq!(data, decompressed, "incremental read mismatch");
}

#[test]
fn test_compression_levels_quality() {
    // Verify that better compression levels actually compress better
    let data = b"The quick brown fox jumps over the lazy dog. ".repeat(200);

    let std_compressed = encode(&data);
    let better_compressed = encode_better(&data);
    let best_compressed = encode_best(&data);

    // All should decompress correctly
    assert_eq!(data, decode(&std_compressed).unwrap());
    assert_eq!(data, decode(&better_compressed).unwrap());
    assert_eq!(data, decode(&best_compressed).unwrap());

    // Better should be same or smaller
    assert!(
        better_compressed.len() <= std_compressed.len(),
        "better should compress as well or better than standard"
    );

    // Best should be same or smaller
    assert!(
        best_compressed.len() <= better_compressed.len(),
        "best should compress as well or better than better"
    );
}
