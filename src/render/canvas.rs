use crate::constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};

pub struct Canvas {
    // The buffer described in your README
    pub buffer: Vec<u8>, 
}

impl Canvas {
    pub fn new() -> Self {
        Self { buffer: Vec::with_capacity(4096) }
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
        // Maybe write the "Clear Screen" ANSI code into the buffer here
        self.buffer.extend_from_slice(CLEAR_ALL.as_bytes());
        self.buffer.extend_from_slice(TO_TOP_LEFT.as_bytes());
    }
}

// Allow widgets to write directly into the canvas
impl std::io::Write for Canvas {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}
