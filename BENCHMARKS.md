# Performance Benchmarks: Rust vs Go S2 Implementation

This document records the performance of the Rust implementation (`minlz`)
against the Go reference (`github.com/klauspost/compress/s2`, v1.18.6).
Both columns were measured on the **same machine** with single-threaded
benchmarks using identical input patterns and sizes.

## Test Environment

- CPU: Intel Core i9-14900K
- OS: Linux 6.12.41-gentoo
- Rust: minlz 1.1.0 (rustc 1.95.0), `RUSTFLAGS="-C target-cpu=native"`
- Go: 1.25.3, `GOAMD64=v3` (enables AVX2)
- Harness: criterion 0.5 for Rust (100 samples / bench, 3 s warmup);
  `go test -bench -benchtime=2s -cpu=1` for Go
- Inputs are generated identically in both harnesses (see
  `benches/compression.rs` and `bench_test.go`).

**Note**: The often-quoted ~50 GB/s figure in the Go README is the
**parallel 16-core aggregate**. Per-core throughput (what matters for
a like-for-like comparison) is roughly 1/16 of that.

## Encoding Performance

Throughput in MB/s (decimal megabytes / second). Rust columns converted
from MiB/s to MB/s using `× 1.048576`.

### Standard (`encode` / `s2.Encode`)

Both implementations now use the same fundamental algorithm — minlz
ports klauspost/compress/s2's per-size-bucket asm variants
(`encodeBlockAsm8B`/`10B`/`12B`/`4MB`) to bit-compatible Rust. Three
hashes per scalar iteration covering s, s+1, s+2, free repeat-first
check, plus the same `(s − next_emit) >> N + 4` skip stride. Output
is byte-for-byte identical to Go's `s2.Encode` on every tested
input. Go retains an edge mainly because its asm uses AVX2 SIMD
`memmove` for the large literal copies; minlz still relies on
`copy_from_slice` (LLVM autovec).

| Data Size | Pattern    | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|------------|-------------|-----------|-----------|
| 1 KB      | Random     |  5362       |  6502     | 0.82×     |
| 1 KB      | Repeated   |  7046       | 13308     | 0.53×     |
| 1 KB      | Text       |  7024       | 10105     | 0.70×     |
| 1 KB      | Sequential |  5460       |  6367     | 0.86×     |
| 10 KB     | Random     | 17616       | 24851     | 0.71×     |
| 10 KB     | Repeated   | 13968       | 28785     | 0.49×     |
| 10 KB     | Text       | 18403       | 27667     | 0.67×     |
| 10 KB     | Sequential | 17569       | 24311     | 0.72×     |
| 100 KB    | Random     | 21300       | 33225     | 0.64×     |
| 100 KB    | Repeated   | 15007       | 32433     | 0.46×     |
| 100 KB    | Text       | 21613       | 32837     | 0.66×     |
| 100 KB    | Sequential | 21285       | 32444     | 0.66×     |

### Better (`encode_better` / `s2.EncodeBetter`)

| Data Size | Pattern   | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|-----------|-------------|-----------|-----------|
| 1 KB      | Random    | 3498        | 3547      | 0.99×     |
| 1 KB      | Repeated  | 5533        | 6049      | 0.91×     |
| 1 KB      | Text      | 4277        | 4328      | 0.99×     |
| 10 KB     | Random    | 11383       | 7259      | **1.57×** |
| 10 KB     | Repeated  | 11764       | 8212      | **1.43×** |
| 10 KB     | Text      | 11476       | 7813      | **1.47×** |
| 100 KB    | Random    | 8881        | 10508     | 0.85×     |
| 100 KB    | Repeated  | 9060        | 10649     | 0.85×     |
| 100 KB    | Text      | 8897        | 10791     | 0.82×     |

Mixed picture: minlz wins decisively in the L1/L2-resident 10 KB
range, Go wins on 100 KB inputs (memory bandwidth + assembly), and
they're effectively tied at 1 KB.

### Best (`encode_best` / `s2.EncodeBest`)

| Data Size | Pattern  | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|----------|-------------|-----------|-----------|
| 1 KB      | Repeated | 12          | 11.9      | 1.01×     |
| 1 KB      | Text     | 12          | 11.7      | 1.03×     |
| 10 KB     | Repeated | 112         | 112       | 1.00×     |
| 10 KB     | Text     | 116         | 116       | 1.00×     |
| 100 KB    | Repeated | 724         | 686       | 1.06×     |
| 100 KB    | Text     | 1108        | 1038      | 1.07×     |

Essentially tied. Both implementations run the same multi-candidate
scoring algorithm; the work is bottlenecked by the algorithm itself,
not the inner loop. Output is byte-for-byte identical to Go.

## Decoding Performance

| Data Size | Pattern    | Rust (GiB/s) | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|------------|--------------|-------------|-----------|-----------|
| 1 KB      | Random     |  37.8        | 40597       | 6360      | **6.4×**  |
| 1 KB      | Repeated   |  42.6        | 45700       | 4874      | **9.4×**  |
| 1 KB      | Text       |  29.9        | 32145       | 5075      | **6.3×**  |
| 1 KB      | Sequential |  38.5        | 41332       | 6016      | **6.9×**  |
| 10 KB     | Random     | 103.8        | 111442      | 5279      | **21.1×** |
| 10 KB     | Repeated   | 127.2        | 136572      | 5375      | **25.4×** |
| 10 KB     | Text       |  90.1        | 96712       | 5327      | **18.2×** |
| 10 KB     | Sequential | 104.0        | 111696      | 4836      | **23.1×** |
| 100 KB    | Random     |  71.0        | 76267       | 5308      | **14.4×** |
| 100 KB    | Repeated   |  79.2        | 85034       | 5529      | **15.4×** |
| 100 KB    | Text       |  69.8        | 74970       | 5390      | **13.9×** |
| 100 KB    | Sequential |  71.3        | 76576       | 5221      | **14.7×** |

The decode side is where minlz pulls clearly ahead — Go's per-core
decode tops out around 5–6 GB/s, while minlz peaks at 135 GiB/s on
L1/L2-resident inputs and is memory-bandwidth-bound at 70+ GiB/s for
DRAM-resident inputs. The improvements come from:
- Skipping the `vec![0; n]` zero-fill of the destination buffer.
- Replacing the byte-by-byte overlapping-copy loop with `slice::fill`
  + memmove doubling.
- Slice + `try_into` for the unaligned word loads.

## Roundtrip (Encode + Decode)

The roundtrip is mostly dominated by encode now:

| Data Size | Pattern  | Rust (MiB/s) | Rust (MB/s) |
|-----------|----------|--------------|-------------|
| 1 KB      | Text     | 5751         | 6030        |
| 1 KB      | Repeated | 5897         | 6183        |
| 10 KB     | Text     | 14945        | 15670       |
| 10 KB     | Repeated | 12213        | 12807       |

## Encoder (buffer reuse)

The stateful `Encoder` keeps its hash tables across calls. Reuse
matters most on small inputs:

| Mode     | Size  | Pattern | Free fn      | Encoder     | Δ     |
|----------|-------|---------|--------------|-------------|-------|
| Standard | 1024  | Text    | 6.54 GiB/s   | 8.26 GiB/s  | +26%  |
| Better   | 1024  | Random  | 3.26 GiB/s   | 4.13 GiB/s  | +27%  |
| Better   | 1024  | Text    | 3.98 GiB/s   | 5.35 GiB/s  | +34%  |
| Better   | 10240 | Random  | 10.60 GiB/s  | 12.19 GiB/s | +15%  |
| Better   | 10240 | Text    | 10.69 GiB/s  | 12.33 GiB/s | +15%  |
| Best     | 10240 | Text    | 116 MiB/s    | 121 MiB/s   | +4%   |

`encode_best` doesn't benefit much from Encoder reuse because its
4.5 MiB hash table is zero-filled on every call. Eliminating that
memset (e.g. via a generation-based eviction scheme) is the main
remaining lever for that mode.

## Summary

### Where minlz beats Go
- **Decode, every pattern, every size**: 6–25× faster. Peak 127 GiB/s
  on L1-resident inputs, ~70 GiB/s on DRAM-resident.
- **`encode_better` on 10 KB inputs**: 1.43–1.57× faster than Go's
  AMD64 assembly path.

### Where Go beats minlz
- **`encode` (standard mode), repeat-heavy data**: Go is ~2× faster.
  All remaining gap is in the literal/copy `memmove` — Go uses AVX2
  16-byte SIMD moves; we use `copy_from_slice` (LLVM autovec). For
  other patterns the gap is now 0.6–0.9×, down from 0.3× before the
  asm port.
- **`encode_better` on 100 KB inputs**: Go is ~15–20% faster.

### Where they tie
- **`encode_best`**: within ±10 % on every size/pattern combination.
  Both implementations are bottlenecked by the multi-candidate
  scoring algorithm rather than the inner loop. Output is
  byte-for-byte identical for interop.

## Binary compatibility

Output of all four encode modes (`encode`, `encode_better`,
`encode_best`, `encode_snappy`) is byte-for-byte identical to Go's
corresponding functions on every test input — verified by
`tests/go_compatibility.rs`, `tests/better_compatibility.rs`,
`tests/best_compatibility.rs`, and `tests/snappy_compat.rs`.

## What was optimized along the way

| Release | Change | Headline effect |
|---------|--------|-----------------|
| 0.1.4   | Decoder: replace byte-by-byte overlapping copy with `slice::fill` + doubling memmoves | `decode/repeated` 1 GiB/s → 40–99 GiB/s |
| 0.1.4   | Standard encoder: input-adaptive hash table size | 1 KB encode 0.87 → 1.8 GiB/s |
| 0.1.5   | **Correctness fix**: encoder match-extension off-by-N (silent data corruption on ~95% of run-heavy inputs) | Bug closed; permanent regression test |
| 0.1.6   | `load32`/`load64` use slice + `try_into` (single MOV instead of N indexed reads) | 3–4× universal encoder boost |
| 0.1.6   | Decoder skips dst zero-fill via `Vec::with_capacity + set_len` | `decode/100 KB` 37 → 71 GiB/s |
| 0.1.6   | All encoders also skip dst zero-fill | +10–15% on large encode inputs |
| 1.0     | Stateful `Encoder` API with reusable hash-table buffers; same trick applied to Snappy | +30% on 1 KB `encode_better` |
| 1.0.1   | `try_reserve_exact` in decoder dst alloc (fuzz-caught OOM) | Decoder cannot abort process on adversarial input |
| 1.0.2   | Hard cap `MAX_DECODE_DST_SIZE = 256 MiB` (second fuzz-caught OOM) | Decoder returns `Err(TooLarge)` instead of `OOM` |
| 1.1.0   | Port of klauspost/compress/s2's `encodeBlockAsm{8B,10B,12B,4MB}` to Rust: 3 hashes per iter + free repeat-first check + size-bucketed dispatch | `encode/10 KB` 16 → 18 GiB/s, `encode/100 KB` 20 → 21 GiB/s, byte-for-byte Go-compatible output |
| 1.1.0   | **Correctness fix**: two fuzz-caught bugs in the asm-port (SIMD match-extension off-by-N + repeat-shorthand on first emit) | Bugs closed; permanent regression tests |

## Remaining opportunities

1. **SIMD-assisted standard encoder**: the 2–4× gap to Go on
   `encode` is almost entirely the AMD64 assembly inner loop. Hand-
   written `std::arch` intrinsics for x86-64 with a portable
   scalar fallback would close most of it.
2. **Generation-based eviction in `Encoder::encode_best`**: the
   4.5 MiB hash-table memset still dominates small-input cost.
3. **Dictionary-aware better/best modes**: `encode_better_with_dict`
   and `encode_best_with_dict` currently fall through to their
   non-dict counterparts.
