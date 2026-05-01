#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let p = 0x40100000 as *mut u64;
    unsafe {
        *p = 0xDEADAFFE;
    }

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
