// Copyright 2024 Karpeles Lab Inc.
// Example demonstrating stream compression/decompression

use minlz::{Reader, Writer};
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("S2 Stream Format Demo\n");

    let original_data = b"The S2 stream format provides framed compression with CRC validation. \
                          It's perfect for streaming scenarios where you don't know the total size upfront.";

    println!("Original data:");
    println!("  Size: {} bytes", original_data.len());
    println!("  Content: {:?}\n", std::str::from_utf8(original_data)?);

    // Compress using stream format
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(original_data)?;
        writer.flush()?;
    }

    println!("Compressed (stream format):");
    println!("  Size: {} bytes", compressed.len());
    println!(
        "  Ratio: {:.2}%\n",
        (compressed.len() as f64 / original_data.len() as f64) * 100.0
    );

    // Show magic bytes
    println!("Magic bytes: {:02x?}", &compressed[..10]);
    println!("(ff 06 00 00 = stream identifier chunk)\n");

    // Decompress using stream format
    let mut reader = Reader::new(&compressed[..]);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed)?;

    println!("Decompressed:");
    println!("  Size: {} bytes", decompressed.len());
    println!("  Match: {}\n", decompressed == original_data);

    // Test incremental reading
    println!("--- Incremental Reading Test ---");
    let mut reader = Reader::new(&compressed[..]);
    let mut buffer = [0u8; 20];
    let mut total_read = 0;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        total_read += n;
        println!("Read {} bytes: {:?}", n, std::str::from_utf8(&buffer[..n])?);
    }

    println!("\nTotal read: {} bytes", total_read);

    // Test multiple writes creating multiple chunks
    println!("\n--- Multiple Chunks Test ---");
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::with_block_size(&mut compressed, 32); // Small blocks
        for i in 0..5 {
            writer.write_all(format!("Chunk {} ", i).as_bytes())?;
        }
        writer.flush()?;
    }

    let mut reader = Reader::new(&compressed[..]);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed)?;

    println!("Compressed with small blocks: {} bytes", compressed.len());
    println!("Decompressed: {:?}", std::str::from_utf8(&decompressed)?);

    Ok(())
}
