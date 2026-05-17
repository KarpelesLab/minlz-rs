# Performance Benchmarks: Rust vs Go S2 Implementation

This document records the performance of the Rust implementation (`minlz`)
against the Go reference (`github.com/klauspost/compress/s2`, v1.18.6).
Both columns were measured on the **same machine** with single-threaded
benchmarks using identical input patterns and sizes.

## Test Environment

- CPU: Intel Core i9-14900K
- OS: Linux 6.12.41-gentoo
- Rust: minlz 1.0.x (rustc 1.95.0), `RUSTFLAGS="-C target-cpu=native"`
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

Go uses hand-written AMD64 assembly with explicit SIMD for the
match-search inner loop; minlz is pure Rust relying on LLVM
autovectorization.

| Data Size | Pattern    | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|------------|-------------|-----------|-----------|
| 1 KB      | Random     | 4720        | 6502      | 0.73×     |
| 1 KB      | Repeated   | 6115        | 13308     | 0.46×     |
| 1 KB      | Text       | 5811        | 10105     | 0.58×     |
| 1 KB      | Sequential | 4401        | 6367      | 0.69×     |
| 10 KB     | Random     | 8839        | 24851     | 0.36×     |
| 10 KB     | Repeated   | 9468        | 28785     | 0.33×     |
| 10 KB     | Text       | 9139        | 27667     | 0.33×     |
| 10 KB     | Sequential | 6221        | 24311     | 0.26×     |
| 100 KB    | Random     | 9498        | 33225     | 0.29×     |
| 100 KB    | Repeated   | 9013        | 32433     | 0.28×     |
| 100 KB    | Text       | 8994        | 32837     | 0.27×     |
| 100 KB    | Sequential | 9665        | 32444     | 0.30×     |

Go's AVX2 literal-emit and table lookups beat us by 2–4× on this
mode. This is the cleanest place for future SIMD work in `minlz`.

### Better (`encode_better` / `s2.EncodeBetter`)

| Data Size | Pattern   | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|-----------|-------------|-----------|-----------|
| 1 KB      | Random    | 3367        | 3547      | 0.95×     |
| 1 KB      | Repeated  | 5622        | 6049      | 0.93×     |
| 1 KB      | Text      | 4144        | 4328      | 0.96×     |
| 10 KB     | Random    | 11272       | 7259      | **1.55×** |
| 10 KB     | Repeated  | 11121       | 8212      | **1.35×** |
| 10 KB     | Text      | 11651       | 7813      | **1.49×** |
| 100 KB    | Random    | 8589        | 10508     | 0.82×     |
| 100 KB    | Repeated  | 8932        | 10649     | 0.84×     |
| 100 KB    | Text      | 8631        | 10791     | 0.80×     |

Mixed picture: minlz wins decisively in the L1/L2-resident 10 KB
range, Go wins on 100 KB inputs (memory bandwidth + assembly), and
they're effectively tied at 1 KB.

### Best (`encode_best` / `s2.EncodeBest`)

| Data Size | Pattern  | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|----------|-------------|-----------|-----------|
| 1 KB      | Repeated | 10.9        | 11.9      | 0.91×     |
| 1 KB      | Text     | 10.9        | 11.7      | 0.93×     |
| 10 KB     | Repeated | 105         | 112       | 0.94×     |
| 10 KB     | Text     | 109         | 116       | 0.94×     |
| 100 KB    | Repeated | 686         | 686       | 1.00×     |
| 100 KB    | Text     | 1031        | 1038      | 0.99×     |

Essentially tied. Both implementations run the same multi-candidate
scoring algorithm; the work is bottlenecked by the algorithm itself,
not the inner loop. Output is byte-for-byte identical to Go.

## Decoding Performance

| Data Size | Pattern    | Rust (GiB/s) | Rust (MB/s) | Go (MB/s) | Rust / Go |
|-----------|------------|--------------|-------------|-----------|-----------|
| 1 KB      | Random     |  38.1        | 40950       | 6360      | **6.4×**  |
| 1 KB      | Repeated   |  43.4        | 46638       | 4874      | **9.6×**  |
| 1 KB      | Text       |  29.7        | 31921       | 5075      | **6.3×**  |
| 1 KB      | Sequential |  38.7        | 41560       | 6016      | **6.9×**  |
| 10 KB     | Random     | 103.6        | 111301      | 5279      | **21.1×** |
| 10 KB     | Repeated   | 134.8        | 144800      | 5375      | **26.9×** |
| 10 KB     | Text       |  91.4        | 98135       | 5327      | **18.4×** |
| 10 KB     | Sequential | 106.9        | 114774      | 4836      | **23.7×** |
| 100 KB    | Random     |  70.6        | 75793       | 5308      | **14.3×** |
| 100 KB    | Repeated   |  77.5        | 83257       | 5529      | **15.1×** |
| 100 KB    | Text       |  68.3        | 73347       | 5390      | **13.6×** |
| 100 KB    | Sequential |  71.4        | 76660       | 5221      | **14.7×** |

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
| 1 KB      | Text     | 4209         | 4414        |
| 1 KB      | Repeated | 5256         | 5512        |
| 10 KB     | Text     | 7304         | 7657        |
| 10 KB     | Repeated | 7584         | 7951        |

## Encoder (buffer reuse)

The stateful `Encoder` keeps its hash tables across calls. Reuse
matters most on small inputs:

| Mode     | Size  | Pattern | Free fn      | Encoder     | Δ     |
|----------|-------|---------|--------------|-------------|-------|
| Standard | 1024  | Text    | 5.41 GiB/s   | 5.83 GiB/s  | +8%   |
| Better   | 1024  | Random  | 3.14 GiB/s   | 4.09 GiB/s  | +30%  |
| Better   | 1024  | Text    | 3.86 GiB/s   | 5.20 GiB/s  | +35%  |
| Better   | 10240 | Random  | 10.50 GiB/s  | 11.53 GiB/s | +10%  |
| Better   | 10240 | Text    | 10.85 GiB/s  | 11.15 GiB/s | +3%   |
| Best     | 10240 | Text    | 109 MiB/s    | 111 MiB/s   | +2%   |

`encode_best` doesn't benefit much from Encoder reuse because its
4.5 MiB hash table is zero-filled on every call. Eliminating that
memset (e.g. via a generation-based eviction scheme) is the main
remaining lever for that mode.

## Summary

### Where minlz beats Go
- **Decode, every pattern, every size**: 6–27× faster. Peak 135 GiB/s
  on L1-resident inputs, ~70 GiB/s on DRAM-resident.
- **`encode_better` on 10 KB inputs**: 1.35–1.55× faster than Go's
  AMD64 assembly path.

### Where Go beats minlz
- **`encode` (standard mode), all sizes**: Go is 2–4× faster, owing
  almost entirely to its hand-tuned AVX2 inner loop. Future versions
  of minlz could close this gap with explicit SIMD intrinsics or
  inline assembly.
- **`encode_better` on 100 KB inputs**: Go is ~20% faster.

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

## Remaining opportunities

### 1. Port klauspost/compress/s2's AMD64 asm variants to Rust

The 2–4× gap to Go on `encode_standard` is **not** primarily SIMD vs
scalar — it's **algorithmic**. A session of investigation (see notes
below) found that Go's encoder runs a fundamentally different and
more iteration-efficient search structure than what minlz currently
ports.

**What Go does per scalar iteration** (`encodeBlockGo64K` in
`encode_all.go` + the matching `encodeBlockAsm*` variants in
`encodeblock_amd64.s`):

- Loads one u64 window `cv = load64(src, s)`.
- Computes **three** 6-byte hashes — `hash6(cv)`, `hash6(cv >> 8)`,
  `hash6(cv >> 16)` — covering positions `s`, `s+1`, `s+2`.
- Looks up two candidates upfront and writes back `s` / `s+1`.
- **Checks the repeat offset first** — if `cv >> 8` matches
  `src[s+1-repeat..]`, take the repeat (essentially free on
  repeat-heavy data).
- Otherwise checks the three regular candidates in lane order.
- Advances by `(s - next_emit) >> 5 + 4` (or `>> 6 + 4` for the
  >64 KiB variant) — stride grows with how far we've moved past
  the last emit.

What minlz currently does per iteration: one 4-byte hash, one
candidate, one compare. ~3× the iterations to cover the same input.

**Why this didn't land in this session**:

- Direct port of `encodeBlockGo64K` gives only +5–9 % on 100 KiB
  cases and **breaks `tests/go_compatibility.rs`** on `repeated`
  patterns — because Go's amd64 assembly variants
  (`encodeBlockAsm10B`/`12B`/`4MB`) and Go's reference
  `encodeBlockGo64K` **don't always produce identical output**.
  Our existing compat test captures what the asm does (since the
  reference Go runner is amd64), so matching the reference
  diverges from the asm.
- The real path is **porting each size-specific asm variant**
  (`encodeBlockAsm8B` < 512 B, `…10B` < 4 KiB, `…12B` < 16 KiB,
  `…4MB` < 4 MiB, `encodeBlockAsm` ≥ 4 MiB) to bit-compatible Rust.
  Each is ~500 lines of assembly to read and translate carefully.
- Estimated effort: 3–5 focused sessions, with `go_compatibility`
  test as the bit-compat guardrail.

**Things we tried this session that didn't pan out**:

| Attempt | Result |
|---|---|
| Manual unroll-by-4 of search loop | +2–5 %, LLVM was already pipelining |
| `_mm_prefetch` on the 4 lookahead table slots | +3–9 % on 100 KiB cases (memory-bound) |
| `pulp` for portable SIMD | nightly only; bit-compat-preserving SIMD search is hard because batched lookups break collision ordering |
| `std::simd` (nightly only) | Same blocker as pulp + nightly is a non-starter for 1.x |
| Port `encodeBlockGo64K` directly | +5–9 % but breaks `tests/go_compatibility.rs` because Go reference ≠ Go asm on some inputs |

### 2. Generation-based eviction in `Encoder::encode_best`

The 4.5 MiB hash-table memset still dominates small-input cost.
Tracking a per-call generation in a separate `Vec<u8>` would
eliminate it without changing output.

### 3. Dictionary-aware better/best modes

`encode_better_with_dict` and `encode_best_with_dict` currently
fall through to their non-dict counterparts.
