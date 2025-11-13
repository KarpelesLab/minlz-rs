// Copyright 2024 Karpeles Lab Inc.
// Example demonstrating different compression levels

use minlz::{decode, encode, encode_better, encode_best};

fn main() {
    println!("S2 Compression Levels Comparison\n");

    // Test with highly compressible data
    let data1 = vec![b'a'; 10000];
    println!("Test 1: Highly compressible data (10,000 'a' bytes)");
    test_compression(&data1);

    println!();

    // Test with moderately compressible data
    let data2 = "The quick brown fox jumps over the lazy dog. "
        .repeat(100)
        .into_bytes();
    println!("Test 2: Moderately compressible text ({} bytes)", data2.len());
    test_compression(&data2);

    println!();

    // Test with less compressible data
    let data3: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    println!("Test 3: Sequential pattern (10,000 bytes)");
    test_compression(&data3);

    println!();

    // Test with random-like data
    let data4: Vec<u8> = (0..10000).map(|i| ((i * 7919) % 256) as u8).collect();
    println!("Test 4: Pseudo-random data (10,000 bytes)");
    test_compression(&data4);
}

fn test_compression(data: &[u8]) {
    let original_size = data.len();

    // Standard compression
    let compressed_std = encode(data);
    let std_ratio = (compressed_std.len() as f64 / original_size as f64) * 100.0;
    println!(
        "  Standard:  {} bytes ({:.2}%)",
        compressed_std.len(),
        std_ratio
    );

    // Verify decompression
    match decode(&compressed_std) {
        Ok(decompressed) => {
            assert_eq!(decompressed, data, "Standard decompression mismatch!");
        }
        Err(e) => panic!("Standard decompression failed: {}", e),
    }

    // Better compression
    let compressed_better = encode_better(data);
    let better_ratio = (compressed_better.len() as f64 / original_size as f64) * 100.0;
    let better_improvement = std_ratio - better_ratio;
    println!(
        "  Better:    {} bytes ({:.2}%, {:.2}% better)",
        compressed_better.len(),
        better_ratio,
        better_improvement
    );

    // Verify decompression
    match decode(&compressed_better) {
        Ok(decompressed) => {
            assert_eq!(decompressed, data, "Better decompression mismatch!");
        }
        Err(e) => panic!("Better decompression failed: {}", e),
    }

    // Best compression
    let compressed_best = encode_best(data);
    let best_ratio = (compressed_best.len() as f64 / original_size as f64) * 100.0;
    let best_improvement = std_ratio - best_ratio;
    println!(
        "  Best:      {} bytes ({:.2}%, {:.2}% better)",
        compressed_best.len(),
        best_ratio,
        best_improvement
    );

    // Verify decompression
    match decode(&compressed_best) {
        Ok(decompressed) => {
            assert_eq!(decompressed, data, "Best decompression mismatch!");
        }
        Err(e) => panic!("Best decompression failed: {}", e),
    }
}
