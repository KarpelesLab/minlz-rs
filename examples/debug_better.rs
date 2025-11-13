use minlz::{decode, encode, encode_better};

fn main() {
    let data = vec![b'a'; 100];

    println!("Original data: {} bytes", data.len());

    // Test standard encoding
    let encoded = encode(&data);
    println!("Standard encoded: {} bytes", encoded.len());
    println!(
        "Standard encoded bytes: {:02x?}",
        &encoded[..encoded.len().min(30)]
    );

    match decode(&encoded) {
        Ok(decoded) => {
            println!("Standard decode OK: {} bytes", decoded.len());
            if decoded != data {
                println!("Standard decode MISMATCH!");
            }
        }
        Err(e) => println!("Standard decode ERROR: {}", e),
    }

    // Test better encoding
    let encoded_better = encode_better(&data);
    println!("\nBetter encoded: {} bytes", encoded_better.len());
    println!("Better encoded ALL bytes: {:02x?}", &encoded_better);

    match decode(&encoded_better) {
        Ok(decoded) => {
            println!("Better decode OK: {} bytes", decoded.len());
            if decoded != data {
                println!("Better decode MISMATCH! got {} bytes", decoded.len());
            }
        }
        Err(e) => println!("Better decode ERROR: {}", e),
    }
}
