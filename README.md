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

### Block Format (Simple Compression)

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

### Stream Format (With CRC Validation)

```rust
use minlz::{Writer, Reader};
use std::io::{Write, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"Streaming compression with CRC validation!";

    // Compress using stream format
    let mut compressed = Vec::new();
    {
        let mut writer = Writer::new(&mut compressed);
        writer.write_all(data)?;
        writer.flush()?;
    }

    // Decompress using stream format
    let mut reader = Reader::new(&compressed[..]);
    let mut decompressed = Vec::new();
    reader.read_to_end(&mut decompressed)?;

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

This library implements **both formats**:

### Block Format
Suitable for:
- Data of known size
- In-memory compression
- Simple use cases
- Maximum compression speed

### Stream Format
Includes:
- ✓ CRC32 validation (Castagnoli polynomial)
- ✓ Chunk framing with magic headers
- ✓ Full streaming support via Reader/Writer
- ✓ Incremental reading/writing
- ✓ Compatible with Go s2.Reader/Writer

Use stream format for file I/O, network streaming, or when you need data integrity validation.

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

## Current Status

**Implemented:**
- ✓ Block format compression/decompression
- ✓ Stream format (Reader/Writer with framing)
- ✓ CRC32 validation (Castagnoli polynomial)
- ✓ Varint encoding/decoding
- ✓ Copy operations (1-byte, 2-byte, 4-byte offsets)
- ✓ Repeat offsets (S2 extension)
- ✓ Literal encoding (all size ranges)
- ✓ Compressed and uncompressed chunks
- ✓ Skippable frames and padding support
- ✓ Snappy format decoding compatibility
- ✓ Better compression algorithm (dual hash tables, hash4/hash7)
- ✓ Best compression algorithm (larger hash tables, hash5/hash8)
- ✓ All 31 tests passing

**Missing (for full Go s2 compatibility):**
- ✗ Dictionary support
- ✗ Index support for seeking
- ✗ Concurrent compression (async/parallel)

## Roadmap

The goal is full binary compatibility with [github.com/klauspost/compress/s2](https://github.com/klauspost/compress/s2).
