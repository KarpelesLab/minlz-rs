//! Test that our encoder produces byte-identical output to Go's s2 encoder

#[test]
fn test_encode_matches_go_faster() {
    // Test input - same as test_large.txt
    let input = b"This is a larger test file with more data to compress. It contains repeated patterns that should compress well. This is a larger test file with more data to compress. It contains repeated patterns that should compress well.\n";

    // Expected output from Go's s2 encoder (block data only, no stream wrapper)
    // This matches what encode() returns
    let expected_go_output: &[u8] = &[
        0xe0, 0x01, // Varint: uncompressed length 224
        0xf0, 0x5f, // LITERAL tag + data (96 bytes)
        0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65,
        0x72, 0x20, 0x74, 0x65, 0x73, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x77, 0x69, 0x74,
        0x68, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x6f, 0x20,
        0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x63, 0x6f,
        0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64,
        0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
        0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x15, 0x34, // COPY1 len=9 offset=52
        0x18, 0x20, 0x77, 0x65, 0x6c, 0x6c, 0x2e, 0x20, // LITERAL 7 bytes
        0x11, 0x70, // COPY1 len=8 offset=112
        0x15, 0x00, // COPY1 len=9 offset=0 (repeat offset)
        0x5f, 0x00, 0x0a, // 3-byte repeat encoding: len=103 -> 95 bytes copied
    ];

    // Compress with our encoder using encode() which uses Fast level
    let compressed = minlz::encode(input);

    // Compare outputs
    if compressed != expected_go_output {
        eprintln!(
            "Expected {} bytes, got {} bytes",
            expected_go_output.len(),
            compressed.len()
        );
        eprintln!("\nExpected:");
        for (i, chunk) in expected_go_output.chunks(16).enumerate() {
            eprint!("{:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }
        eprintln!("\nGot:");
        for (i, chunk) in compressed.chunks(16).enumerate() {
            eprint!("{:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }
    }

    assert_eq!(
        compressed, expected_go_output,
        "Encoded output should match Go's s2 encoder exactly"
    );
}
