/* The CPU starts executing at the ELF entry symbol. */
ENTRY(_start)

/*
 * Program headers describe the loadable chunks QEMU places in guest memory.
 *
 * FLAGS values are ELF permission bits:
 *   5 = read + execute
 *   4 = read
 *   6 = read + write
 */
PHDRS {
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
}

SECTIONS {
    /*
     * QEMU's aarch64 `virt` machine loads `-kernel` images at 0x40000000.
     * The linker must assign the same virtual/physical addresses, otherwise
     * absolute addresses such as `stack_top` would point somewhere else.
     */
    . = 0x40000000;

    /*
     * Executable code. Keep `_start` first so the ELF entry points at the
     * initial assembly stub, not at whichever Rust function the linker happens
     * to place first.
     */
    .text : ALIGN(4K) {
        KEEP(*(.text._start))
        *(.text*)
    } :text

    /*
     * Read-only data such as string literals. The exception-frame sections can
     * appear even in tiny freestanding Rust objects, so collect them here.
     */
    .rodata : ALIGN(4K) {
        *(.rodata*)
        *(.eh_frame_hdr*)
        *(.eh_frame*)
    } :rodata

    /*
     * Writable initialized data. `.got` is included so any generated global
     * offset table entries live in writable memory.
     */
    .data : ALIGN(4K) {
        *(.data*)
        *(.got*)
    } :data

    /*
     * Zero-initialized writable data. NOLOAD means the ELF file does not store
     * bytes for this section; `_start` zeros the range at runtime instead.
     */
    .bss (NOLOAD) : ALIGN(4K) {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    } :data

    /*
     * A simple fixed-size boot stack. The stack grows downward, so `_start`
     * loads `stack_top` into SP before calling Rust code.
     */
    .stack (NOLOAD) : ALIGN(16) {
        stack_bottom = .;
        . += 0x10000;
        stack_top = .;
    } :data

    /*
     * Drop metadata that is useful to host tooling but not needed by the guest
     * kernel image at runtime.
     */
    /DISCARD/ : {
        *(.comment*)
        *(.note*)
    }
}
