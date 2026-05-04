pub type Buf = LineBuf<128>;

pub struct LineBuf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

pub enum PushResult<'a> {
    Reading,
    Complete(&'a [u8]),
    Full,
}

impl<const N: usize> LineBuf<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn push(&mut self, b: u8) -> PushResult<'_> {
        match b {
            b'\r' | b'\n' => PushResult::Complete(&self.buf[..self.len]),
            0x08 | 0x7f => {
                if self.len > 0 {
                    self.len -= 1;
                }
                PushResult::Reading
            }
            _ if self.len == N => PushResult::Full,
            _ => {
                self.buf[self.len] = b;
                self.len += 1;
                PushResult::Reading
            }
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
