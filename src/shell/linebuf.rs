pub struct Buf {
    buf: [u8; 128],
    len: usize,
}

impl Buf {
    pub fn new() -> Self {
        Buf {
            buf: [const { 0x0 as u8 }; 128],
            len: 0,
        }
    }

    pub fn push(&mut self, b: u8) {}
}
