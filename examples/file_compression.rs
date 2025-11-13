// Copyright 2024 Karpeles Lab Inc.
// Example demonstrating file compression and decompression

use minlz::{decode, encode_best, Reader, Writer};
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("S2 File Compression Example\n");

    // Example 1: Simple block format compression
    let original_data = b"Hello, World! This is a test of S2 compression. ".repeat(100);

    println!("Example 1: Block Format");
    println!("  Original size: {} bytes", original_data.len());

    // Compress
    let compressed = encode_best(&original_data);
    println!(
        "  Compressed size: {} bytes ({:.2}% of original)",
        compressed.len(),
        (compressed.len() as f64 / original_data.len() as f64) * 100.0
    );

    // Decompress
    let decompressed = decode(&compressed)?;
    assert_eq!(original_data, decompressed);
    println!("  Decompression: OK\n");

    // Example 2: Stream format for file-like data
    println!("Example 2: Stream Format");

    // Simulate writing to a file
    let mut compressed_stream = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed_stream);

        // Write data in chunks (simulating streaming data)
        for i in 0..10 {
            let chunk = format!("Chunk {} data... ", i).repeat(50);
            writer.write_all(chunk.as_bytes())?;
        }

        writer.flush()?;
    }

    println!(
        "  Compressed stream size: {} bytes",
        compressed_stream.len()
    );

    // Read back
    let mut reader = Reader::new(&compressed_stream[..]);
    let mut decompressed_stream = Vec::new();
    reader.read_to_end(&mut decompressed_stream)?;

    println!("  Decompressed size: {} bytes", decompressed_stream.len());
    println!("  Stream format includes CRC validation: OK\n");

    // Example 3: Practical file compression
    println!("Example 3: Simulated File Compression");

    // Create sample data (simulating a log file)
    let log_data = r#"[2024-01-15 10:30:45] INFO: Application started
[2024-01-15 10:30:46] INFO: Loading configuration
[2024-01-15 10:30:47] INFO: Connecting to database
[2024-01-15 10:30:48] INFO: Connection established
[2024-01-15 10:30:49] INFO: Processing request #1
[2024-01-15 10:30:50] INFO: Processing request #2
[2024-01-15 10:30:51] INFO: Processing request #3
"#
    .repeat(50);

    println!("  Original log size: {} bytes", log_data.len());

    // Compress with best compression
    let compressed_log = encode_best(log_data.as_bytes());
    println!(
        "  Compressed log size: {} bytes ({:.2}% compression)",
        compressed_log.len(),
        (1.0 - compressed_log.len() as f64 / log_data.len() as f64) * 100.0
    );

    // Verify decompression
    let decompressed_log = decode(&compressed_log)?;
    assert_eq!(log_data.as_bytes(), &decompressed_log[..]);
    println!("  Verification: OK\n");

    // Example 4: Performance comparison
    println!("Example 4: Different Data Types");

    let test_cases = vec![
        ("Highly repetitive", vec![b'X'; 10000]),
        (
            "Sequential pattern",
            (0..10000).map(|i| (i % 256) as u8).collect(),
        ),
        (
            "Text-like data",
            b"The quick brown fox jumps over the lazy dog. ".repeat(200),
        ),
    ];

    for (name, data) in test_cases {
        let compressed = encode_best(&data);
        let ratio = (compressed.len() as f64 / data.len() as f64) * 100.0;
        println!(
            "  {}: {} -> {} bytes ({:.2}%)",
            name,
            data.len(),
            compressed.len(),
            ratio
        );

        // Verify
        let decompressed = decode(&compressed)?;
        assert_eq!(data, decompressed);
    }

    println!("\nAll examples completed successfully!");
    Ok(())
}
