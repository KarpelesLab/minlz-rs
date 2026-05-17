# Performance Benchmarks: Rust vs Go S2 Implementation

This document records the performance of the Rust implementation (`minlz`)
and compares it to the Go reference (`github.com/klauspost/compress/s2`)
where comparable numbers are available.

## Test Environment

- CPU: Intel Core i9-14900K
- OS: Linux 6.12.41-gentoo
- Rust: minlz 1.0 (rustc 1.95.0)
- Build: `RUSTFLAGS="-C target-cpu=native" cargo bench --bench compression`
- Harness: criterion 0.5 (100 samples / bench, 3 s warmup)

Go numbers below are carried over from the original report
(klauspost/compress/s2, same CPU class) for historical comparison —
only the Rust column has been re-measured.

## Encoding Performance — free functions

These numbers reflect the canonical `encode`, `encode_better`,
`encode_best`, and `encode_snappy` free functions, which allocate a
fresh hash table on each call.

### Standard (`encode`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner   |
|-----------|------------|-----------|--------------|-------------|----------|
| 1KB       | Random     | 734       | 4514         | 4734        | Rust 6.5×|
| 1KB       | Repeated   | 997       | 6160         | 6459        | Rust 6.5×|
| 1KB       | Text       | 887       | 5301         | 5559        | Rust 6.3×|
| 1KB       | Sequential | 739       | 4290         | 4499        | Rust 6.1×|
| 10KB      | Random     | 1280      | 8330         | 8736        | Rust 6.8×|
| 10KB      | Repeated   | 1199      | 8851         | 9282        | Rust 7.7×|
| 10KB      | Text       | 1312      | 8967         | 9402        | Rust 7.2×|
| 10KB      | Sequential | 1291      | 8405         | 8814        | Rust 6.8×|
| 100KB     | Random     | 1570      | 9460         | 9920        | Rust 6.3×|
| 100KB     | Repeated   | 1292      | 10017        | 10503       | Rust 8.1×|
| 100KB     | Text       | 1545      | 9863         | 10342       | Rust 6.7×|
| 100KB     | Sequential | 1231      | 9509         | 9971        | Rust 8.1×|

### Better (`encode_better`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner   |
|-----------|------------|-----------|--------------|-------------|----------|
| 1KB       | Random     | N/A       | 3331         | 3492        | -        |
| 1KB       | Repeated   | N/A       | 5117         | 5366        | -        |
| 1KB       | Text       | N/A       | 4019         | 4214        | -        |
| 10KB      | Random     | N/A       | 11095        | 11633       | -        |
| 10KB      | Repeated   | 1430      | 11173        | 11716       | Rust 8.2×|
| 10KB      | Text       | 2232      | 10987        | 11521       | Rust 5.2×|
| 100KB     | Random     | N/A       | 8503         | 8916        | -        |
| 100KB     | Repeated   | N/A       | 8819         | 9248        | -        |
| 100KB     | Text       | N/A       | 8542         | 8957        | -        |

### Best (`encode_best`)

| Data Size | Pattern    | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Winner   |
|-----------|------------|-----------|--------------|-------------|----------|
| 1KB       | Repeated   | N/A       | 11.2         | 11.7        | -        |
| 1KB       | Text       | N/A       | 11.3         | 11.8        | -        |
| 10KB      | Repeated   | 7.04      | 106.9        | 112.1       | Rust 16× |
| 10KB      | Text       | 7.15      | 109.6        | 114.9       | Rust 16× |
| 100KB     | Repeated   | N/A       | 704          | 738         | -        |
| 100KB     | Text       | N/A       | 1040         | 1090        | -        |

`encode_best` output is bit-for-bit identical to Go's `s2.EncodeBest`
on every test input.

## Encoding Performance — `Encoder` (buffer reuse)

The stateful `Encoder` keeps its hash tables across calls. The numbers
below are for `b.iter(|| enc.encode_*(data))` — i.e. each iteration
reuses the buffers warmed up on the previous one.

| Mode     | Data Size | Pattern  | Free fn      | Encoder       | Δ     |
|----------|-----------|----------|--------------|---------------|-------|
| Standard | 1024      | Random   | 4.51 GiB/s   | 4.58 GiB/s    |  +2%  |
| Standard | 1024      | Text     | 5.18 GiB/s   | 5.83 GiB/s    | +13%  |
| Standard | 10240     | Random   | 8.13 GiB/s   | 8.27 GiB/s    |  +2%  |
| Standard | 10240     | Text     | 8.76 GiB/s   | 8.52 GiB/s    |  flat |
| Standard | 102400    | Random   | 9.24 GiB/s   | 8.97 GiB/s    |  flat |
| Standard | 102400    | Text     | 9.63 GiB/s   | 9.63 GiB/s    |  flat |
| Better   | 1024      | Random   | 3.25 GiB/s   | 4.08 GiB/s    | +25%  |
| Better   | 1024      | Text     | 3.93 GiB/s   | 5.20 GiB/s    | +32%  |
| Better   | 10240     | Random   | 10.84 GiB/s  | 11.53 GiB/s   |  +6%  |
| Better   | 10240     | Text     | 10.73 GiB/s  | 11.15 GiB/s   |  +4%  |
| Better   | 102400    | Random   | 8.30 GiB/s   | 8.30 GiB/s    |  flat |
| Better   | 102400    | Text     | 8.34 GiB/s   | 8.45 GiB/s    |  +1%  |
| Best     | 1024      | Repeated | 11.2 MiB/s   | 11.3 MiB/s    |  flat |
| Best     | 1024      | Text     | 11.3 MiB/s   | 11.1 MiB/s    |  flat |
| Best     | 10240     | Repeated | 106.9 MiB/s  | 105.8 MiB/s   |  flat |
| Best     | 10240     | Text     | 109.6 MiB/s  | 111.2 MiB/s   |  +1%  |
| Best     | 102400    | Text     | 1040 MiB/s   | 1010 MiB/s    |  flat |

The reuse win is biggest on 1 KB `encode_better` (where the small-table
memset and per-call alloc together dominate). `encode_best` doesn't
benefit much from `Encoder` because its 4.5 MiB hash table still gets
zero-filled every call — the alloc cost is small relative to the
memset itself.

## Decoding Performance

| Data Size | Pattern    | Go (MB/s) | Rust (GiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|------------|-----------|--------------|-------------|---------------|
| 1KB       | Random     | 672       | 40.5         | 43439       | 65x           |
| 1KB       | Repeated   | 547       | 46.9         | 50311       | 92x           |
| 1KB       | Text       | 560       | 30.8         | 33041       | 59x           |
| 1KB       | Sequential | N/A       | 41.7         | 44724       | N/A           |
| 10KB      | Random     | 538       | 110.2        | 118353      | 220x          |
| 10KB      | Repeated   | 537       | 134.9        | 144831      | 270x          |
| 10KB      | Text       | 509       | 94.1         | 101010      | 198x          |
| 10KB      | Sequential | N/A       | 109.2        | 117278      | N/A           |
| 100KB     | Random     | 654       | 70.1         | 75252       | 115x          |
| 100KB     | Repeated   | 685       | 79.5         | 85419       | 125x          |
| 100KB     | Text       | 627       | 69.9         | 75064       | 120x          |
| 100KB     | Sequential | N/A       | 71.3         | 76544       | N/A           |

10 KB cases sit in L1/L2 and approach memory bandwidth at 95–135 GiB/s.
100 KB cases hit DRAM at 70–80 GiB/s.

## Roundtrip Performance (Encode + Decode)

| Data Size | Pattern  | Go (MB/s) | Rust (MiB/s) | Rust (MB/s) | Speedup vs Go |
|-----------|----------|-----------|--------------|-------------|---------------|
| 1KB       | Text     | 329       | 4314         | 4524        | 13.7x         |
| 1KB       | Repeated | 294       | 5394         | 5658        | 19.2x         |
| 10KB      | Text     | 354       | 8111         | 8504        | 24.0x         |
| 10KB      | Repeated | 302       | 8215         | 8615        | 28.5x         |

## What was optimized along the way to 1.0

| Release | Change | Headline win |
|---------|--------|--------------|
| 0.1.4   | Decoder: replace byte-by-byte loop in `copy_within` with `slice::fill` + doubling memmoves for overlapping copies. | `decode/repeated` 1 GiB/s → 40–99 GiB/s (40–99×) |
| 0.1.4   | `encode_standard`: hash table sized to input (10/12/14/17 bits) instead of always 14. | 1 KB encode 870 MiB/s → 1.8 GiB/s |
| 0.1.5   | **Correctness fix**: `encode_block` match-extension over-counted match length by `diff` after a partial-match break. Silent data corruption on ~95% of run-heavy inputs. | All affected inputs round-trip again |
| 0.1.6   | `load32`/`load64` now use slice + `try_into` so LLVM emits a single unaligned word load instead of N byte loads. | 3–4× universal encoder speedup |
| 0.1.6   | Decoder skips dst zero-fill via `Vec::with_capacity + set_len`; the decoder writes every byte before reading. | `decode/*/102400` 37 → 71 GiB/s (≈ 2×) |
| 0.1.6   | All encoders also skip dst zero-fill. | +10–15 % on large encoder inputs |
| 1.0     | Stateful `Encoder` API with reusable hash-table buffers; new `encode/encode_better/encode_best/encode_snappy` methods alongside the existing free functions. | +25–32 % on 1 KB `encode_better` |
| 1.0     | `encode_snappy` got the same input-adaptive table sizing as `encode_standard`. | small-input Snappy now sized to fit |
| 1.0     | `decode_into` made public for callers that bring their own buffer. | API completeness |

Cumulative vs. the original 0.1.3 baseline this whole effort started from:

| Bench                          | 0.1.3 baseline | 1.0          | Δ       |
|--------------------------------|----------------|--------------|---------|
| encode_standard/random/1024    |  848 MiB/s     | 4.51 GiB/s   | +446 %  |
| encode_standard/repeated/102400| 2.13 GiB/s     | 9.78 GiB/s   | +359 %  |
| encode_better/random/10240     | 2.08 GiB/s     | 10.84 GiB/s  | +421 %  |
| encode_better/text/102400      | 2.01 GiB/s     | 8.34 GiB/s   | +315 %  |
| encode_best/text/102400        |  849 MiB/s     | 1.02 GiB/s   |  +23 %  |
| decode/repeated/10240          | 1.14 GiB/s     | 134.9 GiB/s  | +11600 %|
| decode/repeated/102400         | 1.10 GiB/s     | 79.5 GiB/s   | +7124 % |
| decode/text/102400             | 8.27 GiB/s     | 69.9 GiB/s   | +745 %  |
| roundtrip/text/10240           | 1.74 GiB/s     | 7.92 GiB/s   | +355 %  |

## Summary

### Where Rust wins (1.0)
1. **Decode**: 59–270× faster than Go across every pattern.
2. **Standard encode**: 6–8× faster than Go everywhere.
3. **Better encode**: 5–8× faster than Go where Go numbers exist.
4. **Best encode**: 16× faster than Go for medium inputs, with
   byte-identical output for interop.

### Remaining opportunities (post-1.0)
1. **Generation-based eviction in `Encoder::encode_best`**: the 4.5 MiB
   hash-table memset is still ~75 % of `encode_best` time on small
   inputs. Tracking a per-call generation in a separate `Vec<u8>` would
   eliminate it.
2. **SIMD-assisted match extension**: the 8-byte XOR + trailing-zeros
   loop already auto-vectorises well, but a hand-tuned AVX2 path could
   double the match-extension throughput on long matches.
3. **Dictionary-aware better/best modes**: `encode_better_with_dict`
   and `encode_best_with_dict` currently fall through to their
   non-dict counterparts.
