// This kernel does not link Rust's standard library. A bare-metal target has no
// operating system, allocator, files, threads, or process runtime for std to use.
#![no_std]
// Disable Rust's generated `main` entry point. QEMU jumps directly to the ELF
// entry address, so the linker script and `_start` below define the first code.
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// `_start` is the true CPU entry point.
//
// Rust functions expect a valid stack before they run. At reset/QEMU entry we
// cannot assume one exists, so this tiny assembly stub sets `sp` first. It also
// clears `.bss`, because normal language runtimes usually zero uninitialized
// static memory before calling application code. In a kernel, we are the runtime.
global_asm!(
    r#"
    // Put this symbol in its own text subsection so the linker script can keep
    // it first, even when section garbage collection is enabled.
    .section .text._start, "ax"
    .global _start
_start:
    // Load the linker-defined stack top address and install it as the current
    // stack pointer. AArch64 stacks grow downward, so the top is the first SP.
    ldr x0, =stack_top
    mov sp, x0

    // Zero the .bss range [__bss_start, __bss_end). Rust may place
    // zero-initialized statics here; the ELF file does not store those zeros.
    ldr x0, =__bss_start
    ldr x1, =__bss_end
0:
    cmp x0, x1
    b.hs 1f

    // Store eight zero bytes, then advance the pointer by eight bytes.
    // The linker aligns the end of .bss to 16 bytes, so this loop is safe for
    // the simple sections we currently emit.
    str xzr, [x0], #8
    b 0b
1:
    // Now that the minimal runtime setup is done, enter Rust code.
    bl kmain
2:
    // `kmain` is declared as diverging, but stay parked if it ever returns.
    wfe
    b 2b
"#
);

// QEMU's `virt` machine exposes a PL011 UART at this fixed physical address.
// With `-nographic`, QEMU connects that UART to the terminal.
const UART_BASE: usize = 0x0900_0000;

// PL011 UART registers used by this kernel. They are memory-mapped hardware
// registers, so reads and writes must be volatile: the compiler must not remove,
// reorder, or cache them as ordinary memory.
const UART_DR: *mut u32 = (UART_BASE + 0x000) as *mut u32;
const UART_FR: *const u32 = (UART_BASE + 0x018) as *const u32;
const UART_IBRD: *mut u32 = (UART_BASE + 0x024) as *mut u32;
const UART_FBRD: *mut u32 = (UART_BASE + 0x028) as *mut u32;
const UART_LCRH: *mut u32 = (UART_BASE + 0x02c) as *mut u32;
const UART_CR: *mut u32 = (UART_BASE + 0x030) as *mut u32;
const UART_IMSC: *mut u32 = (UART_BASE + 0x038) as *mut u32;
const UART_ICR: *mut u32 = (UART_BASE + 0x044) as *mut u32;

// Selected PL011 bit fields. Naming the bits keeps the register writes below
// readable without pulling in a full hardware abstraction layer yet.
const UART_FR_TXFF: u32 = 1 << 5;
const UART_CR_UARTEN: u32 = 1 << 0;
const UART_CR_TXE: u32 = 1 << 8;
const UART_CR_RXE: u32 = 1 << 9;
const UART_LCRH_FEN: u32 = 1 << 4;
const UART_LCRH_WLEN_8: u32 = 3 << 5;

// Put the UART into a known transmit-capable state.
fn uart_init() {
    unsafe {
        // Disable the UART before changing its configuration.
        UART_CR.write_volatile(0);

        // Clear pending UART interrupts. We are not using interrupts yet, but
        // clearing stale state makes this initialization deterministic.
        UART_ICR.write_volatile(0x7ff);

        // Baud-rate divisor registers. Under QEMU these values are enough for
        // working serial output; QEMU is forgiving compared to real hardware.
        UART_IBRD.write_volatile(1);
        UART_FBRD.write_volatile(40);

        // 8 data bits, FIFO enabled.
        UART_LCRH.write_volatile(UART_LCRH_WLEN_8 | UART_LCRH_FEN);

        // Mask all UART interrupts. This early kernel only polls the UART.
        UART_IMSC.write_volatile(0);

        // Enable the UART itself plus transmit and receive paths.
        UART_CR.write_volatile(UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE);
    }
}

// Send one byte to the UART.
fn uart_putc(c: u8) {
    unsafe {
        // Wait while the transmit FIFO is full. Without this, a byte could be
        // dropped if software writes faster than the UART can accept data.
        while UART_FR.read_volatile() & UART_FR_TXFF != 0 {}

        // Writing to DR pushes the byte into the UART transmit FIFO.
        UART_DR.write_volatile(c as u32);
    }
}

// Tiny string printer for early boot diagnostics.
fn kprint(s: &str) {
    for b in s.bytes() {
        // Many serial terminals expect CRLF for a new line. Rust strings use
        // LF, so emit CR before LF to keep terminal output tidy.
        if b == b'\n' {
            uart_putc(b'\r');
        }
        uart_putc(b);
    }
}

// Rust entry point called by `_start`.
//
// `no_mangle` keeps the symbol name exactly `kmain`, so the assembly `bl kmain`
// instruction can find it. In Rust 2024 this attribute is marked unsafe because
// exporting exact symbol names can conflict with other linker symbols.
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    uart_init();
    kprint("steelray: booted\n");

    // There is no scheduler or power management yet. Spin forever so execution
    // stays in known kernel code after the boot message.
    loop {
        core::hint::spin_loop();
    }
}

// `panic=abort` avoids stack unwinding, but the core library still requires a
// panic handler for no_std binaries. If a panic happens after UART init, this
// gives us a visible failure message before halting.
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    kprint("steelray: panic\n");

    // Do not return from a panic in a kernel.
    loop {
        core::hint::spin_loop();
    }
}
