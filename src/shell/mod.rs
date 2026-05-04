use core::fmt::Write;

use crate::{
    io, kprintln, println,
    shell::linebuf::{Buf, PushResult},
    uart::Uart,
};

mod linebuf;

const PROMPT: &str = "xsh> ";

/// x shell :O
pub struct Xsh;

#[derive(Debug)]
pub enum XshError {
    UnkownCommand,
    UnexpectedEof,
}

impl Xsh {
    pub fn new() -> Self {
        Xsh
    }

    fn print_version(&self) {
        let (sec, usec) = io::time::time_sec_frac();

        println!(
            "{} {}",
            option_env!("CARGO_PKG_NAME").unwrap_or("steelray"),
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
        );
        println!(
            "rustc: {}",
            option_env!("STEELRAY_RUSTC_VERSION").unwrap_or("unknown")
        );
        println!(
            "profile: {}",
            option_env!("STEELRAY_BUILD_PROFILE").unwrap_or("unknown")
        );
        println!(
            "target: {}",
            option_env!("STEELRAY_BUILD_TARGET").unwrap_or("unknown")
        );
        println!(
            "host: {}",
            option_env!("STEELRAY_BUILD_HOST").unwrap_or("unknown")
        );
        println!(
            "git commit: {}",
            option_env!("STEELRAY_GIT_COMMIT").unwrap_or("unknown")
        );
        println!(
            "git dirty: {}",
            option_env!("STEELRAY_GIT_DIRTY").unwrap_or("unknown")
        );
        println!(
            "timer: uptime={}.{:06}, cntvct_el0={}, cntfrq_el0={}",
            sec,
            usec,
            io::aarch64::cntvct_el0(),
            io::aarch64::cntfrq_el0()
        );
    }

    pub fn dispatch(&self, line: &str) -> Result<(), XshError> {
        let mut split = line.split_ascii_whitespace();
        let Some(cmd) = split.nth(0) else {
            return Err(XshError::UnexpectedEof);
        };

        match cmd {
            "version" => self.print_version(),
            "time" => println!(
                "secs={},cntvct_el0={},cntfrq_el0={}",
                io::time::time_sec_frac().0,
                io::aarch64::cntvct_el0(),
                io::aarch64::cntfrq_el0()
            ),
            _ => return Err(XshError::UnkownCommand),
        };

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), XshError> {
        let mut buf: Buf = Buf::new();
        Uart {}.write_str(PROMPT).unwrap();
        loop {
            let b = Uart::getc();
            let pushed = buf.push(b);
            match pushed {
                PushResult::Reading => {
                    if b == 0x08 || b == 0x7f {
                        Uart::putc(0x08);
                        Uart::putc(b' ');
                        Uart::putc(0x08);
                    } else {
                        Uart::putc(b);
                    }
                }
                PushResult::Complete(line) => {
                    Uart::putc(b'\r');
                    Uart::putc(b'\n');
                    let line = core::str::from_utf8(line).unwrap_or("<invalid utf-8>");
                    self.dispatch(line);
                    buf.clear();
                    Uart {}.write_str(PROMPT).unwrap();
                }
                PushResult::Full => {
                    Uart::putc(b'\r');
                    Uart::putc(b'\n');
                    kprintln!("input line too long");
                    buf.clear();
                }
            }
        }
    }
}
