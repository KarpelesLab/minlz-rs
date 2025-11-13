// Copyright 2024 Karpeles Lab Inc.
// Example of basic S2 compression/decompression

use minlz::{decode, encode, encode_better, encode_best};

fn main() {
    let original_data = b"Hello, World! This is a test of S2 compression. \
                          S2 is an extension of Snappy that provides better \
                          compression ratios while maintaining high speed.";

    println!("Original data: {} bytes", original_data.len());
    println!("Data: {:?}\n", std::str::from_utf8(original_data).unwrap());

    // Standard compression
    let compressed = encode(original_data);
    println!("Standard compression:");
    println!("  Compressed: {} bytes", compressed.len());
    println!("  Ratio: {:.2}%\n", (compressed.len() as f64 / original_data.len() as f64) * 100.0);

    // Better compression
    let compressed_better = encode_better(original_data);
    println!("Better compression:");
    println!("  Compressed: {} bytes", compressed_better.len());
    println!("  Ratio: {:.2}%\n", (compressed_better.len() as f64 / original_data.len() as f64) * 100.0);

    // Best compression
    let compressed_best = encode_best(original_data);
    println!("Best compression:");
    println!("  Compressed: {} bytes", compressed_best.len());
    println!("  Ratio: {:.2}%\n", (compressed_best.len() as f64 / original_data.len() as f64) * 100.0);

    // Decompress
    match decode(&compressed) {
        Ok(decompressed) => {
            println!("Decompression successful!");
            println!("Decompressed: {} bytes", decompressed.len());

            if decompressed == original_data {
                println!("✓ Data matches original!");
            } else {
                println!("✗ Data mismatch!");
            }
        }
        Err(e) => {
            eprintln!("Decompression error: {}", e);
        }
    }

    // Test with highly compressible data
    println!("\n--- Highly Compressible Data ---");
    let repeated_data = vec![b'A'; 10000];
    let compressed_repeated = encode(&repeated_data);
    println!("Original: {} bytes", repeated_data.len());
    println!("Compressed: {} bytes", compressed_repeated.len());
    println!("Ratio: {:.2}%", (compressed_repeated.len() as f64 / repeated_data.len() as f64) * 100.0);
}
