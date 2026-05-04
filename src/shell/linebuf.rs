pub struct Buf {
    buf: [u8; 128],
    len: usize,
}

pub enum BufState {
    // TODO: how to represent "owned" bytes
    Filled(()),
    Reading,
}

impl Buf {
    pub fn new() -> Self {
        Buf {
            buf: [const { 0x0 as u8 }; 128],
            len: 0,
        }
    }

    pub fn push(&mut self, b: u8) -> BufState {
        if self.len == 128 || b == b'\n' {
            return BufState::Filled(());
        }

        self.buf[self.len] = b;
        self.len += 1;
        BufState::Reading
    }
}
