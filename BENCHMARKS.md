# Performance Benchmarks: Rust vs Go S2 Implementation

This document compares the performance of the Rust implementation (minlz) against the Go reference implementation (klauspost/compress/s2).

## Test Environment

- CPU: Intel Core i9-14900K (32 cores)
- OS: Linux 6.6.30-gentoo-shizuku
- Go: github.com/klauspost/compress/s2
- Rust: minlz 0.1.0

## Encoding Performance

### Standard Compression

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner |
|-----------|------------|-----------|--------------|-------------|--------|
| 1KB       | Random     | 734       | 733          | 769         | Rust   |
| 1KB       | Repeated   | 997       | 880          | 923         | Go     |
| 1KB       | Text       | 887       | 826          | 866         | Go     |
| 1KB       | Sequential | 739       | 727          | 762         | Rust   |
| 10KB      | Random     | 1280      | 2000         | 2097        | Rust   |
| 10KB      | Repeated   | 1199      | 2031         | 2129        | Rust   |
| 10KB      | Text       | 1312      | 2027         | 2125        | Rust   |
| 10KB      | Sequential | 1291      | 1819         | 1907        | Rust   |
| 100KB     | Random     | 1570      | 1811         | 1899        | Rust   |
| 100KB     | Repeated   | 1292      | 1898         | 1990        | Rust   |
| 100KB     | Text       | 1545      | 1944         | 2038        | Rust   |
| 100KB     | Sequential | 1231      | 1532         | 1606        | Rust   |

### Better Compression

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner |
|-----------|------------|-----------|--------------|-------------|--------|
| 10KB      | Repeated   | 1430      | 910          | 954         | Go     |
| 10KB      | Text       | 2232      | 859          | 901         | Go     |
| 100KB     | Repeated   | N/A       | 1398         | 1466        | Rust   |
| 100KB     | Text       | N/A       | 2007         | 2104        | Rust   |

### Best Compression

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner      |
|-----------|------------|-----------|--------------|-------------|-------------|
| 10KB      | Repeated   | 7.04      | 277          | 290         | Rust (41x)  |
| 10KB      | Text       | 7.15      | 174          | 182         | Rust (25x)  |
| 100KB     | Repeated   | N/A       | 1286         | 1348        | Rust        |
| 100KB     | Text       | N/A       | 1039         | 1089        | Rust        |

**Key Finding**: The Rust Best compression mode is 25-41x faster than Go while maintaining the same compression format.

## Decoding Performance

| Data Size | Pattern    | Go (MB/s) | Rust (GiB/s) | Rust (MB/s) | Speedup |
|-----------|------------|-----------|--------------|-------------|---------|
| 1KB       | Random     | 672       | 16.5         | 17203       | 25.6x   |
| 1KB       | Repeated   | 547       | 0.93         | 970         | 1.8x    |
| 1KB       | Text       | 560       | 5.2          | 5423        | 9.7x    |
| 1KB       | Sequential | N/A       | 13.6         | 14182       | N/A     |
| 10KB      | Random     | 538       | 24.3         | 25343       | 47.1x   |
| 10KB      | Repeated   | 537       | 1.03         | 1074        | 2.0x    |
| 10KB      | Text       | 509       | 6.3          | 6570        | 12.9x   |
| 10KB      | Sequential | N/A       | 24.4         | 25447       | N/A     |
| 100KB     | Random     | 654       | 21.3         | 22196       | 33.9x   |
| 100KB     | Repeated   | 685       | 1.03         | 1074        | 1.6x    |
| 100KB     | Text       | 627       | 6.5          | 6780        | 10.8x   |
| 100KB     | Sequential | N/A       | 21.4         | 22313       | N/A     |

**Key Finding**: The Rust decoder is significantly faster than Go:
- **1.6-2x faster** on highly compressible repeated data
- **10-13x faster** on text data
- **26-47x faster** on random and sequential data

## Roundtrip Performance (Encode + Decode)

| Data Size | Pattern  | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Speedup |
|-----------|----------|-----------|--------------|-------------|---------|
| 1KB       | Text     | 329       | 655          | 687         | 2.1x    |
| 1KB       | Repeated | 294       | 491          | 515         | 1.8x    |
| 10KB      | Text     | 354       | 1537         | 1612        | 4.6x    |
| 10KB      | Repeated | 302       | 728          | 763         | 2.5x    |

## Memory Allocations

Go implementation shows excellent memory efficiency with minimal allocations:
- Standard encoding: 1 allocation per operation
- Decoding: 1 allocation per operation
- Zero allocations in many optimized paths

The Rust implementation uses safe memory handling with Vec allocations but maintains competitive performance.

## Summary

### Rust Advantages
1. **Decode Performance**: 1.6-47x faster depending on data pattern
2. **Best Mode Encoding**: 25-41x faster than Go
3. **Larger Data**: Scales better with 10KB+ data sizes
4. **Random/Sequential Data**: Exceptional performance (20+ GiB/s)

### Go Advantages
1. **Better Mode Encoding**: 1.5-2.5x faster on 10KB text/repeated data
2. **Memory Efficiency**: Excellent allocation patterns
3. **Small Data**: Slightly better on some 1KB patterns
4. **Mature Optimizations**: Highly optimized for production use

### Overall Assessment

The Rust implementation demonstrates **exceptional decode performance** and makes the "Best" compression mode practical for real-world use. The standard encoding performance is competitive with Go, and the implementation successfully achieves the goal of a high-performance S2 codec.

For decode-heavy workloads, the Rust implementation offers significant performance advantages (10-47x faster). For encode-heavy workloads, both implementations perform well, with Go having an edge in "Better" mode and Rust excelling in "Best" mode.

## Next Steps

Potential optimizations to consider:
1. Improve "Better" mode encoding performance for small data
2. Optimize repeated data encoding (currently slower than Go)
3. Consider SIMD optimizations for even faster decoding
4. Profile memory allocation patterns to match Go's efficiency
