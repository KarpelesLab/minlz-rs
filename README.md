# minlz

A Rust implementation of the S2 compression format, providing binary compatibility with the Go implementation at [github.com/klauspost/compress/s2](https://github.com/klauspost/compress/s2).

## Features

- **Binary Compatible**: Produces output compatible with the Go S2 implementation
- **Fast Compression**: Optimized for high throughput
- **Multiple Compression Levels**: Standard, Better, and Best modes
- **Block Format**: Simple block-based compression for known-size data
- **Pure Rust**: No unsafe code, written entirely in safe Rust

## S2 Format

S2 is an extension of the Snappy compression format that provides:

- Better compression ratios than Snappy
- Faster decompression
- Extended copy operations for better compression
- Compatible with Snappy-compressed data (for decompression)

**Note**: S2-compressed data cannot be decompressed by Snappy decoders.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
minlz = "0.1"
```

## Usage

### Basic Compression and Decompression

```rust
use minlz::{encode, decode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"Hello, World! This is a test.";

    // Compress
    let compressed = encode(data);
    println!("Compressed {} bytes to {} bytes", data.len(), compressed.len());

    // Decompress
    let decompressed = decode(&compressed)?;
    assert_eq!(data, &decompressed[..]);

    Ok(())
}
```

### Multiple Compression Levels

```rust
use minlz::{encode, encode_better, encode_best};

let data = b"Some data to compress...";

// Fast compression (default)
let compressed = encode(data);

// Better compression (slower)
let compressed_better = encode_better(data);

// Best compression (slowest)
let compressed_best = encode_best(data);
```

## Performance

S2 is designed for high-speed compression and decompression:

- **Compression**: Typically 250-500 MB/s
- **Decompression**: Typically 500-1500 MB/s

Actual performance depends on data characteristics and hardware.

## Binary Compatibility

This implementation is binary compatible with the Go version. You can compress data with this Rust library and decompress it with the Go library, and vice versa.

### Example: Interoperability with Go

Rust side:
```rust
use minlz::encode;
use std::fs::File;
use std::io::Write;

let data = b"Hello from Rust!";
let compressed = encode(data);

let mut file = File::create("data.s2")?;
file.write_all(&compressed)?;
```

Go side:
```go
package main

import (
    "os"
    "github.com/klauspost/compress/s2"
)

func main() {
    compressed, _ := os.ReadFile("data.s2")
    decompressed, _ := s2.Decode(nil, compressed)
    println(string(decompressed)) // "Hello from Rust!"
}
```

## Examples

Run the included examples:

```bash
# Basic compression example
cargo run --example basic

# Debug/testing example
cargo run --example debug
```

## Block vs Stream Format

This library currently implements the **block format**, which is suitable for:

- Data of known size
- In-memory compression
- Simple use cases

The block format does not include:

- CRC validation
- Chunk framing
- Streaming support

For streaming compression with CRC validation, the stream format would be needed (not yet implemented).

## Testing

Run the test suite:

```bash
cargo test
```

The tests verify:
- Round-trip compression/decompression
- Various data patterns (random, regular, repeated)
- Different data sizes
- Edge cases and boundary conditions
- Compression ratios

## License

BSD-3-Clause

## References

- [S2 Format Specification](https://github.com/klauspost/compress/tree/master/s2)
- [Snappy Format Specification](https://github.com/google/snappy/blob/main/format_description.txt)
- [Go S2 Implementation](https://github.com/klauspost/compress/tree/master/s2)

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Binary compatibility with Go implementation is maintained

## Roadmap

- [ ] Stream format support
- [ ] CRC validation
- [ ] Dictionary compression
- [ ] Index support for seeking
- [ ] Snappy compatibility mode
- [ ] Performance optimizations (SIMD)
- [ ] Benchmarking suite
