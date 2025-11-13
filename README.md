# minlz

A high-performance Rust implementation of the S2 compression format, providing binary compatibility with the Go implementation at [github.com/klauspost/compress/s2](https://github.com/klauspost/compress/tree/master/s2).

## Features

- **Binary Compatible**: Produces output 100% compatible with the Go S2 implementation
- **High Performance**: 1.6-47x faster decoding than Go depending on data pattern
- **Multiple Compression Levels**: Standard, Better, and Best modes
- **Stream Format**: Full Reader/Writer support with CRC32 validation
- **Block Format**: Simple block-based compression for known-size data
- **Pure Rust**: Written entirely in safe Rust with no unsafe code
- **Well Tested**: 50 tests, fuzz testing, and property-based testing

## S2 Format

S2 is an extension of the Snappy compression format that provides:

- Better compression ratios than Snappy
- Faster decompression than Snappy
- Extended copy operations for better compression
- Repeat offset optimization (S2 extension)
- Compatible with Snappy-compressed data (for decompression)

**Note**: S2-compressed data cannot be decompressed by Snappy decoders.

**More Information**: [S2 Design & Improvements](https://gist.github.com/klauspost/a25b66198cdbdf7b5b224f670c894ed5) - Overview of S2's design and improvements

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
minlz = "0.1"
```

### Optional Features

Enable concurrent compression for improved performance on multi-core systems:

```toml
[dependencies]
minlz = { version = "0.1", features = ["concurrent"] }
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

### Concurrent Compression (Optional Feature)

Enable the `concurrent` feature for parallel compression on multi-core systems:

```rust
use minlz::ConcurrentWriter;
use std::io::Write;

let mut compressed = Vec::new();
{
    // Compress with 4 concurrent workers
    let mut writer = ConcurrentWriter::new(&mut compressed, 4);
    writer.write_all(&large_data)?;
    writer.flush()?;
}
```

### Dictionary Compression

Dictionaries can improve compression of similar data by pre-seeding the compressor with common patterns:

```rust
use minlz::{make_dict, encode_with_dict, decode_with_dict};

// Create a dictionary from sample data
let samples = b"Common patterns that appear frequently in your data...";
let dict = make_dict(samples, Some(b"Common")).unwrap();

// Encode with dictionary
let data = b"Data to compress...";
let compressed = encode_with_dict(data, &dict);

// Decode with dictionary
let decompressed = decode_with_dict(&compressed, &dict)?;
assert_eq!(data, &decompressed[..]);

// Serialize dictionary for storage/transmission
let dict_bytes = dict.to_bytes();
```

**Note**: Dictionary encoding currently falls back to standard compression while full implementation is in progress. Dictionary decoding is fully functional.

## Performance

This Rust implementation delivers exceptional performance, often exceeding the Go reference implementation.

### Benchmark Results (Intel i9-14900K)

#### Encoding Performance

| Mode     | Data Size | Pattern    | Rust       | Go        | Speedup |
|----------|-----------|------------|------------|-----------|---------|
| Standard | 10KB      | Random     | 2.0 GiB/s  | 1280 MB/s | 1.6x    |
| Standard | 100KB     | Text       | 1.9 GiB/s  | 1545 MB/s | 1.3x    |
| Better   | 10KB      | Text       | 859 MiB/s  | 2232 MB/s | 0.4x    |
| Best     | 10KB      | Repeated   | 277 MiB/s  | 7 MB/s    | **41x** |
| Best     | 10KB      | Text       | 174 MiB/s  | 7 MB/s    | **25x** |

#### Decoding Performance

| Data Size | Pattern    | Rust       | Go        | Speedup  |
|-----------|------------|------------|-----------|----------|
| 1KB       | Random     | 16.5 GiB/s | 672 MB/s  | **26x**  |
| 10KB      | Random     | 24.3 GiB/s | 538 MB/s  | **47x**  |
| 10KB      | Text       | 6.3 GiB/s  | 509 MB/s  | **13x**  |
| 100KB     | Random     | 21.3 GiB/s | 654 MB/s  | **34x**  |
| 100KB     | Repeated   | 1.03 GiB/s | 685 MB/s  | 1.6x     |

**Key Takeaways:**
- **Decode-heavy workloads**: Rust is 10-47x faster (random/text data)
- **Best compression mode**: Rust is 25-41x faster, making it practical for production use
- **Standard encoding**: Competitive with Go, 1.3-1.6x faster on larger data
- **Better mode**: Go currently faster (area for future optimization)

See [BENCHMARKS.md](BENCHMARKS.md) for detailed performance analysis.

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

This implementation includes comprehensive testing infrastructure:

### Run Tests

```bash
# Unit and integration tests (48 tests)
cargo test

# Property-based tests (proptest)
cargo test --test proptest

# Benchmarks
cargo bench

# Fuzz testing
cargo install cargo-fuzz
cargo fuzz run fuzz_roundtrip
cargo fuzz run fuzz_decode
cargo fuzz run fuzz_stream
```

### Test Coverage

- **48 Unit/Integration Tests**: Core functionality and edge cases
- **10 Property-Based Tests**: Using proptest for randomized testing
  - Roundtrip verification for all compression levels
  - Stream format validation
  - Compression ratio verification
  - Decoder robustness (never panics on invalid input)
  - Edge cases (empty data, small data, all-same-byte)
  - Compression level compatibility
- **3 Fuzz Targets**: Continuous fuzzing with libfuzzer
  - Roundtrip fuzzing for all compression levels
  - Decode fuzzing (arbitrary input)
  - Stream format fuzzing
- **Benchmark Suite**: Performance comparison with Go implementation

## License

BSD-3-Clause

## References

- [S2 Design & Improvements](https://gist.github.com/klauspost/a25b66198cdbdf7b5b224f670c894ed5) - Overview of S2's design and improvements over Snappy
- [Go S2 Implementation](https://github.com/klauspost/compress/tree/master/s2) - Reference implementation
- [Snappy Format Specification](https://github.com/google/snappy/blob/main/format_description.txt) - Base Snappy format

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Binary compatibility with Go implementation is maintained

The current implementation passes all 48 tests, is formatted with rustfmt, and has been checked with clippy.

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
- ✓ Standard compression algorithm (hash6 table)
- ✓ Better compression algorithm (dual hash tables, hash4/hash7)
- ✓ Best compression algorithm (larger hash tables, hash5/hash8)
- ✓ Index support (offset tracking for seeking)
- ✓ Concurrent compression (optional feature, uses Rayon)
- ✓ Dictionary support (decoding complete, encoding API available)
- ✓ Comprehensive test suite (58 tests + 10 property tests + 3 fuzz targets)
- ✓ Binary compatibility verified with Go implementation
- ✓ Performance benchmarking suite

**Partially Implemented:**
- ⚠️ Index support for seeking (core structure complete, Reader integration pending)
- ⚠️ Dictionary encoding (API available, falls back to standard encoding; full optimization pending)

See [MISSING_FEATURES.md](MISSING_FEATURES.md) for detailed analysis of missing features.

## Project Goals

1. **Binary Compatibility**: 100% compatible with [github.com/klauspost/compress/s2](https://github.com/klauspost/compress/tree/master/s2)
2. **High Performance**: Match or exceed Go implementation performance
3. **Production Ready**: Comprehensive testing, fuzzing, and validation
4. **Safe Rust**: No unsafe code, leveraging Rust's memory safety guarantees

Status: **Core functionality complete and production-ready**
