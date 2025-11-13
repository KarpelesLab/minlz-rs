// Copyright 2024 Karpeles Lab Inc.
// Example demonstrating interoperability with Go S2 implementation

use minlz::{decode, encode};
use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("S2 Rust <-> Go Interoperability Demo\n");

    // Create some test data
    let test_data = b"This data will be compressed by Rust and can be decompressed by Go!";

    println!("Original data:");
    println!("  Size: {} bytes", test_data.len());
    println!("  Content: {:?}\n", std::str::from_utf8(test_data)?);

    // Compress with Rust
    let compressed = encode(test_data);
    println!("Compressed with Rust:");
    println!("  Size: {} bytes", compressed.len());
    println!(
        "  Hex: {}\n",
        hex::encode(&compressed[..compressed.len().min(40)])
    );

    // Save to file for Go to read
    let filename = "rust_compressed.s2";
    let mut file = fs::File::create(filename)?;
    file.write_all(&compressed)?;
    println!(
        "Saved to {} (Go can decompress this with s2.Decode)\n",
        filename
    );

    // Test round-trip in Rust
    let decompressed = decode(&compressed)?;
    assert_eq!(test_data, &decompressed[..]);
    println!("✓ Round-trip test in Rust: PASSED");

    // Simulate what Go would do (in comments):
    println!("\n--- Go Interoperability ---");
    println!("To test with Go:");
    println!("```go");
    println!("data, _ := os.ReadFile(\"{}\")", filename);
    println!("decompressed, _ := s2.Decode(nil, data)");
    println!("fmt.Println(string(decompressed))");
    println!("```");

    Ok(())
}

// Helper module for hex encoding
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}
