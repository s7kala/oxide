.section .text.boot
.global _start
.type _start, %function

_start:
    ldr x0, =__boot_stack_top
    mov sp, x0

    ldr x0, =__bss_start
    ldr x1, =__bss_end

clear_bss:
    cmp x0, x1
    b.hs bss_cleared
    str xzr, [x0], #8
    b clear_bss

bss_cleared:
    mrs x0, CurrentEL
    lsr x0, x0, #2

    cmp x0, #2
    b.eq drop_to_el1

    cmp x0, #1
    b.eq setup_el1

hang:
    wfe
    b hang

setup_el1:
    // Allow EL1/EL0 FP/SIMD access.
    mov x0, #(3 << 20)
    msr CPACR_EL1, x0
    isb

    ldr x0, =0x30D00800
    msr SCTLR_EL1, x0
    isb

    adr x0, el1_vectors
    msr VBAR_EL1, x0
    isb

    b el1_entry

drop_to_el1:
    // EL1 is AArch64.
    mov x0, #(1 << 31)
    msr HCR_EL2, x0
    isb

    // Do not trap FP/SIMD at EL2.
    msr CPTR_EL2, xzr
    isb

    // Allow EL1/EL0 FP/SIMD access.
    mov x0, #(3 << 20)
    msr CPACR_EL1, x0
    isb

    ldr x0, =0x30D00800
    msr SCTLR_EL1, x0
    isb

    // EL1 stack.
    ldr x0, =__boot_stack_top
    msr SP_EL1, x0

    // Return target.
    adr x0, el1_entry
    msr ELR_EL2, x0

    adr x0, el1_vectors
    msr VBAR_EL1, x0
    isb

    // EL1h, DAIF masked.
    mov x0, #0x3c5
    msr SPSR_EL2, x0
    isb

    eret

.global el1_entry
.type el1_entry, %function
el1_entry:
    ldr x0, =__boot_stack_top
    mov sp, x0

    bl kernel_main

1:
    wfe
    b 1b

.align 11
.global el1_vectors
el1_vectors:
    .rept 16
        b exception_hang
        .space 0x80 - 4
    .endr

exception_hang:
    wfe
    b exception_hang

.size _start, . - _start
