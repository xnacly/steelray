#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Rust functions expect a valid stack before they run. On bare metal there is
// no caller-provided runtime, so `_start` creates the minimal environment before
// branching into Rust.
global_asm!(
    r#"
    .section .text._start, "ax"
    .global _start
_start:
    ldr x0, =stack_top
    mov sp, x0

    ldr x0, =__bss_start
    ldr x1, =__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b
1:
    bl kmain
2:
    wfe
    b 2b
"#
);

// QEMU's `virt` machine exposes a PL011 UART here; `-nographic` wires it to the
// terminal.
const UART_BASE: usize = 0x0900_0000;

const UART_DR: *mut u32 = (UART_BASE + 0x000) as *mut u32;
const UART_FR: *const u32 = (UART_BASE + 0x018) as *const u32;
const UART_IBRD: *mut u32 = (UART_BASE + 0x024) as *mut u32;
const UART_FBRD: *mut u32 = (UART_BASE + 0x028) as *mut u32;
const UART_LCRH: *mut u32 = (UART_BASE + 0x02c) as *mut u32;
const UART_CR: *mut u32 = (UART_BASE + 0x030) as *mut u32;
const UART_IMSC: *mut u32 = (UART_BASE + 0x038) as *mut u32;
const UART_ICR: *mut u32 = (UART_BASE + 0x044) as *mut u32;

const UART_FR_TXFF: u32 = 1 << 5;
const UART_CR_UARTEN: u32 = 1 << 0;
const UART_CR_TXE: u32 = 1 << 8;
const UART_CR_RXE: u32 = 1 << 9;
const UART_LCRH_FEN: u32 = 1 << 4;
const UART_LCRH_WLEN_8: u32 = 3 << 5;

/// Initializes the PL011 UART for polled serial output.
fn uart_init() {
    unsafe {
        UART_CR.write_volatile(0);

        UART_ICR.write_volatile(0x7ff);

        UART_IBRD.write_volatile(1);
        UART_FBRD.write_volatile(40);

        UART_LCRH.write_volatile(UART_LCRH_WLEN_8 | UART_LCRH_FEN);
        UART_IMSC.write_volatile(0);
        UART_CR.write_volatile(UART_CR_UARTEN | UART_CR_TXE | UART_CR_RXE);
    }
}

/// Sends one byte through the UART transmit FIFO.
fn uart_putc(c: u8) {
    unsafe {
        while UART_FR.read_volatile() & UART_FR_TXFF != 0 {}
        UART_DR.write_volatile(c as u32);
    }
}

/// Prints a UTF-8 string as raw bytes over the boot UART.
fn kprint(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            uart_putc(b'\r');
        }
        uart_putc(b);
    }
}

/// Rust-side kernel entry point called after `_start` has installed a stack.
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    uart_init();
    kprint("steelray: booted\n");

    loop {
        core::hint::spin_loop();
    }
}

/// Stops the kernel after reporting a panic on the boot UART.
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    kprint("steelray: panic\n");

    loop {
        core::hint::spin_loop();
    }
}
