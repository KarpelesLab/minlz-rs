# Performance Benchmarks: Rust vs Go S2 Implementation

This document records the performance of the Rust implementation (`minlz`)
and compares it to the Go reference (`github.com/klauspost/compress/s2`)
where comparable numbers are available.

## Test Environment

- CPU: Intel Core i9-14900K
- OS: Linux 6.12.41-gentoo
- Rust: minlz 0.1.3 (rustc 1.95.0)
- Build: `RUSTFLAGS="-C target-cpu=native" cargo bench --bench compression`
- Harness: criterion 0.5 (100 samples / bench, 3 s warmup)

Go numbers below are carried over from the previous report
(klauspost/compress/s2, same CPU class) and are unchanged in this revision —
only the Rust column has been re-measured.

## Encoding Performance

### Standard Compression (`encode`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner |
|-----------|------------|-----------|--------------|-------------|--------|
| 1KB       | Random     | 734       | 1793         | 1880        | Rust   |
| 1KB       | Repeated   | 997       | 1875         | 1966        | Rust   |
| 1KB       | Text       | 887       | 1801         | 1888        | Rust   |
| 1KB       | Sequential | 739       | 1774         | 1860        | Rust   |
| 10KB      | Random     | 1280      | 2071         | 2172        | Rust   |
| 10KB      | Repeated   | 1199      | 2061         | 2161        | Rust   |
| 10KB      | Text       | 1312      | 2080         | 2181        | Rust   |
| 10KB      | Sequential | 1291      | 2028         | 2127        | Rust   |
| 100KB     | Random     | 1570      | 2122         | 2225        | Rust   |
| 100KB     | Repeated   | 1292      | 2116         | 2219        | Rust   |
| 100KB     | Text       | 1545      | 2100         | 2202        | Rust   |
| 100KB     | Sequential | 1231      | 2001         | 2098        | Rust   |

### Better Compression (`encode_better`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner |
|-----------|------------|-----------|--------------|-------------|--------|
| 1KB       | Random     | N/A       | 997          | 1046        | -      |
| 1KB       | Repeated   | N/A       | 1821         | 1909        | -      |
| 1KB       | Text       | N/A       | 1299         | 1362        | -      |
| 10KB      | Random     | N/A       | 1979         | 2076        | -      |
| 10KB      | Repeated   | 1430      | 2093         | 2195        | Rust   |
| 10KB      | Text       | 2232      | 2044         | 2143        | Go     |
| 100KB     | Random     | N/A       | 1949         | 2044        | -      |
| 100KB     | Repeated   | N/A       | 2015         | 2113        | -      |
| 100KB     | Text       | N/A       | 1993         | 2090        | -      |

### Best Compression (`encode_best`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner      |
|-----------|------------|-----------|--------------|-------------|-------------|
| 1KB       | Repeated   | N/A       | 11.0         | 11.6        | -           |
| 1KB       | Text       | N/A       | 11.5         | 12.1        | -           |
| 10KB      | Repeated   | 7.04      | 80.0         | 83.9        | Rust (12x)  |
| 10KB      | Text       | 7.15      | 110.0        | 115.4       | Rust (16x)  |
| 100KB     | Repeated   | N/A       | 219.7        | 230.4       | -           |
| 100KB     | Text       | N/A       | 759.0        | 795.8       | -           |

**Note:** Best mode produces byte-identical output to Go's `s2.EncodeBest`.

## Decoding Performance

| Data Size | Pattern    | Go (MB/s) | Rust (GiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|------------|-----------|--------------|-------------|---------------|
| 1KB       | Random     | 672       | 36.2         | 38879       | 58x           |
| 1KB       | Repeated   | 547       | 46.1         | 49479       | 90x           |
| 1KB       | Text       | 560       | 27.7         | 29742       | 53x           |
| 1KB       | Sequential | N/A       | 35.0         | 37551       | N/A           |
| 10KB      | Random     | 538       | 76.4         | 82072       | 152x          |
| 10KB      | Repeated   | 537       | 88.8         | 95289       | 177x          |
| 10KB      | Text       | 509       | 68.8         | 73917       | 145x          |
| 10KB      | Sequential | N/A       | 80.7         | 86610       | N/A           |
| 100KB     | Random     | 654       | 38.5         | 41377       | 63x           |
| 100KB     | Repeated   | 685       | 39.8         | 42685       | 62x           |
| 100KB     | Text       | 627       | 38.8         | 41618       | 66x           |
| 100KB     | Sequential | N/A       | 39.1         | 41992       | N/A           |

10 KB cases sit in L1/L2 and reach 70–90 GiB/s; 100 KB cases hit DRAM
bandwidth limits at ~39 GiB/s for all patterns.

## Roundtrip Performance (Encode + Decode)

| Data Size | Pattern  | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|----------|-----------|--------------|-------------|---------------|
| 1KB       | Text     | 329       | 1615         | 1693        | 5.1x          |
| 1KB       | Repeated | 294       | 1789         | 1875        | 6.4x          |
| 10KB      | Text     | 354       | 1998         | 2095        | 5.9x          |
| 10KB      | Repeated | 302       | 1993         | 2090        | 6.9x          |

## What changed in 0.1.4

Two independent optimizations landed:

### 1. Decoder: overlapping-copy fix

The previous report showed `decode/repeated` at ~1 GiB/s and `decode/text`
at ~3 GiB/s — 35× slower than `decode/random` and `decode/sequential`. The
culprit was a byte-by-byte loop in `copy_within` whenever a copy operation
had `offset < length` (i.e. the match overlapped its own destination —
common for runs and short cycles). Replacing it with `slice::fill` for the
`offset == 1` RLE case and an O(log length) chain of `slice::copy_within`
memmoves for the general overlap case lifted those numbers up to the same
plateau as the non-overlapping cases.

| Bench                  | Before     | After       | Δ      |
|------------------------|------------|-------------|--------|
| decode/repeated/1024   | 1.03 GiB/s | 46.1 GiB/s  | +45x   |
| decode/repeated/10240  | 1.01 GiB/s | 88.8 GiB/s  | +88x   |
| decode/repeated/102400 | 0.99 GiB/s | 39.8 GiB/s  | +40x   |
| decode/text/1024       | 2.58 GiB/s | 27.7 GiB/s  | +10.7x |
| decode/text/10240      | 2.99 GiB/s | 68.8 GiB/s  | +23.0x |
| decode/text/102400     | 3.20 GiB/s | 38.8 GiB/s  | +12.1x |

Non-overlapping cases also picked up (random/sequential 1.5–2.9×) because
simplifying the function shape let LLVM inline `copy_within` more
aggressively. The 100 KB random/sequential cases were already DRAM-bound
and didn't move.

### 2. Standard encoder: adaptive hash table size

`encode()` previously allocated a 64 KB hash table (16384 × u32) for every
input ≤ 64 KB, regardless of how small the input was. For 1 KB calls the
mandatory zero-init of that 64 KB buffer was the single largest cost. The
table now sizes to the input:

| Input size       | Table bits | Table memory |
|------------------|-----------:|-------------:|
| `< 1024`         | 10         | 4 KB         |
| `1024..8192`     | 12         | 16 KB        |
| `8192..=65536`   | 14         | 64 KB        |
| `> 65536`        | 17         | 512 KB       |

Smaller inputs also keep the table in L1 instead of spilling into L2.
Compression ratio is unchanged on the standard benchmark patterns
(verified by encoding before and after — every output was byte-identical).

| Bench                          | Before     | After       | Δ      |
|--------------------------------|------------|-------------|--------|
| encode_standard/random/1024    | 848 MiB/s  | 1.75 GiB/s  | +110%  |
| encode_standard/repeated/1024  | 872 MiB/s  | 1.83 GiB/s  | +115%  |
| encode_standard/text/1024      | 873 MiB/s  | 1.76 GiB/s  | +106%  |
| encode_standard/sequential/1024| 863 MiB/s  | 1.73 GiB/s  | +106%  |
| encode_standard/10240, 102400  | unchanged (within ±2% noise) | | |

The 10 KB and 100 KB cases hit unchanged code paths (still 14-bit / 17-bit
tables) so they show the usual ±2 % criterion variance.

The combined effect on small-block roundtrip is roughly 2×:

| Bench                  | Before     | After       | Δ     |
|------------------------|------------|-------------|-------|
| roundtrip/text/1024    | 670 MiB/s  | 1.58 GiB/s  | +141% |
| roundtrip/repeated/1024| 457 MiB/s  | 1.75 GiB/s  | +291% |
| roundtrip/text/10240   | 1.24 GiB/s | 1.95 GiB/s  | +57%  |
| roundtrip/repeated/10240| 689 MiB/s | 1.95 GiB/s  | +189% |

## Summary

### Rust advantages
1. **Decode performance**: 53–177x faster than Go across all patterns; the
   `repeated`/`text` patterns are no longer outliers.
2. **Standard encoding**: Faster than Go in every measured case, with a
   2× lead on 1 KB inputs after the hash-table sizing change.
3. **Best mode**: 12–16x faster than Go where Go benchmarks are available,
   while producing byte-identical output for interop.

### Go advantages
1. **Better mode on 10 KB text**: Go is still ~5% ahead on this specific
   pattern; on other patterns / sizes Rust matches or beats Go.
2. **Memory efficiency**: Go's per-call allocation profile is still leaner.

### Next steps

Remaining opportunities to investigate:
1. **Buffer reuse on the `Encoder` struct.** Even with adaptive table
   sizing, each `encode()` call still allocates fresh. Plumbing buffer
   reuse through `Encoder` would help latency-sensitive callers further.
2. **`encode_better` on small inputs.** 1 KB random sits at 1.0 GiB/s,
   half of the 10 KB+ throughput — fixed overhead in the better encoder
   could likely be cut.
3. **SIMD-assisted match extension.** The 8-byte XOR / trailing-zero loop
   in match extension could be unrolled or vectorised.
