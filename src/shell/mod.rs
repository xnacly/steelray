use core::fmt::Write;

use crate::{
    io, kprintln, println,
    shell::linebuf::{Buf, PushResult},
    uart::Uart,
};

mod linebuf;

const PROMPT: &str = "xsh> ";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";

struct Command {
    name: &'static str,
    help: &'static str,
    run: fn(&Xsh, &str),
}

struct Control {
    byte: u8,
    run: fn(&Xsh, &Buf),
}

const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        help: "list available commands",
        run: |_xsh, _args| {
            for command in COMMANDS {
                println!("{:<8} {}", command.name, command.help);
            }
        },
    },
    Command {
        name: "time",
        help: "show timer state",
        run: |_xsh, _args| {
            println!(
                "secs={},cntvct_el0={},cntfrq_el0={}",
                io::time::time_sec_frac().0,
                io::aarch64::cntvct_el0(),
                io::aarch64::cntfrq_el0()
            );
        },
    },
    Command {
        name: "shutdown",
        help: "power off via PSCI SYSTEM_OFF",
        run: |_xsh, _args| {
            kprintln!("got shutdown, requesting PSCI_SYSTEM_OFF");
            io::psci::system_off();
        },
    },
    Command {
        name: "uart",
        help: "show PL011 UART registers",
        run: |_xsh, _args| {
            let fr = Uart::fr();
            let cr = Uart::cr();
            let lcrh = Uart::lcrh();

            println!("base={:#010x} MMIO base address", Uart::UART_BASE);
            println!("fr={:#010x} flag register", fr);
            println!(
                "  busy={} transmitter is still sending data",
                fr & Uart::UART_FR_BUSY != 0,
            );
            println!(
                "  rxfe={} receive FIFO is empty",
                fr & Uart::UART_FR_RXFE != 0,
            );
            println!(
                "  rxff={} receive FIFO is full",
                fr & Uart::UART_FR_RXFF != 0,
            );
            println!(
                "  txfe={} transmit FIFO is empty",
                fr & Uart::UART_FR_TXFE != 0,
            );
            println!(
                "  txff={} transmit FIFO is full",
                fr & Uart::UART_FR_TXFF != 0
            );
            println!("ibrd={:#010x} integer baud-rate divisor", Uart::ibrd());
            println!("fbrd={:#010x} fractional baud-rate divisor", Uart::fbrd());
            println!("lcrh={:#010x} line control register", lcrh);
            println!(
                "  fen={} FIFOs are enabled",
                lcrh & Uart::UART_LCRH_FEN != 0,
            );
            println!("  wlen={} word length selector", (lcrh >> 5) & 0b11);
            println!("cr={:#010x} control register", cr);
            println!(
                "  uarten={} UART is enabled",
                cr & Uart::UART_CR_UARTEN != 0,
            );
            println!(
                "  txe={} transmitter is enabled",
                cr & Uart::UART_CR_TXE != 0,
            );
            println!("  rxe={} receiver is enabled", cr & Uart::UART_CR_RXE != 0);
            println!(
                "imsc={:#010x} interrupt mask set/clear register",
                Uart::imsc()
            );
        },
    },
    Command {
        name: "version",
        help: "show build and kernel version information",
        run: |_xsh, _args| {
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
        },
    },
];

const CONTROLS: &[Control] = &[Control {
    byte: 0x0c,
    run: Xsh::ctrl_clear_screen,
}];

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

    fn write_prompt(&self) {
        Uart {}.write_str(PROMPT).unwrap();
    }

    fn redraw_line(&self, buf: &Buf) {
        self.write_prompt();
        Uart {}
            .write_str(core::str::from_utf8(buf.as_slice()).unwrap_or(""))
            .unwrap();
    }

    fn ctrl_clear_screen(&self, buf: &Buf) {
        Uart {}.write_str(CLEAR_SCREEN).unwrap();
        self.redraw_line(buf);
    }

    fn handle_control(&self, b: u8, buf: &Buf) -> bool {
        for control in CONTROLS {
            if control.byte == b {
                (control.run)(self, buf);
                return true;
            }
        }

        false
    }

    pub fn dispatch(&self, line: &str) -> Result<(), XshError> {
        let mut split = line.split_ascii_whitespace();
        let Some(cmd) = split.nth(0) else {
            return Ok(());
        };

        for command in COMMANDS {
            if command.name == cmd {
                let args = line[cmd.len()..].trim_start();
                (command.run)(self, args);
                return Ok(());
            }
        }

        Err(XshError::UnkownCommand)
    }

    pub fn run(&mut self) -> Result<(), XshError> {
        let mut buf: Buf = Buf::new();
        self.write_prompt();
        loop {
            let b = Uart::getc();
            if self.handle_control(b, &buf) {
                continue;
            }

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
                    if let Err(err) = self.dispatch(line) {
                        println!("shell error: {:?}", err);
                    }
                    buf.clear();
                    self.write_prompt();
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
