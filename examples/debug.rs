use minlz::{decode, encode};

fn test_size(size: usize) {
    let data = vec![b'x'; size];
    println!("\nTesting size: {}", size);

    let encoded = encode(&data);
    println!("Encoded length: {}", encoded.len());
    println!(
        "Encoded bytes (first 20): {:02x?}",
        &encoded[..encoded.len().min(20)]
    );

    match decode(&encoded) {
        Ok(decoded) => {
            if data == decoded {
                println!("SUCCESS!");
            } else {
                println!(
                    "MISMATCH! Expected {} bytes, got {} bytes",
                    data.len(),
                    decoded.len()
                );
            }
        }
        Err(e) => {
            println!("Decode error: {}", e);
        }
    }
}

fn main() {
    // Test various sizes that might have issues
    test_size(10);
    test_size(59);
    test_size(60);
    test_size(61);
    test_size(100);
    test_size(255);
    test_size(256);
}
