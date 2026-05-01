#![no_std]
#![no_main]

use core::panic::PanicInfo;

const UARTDR: *mut u32 = 0x09000000 as *mut u32;
const UARTCR: *mut u32 = 0x09000030 as *mut u32;
const UARTLCRH: *mut u32 = 0x0900002C as *mut u32;

fn uart_init() {
    unsafe {
        UARTCR.write_volatile(0);
        UARTLCRH.write_volatile(1 << 4);
        UARTCR.write_volatile((1 << 0) | (1 << 8) | (1 << 9));
    }
}

fn uart_putc(c: u8) {
    unsafe {
        UARTDR.write_volatile(c as u32);
    }
}

fn kprint(s: &str) {
    for b in s.bytes() {
        uart_putc(b)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // uart_init();
    // kprint("i want to kill myself");
    let p = 0x40000000 as *mut u64;
    unsafe {
        *p = 0xDEADAFFE;
    }
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
