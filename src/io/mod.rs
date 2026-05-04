//! basic input output directives
use crate::uart;
use core::fmt::{self, Write};

pub mod aarch64;
pub mod psci;
pub mod time;

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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    );
}

pub fn print(args: fmt::Arguments) {
    let mut uart = uart::Uart;
    let _ = uart.write_fmt(args);
}
