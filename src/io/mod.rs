//! basic input output directives
use crate::uart;
use core::fmt::{self, Write};

pub mod aarch64;
mod time;

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

pub fn kprint(args: fmt::Arguments) {
    let mut uart = uart::Uart;

    let (sec, usec) = time::time_sec_frac();
    let _ = write!(uart, "[{:5}.{:06}] ", sec, usec);
    let _ = uart.write_fmt(args);
}
