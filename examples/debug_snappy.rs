use minlz::decode_snappy;

fn main() {
    // Simple test case
    let simple_data = vec![
        0x09, // varint: 9 bytes uncompressed
        0x0c, // literal, 4 bytes ((4-1) << 2 | 0x00 = 0x0c)
        b'a', b'b', b'c', b'd',
        0x12, 0x04, 0x00, // copy2: length=5 ((5-1) << 2 | 0x02 = 0x12), offset=4
    ];

    println!("Input data: {:02x?}", simple_data);
    println!("Expected output: \"abcdabcda\" (9 bytes)");

    match decode_snappy(&simple_data) {
        Ok(decompressed) => {
            println!("Decoded OK: {} bytes", decompressed.len());
            println!("Output: {:?}", String::from_utf8_lossy(&decompressed));
            println!("Matches: {}", decompressed == b"abcdabcda");
        }
        Err(e) => {
            println!("Decode ERROR: {}", e);
        }
    }
}
