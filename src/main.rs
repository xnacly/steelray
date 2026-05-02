#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod io;
mod uart;

global_asm!(include_str!("./boot.s"));

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    uart::Uart::init();
    kprintln!("initialised uart");
    kprintln!(
        "cntvct_el0={},cntfrq_el0={}",
        io::cntvct_el0(),
        io::cntfrq_el0()
    );
    kprintln!("entering spin loop");

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    kprintln!("panic");

    loop {
        core::hint::spin_loop();
    }
}
