#![no_std]
#![no_main]

use crate::uart::Uart;
use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

mod uart;

global_asm!(include_str!("./boot.s"));

/// Prints a UTF-8 string as raw bytes over the boot UART.
pub fn kprint(s: &str) {
    Uart {}.write_str(s).unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    Uart::init();
    kprint("initialised\n");

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    kprint("panic\n");

    loop {
        core::hint::spin_loop();
    }
}
