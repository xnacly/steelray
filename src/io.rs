//! basic input output directives
use core::fmt::{self, Write};

use crate::uart;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::io::kprint(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($fmt:expr) => ($crate::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::kprint!(concat!($fmt, "\n"), $($arg)*)
    );
}

fn time_sec_frac() -> (u64, u32) {
    let v = cntvct_el0();
    let f = cntfrq_el0();

    let secs = v / f;
    let rem = v % f;

    let micros = (rem * 1_000_000) / f;

    (secs, micros as u32)
}

pub fn kprint(args: fmt::Arguments) {
    let mut uart = uart::Uart;

    let (sec, usec) = time_sec_frac();
    let _ = write!(uart, "[{:5}.{:06}] ", sec, usec);
    let _ = uart.write_fmt(args);
}

/// codegen for wrapping fetches of aarch64 system registers
macro_rules! sysreg {
    ($(#[$meta:meta])* $r:ident) => {
        #[doc = concat!("fetches the raw value of the ", stringify!($r), " aarch64 register")]
        $(#[$meta])*
        #[inline(always)]
        pub fn $r() -> u64 {
            let val: u64;
            unsafe {
                core::arch::asm!(concat!("mrs {}, ", stringify!($r)), out(reg) val);
            }
            val
        }
    };
}

sysreg! {
    /// see: https://developer.arm.com/documentation/ddi0601/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register
    cntvct_el0
}
sysreg! {
    /// see: https://developer.arm.com/documentation/ddi0601/2020-12/AArch64-Registers/CNTFRQ-EL0--Counter-timer-Frequency-register
    cntfrq_el0
}
