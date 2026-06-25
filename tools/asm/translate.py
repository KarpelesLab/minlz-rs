#!/usr/bin/env python3
# Translate Plan9 (Go) amd64 asm for ·encodeBlockAsm into AT&T global_asm! body.
# Rules:
#  - operand order is preserved (Plan9 src,dst == AT&T src,dst) EXCEPT CMP* which swaps.
#  - register width derived from mnemonic suffix (Q/L/W/B); memory base/index always 64-bit.
#  - MOVOU/MOVOA -> movdqu ; PXOR -> pxor ; X regs -> %xmmN.
#  - FP args mapped to my stack slots; SP locals kept as N(%rsp); RET -> jmp epilogue.
import re, sys

# argv: <plan9.s> <global_symbol> <epilogue_label>
SRC = sys.argv[1]
SYM = sys.argv[2]
EPI = sys.argv[3]
lines = open(SRC).read().splitlines()

# 64/32/16/8-bit register name tables
R64 = {'AX':'rax','BX':'rbx','CX':'rcx','DX':'rdx','SI':'rsi','DI':'rdi','BP':'rbp','SP':'rsp',
       'R8':'r8','R9':'r9','R10':'r10','R11':'r11','R12':'r12','R13':'r13','R14':'r14','R15':'r15'}
def reg(name, w):
    b = R64[name]
    if w == 8:
        m = {'rax':'al','rbx':'bl','rcx':'cl','rdx':'dl','rsi':'sil','rdi':'dil','rbp':'bpl','rsp':'spl'}
        if b in m: return m[b]
        return b+'b'            # r8b..r15b
    if w == 16:
        m = {'rax':'ax','rbx':'bx','rcx':'cx','rdx':'dx','rsi':'si','rdi':'di','rbp':'bp','rsp':'sp'}
        if b in m: return m[b]
        return b+'w'
    if w == 32:
        m = {'rax':'eax','rbx':'ebx','rcx':'ecx','rdx':'edx','rsi':'esi','rdi':'edi','rbp':'ebp','rsp':'esp'}
        if b in m: return m[b]
        return b+'d'
    return b                    # 64

XRE = re.compile(r'^X([0-9]+)$')

# Explicit byte-register literals that may appear in the Plan9 source.
BYTEREG = {'AL':'al','BL':'bl','CL':'cl','DL':'dl','SIL':'sil','DIL':'dil','BPL':'bpl','SPL':'spl',
           'R8B':'r8b','R9B':'r9b','R10B':'r10b','R11B':'r11b','R12B':'r12b','R13B':'r13b',
           'R14B':'r14b','R15B':'r15b'}

# FP arg -> my stack slot ; ret -> slot
FP_MAP = {
    'dst_base+0(FP)': '32(%rsp)',
    'src_base+24(FP)': '40(%rsp)',
    'src_len+32(FP)': '48(%rsp)',
    'tmp+48(FP)': '56(%rsp)',
    'ret+56(FP)': '64(%rsp)',
}

MEM = re.compile(r'^(-?\w*)\(([A-Z0-9]+)\)(?:\(([A-Z0-9]+)\*(\d+)\))?$')

def operand(tok, w):
    tok = tok.strip()
    if tok.startswith('$'):
        return tok                                  # immediate
    if tok in FP_MAP:
        return FP_MAP[tok]
    m = XRE.match(tok)
    if m:
        return '%xmm'+m.group(1)
    if tok in BYTEREG:                              # explicit byte register
        return '%'+BYTEREG[tok]
    if tok in R64:
        return '%'+reg(tok, w)
    # SP local: N(SP)
    msp = re.match(r'^(-?\d+)\(SP\)$', tok)
    if msp:
        return msp.group(1)+'(%rsp)'
    # general memory DISP(BASE)(INDEX*SCALE)
    mm = MEM.match(tok)
    if mm:
        disp, base, idx, scale = mm.groups()
        base_r = '%'+R64[base]
        if idx:
            return f'{disp}({base_r},%{R64[idx]},{scale})'
        return f'{disp}({base_r})'
    raise SystemExit(f'UNPARSED OPERAND: {tok!r}')

def width_of(mn):
    if mn.endswith('Q'): return 64
    if mn.endswith('L'): return 32
    if mn.endswith('W'): return 16
    if mn.endswith('B'): return 8
    return 64

# mnemonic translation (mnemonic only); operands handled separately
JCC = {'JMP':'jmp','JB':'jb','JBE':'jbe','JNE':'jne','JNZ':'jne','JAE':'jae','JZ':'je',
       'JA':'ja','JE':'je','JEQ':'je','JNA':'jbe','JLE':'jle','JL':'jl'}

def split_ops(s):
    out, depth, cur = [], 0, ''
    for ch in s:
        if ch == '(' : depth += 1
        if ch == ')' : depth -= 1
        if ch == ',' and depth == 0:
            out.append(cur); cur=''
        else:
            cur += ch
    if cur.strip(): out.append(cur)
    return [o.strip() for o in out]

# Pre-pass: resolve #ifdef GOAMD64_v3 / #else / #endif by taking the v3 branch
# (TZCNT instead of BSF — TZCNT has no false dependency on its destination, so it
# does not stall the tight match-length loop). Requires BMI1; the caller gates on
# runtime detection and falls back to the scalar matcher when BMI1 is absent.
resolved = []
state = None   # None=normal, 'keep'=in v3 branch (keep), 'skip'=in else branch
for raw in lines:
    t = raw.strip()
    if t.startswith('#ifdef'):
        state = 'keep'; continue
    if t.startswith('#else'):
        state = 'skip'; continue
    if t.startswith('#endif'):
        state = None; continue
    if state == 'skip':
        continue
    resolved.append(raw)
lines = resolved

out = []
for raw in lines:
    line = raw.split('//')[0].rstrip()
    if not line.strip():
        continue
    s = line.strip()
    if s.startswith('TEXT'):
        continue
    if re.match(r'^[A-Za-z_]\w*:$', s):          # label def -> prefix per-function
        out.append(f'{SYM}_{s}')
        continue
    parts = s.split(None, 1)
    mn = parts[0]
    ops = split_ops(parts[1]) if len(parts) > 1 else []

    if mn == 'RET':
        out.append('jmp '+EPI)
        continue
    if mn in JCC:
        out.append(f'{JCC[mn]} {SYM}_{ops[0]}')   # target -> same per-function prefix
        continue
    if mn == 'PXOR':
        out.append(f'pxor {operand(ops[0],128)}, {operand(ops[1],128)}')
        continue
    if mn in ('MOVOU','MOVOA'):
        out.append(f'movdqu {operand(ops[0],128)}, {operand(ops[1],128)}')
        continue
    if mn == 'CMOVLGE':
        out.append(f'cmovge {operand(ops[0],32)}, {operand(ops[1],32)}')
        continue
    if mn == 'CMOVLLT':
        out.append(f'cmovl {operand(ops[0],32)}, {operand(ops[1],32)}')
        continue
    if mn == 'BSFQ':
        # Replaced with TZCNT (requires BMI1, which the caller checks): identical
        # result for nonzero input — and every use here is guarded by a prior JNZ,
        # so the operand is always nonzero — but without BSF's false dependency on
        # the destination, which otherwise stalls the tight match-length loop. Go's
        # v3 build leaves these as BSF, so this is strictly fewer stalls.
        out.append(f'tzcntq {operand(ops[0],64)}, {operand(ops[1],64)}')
        continue
    if mn == 'MOVLQZX':
        # load 32-bit -> zero-extend to 64: writing a 32-bit dest reg auto-zeroes
        # the upper half, so a plain `movl` with 32-bit operands is exact.
        out.append(f'movl {operand(ops[0],32)}, {operand(ops[1],32)}')
        continue

    w = width_of(mn)
    base = {'MOV':'mov','LEA':'lea','ADD':'add','SUB':'sub','CMP':'cmp','SHR':'shr','SHL':'shl',
            'SAR':'sar','XOR':'xor','AND':'and','OR':'or','TEST':'test','INC':'inc','DEC':'dec',
            'IMUL':'imul','TZCNT':'tzcnt','BSF':'bsf'}
    stem = mn[:-1] if mn[-1] in 'QLWB' else mn
    if stem not in base:
        raise SystemExit(f'UNKNOWN MNEMONIC: {mn} ({s!r})')
    suf = {64:'q',32:'l',16:'w',8:'b'}[w]
    att = base[stem]+suf
    rops = [operand(o, w) for o in ops]
    if stem == 'CMP' and len(rops) == 2:
        rops = [rops[1], rops[0]]                  # swap for AT&T cmp
    out.append(att + ' ' + ', '.join(rops))

body = '\n'.join('    '+l if not l.endswith(':') else l for l in out)

prologue = f""".p2align 4
.globl {SYM}
.hidden {SYM}
{SYM}:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)"""

epilogue = f"""{EPI}:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret"""

print(prologue)
print(body)
print(epilogue)
