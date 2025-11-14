/// Decode and display S2 instructions from compressed data
use std::fs;

fn main() {
    let go_data = fs::read("/tmp/test_best_go.s2").expect("Failed to read Go output");
    let rust_data = fs::read("/tmp/test_best_rust.s2").expect("Failed to read Rust output");

    println!("=== Go s2.EncodeBest ({} bytes) ===\n", go_data.len());
    decode_and_verify(&go_data, "Go");

    println!("\n=== Rust encode_best ({} bytes) ===\n", rust_data.len());
    decode_and_verify(&rust_data, "Rust");
}

fn decode_and_verify(data: &[u8], name: &str) {
    // Decode using minlz
    match minlz::decode(data) {
        Ok(decoded) => {
            println!("{} output decodes to {} bytes", name, decoded.len());

            // Show first 100 bytes
            println!("\nFirst 100 bytes of decoded output:");
            let preview = &decoded[..100.min(decoded.len())];
            println!("{}", String::from_utf8_lossy(preview));
            println!();
        }
        Err(e) => {
            println!("ERROR: Failed to decode {} output: {:?}", name, e);
        }
    }
}
