// Copyright 2024 Karpeles Lab Inc.
// MinLZ block encoder. See ../mod.rs for licensing/attribution.
//
// A self-contained LZ encoder. The MinLZ spec leaves encoder output
// implementation-defined — any token stream the reference decoder accepts is
// conformant — so this does not attempt to match the reference encoder's bytes.
// `Fastest` is a single-table matcher that hashes three positions per iteration;
// `Balanced` is a two-hash-table ("better") matcher; `Smallest` is a deep
// hash-chain search with lazy matching. All share the same (verified) token
// emission — including fused literal+copy — and a repeat (last-offset) fast path.

use super::{
    Level, MAX_BLOCK_SIZE, MAX_COPY1_LENGTH, MAX_COPY1_OFFSET, MAX_COPY2_OFFSET, MAX_COPY3_OFFSET,
    MIN_COPY2_OFFSET, MIN_COPY3_OFFSET,
};
use crate::error::{Error, Result};
use crate::varint::{encode_varint, varint_size};
use alloc::vec::Vec;

/// Below this size, matching rarely pays for its overhead; store instead.
const MIN_NON_LITERAL_BLOCK_SIZE: usize = 16;
/// The match loop reads 4-byte words at `s`; keep that in bounds.
const INPUT_MARGIN: usize = 4;

/// Maximum number of bytes [`compress`] can produce for `src_len` input bytes,
/// or `None` if `src_len` exceeds [`MAX_BLOCK_SIZE`].
pub fn max_compressed_len(src_len: usize) -> Option<usize> {
    if src_len > MAX_BLOCK_SIZE {
        return None;
    }
    if src_len == 0 {
        return Some(1);
    }
    // Worst case is a stored block: indicator + zero-length varint + literals.
    Some(src_len + 2)
}

/// Compress `src` into a single MinLZ block at the default ([`Level::Fastest`])
/// level.
///
/// Returns [`Error::TooLarge`] if `src` exceeds [`MAX_BLOCK_SIZE`]; larger
/// inputs must be split across blocks (the streaming API does this).
pub fn compress(src: &[u8]) -> Result<Vec<u8>> {
    compress_level(src, Level::Fastest)
}

/// Compress `src` into a single MinLZ block at the given [`Level`].
pub fn compress_level(src: &[u8], level: Level) -> Result<Vec<u8>> {
    if src.len() > MAX_BLOCK_SIZE {
        return Err(Error::TooLarge);
    }
    if src.is_empty() {
        // Canonical empty block: a single zero indicator byte.
        return Ok(vec![0u8]);
    }
    if let Some(body) = compress_body(src, level) {
        let mut out = Vec::with_capacity(body.len() + 1);
        out.push(0u8);
        out.extend_from_slice(&body);
        return Ok(out);
    }
    Ok(store(src))
}

/// Produce the *body* of a compressed MinLZ block — `[uvarint(len)][tokens]`,
/// without the leading `0x00` indicator — but only if it is smaller than `src`.
/// Returns `None` when matching does not save space (the caller stores instead).
///
/// This is also the form used inside the stream format (chunk type 0x02).
pub(crate) fn compress_body(src: &[u8], level: Level) -> Option<Vec<u8>> {
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        return None;
    }
    let header = varint_size(src.len() as u64);
    let mut out = Vec::with_capacity(src.len());
    let mut lenbuf = [0u8; 10];
    let n = encode_varint(&mut lenbuf, src.len() as u64);
    out.extend_from_slice(&lenbuf[..n]);
    debug_assert_eq!(out.len(), header);

    match level {
        Level::Fastest => encode_block_greedy(&mut out, src),
        Level::Balanced => encode_block_better(&mut out, src),
        Level::Smallest => encode_block_chain(&mut out, src, 64, true),
    }

    // The decoder requires a compressed block to be no larger than its output.
    if out.len() - header < src.len() {
        Some(out)
    } else {
        None
    }
}

/// Encode `src` as a stored (literals-only) block: `0x00 0x00 <literals>`.
fn store(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() + 2);
    out.push(0);
    out.push(0);
    out.extend_from_slice(src);
    out
}

#[inline(always)]
fn load32(src: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([src[i], src[i + 1], src[i + 2], src[i + 3]])
}

#[inline(always)]
fn load64(src: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(src[i..i + 8].try_into().unwrap())
}

#[inline(always)]
fn hash4(v: u32, bits: u32) -> usize {
    (v.wrapping_mul(0x9E37_79B1) >> (32 - bits)) as usize
}

/// Hash of the lowest 7 bytes of `u` (for the "long" table).
#[inline(always)]
fn hash7(u: u64, bits: u32) -> usize {
    (((u << 8).wrapping_mul(58295818150454627)) >> (64 - bits)) as usize
}

/// Hash of the lowest 4 bytes of `u` (for the "short" table).
#[inline(always)]
fn hash4u(u: u64, bits: u32) -> usize {
    ((u as u32).wrapping_mul(2654435761) >> (32 - bits)) as usize
}

/// Hash of the lowest 6 bytes of `u` (for the Fastest single table).
#[inline(always)]
fn hash6(u: u64, bits: u32) -> usize {
    (((u << 16).wrapping_mul(227718039650203)) >> (64 - bits)) as usize
}

/// Hash-table size (log2) chosen from the input length.
fn table_bits(n: usize) -> u32 {
    let mut bits = 9;
    while (1usize << bits) < n && bits < 17 {
        bits += 1;
    }
    bits
}

/// Fastest single-table matcher (ported from the reference `encodeBlockGo`).
/// Hashes three positions per iteration (s, s+1, s+2), checks the last offset
/// (repeat), and after a copy keeps emitting copies as long as the immediately
/// following bytes match.
fn encode_block_greedy(out: &mut Vec<u8>, src: &[u8]) {
    let n = src.len();
    const TABLE_BITS: u32 = 15;
    let mut table = vec![0u32; 1usize << TABLE_BITS];

    let s_limit = n - 8; // loads read 8 bytes at s / next_s
    let mut next_emit = 0usize;
    let mut last_offset = 1usize; // the "repeat" offset
    let mut s = 1usize;
    let mut cv = load64(src, s);

    /// Forward-extend a 4-byte match: `s`/`c` point past the matched prefix.
    #[inline(always)]
    fn extend_fwd(src: &[u8], n: usize, mut s: usize, mut c: usize) -> usize {
        while s <= n - 8 {
            let diff = load64(src, s) ^ load64(src, c);
            if diff != 0 {
                return s + (diff.trailing_zeros() >> 3) as usize;
            }
            s += 8;
            c += 8;
        }
        s
    }

    'outer: loop {
        let mut candidate;
        loop {
            let next_s = s + ((s - next_emit) >> 6) + 4;
            if next_s > s_limit {
                break 'outer;
            }
            let min_src_pos = s as isize - MAX_COPY3_OFFSET as isize;
            let hash0 = hash6(cv, TABLE_BITS);
            let hash1 = hash6(cv >> 8, TABLE_BITS);
            candidate = table[hash0] as usize;
            let candidate2 = table[hash1] as usize;
            table[hash0] = s as u32;
            table[hash1] = (s + 1) as u32;
            let hash2 = hash6(cv >> 16, TABLE_BITS);

            // Repeat (last-offset) check on the 4 bytes at s+1.
            if (cv >> 8) as u32 == load32(src, s - last_offset + 1) {
                let mut base = s + 1;
                let mut i = base - last_offset;
                while base > next_emit && i > 0 && src[i - 1] == src[base - 1] {
                    i -= 1;
                    base -= 1;
                }
                emit_literals(out, &src[next_emit..base]);
                let ce = s - last_offset + 5;
                s = extend_fwd(src, n, s + 5, ce);
                emit_repeat(out, s - base);
                next_emit = s;
                if s >= s_limit {
                    break 'outer;
                }
                cv = load64(src, s);
                continue;
            }

            if candidate as isize >= min_src_pos && cv as u32 == load32(src, candidate) {
                break;
            }
            candidate = table[hash2] as usize;
            if candidate2 as isize >= min_src_pos && (cv >> 8) as u32 == load32(src, candidate2) {
                table[hash2] = (s + 2) as u32;
                candidate = candidate2;
                s += 1;
                break;
            }
            table[hash2] = (s + 2) as u32;
            if candidate as isize >= min_src_pos && (cv >> 16) as u32 == load32(src, candidate) {
                s += 2;
                break;
            }

            cv = load64(src, next_s);
            s = next_s;
        }

        // Extend backwards, then forwards.
        while candidate > 0 && s > next_emit && src[candidate - 1] == src[s - 1] {
            candidate -= 1;
            s -= 1;
        }
        let mut base = s;
        let mut offset = base - candidate;
        s = extend_fwd(src, n, s + 4, candidate + 4);
        emit_match(
            out,
            &src[next_emit..base],
            offset,
            s - base,
            &mut last_offset,
        );

        // Keep emitting copies while the bytes immediately after the match
        // continue to match a recent position.
        loop {
            next_emit = s;
            if s >= s_limit {
                break 'outer;
            }
            let x = load64(src, s - 2);
            let m2_hash = hash6(x, TABLE_BITS);
            let curr = (x >> 16) as u32;
            let curr_hash = hash6(x >> 16, TABLE_BITS);
            candidate = table[curr_hash] as usize;
            table[m2_hash] = (s - 2) as u32;
            table[curr_hash] = s as u32;
            if s - candidate > MAX_COPY3_OFFSET || curr != load32(src, candidate) {
                cv = load64(src, s + 1);
                s += 1;
                break;
            }
            offset = s - candidate;
            base = s;
            s = extend_fwd(src, n, s + 4, candidate + 4);
            emit_copy(out, offset, s - base, &mut last_offset);
        }
    }

    // Trailing literals.
    if next_emit < n {
        emit_literals(out, &src[next_emit..n]);
    }
}

/// Two-hash-table "better" matcher (ported from the reference `encodeBlockBetter`
/// approach, emitting MinLZ tokens). A long hash (7 bytes) finds long matches and
/// a short hash (4 bytes) finds short ones — one lookup each, no chain walking —
/// and positions inside each match are indexed cheaply ("index in-between").
fn encode_block_better(out: &mut Vec<u8>, src: &[u8]) {
    let n = src.len();
    const L_BITS: u32 = 17;
    const S_BITS: u32 = 14;
    let mut l_table = vec![0u32; 1usize << L_BITS];
    let mut s_table = vec![0u32; 1usize << S_BITS];

    let s_limit = n - 8; // load64 reads 8 bytes at s
    let mut next_emit = 0usize;
    let mut last_offset = 1usize; // the "repeat" offset
    let mut s = 1usize;
    let mut cv = load64(src, s);
    let mut next_s;

    'outer: loop {
        let mut candidate_l;
        loop {
            next_s = s + ((s - next_emit) >> 7) + 1;
            if next_s > s_limit {
                break 'outer;
            }
            let min_src_pos = s as isize - MAX_COPY3_OFFSET as isize + 1;
            let hash_l = hash7(cv, L_BITS);
            let hash_s = hash4u(cv, S_BITS);
            candidate_l = l_table[hash_l] as usize;
            let candidate_s = s_table[hash_s] as usize;
            l_table[hash_l] = s as u32;
            s_table[hash_s] = s as u32;
            let val_long = load64(src, candidate_l);
            let val_short = load64(src, candidate_s);

            // Long candidate matches 8 bytes — take it.
            if candidate_l as isize > min_src_pos && cv == val_long {
                break;
            }

            // Repeat (last-offset) check on the 4 bytes at s+1.
            const REPEAT_MASK: u64 = 0xffff_ffffu64 << 8;
            if s >= last_offset && cv & REPEAT_MASK == load64(src, s - last_offset) & REPEAT_MASK {
                let mut base = s + 1;
                let mut i = base - last_offset;
                while base > next_emit && i > 0 && src[i - 1] == src[base - 1] {
                    i -= 1;
                    base -= 1;
                }
                emit_literals(out, &src[next_emit..base]);

                let mut se = s + 5;
                let mut ce = s - last_offset + 5;
                while se < n {
                    if n - se < 8 {
                        if src[se] == src[ce] {
                            se += 1;
                            ce += 1;
                            continue;
                        }
                        break;
                    }
                    let diff = load64(src, se) ^ load64(src, ce);
                    if diff != 0 {
                        se += (diff.trailing_zeros() >> 3) as usize;
                        break;
                    }
                    se += 8;
                    ce += 8;
                }
                emit_repeat(out, se - base);
                s = se;
                next_emit = s;
                if s >= s_limit {
                    break 'outer;
                }
                // Index the gap between the match ends.
                let mut index0 = base + 1;
                let mut index1 = s - 2;
                while index0 < index1 {
                    let cv0 = load64(src, index0);
                    let cv1 = load64(src, index1);
                    l_table[hash7(cv0, L_BITS)] = index0 as u32;
                    s_table[hash4u(cv0 >> 8, S_BITS)] = (index0 + 1) as u32;
                    l_table[hash7(cv1, L_BITS)] = index1 as u32;
                    s_table[hash4u(cv1 >> 8, S_BITS)] = (index1 + 1) as u32;
                    index0 += 2;
                    index1 -= 2;
                }
                cv = load64(src, s);
                continue;
            }

            // Long candidate matches 4 bytes.
            if candidate_l as isize >= min_src_pos && cv as u32 == val_long as u32 {
                break;
            }

            // Short candidate matches 4 bytes.
            if candidate_s as isize >= min_src_pos && cv as u32 == val_short as u32 {
                // Prefer a long candidate one byte ahead, if any.
                let h = hash7(cv >> 8, L_BITS);
                candidate_l = l_table[h] as usize;
                l_table[h] = (s + 1) as u32;
                if candidate_l as isize > min_src_pos
                    && (cv >> 8) as u32 == load32(src, candidate_l)
                {
                    s += 1;
                    break;
                }
                candidate_l = candidate_s;
                break;
            }

            cv = load64(src, next_s);
            s = next_s;
        }

        // Extend the match backwards.
        while candidate_l > 0 && s > next_emit && src[candidate_l - 1] == src[s - 1] {
            candidate_l -= 1;
            s -= 1;
        }
        let base = s;
        let offset = base - candidate_l;

        // Extend forwards from the known 4-byte match.
        let mut se = s + 4;
        let mut ce = candidate_l + 4;
        while se < n {
            if n - se < 8 {
                if src[se] == src[ce] {
                    se += 1;
                    ce += 1;
                    continue;
                }
                break;
            }
            let diff = load64(src, se) ^ load64(src, ce);
            if diff != 0 {
                se += (diff.trailing_zeros() >> 3) as usize;
                break;
            }
            se += 8;
            ce += 8;
        }
        let length = se - base;

        // A far (copy3) 4-byte match rarely pays for itself; skip it.
        if offset > MAX_COPY2_OFFSET && length <= 4 && last_offset != offset {
            s = next_s + 1;
            if s >= s_limit {
                break 'outer;
            }
            cv = load64(src, s);
            continue;
        }

        emit_match(out, &src[next_emit..base], offset, length, &mut last_offset);
        s = se;
        next_emit = s;
        if s >= s_limit {
            break 'outer;
        }

        // Index short & long around the match, then sparsely in the middle.
        let mut index0 = base + 1;
        let mut index1 = s - 2;
        let cv0 = load64(src, index0);
        let cv1 = load64(src, index1);
        l_table[hash7(cv0, L_BITS)] = index0 as u32;
        s_table[hash4u(cv0 >> 8, S_BITS)] = (index0 + 1) as u32;
        l_table[hash7(cv1, L_BITS)] = index1 as u32;
        s_table[hash4u(cv1 >> 8, S_BITS)] = (index1 + 1) as u32;
        index0 += 1;
        index1 -= 1;
        cv = load64(src, s);

        let mut index2 = (index0 + index1 + 1) >> 1;
        while index2 < index1 {
            l_table[hash7(load64(src, index0), L_BITS)] = index0 as u32;
            l_table[hash7(load64(src, index2), L_BITS)] = index2 as u32;
            index0 += 2;
            index2 += 2;
        }
    }

    if next_emit < n {
        emit_literals(out, &src[next_emit..n]);
    }
}

/// Compress `src` against a dictionary `prefix`, returning a dictionary block
/// body `[uvarint(src.len())][tokens]` (no indicator byte). Backreferences may
/// reach into the prefix. The decoder must supply the same prefix.
pub(crate) fn compress_body_dict(src: &[u8], prefix: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() / 2 + 16);
    let mut lenbuf = [0u8; 10];
    let n = encode_varint(&mut lenbuf, src.len() as u64);
    out.extend_from_slice(&lenbuf[..n]);

    if src.is_empty() {
        return out;
    }
    if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
        emit_literals(&mut out, src);
        return out;
    }

    // Work over `prefix || src`; emit only the `src` portion.
    let mut combined = Vec::with_capacity(prefix.len() + src.len());
    combined.extend_from_slice(prefix);
    combined.extend_from_slice(src);
    encode_block_prefixed(&mut out, &combined, prefix.len());
    out
}

/// Greedy matcher over `combined`, emitting tokens for `combined[prefix..]`.
/// Positions in the prefix are seeded into the hash table so matches can point
/// into the dictionary.
fn encode_block_prefixed(out: &mut Vec<u8>, combined: &[u8], prefix: usize) {
    let n = combined.len();
    let bits = table_bits(n);
    let mut table = vec![u32::MAX; 1usize << bits];

    // Seed the dictionary positions.
    let seed_end = prefix.saturating_sub(INPUT_MARGIN);
    for p in 0..seed_end {
        table[hash4(load32(combined, p), bits)] = p as u32;
    }

    let s_limit = n - INPUT_MARGIN;
    let mut next_emit = prefix;
    let mut last_offset = 1usize;
    let mut s = prefix.max(1);

    while s <= s_limit {
        if s >= last_offset && load32(combined, s) == load32(combined, s - last_offset) {
            let cand = s - last_offset;
            let (start, len) = extend(combined, s, cand, next_emit);
            emit_literals(out, &combined[next_emit..start]);
            emit_repeat(out, len);
            s = start + len;
            next_emit = s;
            continue;
        }

        let cv = load32(combined, s);
        let h = hash4(cv, bits);
        let cand = table[h];
        table[h] = s as u32;

        let usable = cand != u32::MAX && {
            let c = cand as usize;
            let off = s - c;
            off <= MAX_COPY3_OFFSET && load32(combined, c) == cv
        };
        if !usable {
            s += ((s - next_emit) >> 5) + 1;
            continue;
        }

        let cand = cand as usize;
        let offset = s - cand;
        let (start, len) = extend(combined, s, cand, next_emit);
        emit_match(
            out,
            &combined[next_emit..start],
            offset,
            len,
            &mut last_offset,
        );
        s = start + len;
        next_emit = s;
    }

    if next_emit < n {
        emit_literals(out, &combined[next_emit..n]);
    }
}

/// Forward-only match length from `s` against `cand` (`cand < s`), 8 bytes at a
/// time. Used to rank chain candidates cheaply before committing to one.
#[inline(always)]
fn forward_len(src: &[u8], s: usize, cand: usize) -> usize {
    let max = src.len() - s;
    let mut len = 0usize;
    while len + 8 <= max {
        let a = u64::from_le_bytes(src[s + len..s + len + 8].try_into().unwrap());
        let b = u64::from_le_bytes(src[cand + len..cand + len + 8].try_into().unwrap());
        let diff = a ^ b;
        if diff != 0 {
            return len + (diff.trailing_zeros() / 8) as usize;
        }
        len += 8;
    }
    while len < max && src[cand + len] == src[s + len] {
        len += 1;
    }
    len
}

/// Find the best match for position `s` by walking the hash chain up to `depth`
/// links. Returns `(candidate, forward_length)` of the longest match, or `None`.
/// Prefers longer matches; on a tie prefers the smaller offset (fewer encoded
/// bytes). Candidates that cannot beat the current best (their byte at the
/// best-length boundary differs) are rejected without a full comparison.
fn best_match(
    src: &[u8],
    s: usize,
    head: &[u32],
    prev: &[u32],
    bits: u32,
    depth: u32,
) -> Option<(usize, usize)> {
    let cv = load32(src, s);
    let mut c = head[hash4(cv, bits)];
    let mut best_len = 0usize;
    let mut best_cand = 0usize;
    let mut tries = depth;
    let max = src.len() - s;
    while c != u32::MAX && tries > 0 {
        let cand = c as usize;
        if cand >= s {
            // The just-inserted position(s) at/after `s`: not a backreference.
            c = prev[cand];
            continue;
        }
        let off = s - cand;
        if off > MAX_COPY3_OFFSET {
            break; // chain is ordered newest-first; everything older is farther
        }
        tries -= 1;
        let next = prev[cand];
        // Quick reject: a longer match must at least match at the boundary byte.
        if (best_len == 0 || src[cand + best_len] == src[s + best_len]) && load32(src, cand) == cv {
            let len = forward_len(src, s, cand);
            if len > best_len || (len == best_len && off < s - best_cand) {
                best_len = len;
                best_cand = cand;
                if best_len == max {
                    break; // matched to the end of input
                }
            }
        }
        c = next;
    }
    if best_len >= 4 {
        Some((best_cand, best_len))
    } else {
        None
    }
}

/// Hash-chain match loop with optional one-step lazy matching.
fn encode_block_chain(out: &mut Vec<u8>, src: &[u8], depth: u32, lazy: bool) {
    let n = src.len();
    let bits = table_bits(n);
    let mut head = vec![u32::MAX; 1usize << bits];
    let mut prev = vec![u32::MAX; n];

    let insert = |head: &mut [u32], prev: &mut [u32], p: usize| {
        let h = hash4(load32(src, p), bits);
        prev[p] = head[h];
        head[h] = p as u32;
    };

    let s_limit = n - INPUT_MARGIN;
    let mut next_emit = 0usize;
    let mut last_offset = 1usize;
    let mut s = 1usize;
    // Seed position 0 so it can be matched against.
    insert(&mut head, &mut prev, 0);

    while s <= s_limit {
        // Repeat (last-offset) fast path.
        if s >= last_offset && load32(src, s) == load32(src, s - last_offset) {
            let cand = s - last_offset;
            let (start, len) = extend(src, s, cand, next_emit);
            emit_literals(out, &src[next_emit..start]);
            emit_repeat(out, len);
            for p in s..(start + len).min(s_limit + 1) {
                insert(&mut head, &mut prev, p);
            }
            s = start + len;
            next_emit = s;
            continue;
        }

        insert(&mut head, &mut prev, s);
        let m = best_match(src, s, &head, &prev, bits, depth);
        let (mut cand, len) = match m {
            Some(v) => v,
            None => {
                s += ((s - next_emit) >> 5) + 1;
                continue;
            }
        };

        // Lazy matching: if inserting s+1 yields a strictly longer match, defer
        // and take that one instead (emitting src[s] as a literal). The final
        // length is recomputed by `extend` below, so we only need s and cand.
        if lazy && s < s_limit {
            insert(&mut head, &mut prev, s + 1);
            if let Some((c2, l2)) = best_match(src, s + 1, &head, &prev, bits, depth) {
                if l2 > len {
                    s += 1;
                    cand = c2;
                }
            }
        }

        let offset = s - cand;
        let (start, mlen) = extend(src, s, cand, next_emit);
        emit_match(out, &src[next_emit..start], offset, mlen, &mut last_offset);
        // Insert positions covered by the match so later matches can find them.
        for p in (s + 1)..(start + mlen).min(s_limit + 1) {
            insert(&mut head, &mut prev, p);
        }
        s = start + mlen;
        next_emit = s;
    }

    if next_emit < n {
        emit_literals(out, &src[next_emit..n]);
    }
}

/// Extend a 4-byte match at `(s, cand)` backwards (not past `next_emit`) and
/// forwards. Returns the match start in `src` and its length.
fn extend(src: &[u8], mut s: usize, mut cand: usize, next_emit: usize) -> (usize, usize) {
    let n = src.len();
    while s > next_emit && cand > 0 && src[s - 1] == src[cand - 1] {
        s -= 1;
        cand -= 1;
    }
    // Forward extension, 8 bytes at a time. `cand < s`, so `cand + len + 8` is
    // always `<= s + len + 8 <= n` whenever the `s`-side read is in bounds.
    let max = n - s;
    let mut len = 0usize;
    while len + 8 <= max {
        let a = u64::from_le_bytes(src[s + len..s + len + 8].try_into().unwrap());
        let b = u64::from_le_bytes(src[cand + len..cand + len + 8].try_into().unwrap());
        let diff = a ^ b;
        if diff != 0 {
            return (s, len + (diff.trailing_zeros() / 8) as usize);
        }
        len += 8;
    }
    while len < max && src[cand + len] == src[s + len] {
        len += 1;
    }
    (s, len)
}

/// Emit a run of literals (`lits` may be empty, in which case nothing is done).
fn emit_literals(out: &mut Vec<u8>, lits: &[u8]) {
    let n = lits.len();
    if n == 0 {
        return;
    }
    emit_length_tag(out, 0, n);
    out.extend_from_slice(lits);
}

/// Emit a repeat (copy from the last offset) of `length` bytes.
fn emit_repeat(out: &mut Vec<u8>, length: usize) {
    // Tag bit 2 set selects repeat.
    emit_length_tag(out, 0b100, length);
}

/// Emit a tag-0 byte (literal or repeat) carrying `length`, with `flags` ORed
/// into the low bits. Shared by literals and repeats (identical length codes).
fn emit_length_tag(out: &mut Vec<u8>, flags: u8, length: usize) {
    debug_assert!(length >= 1);
    if length <= 29 {
        out.push(flags | (((length - 1) as u8) << 3));
    } else {
        let m = length - 30;
        if m <= 0xff {
            out.push(flags | (29 << 3));
            out.push(m as u8);
        } else if m <= 0xffff {
            out.push(flags | (30 << 3));
            out.push(m as u8);
            out.push((m >> 8) as u8);
        } else {
            out.push(flags | (31 << 3));
            out.push(m as u8);
            out.push((m >> 8) as u8);
            out.push((m >> 16) as u8);
        }
    }
}

/// Emit a copy of `length` bytes at `offset`, choosing the smallest form and
/// updating `last_offset`.
fn emit_copy(out: &mut Vec<u8>, offset: usize, length: usize, last_offset: &mut usize) {
    debug_assert!(length >= 4);
    debug_assert!((1..=MAX_COPY3_OFFSET).contains(&offset));

    if offset <= MAX_COPY1_OFFSET && length <= MAX_COPY1_LENGTH {
        emit_copy1(out, offset, length);
    } else if offset < MIN_COPY2_OFFSET {
        // Small offset (< Copy2's minimum), long match: Copy1 cannot reach
        // Copy2's offset range, so emit a max-length Copy1 then continue with
        // a repeat for the remainder.
        emit_copy1(out, offset, MAX_COPY1_LENGTH);
        *last_offset = offset;
        emit_repeat(out, length - MAX_COPY1_LENGTH);
        return;
    } else if offset <= MAX_COPY2_OFFSET {
        emit_copy2(out, offset, length);
    } else {
        debug_assert!(offset >= MIN_COPY3_OFFSET);
        emit_copy3(out, offset, length);
    }
    *last_offset = offset;
}

fn emit_copy1(out: &mut Vec<u8>, offset: usize, length: usize) {
    debug_assert!((1..=MAX_COPY1_OFFSET).contains(&offset));
    debug_assert!((4..=MAX_COPY1_LENGTH).contains(&length));
    let stored = (offset - 1) as u16; // 10-bit offset
    let lo2 = (stored & 3) as u8;
    let hi = (stored >> 2) as u8;
    if length <= 18 {
        out.push(1 | (((length - 4) as u8) << 2) | (lo2 << 6));
        out.push(hi);
    } else {
        out.push(1 | (15 << 2) | (lo2 << 6));
        out.push(hi);
        out.push((length - 18) as u8);
    }
}

fn emit_copy2(out: &mut Vec<u8>, offset: usize, length: usize) {
    debug_assert!((MIN_COPY2_OFFSET..=MAX_COPY2_OFFSET).contains(&offset));
    debug_assert!(length >= 4);
    let stored = (offset - MIN_COPY2_OFFSET) as u16;
    let off_lo = stored as u8;
    let off_hi = (stored >> 8) as u8;
    if length <= 64 {
        out.push(2 | (((length - 4) as u8) << 2));
        out.push(off_lo);
        out.push(off_hi);
    } else {
        let m = length - 64;
        let lc = if m <= 0xff {
            61
        } else if m <= 0xffff {
            62
        } else {
            63
        };
        out.push(2 | (lc << 2));
        out.push(off_lo);
        out.push(off_hi);
        push_extra(out, m, (lc - 60) as usize);
    }
}

fn emit_copy3(out: &mut Vec<u8>, offset: usize, length: usize) {
    debug_assert!((MIN_COPY3_OFFSET..=MAX_COPY3_OFFSET).contains(&offset));
    debug_assert!(length >= 4);
    let stored = (offset - MIN_COPY3_OFFSET) as u32; // 21-bit offset
    let (lc, extra) = if length <= 64 {
        ((length - 4) as u32, 0usize)
    } else {
        let m = length - 64;
        if m <= 0xff {
            (61, 1)
        } else if m <= 0xffff {
            (62, 2)
        } else {
            (63, 3)
        }
    };
    // val: tag=3, copy3 bit (1<<2), litLen=0, length code <<5, offset <<11.
    let val: u32 = 0x07 | (lc << 5) | (stored << 11);
    out.extend_from_slice(&val.to_le_bytes());
    if extra > 0 {
        push_extra(out, length - 64, extra);
    }
}

/// Append the low `count` bytes of `m` in little-endian order.
fn push_extra(out: &mut Vec<u8>, m: usize, count: usize) {
    for i in 0..count {
        out.push((m >> (8 * i)) as u8);
    }
}

/// Fused Copy2/Copy3 carry up to this many preceding literals.
const MAX_COPY2_LITS: usize = 4;
const MAX_COPY3_LITS: usize = 3;

/// Emit `lits` followed by a copy of `length` bytes at `offset`, using a fused
/// literal+copy token when the literal run is short enough and the offset is in
/// range (cheaper to decode and one byte smaller). Falls back to separate
/// literal + copy otherwise.
fn emit_match(
    out: &mut Vec<u8>,
    lits: &[u8],
    offset: usize,
    length: usize,
    last_offset: &mut usize,
) {
    if lits.is_empty() {
        emit_copy(out, offset, length, last_offset);
        return;
    }
    if offset <= MAX_COPY2_OFFSET {
        if lits.len() > MAX_COPY2_LITS || offset < MIN_COPY2_OFFSET {
            emit_literals(out, lits);
            emit_copy(out, offset, length, last_offset);
        } else {
            emit_fused_copy2(out, lits, offset, length);
            *last_offset = offset;
        }
    } else if lits.len() > MAX_COPY3_LITS {
        emit_literals(out, lits);
        emit_copy(out, offset, length, last_offset);
    } else {
        emit_fused_copy3(out, lits, offset, length);
        *last_offset = offset;
    }
}

/// Fused Copy2: 1..4 literals + a 4..11-byte copy at a 16-bit offset (longer
/// copies continue with a repeat).
fn emit_fused_copy2(out: &mut Vec<u8>, lits: &[u8], offset: usize, length: usize) {
    debug_assert!((1..=MAX_COPY2_LITS).contains(&lits.len()));
    debug_assert!((MIN_COPY2_OFFSET..=MAX_COPY2_OFFSET).contains(&offset));
    let stored = (offset - MIN_COPY2_OFFSET) as u16;
    let lit_bits = ((lits.len() - 1) as u8) << 3;
    let len_raw = length - 4;
    if len_raw > 7 {
        out.push(0x03 | (7 << 5) | lit_bits);
        out.push(stored as u8);
        out.push((stored >> 8) as u8);
        out.extend_from_slice(lits);
        emit_repeat(out, len_raw - 7);
    } else {
        out.push(0x03 | ((len_raw as u8) << 5) | lit_bits);
        out.push(stored as u8);
        out.push((stored >> 8) as u8);
        out.extend_from_slice(lits);
    }
}

/// Fused Copy3: 1..3 literals + a copy at a 21-bit offset.
fn emit_fused_copy3(out: &mut Vec<u8>, lits: &[u8], offset: usize, length: usize) {
    debug_assert!((1..=MAX_COPY3_LITS).contains(&lits.len()));
    debug_assert!((MIN_COPY3_OFFSET..=MAX_COPY3_OFFSET).contains(&offset));
    let stored = (offset - MIN_COPY3_OFFSET) as u32;
    let (lc, extra) = if length <= 64 {
        ((length - 4) as u32, 0usize)
    } else {
        let m = length - 64;
        if m <= 0xff {
            (61, 1)
        } else if m <= 0xffff {
            (62, 2)
        } else {
            (63, 3)
        }
    };
    // val: tag 3, copy3 bit, litLen, length code, offset.
    let val: u32 = 0x07 | ((lits.len() as u32) << 3) | (lc << 5) | (stored << 11);
    out.extend_from_slice(&val.to_le_bytes());
    if extra > 0 {
        push_extra(out, length - 64, extra);
    }
    out.extend_from_slice(lits);
}
