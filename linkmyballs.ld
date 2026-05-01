ENTRY(_start)

SECTIONS {
    . = 0x40000000;

    /* executable code */
    .text : ALIGN(4K) {
        KEEP(*(.text._start))
        *(.text*)
    }

    /* read-only data */
    .rodata : ALIGN(4K) {
        *(.rodata*)
    }

    /* writable data */
    .data : ALIGN(4K) {
        *(.data*)
    }

    /* uninitialized data */
    .bss : ALIGN(4K) {
        *(.bss*)
        *(COMMON)
    }

    /* stack (simple static one) */
    . = ALIGN(16);
    stack_bottom = .;
    . += 0x10000;
    stack_top = .;
}
