ENTRY(_start)

KERNEL_BASE = 0x40000000;
QEMU_VIRT_RAM_SIZE = 128M;
BOOT_STACK_SIZE = 64K;

/* QEMU `virt` RAM starts at 0x40000000; ELF program headers choose where the
 * `-kernel` ELF segments are loaded inside that RAM. */
MEMORY {
    RAM (rwx) : ORIGIN = KERNEL_BASE, LENGTH = QEMU_VIRT_RAM_SIZE
}

PHDRS {
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
}

SECTIONS {
    . = ORIGIN(RAM);

    .text : ALIGN(4K) {
        KEEP(*(.text._start))
        *(.text*)
    } > RAM :text

    .rodata : ALIGN(4K) {
        *(.rodata*)
        *(.eh_frame_hdr*)
        *(.eh_frame*)
    } > RAM :rodata

    .data : ALIGN(4K) {
        *(.data*)
        *(.got*)
    } > RAM :data

    .bss (NOLOAD) : ALIGN(4K) {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    } > RAM :data

    /* AArch64 stacks grow downward; `_start` loads `stack_top` into SP. */
    .stack (NOLOAD) : ALIGN(16) {
        stack_bottom = .;
        . += BOOT_STACK_SIZE;
        stack_top = .;
    } > RAM :data

    /DISCARD/ : {
        *(.comment*)
        *(.note*)
    }
}

ASSERT(stack_bottom >= ORIGIN(RAM), "boot stack starts below QEMU virt RAM");
ASSERT(stack_top <= ORIGIN(RAM) + LENGTH(RAM),
       "kernel image plus boot stack exceeds QEMU virt RAM");
ASSERT((stack_top & 0xf) == 0, "boot stack top must be 16-byte aligned");
