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
    bl kernel_main

1:
    wfe
    b 1b

.size _start, . - _start
