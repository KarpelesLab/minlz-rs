# minlz-tools

Command-line tools for S2 compression format, compatible with the Go [s2 tools](https://github.com/klauspost/compress/tree/master/s2/cmd).

## Features

- **Binary Compatible**: Produces output 100% compatible with Go s2 tools
- **High Performance**: Powered by the minlz library
- **Full Feature Set**: All options from Go s2c/s2d tools
- **Drop-in Replacement**: Compatible command-line interface

## Installation

### From Source

```bash
cargo install --path minlz-tools
```

Or build locally:

```bash
cd minlz-tools
cargo build --release

# Binaries will be in target/release/
# - s2c: compression tool
# - s2d: decompression tool
```

### From crates.io

```bash
cargo install minlz-tools
```

## s2c - Compression Tool

### Basic Usage

```bash
# Compress a file
s2c input.txt
# Creates input.txt.s2

# Custom output file
s2c input.txt -o output.s2

# Compress to stdout
s2c -c input.txt > output.s2

# Stdin to stdout
cat input.txt | s2c - -c > output.s2
```

### Compression Levels

```bash
# Fast compression (fastest)
s2c --faster input.txt

# Standard compression (default, balanced)
s2c input.txt

# Best compression (slowest, highest compression)
s2c --slower input.txt
```

### Advanced Options

```bash
# Custom block size (default: 4MB)
s2c --blocksize 1M input.txt
s2c --blocksize 8M input.txt

# Remove source file after compression
s2c --rm input.txt

# Safe mode (don't overwrite existing files)
s2c --safe input.txt

# Quiet mode (suppress output)
s2c -q input.txt

# CPU profiling (writes to cpu.pprof)
s2c --cpu input.txt

# Verify compressed output by decompressing
s2c --verify input.txt
```

### Examples

```bash
# Compress large file with 8MB blocks
s2c --blocksize 8M large_file.bin

# Fast compression for temporary files
s2c --faster --rm temp.log

# Best compression with verification
s2c --slower --verify important.dat

# Compress all text files in directory
for f in *.txt; do s2c "$f"; done

# Pipeline compression
tar cf - directory/ | s2c - -c > directory.tar.s2
```

## s2d - Decompression Tool

### Basic Usage

```bash
# Decompress a file
s2d input.txt.s2
# Creates input.txt

# Custom output file
s2d input.txt.s2 -o output.txt

# Decompress to stdout
s2d -c input.txt.s2 > output.txt

# Stdin to stdout
cat input.txt.s2 | s2d - -c > output.txt
```

### Verification and Options

```bash
# Verify file integrity (no output)
s2d --verify input.txt.s2

# Remove source file after decompression
s2d --rm input.txt.s2

# Safe mode (don't overwrite existing files)
s2d --safe input.txt.s2

# Quiet mode (suppress output)
s2d -q input.txt.s2

# CPU profiling (writes to cpu.pprof)
s2d --cpu input.txt.s2
```

### Examples

```bash
# Decompress all .s2 files in directory
for f in *.s2; do s2d "$f"; done

# Verify multiple files
s2d --verify *.s2

# Pipeline decompression
s2d -c archive.tar.s2 | tar xf -

# Decompress and pipe to another program
s2d -c data.json.s2 | jq '.results'
```

## Cross-Compatibility with Go Tools

The tools are fully compatible with the Go s2 implementation:

```bash
# Compress with Rust s2c, decompress with Go s2d
./s2c file.txt
go run github.com/klauspost/compress/s2/cmd/s2d@latest file.txt.s2

# Compress with Go s2c, decompress with Rust s2d
go run github.com/klauspost/compress/s2/cmd/s2c@latest file.txt
./s2d file.txt.s2

# Files are byte-for-byte identical
```

## Performance

The minlz-tools leverage the high-performance minlz library:

- **Compression**: 1.5-1.7x faster than Go for standard mode
- **Decompression**: 17-98x faster depending on data pattern
- **Best Mode**: 12-16x faster with binary-compatible output

See the main [minlz README](../README.md#performance) for detailed benchmarks.

## Block Size Selection

The `--blocksize` option affects compression characteristics:

| Block Size | Use Case | Compression | Speed |
|------------|----------|-------------|-------|
| 64KB       | Small files, low memory | Lower | Faster |
| 1MB        | Balanced | Good | Good |
| 4MB (default) | Most use cases | Better | Fast |
| 8MB+       | Large files, max compression | Best | Slower |

**Recommendation**: Use the default 4MB for most cases. Use smaller blocks for memory-constrained environments or real-time streaming. Use larger blocks for archival compression of large files.

## Exit Codes

Both tools follow standard Unix conventions:

- `0`: Success
- `1`: Error (invalid arguments, file not found, compression/decompression failure, etc.)

## File Naming

### s2c (compression)
- `input.txt` → `input.txt.s2` (default)
- `input.txt.gz` → `input.txt.gz.s2` (preserves extension)
- Use `-o` to specify custom output name

### s2d (decompression)
- `input.txt.s2` → `input.txt` (removes .s2)
- `input.s2` → `input` (removes .s2)
- Use `-o` to specify custom output name

## Error Handling

The tools provide clear error messages:

```bash
$ s2d corrupted.s2
Error: decode error: invalid block header

$ s2c --safe existing.txt
Error: output file existing.txt.s2 already exists (use --safe=false to overwrite)

$ s2c --blocksize 64GB input.txt
Error: block size too large, maximum is 4294967294 bytes
```

## Integration Examples

### Backup Script

```bash
#!/bin/bash
# Compress and remove originals
find /var/log -name "*.log" -mtime +7 -exec s2c --rm {} \;
```

### Archive Creation

```bash
# Create compressed tar archive
tar cf - project/ | s2c --slower - -c > project.tar.s2

# Extract
s2d -c project.tar.s2 | tar xf -
```

### Data Processing Pipeline

```bash
# Decompress, process, recompress
s2d -c data.json.s2 | jq '.filtered' | s2c - -c > filtered.json.s2
```

## Differences from Go Tools

The Rust implementation aims for 100% compatibility. Minor differences:

1. **Performance**: Rust tools are significantly faster for most operations
2. **Memory**: Similar memory usage with slightly different allocation patterns
3. **Error Messages**: May have slightly different wording but same information
4. **CPU Profiling**: Generates pprof-compatible output (implementation differs)

All compressed output is byte-for-byte identical between implementations.

## Building from Source

```bash
# Clone repository
git clone https://github.com/KarpelesLab/minlz-rs.git
cd minlz-rs/minlz-tools

# Build release version
cargo build --release

# Run tests
cargo test

# Install to system
cargo install --path .
```

## License

BSD-3-Clause

## References

- [minlz Library](../README.md) - Core compression library
- [Go s2 Tools](https://github.com/klauspost/compress/tree/master/s2/cmd) - Reference implementation
- [S2 Format](https://github.com/klauspost/compress/tree/master/s2) - Format specification
