# MinLZ assembly encoder generator

`../../src/minlz/block/encode_block_amd64.s` is **generated**, not hand-edited.
It is a mechanical Plan9→AT&T translation of the `encodeBlockAsm*` /
`encodeBetterBlockAsm*` routines from the reference encoder
[github.com/minio/minlz](https://github.com/minio/minlz) (Apache-2.0), used by
the default-on `asm` feature for the `Fastest`/`Balanced` levels. See
`../../src/minlz/block/encode_asm.rs` for how it is wired in.

## Why generated

The routines reproduce the reference encoder's hand-scheduled instruction stream,
so we get its throughput rather than LLVM's codegen of the scalar matcher. Six
size-tuned variants are emitted: `encodeBlockAsm{512K,2MB,}` (greedy / `Fastest`)
and `encodeBetterBlockAsm{512K,2MB,}` (`Balanced`). Selection by input size and
the SysV-ABI wrapper live in `encode_asm.rs`.

## `translate.py`

```
python3 translate.py <plan9.s> <global_symbol> <epilogue_label> > out.s
```

Translation rules (Plan9 → AT&T):

- Operand order is preserved **except `CMP*`**, which swaps (Plan9 `CMP a,b`
  computes `a-b`; AT&T `cmp a,b` computes `b-a`).
- Register width comes from the mnemonic suffix (`Q/L/W/B`); memory base/index
  registers are always 64-bit.
- `MOVOU`/`MOVOA` → `movdqu` (unaligned; avoids alignment faults writing into the
  caller's unaligned output buffer).
- The `GOAMD64_v3` branch is taken: `BSF` → `TZCNT` (no false destination
  dependency, so it doesn't stall the tight match-length loop). This needs BMI1,
  which `encode_asm.rs` checks at runtime (scalar fallback otherwise).
- Go's frame-pointer args are remapped onto a fresh SysV stack frame; `RET`
  jumps to a per-function epilogue. Each function gets its labels prefixed with
  its symbol to avoid cross-variant collisions.

## Cache-resident table tuning

After translation, each variant's hash-finalize `shrq` shift is bumped to make
its L1-bound table one bit smaller (cache-resident → faster on low-locality
inputs). The `Balanced` long table is left at the reference size (it lives in L2
regardless, so shrinking it only costs ratio). This is the `asm`-feature
speed/ratio trade; the scalar matchers (feature off) keep the larger tables and
better ratio. The exact shifts are documented inline near the top of the `.s`.

## Regenerating

1. Locate the reference `asm_amd64.s` (Go module cache:
   `$(go env GOMODCACHE)/github.com/minio/minlz@*/asm_amd64.s`).
2. Extract each `TEXT ·<fn>(SB)` block through the next `TEXT`.
3. Run `translate.py` per variant with a unique symbol + epilogue label.
4. Concatenate under a `.text` header; apply the table-size `shrq` bumps.
5. Verify byte-exact against the Go decoder (differential fuzz) before trusting.
