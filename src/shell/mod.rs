use crate::{shell::linebuf::Buf, uart::Uart};

mod linebuf;

/// x shell :O
pub struct Xsh {
    buf: Buf,
}

#[derive(Debug)]
pub enum XshError {
    BadInput,
}

impl Xsh {
    pub fn new() -> Self {
        Xsh { buf: Buf::new() }
    }

    pub fn start(&mut self) -> Result<(), XshError> {
        loop {
            let b = Uart::getc();
            if b == b'\r' {
                Uart::putc(b'\r');
                Uart::putc(b'\n');
                return Err(XshError::BadInput);
            } else {
                Uart::putc(b);
            }
        }
    }
}
