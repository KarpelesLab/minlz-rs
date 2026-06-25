// MinLZ Fastest/Balanced block encoders — ported from minio/minlz (Apache-2.0).
// Auto-translated Plan9->AT&T, GOAMD64_v3 path: SSE2 + CMOV + BMI1 (TZCNT).
// The caller gates on runtime BMI1 detection (scalar fallback otherwise).
// SysV ABI: rdi=dst rsi=src rdx=src_len rcx=tmp -> rax=bytes written (0=incompressible)
.text
.p2align 4
.globl minlz_encode_block_asm_512k
.hidden minlz_encode_block_asm_512k
minlz_encode_block_asm_512k:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00000200, %rdx
    movq %rax, %rbx
    pxor %xmm0, %xmm0
minlz_encode_block_asm_512k_zero_loop_encodeBlockAsm512K:
    movdqu %xmm0, (%rbx)
    movdqu %xmm0, 16(%rbx)
    movdqu %xmm0, 32(%rbx)
    movdqu %xmm0, 48(%rbx)
    movdqu %xmm0, 64(%rbx)
    movdqu %xmm0, 80(%rbx)
    movdqu %xmm0, 96(%rbx)
    movdqu %xmm0, 112(%rbx)
    addq $0x80, %rbx
    decq %rdx
    jne minlz_encode_block_asm_512k_zero_loop_encodeBlockAsm512K
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rdx
    leaq -17(%rdx), %rbx
    leaq -17(%rdx), %rsi
    movl %esi, 8(%rsp)
    shrq $0x05, %rdx
    subl %edx, %ebx
    leaq (%rcx,%rbx,1), %rbx
    movq %rbx, (%rsp)
    movl $0x00000001, %edx
    movl %edx, 16(%rsp)
    movq 40(%rsp), %rbx
minlz_encode_block_asm_512k_search_loop_encodeBlockAsm512K:
    movl %edx, %esi
    subl 12(%rsp), %esi
    shrl $0x06, %esi
    leal 4(%rdx,%rsi,1), %esi
    cmpl 8(%rsp), %esi
    jae minlz_encode_block_asm_512k_emit_remainder_encodeBlockAsm512K
    movq (%rbx,%rdx,1), %rdi
    movl %esi, 20(%rsp)
    movq $0x0000cf1bbcdcbf9b, %r9
    movq %rdi, %r10
    movq %rdi, %r11
    shrq $0x08, %r11
    shlq $0x10, %r10
    imulq %r9, %r10
    shrq $0x33, %r10
    shlq $0x10, %r11
    imulq %r9, %r11
    shrq $0x33, %r11
    movl (%rax,%r10,4), %esi
    movl (%rax,%r11,4), %r8d
    movl %edx, (%rax,%r10,4)
    movl %edx, (%rax,%r11,4)
    movq %rdi, %r10
    shrq $0x10, %r10
    shlq $0x10, %r10
    imulq %r9, %r10
    shrq $0x33, %r10
    movl %edx, %r9d
    subl 16(%rsp), %r9d
    movl 1(%rbx,%r9,1), %r11d
    movq %rdi, %r9
    shrq $0x08, %r9
    cmpl %r11d, %r9d
    jne minlz_encode_block_asm_512k_no_repeat_found_encodeBlockAsm512K
    leal 1(%rdx), %edi
    movl 12(%rsp), %esi
    movl %edi, %r8d
    subl 16(%rsp), %r8d
    je minlz_encode_block_asm_512k_repeat_extend_back_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_extend_back_loop_encodeBlockAsm512K:
    cmpl %esi, %edi
    jbe minlz_encode_block_asm_512k_repeat_extend_back_end_encodeBlockAsm512K
    movb -1(%rbx,%r8,1), %r9b
    movb -1(%rbx,%rdi,1), %r10b
    cmpb %r10b, %r9b
    jne minlz_encode_block_asm_512k_repeat_extend_back_end_encodeBlockAsm512K
    leal -1(%rdi), %edi
    decl %r8d
    jne minlz_encode_block_asm_512k_repeat_extend_back_loop_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_extend_back_end_encodeBlockAsm512K:
    movl %edi, %esi
    movl 12(%rsp), %r8d
    subl %r8d, %esi
    leaq 4(%rcx,%rsi,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_512k_dst_size_check_ok_1
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_dst_size_check_ok_1:
    leaq (%rbx,%r8,1), %r8
    leal -1(%rsi), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_512k_one_byte_repeat_emit_lits_encodeBlockAsm512K
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_512k_two_bytes_repeat_emit_lits_encodeBlockAsm512K
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_512k_three_bytes_repeat_emit_lits_encodeBlockAsm512K
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_512k_memmove_long_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_three_bytes_repeat_emit_lits_encodeBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_512k_memmove_long_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_two_bytes_repeat_emit_lits_encodeBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_512k_memmove_midrepeat_emit_lits_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_memmove_long_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_one_byte_repeat_emit_lits_encodeBlockAsm512K:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x10, %rsi
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_512k_memmove_end_copy_repeat_emit_lits_encodeBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_512k_repeat_emit_lits_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_midrepeat_emit_lits_encodeBlockAsm512K:
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_512k_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_512k_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_512k_repeat_emit_lits_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_long_repeat_emit_lits_encodeBlockAsm512K:
    leaq (%rcx,%rsi,1), %r9
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movq %rsi, %r11
    shrq $0x05, %r11
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r12
    subq %r10, %r12
    decq %r11
    ja minlz_encode_block_asm_512k_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%r8,%r12,1), %r10
    leaq -32(%rcx,%r12,1), %r13
minlz_encode_block_asm_512k_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm512Klarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r13)
    movdqu %xmm5, 16(%r13)
    addq $0x20, %r13
    addq $0x20, %r10
    addq $0x20, %r12
    decq %r11
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm512Klarge_big_loop_back
minlz_encode_block_asm_512k_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%r8,%r12,1), %xmm4
    movdqu -16(%r8,%r12,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r12,1)
    movdqu %xmm5, -16(%rcx,%r12,1)
    addq $0x20, %r12
    cmpq %r12, %rsi
    jae minlz_encode_block_asm_512k_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
    movq %r9, %rcx
minlz_encode_block_asm_512k_repeat_emit_lits_end_encodeBlockAsm512K:
    addl $0x05, %edx
    movl %edx, %esi
    subl 16(%rsp), %esi
    movq 48(%rsp), %r8
    subl %edx, %r8d
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_512k_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_loopback_16_repeat_extend_encodeBlockAsm512K:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_repeat_extend_encodeBlockAsm512K
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_512k_matchlen_bsf_16repeat_extend_encodeBlockAsm512K
    leal -16(%r8), %r8d
    leal 16(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm512K:
    cmpl $0x10, %r8d
    jae minlz_encode_block_asm_512k_matchlen_loopback_16_repeat_extend_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_matchlen_match8_repeat_extend_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_16repeat_extend_encodeBlockAsm512K:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match8_repeat_extend_encodeBlockAsm512K:
    cmpl $0x08, %r8d
    jb minlz_encode_block_asm_512k_matchlen_match4_repeat_extend_encodeBlockAsm512K
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_repeat_extend_encodeBlockAsm512K
    leal -8(%r8), %r8d
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_512k_matchlen_match4_repeat_extend_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_8_repeat_extend_encodeBlockAsm512K:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match4_repeat_extend_encodeBlockAsm512K:
    cmpl $0x04, %r8d
    jb minlz_encode_block_asm_512k_matchlen_match2_repeat_extend_encodeBlockAsm512K
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match2_repeat_extend_encodeBlockAsm512K
    leal -4(%r8), %r8d
    leal 4(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_match2_repeat_extend_encodeBlockAsm512K:
    cmpl $0x01, %r8d
    je minlz_encode_block_asm_512k_matchlen_match1_repeat_extend_encodeBlockAsm512K
    jb minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match1_repeat_extend_encodeBlockAsm512K
    leal 2(%r11), %r11d
    subl $0x02, %r8d
    je minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match1_repeat_extend_encodeBlockAsm512K:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K
    leal 1(%r11), %r11d
minlz_encode_block_asm_512k_repeat_extend_forward_end_encodeBlockAsm512K:
    addl %r11d, %edx
    movl %edx, %esi
    subl %edi, %esi
    movl 16(%rsp), %edi
    leal -1(%rsi), %edi
    cmpl $0x1d, %esi
    jbe minlz_encode_block_asm_512k_repeat_one_match_repeat_encodeBlockAsm512K
    leal -30(%rsi), %edi
    cmpl $0x0000011e, %esi
    jb minlz_encode_block_asm_512k_repeat_two_match_repeat_encodeBlockAsm512K
    cmpl $0x0001001e, %esi
    jb minlz_encode_block_asm_512k_repeat_three_match_repeat_encodeBlockAsm512K
    movb $0xfc, (%rcx)
    movl %edi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_512k_repeat_end_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_three_match_repeat_encodeBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %di, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_512k_repeat_end_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_two_match_repeat_encodeBlockAsm512K:
    movb $0xec, (%rcx)
    movb %dil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_512k_repeat_end_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_one_match_repeat_encodeBlockAsm512K:
    xorl %edi, %edi
    leal -4(%rdi,%rsi,8), %edi
    movb %dil, (%rcx)
    addq $0x01, %rcx
minlz_encode_block_asm_512k_repeat_end_emit_encodeBlockAsm512K:
    movl %edx, 12(%rsp)
    jmp minlz_encode_block_asm_512k_search_loop_encodeBlockAsm512K
minlz_encode_block_asm_512k_no_repeat_found_encodeBlockAsm512K:
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_512k_candidate_match_encodeBlockAsm512K
    shrq $0x08, %rdi
    movl (%rax,%r10,4), %esi
    leal 2(%rdx), %r9d
    cmpl %edi, (%rbx,%r8,1)
    je minlz_encode_block_asm_512k_candidate2_match_encodeBlockAsm512K
    movl %r9d, (%rax,%r10,4)
    shrq $0x08, %rdi
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_512k_candidate3_match_encodeBlockAsm512K
    movl 20(%rsp), %edx
    jmp minlz_encode_block_asm_512k_search_loop_encodeBlockAsm512K
minlz_encode_block_asm_512k_candidate3_match_encodeBlockAsm512K:
    addl $0x02, %edx
    jmp minlz_encode_block_asm_512k_candidate_match_encodeBlockAsm512K
minlz_encode_block_asm_512k_candidate2_match_encodeBlockAsm512K:
    movl %r9d, (%rax,%r10,4)
    incl %edx
    movl %r8d, %esi
minlz_encode_block_asm_512k_candidate_match_encodeBlockAsm512K:
    movl 12(%rsp), %edi
    testl %esi, %esi
    je minlz_encode_block_asm_512k_match_extend_back_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_match_extend_back_loop_encodeBlockAsm512K:
    cmpl %edi, %edx
    jbe minlz_encode_block_asm_512k_match_extend_back_end_encodeBlockAsm512K
    movb -1(%rbx,%rsi,1), %r8b
    movb -1(%rbx,%rdx,1), %r9b
    cmpb %r9b, %r8b
    jne minlz_encode_block_asm_512k_match_extend_back_end_encodeBlockAsm512K
    leal -1(%rdx), %edx
    decl %esi
    je minlz_encode_block_asm_512k_match_extend_back_end_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_match_extend_back_loop_encodeBlockAsm512K
minlz_encode_block_asm_512k_match_extend_back_end_encodeBlockAsm512K:
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_512k_dst_size_check_ok_2
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_dst_size_check_ok_2:
    movl %edx, %r8d
    movl %edx, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    addl $0x04, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_512k_matchlen_loop_16_entry_match_nolit_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_loopback_16_match_nolit_encodeBlockAsm512K:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit_encodeBlockAsm512K
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_512k_matchlen_bsf_16match_nolit_encodeBlockAsm512K
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_loop_16_entry_match_nolit_encodeBlockAsm512K:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_512k_matchlen_loopback_16_match_nolit_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_matchlen_match8_match_nolit_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_16match_nolit_encodeBlockAsm512K:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match8_match_nolit_encodeBlockAsm512K:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_512k_matchlen_match4_match_nolit_encodeBlockAsm512K
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit_encodeBlockAsm512K
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_512k_matchlen_match4_match_nolit_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit_encodeBlockAsm512K:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match4_match_nolit_encodeBlockAsm512K:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_512k_matchlen_match2_match_nolit_encodeBlockAsm512K
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match2_match_nolit_encodeBlockAsm512K
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_match2_match_nolit_encodeBlockAsm512K:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_512k_matchlen_match1_match_nolit_encodeBlockAsm512K
    jb minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match1_match_nolit_encodeBlockAsm512K
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match1_match_nolit_encodeBlockAsm512K:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K
    leal 1(%r11), %r11d
minlz_encode_block_asm_512k_match_nolit_end_encodeBlockAsm512K:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl 16(%rsp), %esi
    movl 12(%rsp), %edi
    movl %edx, 12(%rsp)
    subl %edi, %r8d
    je minlz_encode_block_asm_512k_match_nolits_copy_encodeBlockAsm512K
    leaq (%rbx,%rdi,1), %rdi
    cmpl $0x03, %r8d
    ja minlz_encode_block_asm_512k_match_emit_lits_copy_encodeBlockAsm512K
    cmpl $0x40, %esi
    jb minlz_encode_block_asm_512k_match_emit_lits_copy_encodeBlockAsm512K
    movl (%rdi), %edi
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_512k_match_emit_copy2lits_encodeBlockAsm512K
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    leal 7(%rsi,%r8,8), %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_512k_emit_copy3_0_match_emit_lits_encodeBlockAsm512K
    leal -60(%r11), %r9d
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy3_1_match_emit_lits_encodeBlockAsm512K
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy3_2_match_emit_lits_encodeBlockAsm512K
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_512k_match_emit_copy_litsencodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_2_match_emit_lits_encodeBlockAsm512K:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_512k_match_emit_copy_litsencodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_1_match_emit_lits_encodeBlockAsm512K:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_512k_match_emit_copy_litsencodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_0_match_emit_lits_encodeBlockAsm512K:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
minlz_encode_block_asm_512k_match_emit_copy_litsencodeBlockAsm512K:
    movl %edi, (%rcx)
    addq %r8, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_match_emit_copy2lits_encodeBlockAsm512K:
    xorq %r9, %r9
    subl $0x40, %esi
    leal -11(%r11), %r10d
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x07, %r11d
    cmovge %r10d, %r9d
    movq $0x00000007, %rsi
    cmovl %r11d, %esi
    leal -1(%r8,%rsi,4), %esi
    movl $0x00000003, %r10d
    leal (%r10,%rsi,8), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
    movl %edi, (%rcx)
    addq %r8, %rcx
    testl %r9d, %r9d
    je minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
    leal -1(%r9), %esi
    cmpl $0x1d, %r9d
    jbe minlz_encode_block_asm_512k_repeat_one_match_emit_repeat_copy2_encodeBlockAsm512K
    leal -30(%r9), %esi
    cmpl $0x0000011e, %r9d
    jb minlz_encode_block_asm_512k_repeat_two_match_emit_repeat_copy2_encodeBlockAsm512K
    cmpl $0x0001001e, %r9d
    jb minlz_encode_block_asm_512k_repeat_three_match_emit_repeat_copy2_encodeBlockAsm512K
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_three_match_emit_repeat_copy2_encodeBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_two_match_emit_repeat_copy2_encodeBlockAsm512K:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_one_match_emit_repeat_copy2_encodeBlockAsm512K:
    xorl %esi, %esi
    leal -4(%rsi,%r9,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_match_emit_lits_copy_encodeBlockAsm512K:
    leaq 4(%rcx,%r8,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_512k_dst_size_check_ok_3
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_dst_size_check_ok_3:
    leal -1(%r8), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_512k_one_byte_match_emit_encodeBlockAsm512K
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_512k_two_bytes_match_emit_encodeBlockAsm512K
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_512k_three_bytes_match_emit_encodeBlockAsm512K
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_512k_memmove_long_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_three_bytes_match_emit_encodeBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_512k_memmove_long_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_two_bytes_match_emit_encodeBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_512k_memmove_midmatch_emit_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_memmove_long_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_one_byte_match_emit_encodeBlockAsm512K:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r8,1), %r9
    cmpq $0x10, %r8
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_8through16:
    movdqu (%rdi), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_match_emit_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_512k_memmove_end_copy_match_emit_encodeBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_512k_match_nolits_copy_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_midmatch_emit_encodeBlockAsm512K:
    leaq (%rcx,%r8,1), %r9
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_mid_match_emit_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_mid_match_emit_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_mid_match_emit_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_512k_memmove_mid_end_copy_match_emit_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_mid_match_emit_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_512k_memmove_mid_end_copy_match_emit_encodeBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_512k_match_nolits_copy_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_long_match_emit_encodeBlockAsm512K:
    leaq (%rcx,%r8,1), %r9
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movq %r8, %r12
    shrq $0x05, %r12
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r13
    subq %r10, %r13
    decq %r12
    ja minlz_encode_block_asm_512k_emit_lit_memmove_long_match_emit_encodeBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%rdi,%r13,1), %r10
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_block_asm_512k_emit_lit_memmove_long_match_emit_encodeBlockAsm512Klarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r10
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_long_match_emit_encodeBlockAsm512Klarge_big_loop_back
minlz_encode_block_asm_512k_emit_lit_memmove_long_match_emit_encodeBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%rdi,%r13,1), %xmm4
    movdqu -16(%rdi,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %r8
    jae minlz_encode_block_asm_512k_emit_lit_memmove_long_match_emit_encodeBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
    movq %r9, %rcx
minlz_encode_block_asm_512k_match_nolits_copy_encodeBlockAsm512K:
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_512k_two_byte_offset_match_nolit_encodeBlockAsm512K
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    addl $0x07, %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_512k_emit_copy3_0_match_nolit_encodeBlockAsm512K_emit3
    leal -60(%r11), %edi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy3_1_match_nolit_encodeBlockAsm512K_emit3
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy3_2_match_nolit_encodeBlockAsm512K_emit3
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %edi, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_2_match_nolit_encodeBlockAsm512K_emit3:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %di, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_1_match_nolit_encodeBlockAsm512K_emit3:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %dil, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy3_0_match_nolit_encodeBlockAsm512K_emit3:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_two_byte_offset_match_nolit_encodeBlockAsm512K:
    cmpl $0x00000400, %esi
    ja minlz_encode_block_asm_512k_two_byte_match_nolit_encodeBlockAsm512K
    cmpl $0x00000013, %r11d
    jae minlz_encode_block_asm_512k_emit_one_longer_match_nolit_encodeBlockAsm512K
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal -15(%rsi,%r11,4), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_one_longer_match_nolit_encodeBlockAsm512K:
    cmpl $0x00000112, %r11d
    jae minlz_encode_block_asm_512k_emit_copy1_repeat_match_nolit_encodeBlockAsm512K
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 61(%rsi), %esi
    movw %si, (%rcx)
    leal -18(%r11), %esi
    movb %sil, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy1_repeat_match_nolit_encodeBlockAsm512K:
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 57(%rsi), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r11d
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_block_asm_512k_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_block_asm_512k_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K
    cmpl $0x0001001e, %r11d
    jb minlz_encode_block_asm_512k_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm512K:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_two_byte_match_nolit_encodeBlockAsm512K:
    leal -64(%rsi), %esi
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_512k_emit_copy2_0_match_nolit_encodeBlockAsm512K_emit2
    leal -60(%r11), %esi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy2_1_match_nolit_encodeBlockAsm512K_emit2
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_512k_emit_copy2_2_match_nolit_encodeBlockAsm512K_emit2
    movb $0xfe, (%rcx)
    movl %esi, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy2_2_match_nolit_encodeBlockAsm512K_emit2:
    movb $0xfa, (%rcx)
    movw %si, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy2_1_match_nolit_encodeBlockAsm512K_emit2:
    movb $0xf6, (%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_copy2_0_match_nolit_encodeBlockAsm512K_emit2:
    movl $0x00000002, %esi
    leal (%rsi,%r11,4), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
minlz_encode_block_asm_512k_match_nolit_emitcopy_end_encodeBlockAsm512K:
    cmpl 8(%rsp), %edx
    jae minlz_encode_block_asm_512k_emit_remainder_encodeBlockAsm512K
    movq -2(%rbx,%rdx,1), %rdi
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_512k_match_nolit_dst_ok_encodeBlockAsm512K
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_match_nolit_dst_ok_encodeBlockAsm512K:
    movq $0x0000cf1bbcdcbf9b, %rsi
    movq %rdi, %r8
    shrq $0x10, %rdi
    movq %rdi, %r9
    shlq $0x10, %r8
    imulq %rsi, %r8
    shrq $0x33, %r8
    shlq $0x10, %r9
    imulq %rsi, %r9
    shrq $0x33, %r9
    leal -2(%rdx), %r10d
    movl (%rax,%r9,4), %esi
    movl %r10d, (%rax,%r8,4)
    movl %edx, (%rax,%r9,4)
    movl %edx, %r8d
    incl %edx
    cmpl %edi, (%rbx,%rsi,1)
    jne minlz_encode_block_asm_512k_search_loop_encodeBlockAsm512K
    movl %r8d, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_512k_dst_size_check_ok_4
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_dst_size_check_ok_4:
    addl $0x03, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r8
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_512k_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_loopback_16_match_nolit2_encodeBlockAsm512K:
    movq (%r8,%r11,1), %r9
    movq 8(%r8,%r11,1), %r10
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit2_encodeBlockAsm512K
    xorq 8(%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_512k_matchlen_bsf_16match_nolit2_encodeBlockAsm512K
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm512K:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_512k_matchlen_loopback_16_match_nolit2_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_matchlen_match8_match_nolit2_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_16match_nolit2_encodeBlockAsm512K:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal 8(%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match8_match_nolit2_encodeBlockAsm512K:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_512k_matchlen_match4_match_nolit2_encodeBlockAsm512K
    movq (%r8,%r11,1), %r9
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit2_encodeBlockAsm512K
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_512k_matchlen_match4_match_nolit2_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_bsf_8_match_nolit2_encodeBlockAsm512K:
    tzcntq %r9, %r9
    sarq $0x03, %r9
    leal (%r11,%r9,1), %r11d
    jmp minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match4_match_nolit2_encodeBlockAsm512K:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_512k_matchlen_match2_match_nolit2_encodeBlockAsm512K
    movl (%r8,%r11,1), %r9d
    cmpl %r9d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match2_match_nolit2_encodeBlockAsm512K
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_512k_matchlen_match2_match_nolit2_encodeBlockAsm512K:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_512k_matchlen_match1_match_nolit2_encodeBlockAsm512K
    jb minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K
    movw (%r8,%r11,1), %r9w
    cmpw %r9w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_matchlen_match1_match_nolit2_encodeBlockAsm512K
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_matchlen_match1_match_nolit2_encodeBlockAsm512K:
    movb (%r8,%r11,1), %r9b
    cmpb %r9b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K
    leal 1(%r11), %r11d
minlz_encode_block_asm_512k_match_nolit2_end_encodeBlockAsm512K:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl %edx, 12(%rsp)
    movl 16(%rsp), %esi
    jmp minlz_encode_block_asm_512k_match_nolits_copy_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_remainder_encodeBlockAsm512K:
    movq 48(%rsp), %rax
    movl 12(%rsp), %edx
    subl %edx, %eax
    je minlz_encode_block_asm_512k_emit_remainder_end_encodeBlockAsm512K
    leaq (%rbx,%rdx,1), %rdx
    leaq 4(%rcx,%rax,1), %rbx
    cmpq (%rsp), %rbx
    jb minlz_encode_block_asm_512k_dst_size_check_ok_5
    movq $0x00000000, 64(%rsp)
    jmp Lepi_512k
minlz_encode_block_asm_512k_dst_size_check_ok_5:
    leal -1(%rax), %ebx
    cmpl $0x1d, %ebx
    jb minlz_encode_block_asm_512k_one_byte_emit_remainder_encodeBlockAsm512K
    subl $0x1d, %ebx
    cmpl $0x00000100, %ebx
    jb minlz_encode_block_asm_512k_two_bytes_emit_remainder_encodeBlockAsm512K
    cmpl $0x00010000, %ebx
    jb minlz_encode_block_asm_512k_three_bytes_emit_remainder_encodeBlockAsm512K
    movl %ebx, %esi
    shrl $0x10, %esi
    movb $0xf8, (%rcx)
    movw %bx, 1(%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_512k_memmove_long_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_three_bytes_emit_remainder_encodeBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %bx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_512k_memmove_long_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_two_bytes_emit_remainder_encodeBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %bl, 1(%rcx)
    addl $0x1d, %ebx
    addq $0x02, %rcx
    cmpl $0x40, %ebx
    jb minlz_encode_block_asm_512k_memmove_midemit_remainder_encodeBlockAsm512K
    jmp minlz_encode_block_asm_512k_memmove_long_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_one_byte_emit_remainder_encodeBlockAsm512K:
    shlb $0x03, %bl
    movb %bl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x03, %rax
    jb minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_1or2
    je minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_3
    cmpq $0x08, %rax
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_4through8
    cmpq $0x10, %rax
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_1or2:
    movb (%rdx), %sil
    movb -1(%rdx,%rax,1), %dl
    movb %sil, (%rcx)
    movb %dl, -1(%rcx,%rax,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_3:
    movw (%rdx), %si
    movb 2(%rdx), %dl
    movw %si, (%rcx)
    movb %dl, 2(%rcx)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_4through8:
    movl (%rdx), %esi
    movl -4(%rdx,%rax,1), %edx
    movl %esi, (%rcx)
    movl %edx, -4(%rcx,%rax,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_8through16:
    movq (%rdx), %rsi
    movq -8(%rdx,%rax,1), %rdx
    movq %rsi, (%rcx)
    movq %rdx, -8(%rcx,%rax,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_emit_remainder_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_512k_memmove_end_copy_emit_remainder_encodeBlockAsm512K:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_512k_emit_remainder_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_midemit_remainder_encodeBlockAsm512K:
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_block_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm512K_memmove_move_33through64
minlz_encode_block_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm512K_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_512k_memmove_mid_end_copy_emit_remainder_encodeBlockAsm512K
minlz_encode_block_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm512K_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_512k_memmove_mid_end_copy_emit_remainder_encodeBlockAsm512K:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_512k_emit_remainder_end_encodeBlockAsm512K
minlz_encode_block_asm_512k_memmove_long_emit_remainder_encodeBlockAsm512K:
    leaq (%rcx,%rax,1), %rbx
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movq %rax, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_block_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%rdx,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_block_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBlockAsm512Klarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_block_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBlockAsm512Klarge_big_loop_back
minlz_encode_block_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%rdx,%r8,1), %xmm4
    movdqu -16(%rdx,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rax
    jae minlz_encode_block_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
    movq %rbx, %rcx
minlz_encode_block_asm_512k_emit_remainder_end_encodeBlockAsm512K:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_512k
Lepi_512k:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
.p2align 4
.globl minlz_encode_block_asm_2mb
.hidden minlz_encode_block_asm_2mb
minlz_encode_block_asm_2mb:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00000400, %rdx
    movq %rax, %rbx
    pxor %xmm0, %xmm0
minlz_encode_block_asm_2mb_zero_loop_encodeBlockAsm2MB:
    movdqu %xmm0, (%rbx)
    movdqu %xmm0, 16(%rbx)
    movdqu %xmm0, 32(%rbx)
    movdqu %xmm0, 48(%rbx)
    movdqu %xmm0, 64(%rbx)
    movdqu %xmm0, 80(%rbx)
    movdqu %xmm0, 96(%rbx)
    movdqu %xmm0, 112(%rbx)
    addq $0x80, %rbx
    decq %rdx
    jne minlz_encode_block_asm_2mb_zero_loop_encodeBlockAsm2MB
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rdx
    leaq -17(%rdx), %rbx
    leaq -17(%rdx), %rsi
    movl %esi, 8(%rsp)
    shrq $0x05, %rdx
    subl %edx, %ebx
    leaq (%rcx,%rbx,1), %rbx
    movq %rbx, (%rsp)
    movl $0x00000001, %edx
    movl %edx, 16(%rsp)
    movq 40(%rsp), %rbx
minlz_encode_block_asm_2mb_search_loop_encodeBlockAsm2MB:
    movl %edx, %esi
    subl 12(%rsp), %esi
    shrl $0x06, %esi
    leal 4(%rdx,%rsi,1), %esi
    cmpl 8(%rsp), %esi
    jae minlz_encode_block_asm_2mb_emit_remainder_encodeBlockAsm2MB
    movq (%rbx,%rdx,1), %rdi
    movl %esi, 20(%rsp)
    movq $0x0000cf1bbcdcbf9b, %r9
    movq %rdi, %r10
    movq %rdi, %r11
    shrq $0x08, %r11
    shlq $0x10, %r10
    imulq %r9, %r10
    shrq $0x32, %r10
    shlq $0x10, %r11
    imulq %r9, %r11
    shrq $0x32, %r11
    movl (%rax,%r10,4), %esi
    movl (%rax,%r11,4), %r8d
    movl %edx, (%rax,%r10,4)
    movl %edx, (%rax,%r11,4)
    movq %rdi, %r10
    shrq $0x10, %r10
    shlq $0x10, %r10
    imulq %r9, %r10
    shrq $0x32, %r10
    movl %edx, %r9d
    subl 16(%rsp), %r9d
    movl 1(%rbx,%r9,1), %r11d
    movq %rdi, %r9
    shrq $0x08, %r9
    cmpl %r11d, %r9d
    jne minlz_encode_block_asm_2mb_no_repeat_found_encodeBlockAsm2MB
    leal 1(%rdx), %edi
    movl 12(%rsp), %esi
    movl %edi, %r8d
    subl 16(%rsp), %r8d
    je minlz_encode_block_asm_2mb_repeat_extend_back_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_extend_back_loop_encodeBlockAsm2MB:
    cmpl %esi, %edi
    jbe minlz_encode_block_asm_2mb_repeat_extend_back_end_encodeBlockAsm2MB
    movb -1(%rbx,%r8,1), %r9b
    movb -1(%rbx,%rdi,1), %r10b
    cmpb %r10b, %r9b
    jne minlz_encode_block_asm_2mb_repeat_extend_back_end_encodeBlockAsm2MB
    leal -1(%rdi), %edi
    decl %r8d
    jne minlz_encode_block_asm_2mb_repeat_extend_back_loop_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_extend_back_end_encodeBlockAsm2MB:
    movl %edi, %esi
    movl 12(%rsp), %r8d
    subl %r8d, %esi
    leaq 4(%rcx,%rsi,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_2mb_dst_size_check_ok_1
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_dst_size_check_ok_1:
    leaq (%rbx,%r8,1), %r8
    leal -1(%rsi), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_2mb_one_byte_repeat_emit_lits_encodeBlockAsm2MB
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_2mb_two_bytes_repeat_emit_lits_encodeBlockAsm2MB
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_2mb_three_bytes_repeat_emit_lits_encodeBlockAsm2MB
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_2mb_memmove_long_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_three_bytes_repeat_emit_lits_encodeBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_2mb_memmove_long_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_two_bytes_repeat_emit_lits_encodeBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_2mb_memmove_midrepeat_emit_lits_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_memmove_long_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_one_byte_repeat_emit_lits_encodeBlockAsm2MB:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x10, %rsi
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_2mb_memmove_end_copy_repeat_emit_lits_encodeBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_2mb_repeat_emit_lits_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_midrepeat_emit_lits_encodeBlockAsm2MB:
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_2mb_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_2mb_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_2mb_repeat_emit_lits_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_long_repeat_emit_lits_encodeBlockAsm2MB:
    leaq (%rcx,%rsi,1), %r9
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movq %rsi, %r11
    shrq $0x05, %r11
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r12
    subq %r10, %r12
    decq %r11
    ja minlz_encode_block_asm_2mb_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%r8,%r12,1), %r10
    leaq -32(%rcx,%r12,1), %r13
minlz_encode_block_asm_2mb_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm2MBlarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r13)
    movdqu %xmm5, 16(%r13)
    addq $0x20, %r13
    addq $0x20, %r10
    addq $0x20, %r12
    decq %r11
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm2MBlarge_big_loop_back
minlz_encode_block_asm_2mb_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%r8,%r12,1), %xmm4
    movdqu -16(%r8,%r12,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r12,1)
    movdqu %xmm5, -16(%rcx,%r12,1)
    addq $0x20, %r12
    cmpq %r12, %rsi
    jae minlz_encode_block_asm_2mb_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
    movq %r9, %rcx
minlz_encode_block_asm_2mb_repeat_emit_lits_end_encodeBlockAsm2MB:
    addl $0x05, %edx
    movl %edx, %esi
    subl 16(%rsp), %esi
    movq 48(%rsp), %r8
    subl %edx, %r8d
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_loopback_16_repeat_extend_encodeBlockAsm2MB:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBlockAsm2MB
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_2mb_matchlen_bsf_16repeat_extend_encodeBlockAsm2MB
    leal -16(%r8), %r8d
    leal 16(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm2MB:
    cmpl $0x10, %r8d
    jae minlz_encode_block_asm_2mb_matchlen_loopback_16_repeat_extend_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_matchlen_match8_repeat_extend_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_16repeat_extend_encodeBlockAsm2MB:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match8_repeat_extend_encodeBlockAsm2MB:
    cmpl $0x08, %r8d
    jb minlz_encode_block_asm_2mb_matchlen_match4_repeat_extend_encodeBlockAsm2MB
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBlockAsm2MB
    leal -8(%r8), %r8d
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_match4_repeat_extend_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBlockAsm2MB:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match4_repeat_extend_encodeBlockAsm2MB:
    cmpl $0x04, %r8d
    jb minlz_encode_block_asm_2mb_matchlen_match2_repeat_extend_encodeBlockAsm2MB
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match2_repeat_extend_encodeBlockAsm2MB
    leal -4(%r8), %r8d
    leal 4(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_match2_repeat_extend_encodeBlockAsm2MB:
    cmpl $0x01, %r8d
    je minlz_encode_block_asm_2mb_matchlen_match1_repeat_extend_encodeBlockAsm2MB
    jb minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match1_repeat_extend_encodeBlockAsm2MB
    leal 2(%r11), %r11d
    subl $0x02, %r8d
    je minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match1_repeat_extend_encodeBlockAsm2MB:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB
    leal 1(%r11), %r11d
minlz_encode_block_asm_2mb_repeat_extend_forward_end_encodeBlockAsm2MB:
    addl %r11d, %edx
    movl %edx, %esi
    subl %edi, %esi
    movl 16(%rsp), %edi
    leal -1(%rsi), %edi
    cmpl $0x1d, %esi
    jbe minlz_encode_block_asm_2mb_repeat_one_match_repeat_encodeBlockAsm2MB
    leal -30(%rsi), %edi
    cmpl $0x0000011e, %esi
    jb minlz_encode_block_asm_2mb_repeat_two_match_repeat_encodeBlockAsm2MB
    cmpl $0x0001001e, %esi
    jb minlz_encode_block_asm_2mb_repeat_three_match_repeat_encodeBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %edi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_2mb_repeat_end_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_three_match_repeat_encodeBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %di, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_2mb_repeat_end_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_two_match_repeat_encodeBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %dil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_2mb_repeat_end_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_one_match_repeat_encodeBlockAsm2MB:
    xorl %edi, %edi
    leal -4(%rdi,%rsi,8), %edi
    movb %dil, (%rcx)
    addq $0x01, %rcx
minlz_encode_block_asm_2mb_repeat_end_emit_encodeBlockAsm2MB:
    movl %edx, 12(%rsp)
    jmp minlz_encode_block_asm_2mb_search_loop_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_no_repeat_found_encodeBlockAsm2MB:
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_2mb_candidate_match_encodeBlockAsm2MB
    shrq $0x08, %rdi
    movl (%rax,%r10,4), %esi
    leal 2(%rdx), %r9d
    cmpl %edi, (%rbx,%r8,1)
    je minlz_encode_block_asm_2mb_candidate2_match_encodeBlockAsm2MB
    movl %r9d, (%rax,%r10,4)
    shrq $0x08, %rdi
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_2mb_candidate3_match_encodeBlockAsm2MB
    movl 20(%rsp), %edx
    jmp minlz_encode_block_asm_2mb_search_loop_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_candidate3_match_encodeBlockAsm2MB:
    addl $0x02, %edx
    jmp minlz_encode_block_asm_2mb_candidate_match_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_candidate2_match_encodeBlockAsm2MB:
    movl %r9d, (%rax,%r10,4)
    incl %edx
    movl %r8d, %esi
minlz_encode_block_asm_2mb_candidate_match_encodeBlockAsm2MB:
    movl 12(%rsp), %edi
    testl %esi, %esi
    je minlz_encode_block_asm_2mb_match_extend_back_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_match_extend_back_loop_encodeBlockAsm2MB:
    cmpl %edi, %edx
    jbe minlz_encode_block_asm_2mb_match_extend_back_end_encodeBlockAsm2MB
    movb -1(%rbx,%rsi,1), %r8b
    movb -1(%rbx,%rdx,1), %r9b
    cmpb %r9b, %r8b
    jne minlz_encode_block_asm_2mb_match_extend_back_end_encodeBlockAsm2MB
    leal -1(%rdx), %edx
    decl %esi
    je minlz_encode_block_asm_2mb_match_extend_back_end_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_match_extend_back_loop_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_match_extend_back_end_encodeBlockAsm2MB:
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_2mb_dst_size_check_ok_2
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_dst_size_check_ok_2:
    movl %edx, %r8d
    movl %edx, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    addl $0x04, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_loop_16_entry_match_nolit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_loopback_16_match_nolit_encodeBlockAsm2MB:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit_encodeBlockAsm2MB
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_2mb_matchlen_bsf_16match_nolit_encodeBlockAsm2MB
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_loop_16_entry_match_nolit_encodeBlockAsm2MB:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_2mb_matchlen_loopback_16_match_nolit_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_matchlen_match8_match_nolit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_16match_nolit_encodeBlockAsm2MB:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match8_match_nolit_encodeBlockAsm2MB:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_2mb_matchlen_match4_match_nolit_encodeBlockAsm2MB
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit_encodeBlockAsm2MB
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_match4_match_nolit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit_encodeBlockAsm2MB:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match4_match_nolit_encodeBlockAsm2MB:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_2mb_matchlen_match2_match_nolit_encodeBlockAsm2MB
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match2_match_nolit_encodeBlockAsm2MB
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_match2_match_nolit_encodeBlockAsm2MB:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_2mb_matchlen_match1_match_nolit_encodeBlockAsm2MB
    jb minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match1_match_nolit_encodeBlockAsm2MB
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match1_match_nolit_encodeBlockAsm2MB:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB
    leal 1(%r11), %r11d
minlz_encode_block_asm_2mb_match_nolit_end_encodeBlockAsm2MB:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl 16(%rsp), %esi
    movl 12(%rsp), %edi
    movl %edx, 12(%rsp)
    subl %edi, %r8d
    je minlz_encode_block_asm_2mb_match_nolits_copy_encodeBlockAsm2MB
    leaq (%rbx,%rdi,1), %rdi
    cmpl $0x03, %r8d
    ja minlz_encode_block_asm_2mb_match_emit_lits_copy_encodeBlockAsm2MB
    cmpl $0x40, %esi
    jb minlz_encode_block_asm_2mb_match_emit_lits_copy_encodeBlockAsm2MB
    movl (%rdi), %edi
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_2mb_match_emit_copy2lits_encodeBlockAsm2MB
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    leal 7(%rsi,%r8,8), %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_2mb_emit_copy3_0_match_emit_lits_encodeBlockAsm2MB
    leal -60(%r11), %r9d
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy3_1_match_emit_lits_encodeBlockAsm2MB
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy3_2_match_emit_lits_encodeBlockAsm2MB
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_2mb_match_emit_copy_litsencodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_2_match_emit_lits_encodeBlockAsm2MB:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_2mb_match_emit_copy_litsencodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_1_match_emit_lits_encodeBlockAsm2MB:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_2mb_match_emit_copy_litsencodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_0_match_emit_lits_encodeBlockAsm2MB:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
minlz_encode_block_asm_2mb_match_emit_copy_litsencodeBlockAsm2MB:
    movl %edi, (%rcx)
    addq %r8, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_match_emit_copy2lits_encodeBlockAsm2MB:
    xorq %r9, %r9
    subl $0x40, %esi
    leal -11(%r11), %r10d
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x07, %r11d
    cmovge %r10d, %r9d
    movq $0x00000007, %rsi
    cmovl %r11d, %esi
    leal -1(%r8,%rsi,4), %esi
    movl $0x00000003, %r10d
    leal (%r10,%rsi,8), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
    movl %edi, (%rcx)
    addq %r8, %rcx
    testl %r9d, %r9d
    je minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
    leal -1(%r9), %esi
    cmpl $0x1d, %r9d
    jbe minlz_encode_block_asm_2mb_repeat_one_match_emit_repeat_copy2_encodeBlockAsm2MB
    leal -30(%r9), %esi
    cmpl $0x0000011e, %r9d
    jb minlz_encode_block_asm_2mb_repeat_two_match_emit_repeat_copy2_encodeBlockAsm2MB
    cmpl $0x0001001e, %r9d
    jb minlz_encode_block_asm_2mb_repeat_three_match_emit_repeat_copy2_encodeBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_three_match_emit_repeat_copy2_encodeBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_two_match_emit_repeat_copy2_encodeBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_one_match_emit_repeat_copy2_encodeBlockAsm2MB:
    xorl %esi, %esi
    leal -4(%rsi,%r9,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_match_emit_lits_copy_encodeBlockAsm2MB:
    leaq 4(%rcx,%r8,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_2mb_dst_size_check_ok_3
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_dst_size_check_ok_3:
    leal -1(%r8), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_2mb_one_byte_match_emit_encodeBlockAsm2MB
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_2mb_two_bytes_match_emit_encodeBlockAsm2MB
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_2mb_three_bytes_match_emit_encodeBlockAsm2MB
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_2mb_memmove_long_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_three_bytes_match_emit_encodeBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_2mb_memmove_long_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_two_bytes_match_emit_encodeBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_2mb_memmove_midmatch_emit_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_memmove_long_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_one_byte_match_emit_encodeBlockAsm2MB:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r8,1), %r9
    cmpq $0x10, %r8
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_8through16:
    movdqu (%rdi), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_match_emit_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_2mb_memmove_end_copy_match_emit_encodeBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolits_copy_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_midmatch_emit_encodeBlockAsm2MB:
    leaq (%rcx,%r8,1), %r9
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_2mb_memmove_mid_end_copy_match_emit_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_2mb_memmove_mid_end_copy_match_emit_encodeBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolits_copy_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_long_match_emit_encodeBlockAsm2MB:
    leaq (%rcx,%r8,1), %r9
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movq %r8, %r12
    shrq $0x05, %r12
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r13
    subq %r10, %r13
    decq %r12
    ja minlz_encode_block_asm_2mb_emit_lit_memmove_long_match_emit_encodeBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%rdi,%r13,1), %r10
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_block_asm_2mb_emit_lit_memmove_long_match_emit_encodeBlockAsm2MBlarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r10
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_long_match_emit_encodeBlockAsm2MBlarge_big_loop_back
minlz_encode_block_asm_2mb_emit_lit_memmove_long_match_emit_encodeBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%rdi,%r13,1), %xmm4
    movdqu -16(%rdi,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %r8
    jae minlz_encode_block_asm_2mb_emit_lit_memmove_long_match_emit_encodeBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
    movq %r9, %rcx
minlz_encode_block_asm_2mb_match_nolits_copy_encodeBlockAsm2MB:
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_2mb_two_byte_offset_match_nolit_encodeBlockAsm2MB
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    addl $0x07, %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_2mb_emit_copy3_0_match_nolit_encodeBlockAsm2MB_emit3
    leal -60(%r11), %edi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy3_1_match_nolit_encodeBlockAsm2MB_emit3
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy3_2_match_nolit_encodeBlockAsm2MB_emit3
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %edi, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_2_match_nolit_encodeBlockAsm2MB_emit3:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %di, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_1_match_nolit_encodeBlockAsm2MB_emit3:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %dil, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy3_0_match_nolit_encodeBlockAsm2MB_emit3:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_two_byte_offset_match_nolit_encodeBlockAsm2MB:
    cmpl $0x00000400, %esi
    ja minlz_encode_block_asm_2mb_two_byte_match_nolit_encodeBlockAsm2MB
    cmpl $0x00000013, %r11d
    jae minlz_encode_block_asm_2mb_emit_one_longer_match_nolit_encodeBlockAsm2MB
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal -15(%rsi,%r11,4), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_one_longer_match_nolit_encodeBlockAsm2MB:
    cmpl $0x00000112, %r11d
    jae minlz_encode_block_asm_2mb_emit_copy1_repeat_match_nolit_encodeBlockAsm2MB
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 61(%rsi), %esi
    movw %si, (%rcx)
    leal -18(%r11), %esi
    movb %sil, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy1_repeat_match_nolit_encodeBlockAsm2MB:
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 57(%rsi), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r11d
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_block_asm_2mb_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_block_asm_2mb_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB
    cmpl $0x0001001e, %r11d
    jb minlz_encode_block_asm_2mb_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm2MB:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_two_byte_match_nolit_encodeBlockAsm2MB:
    leal -64(%rsi), %esi
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_2mb_emit_copy2_0_match_nolit_encodeBlockAsm2MB_emit2
    leal -60(%r11), %esi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy2_1_match_nolit_encodeBlockAsm2MB_emit2
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_2mb_emit_copy2_2_match_nolit_encodeBlockAsm2MB_emit2
    movb $0xfe, (%rcx)
    movl %esi, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy2_2_match_nolit_encodeBlockAsm2MB_emit2:
    movb $0xfa, (%rcx)
    movw %si, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy2_1_match_nolit_encodeBlockAsm2MB_emit2:
    movb $0xf6, (%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_copy2_0_match_nolit_encodeBlockAsm2MB_emit2:
    movl $0x00000002, %esi
    leal (%rsi,%r11,4), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
minlz_encode_block_asm_2mb_match_nolit_emitcopy_end_encodeBlockAsm2MB:
    cmpl 8(%rsp), %edx
    jae minlz_encode_block_asm_2mb_emit_remainder_encodeBlockAsm2MB
    movq -2(%rbx,%rdx,1), %rdi
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_2mb_match_nolit_dst_ok_encodeBlockAsm2MB
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_match_nolit_dst_ok_encodeBlockAsm2MB:
    movq $0x0000cf1bbcdcbf9b, %rsi
    movq %rdi, %r8
    shrq $0x10, %rdi
    movq %rdi, %r9
    shlq $0x10, %r8
    imulq %rsi, %r8
    shrq $0x32, %r8
    shlq $0x10, %r9
    imulq %rsi, %r9
    shrq $0x32, %r9
    leal -2(%rdx), %r10d
    movl (%rax,%r9,4), %esi
    movl %r10d, (%rax,%r8,4)
    movl %edx, (%rax,%r9,4)
    movl %edx, %r8d
    incl %edx
    cmpl %edi, (%rbx,%rsi,1)
    jne minlz_encode_block_asm_2mb_search_loop_encodeBlockAsm2MB
    movl %r8d, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_2mb_dst_size_check_ok_4
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_dst_size_check_ok_4:
    addl $0x03, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r8
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_loopback_16_match_nolit2_encodeBlockAsm2MB:
    movq (%r8,%r11,1), %r9
    movq 8(%r8,%r11,1), %r10
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit2_encodeBlockAsm2MB
    xorq 8(%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_2mb_matchlen_bsf_16match_nolit2_encodeBlockAsm2MB
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm2MB:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_2mb_matchlen_loopback_16_match_nolit2_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_matchlen_match8_match_nolit2_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_16match_nolit2_encodeBlockAsm2MB:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal 8(%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match8_match_nolit2_encodeBlockAsm2MB:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_2mb_matchlen_match4_match_nolit2_encodeBlockAsm2MB
    movq (%r8,%r11,1), %r9
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit2_encodeBlockAsm2MB
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_2mb_matchlen_match4_match_nolit2_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_bsf_8_match_nolit2_encodeBlockAsm2MB:
    tzcntq %r9, %r9
    sarq $0x03, %r9
    leal (%r11,%r9,1), %r11d
    jmp minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match4_match_nolit2_encodeBlockAsm2MB:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_2mb_matchlen_match2_match_nolit2_encodeBlockAsm2MB
    movl (%r8,%r11,1), %r9d
    cmpl %r9d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match2_match_nolit2_encodeBlockAsm2MB
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_2mb_matchlen_match2_match_nolit2_encodeBlockAsm2MB:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_2mb_matchlen_match1_match_nolit2_encodeBlockAsm2MB
    jb minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB
    movw (%r8,%r11,1), %r9w
    cmpw %r9w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_matchlen_match1_match_nolit2_encodeBlockAsm2MB
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_matchlen_match1_match_nolit2_encodeBlockAsm2MB:
    movb (%r8,%r11,1), %r9b
    cmpb %r9b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB
    leal 1(%r11), %r11d
minlz_encode_block_asm_2mb_match_nolit2_end_encodeBlockAsm2MB:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl %edx, 12(%rsp)
    movl 16(%rsp), %esi
    jmp minlz_encode_block_asm_2mb_match_nolits_copy_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_remainder_encodeBlockAsm2MB:
    movq 48(%rsp), %rax
    movl 12(%rsp), %edx
    subl %edx, %eax
    je minlz_encode_block_asm_2mb_emit_remainder_end_encodeBlockAsm2MB
    leaq (%rbx,%rdx,1), %rdx
    leaq 4(%rcx,%rax,1), %rbx
    cmpq (%rsp), %rbx
    jb minlz_encode_block_asm_2mb_dst_size_check_ok_5
    movq $0x00000000, 64(%rsp)
    jmp Lepi_2mb
minlz_encode_block_asm_2mb_dst_size_check_ok_5:
    leal -1(%rax), %ebx
    cmpl $0x1d, %ebx
    jb minlz_encode_block_asm_2mb_one_byte_emit_remainder_encodeBlockAsm2MB
    subl $0x1d, %ebx
    cmpl $0x00000100, %ebx
    jb minlz_encode_block_asm_2mb_two_bytes_emit_remainder_encodeBlockAsm2MB
    cmpl $0x00010000, %ebx
    jb minlz_encode_block_asm_2mb_three_bytes_emit_remainder_encodeBlockAsm2MB
    movl %ebx, %esi
    shrl $0x10, %esi
    movb $0xf8, (%rcx)
    movw %bx, 1(%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_2mb_memmove_long_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_three_bytes_emit_remainder_encodeBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %bx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_2mb_memmove_long_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_two_bytes_emit_remainder_encodeBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %bl, 1(%rcx)
    addl $0x1d, %ebx
    addq $0x02, %rcx
    cmpl $0x40, %ebx
    jb minlz_encode_block_asm_2mb_memmove_midemit_remainder_encodeBlockAsm2MB
    jmp minlz_encode_block_asm_2mb_memmove_long_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_one_byte_emit_remainder_encodeBlockAsm2MB:
    shlb $0x03, %bl
    movb %bl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x03, %rax
    jb minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_1or2
    je minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_3
    cmpq $0x08, %rax
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_4through8
    cmpq $0x10, %rax
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_1or2:
    movb (%rdx), %sil
    movb -1(%rdx,%rax,1), %dl
    movb %sil, (%rcx)
    movb %dl, -1(%rcx,%rax,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_3:
    movw (%rdx), %si
    movb 2(%rdx), %dl
    movw %si, (%rcx)
    movb %dl, 2(%rcx)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_4through8:
    movl (%rdx), %esi
    movl -4(%rdx,%rax,1), %edx
    movl %esi, (%rcx)
    movl %edx, -4(%rcx,%rax,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_8through16:
    movq (%rdx), %rsi
    movq -8(%rdx,%rax,1), %rdx
    movq %rsi, (%rcx)
    movq %rdx, -8(%rcx,%rax,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_emit_remainder_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_2mb_memmove_end_copy_emit_remainder_encodeBlockAsm2MB:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_2mb_emit_remainder_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_midemit_remainder_encodeBlockAsm2MB:
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_block_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm2MB_memmove_move_33through64
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm2MB_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_2mb_memmove_mid_end_copy_emit_remainder_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm2MB_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_2mb_memmove_mid_end_copy_emit_remainder_encodeBlockAsm2MB:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_2mb_emit_remainder_end_encodeBlockAsm2MB
minlz_encode_block_asm_2mb_memmove_long_emit_remainder_encodeBlockAsm2MB:
    leaq (%rcx,%rax,1), %rbx
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movq %rax, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_block_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%rdx,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_block_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBlockAsm2MBlarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_block_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBlockAsm2MBlarge_big_loop_back
minlz_encode_block_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%rdx,%r8,1), %xmm4
    movdqu -16(%rdx,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rax
    jae minlz_encode_block_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
    movq %rbx, %rcx
minlz_encode_block_asm_2mb_emit_remainder_end_encodeBlockAsm2MB:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_2mb
Lepi_2mb:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
.p2align 4
.globl minlz_encode_block_asm
.hidden minlz_encode_block_asm
minlz_encode_block_asm:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00000400, %rdx
    movq %rax, %rbx
    pxor %xmm0, %xmm0
minlz_encode_block_asm_zero_loop_encodeBlockAsm:
    movdqu %xmm0, (%rbx)
    movdqu %xmm0, 16(%rbx)
    movdqu %xmm0, 32(%rbx)
    movdqu %xmm0, 48(%rbx)
    movdqu %xmm0, 64(%rbx)
    movdqu %xmm0, 80(%rbx)
    movdqu %xmm0, 96(%rbx)
    movdqu %xmm0, 112(%rbx)
    addq $0x80, %rbx
    decq %rdx
    jne minlz_encode_block_asm_zero_loop_encodeBlockAsm
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rdx
    leaq -17(%rdx), %rbx
    leaq -17(%rdx), %rsi
    movl %esi, 8(%rsp)
    shrq $0x05, %rdx
    subl %edx, %ebx
    leaq (%rcx,%rbx,1), %rbx
    movq %rbx, (%rsp)
    movl $0x00000001, %edx
    movl %edx, 16(%rsp)
    movq 40(%rsp), %rbx
minlz_encode_block_asm_search_loop_encodeBlockAsm:
    movl %edx, %esi
    subl 12(%rsp), %esi
    shrl $0x06, %esi
    leal 4(%rdx,%rsi,1), %esi
    cmpl 8(%rsp), %esi
    jae minlz_encode_block_asm_emit_remainder_encodeBlockAsm
    movq (%rbx,%rdx,1), %rdi
    leal -2162685(%rdx), %r8d
    movl %esi, 20(%rsp)
    movq $0x0000cf1bbcdcbf9b, %r10
    movq %rdi, %r11
    movq %rdi, %r12
    shrq $0x08, %r12
    shlq $0x10, %r11
    imulq %r10, %r11
    shrq $0x32, %r11
    shlq $0x10, %r12
    imulq %r10, %r12
    shrq $0x32, %r12
    movl (%rax,%r11,4), %esi
    movl (%rax,%r12,4), %r9d
    movl %edx, (%rax,%r11,4)
    movl %edx, (%rax,%r12,4)
    movq %rdi, %r11
    shrq $0x10, %r11
    shlq $0x10, %r11
    imulq %r10, %r11
    shrq $0x32, %r11
    movl %edx, %r10d
    subl 16(%rsp), %r10d
    movl 1(%rbx,%r10,1), %r12d
    movq %rdi, %r10
    shrq $0x08, %r10
    cmpl %r12d, %r10d
    jne minlz_encode_block_asm_no_repeat_found_encodeBlockAsm
    leal 1(%rdx), %edi
    movl 12(%rsp), %esi
    movl %edi, %r8d
    subl 16(%rsp), %r8d
    je minlz_encode_block_asm_repeat_extend_back_end_encodeBlockAsm
minlz_encode_block_asm_repeat_extend_back_loop_encodeBlockAsm:
    cmpl %esi, %edi
    jbe minlz_encode_block_asm_repeat_extend_back_end_encodeBlockAsm
    movb -1(%rbx,%r8,1), %r9b
    movb -1(%rbx,%rdi,1), %r10b
    cmpb %r10b, %r9b
    jne minlz_encode_block_asm_repeat_extend_back_end_encodeBlockAsm
    leal -1(%rdi), %edi
    decl %r8d
    jne minlz_encode_block_asm_repeat_extend_back_loop_encodeBlockAsm
minlz_encode_block_asm_repeat_extend_back_end_encodeBlockAsm:
    movl %edi, %esi
    movl 12(%rsp), %r8d
    subl %r8d, %esi
    leaq 4(%rcx,%rsi,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_dst_size_check_ok_1
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_dst_size_check_ok_1:
    leaq (%rbx,%r8,1), %r8
    leal -1(%rsi), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_one_byte_repeat_emit_lits_encodeBlockAsm
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_two_bytes_repeat_emit_lits_encodeBlockAsm
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_three_bytes_repeat_emit_lits_encodeBlockAsm
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_memmove_long_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_three_bytes_repeat_emit_lits_encodeBlockAsm:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_memmove_long_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_two_bytes_repeat_emit_lits_encodeBlockAsm:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_memmove_midrepeat_emit_lits_encodeBlockAsm
    jmp minlz_encode_block_asm_memmove_long_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_one_byte_repeat_emit_lits_encodeBlockAsm:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x10, %rsi
    jbe minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_8through16
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_memmove_end_copy_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_memmove_end_copy_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_repeat_emit_lits_encodeBlockAsm_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_memmove_end_copy_repeat_emit_lits_encodeBlockAsm:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_repeat_emit_lits_end_encodeBlockAsm
minlz_encode_block_asm_memmove_midrepeat_emit_lits_encodeBlockAsm:
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x20, %rsi
    jbe minlz_encode_block_asm_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_block_asm_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_mid_repeat_emit_lits_encodeBlockAsm_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_block_asm_memmove_mid_end_copy_repeat_emit_lits_encodeBlockAsm:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_repeat_emit_lits_end_encodeBlockAsm
minlz_encode_block_asm_memmove_long_repeat_emit_lits_encodeBlockAsm:
    leaq (%rcx,%rsi,1), %r9
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movq %rsi, %r11
    shrq $0x05, %r11
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r12
    subq %r10, %r12
    decq %r11
    ja minlz_encode_block_asm_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsmlarge_forward_sse_loop_32
    leaq -32(%r8,%r12,1), %r10
    leaq -32(%rcx,%r12,1), %r13
minlz_encode_block_asm_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsmlarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r13)
    movdqu %xmm5, 16(%r13)
    addq $0x20, %r13
    addq $0x20, %r10
    addq $0x20, %r12
    decq %r11
    jbe minlz_encode_block_asm_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsmlarge_big_loop_back
minlz_encode_block_asm_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%r8,%r12,1), %xmm4
    movdqu -16(%r8,%r12,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r12,1)
    movdqu %xmm5, -16(%rcx,%r12,1)
    addq $0x20, %r12
    cmpq %r12, %rsi
    jae minlz_encode_block_asm_emit_lit_memmove_long_repeat_emit_lits_encodeBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
    movq %r9, %rcx
minlz_encode_block_asm_repeat_emit_lits_end_encodeBlockAsm:
    addl $0x05, %edx
    movl %edx, %esi
    subl 16(%rsp), %esi
    movq 48(%rsp), %r8
    subl %edx, %r8d
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm
minlz_encode_block_asm_matchlen_loopback_16_repeat_extend_encodeBlockAsm:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_matchlen_bsf_8_repeat_extend_encodeBlockAsm
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_matchlen_bsf_16repeat_extend_encodeBlockAsm
    leal -16(%r8), %r8d
    leal 16(%r11), %r11d
minlz_encode_block_asm_matchlen_loop_16_entry_repeat_extend_encodeBlockAsm:
    cmpl $0x10, %r8d
    jae minlz_encode_block_asm_matchlen_loopback_16_repeat_extend_encodeBlockAsm
    jmp minlz_encode_block_asm_matchlen_match8_repeat_extend_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_16repeat_extend_encodeBlockAsm:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match8_repeat_extend_encodeBlockAsm:
    cmpl $0x08, %r8d
    jb minlz_encode_block_asm_matchlen_match4_repeat_extend_encodeBlockAsm
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_matchlen_bsf_8_repeat_extend_encodeBlockAsm
    leal -8(%r8), %r8d
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_matchlen_match4_repeat_extend_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_8_repeat_extend_encodeBlockAsm:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match4_repeat_extend_encodeBlockAsm:
    cmpl $0x04, %r8d
    jb minlz_encode_block_asm_matchlen_match2_repeat_extend_encodeBlockAsm
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match2_repeat_extend_encodeBlockAsm
    leal -4(%r8), %r8d
    leal 4(%r11), %r11d
minlz_encode_block_asm_matchlen_match2_repeat_extend_encodeBlockAsm:
    cmpl $0x01, %r8d
    je minlz_encode_block_asm_matchlen_match1_repeat_extend_encodeBlockAsm
    jb minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match1_repeat_extend_encodeBlockAsm
    leal 2(%r11), %r11d
    subl $0x02, %r8d
    je minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match1_repeat_extend_encodeBlockAsm:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm
    leal 1(%r11), %r11d
minlz_encode_block_asm_repeat_extend_forward_end_encodeBlockAsm:
    addl %r11d, %edx
    movl %edx, %esi
    subl %edi, %esi
    movl 16(%rsp), %edi
    leal -1(%rsi), %edi
    cmpl $0x1d, %esi
    jbe minlz_encode_block_asm_repeat_one_match_repeat_encodeBlockAsm
    leal -30(%rsi), %edi
    cmpl $0x0000011e, %esi
    jb minlz_encode_block_asm_repeat_two_match_repeat_encodeBlockAsm
    cmpl $0x0001001e, %esi
    jb minlz_encode_block_asm_repeat_three_match_repeat_encodeBlockAsm
    movb $0xfc, (%rcx)
    movl %edi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_repeat_end_emit_encodeBlockAsm
minlz_encode_block_asm_repeat_three_match_repeat_encodeBlockAsm:
    movb $0xf4, (%rcx)
    movw %di, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_repeat_end_emit_encodeBlockAsm
minlz_encode_block_asm_repeat_two_match_repeat_encodeBlockAsm:
    movb $0xec, (%rcx)
    movb %dil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_repeat_end_emit_encodeBlockAsm
minlz_encode_block_asm_repeat_one_match_repeat_encodeBlockAsm:
    xorl %edi, %edi
    leal -4(%rdi,%rsi,8), %edi
    movb %dil, (%rcx)
    addq $0x01, %rcx
minlz_encode_block_asm_repeat_end_emit_encodeBlockAsm:
    movl %edx, 12(%rsp)
    jmp minlz_encode_block_asm_search_loop_encodeBlockAsm
minlz_encode_block_asm_no_repeat_found_encodeBlockAsm:
    cmpl %r8d, %esi
    jle minlz_encode_block_asm_offset_ok_0_encodeBlockAsm
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_candidate_match_encodeBlockAsm
minlz_encode_block_asm_offset_ok_0_encodeBlockAsm:
    shrq $0x08, %rdi
    movl (%rax,%r11,4), %esi
    leal 2(%rdx), %r10d
    cmpl %r8d, %r9d
    jle minlz_encode_block_asm_offset_ok_1_encodeBlockAsm
    cmpl %edi, (%rbx,%r9,1)
    je minlz_encode_block_asm_candidate2_match_encodeBlockAsm
minlz_encode_block_asm_offset_ok_1_encodeBlockAsm:
    movl %r10d, (%rax,%r11,4)
    shrq $0x08, %rdi
    cmpl %r8d, %esi
    jle minlz_encode_block_asm_offset_ok_2_encodeBlockAsm
    cmpl %edi, (%rbx,%rsi,1)
    je minlz_encode_block_asm_candidate3_match_encodeBlockAsm
minlz_encode_block_asm_offset_ok_2_encodeBlockAsm:
    movl 20(%rsp), %edx
    jmp minlz_encode_block_asm_search_loop_encodeBlockAsm
minlz_encode_block_asm_candidate3_match_encodeBlockAsm:
    addl $0x02, %edx
    jmp minlz_encode_block_asm_candidate_match_encodeBlockAsm
minlz_encode_block_asm_candidate2_match_encodeBlockAsm:
    movl %r10d, (%rax,%r11,4)
    incl %edx
    movl %r9d, %esi
minlz_encode_block_asm_candidate_match_encodeBlockAsm:
    movl 12(%rsp), %edi
    testl %esi, %esi
    je minlz_encode_block_asm_match_extend_back_end_encodeBlockAsm
minlz_encode_block_asm_match_extend_back_loop_encodeBlockAsm:
    cmpl %edi, %edx
    jbe minlz_encode_block_asm_match_extend_back_end_encodeBlockAsm
    movb -1(%rbx,%rsi,1), %r8b
    movb -1(%rbx,%rdx,1), %r9b
    cmpb %r9b, %r8b
    jne minlz_encode_block_asm_match_extend_back_end_encodeBlockAsm
    leal -1(%rdx), %edx
    decl %esi
    je minlz_encode_block_asm_match_extend_back_end_encodeBlockAsm
    jmp minlz_encode_block_asm_match_extend_back_loop_encodeBlockAsm
minlz_encode_block_asm_match_extend_back_end_encodeBlockAsm:
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_dst_size_check_ok_2
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_dst_size_check_ok_2:
    movl %edx, %r8d
    movl %edx, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    addl $0x04, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r9
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_matchlen_loop_16_entry_match_nolit_encodeBlockAsm
minlz_encode_block_asm_matchlen_loopback_16_match_nolit_encodeBlockAsm:
    movq (%r9,%r11,1), %r10
    movq 8(%r9,%r11,1), %r12
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_matchlen_bsf_8_match_nolit_encodeBlockAsm
    xorq 8(%rsi,%r11,1), %r12
    jne minlz_encode_block_asm_matchlen_bsf_16match_nolit_encodeBlockAsm
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_matchlen_loop_16_entry_match_nolit_encodeBlockAsm:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_matchlen_loopback_16_match_nolit_encodeBlockAsm
    jmp minlz_encode_block_asm_matchlen_match8_match_nolit_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_16match_nolit_encodeBlockAsm:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_block_asm_match_nolit_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match8_match_nolit_encodeBlockAsm:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_matchlen_match4_match_nolit_encodeBlockAsm
    movq (%r9,%r11,1), %r10
    xorq (%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_matchlen_bsf_8_match_nolit_encodeBlockAsm
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_matchlen_match4_match_nolit_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_8_match_nolit_encodeBlockAsm:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_match_nolit_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match4_match_nolit_encodeBlockAsm:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_matchlen_match2_match_nolit_encodeBlockAsm
    movl (%r9,%r11,1), %r10d
    cmpl %r10d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match2_match_nolit_encodeBlockAsm
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_matchlen_match2_match_nolit_encodeBlockAsm:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_matchlen_match1_match_nolit_encodeBlockAsm
    jb minlz_encode_block_asm_match_nolit_end_encodeBlockAsm
    movw (%r9,%r11,1), %r10w
    cmpw %r10w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match1_match_nolit_encodeBlockAsm
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_match_nolit_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match1_match_nolit_encodeBlockAsm:
    movb (%r9,%r11,1), %r10b
    cmpb %r10b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_match_nolit_end_encodeBlockAsm
    leal 1(%r11), %r11d
minlz_encode_block_asm_match_nolit_end_encodeBlockAsm:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl 16(%rsp), %esi
    movl 12(%rsp), %edi
    movl %edx, 12(%rsp)
    subl %edi, %r8d
    je minlz_encode_block_asm_match_nolits_copy_encodeBlockAsm
    leaq (%rbx,%rdi,1), %rdi
    cmpl $0x03, %r8d
    ja minlz_encode_block_asm_match_emit_lits_copy_encodeBlockAsm
    cmpl $0x40, %esi
    jb minlz_encode_block_asm_match_emit_lits_copy_encodeBlockAsm
    movl (%rdi), %edi
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_match_emit_copy2lits_encodeBlockAsm
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    leal 7(%rsi,%r8,8), %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_emit_copy3_0_match_emit_lits_encodeBlockAsm
    leal -60(%r11), %r9d
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_emit_copy3_1_match_emit_lits_encodeBlockAsm
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_emit_copy3_2_match_emit_lits_encodeBlockAsm
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_match_emit_copy_litsencodeBlockAsm
minlz_encode_block_asm_emit_copy3_2_match_emit_lits_encodeBlockAsm:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_match_emit_copy_litsencodeBlockAsm
minlz_encode_block_asm_emit_copy3_1_match_emit_lits_encodeBlockAsm:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_match_emit_copy_litsencodeBlockAsm
minlz_encode_block_asm_emit_copy3_0_match_emit_lits_encodeBlockAsm:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
minlz_encode_block_asm_match_emit_copy_litsencodeBlockAsm:
    movl %edi, (%rcx)
    addq %r8, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_match_emit_copy2lits_encodeBlockAsm:
    xorq %r9, %r9
    subl $0x40, %esi
    leal -11(%r11), %r10d
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x07, %r11d
    cmovge %r10d, %r9d
    movq $0x00000007, %rsi
    cmovl %r11d, %esi
    leal -1(%r8,%rsi,4), %esi
    movl $0x00000003, %r10d
    leal (%r10,%rsi,8), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
    movl %edi, (%rcx)
    addq %r8, %rcx
    testl %r9d, %r9d
    je minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
    leal -1(%r9), %esi
    cmpl $0x1d, %r9d
    jbe minlz_encode_block_asm_repeat_one_match_emit_repeat_copy2_encodeBlockAsm
    leal -30(%r9), %esi
    cmpl $0x0000011e, %r9d
    jb minlz_encode_block_asm_repeat_two_match_emit_repeat_copy2_encodeBlockAsm
    cmpl $0x0001001e, %r9d
    jb minlz_encode_block_asm_repeat_three_match_emit_repeat_copy2_encodeBlockAsm
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_three_match_emit_repeat_copy2_encodeBlockAsm:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_two_match_emit_repeat_copy2_encodeBlockAsm:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_one_match_emit_repeat_copy2_encodeBlockAsm:
    xorl %esi, %esi
    leal -4(%rsi,%r9,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_match_emit_lits_copy_encodeBlockAsm:
    leaq 4(%rcx,%r8,1), %r9
    cmpq (%rsp), %r9
    jb minlz_encode_block_asm_dst_size_check_ok_3
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_dst_size_check_ok_3:
    leal -1(%r8), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_block_asm_one_byte_match_emit_encodeBlockAsm
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_block_asm_two_bytes_match_emit_encodeBlockAsm
    cmpl $0x00010000, %r9d
    jb minlz_encode_block_asm_three_bytes_match_emit_encodeBlockAsm
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_memmove_long_match_emit_encodeBlockAsm
minlz_encode_block_asm_three_bytes_match_emit_encodeBlockAsm:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_block_asm_memmove_long_match_emit_encodeBlockAsm
minlz_encode_block_asm_two_bytes_match_emit_encodeBlockAsm:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_block_asm_memmove_midmatch_emit_encodeBlockAsm
    jmp minlz_encode_block_asm_memmove_long_match_emit_encodeBlockAsm
minlz_encode_block_asm_one_byte_match_emit_encodeBlockAsm:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r8,1), %r9
    cmpq $0x10, %r8
    jbe minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_8through16
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_8through16:
    movdqu (%rdi), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_block_asm_memmove_end_copy_match_emit_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_memmove_end_copy_match_emit_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_match_emit_encodeBlockAsm_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_memmove_end_copy_match_emit_encodeBlockAsm:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_match_nolits_copy_encodeBlockAsm
minlz_encode_block_asm_memmove_midmatch_emit_encodeBlockAsm:
    leaq (%rcx,%r8,1), %r9
    cmpq $0x20, %r8
    jbe minlz_encode_block_asm_emit_lit_memmove_mid_match_emit_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_mid_match_emit_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_mid_match_emit_encodeBlockAsm_memmove_move_17through32:
    movdqu (%rdi), %xmm0
    movdqu -16(%rdi,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_block_asm_memmove_mid_end_copy_match_emit_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_mid_match_emit_encodeBlockAsm_memmove_move_33through64:
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_block_asm_memmove_mid_end_copy_match_emit_encodeBlockAsm:
    movq %r9, %rcx
    jmp minlz_encode_block_asm_match_nolits_copy_encodeBlockAsm
minlz_encode_block_asm_memmove_long_match_emit_encodeBlockAsm:
    leaq (%rcx,%r8,1), %r9
    movdqu (%rdi), %xmm0
    movdqu 16(%rdi), %xmm1
    movdqu -32(%rdi,%r8,1), %xmm2
    movdqu -16(%rdi,%r8,1), %xmm3
    movq %r8, %r12
    shrq $0x05, %r12
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r13
    subq %r10, %r13
    decq %r12
    ja minlz_encode_block_asm_emit_lit_memmove_long_match_emit_encodeBlockAsmlarge_forward_sse_loop_32
    leaq -32(%rdi,%r13,1), %r10
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_block_asm_emit_lit_memmove_long_match_emit_encodeBlockAsmlarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r10
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_block_asm_emit_lit_memmove_long_match_emit_encodeBlockAsmlarge_big_loop_back
minlz_encode_block_asm_emit_lit_memmove_long_match_emit_encodeBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%rdi,%r13,1), %xmm4
    movdqu -16(%rdi,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %r8
    jae minlz_encode_block_asm_emit_lit_memmove_long_match_emit_encodeBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
    movq %r9, %rcx
minlz_encode_block_asm_match_nolits_copy_encodeBlockAsm:
    cmpl $0x0001003f, %esi
    jbe minlz_encode_block_asm_two_byte_offset_match_nolit_encodeBlockAsm
    leal -4(%r11), %r11d
    leal -65536(%rsi), %esi
    shll $0x0b, %esi
    addl $0x07, %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_emit_copy3_0_match_nolit_encodeBlockAsm_emit3
    leal -60(%r11), %edi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_emit_copy3_1_match_nolit_encodeBlockAsm_emit3
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_emit_copy3_2_match_nolit_encodeBlockAsm_emit3
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %edi, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy3_2_match_nolit_encodeBlockAsm_emit3:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %di, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy3_1_match_nolit_encodeBlockAsm_emit3:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %dil, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy3_0_match_nolit_encodeBlockAsm_emit3:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_two_byte_offset_match_nolit_encodeBlockAsm:
    cmpl $0x00000400, %esi
    ja minlz_encode_block_asm_two_byte_match_nolit_encodeBlockAsm
    cmpl $0x00000013, %r11d
    jae minlz_encode_block_asm_emit_one_longer_match_nolit_encodeBlockAsm
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal -15(%rsi,%r11,4), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_one_longer_match_nolit_encodeBlockAsm:
    cmpl $0x00000112, %r11d
    jae minlz_encode_block_asm_emit_copy1_repeat_match_nolit_encodeBlockAsm
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 61(%rsi), %esi
    movw %si, (%rcx)
    leal -18(%r11), %esi
    movb %sil, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy1_repeat_match_nolit_encodeBlockAsm:
    leal -1(%rsi), %esi
    shll $0x06, %esi
    leal 57(%rsi), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r11d
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_block_asm_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_block_asm_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm
    cmpl $0x0001001e, %r11d
    jb minlz_encode_block_asm_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBlockAsm:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBlockAsm:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBlockAsm:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_two_byte_match_nolit_encodeBlockAsm:
    leal -64(%rsi), %esi
    leal -4(%r11), %r11d
    movw %si, 1(%rcx)
    cmpl $0x3c, %r11d
    jbe minlz_encode_block_asm_emit_copy2_0_match_nolit_encodeBlockAsm_emit2
    leal -60(%r11), %esi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_block_asm_emit_copy2_1_match_nolit_encodeBlockAsm_emit2
    cmpl $0x0001003c, %r11d
    jb minlz_encode_block_asm_emit_copy2_2_match_nolit_encodeBlockAsm_emit2
    movb $0xfe, (%rcx)
    movl %esi, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy2_2_match_nolit_encodeBlockAsm_emit2:
    movb $0xfa, (%rcx)
    movw %si, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy2_1_match_nolit_encodeBlockAsm_emit2:
    movb $0xf6, (%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm
minlz_encode_block_asm_emit_copy2_0_match_nolit_encodeBlockAsm_emit2:
    movl $0x00000002, %esi
    leal (%rsi,%r11,4), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
minlz_encode_block_asm_match_nolit_emitcopy_end_encodeBlockAsm:
    cmpl 8(%rsp), %edx
    jae minlz_encode_block_asm_emit_remainder_encodeBlockAsm
    movq -2(%rbx,%rdx,1), %rdi
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_match_nolit_dst_ok_encodeBlockAsm
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_match_nolit_dst_ok_encodeBlockAsm:
    movq $0x0000cf1bbcdcbf9b, %rsi
    movq %rdi, %r8
    shrq $0x10, %rdi
    movq %rdi, %r9
    shlq $0x10, %r8
    imulq %rsi, %r8
    shrq $0x32, %r8
    shlq $0x10, %r9
    imulq %rsi, %r9
    shrq $0x32, %r9
    leal -2(%rdx), %r10d
    movl (%rax,%r9,4), %esi
    movl %r10d, (%rax,%r8,4)
    movl %edx, (%rax,%r9,4)
    movl %edx, %r8d
    incl %edx
    leal -2162687(%r8), %r9d
    cmpl %r9d, %esi
    ja minlz_encode_block_asm_match_nolit_len_okencodeBlockAsm
    jmp minlz_encode_block_asm_search_loop_encodeBlockAsm
minlz_encode_block_asm_match_nolit_len_okencodeBlockAsm:
    cmpl %edi, (%rbx,%rsi,1)
    jne minlz_encode_block_asm_search_loop_encodeBlockAsm
    movl %r8d, %edi
    subl %esi, %edi
    movl %edi, 16(%rsp)
    cmpq (%rsp), %rcx
    jb minlz_encode_block_asm_dst_size_check_ok_4
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_dst_size_check_ok_4:
    addl $0x03, %edx
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %edx, %edi
    leaq (%rbx,%rdx,1), %r8
    leaq (%rbx,%rsi,1), %rsi
    xorl %r11d, %r11d
    jmp minlz_encode_block_asm_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm
minlz_encode_block_asm_matchlen_loopback_16_match_nolit2_encodeBlockAsm:
    movq (%r8,%r11,1), %r9
    movq 8(%r8,%r11,1), %r10
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_matchlen_bsf_8_match_nolit2_encodeBlockAsm
    xorq 8(%rsi,%r11,1), %r10
    jne minlz_encode_block_asm_matchlen_bsf_16match_nolit2_encodeBlockAsm
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_block_asm_matchlen_loop_16_entry_match_nolit2_encodeBlockAsm:
    cmpl $0x10, %edi
    jae minlz_encode_block_asm_matchlen_loopback_16_match_nolit2_encodeBlockAsm
    jmp minlz_encode_block_asm_matchlen_match8_match_nolit2_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_16match_nolit2_encodeBlockAsm:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal 8(%r11,%r10,1), %r11d
    jmp minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match8_match_nolit2_encodeBlockAsm:
    cmpl $0x08, %edi
    jb minlz_encode_block_asm_matchlen_match4_match_nolit2_encodeBlockAsm
    movq (%r8,%r11,1), %r9
    xorq (%rsi,%r11,1), %r9
    jne minlz_encode_block_asm_matchlen_bsf_8_match_nolit2_encodeBlockAsm
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_block_asm_matchlen_match4_match_nolit2_encodeBlockAsm
minlz_encode_block_asm_matchlen_bsf_8_match_nolit2_encodeBlockAsm:
    tzcntq %r9, %r9
    sarq $0x03, %r9
    leal (%r11,%r9,1), %r11d
    jmp minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match4_match_nolit2_encodeBlockAsm:
    cmpl $0x04, %edi
    jb minlz_encode_block_asm_matchlen_match2_match_nolit2_encodeBlockAsm
    movl (%r8,%r11,1), %r9d
    cmpl %r9d, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match2_match_nolit2_encodeBlockAsm
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_block_asm_matchlen_match2_match_nolit2_encodeBlockAsm:
    cmpl $0x01, %edi
    je minlz_encode_block_asm_matchlen_match1_match_nolit2_encodeBlockAsm
    jb minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm
    movw (%r8,%r11,1), %r9w
    cmpw %r9w, (%rsi,%r11,1)
    jne minlz_encode_block_asm_matchlen_match1_match_nolit2_encodeBlockAsm
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm
minlz_encode_block_asm_matchlen_match1_match_nolit2_encodeBlockAsm:
    movb (%r8,%r11,1), %r9b
    cmpb %r9b, (%rsi,%r11,1)
    jne minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm
    leal 1(%r11), %r11d
minlz_encode_block_asm_match_nolit2_end_encodeBlockAsm:
    addl %r11d, %edx
    addl $0x04, %r11d
    movl %edx, 12(%rsp)
    movl 16(%rsp), %esi
    jmp minlz_encode_block_asm_match_nolits_copy_encodeBlockAsm
minlz_encode_block_asm_emit_remainder_encodeBlockAsm:
    movq 48(%rsp), %rax
    movl 12(%rsp), %edx
    subl %edx, %eax
    je minlz_encode_block_asm_emit_remainder_end_encodeBlockAsm
    leaq (%rbx,%rdx,1), %rdx
    leaq 4(%rcx,%rax,1), %rbx
    cmpq (%rsp), %rbx
    jb minlz_encode_block_asm_dst_size_check_ok_5
    movq $0x00000000, 64(%rsp)
    jmp Lepi_gen
minlz_encode_block_asm_dst_size_check_ok_5:
    leal -1(%rax), %ebx
    cmpl $0x1d, %ebx
    jb minlz_encode_block_asm_one_byte_emit_remainder_encodeBlockAsm
    subl $0x1d, %ebx
    cmpl $0x00000100, %ebx
    jb minlz_encode_block_asm_two_bytes_emit_remainder_encodeBlockAsm
    cmpl $0x00010000, %ebx
    jb minlz_encode_block_asm_three_bytes_emit_remainder_encodeBlockAsm
    movl %ebx, %esi
    shrl $0x10, %esi
    movb $0xf8, (%rcx)
    movw %bx, 1(%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_memmove_long_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_three_bytes_emit_remainder_encodeBlockAsm:
    movb $0xf0, (%rcx)
    movw %bx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %ebx
    jmp minlz_encode_block_asm_memmove_long_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_two_bytes_emit_remainder_encodeBlockAsm:
    movb $0xe8, (%rcx)
    movb %bl, 1(%rcx)
    addl $0x1d, %ebx
    addq $0x02, %rcx
    cmpl $0x40, %ebx
    jb minlz_encode_block_asm_memmove_midemit_remainder_encodeBlockAsm
    jmp minlz_encode_block_asm_memmove_long_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_one_byte_emit_remainder_encodeBlockAsm:
    shlb $0x03, %bl
    movb %bl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x03, %rax
    jb minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_1or2
    je minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_3
    cmpq $0x08, %rax
    jbe minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_4through8
    cmpq $0x10, %rax
    jbe minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_8through16
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_1or2:
    movb (%rdx), %sil
    movb -1(%rdx,%rax,1), %dl
    movb %sil, (%rcx)
    movb %dl, -1(%rcx,%rax,1)
    jmp minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_3:
    movw (%rdx), %si
    movb 2(%rdx), %dl
    movw %si, (%rcx)
    movb %dl, 2(%rcx)
    jmp minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_4through8:
    movl (%rdx), %esi
    movl -4(%rdx,%rax,1), %edx
    movl %esi, (%rcx)
    movl %edx, -4(%rcx,%rax,1)
    jmp minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_8through16:
    movq (%rdx), %rsi
    movq -8(%rdx,%rax,1), %rdx
    movq %rsi, (%rcx)
    movq %rdx, -8(%rcx,%rax,1)
    jmp minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_emit_remainder_encodeBlockAsm_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_memmove_end_copy_emit_remainder_encodeBlockAsm:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_emit_remainder_end_encodeBlockAsm
minlz_encode_block_asm_memmove_midemit_remainder_encodeBlockAsm:
    leaq (%rcx,%rax,1), %rbx
    cmpq $0x20, %rax
    jbe minlz_encode_block_asm_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm_memmove_move_17through32
    jmp minlz_encode_block_asm_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm_memmove_move_33through64
minlz_encode_block_asm_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm_memmove_move_17through32:
    movdqu (%rdx), %xmm0
    movdqu -16(%rdx,%rax,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rax,1)
    jmp minlz_encode_block_asm_memmove_mid_end_copy_emit_remainder_encodeBlockAsm
minlz_encode_block_asm_emit_lit_memmove_mid_emit_remainder_encodeBlockAsm_memmove_move_33through64:
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
minlz_encode_block_asm_memmove_mid_end_copy_emit_remainder_encodeBlockAsm:
    movq %rbx, %rcx
    jmp minlz_encode_block_asm_emit_remainder_end_encodeBlockAsm
minlz_encode_block_asm_memmove_long_emit_remainder_encodeBlockAsm:
    leaq (%rcx,%rax,1), %rbx
    movdqu (%rdx), %xmm0
    movdqu 16(%rdx), %xmm1
    movdqu -32(%rdx,%rax,1), %xmm2
    movdqu -16(%rdx,%rax,1), %xmm3
    movq %rax, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_block_asm_emit_lit_memmove_long_emit_remainder_encodeBlockAsmlarge_forward_sse_loop_32
    leaq -32(%rdx,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_block_asm_emit_lit_memmove_long_emit_remainder_encodeBlockAsmlarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_block_asm_emit_lit_memmove_long_emit_remainder_encodeBlockAsmlarge_big_loop_back
minlz_encode_block_asm_emit_lit_memmove_long_emit_remainder_encodeBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%rdx,%r8,1), %xmm4
    movdqu -16(%rdx,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rax
    jae minlz_encode_block_asm_emit_lit_memmove_long_emit_remainder_encodeBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rax,1)
    movdqu %xmm3, -16(%rcx,%rax,1)
    movq %rbx, %rcx
minlz_encode_block_asm_emit_remainder_end_encodeBlockAsm:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_gen
Lepi_gen:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
.p2align 4
.globl minlz_encode_better_asm_512k
.hidden minlz_encode_better_asm_512k
minlz_encode_better_asm_512k:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00000900, %rdx
    pxor %xmm0, %xmm0
minlz_encode_better_asm_512k_zero_loop_encodeBetterBlockAsm512K:
    movdqu %xmm0, (%rax)
    movdqu %xmm0, 16(%rax)
    movdqu %xmm0, 32(%rax)
    movdqu %xmm0, 48(%rax)
    movdqu %xmm0, 64(%rax)
    movdqu %xmm0, 80(%rax)
    movdqu %xmm0, 96(%rax)
    movdqu %xmm0, 112(%rax)
    addq $0x80, %rax
    decq %rdx
    jne minlz_encode_better_asm_512k_zero_loop_encodeBetterBlockAsm512K
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rax
    leaq -11(%rax), %rdx
    leaq -8(%rax), %rbx
    movl %ebx, 8(%rsp)
    shrq $0x05, %rax
    subl %eax, %edx
    leaq (%rcx,%rdx,1), %rdx
    movq %rdx, (%rsp)
    movl $0x00000001, %eax
    movl %eax, 16(%rsp)
    movq 40(%rsp), %rdx
minlz_encode_better_asm_512k_search_loop_encodeBetterBlockAsm512K:
    movq 56(%rsp), %rbx
    movl %eax, %esi
    subl 12(%rsp), %esi
    shrl $0x07, %esi
    cmpl $0x63, %esi
    jbe minlz_encode_better_asm_512k_check_maxskip_ok_encodeBetterBlockAsm512K
    leal 100(%rax), %esi
    jmp minlz_encode_better_asm_512k_check_maxskip_cont_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_check_maxskip_ok_encodeBetterBlockAsm512K:
    leal 1(%rax,%rsi,1), %esi
minlz_encode_better_asm_512k_check_maxskip_cont_encodeBetterBlockAsm512K:
    cmpl 8(%rsp), %esi
    jae minlz_encode_better_asm_512k_emit_remainder_encodeBetterBlockAsm512K
    movq (%rdx,%rax,1), %rdi
    movl %esi, 20(%rsp)
    movq $0x00cf1bbcdcbfa563, %r9
    movq $0x9e3779b1, %rsi
    movq %rdi, %r10
    movq %rdi, %r11
    shlq $0x08, %r10
    imulq %r9, %r10
    shrq $0x30, %r10
    shlq $0x20, %r11
    imulq %rsi, %r11
    shrq $0x34, %r11
    movl (%rbx,%r10,4), %esi
    movl 262144(%rbx,%r11,4), %r8d
    movl %eax, (%rbx,%r10,4)
    movl %eax, 262144(%rbx,%r11,4)
    movq (%rdx,%rsi,1), %r10
    cmpq %rdi, %r10
    je minlz_encode_better_asm_512k_candidate_match_encodeBetterBlockAsm512K
    movq (%rdx,%r8,1), %r11
    cmpq %rdi, %r11
    movl %eax, %r12d
    subl 16(%rsp), %r12d
    movq (%rdx,%r12,1), %r12
    movq $0x000000ffffffff00, %r13
    xorq %rdi, %r12
    testq %r13, %r12
    jne minlz_encode_better_asm_512k_no_repeat_found_encodeBetterBlockAsm512K
    leal 1(%rax), %ebx
    movl 12(%rsp), %esi
    movl %ebx, %edi
    subl 16(%rsp), %edi
    je minlz_encode_better_asm_512k_repeat_extend_back_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_extend_back_loop_encodeBetterBlockAsm512K:
    cmpl %esi, %ebx
    jbe minlz_encode_better_asm_512k_repeat_extend_back_end_encodeBetterBlockAsm512K
    movb -1(%rdx,%rdi,1), %r8b
    movb -1(%rdx,%rbx,1), %r9b
    cmpb %r9b, %r8b
    jne minlz_encode_better_asm_512k_repeat_extend_back_end_encodeBetterBlockAsm512K
    leal -1(%rbx), %ebx
    decl %edi
    jne minlz_encode_better_asm_512k_repeat_extend_back_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_extend_back_end_encodeBetterBlockAsm512K:
    movl %ebx, %esi
    subl 12(%rsp), %esi
    leaq 4(%rcx,%rsi,1), %rsi
    cmpq (%rsp), %rsi
    jb minlz_encode_better_asm_512k_repeat_dst_size_check_encodeBetterBlockAsm512K
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b512k
minlz_encode_better_asm_512k_repeat_dst_size_check_encodeBetterBlockAsm512K:
    movl 12(%rsp), %esi
    cmpl %ebx, %esi
    je minlz_encode_better_asm_512k_emit_literal_done_repeat_emit_encodeBetterBlockAsm512K
    movl %ebx, %edi
    movl %ebx, 12(%rsp)
    leaq (%rdx,%rsi,1), %r8
    subl %esi, %edi
    leal -1(%rdi), %esi
    cmpl $0x1d, %esi
    jb minlz_encode_better_asm_512k_one_byte_repeat_emit_encodeBetterBlockAsm512K
    subl $0x1d, %esi
    cmpl $0x00000100, %esi
    jb minlz_encode_better_asm_512k_two_bytes_repeat_emit_encodeBetterBlockAsm512K
    cmpl $0x00010000, %esi
    jb minlz_encode_better_asm_512k_three_bytes_repeat_emit_encodeBetterBlockAsm512K
    movl %esi, %r9d
    shrl $0x10, %r9d
    movb $0xf8, (%rcx)
    movw %si, 1(%rcx)
    movb %r9b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_512k_memmove_long_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_three_bytes_repeat_emit_encodeBetterBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_512k_memmove_long_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_bytes_repeat_emit_encodeBetterBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %sil, 1(%rcx)
    addl $0x1d, %esi
    addq $0x02, %rcx
    cmpl $0x40, %esi
    jb minlz_encode_better_asm_512k_memmove_midrepeat_emit_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_memmove_long_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_one_byte_repeat_emit_encodeBetterBlockAsm512K:
    shlb $0x03, %sil
    movb %sil, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x08, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_8
    cmpq $0x10, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_8:
    movq (%r8), %r9
    movq %r9, (%rcx)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_8through16:
    movq (%r8), %r9
    movq -8(%r8,%rdi,1), %r8
    movq %r9, (%rcx)
    movq %r8, -8(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_512k_memmove_end_copy_repeat_emit_encodeBetterBlockAsm512K:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_midrepeat_emit_encodeBetterBlockAsm512K:
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_512k_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm512K:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_repeat_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_long_repeat_emit_encodeBetterBlockAsm512K:
    leaq (%rcx,%rdi,1), %rsi
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movq %rdi, %r10
    shrq $0x05, %r10
    movq %rcx, %r9
    andl $0x0000001f, %r9d
    movq $0x00000040, %r11
    subq %r9, %r11
    decq %r10
    ja minlz_encode_better_asm_512k_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%r8,%r11,1), %r9
    leaq -32(%rcx,%r11,1), %r12
minlz_encode_better_asm_512k_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm512Klarge_big_loop_back:
    movdqu (%r9), %xmm4
    movdqu 16(%r9), %xmm5
    movdqu %xmm4, (%r12)
    movdqu %xmm5, 16(%r12)
    addq $0x20, %r12
    addq $0x20, %r9
    addq $0x20, %r11
    decq %r10
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm512Klarge_big_loop_back
minlz_encode_better_asm_512k_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%r8,%r11,1), %xmm4
    movdqu -16(%r8,%r11,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r11,1)
    movdqu %xmm5, -16(%rcx,%r11,1)
    addq $0x20, %r11
    cmpq %r11, %rdi
    jae minlz_encode_better_asm_512k_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
    movq %rsi, %rcx
minlz_encode_better_asm_512k_emit_literal_done_repeat_emit_encodeBetterBlockAsm512K:
    addl $0x05, %eax
    movl %eax, %esi
    subl 16(%rsp), %esi
    movq 48(%rsp), %rdi
    subl %eax, %edi
    leaq (%rdx,%rax,1), %r8
    leaq (%rdx,%rsi,1), %rsi
    xorl %r10d, %r10d
    jmp minlz_encode_better_asm_512k_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm512K:
    movq (%r8,%r10,1), %r9
    movq 8(%r8,%r10,1), %r11
    xorq (%rsi,%r10,1), %r9
    jne minlz_encode_better_asm_512k_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm512K
    xorq 8(%rsi,%r10,1), %r11
    jne minlz_encode_better_asm_512k_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm512K
    leal -16(%rdi), %edi
    leal 16(%r10), %r10d
minlz_encode_better_asm_512k_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm512K:
    cmpl $0x10, %edi
    jae minlz_encode_better_asm_512k_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_matchlen_match8_repeat_extend_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm512K:
    tzcntq %r11, %r11
    sarq $0x03, %r11
    leal 8(%r10,%r11,1), %r10d
    jmp minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match8_repeat_extend_encodeBetterBlockAsm512K:
    cmpl $0x08, %edi
    jb minlz_encode_better_asm_512k_matchlen_match4_repeat_extend_encodeBetterBlockAsm512K
    movq (%r8,%r10,1), %r9
    xorq (%rsi,%r10,1), %r9
    jne minlz_encode_better_asm_512k_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm512K
    leal -8(%rdi), %edi
    leal 8(%r10), %r10d
    jmp minlz_encode_better_asm_512k_matchlen_match4_repeat_extend_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm512K:
    tzcntq %r9, %r9
    sarq $0x03, %r9
    leal (%r10,%r9,1), %r10d
    jmp minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match4_repeat_extend_encodeBetterBlockAsm512K:
    cmpl $0x04, %edi
    jb minlz_encode_better_asm_512k_matchlen_match2_repeat_extend_encodeBetterBlockAsm512K
    movl (%r8,%r10,1), %r9d
    cmpl %r9d, (%rsi,%r10,1)
    jne minlz_encode_better_asm_512k_matchlen_match2_repeat_extend_encodeBetterBlockAsm512K
    leal -4(%rdi), %edi
    leal 4(%r10), %r10d
minlz_encode_better_asm_512k_matchlen_match2_repeat_extend_encodeBetterBlockAsm512K:
    cmpl $0x01, %edi
    je minlz_encode_better_asm_512k_matchlen_match1_repeat_extend_encodeBetterBlockAsm512K
    jb minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K
    movw (%r8,%r10,1), %r9w
    cmpw %r9w, (%rsi,%r10,1)
    jne minlz_encode_better_asm_512k_matchlen_match1_repeat_extend_encodeBetterBlockAsm512K
    leal 2(%r10), %r10d
    subl $0x02, %edi
    je minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match1_repeat_extend_encodeBetterBlockAsm512K:
    movb (%r8,%r10,1), %r9b
    cmpb %r9b, (%rsi,%r10,1)
    jne minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K
    leal 1(%r10), %r10d
minlz_encode_better_asm_512k_repeat_extend_forward_end_encodeBetterBlockAsm512K:
    addl %r10d, %eax
    movl %eax, %esi
    subl %ebx, %esi
    movl 16(%rsp), %ebx
    leal -1(%rsi), %ebx
    cmpl $0x1d, %esi
    jbe minlz_encode_better_asm_512k_repeat_one_match_repeat_encodeBetterBlockAsm512K
    leal -30(%rsi), %ebx
    cmpl $0x0000011e, %esi
    jb minlz_encode_better_asm_512k_repeat_two_match_repeat_encodeBetterBlockAsm512K
    cmpl $0x0001001e, %esi
    jb minlz_encode_better_asm_512k_repeat_three_match_repeat_encodeBetterBlockAsm512K
    movb $0xfc, (%rcx)
    movl %ebx, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_repeat_end_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_three_match_repeat_encodeBetterBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %bx, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_repeat_end_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_two_match_repeat_encodeBetterBlockAsm512K:
    movb $0xec, (%rcx)
    movb %bl, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_512k_repeat_end_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_one_match_repeat_encodeBetterBlockAsm512K:
    xorl %ebx, %ebx
    leal -4(%rbx,%rsi,8), %ebx
    movb %bl, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_512k_repeat_end_emit_encodeBetterBlockAsm512K:
    movl %eax, 12(%rsp)
    jmp minlz_encode_better_asm_512k_search_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_no_repeat_found_encodeBetterBlockAsm512K:
    cmpl %edi, %r10d
    je minlz_encode_better_asm_512k_candidate_match_encodeBetterBlockAsm512K
    cmpl %edi, %r11d
    je minlz_encode_better_asm_512k_candidateS_match_encodeBetterBlockAsm512K
    movl 20(%rsp), %eax
    jmp minlz_encode_better_asm_512k_search_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_candidateS_match_encodeBetterBlockAsm512K:
    shrq $0x08, %rdi
    movq %rdi, %r10
    shlq $0x08, %r10
    imulq %r9, %r10
    shrq $0x30, %r10
    movl (%rbx,%r10,4), %esi
    incl %eax
    movl %eax, (%rbx,%r10,4)
    cmpl %edi, (%rdx,%rsi,1)
    je minlz_encode_better_asm_512k_candidate_match_encodeBetterBlockAsm512K
    decl %eax
    movl %r8d, %esi
minlz_encode_better_asm_512k_candidate_match_encodeBetterBlockAsm512K:
    movl 12(%rsp), %ebx
    testl %esi, %esi
    je minlz_encode_better_asm_512k_match_extend_back_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_match_extend_back_loop_encodeBetterBlockAsm512K:
    cmpl %ebx, %eax
    jbe minlz_encode_better_asm_512k_match_extend_back_end_encodeBetterBlockAsm512K
    movb -1(%rdx,%rsi,1), %dil
    movb -1(%rdx,%rax,1), %r8b
    cmpb %r8b, %dil
    jne minlz_encode_better_asm_512k_match_extend_back_end_encodeBetterBlockAsm512K
    leal -1(%rax), %eax
    decl %esi
    je minlz_encode_better_asm_512k_match_extend_back_end_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_match_extend_back_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_match_extend_back_end_encodeBetterBlockAsm512K:
    movl %eax, %ebx
    subl 12(%rsp), %ebx
    leaq 4(%rcx,%rbx,1), %rbx
    cmpq (%rsp), %rbx
    jb minlz_encode_better_asm_512k_match_dst_size_check_encodeBetterBlockAsm512K
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b512k
minlz_encode_better_asm_512k_match_dst_size_check_encodeBetterBlockAsm512K:
    movl %eax, %ebx
    addl $0x04, %eax
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %eax, %edi
    leaq (%rdx,%rax,1), %r8
    leaq (%rdx,%rsi,1), %r9
    xorl %r11d, %r11d
    jmp minlz_encode_better_asm_512k_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm512K:
    movq (%r8,%r11,1), %r10
    movq 8(%r8,%r11,1), %r12
    xorq (%r9,%r11,1), %r10
    jne minlz_encode_better_asm_512k_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm512K
    xorq 8(%r9,%r11,1), %r12
    jne minlz_encode_better_asm_512k_matchlen_bsf_16match_nolit_encodeBetterBlockAsm512K
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_better_asm_512k_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x10, %edi
    jae minlz_encode_better_asm_512k_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_matchlen_match8_match_nolit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_bsf_16match_nolit_encodeBetterBlockAsm512K:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match8_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x08, %edi
    jb minlz_encode_better_asm_512k_matchlen_match4_match_nolit_encodeBetterBlockAsm512K
    movq (%r8,%r11,1), %r10
    xorq (%r9,%r11,1), %r10
    jne minlz_encode_better_asm_512k_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm512K
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_better_asm_512k_matchlen_match4_match_nolit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm512K:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match4_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x04, %edi
    jb minlz_encode_better_asm_512k_matchlen_match2_match_nolit_encodeBetterBlockAsm512K
    movl (%r8,%r11,1), %r10d
    cmpl %r10d, (%r9,%r11,1)
    jne minlz_encode_better_asm_512k_matchlen_match2_match_nolit_encodeBetterBlockAsm512K
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_better_asm_512k_matchlen_match2_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x01, %edi
    je minlz_encode_better_asm_512k_matchlen_match1_match_nolit_encodeBetterBlockAsm512K
    jb minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K
    movw (%r8,%r11,1), %r10w
    cmpw %r10w, (%r9,%r11,1)
    jne minlz_encode_better_asm_512k_matchlen_match1_match_nolit_encodeBetterBlockAsm512K
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_matchlen_match1_match_nolit_encodeBetterBlockAsm512K:
    movb (%r8,%r11,1), %r10b
    cmpb %r10b, (%r9,%r11,1)
    jne minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K
    leal 1(%r11), %r11d
minlz_encode_better_asm_512k_match_nolit_end_encodeBetterBlockAsm512K:
    movl %eax, %edi
    subl %esi, %edi
    cmpl $0x01, %r11d
    ja minlz_encode_better_asm_512k_match_length_ok_encodeBetterBlockAsm512K
    cmpl $0x0001003f, %edi
    jbe minlz_encode_better_asm_512k_match_length_ok_encodeBetterBlockAsm512K
    movl 20(%rsp), %eax
    incl %eax
    jmp minlz_encode_better_asm_512k_search_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_match_length_ok_encodeBetterBlockAsm512K:
    movl %edi, 16(%rsp)
    movl 12(%rsp), %r8d
    movl %ebx, %esi
    subl %r8d, %esi
    je minlz_encode_better_asm_512k_match_emit_nolits_encodeBetterBlockAsm512K
    cmpl $0x00000040, %edi
    jl minlz_encode_better_asm_512k_match_emit_lits_encodeBetterBlockAsm512K
    cmpl $0x0001003f, %edi
    ja minlz_encode_better_asm_512k_match_emit_copy3_encodeBetterBlockAsm512K
    cmpl $0x04, %esi
    ja minlz_encode_better_asm_512k_match_emit_lits_encodeBetterBlockAsm512K
    movl (%rdx,%r8,1), %r8d
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    xorq %r9, %r9
    subl $0x40, %edi
    leal -11(%r11), %r10d
    leal -4(%r11), %r11d
    movw %di, 1(%rcx)
    cmpl $0x07, %r11d
    cmovge %r10d, %r9d
    movq $0x00000007, %rdi
    cmovl %r11d, %edi
    leal -1(%rsi,%rdi,4), %edi
    movl $0x00000003, %r10d
    leal (%r10,%rdi,8), %edi
    movb %dil, (%rcx)
    addq $0x03, %rcx
    movl %r8d, (%rcx)
    addq %rsi, %rcx
    testl %r9d, %r9d
    je minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
    leal -1(%r9), %esi
    cmpl $0x1d, %r9d
    jbe minlz_encode_better_asm_512k_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm512K
    leal -30(%r9), %esi
    cmpl $0x0000011e, %r9d
    jb minlz_encode_better_asm_512k_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm512K
    cmpl $0x0001001e, %r9d
    jb minlz_encode_better_asm_512k_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm512K
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm512K:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm512K:
    xorl %esi, %esi
    leal -4(%rsi,%r9,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_match_emit_copy3_encodeBetterBlockAsm512K:
    cmpl $0x03, %esi
    ja minlz_encode_better_asm_512k_match_emit_lits_encodeBetterBlockAsm512K
    movl 12(%rsp), %r8d
    movl (%rdx,%r8,1), %r8d
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    leal -4(%r11), %r11d
    leal -65536(%rdi), %edi
    shll $0x0b, %edi
    leal 7(%rdi,%rsi,8), %edi
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_512k_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm512K
    leal -60(%r11), %r9d
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm512K
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm512K
    addl $0x000007e0, %edi
    movl %edi, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_512k_match_emit_copy_litsencodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm512K:
    addl $0x000007c0, %edi
    movl %edi, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_512k_match_emit_copy_litsencodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm512K:
    addl $0x000007a0, %edi
    movl %edi, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_512k_match_emit_copy_litsencodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm512K:
    shll $0x05, %r11d
    orl %r11d, %edi
    movl %edi, (%rcx)
    addq $0x04, %rcx
minlz_encode_better_asm_512k_match_emit_copy_litsencodeBetterBlockAsm512K:
    movl %r8d, (%rcx)
    addq %rsi, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_match_emit_lits_encodeBetterBlockAsm512K:
    leaq (%rdx,%r8,1), %r8
    leal -1(%rsi), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_better_asm_512k_one_byte_match_emit_encodeBetterBlockAsm512K
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_better_asm_512k_two_bytes_match_emit_encodeBetterBlockAsm512K
    cmpl $0x00010000, %r9d
    jb minlz_encode_better_asm_512k_three_bytes_match_emit_encodeBetterBlockAsm512K
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_three_bytes_match_emit_encodeBetterBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_bytes_match_emit_encodeBetterBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_better_asm_512k_memmove_midmatch_emit_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_one_byte_match_emit_encodeBetterBlockAsm512K:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x08, %rsi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_8
    cmpq $0x10, %rsi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rsi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_8:
    movq (%r8), %r10
    movq %r10, (%rcx)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_8through16:
    movq (%r8), %r10
    movq -8(%r8,%rsi,1), %r8
    movq %r10, (%rcx)
    movq %r8, -8(%rcx,%rsi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_better_asm_512k_memmove_end_copy_match_emit_encodeBetterBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_better_asm_512k_match_emit_nolits_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_midmatch_emit_encodeBetterBlockAsm512K:
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x20, %rsi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_better_asm_512k_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_better_asm_512k_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm512K:
    movq %r9, %rcx
    jmp minlz_encode_better_asm_512k_match_emit_nolits_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_long_match_emit_encodeBetterBlockAsm512K:
    leaq (%rcx,%rsi,1), %r9
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movq %rsi, %r12
    shrq $0x05, %r12
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r13
    subq %r10, %r13
    decq %r12
    ja minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%r8,%r13,1), %r10
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm512Klarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r10
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm512Klarge_big_loop_back
minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%r8,%r13,1), %xmm4
    movdqu -16(%r8,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %rsi
    jae minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
    movq %r9, %rcx
minlz_encode_better_asm_512k_match_emit_nolits_encodeBetterBlockAsm512K:
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    cmpl $0x0001003f, %edi
    jbe minlz_encode_better_asm_512k_two_byte_offset_match_nolit_encodeBetterBlockAsm512K
    leal -4(%r11), %r11d
    leal -65536(%rdi), %esi
    shll $0x0b, %esi
    addl $0x07, %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_512k_emit_copy3_0_match_nolit_encodeBetterBlockAsm512K_emit3
    leal -60(%r11), %edi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy3_1_match_nolit_encodeBetterBlockAsm512K_emit3
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy3_2_match_nolit_encodeBetterBlockAsm512K_emit3
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %edi, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_2_match_nolit_encodeBetterBlockAsm512K_emit3:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %di, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_1_match_nolit_encodeBetterBlockAsm512K_emit3:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %dil, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy3_0_match_nolit_encodeBetterBlockAsm512K_emit3:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_byte_offset_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x00000400, %edi
    ja minlz_encode_better_asm_512k_two_byte_match_nolit_encodeBetterBlockAsm512K
    cmpl $0x00000013, %r11d
    jae minlz_encode_better_asm_512k_emit_one_longer_match_nolit_encodeBetterBlockAsm512K
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal -15(%rsi,%r11,4), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_one_longer_match_nolit_encodeBetterBlockAsm512K:
    cmpl $0x00000112, %r11d
    jae minlz_encode_better_asm_512k_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm512K
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal 61(%rsi), %esi
    movw %si, (%rcx)
    leal -18(%r11), %esi
    movb %sil, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm512K:
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal 57(%rsi), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r11d
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_better_asm_512k_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_better_asm_512k_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K
    cmpl $0x0001001e, %r11d
    jb minlz_encode_better_asm_512k_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm512K:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_byte_match_nolit_encodeBetterBlockAsm512K:
    leal -64(%rdi), %edi
    leal -4(%r11), %r11d
    movw %di, 1(%rcx)
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_512k_emit_copy2_0_match_nolit_encodeBetterBlockAsm512K_emit2
    leal -60(%r11), %esi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy2_1_match_nolit_encodeBetterBlockAsm512K_emit2
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_512k_emit_copy2_2_match_nolit_encodeBetterBlockAsm512K_emit2
    movb $0xfe, (%rcx)
    movl %esi, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy2_2_match_nolit_encodeBetterBlockAsm512K_emit2:
    movb $0xfa, (%rcx)
    movw %si, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy2_1_match_nolit_encodeBetterBlockAsm512K_emit2:
    movb $0xf6, (%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_copy2_0_match_nolit_encodeBetterBlockAsm512K_emit2:
    movl $0x00000002, %esi
    leal (%rsi,%r11,4), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
    movl 12(%rsp), %esi
    cmpl %ebx, %esi
    je minlz_encode_better_asm_512k_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm512K
    movl %ebx, %edi
    movl %ebx, 12(%rsp)
    leaq (%rdx,%rsi,1), %r8
    subl %esi, %edi
    leal -1(%rdi), %esi
    cmpl $0x1d, %esi
    jb minlz_encode_better_asm_512k_one_byte_match_emit_repeat_encodeBetterBlockAsm512K
    subl $0x1d, %esi
    cmpl $0x00000100, %esi
    jb minlz_encode_better_asm_512k_two_bytes_match_emit_repeat_encodeBetterBlockAsm512K
    cmpl $0x00010000, %esi
    jb minlz_encode_better_asm_512k_three_bytes_match_emit_repeat_encodeBetterBlockAsm512K
    movl %esi, %r9d
    shrl $0x10, %r9d
    movb $0xf8, (%rcx)
    movw %si, 1(%rcx)
    movb %r9b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_three_bytes_match_emit_repeat_encodeBetterBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_bytes_match_emit_repeat_encodeBetterBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %sil, 1(%rcx)
    addl $0x1d, %esi
    addq $0x02, %rcx
    cmpl $0x40, %esi
    jb minlz_encode_better_asm_512k_memmove_midmatch_emit_repeat_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_memmove_long_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_one_byte_match_emit_repeat_encodeBetterBlockAsm512K:
    shlb $0x03, %sil
    movb %sil, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x08, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_8
    cmpq $0x10, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_8:
    movq (%r8), %r9
    movq %r9, (%rcx)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_8through16:
    movq (%r8), %r9
    movq -8(%r8,%rdi,1), %r8
    movq %r9, (%rcx)
    movq %r8, -8(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_512k_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm512K:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_midmatch_emit_repeat_encodeBetterBlockAsm512K:
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_512k_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_512k_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm512K:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_long_match_emit_repeat_encodeBetterBlockAsm512K:
    leaq (%rcx,%rdi,1), %rsi
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movq %rdi, %r10
    shrq $0x05, %r10
    movq %rcx, %r9
    andl $0x0000001f, %r9d
    movq $0x00000040, %r12
    subq %r9, %r12
    decq %r10
    ja minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%r8,%r12,1), %r9
    leaq -32(%rcx,%r12,1), %r13
minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm512Klarge_big_loop_back:
    movdqu (%r9), %xmm4
    movdqu 16(%r9), %xmm5
    movdqu %xmm4, (%r13)
    movdqu %xmm5, 16(%r13)
    addq $0x20, %r13
    addq $0x20, %r9
    addq $0x20, %r12
    decq %r10
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm512Klarge_big_loop_back
minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%r8,%r12,1), %xmm4
    movdqu -16(%r8,%r12,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r12,1)
    movdqu %xmm5, -16(%rcx,%r12,1)
    addq $0x20, %r12
    cmpq %r12, %rdi
    jae minlz_encode_better_asm_512k_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
    movq %rsi, %rcx
minlz_encode_better_asm_512k_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm512K:
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_better_asm_512k_repeat_one_match_nolit_repeat_encodeBetterBlockAsm512K
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_better_asm_512k_repeat_two_match_nolit_repeat_encodeBetterBlockAsm512K
    cmpl $0x0001001e, %r11d
    jb minlz_encode_better_asm_512k_repeat_three_match_nolit_repeat_encodeBetterBlockAsm512K
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_three_match_nolit_repeat_encodeBetterBlockAsm512K:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_two_match_nolit_repeat_encodeBetterBlockAsm512K:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_repeat_one_match_nolit_repeat_encodeBetterBlockAsm512K:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_512k_match_nolit_emitcopy_end_encodeBetterBlockAsm512K:
    cmpl 8(%rsp), %eax
    jae minlz_encode_better_asm_512k_emit_remainder_encodeBetterBlockAsm512K
    cmpq (%rsp), %rcx
    jb minlz_encode_better_asm_512k_match_nolit_dst_ok_encodeBetterBlockAsm512K
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b512k
minlz_encode_better_asm_512k_match_nolit_dst_ok_encodeBetterBlockAsm512K:
    movq 56(%rsp), %rsi
    movq $0x00cf1bbcdcbfa563, %rdi
    movq $0x9e3779b1, %r8
    leaq 1(%rbx), %rbx
    leaq -2(%rax), %r9
    movq (%rdx,%rbx,1), %r10
    movq 1(%rdx,%rbx,1), %r11
    movq (%rdx,%r9,1), %r12
    movq 1(%rdx,%r9,1), %r13
    shlq $0x08, %r10
    imulq %rdi, %r10
    shrq $0x30, %r10
    shlq $0x20, %r11
    imulq %r8, %r11
    shrq $0x34, %r11
    shlq $0x08, %r12
    imulq %rdi, %r12
    shrq $0x30, %r12
    shlq $0x20, %r13
    imulq %r8, %r13
    shrq $0x34, %r13
    leaq 1(%rbx), %r8
    leaq 1(%r9), %r14
    movl %ebx, (%rsi,%r10,4)
    movl %r9d, (%rsi,%r12,4)
    leaq 1(%r9,%rbx,1), %r10
    shrq $0x01, %r10
    addq $0x01, %rbx
    subq $0x01, %r9
    movl %r8d, 262144(%rsi,%r11,4)
    movl %r14d, 262144(%rsi,%r13,4)
minlz_encode_better_asm_512k_index_loop_encodeBetterBlockAsm512K:
    cmpq %r9, %r10
    jae minlz_encode_better_asm_512k_search_loop_encodeBetterBlockAsm512K
    movq (%rdx,%rbx,1), %r8
    movq (%rdx,%r10,1), %r11
    shlq $0x08, %r8
    imulq %rdi, %r8
    shrq $0x30, %r8
    shlq $0x08, %r11
    imulq %rdi, %r11
    shrq $0x30, %r11
    movl %ebx, (%rsi,%r8,4)
    movl %r9d, (%rsi,%r11,4)
    addq $0x02, %rbx
    addq $0x02, %r10
    jmp minlz_encode_better_asm_512k_index_loop_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_remainder_encodeBetterBlockAsm512K:
    movq 48(%rsp), %rax
    subl 12(%rsp), %eax
    leaq 4(%rcx,%rax,1), %rax
    cmpq (%rsp), %rax
    jb minlz_encode_better_asm_512k_emit_remainder_ok_encodeBetterBlockAsm512K
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b512k
minlz_encode_better_asm_512k_emit_remainder_ok_encodeBetterBlockAsm512K:
    movq 48(%rsp), %rax
    movl 12(%rsp), %ebx
    cmpl %eax, %ebx
    je minlz_encode_better_asm_512k_emit_literal_done_emit_remainder_encodeBetterBlockAsm512K
    movl %eax, %esi
    movl %eax, 12(%rsp)
    leaq (%rdx,%rbx,1), %rax
    subl %ebx, %esi
    leal -1(%rsi), %edx
    cmpl $0x1d, %edx
    jb minlz_encode_better_asm_512k_one_byte_emit_remainder_encodeBetterBlockAsm512K
    subl $0x1d, %edx
    cmpl $0x00000100, %edx
    jb minlz_encode_better_asm_512k_two_bytes_emit_remainder_encodeBetterBlockAsm512K
    cmpl $0x00010000, %edx
    jb minlz_encode_better_asm_512k_three_bytes_emit_remainder_encodeBetterBlockAsm512K
    movl %edx, %ebx
    shrl $0x10, %ebx
    movb $0xf8, (%rcx)
    movw %dx, 1(%rcx)
    movb %bl, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_512k_memmove_long_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_three_bytes_emit_remainder_encodeBetterBlockAsm512K:
    movb $0xf0, (%rcx)
    movw %dx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_512k_memmove_long_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_two_bytes_emit_remainder_encodeBetterBlockAsm512K:
    movb $0xe8, (%rcx)
    movb %dl, 1(%rcx)
    addl $0x1d, %edx
    addq $0x02, %rcx
    cmpl $0x40, %edx
    jb minlz_encode_better_asm_512k_memmove_midemit_remainder_encodeBetterBlockAsm512K
    jmp minlz_encode_better_asm_512k_memmove_long_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_one_byte_emit_remainder_encodeBetterBlockAsm512K:
    shlb $0x03, %dl
    movb %dl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x03, %rbx
    jb minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_1or2
    je minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_3
    cmpq $0x08, %rbx
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_4through8
    cmpq $0x10, %rbx
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_8through16
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_1or2:
    movb (%rax), %sil
    movb -1(%rax,%rbx,1), %al
    movb %sil, (%rcx)
    movb %al, -1(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_3:
    movw (%rax), %si
    movb 2(%rax), %al
    movw %si, (%rcx)
    movb %al, 2(%rcx)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_4through8:
    movl (%rax), %esi
    movl -4(%rax,%rbx,1), %eax
    movl %esi, (%rcx)
    movl %eax, -4(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_8through16:
    movq (%rax), %rsi
    movq -8(%rax,%rbx,1), %rax
    movq %rsi, (%rcx)
    movq %rax, -8(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_512k_memmove_end_copy_emit_remainder_encodeBetterBlockAsm512K:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_midemit_remainder_encodeBetterBlockAsm512K:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm512K_memmove_move_17through32
    jmp minlz_encode_better_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm512K_memmove_move_33through64
minlz_encode_better_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm512K_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_512k_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm512K_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_512k_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm512K:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_512k_emit_literal_done_emit_remainder_encodeBetterBlockAsm512K
minlz_encode_better_asm_512k_memmove_long_emit_remainder_encodeBetterBlockAsm512K:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movq %rbx, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_better_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    leaq -32(%rax,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_better_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm512Klarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_better_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm512Klarge_big_loop_back
minlz_encode_better_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm512Klarge_forward_sse_loop_32:
    movdqu -32(%rax,%r8,1), %xmm4
    movdqu -16(%rax,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rbx
    jae minlz_encode_better_asm_512k_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm512Klarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
    movq %rdx, %rcx
minlz_encode_better_asm_512k_emit_literal_done_emit_remainder_encodeBetterBlockAsm512K:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_b512k
Lepi_b512k:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
.p2align 4
.globl minlz_encode_better_asm_2mb
.hidden minlz_encode_better_asm_2mb
minlz_encode_better_asm_2mb:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00001200, %rdx
    pxor %xmm0, %xmm0
minlz_encode_better_asm_2mb_zero_loop_encodeBetterBlockAsm2MB:
    movdqu %xmm0, (%rax)
    movdqu %xmm0, 16(%rax)
    movdqu %xmm0, 32(%rax)
    movdqu %xmm0, 48(%rax)
    movdqu %xmm0, 64(%rax)
    movdqu %xmm0, 80(%rax)
    movdqu %xmm0, 96(%rax)
    movdqu %xmm0, 112(%rax)
    addq $0x80, %rax
    decq %rdx
    jne minlz_encode_better_asm_2mb_zero_loop_encodeBetterBlockAsm2MB
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rax
    leaq -17(%rax), %rdx
    leaq -17(%rax), %rbx
    movl %ebx, 8(%rsp)
    shrq $0x05, %rax
    subl %eax, %edx
    leaq (%rcx,%rdx,1), %rdx
    movq %rdx, (%rsp)
    movl $0x00000001, %eax
    movl %eax, 16(%rsp)
    movq 40(%rsp), %rdx
minlz_encode_better_asm_2mb_search_loop_encodeBetterBlockAsm2MB:
    movq 56(%rsp), %rbx
    movl %eax, %esi
    subl 12(%rsp), %esi
    shrl $0x07, %esi
    cmpl $0x63, %esi
    jbe minlz_encode_better_asm_2mb_check_maxskip_ok_encodeBetterBlockAsm2MB
    leal 100(%rax), %esi
    jmp minlz_encode_better_asm_2mb_check_maxskip_cont_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_check_maxskip_ok_encodeBetterBlockAsm2MB:
    leal 1(%rax,%rsi,1), %esi
minlz_encode_better_asm_2mb_check_maxskip_cont_encodeBetterBlockAsm2MB:
    cmpl 8(%rsp), %esi
    jae minlz_encode_better_asm_2mb_emit_remainder_encodeBetterBlockAsm2MB
    movq (%rdx,%rax,1), %rdi
    movl %esi, 20(%rsp)
    movq $0x00cf1bbcdcbfa563, %r9
    movq $0x9e3779b1, %rsi
    movq %rdi, %r10
    movq %rdi, %r11
    shlq $0x08, %r10
    imulq %r9, %r10
    shrq $0x2f, %r10
    shlq $0x20, %r11
    imulq %rsi, %r11
    shrq $0x33, %r11
    movl (%rbx,%r10,4), %esi
    movl 524288(%rbx,%r11,4), %r8d
    movl %eax, (%rbx,%r10,4)
    movl %eax, 524288(%rbx,%r11,4)
    movq (%rdx,%rsi,1), %r10
    cmpq %rdi, %r10
    je minlz_encode_better_asm_2mb_candidate_match_encodeBetterBlockAsm2MB
    movq (%rdx,%r8,1), %r11
    cmpq %rdi, %r11
    movl %eax, %r12d
    subl 16(%rsp), %r12d
    movq (%rdx,%r12,1), %r12
    movq $0x000000ffffffff00, %r13
    xorq %rdi, %r12
    testq %r13, %r12
    jne minlz_encode_better_asm_2mb_no_repeat_found_encodeBetterBlockAsm2MB
    leal 1(%rax), %ebx
    movl 12(%rsp), %esi
    movl %ebx, %edi
    subl 16(%rsp), %edi
    je minlz_encode_better_asm_2mb_repeat_extend_back_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_extend_back_loop_encodeBetterBlockAsm2MB:
    cmpl %esi, %ebx
    jbe minlz_encode_better_asm_2mb_repeat_extend_back_end_encodeBetterBlockAsm2MB
    movb -1(%rdx,%rdi,1), %r8b
    movb -1(%rdx,%rbx,1), %r9b
    cmpb %r9b, %r8b
    jne minlz_encode_better_asm_2mb_repeat_extend_back_end_encodeBetterBlockAsm2MB
    leal -1(%rbx), %ebx
    decl %edi
    jne minlz_encode_better_asm_2mb_repeat_extend_back_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_extend_back_end_encodeBetterBlockAsm2MB:
    movl %ebx, %esi
    subl 12(%rsp), %esi
    leaq 4(%rcx,%rsi,1), %rsi
    cmpq (%rsp), %rsi
    jb minlz_encode_better_asm_2mb_repeat_dst_size_check_encodeBetterBlockAsm2MB
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b2mb
minlz_encode_better_asm_2mb_repeat_dst_size_check_encodeBetterBlockAsm2MB:
    movl 12(%rsp), %esi
    cmpl %ebx, %esi
    je minlz_encode_better_asm_2mb_emit_literal_done_repeat_emit_encodeBetterBlockAsm2MB
    movl %ebx, %edi
    movl %ebx, 12(%rsp)
    leaq (%rdx,%rsi,1), %r8
    subl %esi, %edi
    leal -1(%rdi), %esi
    cmpl $0x1d, %esi
    jb minlz_encode_better_asm_2mb_one_byte_repeat_emit_encodeBetterBlockAsm2MB
    subl $0x1d, %esi
    cmpl $0x00000100, %esi
    jb minlz_encode_better_asm_2mb_two_bytes_repeat_emit_encodeBetterBlockAsm2MB
    cmpl $0x00010000, %esi
    jb minlz_encode_better_asm_2mb_three_bytes_repeat_emit_encodeBetterBlockAsm2MB
    movl %esi, %r9d
    shrl $0x10, %r9d
    movb $0xf8, (%rcx)
    movw %si, 1(%rcx)
    movb %r9b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_2mb_memmove_long_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_three_bytes_repeat_emit_encodeBetterBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_2mb_memmove_long_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_bytes_repeat_emit_encodeBetterBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %sil, 1(%rcx)
    addl $0x1d, %esi
    addq $0x02, %rcx
    cmpl $0x40, %esi
    jb minlz_encode_better_asm_2mb_memmove_midrepeat_emit_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_memmove_long_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_one_byte_repeat_emit_encodeBetterBlockAsm2MB:
    shlb $0x03, %sil
    movb %sil, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x10, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_2mb_memmove_end_copy_repeat_emit_encodeBetterBlockAsm2MB:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_midrepeat_emit_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_2mb_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_2mb_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm2MB:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_repeat_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_long_repeat_emit_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rdi,1), %rsi
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movq %rdi, %r10
    shrq $0x05, %r10
    movq %rcx, %r9
    andl $0x0000001f, %r9d
    movq $0x00000040, %r11
    subq %r9, %r11
    decq %r10
    ja minlz_encode_better_asm_2mb_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%r8,%r11,1), %r9
    leaq -32(%rcx,%r11,1), %r12
minlz_encode_better_asm_2mb_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm2MBlarge_big_loop_back:
    movdqu (%r9), %xmm4
    movdqu 16(%r9), %xmm5
    movdqu %xmm4, (%r12)
    movdqu %xmm5, 16(%r12)
    addq $0x20, %r12
    addq $0x20, %r9
    addq $0x20, %r11
    decq %r10
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm2MBlarge_big_loop_back
minlz_encode_better_asm_2mb_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%r8,%r11,1), %xmm4
    movdqu -16(%r8,%r11,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r11,1)
    movdqu %xmm5, -16(%rcx,%r11,1)
    addq $0x20, %r11
    cmpq %r11, %rdi
    jae minlz_encode_better_asm_2mb_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
    movq %rsi, %rcx
minlz_encode_better_asm_2mb_emit_literal_done_repeat_emit_encodeBetterBlockAsm2MB:
    addl $0x05, %eax
    movl %eax, %esi
    subl 16(%rsp), %esi
    movq 48(%rsp), %rdi
    subl %eax, %edi
    leaq (%rdx,%rax,1), %r8
    leaq (%rdx,%rsi,1), %rsi
    xorl %r10d, %r10d
    jmp minlz_encode_better_asm_2mb_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm2MB:
    movq (%r8,%r10,1), %r9
    movq 8(%r8,%r10,1), %r11
    xorq (%rsi,%r10,1), %r9
    jne minlz_encode_better_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm2MB
    xorq 8(%rsi,%r10,1), %r11
    jne minlz_encode_better_asm_2mb_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm2MB
    leal -16(%rdi), %edi
    leal 16(%r10), %r10d
minlz_encode_better_asm_2mb_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm2MB:
    cmpl $0x10, %edi
    jae minlz_encode_better_asm_2mb_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_matchlen_match8_repeat_extend_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm2MB:
    tzcntq %r11, %r11
    sarq $0x03, %r11
    leal 8(%r10,%r11,1), %r10d
    jmp minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match8_repeat_extend_encodeBetterBlockAsm2MB:
    cmpl $0x08, %edi
    jb minlz_encode_better_asm_2mb_matchlen_match4_repeat_extend_encodeBetterBlockAsm2MB
    movq (%r8,%r10,1), %r9
    xorq (%rsi,%r10,1), %r9
    jne minlz_encode_better_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm2MB
    leal -8(%rdi), %edi
    leal 8(%r10), %r10d
    jmp minlz_encode_better_asm_2mb_matchlen_match4_repeat_extend_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm2MB:
    tzcntq %r9, %r9
    sarq $0x03, %r9
    leal (%r10,%r9,1), %r10d
    jmp minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match4_repeat_extend_encodeBetterBlockAsm2MB:
    cmpl $0x04, %edi
    jb minlz_encode_better_asm_2mb_matchlen_match2_repeat_extend_encodeBetterBlockAsm2MB
    movl (%r8,%r10,1), %r9d
    cmpl %r9d, (%rsi,%r10,1)
    jne minlz_encode_better_asm_2mb_matchlen_match2_repeat_extend_encodeBetterBlockAsm2MB
    leal -4(%rdi), %edi
    leal 4(%r10), %r10d
minlz_encode_better_asm_2mb_matchlen_match2_repeat_extend_encodeBetterBlockAsm2MB:
    cmpl $0x01, %edi
    je minlz_encode_better_asm_2mb_matchlen_match1_repeat_extend_encodeBetterBlockAsm2MB
    jb minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB
    movw (%r8,%r10,1), %r9w
    cmpw %r9w, (%rsi,%r10,1)
    jne minlz_encode_better_asm_2mb_matchlen_match1_repeat_extend_encodeBetterBlockAsm2MB
    leal 2(%r10), %r10d
    subl $0x02, %edi
    je minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match1_repeat_extend_encodeBetterBlockAsm2MB:
    movb (%r8,%r10,1), %r9b
    cmpb %r9b, (%rsi,%r10,1)
    jne minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB
    leal 1(%r10), %r10d
minlz_encode_better_asm_2mb_repeat_extend_forward_end_encodeBetterBlockAsm2MB:
    addl %r10d, %eax
    movl %eax, %esi
    subl %ebx, %esi
    movl 16(%rsp), %ebx
    leal -1(%rsi), %ebx
    cmpl $0x1d, %esi
    jbe minlz_encode_better_asm_2mb_repeat_one_match_repeat_encodeBetterBlockAsm2MB
    leal -30(%rsi), %ebx
    cmpl $0x0000011e, %esi
    jb minlz_encode_better_asm_2mb_repeat_two_match_repeat_encodeBetterBlockAsm2MB
    cmpl $0x0001001e, %esi
    jb minlz_encode_better_asm_2mb_repeat_three_match_repeat_encodeBetterBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %ebx, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_repeat_end_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_three_match_repeat_encodeBetterBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %bx, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_repeat_end_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_two_match_repeat_encodeBetterBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %bl, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_2mb_repeat_end_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_one_match_repeat_encodeBetterBlockAsm2MB:
    xorl %ebx, %ebx
    leal -4(%rbx,%rsi,8), %ebx
    movb %bl, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_2mb_repeat_end_emit_encodeBetterBlockAsm2MB:
    movl %eax, 12(%rsp)
    jmp minlz_encode_better_asm_2mb_search_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_no_repeat_found_encodeBetterBlockAsm2MB:
    cmpl %edi, %r10d
    je minlz_encode_better_asm_2mb_candidate_match_encodeBetterBlockAsm2MB
    cmpl %edi, %r11d
    je minlz_encode_better_asm_2mb_candidateS_match_encodeBetterBlockAsm2MB
    movl 20(%rsp), %eax
    jmp minlz_encode_better_asm_2mb_search_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_candidateS_match_encodeBetterBlockAsm2MB:
    shrq $0x08, %rdi
    movq %rdi, %r10
    shlq $0x08, %r10
    imulq %r9, %r10
    shrq $0x2f, %r10
    movl (%rbx,%r10,4), %esi
    incl %eax
    movl %eax, (%rbx,%r10,4)
    cmpl %edi, (%rdx,%rsi,1)
    je minlz_encode_better_asm_2mb_candidate_match_encodeBetterBlockAsm2MB
    decl %eax
    movl %r8d, %esi
minlz_encode_better_asm_2mb_candidate_match_encodeBetterBlockAsm2MB:
    movl 12(%rsp), %ebx
    testl %esi, %esi
    je minlz_encode_better_asm_2mb_match_extend_back_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_match_extend_back_loop_encodeBetterBlockAsm2MB:
    cmpl %ebx, %eax
    jbe minlz_encode_better_asm_2mb_match_extend_back_end_encodeBetterBlockAsm2MB
    movb -1(%rdx,%rsi,1), %dil
    movb -1(%rdx,%rax,1), %r8b
    cmpb %r8b, %dil
    jne minlz_encode_better_asm_2mb_match_extend_back_end_encodeBetterBlockAsm2MB
    leal -1(%rax), %eax
    decl %esi
    je minlz_encode_better_asm_2mb_match_extend_back_end_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_match_extend_back_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_match_extend_back_end_encodeBetterBlockAsm2MB:
    movl %eax, %ebx
    subl 12(%rsp), %ebx
    leaq 4(%rcx,%rbx,1), %rbx
    cmpq (%rsp), %rbx
    jb minlz_encode_better_asm_2mb_match_dst_size_check_encodeBetterBlockAsm2MB
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b2mb
minlz_encode_better_asm_2mb_match_dst_size_check_encodeBetterBlockAsm2MB:
    movl %eax, %ebx
    addl $0x04, %eax
    addl $0x04, %esi
    movq 48(%rsp), %rdi
    subl %eax, %edi
    leaq (%rdx,%rax,1), %r8
    leaq (%rdx,%rsi,1), %r9
    xorl %r11d, %r11d
    jmp minlz_encode_better_asm_2mb_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm2MB:
    movq (%r8,%r11,1), %r10
    movq 8(%r8,%r11,1), %r12
    xorq (%r9,%r11,1), %r10
    jne minlz_encode_better_asm_2mb_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm2MB
    xorq 8(%r9,%r11,1), %r12
    jne minlz_encode_better_asm_2mb_matchlen_bsf_16match_nolit_encodeBetterBlockAsm2MB
    leal -16(%rdi), %edi
    leal 16(%r11), %r11d
minlz_encode_better_asm_2mb_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x10, %edi
    jae minlz_encode_better_asm_2mb_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_matchlen_match8_match_nolit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_bsf_16match_nolit_encodeBetterBlockAsm2MB:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal 8(%r11,%r12,1), %r11d
    jmp minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match8_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x08, %edi
    jb minlz_encode_better_asm_2mb_matchlen_match4_match_nolit_encodeBetterBlockAsm2MB
    movq (%r8,%r11,1), %r10
    xorq (%r9,%r11,1), %r10
    jne minlz_encode_better_asm_2mb_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm2MB
    leal -8(%rdi), %edi
    leal 8(%r11), %r11d
    jmp minlz_encode_better_asm_2mb_matchlen_match4_match_nolit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm2MB:
    tzcntq %r10, %r10
    sarq $0x03, %r10
    leal (%r11,%r10,1), %r11d
    jmp minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match4_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x04, %edi
    jb minlz_encode_better_asm_2mb_matchlen_match2_match_nolit_encodeBetterBlockAsm2MB
    movl (%r8,%r11,1), %r10d
    cmpl %r10d, (%r9,%r11,1)
    jne minlz_encode_better_asm_2mb_matchlen_match2_match_nolit_encodeBetterBlockAsm2MB
    leal -4(%rdi), %edi
    leal 4(%r11), %r11d
minlz_encode_better_asm_2mb_matchlen_match2_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x01, %edi
    je minlz_encode_better_asm_2mb_matchlen_match1_match_nolit_encodeBetterBlockAsm2MB
    jb minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB
    movw (%r8,%r11,1), %r10w
    cmpw %r10w, (%r9,%r11,1)
    jne minlz_encode_better_asm_2mb_matchlen_match1_match_nolit_encodeBetterBlockAsm2MB
    leal 2(%r11), %r11d
    subl $0x02, %edi
    je minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_matchlen_match1_match_nolit_encodeBetterBlockAsm2MB:
    movb (%r8,%r11,1), %r10b
    cmpb %r10b, (%r9,%r11,1)
    jne minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB
    leal 1(%r11), %r11d
minlz_encode_better_asm_2mb_match_nolit_end_encodeBetterBlockAsm2MB:
    movl %eax, %edi
    subl %esi, %edi
    cmpl $0x01, %r11d
    ja minlz_encode_better_asm_2mb_match_length_ok_encodeBetterBlockAsm2MB
    cmpl $0x0001003f, %edi
    jbe minlz_encode_better_asm_2mb_match_length_ok_encodeBetterBlockAsm2MB
    movl 20(%rsp), %eax
    incl %eax
    jmp minlz_encode_better_asm_2mb_search_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_match_length_ok_encodeBetterBlockAsm2MB:
    movl %edi, 16(%rsp)
    movl 12(%rsp), %r8d
    movl %ebx, %esi
    subl %r8d, %esi
    je minlz_encode_better_asm_2mb_match_emit_nolits_encodeBetterBlockAsm2MB
    cmpl $0x00000040, %edi
    jl minlz_encode_better_asm_2mb_match_emit_lits_encodeBetterBlockAsm2MB
    cmpl $0x0001003f, %edi
    ja minlz_encode_better_asm_2mb_match_emit_copy3_encodeBetterBlockAsm2MB
    cmpl $0x04, %esi
    ja minlz_encode_better_asm_2mb_match_emit_lits_encodeBetterBlockAsm2MB
    movl (%rdx,%r8,1), %r8d
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    xorq %r9, %r9
    subl $0x40, %edi
    leal -11(%r11), %r10d
    leal -4(%r11), %r11d
    movw %di, 1(%rcx)
    cmpl $0x07, %r11d
    cmovge %r10d, %r9d
    movq $0x00000007, %rdi
    cmovl %r11d, %edi
    leal -1(%rsi,%rdi,4), %edi
    movl $0x00000003, %r10d
    leal (%r10,%rdi,8), %edi
    movb %dil, (%rcx)
    addq $0x03, %rcx
    movl %r8d, (%rcx)
    addq %rsi, %rcx
    testl %r9d, %r9d
    je minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
    leal -1(%r9), %esi
    cmpl $0x1d, %r9d
    jbe minlz_encode_better_asm_2mb_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm2MB
    leal -30(%r9), %esi
    cmpl $0x0000011e, %r9d
    jb minlz_encode_better_asm_2mb_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm2MB
    cmpl $0x0001001e, %r9d
    jb minlz_encode_better_asm_2mb_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm2MB:
    xorl %esi, %esi
    leal -4(%rsi,%r9,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_match_emit_copy3_encodeBetterBlockAsm2MB:
    cmpl $0x03, %esi
    ja minlz_encode_better_asm_2mb_match_emit_lits_encodeBetterBlockAsm2MB
    movl 12(%rsp), %r8d
    movl (%rdx,%r8,1), %r8d
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    leal -4(%r11), %r11d
    leal -65536(%rdi), %edi
    shll $0x0b, %edi
    leal 7(%rdi,%rsi,8), %edi
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_2mb_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm2MB
    leal -60(%r11), %r9d
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm2MB
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm2MB
    addl $0x000007e0, %edi
    movl %edi, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_2mb_match_emit_copy_litsencodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm2MB:
    addl $0x000007c0, %edi
    movl %edi, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_2mb_match_emit_copy_litsencodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm2MB:
    addl $0x000007a0, %edi
    movl %edi, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_2mb_match_emit_copy_litsencodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm2MB:
    shll $0x05, %r11d
    orl %r11d, %edi
    movl %edi, (%rcx)
    addq $0x04, %rcx
minlz_encode_better_asm_2mb_match_emit_copy_litsencodeBetterBlockAsm2MB:
    movl %r8d, (%rcx)
    addq %rsi, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_match_emit_lits_encodeBetterBlockAsm2MB:
    leaq (%rdx,%r8,1), %r8
    leal -1(%rsi), %r9d
    cmpl $0x1d, %r9d
    jb minlz_encode_better_asm_2mb_one_byte_match_emit_encodeBetterBlockAsm2MB
    subl $0x1d, %r9d
    cmpl $0x00000100, %r9d
    jb minlz_encode_better_asm_2mb_two_bytes_match_emit_encodeBetterBlockAsm2MB
    cmpl $0x00010000, %r9d
    jb minlz_encode_better_asm_2mb_three_bytes_match_emit_encodeBetterBlockAsm2MB
    movl %r9d, %r10d
    shrl $0x10, %r10d
    movb $0xf8, (%rcx)
    movw %r9w, 1(%rcx)
    movb %r10b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_three_bytes_match_emit_encodeBetterBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %r9w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r9d
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_bytes_match_emit_encodeBetterBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %r9b, 1(%rcx)
    addl $0x1d, %r9d
    addq $0x02, %rcx
    cmpl $0x40, %r9d
    jb minlz_encode_better_asm_2mb_memmove_midmatch_emit_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_one_byte_match_emit_encodeBetterBlockAsm2MB:
    shlb $0x03, %r9b
    movb %r9b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x10, %rsi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rsi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_encodeBetterBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_better_asm_2mb_match_emit_nolits_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_midmatch_emit_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rsi,1), %r9
    cmpq $0x20, %rsi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rsi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rsi,1)
    jmp minlz_encode_better_asm_2mb_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
minlz_encode_better_asm_2mb_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm2MB:
    movq %r9, %rcx
    jmp minlz_encode_better_asm_2mb_match_emit_nolits_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_long_match_emit_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rsi,1), %r9
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rsi,1), %xmm2
    movdqu -16(%r8,%rsi,1), %xmm3
    movq %rsi, %r12
    shrq $0x05, %r12
    movq %rcx, %r10
    andl $0x0000001f, %r10d
    movq $0x00000040, %r13
    subq %r10, %r13
    decq %r12
    ja minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%r8,%r13,1), %r10
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm2MBlarge_big_loop_back:
    movdqu (%r10), %xmm4
    movdqu 16(%r10), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r10
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm2MBlarge_big_loop_back
minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%r8,%r13,1), %xmm4
    movdqu -16(%r8,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %rsi
    jae minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rsi,1)
    movdqu %xmm3, -16(%rcx,%rsi,1)
    movq %r9, %rcx
minlz_encode_better_asm_2mb_match_emit_nolits_encodeBetterBlockAsm2MB:
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    cmpl $0x0001003f, %edi
    jbe minlz_encode_better_asm_2mb_two_byte_offset_match_nolit_encodeBetterBlockAsm2MB
    leal -4(%r11), %r11d
    leal -65536(%rdi), %esi
    shll $0x0b, %esi
    addl $0x07, %esi
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_2mb_emit_copy3_0_match_nolit_encodeBetterBlockAsm2MB_emit3
    leal -60(%r11), %edi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy3_1_match_nolit_encodeBetterBlockAsm2MB_emit3
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy3_2_match_nolit_encodeBetterBlockAsm2MB_emit3
    addl $0x000007e0, %esi
    movl %esi, (%rcx)
    movl %edi, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_2_match_nolit_encodeBetterBlockAsm2MB_emit3:
    addl $0x000007c0, %esi
    movl %esi, (%rcx)
    movw %di, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_1_match_nolit_encodeBetterBlockAsm2MB_emit3:
    addl $0x000007a0, %esi
    movl %esi, (%rcx)
    movb %dil, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy3_0_match_nolit_encodeBetterBlockAsm2MB_emit3:
    shll $0x05, %r11d
    orl %r11d, %esi
    movl %esi, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_byte_offset_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x00000400, %edi
    ja minlz_encode_better_asm_2mb_two_byte_match_nolit_encodeBetterBlockAsm2MB
    cmpl $0x00000013, %r11d
    jae minlz_encode_better_asm_2mb_emit_one_longer_match_nolit_encodeBetterBlockAsm2MB
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal -15(%rsi,%r11,4), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_one_longer_match_nolit_encodeBetterBlockAsm2MB:
    cmpl $0x00000112, %r11d
    jae minlz_encode_better_asm_2mb_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm2MB
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal 61(%rsi), %esi
    movw %si, (%rcx)
    leal -18(%r11), %esi
    movb %sil, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm2MB:
    leal -1(%rdi), %esi
    shll $0x06, %esi
    leal 57(%rsi), %esi
    movw %si, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r11d
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_better_asm_2mb_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_better_asm_2mb_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB
    cmpl $0x0001001e, %r11d
    jb minlz_encode_better_asm_2mb_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm2MB:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_byte_match_nolit_encodeBetterBlockAsm2MB:
    leal -64(%rdi), %edi
    leal -4(%r11), %r11d
    movw %di, 1(%rcx)
    cmpl $0x3c, %r11d
    jbe minlz_encode_better_asm_2mb_emit_copy2_0_match_nolit_encodeBetterBlockAsm2MB_emit2
    leal -60(%r11), %esi
    cmpl $0x0000013c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy2_1_match_nolit_encodeBetterBlockAsm2MB_emit2
    cmpl $0x0001003c, %r11d
    jb minlz_encode_better_asm_2mb_emit_copy2_2_match_nolit_encodeBetterBlockAsm2MB_emit2
    movb $0xfe, (%rcx)
    movl %esi, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy2_2_match_nolit_encodeBetterBlockAsm2MB_emit2:
    movb $0xfa, (%rcx)
    movw %si, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy2_1_match_nolit_encodeBetterBlockAsm2MB_emit2:
    movb $0xf6, (%rcx)
    movb %sil, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_copy2_0_match_nolit_encodeBetterBlockAsm2MB_emit2:
    movl $0x00000002, %esi
    leal (%rsi,%r11,4), %esi
    movb %sil, (%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
    movl 12(%rsp), %esi
    cmpl %ebx, %esi
    je minlz_encode_better_asm_2mb_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm2MB
    movl %ebx, %edi
    movl %ebx, 12(%rsp)
    leaq (%rdx,%rsi,1), %r8
    subl %esi, %edi
    leal -1(%rdi), %esi
    cmpl $0x1d, %esi
    jb minlz_encode_better_asm_2mb_one_byte_match_emit_repeat_encodeBetterBlockAsm2MB
    subl $0x1d, %esi
    cmpl $0x00000100, %esi
    jb minlz_encode_better_asm_2mb_two_bytes_match_emit_repeat_encodeBetterBlockAsm2MB
    cmpl $0x00010000, %esi
    jb minlz_encode_better_asm_2mb_three_bytes_match_emit_repeat_encodeBetterBlockAsm2MB
    movl %esi, %r9d
    shrl $0x10, %r9d
    movb $0xf8, (%rcx)
    movw %si, 1(%rcx)
    movb %r9b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_three_bytes_match_emit_repeat_encodeBetterBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %esi
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_bytes_match_emit_repeat_encodeBetterBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %sil, 1(%rcx)
    addl $0x1d, %esi
    addq $0x02, %rcx
    cmpl $0x40, %esi
    jb minlz_encode_better_asm_2mb_memmove_midmatch_emit_repeat_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_one_byte_match_emit_repeat_encodeBetterBlockAsm2MB:
    shlb $0x03, %sil
    movb %sil, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x10, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_8through16:
    movdqu (%r8), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_2mb_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm2MB:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_midmatch_emit_repeat_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rdi,1), %rsi
    cmpq $0x20, %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%r8), %xmm0
    movdqu -16(%r8,%rdi,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rdi,1)
    jmp minlz_encode_better_asm_2mb_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
minlz_encode_better_asm_2mb_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm2MB:
    movq %rsi, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rdi,1), %rsi
    movdqu (%r8), %xmm0
    movdqu 16(%r8), %xmm1
    movdqu -32(%r8,%rdi,1), %xmm2
    movdqu -16(%r8,%rdi,1), %xmm3
    movq %rdi, %r10
    shrq $0x05, %r10
    movq %rcx, %r9
    andl $0x0000001f, %r9d
    movq $0x00000040, %r12
    subq %r9, %r12
    decq %r10
    ja minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%r8,%r12,1), %r9
    leaq -32(%rcx,%r12,1), %r13
minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MBlarge_big_loop_back:
    movdqu (%r9), %xmm4
    movdqu 16(%r9), %xmm5
    movdqu %xmm4, (%r13)
    movdqu %xmm5, 16(%r13)
    addq $0x20, %r13
    addq $0x20, %r9
    addq $0x20, %r12
    decq %r10
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MBlarge_big_loop_back
minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%r8,%r12,1), %xmm4
    movdqu -16(%r8,%r12,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r12,1)
    movdqu %xmm5, -16(%rcx,%r12,1)
    addq $0x20, %r12
    cmpq %r12, %rdi
    jae minlz_encode_better_asm_2mb_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rdi,1)
    movdqu %xmm3, -16(%rcx,%rdi,1)
    movq %rsi, %rcx
minlz_encode_better_asm_2mb_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm2MB:
    addl %r11d, %eax
    addl $0x04, %r11d
    movl %eax, 12(%rsp)
    leal -1(%r11), %esi
    cmpl $0x1d, %r11d
    jbe minlz_encode_better_asm_2mb_repeat_one_match_nolit_repeat_encodeBetterBlockAsm2MB
    leal -30(%r11), %esi
    cmpl $0x0000011e, %r11d
    jb minlz_encode_better_asm_2mb_repeat_two_match_nolit_repeat_encodeBetterBlockAsm2MB
    cmpl $0x0001001e, %r11d
    jb minlz_encode_better_asm_2mb_repeat_three_match_nolit_repeat_encodeBetterBlockAsm2MB
    movb $0xfc, (%rcx)
    movl %esi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_three_match_nolit_repeat_encodeBetterBlockAsm2MB:
    movb $0xf4, (%rcx)
    movw %si, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_two_match_nolit_repeat_encodeBetterBlockAsm2MB:
    movb $0xec, (%rcx)
    movb %sil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_repeat_one_match_nolit_repeat_encodeBetterBlockAsm2MB:
    xorl %esi, %esi
    leal -4(%rsi,%r11,8), %esi
    movb %sil, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_2mb_match_nolit_emitcopy_end_encodeBetterBlockAsm2MB:
    cmpl 8(%rsp), %eax
    jae minlz_encode_better_asm_2mb_emit_remainder_encodeBetterBlockAsm2MB
    cmpq (%rsp), %rcx
    jb minlz_encode_better_asm_2mb_match_nolit_dst_ok_encodeBetterBlockAsm2MB
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b2mb
minlz_encode_better_asm_2mb_match_nolit_dst_ok_encodeBetterBlockAsm2MB:
    movq 56(%rsp), %rsi
    movq $0x00cf1bbcdcbfa563, %rdi
    movq $0x9e3779b1, %r8
    leaq 1(%rbx), %rbx
    leaq -2(%rax), %r9
    movq (%rdx,%rbx,1), %r10
    movq 1(%rdx,%rbx,1), %r11
    movq (%rdx,%r9,1), %r12
    movq 1(%rdx,%r9,1), %r13
    shlq $0x08, %r10
    imulq %rdi, %r10
    shrq $0x2f, %r10
    shlq $0x20, %r11
    imulq %r8, %r11
    shrq $0x33, %r11
    shlq $0x08, %r12
    imulq %rdi, %r12
    shrq $0x2f, %r12
    shlq $0x20, %r13
    imulq %r8, %r13
    shrq $0x33, %r13
    leaq 1(%rbx), %r8
    leaq 1(%r9), %r14
    movl %ebx, (%rsi,%r10,4)
    movl %r9d, (%rsi,%r12,4)
    leaq 1(%r9,%rbx,1), %r10
    shrq $0x01, %r10
    addq $0x01, %rbx
    subq $0x01, %r9
    movl %r8d, 524288(%rsi,%r11,4)
    movl %r14d, 524288(%rsi,%r13,4)
minlz_encode_better_asm_2mb_index_loop_encodeBetterBlockAsm2MB:
    cmpq %r9, %r10
    jae minlz_encode_better_asm_2mb_search_loop_encodeBetterBlockAsm2MB
    movq (%rdx,%rbx,1), %r8
    movq (%rdx,%r10,1), %r11
    shlq $0x08, %r8
    imulq %rdi, %r8
    shrq $0x2f, %r8
    shlq $0x08, %r11
    imulq %rdi, %r11
    shrq $0x2f, %r11
    movl %ebx, (%rsi,%r8,4)
    movl %r9d, (%rsi,%r11,4)
    addq $0x02, %rbx
    addq $0x02, %r10
    jmp minlz_encode_better_asm_2mb_index_loop_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_remainder_encodeBetterBlockAsm2MB:
    movq 48(%rsp), %rax
    subl 12(%rsp), %eax
    leaq 4(%rcx,%rax,1), %rax
    cmpq (%rsp), %rax
    jb minlz_encode_better_asm_2mb_emit_remainder_ok_encodeBetterBlockAsm2MB
    movq $0x00000000, 64(%rsp)
    jmp Lepi_b2mb
minlz_encode_better_asm_2mb_emit_remainder_ok_encodeBetterBlockAsm2MB:
    movq 48(%rsp), %rax
    movl 12(%rsp), %ebx
    cmpl %eax, %ebx
    je minlz_encode_better_asm_2mb_emit_literal_done_emit_remainder_encodeBetterBlockAsm2MB
    movl %eax, %esi
    movl %eax, 12(%rsp)
    leaq (%rdx,%rbx,1), %rax
    subl %ebx, %esi
    leal -1(%rsi), %edx
    cmpl $0x1d, %edx
    jb minlz_encode_better_asm_2mb_one_byte_emit_remainder_encodeBetterBlockAsm2MB
    subl $0x1d, %edx
    cmpl $0x00000100, %edx
    jb minlz_encode_better_asm_2mb_two_bytes_emit_remainder_encodeBetterBlockAsm2MB
    cmpl $0x00010000, %edx
    jb minlz_encode_better_asm_2mb_three_bytes_emit_remainder_encodeBetterBlockAsm2MB
    movl %edx, %ebx
    shrl $0x10, %ebx
    movb $0xf8, (%rcx)
    movw %dx, 1(%rcx)
    movb %bl, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_2mb_memmove_long_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_three_bytes_emit_remainder_encodeBetterBlockAsm2MB:
    movb $0xf0, (%rcx)
    movw %dx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_2mb_memmove_long_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_two_bytes_emit_remainder_encodeBetterBlockAsm2MB:
    movb $0xe8, (%rcx)
    movb %dl, 1(%rcx)
    addl $0x1d, %edx
    addq $0x02, %rcx
    cmpl $0x40, %edx
    jb minlz_encode_better_asm_2mb_memmove_midemit_remainder_encodeBetterBlockAsm2MB
    jmp minlz_encode_better_asm_2mb_memmove_long_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_one_byte_emit_remainder_encodeBetterBlockAsm2MB:
    shlb $0x03, %dl
    movb %dl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x03, %rbx
    jb minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_1or2
    je minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_3
    cmpq $0x08, %rbx
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_4through8
    cmpq $0x10, %rbx
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_8through16
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_1or2:
    movb (%rax), %sil
    movb -1(%rax,%rbx,1), %al
    movb %sil, (%rcx)
    movb %al, -1(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_3:
    movw (%rax), %si
    movb 2(%rax), %al
    movw %si, (%rcx)
    movb %al, 2(%rcx)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_4through8:
    movl (%rax), %esi
    movl -4(%rax,%rbx,1), %eax
    movl %esi, (%rcx)
    movl %eax, -4(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_8through16:
    movq (%rax), %rsi
    movq -8(%rax,%rbx,1), %rax
    movq %rsi, (%rcx)
    movq %rax, -8(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_2mb_memmove_end_copy_emit_remainder_encodeBetterBlockAsm2MB:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_midemit_remainder_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_17through32
    jmp minlz_encode_better_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_33through64
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_2mb_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm2MB_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_2mb_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm2MB:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_2mb_emit_literal_done_emit_remainder_encodeBetterBlockAsm2MB
minlz_encode_better_asm_2mb_memmove_long_emit_remainder_encodeBetterBlockAsm2MB:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movq %rbx, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_better_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    leaq -32(%rax,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_better_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm2MBlarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_better_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm2MBlarge_big_loop_back
minlz_encode_better_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32:
    movdqu -32(%rax,%r8,1), %xmm4
    movdqu -16(%rax,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rbx
    jae minlz_encode_better_asm_2mb_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsm2MBlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
    movq %rdx, %rcx
minlz_encode_better_asm_2mb_emit_literal_done_emit_remainder_encodeBetterBlockAsm2MB:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_b2mb
Lepi_b2mb:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
.p2align 4
.globl minlz_encode_better_asm
.hidden minlz_encode_better_asm
minlz_encode_better_asm:
    push %rbx
    push %r12
    push %r13
    push %r14
    sub $72, %rsp
    movq $0, 64(%rsp)
    movq %rdi, 32(%rsp)
    movq %rsi, 40(%rsp)
    movq %rdx, 48(%rsp)
    movq %rcx, 56(%rsp)
    movq 56(%rsp), %rax
    movq 32(%rsp), %rcx
    movq $0x00001200, %rdx
    pxor %xmm0, %xmm0
minlz_encode_better_asm_zero_loop_encodeBetterBlockAsm:
    movdqu %xmm0, (%rax)
    movdqu %xmm0, 16(%rax)
    movdqu %xmm0, 32(%rax)
    movdqu %xmm0, 48(%rax)
    movdqu %xmm0, 64(%rax)
    movdqu %xmm0, 80(%rax)
    movdqu %xmm0, 96(%rax)
    movdqu %xmm0, 112(%rax)
    addq $0x80, %rax
    decq %rdx
    jne minlz_encode_better_asm_zero_loop_encodeBetterBlockAsm
    movl $0x00000000, 12(%rsp)
    movq 48(%rsp), %rax
    leaq -17(%rax), %rdx
    leaq -17(%rax), %rdi
    movl %edi, 8(%rsp)
    shrq $0x05, %rax
    subl %eax, %edx
    leaq (%rcx,%rdx,1), %rdx
    movq %rdx, (%rsp)
    movl $0x00000001, %eax
    movl %eax, 16(%rsp)
    movq 40(%rsp), %rdx
minlz_encode_better_asm_search_loop_encodeBetterBlockAsm:
    movq 56(%rsp), %rdi
    movl %eax, %r8d
    subl 12(%rsp), %r8d
    shrl $0x08, %r8d
    cmpl $0x63, %r8d
    jbe minlz_encode_better_asm_check_maxskip_ok_encodeBetterBlockAsm
    leal 100(%rax), %r8d
    jmp minlz_encode_better_asm_check_maxskip_cont_encodeBetterBlockAsm
minlz_encode_better_asm_check_maxskip_ok_encodeBetterBlockAsm:
    leal 1(%rax,%r8,1), %r8d
minlz_encode_better_asm_check_maxskip_cont_encodeBetterBlockAsm:
    cmpl 8(%rsp), %r8d
    jae minlz_encode_better_asm_emit_remainder_encodeBetterBlockAsm
    movq (%rdx,%rax,1), %r9
    movl %r8d, 20(%rsp)
    movq $0x00cf1bbcdcbfa563, %r11
    movq $0x9e3779b1, %r8
    movq %r9, %r12
    movq %r9, %r13
    shlq $0x08, %r12
    imulq %r11, %r12
    shrq $0x2f, %r12
    shlq $0x20, %r13
    imulq %r8, %r13
    shrq $0x33, %r13
    movl (%rdi,%r12,4), %r8d
    movl 524288(%rdi,%r13,4), %r10d
    movl %eax, (%rdi,%r12,4)
    movl %eax, 524288(%rdi,%r13,4)
    leal -2162685(%rax), %r12d
    cmpl %r12d, %r8d
    jle minlz_encode_better_asm_offset_ok_0_encodeBetterBlockAsm
    movq (%rdx,%r8,1), %rbx
    cmpq %r9, %rbx
    je minlz_encode_better_asm_candidate_match_encodeBetterBlockAsm
minlz_encode_better_asm_offset_ok_0_encodeBetterBlockAsm:
    cmpl %r12d, %r10d
    jle minlz_encode_better_asm_offset_ok_1_encodeBetterBlockAsm
    movq (%rdx,%r10,1), %rsi
    cmpq %r9, %rsi
minlz_encode_better_asm_offset_ok_1_encodeBetterBlockAsm:
    movl %eax, %r13d
    subl 16(%rsp), %r13d
    movq (%rdx,%r13,1), %r13
    movq $0x000000ffffffff00, %r14
    xorq %r9, %r13
    testq %r14, %r13
    jne minlz_encode_better_asm_no_repeat_found_encodeBetterBlockAsm
    leal 1(%rax), %edi
    movl 12(%rsp), %r8d
    movl %edi, %r9d
    subl 16(%rsp), %r9d
    je minlz_encode_better_asm_repeat_extend_back_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_extend_back_loop_encodeBetterBlockAsm:
    cmpl %r8d, %edi
    jbe minlz_encode_better_asm_repeat_extend_back_end_encodeBetterBlockAsm
    movb -1(%rdx,%r9,1), %r10b
    movb -1(%rdx,%rdi,1), %r11b
    cmpb %r11b, %r10b
    jne minlz_encode_better_asm_repeat_extend_back_end_encodeBetterBlockAsm
    leal -1(%rdi), %edi
    decl %r9d
    jne minlz_encode_better_asm_repeat_extend_back_loop_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_extend_back_end_encodeBetterBlockAsm:
    movl %edi, %r8d
    subl 12(%rsp), %r8d
    leaq 4(%rcx,%r8,1), %r8
    cmpq (%rsp), %r8
    jb minlz_encode_better_asm_repeat_dst_size_check_encodeBetterBlockAsm
    movq $0x00000000, 64(%rsp)
    jmp Lepi_bgen
minlz_encode_better_asm_repeat_dst_size_check_encodeBetterBlockAsm:
    movl 12(%rsp), %r8d
    cmpl %edi, %r8d
    je minlz_encode_better_asm_emit_literal_done_repeat_emit_encodeBetterBlockAsm
    movl %edi, %r9d
    movl %edi, 12(%rsp)
    leaq (%rdx,%r8,1), %r10
    subl %r8d, %r9d
    leal -1(%r9), %r8d
    cmpl $0x1d, %r8d
    jb minlz_encode_better_asm_one_byte_repeat_emit_encodeBetterBlockAsm
    subl $0x1d, %r8d
    cmpl $0x00000100, %r8d
    jb minlz_encode_better_asm_two_bytes_repeat_emit_encodeBetterBlockAsm
    cmpl $0x00010000, %r8d
    jb minlz_encode_better_asm_three_bytes_repeat_emit_encodeBetterBlockAsm
    movl %r8d, %r11d
    shrl $0x10, %r11d
    movb $0xf8, (%rcx)
    movw %r8w, 1(%rcx)
    movb %r11b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r8d
    jmp minlz_encode_better_asm_memmove_long_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_three_bytes_repeat_emit_encodeBetterBlockAsm:
    movb $0xf0, (%rcx)
    movw %r8w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r8d
    jmp minlz_encode_better_asm_memmove_long_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_two_bytes_repeat_emit_encodeBetterBlockAsm:
    movb $0xe8, (%rcx)
    movb %r8b, 1(%rcx)
    addl $0x1d, %r8d
    addq $0x02, %rcx
    cmpl $0x40, %r8d
    jb minlz_encode_better_asm_memmove_midrepeat_emit_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_memmove_long_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_one_byte_repeat_emit_encodeBetterBlockAsm:
    shlb $0x03, %r8b
    movb %r8b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r9,1), %r8
    cmpq $0x10, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_8through16
    cmpq $0x20, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_8through16:
    movdqu (%r10), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_memmove_end_copy_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r9,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r9,1)
    jmp minlz_encode_better_asm_memmove_end_copy_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_repeat_emit_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
minlz_encode_better_asm_memmove_end_copy_repeat_emit_encodeBetterBlockAsm:
    movq %r8, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_midrepeat_emit_encodeBetterBlockAsm:
    leaq (%rcx,%r9,1), %r8
    cmpq $0x20, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r9,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r9,1)
    jmp minlz_encode_better_asm_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_mid_repeat_emit_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
minlz_encode_better_asm_memmove_mid_end_copy_repeat_emit_encodeBetterBlockAsm:
    movq %r8, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_repeat_emit_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_long_repeat_emit_encodeBetterBlockAsm:
    leaq (%rcx,%r9,1), %r8
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movq %r9, %r12
    shrq $0x05, %r12
    movq %rcx, %r11
    andl $0x0000001f, %r11d
    movq $0x00000040, %r13
    subq %r11, %r13
    decq %r12
    ja minlz_encode_better_asm_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32
    leaq -32(%r10,%r13,1), %r11
    leaq -32(%rcx,%r13,1), %r14
minlz_encode_better_asm_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsmlarge_big_loop_back:
    movdqu (%r11), %xmm4
    movdqu 16(%r11), %xmm5
    movdqu %xmm4, (%r14)
    movdqu %xmm5, 16(%r14)
    addq $0x20, %r14
    addq $0x20, %r11
    addq $0x20, %r13
    decq %r12
    jbe minlz_encode_better_asm_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsmlarge_big_loop_back
minlz_encode_better_asm_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%r10,%r13,1), %xmm4
    movdqu -16(%r10,%r13,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r13,1)
    movdqu %xmm5, -16(%rcx,%r13,1)
    addq $0x20, %r13
    cmpq %r13, %r9
    jae minlz_encode_better_asm_emit_lit_memmove_long_repeat_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
    movq %r8, %rcx
minlz_encode_better_asm_emit_literal_done_repeat_emit_encodeBetterBlockAsm:
    addl $0x05, %eax
    movl %eax, %r8d
    subl 16(%rsp), %r8d
    movq 48(%rsp), %r9
    subl %eax, %r9d
    leaq (%rdx,%rax,1), %r10
    leaq (%rdx,%r8,1), %r8
    xorl %r12d, %r12d
    jmp minlz_encode_better_asm_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm:
    movq (%r10,%r12,1), %r11
    movq 8(%r10,%r12,1), %r13
    xorq (%r8,%r12,1), %r11
    jne minlz_encode_better_asm_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm
    xorq 8(%r8,%r12,1), %r13
    jne minlz_encode_better_asm_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm
    leal -16(%r9), %r9d
    leal 16(%r12), %r12d
minlz_encode_better_asm_matchlen_loop_16_entry_repeat_extend_encodeBetterBlockAsm:
    cmpl $0x10, %r9d
    jae minlz_encode_better_asm_matchlen_loopback_16_repeat_extend_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_matchlen_match8_repeat_extend_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_bsf_16repeat_extend_encodeBetterBlockAsm:
    tzcntq %r13, %r13
    sarq $0x03, %r13
    leal 8(%r12,%r13,1), %r12d
    jmp minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match8_repeat_extend_encodeBetterBlockAsm:
    cmpl $0x08, %r9d
    jb minlz_encode_better_asm_matchlen_match4_repeat_extend_encodeBetterBlockAsm
    movq (%r10,%r12,1), %r11
    xorq (%r8,%r12,1), %r11
    jne minlz_encode_better_asm_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm
    leal -8(%r9), %r9d
    leal 8(%r12), %r12d
    jmp minlz_encode_better_asm_matchlen_match4_repeat_extend_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_bsf_8_repeat_extend_encodeBetterBlockAsm:
    tzcntq %r11, %r11
    sarq $0x03, %r11
    leal (%r12,%r11,1), %r12d
    jmp minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match4_repeat_extend_encodeBetterBlockAsm:
    cmpl $0x04, %r9d
    jb minlz_encode_better_asm_matchlen_match2_repeat_extend_encodeBetterBlockAsm
    movl (%r10,%r12,1), %r11d
    cmpl %r11d, (%r8,%r12,1)
    jne minlz_encode_better_asm_matchlen_match2_repeat_extend_encodeBetterBlockAsm
    leal -4(%r9), %r9d
    leal 4(%r12), %r12d
minlz_encode_better_asm_matchlen_match2_repeat_extend_encodeBetterBlockAsm:
    cmpl $0x01, %r9d
    je minlz_encode_better_asm_matchlen_match1_repeat_extend_encodeBetterBlockAsm
    jb minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm
    movw (%r10,%r12,1), %r11w
    cmpw %r11w, (%r8,%r12,1)
    jne minlz_encode_better_asm_matchlen_match1_repeat_extend_encodeBetterBlockAsm
    leal 2(%r12), %r12d
    subl $0x02, %r9d
    je minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match1_repeat_extend_encodeBetterBlockAsm:
    movb (%r10,%r12,1), %r11b
    cmpb %r11b, (%r8,%r12,1)
    jne minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm
    leal 1(%r12), %r12d
minlz_encode_better_asm_repeat_extend_forward_end_encodeBetterBlockAsm:
    addl %r12d, %eax
    movl %eax, %r8d
    subl %edi, %r8d
    movl 16(%rsp), %edi
    leal -1(%r8), %edi
    cmpl $0x1d, %r8d
    jbe minlz_encode_better_asm_repeat_one_match_repeat_encodeBetterBlockAsm
    leal -30(%r8), %edi
    cmpl $0x0000011e, %r8d
    jb minlz_encode_better_asm_repeat_two_match_repeat_encodeBetterBlockAsm
    cmpl $0x0001001e, %r8d
    jb minlz_encode_better_asm_repeat_three_match_repeat_encodeBetterBlockAsm
    movb $0xfc, (%rcx)
    movl %edi, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_repeat_end_emit_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_three_match_repeat_encodeBetterBlockAsm:
    movb $0xf4, (%rcx)
    movw %di, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_repeat_end_emit_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_two_match_repeat_encodeBetterBlockAsm:
    movb $0xec, (%rcx)
    movb %dil, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_repeat_end_emit_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_one_match_repeat_encodeBetterBlockAsm:
    xorl %edi, %edi
    leal -4(%rdi,%r8,8), %edi
    movb %dil, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_repeat_end_emit_encodeBetterBlockAsm:
    movl %eax, 12(%rsp)
    jmp minlz_encode_better_asm_search_loop_encodeBetterBlockAsm
minlz_encode_better_asm_no_repeat_found_encodeBetterBlockAsm:
    cmpl %r12d, %r8d
    jle minlz_encode_better_asm_offset_ok_2_encodeBetterBlockAsm
    cmpl %r9d, %ebx
    je minlz_encode_better_asm_candidate_match_encodeBetterBlockAsm
minlz_encode_better_asm_offset_ok_2_encodeBetterBlockAsm:
    cmpl %r12d, %r10d
    jle minlz_encode_better_asm_offset_ok_3_encodeBetterBlockAsm
    cmpl %r9d, %esi
    je minlz_encode_better_asm_candidateS_match_encodeBetterBlockAsm
minlz_encode_better_asm_offset_ok_3_encodeBetterBlockAsm:
    movl 20(%rsp), %eax
    jmp minlz_encode_better_asm_search_loop_encodeBetterBlockAsm
minlz_encode_better_asm_candidateS_match_encodeBetterBlockAsm:
    shrq $0x08, %r9
    movq %r9, %r13
    shlq $0x08, %r13
    imulq %r11, %r13
    shrq $0x2f, %r13
    movl (%rdi,%r13,4), %r8d
    incl %eax
    movl %eax, (%rdi,%r13,4)
    cmpl %r12d, %r8d
    jle minlz_encode_better_asm_offset_ok_4_encodeBetterBlockAsm
    cmpl %r9d, (%rdx,%r8,1)
    je minlz_encode_better_asm_candidate_match_encodeBetterBlockAsm
minlz_encode_better_asm_offset_ok_4_encodeBetterBlockAsm:
    decl %eax
    movl %r10d, %r8d
minlz_encode_better_asm_candidate_match_encodeBetterBlockAsm:
    movl 12(%rsp), %edi
    testl %r8d, %r8d
    je minlz_encode_better_asm_match_extend_back_end_encodeBetterBlockAsm
minlz_encode_better_asm_match_extend_back_loop_encodeBetterBlockAsm:
    cmpl %edi, %eax
    jbe minlz_encode_better_asm_match_extend_back_end_encodeBetterBlockAsm
    movb -1(%rdx,%r8,1), %r9b
    movb -1(%rdx,%rax,1), %r10b
    cmpb %r10b, %r9b
    jne minlz_encode_better_asm_match_extend_back_end_encodeBetterBlockAsm
    leal -1(%rax), %eax
    decl %r8d
    je minlz_encode_better_asm_match_extend_back_end_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_match_extend_back_loop_encodeBetterBlockAsm
minlz_encode_better_asm_match_extend_back_end_encodeBetterBlockAsm:
    movl %eax, %edi
    subl 12(%rsp), %edi
    leaq 4(%rcx,%rdi,1), %rdi
    cmpq (%rsp), %rdi
    jb minlz_encode_better_asm_match_dst_size_check_encodeBetterBlockAsm
    movq $0x00000000, 64(%rsp)
    jmp Lepi_bgen
minlz_encode_better_asm_match_dst_size_check_encodeBetterBlockAsm:
    movl %eax, %edi
    addl $0x04, %eax
    addl $0x04, %r8d
    movq 48(%rsp), %r9
    subl %eax, %r9d
    leaq (%rdx,%rax,1), %r10
    leaq (%rdx,%r8,1), %r11
    xorl %r13d, %r13d
    jmp minlz_encode_better_asm_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm:
    movq (%r10,%r13,1), %r12
    movq 8(%r10,%r13,1), %r14
    xorq (%r11,%r13,1), %r12
    jne minlz_encode_better_asm_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm
    xorq 8(%r11,%r13,1), %r14
    jne minlz_encode_better_asm_matchlen_bsf_16match_nolit_encodeBetterBlockAsm
    leal -16(%r9), %r9d
    leal 16(%r13), %r13d
minlz_encode_better_asm_matchlen_loop_16_entry_match_nolit_encodeBetterBlockAsm:
    cmpl $0x10, %r9d
    jae minlz_encode_better_asm_matchlen_loopback_16_match_nolit_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_matchlen_match8_match_nolit_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_bsf_16match_nolit_encodeBetterBlockAsm:
    tzcntq %r14, %r14
    sarq $0x03, %r14
    leal 8(%r13,%r14,1), %r13d
    jmp minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match8_match_nolit_encodeBetterBlockAsm:
    cmpl $0x08, %r9d
    jb minlz_encode_better_asm_matchlen_match4_match_nolit_encodeBetterBlockAsm
    movq (%r10,%r13,1), %r12
    xorq (%r11,%r13,1), %r12
    jne minlz_encode_better_asm_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm
    leal -8(%r9), %r9d
    leal 8(%r13), %r13d
    jmp minlz_encode_better_asm_matchlen_match4_match_nolit_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_bsf_8_match_nolit_encodeBetterBlockAsm:
    tzcntq %r12, %r12
    sarq $0x03, %r12
    leal (%r13,%r12,1), %r13d
    jmp minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match4_match_nolit_encodeBetterBlockAsm:
    cmpl $0x04, %r9d
    jb minlz_encode_better_asm_matchlen_match2_match_nolit_encodeBetterBlockAsm
    movl (%r10,%r13,1), %r12d
    cmpl %r12d, (%r11,%r13,1)
    jne minlz_encode_better_asm_matchlen_match2_match_nolit_encodeBetterBlockAsm
    leal -4(%r9), %r9d
    leal 4(%r13), %r13d
minlz_encode_better_asm_matchlen_match2_match_nolit_encodeBetterBlockAsm:
    cmpl $0x01, %r9d
    je minlz_encode_better_asm_matchlen_match1_match_nolit_encodeBetterBlockAsm
    jb minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm
    movw (%r10,%r13,1), %r12w
    cmpw %r12w, (%r11,%r13,1)
    jne minlz_encode_better_asm_matchlen_match1_match_nolit_encodeBetterBlockAsm
    leal 2(%r13), %r13d
    subl $0x02, %r9d
    je minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm
minlz_encode_better_asm_matchlen_match1_match_nolit_encodeBetterBlockAsm:
    movb (%r10,%r13,1), %r12b
    cmpb %r12b, (%r11,%r13,1)
    jne minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm
    leal 1(%r13), %r13d
minlz_encode_better_asm_match_nolit_end_encodeBetterBlockAsm:
    movl %eax, %r9d
    subl %r8d, %r9d
    cmpl $0x01, %r13d
    ja minlz_encode_better_asm_match_length_ok_encodeBetterBlockAsm
    cmpl $0x0001003f, %r9d
    jbe minlz_encode_better_asm_match_length_ok_encodeBetterBlockAsm
    movl 20(%rsp), %eax
    incl %eax
    jmp minlz_encode_better_asm_search_loop_encodeBetterBlockAsm
minlz_encode_better_asm_match_length_ok_encodeBetterBlockAsm:
    movl %r9d, 16(%rsp)
    movl 12(%rsp), %r10d
    movl %edi, %r8d
    subl %r10d, %r8d
    je minlz_encode_better_asm_match_emit_nolits_encodeBetterBlockAsm
    cmpl $0x00000040, %r9d
    jl minlz_encode_better_asm_match_emit_lits_encodeBetterBlockAsm
    cmpl $0x0001003f, %r9d
    ja minlz_encode_better_asm_match_emit_copy3_encodeBetterBlockAsm
    cmpl $0x04, %r8d
    ja minlz_encode_better_asm_match_emit_lits_encodeBetterBlockAsm
    movl (%rdx,%r10,1), %r10d
    addl %r13d, %eax
    addl $0x04, %r13d
    movl %eax, 12(%rsp)
    xorq %r11, %r11
    subl $0x40, %r9d
    leal -11(%r13), %r12d
    leal -4(%r13), %r13d
    movw %r9w, 1(%rcx)
    cmpl $0x07, %r13d
    cmovge %r12d, %r11d
    movq $0x00000007, %r9
    cmovl %r13d, %r9d
    leal -1(%r8,%r9,4), %r9d
    movl $0x00000003, %r12d
    leal (%r12,%r9,8), %r9d
    movb %r9b, (%rcx)
    addq $0x03, %rcx
    movl %r10d, (%rcx)
    addq %r8, %rcx
    testl %r11d, %r11d
    je minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
    leal -1(%r11), %r8d
    cmpl $0x1d, %r11d
    jbe minlz_encode_better_asm_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm
    leal -30(%r11), %r8d
    cmpl $0x0000011e, %r11d
    jb minlz_encode_better_asm_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm
    cmpl $0x0001001e, %r11d
    jb minlz_encode_better_asm_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm
    movb $0xfc, (%rcx)
    movl %r8d, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_three_match_emit_repeat_copy2_encodeBetterBlockAsm:
    movb $0xf4, (%rcx)
    movw %r8w, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_two_match_emit_repeat_copy2_encodeBetterBlockAsm:
    movb $0xec, (%rcx)
    movb %r8b, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_one_match_emit_repeat_copy2_encodeBetterBlockAsm:
    xorl %r8d, %r8d
    leal -4(%r8,%r11,8), %r8d
    movb %r8b, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_match_emit_copy3_encodeBetterBlockAsm:
    cmpl $0x03, %r8d
    ja minlz_encode_better_asm_match_emit_lits_encodeBetterBlockAsm
    movl 12(%rsp), %r10d
    movl (%rdx,%r10,1), %r10d
    addl %r13d, %eax
    addl $0x04, %r13d
    movl %eax, 12(%rsp)
    leal -4(%r13), %r13d
    leal -65536(%r9), %r9d
    shll $0x0b, %r9d
    leal 7(%r9,%r8,8), %r9d
    cmpl $0x3c, %r13d
    jbe minlz_encode_better_asm_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm
    leal -60(%r13), %r11d
    cmpl $0x0000013c, %r13d
    jb minlz_encode_better_asm_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm
    cmpl $0x0001003c, %r13d
    jb minlz_encode_better_asm_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm
    addl $0x000007e0, %r9d
    movl %r9d, (%rcx)
    movl %r11d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_match_emit_copy_litsencodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_2_match_emit_lits_encodeBetterBlockAsm:
    addl $0x000007c0, %r9d
    movl %r9d, (%rcx)
    movw %r11w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_match_emit_copy_litsencodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_1_match_emit_lits_encodeBetterBlockAsm:
    addl $0x000007a0, %r9d
    movl %r9d, (%rcx)
    movb %r11b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_match_emit_copy_litsencodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_0_match_emit_lits_encodeBetterBlockAsm:
    shll $0x05, %r13d
    orl %r13d, %r9d
    movl %r9d, (%rcx)
    addq $0x04, %rcx
minlz_encode_better_asm_match_emit_copy_litsencodeBetterBlockAsm:
    movl %r10d, (%rcx)
    addq %r8, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_match_emit_lits_encodeBetterBlockAsm:
    leaq (%rdx,%r10,1), %r10
    leal -1(%r8), %r11d
    cmpl $0x1d, %r11d
    jb minlz_encode_better_asm_one_byte_match_emit_encodeBetterBlockAsm
    subl $0x1d, %r11d
    cmpl $0x00000100, %r11d
    jb minlz_encode_better_asm_two_bytes_match_emit_encodeBetterBlockAsm
    cmpl $0x00010000, %r11d
    jb minlz_encode_better_asm_three_bytes_match_emit_encodeBetterBlockAsm
    movl %r11d, %r12d
    shrl $0x10, %r12d
    movb $0xf8, (%rcx)
    movw %r11w, 1(%rcx)
    movb %r12b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r11d
    jmp minlz_encode_better_asm_memmove_long_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_three_bytes_match_emit_encodeBetterBlockAsm:
    movb $0xf0, (%rcx)
    movw %r11w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r11d
    jmp minlz_encode_better_asm_memmove_long_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_two_bytes_match_emit_encodeBetterBlockAsm:
    movb $0xe8, (%rcx)
    movb %r11b, 1(%rcx)
    addl $0x1d, %r11d
    addq $0x02, %rcx
    cmpl $0x40, %r11d
    jb minlz_encode_better_asm_memmove_midmatch_emit_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_memmove_long_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_one_byte_match_emit_encodeBetterBlockAsm:
    shlb $0x03, %r11b
    movb %r11b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r8,1), %r11
    cmpq $0x10, %r8
    jbe minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_8through16
    cmpq $0x20, %r8
    jbe minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_8through16:
    movdqu (%r10), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_memmove_end_copy_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_better_asm_memmove_end_copy_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_match_emit_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r8,1), %xmm2
    movdqu -16(%r10,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_better_asm_memmove_end_copy_match_emit_encodeBetterBlockAsm:
    movq %r11, %rcx
    jmp minlz_encode_better_asm_match_emit_nolits_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_midmatch_emit_encodeBetterBlockAsm:
    leaq (%rcx,%r8,1), %r11
    cmpq $0x20, %r8
    jbe minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r8,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r8,1)
    jmp minlz_encode_better_asm_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r8,1), %xmm2
    movdqu -16(%r10,%r8,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
minlz_encode_better_asm_memmove_mid_end_copy_match_emit_encodeBetterBlockAsm:
    movq %r11, %rcx
    jmp minlz_encode_better_asm_match_emit_nolits_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_long_match_emit_encodeBetterBlockAsm:
    leaq (%rcx,%r8,1), %r11
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r8,1), %xmm2
    movdqu -16(%r10,%r8,1), %xmm3
    movq %r8, %r14
    shrq $0x05, %r14
    movq %rcx, %r12
    andl $0x0000001f, %r12d
    movq $0x00000040, %r15
    subq %r12, %r15
    decq %r14
    ja minlz_encode_better_asm_emit_lit_memmove_long_match_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32
    leaq -32(%r10,%r15,1), %r12
    leaq -32(%rcx,%r15,1), %rbp
minlz_encode_better_asm_emit_lit_memmove_long_match_emit_encodeBetterBlockAsmlarge_big_loop_back:
    movdqu (%r12), %xmm4
    movdqu 16(%r12), %xmm5
    movdqu %xmm4, (%rbp)
    movdqu %xmm5, 16(%rbp)
    addq $0x20, %rbp
    addq $0x20, %r12
    addq $0x20, %r15
    decq %r14
    jbe minlz_encode_better_asm_emit_lit_memmove_long_match_emit_encodeBetterBlockAsmlarge_big_loop_back
minlz_encode_better_asm_emit_lit_memmove_long_match_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%r10,%r15,1), %xmm4
    movdqu -16(%r10,%r15,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r15,1)
    movdqu %xmm5, -16(%rcx,%r15,1)
    addq $0x20, %r15
    cmpq %r15, %r8
    jae minlz_encode_better_asm_emit_lit_memmove_long_match_emit_encodeBetterBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r8,1)
    movdqu %xmm3, -16(%rcx,%r8,1)
    movq %r11, %rcx
minlz_encode_better_asm_match_emit_nolits_encodeBetterBlockAsm:
    addl %r13d, %eax
    addl $0x04, %r13d
    movl %eax, 12(%rsp)
    cmpl $0x0001003f, %r9d
    jbe minlz_encode_better_asm_two_byte_offset_match_nolit_encodeBetterBlockAsm
    leal -4(%r13), %r13d
    leal -65536(%r9), %r8d
    shll $0x0b, %r8d
    addl $0x07, %r8d
    cmpl $0x3c, %r13d
    jbe minlz_encode_better_asm_emit_copy3_0_match_nolit_encodeBetterBlockAsm_emit3
    leal -60(%r13), %r9d
    cmpl $0x0000013c, %r13d
    jb minlz_encode_better_asm_emit_copy3_1_match_nolit_encodeBetterBlockAsm_emit3
    cmpl $0x0001003c, %r13d
    jb minlz_encode_better_asm_emit_copy3_2_match_nolit_encodeBetterBlockAsm_emit3
    addl $0x000007e0, %r8d
    movl %r8d, (%rcx)
    movl %r9d, 4(%rcx)
    addq $0x07, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_2_match_nolit_encodeBetterBlockAsm_emit3:
    addl $0x000007c0, %r8d
    movl %r8d, (%rcx)
    movw %r9w, 4(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_1_match_nolit_encodeBetterBlockAsm_emit3:
    addl $0x000007a0, %r8d
    movl %r8d, (%rcx)
    movb %r9b, 4(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy3_0_match_nolit_encodeBetterBlockAsm_emit3:
    shll $0x05, %r13d
    orl %r13d, %r8d
    movl %r8d, (%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_two_byte_offset_match_nolit_encodeBetterBlockAsm:
    cmpl $0x00000400, %r9d
    ja minlz_encode_better_asm_two_byte_match_nolit_encodeBetterBlockAsm
    cmpl $0x00000013, %r13d
    jae minlz_encode_better_asm_emit_one_longer_match_nolit_encodeBetterBlockAsm
    leal -1(%r9), %r8d
    shll $0x06, %r8d
    leal -15(%r8,%r13,4), %r8d
    movw %r8w, (%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_one_longer_match_nolit_encodeBetterBlockAsm:
    cmpl $0x00000112, %r13d
    jae minlz_encode_better_asm_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm
    leal -1(%r9), %r8d
    shll $0x06, %r8d
    leal 61(%r8), %r8d
    movw %r8w, (%rcx)
    leal -18(%r13), %r8d
    movb %r8b, 2(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy1_repeat_match_nolit_encodeBetterBlockAsm:
    leal -1(%r9), %r8d
    shll $0x06, %r8d
    leal 57(%r8), %r8d
    movw %r8w, (%rcx)
    addq $0x02, %rcx
    subl $0x12, %r13d
    leal -1(%r13), %r8d
    cmpl $0x1d, %r13d
    jbe minlz_encode_better_asm_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm
    leal -30(%r13), %r8d
    cmpl $0x0000011e, %r13d
    jb minlz_encode_better_asm_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm
    cmpl $0x0001001e, %r13d
    jb minlz_encode_better_asm_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm
    movb $0xfc, (%rcx)
    movl %r8d, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_three_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm:
    movb $0xf4, (%rcx)
    movw %r8w, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_two_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm:
    movb $0xec, (%rcx)
    movb %r8b, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_one_emit_copy1_do_repeat_match_nolit_encodeBetterBlockAsm:
    xorl %r8d, %r8d
    leal -4(%r8,%r13,8), %r8d
    movb %r8b, (%rcx)
    addq $0x01, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_two_byte_match_nolit_encodeBetterBlockAsm:
    leal -64(%r9), %r9d
    leal -4(%r13), %r13d
    movw %r9w, 1(%rcx)
    cmpl $0x3c, %r13d
    jbe minlz_encode_better_asm_emit_copy2_0_match_nolit_encodeBetterBlockAsm_emit2
    leal -60(%r13), %r8d
    cmpl $0x0000013c, %r13d
    jb minlz_encode_better_asm_emit_copy2_1_match_nolit_encodeBetterBlockAsm_emit2
    cmpl $0x0001003c, %r13d
    jb minlz_encode_better_asm_emit_copy2_2_match_nolit_encodeBetterBlockAsm_emit2
    movb $0xfe, (%rcx)
    movl %r8d, 3(%rcx)
    addq $0x06, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy2_2_match_nolit_encodeBetterBlockAsm_emit2:
    movb $0xfa, (%rcx)
    movw %r8w, 3(%rcx)
    addq $0x05, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy2_1_match_nolit_encodeBetterBlockAsm_emit2:
    movb $0xf6, (%rcx)
    movb %r8b, 3(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_emit_copy2_0_match_nolit_encodeBetterBlockAsm_emit2:
    movl $0x00000002, %r8d
    leal (%r8,%r13,4), %r8d
    movb %r8b, (%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
    movl 12(%rsp), %r8d
    cmpl %edi, %r8d
    je minlz_encode_better_asm_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm
    movl %edi, %r9d
    movl %edi, 12(%rsp)
    leaq (%rdx,%r8,1), %r10
    subl %r8d, %r9d
    leal -1(%r9), %r8d
    cmpl $0x1d, %r8d
    jb minlz_encode_better_asm_one_byte_match_emit_repeat_encodeBetterBlockAsm
    subl $0x1d, %r8d
    cmpl $0x00000100, %r8d
    jb minlz_encode_better_asm_two_bytes_match_emit_repeat_encodeBetterBlockAsm
    cmpl $0x00010000, %r8d
    jb minlz_encode_better_asm_three_bytes_match_emit_repeat_encodeBetterBlockAsm
    movl %r8d, %r11d
    shrl $0x10, %r11d
    movb $0xf8, (%rcx)
    movw %r8w, 1(%rcx)
    movb %r11b, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %r8d
    jmp minlz_encode_better_asm_memmove_long_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_three_bytes_match_emit_repeat_encodeBetterBlockAsm:
    movb $0xf0, (%rcx)
    movw %r8w, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %r8d
    jmp minlz_encode_better_asm_memmove_long_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_two_bytes_match_emit_repeat_encodeBetterBlockAsm:
    movb $0xe8, (%rcx)
    movb %r8b, 1(%rcx)
    addl $0x1d, %r8d
    addq $0x02, %rcx
    cmpl $0x40, %r8d
    jb minlz_encode_better_asm_memmove_midmatch_emit_repeat_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_memmove_long_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_one_byte_match_emit_repeat_encodeBetterBlockAsm:
    shlb $0x03, %r8b
    movb %r8b, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%r9,1), %r8
    cmpq $0x10, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_8through16
    cmpq $0x20, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_8through16:
    movdqu (%r10), %xmm0
    movdqu %xmm0, (%rcx)
    jmp minlz_encode_better_asm_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r9,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r9,1)
    jmp minlz_encode_better_asm_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_match_emit_repeat_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
minlz_encode_better_asm_memmove_end_copy_match_emit_repeat_encodeBetterBlockAsm:
    movq %r8, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_midmatch_emit_repeat_encodeBetterBlockAsm:
    leaq (%rcx,%r9,1), %r8
    cmpq $0x20, %r9
    jbe minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%r10), %xmm0
    movdqu -16(%r10,%r9,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%r9,1)
    jmp minlz_encode_better_asm_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_mid_match_emit_repeat_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
minlz_encode_better_asm_memmove_mid_end_copy_match_emit_repeat_encodeBetterBlockAsm:
    movq %r8, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_long_match_emit_repeat_encodeBetterBlockAsm:
    leaq (%rcx,%r9,1), %r8
    movdqu (%r10), %xmm0
    movdqu 16(%r10), %xmm1
    movdqu -32(%r10,%r9,1), %xmm2
    movdqu -16(%r10,%r9,1), %xmm3
    movq %r9, %r12
    shrq $0x05, %r12
    movq %rcx, %r11
    andl $0x0000001f, %r11d
    movq $0x00000040, %r14
    subq %r11, %r14
    decq %r12
    ja minlz_encode_better_asm_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsmlarge_forward_sse_loop_32
    leaq -32(%r10,%r14,1), %r11
    leaq -32(%rcx,%r14,1), %r15
minlz_encode_better_asm_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsmlarge_big_loop_back:
    movdqu (%r11), %xmm4
    movdqu 16(%r11), %xmm5
    movdqu %xmm4, (%r15)
    movdqu %xmm5, 16(%r15)
    addq $0x20, %r15
    addq $0x20, %r11
    addq $0x20, %r14
    decq %r12
    jbe minlz_encode_better_asm_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsmlarge_big_loop_back
minlz_encode_better_asm_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%r10,%r14,1), %xmm4
    movdqu -16(%r10,%r14,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r14,1)
    movdqu %xmm5, -16(%rcx,%r14,1)
    addq $0x20, %r14
    cmpq %r14, %r9
    jae minlz_encode_better_asm_emit_lit_memmove_long_match_emit_repeat_encodeBetterBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%r9,1)
    movdqu %xmm3, -16(%rcx,%r9,1)
    movq %r8, %rcx
minlz_encode_better_asm_emit_literal_done_match_emit_repeat_encodeBetterBlockAsm:
    addl %r13d, %eax
    addl $0x04, %r13d
    movl %eax, 12(%rsp)
    leal -1(%r13), %r8d
    cmpl $0x1d, %r13d
    jbe minlz_encode_better_asm_repeat_one_match_nolit_repeat_encodeBetterBlockAsm
    leal -30(%r13), %r8d
    cmpl $0x0000011e, %r13d
    jb minlz_encode_better_asm_repeat_two_match_nolit_repeat_encodeBetterBlockAsm
    cmpl $0x0001001e, %r13d
    jb minlz_encode_better_asm_repeat_three_match_nolit_repeat_encodeBetterBlockAsm
    movb $0xfc, (%rcx)
    movl %r8d, 1(%rcx)
    addq $0x04, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_three_match_nolit_repeat_encodeBetterBlockAsm:
    movb $0xf4, (%rcx)
    movw %r8w, 1(%rcx)
    addq $0x03, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_two_match_nolit_repeat_encodeBetterBlockAsm:
    movb $0xec, (%rcx)
    movb %r8b, 1(%rcx)
    addq $0x02, %rcx
    jmp minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm
minlz_encode_better_asm_repeat_one_match_nolit_repeat_encodeBetterBlockAsm:
    xorl %r8d, %r8d
    leal -4(%r8,%r13,8), %r8d
    movb %r8b, (%rcx)
    addq $0x01, %rcx
minlz_encode_better_asm_match_nolit_emitcopy_end_encodeBetterBlockAsm:
    cmpl 8(%rsp), %eax
    jae minlz_encode_better_asm_emit_remainder_encodeBetterBlockAsm
    cmpq (%rsp), %rcx
    jb minlz_encode_better_asm_match_nolit_dst_ok_encodeBetterBlockAsm
    movq $0x00000000, 64(%rsp)
    jmp Lepi_bgen
minlz_encode_better_asm_match_nolit_dst_ok_encodeBetterBlockAsm:
    movq 56(%rsp), %r8
    movq $0x00cf1bbcdcbfa563, %r9
    movq $0x9e3779b1, %r10
    leaq 1(%rdi), %rdi
    leaq -2(%rax), %r11
    movq (%rdx,%rdi,1), %r12
    movq 1(%rdx,%rdi,1), %r13
    movq (%rdx,%r11,1), %r14
    movq 1(%rdx,%r11,1), %r15
    shlq $0x08, %r12
    imulq %r9, %r12
    shrq $0x2f, %r12
    shlq $0x20, %r13
    imulq %r10, %r13
    shrq $0x33, %r13
    shlq $0x08, %r14
    imulq %r9, %r14
    shrq $0x2f, %r14
    shlq $0x20, %r15
    imulq %r10, %r15
    shrq $0x33, %r15
    leaq 1(%rdi), %r10
    leaq 1(%r11), %rbp
    movl %edi, (%r8,%r12,4)
    movl %r11d, (%r8,%r14,4)
    leaq 1(%r11,%rdi,1), %r12
    shrq $0x01, %r12
    addq $0x01, %rdi
    subq $0x01, %r11
    movl %r10d, 524288(%r8,%r13,4)
    movl %ebp, 524288(%r8,%r15,4)
minlz_encode_better_asm_index_loop_encodeBetterBlockAsm:
    cmpq %r11, %r12
    jae minlz_encode_better_asm_search_loop_encodeBetterBlockAsm
    movq (%rdx,%rdi,1), %r10
    movq (%rdx,%r12,1), %r13
    shlq $0x08, %r10
    imulq %r9, %r10
    shrq $0x2f, %r10
    shlq $0x08, %r13
    imulq %r9, %r13
    shrq $0x2f, %r13
    movl %edi, (%r8,%r10,4)
    movl %r11d, (%r8,%r13,4)
    addq $0x02, %rdi
    addq $0x02, %r12
    jmp minlz_encode_better_asm_index_loop_encodeBetterBlockAsm
minlz_encode_better_asm_emit_remainder_encodeBetterBlockAsm:
    movq 48(%rsp), %rax
    subl 12(%rsp), %eax
    leaq 4(%rcx,%rax,1), %rax
    cmpq (%rsp), %rax
    jb minlz_encode_better_asm_emit_remainder_ok_encodeBetterBlockAsm
    movq $0x00000000, 64(%rsp)
    jmp Lepi_bgen
minlz_encode_better_asm_emit_remainder_ok_encodeBetterBlockAsm:
    movq 48(%rsp), %rax
    movl 12(%rsp), %ebx
    cmpl %eax, %ebx
    je minlz_encode_better_asm_emit_literal_done_emit_remainder_encodeBetterBlockAsm
    movl %eax, %esi
    movl %eax, 12(%rsp)
    leaq (%rdx,%rbx,1), %rax
    subl %ebx, %esi
    leal -1(%rsi), %edx
    cmpl $0x1d, %edx
    jb minlz_encode_better_asm_one_byte_emit_remainder_encodeBetterBlockAsm
    subl $0x1d, %edx
    cmpl $0x00000100, %edx
    jb minlz_encode_better_asm_two_bytes_emit_remainder_encodeBetterBlockAsm
    cmpl $0x00010000, %edx
    jb minlz_encode_better_asm_three_bytes_emit_remainder_encodeBetterBlockAsm
    movl %edx, %ebx
    shrl $0x10, %ebx
    movb $0xf8, (%rcx)
    movw %dx, 1(%rcx)
    movb %bl, 3(%rcx)
    addq $0x04, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_memmove_long_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_three_bytes_emit_remainder_encodeBetterBlockAsm:
    movb $0xf0, (%rcx)
    movw %dx, 1(%rcx)
    addq $0x03, %rcx
    addl $0x1d, %edx
    jmp minlz_encode_better_asm_memmove_long_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_two_bytes_emit_remainder_encodeBetterBlockAsm:
    movb $0xe8, (%rcx)
    movb %dl, 1(%rcx)
    addl $0x1d, %edx
    addq $0x02, %rcx
    cmpl $0x40, %edx
    jb minlz_encode_better_asm_memmove_midemit_remainder_encodeBetterBlockAsm
    jmp minlz_encode_better_asm_memmove_long_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_one_byte_emit_remainder_encodeBetterBlockAsm:
    shlb $0x03, %dl
    movb %dl, (%rcx)
    addq $0x01, %rcx
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x03, %rbx
    jb minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_1or2
    je minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_3
    cmpq $0x08, %rbx
    jbe minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_4through8
    cmpq $0x10, %rbx
    jbe minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_8through16
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_1or2:
    movb (%rax), %sil
    movb -1(%rax,%rbx,1), %al
    movb %sil, (%rcx)
    movb %al, -1(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_3:
    movw (%rax), %si
    movb 2(%rax), %al
    movw %si, (%rcx)
    movb %al, 2(%rcx)
    jmp minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_4through8:
    movl (%rax), %esi
    movl -4(%rax,%rbx,1), %eax
    movl %esi, (%rcx)
    movl %eax, -4(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_8through16:
    movq (%rax), %rsi
    movq -8(%rax,%rbx,1), %rax
    movq %rsi, (%rcx)
    movq %rax, -8(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_emit_remainder_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_memmove_end_copy_emit_remainder_encodeBetterBlockAsm:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_midemit_remainder_encodeBetterBlockAsm:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    cmpq $0x20, %rbx
    jbe minlz_encode_better_asm_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm_memmove_move_17through32
    jmp minlz_encode_better_asm_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm_memmove_move_33through64
minlz_encode_better_asm_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm_memmove_move_17through32:
    movdqu (%rax), %xmm0
    movdqu -16(%rax,%rbx,1), %xmm1
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, -16(%rcx,%rbx,1)
    jmp minlz_encode_better_asm_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_emit_lit_memmove_mid_emit_remainder_encodeBetterBlockAsm_memmove_move_33through64:
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
minlz_encode_better_asm_memmove_mid_end_copy_emit_remainder_encodeBetterBlockAsm:
    movq %rdx, %rcx
    jmp minlz_encode_better_asm_emit_literal_done_emit_remainder_encodeBetterBlockAsm
minlz_encode_better_asm_memmove_long_emit_remainder_encodeBetterBlockAsm:
    leaq (%rcx,%rsi,1), %rdx
    movl %esi, %ebx
    movdqu (%rax), %xmm0
    movdqu 16(%rax), %xmm1
    movdqu -32(%rax,%rbx,1), %xmm2
    movdqu -16(%rax,%rbx,1), %xmm3
    movq %rbx, %rdi
    shrq $0x05, %rdi
    movq %rcx, %rsi
    andl $0x0000001f, %esi
    movq $0x00000040, %r8
    subq %rsi, %r8
    decq %rdi
    ja minlz_encode_better_asm_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsmlarge_forward_sse_loop_32
    leaq -32(%rax,%r8,1), %rsi
    leaq -32(%rcx,%r8,1), %r9
minlz_encode_better_asm_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsmlarge_big_loop_back:
    movdqu (%rsi), %xmm4
    movdqu 16(%rsi), %xmm5
    movdqu %xmm4, (%r9)
    movdqu %xmm5, 16(%r9)
    addq $0x20, %r9
    addq $0x20, %rsi
    addq $0x20, %r8
    decq %rdi
    jbe minlz_encode_better_asm_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsmlarge_big_loop_back
minlz_encode_better_asm_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsmlarge_forward_sse_loop_32:
    movdqu -32(%rax,%r8,1), %xmm4
    movdqu -16(%rax,%r8,1), %xmm5
    movdqu %xmm4, -32(%rcx,%r8,1)
    movdqu %xmm5, -16(%rcx,%r8,1)
    addq $0x20, %r8
    cmpq %r8, %rbx
    jae minlz_encode_better_asm_emit_lit_memmove_long_emit_remainder_encodeBetterBlockAsmlarge_forward_sse_loop_32
    movdqu %xmm0, (%rcx)
    movdqu %xmm1, 16(%rcx)
    movdqu %xmm2, -32(%rcx,%rbx,1)
    movdqu %xmm3, -16(%rcx,%rbx,1)
    movq %rdx, %rcx
minlz_encode_better_asm_emit_literal_done_emit_remainder_encodeBetterBlockAsm:
    movq 32(%rsp), %rax
    subq %rax, %rcx
    movq %rcx, 64(%rsp)
    jmp Lepi_bgen
Lepi_bgen:
    movq 64(%rsp), %rax
    add $72, %rsp
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    ret
