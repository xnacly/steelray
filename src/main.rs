#![no_std]
#![no_main]
#![allow(unused)]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod io;
mod shell;
mod uart;

global_asm!(include_str!("./boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    uart::Uart::init();
    kprintln!("initialised uart");
    kprintln!(
        "cntvct_el0={},cntfrq_el0={}",
        io::aarch64::cntvct_el0(),
        io::aarch64::cntfrq_el0()
    );
    kprintln!("entering shell");

    let mut xsh = shell::Xsh::new();
    if let Err(err) = xsh.run() {
        kprintln!("shell error ({:?}), entering spin loop", err)
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("panic: {}", info.message());

    loop {
        core::hint::spin_loop();
    }
}
