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
| 1KB       | Random     | 734       | 848          | 889         | Rust   |
| 1KB       | Repeated   | 997       | 872          | 914         | Go     |
| 1KB       | Text       | 887       | 873          | 915         | Rust   |
| 1KB       | Sequential | 739       | 863          | 905         | Rust   |
| 10KB      | Random     | 1280      | 2110         | 2213        | Rust   |
| 10KB      | Repeated   | 1199      | 2115         | 2218        | Rust   |
| 10KB      | Text       | 1312      | 2101         | 2204        | Rust   |
| 10KB      | Sequential | 1291      | 2080         | 2181        | Rust   |
| 100KB     | Random     | 1570      | 2164         | 2270        | Rust   |
| 100KB     | Repeated   | 1292      | 2179         | 2284        | Rust   |
| 100KB     | Text       | 1545      | 2170         | 2275        | Rust   |
| 100KB     | Sequential | 1231      | 2149         | 2253        | Rust   |

### Better Compression (`encode_better`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner |
|-----------|------------|-----------|--------------|-------------|--------|
| 1KB       | Random     | N/A       | 1046         | 1097        | -      |
| 1KB       | Repeated   | N/A       | 1822         | 1910        | -      |
| 1KB       | Text       | N/A       | 1322         | 1386        | -      |
| 10KB      | Random     | N/A       | 2009         | 2106        | -      |
| 10KB      | Repeated   | 1430      | 2175         | 2280        | Rust   |
| 10KB      | Text       | 2232      | 2053         | 2152        | Go     |
| 100KB     | Random     | N/A       | 1988         | 2084        | -      |
| 100KB     | Repeated   | N/A       | 2083         | 2184        | -      |
| 100KB     | Text       | N/A       | 2054         | 2153        | -      |

### Best Compression (`encode_best`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner      |
|-----------|------------|-----------|--------------|-------------|-------------|
| 1KB       | Repeated   | N/A       | 11.3         | 11.8        | -           |
| 1KB       | Text       | N/A       | 11.4         | 11.9        | -           |
| 10KB      | Repeated   | 7.04      | 81.6         | 85.5        | Rust (12x)  |
| 10KB      | Text       | 7.15      | 109.1        | 114.4       | Rust (16x)  |
| 100KB     | Repeated   | N/A       | 217.5        | 228.1       | -           |
| 100KB     | Text       | N/A       | 780.5        | 818.4       | -           |

**Note:** Best mode produces byte-identical output to Go's `s2.EncodeBest`.

## Decoding Performance

| Data Size | Pattern    | Go (MB/s) | Rust (GiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|------------|-----------|--------------|-------------|---------------|
| 1KB       | Random     | 672       | 38.4         | 41210       | 61x           |
| 1KB       | Repeated   | 547       | 48.1         | 51683       | 94x           |
| 1KB       | Text       | 560       | 28.1         | 30156       | 54x           |
| 1KB       | Sequential | N/A       | 38.1         | 40920       | N/A           |
| 10KB      | Random     | 538       | 81.2         | 87125       | 162x          |
| 10KB      | Repeated   | 537       | 99.7         | 107063      | 199x          |
| 10KB      | Text       | 509       | 71.4         | 76612       | 151x          |
| 10KB      | Sequential | N/A       | 80.5         | 86487       | N/A           |
| 100KB     | Random     | 654       | 38.9         | 41807       | 64x           |
| 100KB     | Repeated   | 685       | 42.0         | 45066       | 66x           |
| 100KB     | Text       | 627       | 39.0         | 41862       | 67x           |
| 100KB     | Sequential | N/A       | 38.9         | 41789       | N/A           |

The 10 KB cases sit in L1/L2 and reach 70–100 GiB/s; the 100 KB cases hit
DRAM bandwidth limits at ~39 GiB/s for all patterns.

### What changed since the previous report

The previous Rust column for decode showed `repeated` at ~1 GiB/s and `text`
at ~3 GiB/s while `random`/`sequential` ran at 36–51 GiB/s. The culprit was
a byte-by-byte loop in `copy_within` whenever a copy operation had
`offset < length` (i.e. the match overlapped its own destination — common
for runs and short cycles). Replacing that loop with `slice::fill` for the
`offset == 1` RLE case and an O(log length) chain of `slice::copy_within`
memmoves for the general overlap case eliminated the bottleneck:

| Bench                  | Before     | After       | Δ      |
|------------------------|------------|-------------|--------|
| decode/repeated/1024   | 1.03 GiB/s | 48.1 GiB/s  | +47x   |
| decode/repeated/10240  | 1.01 GiB/s | 99.7 GiB/s  | +99x   |
| decode/repeated/102400 | 0.99 GiB/s | 42.0 GiB/s  | +42x   |
| decode/text/1024       | 2.58 GiB/s | 28.1 GiB/s  | +10.9x |
| decode/text/10240      | 2.99 GiB/s | 71.4 GiB/s  | +23.9x |
| decode/text/102400     | 3.20 GiB/s | 39.0 GiB/s  | +12.2x |
| decode/random/1024     | 13.4 GiB/s | 38.4 GiB/s  | +2.9x  |
| decode/random/10240    | 45.3 GiB/s | 81.2 GiB/s  | +1.8x  |
| decode/sequential/1024 | 13.0 GiB/s | 38.1 GiB/s  | +2.9x  |
| decode/sequential/10240| 46.0 GiB/s | 80.5 GiB/s  | +1.8x  |

The non-overlapping cases also picked up because simplifying the function
shape allowed `copy_within` to inline more aggressively. The 100 KB
random/sequential cases were already DRAM-bound and didn't move.

## Roundtrip Performance (Encode + Decode)

| Data Size | Pattern  | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|----------|-----------|--------------|-------------|---------------|
| 1KB       | Text     | 329       | 827          | 867         | 2.6x          |
| 1KB       | Repeated | 294       | 835          | 875         | 3.0x          |
| 10KB      | Text     | 354       | 2005         | 2102        | 5.9x          |
| 10KB      | Repeated | 302       | 2029         | 2126        | 7.0x          |

## Summary

### Rust advantages
1. **Decode performance**: 54–199x faster than Go across all patterns; the
   `repeated`/`text` patterns are no longer outliers.
2. **Standard encoding**: Faster than Go in every measured case except 1 KB
   repeated.
3. **Best mode**: 12–16x faster than Go where Go benchmarks are available,
   while producing byte-identical output for interop.

### Go advantages
1. **Better mode on 10 KB text**: Go is still ~5% ahead on this specific
   pattern; on other patterns / sizes Rust matches or beats Go.
2. **Memory efficiency**: Go's per-call allocation profile is still leaner.

### Next steps

Remaining opportunities to investigate:
1. **Buffer reuse on the `Encoder` struct.** Each call to `encode()` /
   `encode_better()` allocates a fresh 64 KB–512 KB hash table. Plumbing
   buffer reuse through `Encoder` would help small-block encoding latency
   without changing the convenience-function API.
2. **`encode_better` on small inputs.** The 1 KB random case (1.05 GiB/s)
   is materially slower than the same encoder on 10 KB+ (2.0 GiB/s),
   suggesting per-call fixed overhead.
3. **SIMD-assisted match extension.** The 8-byte XOR / trailing-zero loop
   in match extension could be unrolled or vectorised.
