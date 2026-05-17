# minlz

[![CI](https://github.com/KarpelesLab/minlz-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/KarpelesLab/minlz-rs/actions/workflows/ci.yml)
[![Fuzz](https://github.com/KarpelesLab/minlz-rs/actions/workflows/fuzz.yml/badge.svg)](https://github.com/KarpelesLab/minlz-rs/actions/workflows/fuzz.yml)
[![Crates.io](https://img.shields.io/crates/v/minlz.svg)](https://crates.io/crates/minlz)
[![Docs.rs](https://docs.rs/minlz/badge.svg)](https://docs.rs/minlz)
[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Downloads](https://img.shields.io/crates/d/minlz.svg)](https://crates.io/crates/minlz)

A high-performance Rust implementation of the S2 compression format, providing binary compatibility with the Go implementation at [github.com/klauspost/compress/s2](https://github.com/klauspost/compress/tree/master/s2).

## Features

- **Binary Compatible**: All four encode modes (`encode`, `encode_better`, `encode_best`, `encode_snappy`) produce byte-for-byte identical output to Go's `s2.Encode*` on every test input
- **Decode-Heavy Performance**: 6–27× faster decode than Go's AMD64 assembly path on the same machine, peaking at ~135 GiB/s on L1-resident blocks; see [BENCHMARKS.md](BENCHMARKS.md) for the full apples-to-apples table
- **Multiple Compression Levels**: Standard, Better, and Best modes
- **Stateful Encoder**: `Encoder` struct that reuses hash-table buffers across calls for hot-loop workloads
- **Stream Format**: Full Reader/Writer support with CRC32 validation
- **Block Format**: Simple block-based compression for known-size data
- **Command-Line Tools**: Full-featured `s2c` and `s2d` tools compatible with Go implementation
- **Dictionary Compression**: Full support for dictionary-based compression
- **Concurrent Compression**: Optional parallel compression with Rayon
- **Index Support**: Seeking within compressed streams
- **Mostly Safe Rust**: A few well-documented `unsafe` blocks in hot paths (uninitialised `Vec` allocation); covered by unit, property-based, libfuzzer, and Go-binary-compat tests

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
minlz = "1"
```

### Optional Features

Enable concurrent compression for improved performance on multi-core systems:

```toml
[dependencies]
minlz = { version = "1", features = ["concurrent"] }
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

### Buffer Reuse with `Encoder`

For hot loops compressing many small/medium blocks, the stateful
`Encoder` keeps its internal hash tables across calls — eliminating
the per-call allocation cost. Output is bit-for-bit identical to the
corresponding free function.

```rust
use minlz::Encoder;

let mut enc = Encoder::new();
let mut outputs: Vec<Vec<u8>> = Vec::new();
for chunk in inputs.chunks(4096) {
    outputs.push(enc.encode(chunk));            // standard
    // or enc.encode_better(chunk), enc.encode_best(chunk), enc.encode_snappy(chunk)
}
# let _ = outputs;
# let inputs: &[u8] = b"";
```

Buffer reuse is up to **+30 %** on 1 KB `encode_better` and matches the
free-function performance for larger inputs.

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

## Command-Line Tools

The `minlz-tools` package provides `s2c` (compression) and `s2d` (decompression) command-line tools that are fully compatible with the Go s2 tools.

```bash
# Install from source
cargo install --path minlz-tools

# Compress a file
s2c input.txt              # Creates input.txt.s2
s2c --slower input.txt     # Best compression
s2c --faster input.txt     # Fast compression

# Decompress a file
s2d input.txt.s2           # Creates input.txt
s2d --verify input.txt.s2  # Verify integrity
```

The tools are cross-compatible with Go's s2c/s2d and offer 12-98x faster performance depending on the operation.

See [minlz-tools/README.md](minlz-tools/README.md) for complete documentation.

## Performance

All numbers below are **single-thread** throughput on Intel Core
i9-14900K with `RUSTFLAGS="-C target-cpu=native"` for Rust and
`GOAMD64=v3` (AVX2 enabled) for Go 1.25. Both columns measured on
the same machine with identical input generators — see
[BENCHMARKS.md](BENCHMARKS.md) for the full table, methodology, and
per-version changelog.

#### Decode (Rust dominates)

| Data Size | Pattern    | Rust         | Go (s2)   | Rust / Go |
|-----------|------------|--------------|-----------|-----------|
| 1 KB      | Random     | 38.1 GiB/s   | 6.4 GB/s  | **6.4×**  |
| 10 KB     | Random     | 103.6 GiB/s  | 5.3 GB/s  | **21×**   |
| 10 KB     | Repeated   | 134.8 GiB/s  | 5.4 GB/s  | **27×**   |
| 10 KB     | Text       | 91.4 GiB/s   | 5.3 GB/s  | **18×**   |
| 100 KB    | Random     | 70.6 GiB/s   | 5.3 GB/s  | **14×**   |

Peak: 135 GiB/s on L1-resident blocks. The 100 KB cases are DRAM-
bandwidth-bound at 70+ GiB/s.

#### Encode (Go wins standard, tied elsewhere)

| Mode     | Data Size | Pattern  | Rust          | Go (s2)    | Rust / Go |
|----------|-----------|----------|---------------|------------|-----------|
| Standard | 10 KB     | Random   |  8.2 GiB/s    | 23.1 GiB/s | 0.36×     |
| Standard | 100 KB    | Text     |  8.4 GiB/s    | 30.6 GiB/s | 0.27×     |
| Better   | 10 KB     | Text     | 10.9 GiB/s    |  7.3 GiB/s | **1.49×** |
| Better   | 100 KB    | Text     |  8.0 GiB/s    | 10.0 GiB/s | 0.80×     |
| Best     | 10 KB     | Text     | 109 MiB/s     | 116 MiB/s  | 0.94×     |
| Best     | 100 KB    | Text     | 1031 MiB/s    | 1038 MiB/s | 0.99×     |

**Honest summary** (against an *apples-to-apples* single-thread Go
run on the same i9-14900K, not the parallel-16-core aggregate the
Go README publishes):

- **Decode**: minlz is 6–27× faster than Go.
- **Standard encode**: Go is 2–4× faster — its hand-tuned AMD64
  assembly inner loop is hard to beat from pure Rust.
- **Better encode**: roughly tied, minlz wins ~10 KB, Go wins
  ~100 KB.
- **Best encode**: essentially identical (both bottleneck on the
  multi-candidate scoring algorithm rather than the inner loop).
- **Encoder output is byte-for-byte identical to Go** across all
  four modes (verified by compat tests).

If decode throughput is your priority — caching, log decompression,
streaming reads — minlz wins decisively. If you encode large blobs
in the standard "fast" mode and never decode in-process,
klauspost/compress/s2 is currently faster on that specific path.

See [BENCHMARKS.md](BENCHMARKS.md) for the full table, per-version
changelog of optimisations, and reused-`Encoder` numbers.

## Binary Compatibility

This implementation is binary compatible with the Go version in both
directions:

- **Decode**: any S2 (or Snappy) stream produced by Go is accepted
  byte-for-byte.
- **Encode**: every encode mode (`encode`, `encode_better`,
  `encode_best`, `encode_snappy`) produces byte-for-byte identical
  output to the corresponding Go function on the test inputs in
  `tests/go_compatibility.rs`, `tests/better_compatibility.rs`,
  `tests/best_compatibility.rs`, and `tests/snappy_compat.rs`.

You can therefore compress data with this Rust library and
decompress it with the Go library, and vice versa.

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
# Unit and integration tests
cargo test

# Property-based tests (proptest) — stress with 2000 cases each
PROPTEST_CASES=2000 cargo test --test proptest

# Benchmarks
RUSTFLAGS="-C target-cpu=native" cargo bench

# Fuzz testing
cargo install cargo-fuzz
cargo fuzz run fuzz_roundtrip
cargo fuzz run fuzz_decode
cargo fuzz run fuzz_stream
```

### Test Coverage

- **86 unit tests** in `src/` — core functionality, edge cases, encoder regressions
- **10 property-based tests** (`tests/proptest.rs`) — roundtrip for every
  compression level, stream format, decoder robustness, empty/all-same-byte edges
- **Go binary-compat integration tests** — `tests/go_compatibility.rs`,
  `tests/better_compatibility.rs`, `tests/best_compatibility.rs`
- **Snappy round-trip tests** — `tests/snappy_compat.rs`
- **3 libfuzzer targets** — `fuzz_roundtrip`, `fuzz_decode`, `fuzz_stream`
- **Concurrent compression tests** (with `concurrent` feature)
- **Benchmark suite** — encode/decode/roundtrip + Encoder-reuse group

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

The current implementation passes all unit, integration, proptest, and
compatibility tests, is formatted with rustfmt, and has zero clippy
warnings under `-D warnings`.
