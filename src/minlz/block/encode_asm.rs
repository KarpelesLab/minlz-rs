// Copyright 2024 Karpeles Lab Inc.
// Hand-written assembly Fastest-level block encoders for x86-64.
//
// `encode_block_amd64.s` is a mechanical Plan9->AT&T translation of the
// `encodeBlockAsm{512K,2MB,}` routines from github.com/minio/minlz (Apache-2.0):
// the same instruction sequences the Go reference runs, so they reproduce the
// reference encoder's hand-scheduled throughput rather than relying on LLVM's
// codegen of the scalar matcher. Each emits the *token stream only* (no block
// indicator / length prefix — the caller adds those) and self-zeroes the
// supplied scratch hash table. The baseline (non-GOAMD64_v3) path is used, so
// they require only SSE2 + CMOV, universally available on x86-64.
//
// As in the reference, the variant is chosen by input size: a 14-bit table for
// inputs up to 512 KiB keeps the random hash probes cache-resident (the bigger
// 15-bit table is measurably slower on low-locality mid-size inputs like text),
// a 15-bit table covers 512 KiB..2 MiB, and the offset-clamping variant covers
// the rest up to the 8 MiB block ceiling.

use core::cell::RefCell;

use alloc::vec::Vec;

core::arch::global_asm!(
    include_str!("encode_block_amd64.s"),
    options(att_syntax)
);

unsafe extern "C" {
    fn minlz_encode_block_asm_512k(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
    fn minlz_encode_block_asm_2mb(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
    fn minlz_encode_block_asm(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
    fn minlz_encode_better_asm_512k(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
    fn minlz_encode_better_asm_2mb(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
    fn minlz_encode_better_asm(d: *mut u8, s: *const u8, n: usize, tmp: *mut u8) -> usize;
}

/// Largest scratch any variant zeroes/uses: the "better" 2 MiB/clamp variants
/// pack a 2^17 long + 2^14 short table = 147456 u32 slots = 589824 bytes. The
/// smaller variants touch only a prefix, so one buffer serves them all.
const SCRATCH_LEN: usize = 589824;

std::thread_local! {
    /// Reused scratch hash table. The asm zeroes the region it uses on entry, so
    /// this is never initialised on the Rust side — only sized once.
    static SCRATCH: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Below this the per-call table-zeroing isn't worth it and the asm's search
/// margins are less battle-tested; the scalar matcher handles small inputs.
const ASM_MIN_INPUT: usize = 1024;
const SZ_512K: usize = 512 * 1024;
const SZ_2M: usize = 2 * 1024 * 1024;

/// Append the Fastest-level token stream for `src` to `out` using the hand-
/// written assembly encoder. Returns `true` if the asm path ran (and produced a
/// compressible result); `false` if the caller should fall back to the scalar
/// encoder (input too small, or asm reported the input incompressible).
/// One x86-64 asm encoder variant: `(dst, src, src_len, tmp) -> bytes_written`.
type AsmFn = unsafe extern "C" fn(*mut u8, *const u8, usize, *mut u8) -> usize;

/// Run `pick(n)`'s asm variant, appending its token stream to `out`. Shared by
/// the Fastest (greedy) and Balanced (better) entry points — they differ only
/// in which three size-tuned variants `pick` returns.
#[inline]
fn run_asm(out: &mut Vec<u8>, src: &[u8], pick: fn(usize) -> AsmFn) -> bool {
    let n = src.len();
    // The asm uses TZCNT (BMI1). `is_x86_feature_detected!` caches its result, so
    // this is a cheap load after the first call; on the rare BMI1-less x86-64 CPU
    // we fall back to the scalar matcher.
    if n < ASM_MIN_INPUT || !std::is_x86_feature_detected!("bmi1") {
        return false;
    }
    // The routine may store up to ~32 bytes past the final cursor while
    // wildcopy-ing literals; reserve generous slack so those writes stay
    // in-bounds. (`dstLimit` inside the asm keeps the logical output < n.)
    out.reserve(n + 64);
    let start = out.len();
    let f = pick(n);

    let written = SCRATCH.with(|cell| {
        let mut scratch = cell.borrow_mut();
        if scratch.len() < SCRATCH_LEN {
            scratch.resize(SCRATCH_LEN, 0);
        }
        let tmp = scratch.as_mut_ptr();
        // SAFETY: `dst` points at `out`'s spare capacity (grown to `n + 64`
        // past `start`); the asm writes < `n + 64` bytes there. `tmp` is a
        // 589824-byte buffer the routine self-zeroes and confines its table(s)
        // to. `src` is `n` readable bytes. `pick` returns a variant whose
        // tables fit `tmp` and whose offset handling is valid for `n` (the
        // clamping variant is used for n > 2 MiB).
        unsafe { f(out.as_mut_ptr().add(start), src.as_ptr(), n, tmp) }
    });

    if written == 0 {
        return false;
    }
    // SAFETY: the asm wrote exactly `written` valid token bytes starting at
    // `start`, and `start + written <= start + n < capacity`.
    unsafe { out.set_len(start + written) };
    true
}

/// Append the Fastest-level token stream for `src` to `out` via the hand-written
/// assembly matcher. Returns `false` (caller falls back to the scalar matcher)
/// when the input is too small or the asm reports it incompressible.
#[inline]
pub(super) fn encode_block_greedy_asm(out: &mut Vec<u8>, src: &[u8]) -> bool {
    run_asm(out, src, |n| {
        if n <= SZ_512K {
            minlz_encode_block_asm_512k
        } else if n <= SZ_2M {
            minlz_encode_block_asm_2mb
        } else {
            minlz_encode_block_asm
        }
    })
}

/// Append the Balanced-level ("better") token stream for `src` to `out` via the
/// hand-written assembly matcher. Returns `false` (caller falls back to scalar)
/// when the input is too small or the asm reports it incompressible.
#[inline]
pub(super) fn encode_block_better_asm(out: &mut Vec<u8>, src: &[u8]) -> bool {
    run_asm(out, src, |n| {
        if n <= SZ_512K {
            minlz_encode_better_asm_512k
        } else if n <= SZ_2M {
            minlz_encode_better_asm_2mb
        } else {
            minlz_encode_better_asm
        }
    })
}
