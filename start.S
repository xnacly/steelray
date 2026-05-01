.section .text._start
.global _start

_start:
     ldr x0, =stack_top
    mov sp, x0

    mov x29, #0

    bl kmain

1:
    wfe
    b 1b
