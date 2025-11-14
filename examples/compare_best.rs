use std::fs;

fn main() {
    // Read input file
    let data = fs::read("/tmp/test_best.txt").expect("Failed to read input file");

    // Compress with Best encoder
    let compressed = minlz::encode_best(&data);

    // Write compressed data
    fs::write("/tmp/test_best_rust.s2", &compressed).expect("Failed to write output file");

    println!(
        "Rust Best compression: {} -> {} bytes ({:.2}%)",
        data.len(),
        compressed.len(),
        compressed.len() as f64 * 100.0 / data.len() as f64
    );

    // Print first 32 bytes as hex
    print!("First 32 bytes: ");
    #[allow(clippy::needless_range_loop)]
    for i in 0..32.min(compressed.len()) {
        print!("{:02x} ", compressed[i]);
    }
    println!();
}
