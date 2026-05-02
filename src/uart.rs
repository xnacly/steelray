use core::fmt::{self, Write};

pub struct Uart;

impl Uart {
    // QEMU -machine virt exposes a PL011 UART, -nographic (or -serial) wires it to the terminal.
    pub const UART_BASE: usize = 0x0900_0000;
    pub const UART_DR: *mut u32 = (Self::UART_BASE + 0x000) as *mut u32;
    pub const UART_FR: *const u32 = (Self::UART_BASE + 0x018) as *const u32;
    pub const UART_IBRD: *mut u32 = (Self::UART_BASE + 0x024) as *mut u32;
    pub const UART_FBRD: *mut u32 = (Self::UART_BASE + 0x028) as *mut u32;
    pub const UART_LCRH: *mut u32 = (Self::UART_BASE + 0x02c) as *mut u32;
    pub const UART_CR: *mut u32 = (Self::UART_BASE + 0x030) as *mut u32;
    pub const UART_IMSC: *mut u32 = (Self::UART_BASE + 0x038) as *mut u32;
    pub const UART_ICR: *mut u32 = (Self::UART_BASE + 0x044) as *mut u32;
    pub const UART_FR_TXFF: u32 = 1 << 5;
    pub const UART_CR_UARTEN: u32 = 1 << 0;
    pub const UART_CR_TXE: u32 = 1 << 8;
    pub const UART_CR_RXE: u32 = 1 << 9;
    pub const UART_LCRH_FEN: u32 = 1 << 4;
    pub const UART_LCRH_WLEN_8: u32 = 3 << 5;

    /// Initializes the PL011 UART for polled serial output.
    pub fn init() {
        unsafe {
            Self::UART_CR.write_volatile(0);

            Self::UART_ICR.write_volatile(0x7ff);

            Self::UART_IBRD.write_volatile(1);
            Self::UART_FBRD.write_volatile(40);

            Self::UART_LCRH.write_volatile(Self::UART_LCRH_WLEN_8 | Self::UART_LCRH_FEN);
            Self::UART_IMSC.write_volatile(0);
            Self::UART_CR
                .write_volatile(Self::UART_CR_UARTEN | Self::UART_CR_TXE | Self::UART_CR_RXE);
        }
    }

    /// Sends one character through the UART transmit FIFO
    pub fn putc(c: char) {
        unsafe {
            while Self::UART_FR.read_volatile() & Self::UART_FR_TXFF != 0 {}
            Self::UART_DR.write_volatile(c as u32);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            Uart::putc(b as char);
        }
        Ok(())
    }
}
