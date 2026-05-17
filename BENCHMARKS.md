# Performance Benchmarks: Rust vs Go S2 Implementation

This document records the performance of the Rust implementation (`minlz`)
and compares it to the Go reference (`github.com/klauspost/compress/s2`)
where comparable numbers are available.

## Test Environment

- CPU: Intel Core i9-14900K
- OS: Linux 6.12.41-gentoo
- Rust: minlz 0.1.6 (rustc 1.95.0)
- Build: `RUSTFLAGS="-C target-cpu=native" cargo bench --bench compression`
- Harness: criterion 0.5 (100 samples / bench, 3 s warmup)

Go numbers below are carried over from the previous report
(klauspost/compress/s2, same CPU class) and are unchanged in this
revision — only the Rust column has been re-measured.

## Encoding Performance

### Standard Compression (`encode`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner    |
|-----------|------------|-----------|--------------|-------------|-----------|
| 1KB       | Random     | 734       | 4631         | 4856        | Rust 6.6× |
| 1KB       | Repeated   | 997       | 6052         | 6346        | Rust 6.4× |
| 1KB       | Text       | 887       | 4962         | 5202        | Rust 5.9× |
| 1KB       | Sequential | 739       | 4246         | 4452        | Rust 6.0× |
| 10KB      | Random     | 1280      | 8222         | 8623        | Rust 6.7× |
| 10KB      | Repeated   | 1199      | 8821         | 9251        | Rust 7.7× |
| 10KB      | Text       | 1312      | 8217         | 8617        | Rust 6.6× |
| 10KB      | Sequential | 1291      | 8421         | 8831        | Rust 6.8× |
| 100KB     | Random     | 1570      | 9540         | 10004       | Rust 6.4× |
| 100KB     | Repeated   | 1292      | 9949         | 10433       | Rust 8.1× |
| 100KB     | Text       | 1545      | 9737         | 10210       | Rust 6.6× |
| 100KB     | Sequential | 1231      | 9588         | 10054       | Rust 8.2× |

### Better Compression (`encode_better`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner    |
|-----------|------------|-----------|--------------|-------------|-----------|
| 1KB       | Random     | N/A       | 3288         | 3447        | -         |
| 1KB       | Repeated   | N/A       | 5423         | 5687        | -         |
| 1KB       | Text       | N/A       | 3973         | 4166        | -         |
| 10KB      | Random     | N/A       | 9376         | 9831        | -         |
| 10KB      | Repeated   | 1430      | 9199         | 9647        | Rust 6.7× |
| 10KB      | Text       | 2232      | 8972         | 9410        | Rust 4.2× |
| 100KB     | Random     | N/A       | 8298         | 8702        | -         |
| 100KB     | Repeated   | N/A       | 8768         | 9194        | -         |
| 100KB     | Text       | N/A       | 8655         | 9077        | -         |

### Best Compression (`encode_best`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner     |
|-----------|------------|-----------|--------------|-------------|------------|
| 1KB       | Repeated   | N/A       | 11.2         | 11.8        | -          |
| 1KB       | Text       | N/A       | 11.5         | 12.0        | -          |
| 10KB      | Repeated   | 7.04      | 107.9        | 113.1       | Rust 16×   |
| 10KB      | Text       | 7.15      | 111.7        | 117.1       | Rust 16×   |
| 100KB     | Repeated   | N/A       | 704          | 738         | -          |
| 100KB     | Text       | N/A       | 1080         | 1132        | -          |

**Note:** Best mode produces byte-identical output to Go's `s2.EncodeBest`.

## Decoding Performance

| Data Size | Pattern    | Go (MB/s) | Rust (GiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|------------|-----------|--------------|-------------|---------------|
| 1KB       | Random     | 672       | 41.6         | 44676       | 66x           |
| 1KB       | Repeated   | 547       | 50.2         | 53902       | 99x           |
| 1KB       | Text       | 560       | 32.7         | 35138       | 63x           |
| 1KB       | Sequential | N/A       | 42.0         | 45106       | N/A           |
| 10KB      | Random     | 538       | 106.6        | 114447      | 213x          |
| 10KB      | Repeated   | 537       | 139.4        | 149643      | 279x          |
| 10KB      | Text       | 509       | 94.2         | 101145      | 199x          |
| 10KB      | Sequential | N/A       | 109.2        | 117233      | N/A           |
| 100KB     | Random     | 654       | 72.7         | 78045       | 119x          |
| 100KB     | Repeated   | 685       | 80.9         | 86813       | 127x          |
| 100KB     | Text       | 627       | 70.7         | 75855       | 121x          |
| 100KB     | Sequential | N/A       | 70.6         | 75815       | N/A           |

10 KB cases sit in L1/L2 and reach 90–140 GiB/s — the decoder there is
memcpy-bound. 100 KB cases hit DRAM at ~70–80 GiB/s.

## Roundtrip Performance (Encode + Decode)

| Data Size | Pattern  | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|----------|-----------|--------------|-------------|---------------|
| 1KB       | Text     | 329       | 4607         | 4830        | 14.7x         |
| 1KB       | Repeated | 294       | 5713         | 5990        | 20.4x         |
| 10KB      | Text     | 354       | 7707         | 8081        | 22.8x         |
| 10KB      | Repeated | 302       | 7948         | 8333        | 27.6x         |

## What changed in 0.1.6

Three independent perf wins landed, plus a small follow-on refactor:

### 1. `load32` / `load64` — replace per-byte indexing with one slice load

The old helpers built a fixed-size byte array via 4 or 8 individual
indexed reads:

```rust
fn load64(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
    ])
}
```

LLVM cannot reliably elide N bounds checks for N indexed reads, so each
call expanded to 8 byte loads + 8 bounds checks instead of a single
unaligned 8-byte load. callgrind on `encode_better/random/1024` showed
54 % of cycles inside `load64`.

The new version does one slice + `try_into`:

```rust
fn load64(data: &[u8], offset: usize) -> u64 {
    let bytes: [u8; 4] = data[offset..offset + 4].try_into().expect(...);
    u32::from_le_bytes(bytes)
}
```

One bounds check, one MOV. Output is bit-for-bit identical.

| Bench                          | Before     | After       | Δ      |
|--------------------------------|------------|-------------|--------|
| encode_standard/random/1024    | 1.74 GiB/s | 4.44 GiB/s  | +156%  |
| encode_standard/repeated/10240 | 1.96 GiB/s | 7.79 GiB/s  | +297%  |
| encode_standard/text/102400    | 2.03 GiB/s | 8.50 GiB/s  | +318%  |
| encode_better/random/10240     | 1.91 GiB/s | 8.88 GiB/s  | +366%  |
| encode_better/repeated/10240   | 2.09 GiB/s | 9.55 GiB/s  | +357%  |

### 2. Decoder: skip `dst` zero-fill

`vec![0u8; dlen]` calloc-style zero-fills the entire destination buffer
before the decoder runs. The decoder writes every byte from `0..dlen`
sequentially before returning Ok and only reads from positions it has
already written, so the zero-fill is pure overhead. Profile showed
~80 % of decode cycles were the memset.

Replaced with `Vec::with_capacity + set_len` (uninitialized bytes); the
decoder's own invariants make this sound (extensively commented in
`alloc_uninit_dst`).

| Bench                    | Before     | After       | Δ     |
|--------------------------|------------|-------------|-------|
| decode/random/102400     | 37.8 GiB/s | 72.7 GiB/s  | +92%  |
| decode/repeated/102400   | 40.3 GiB/s | 80.9 GiB/s  | +101% |
| decode/text/102400       | 37.9 GiB/s | 70.7 GiB/s  | +86%  |
| decode/sequential/102400 | 37.8 GiB/s | 70.6 GiB/s  | +87%  |
| decode/text/10240        | 69.3 GiB/s | 94.2 GiB/s  | +36%  |
| decode/random/10240      | 77.9 GiB/s | 106.6 GiB/s | +37%  |

### 3. Encoders: same trick for the dst buffer

All five encode entry points (`encode`, `encode_better`,
`encode_with_dict`, `encode_snappy`, `encode_best`) wrote the dst
buffer in strictly increasing index order without reading. Applying
the same `alloc_uninit_dst` helper gave another 10–15 % on top of the
load-helper win for larger inputs.

| Bench                            | Pre-uninit | After-uninit | Δ    |
|----------------------------------|------------|--------------|------|
| encode_standard/random/102400    | 8.34 GiB/s | 9.33 GiB/s   | +12% |
| encode_standard/repeated/102400  | 8.65 GiB/s | 9.72 GiB/s   | +12% |
| encode_better/random/102400      | 7.70 GiB/s | 8.10 GiB/s   |  +5% |
| encode_better/text/102400        | 7.74 GiB/s | 8.45 GiB/s   |  +9% |

### Combined effect

vs. v0.1.5 (post bug-fix, no perf changes):

| Bench                          | v0.1.5     | v0.1.6      | Δ      |
|--------------------------------|------------|-------------|--------|
| encode_standard/random/1024    | 1.74 GiB/s | 4.52 GiB/s  | +160%  |
| encode_standard/repeated/102400| 2.03 GiB/s | 9.72 GiB/s  | +379%  |
| encode_better/random/10240     | 1.91 GiB/s | 9.16 GiB/s  | +380%  |
| encode_better/random/102400    | 1.90 GiB/s | 8.10 GiB/s  | +326%  |
| encode_best/text/102400        |  750 MiB/s | 1.05 GiB/s  |  +43%  |
| decode/random/102400           | 37.8 GiB/s | 72.7 GiB/s  |  +92%  |
| decode/repeated/102400         | 40.3 GiB/s | 80.9 GiB/s  | +101%  |
| roundtrip/text/10240           | 1.87 GiB/s | 7.53 GiB/s  | +303%  |
| roundtrip/repeated/10240       | 1.96 GiB/s | 7.76 GiB/s  | +296%  |

Standard encoding picks up roughly **4×** across the board; better
encoding picks up roughly **3–4×** on medium inputs; the decoder
roughly **doubles** on DRAM-bound 100 KB cases.

## Summary

### Rust advantages
1. **Decode performance**: 63–279× faster than Go across all patterns.
2. **Standard encoding**: 6–8× faster than Go on every measured case.
3. **Better encoding**: 4–7× faster than Go where Go numbers exist.
4. **Best mode**: 16× faster than Go where Go benchmarks are available,
   with byte-identical output for interop.

### Go advantages

(None visible in current measurements.)

### Remaining opportunities

1. **Hash-table memset on small inputs.** `encode_best` still spends
   ~76 % of cycles zeroing its 4.5 MB hash tables; `encode_better/1024`
   ~60 %. Eliminating that requires either a `&mut Encoder` API that
   reuses buffers across calls or a generation-based eviction scheme.
2. **SIMD-assisted match extension.** The 8-byte XOR / trailing-zero
   loop in match extension could be vectorised further.
3. **Snappy / dict encoders.** Not yet benched; same load-helper and
   uninit-dst wins should apply directly.
