
use super::CCell;

pub struct Canvas {
    pub buffer: Vec<CCell>,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = make_default_buffer(width, height);
        Self {
            buffer,
            width,
            height,
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer = make_default_buffer(self.width, self.height);
    }

    pub fn get_ccell(&self, x: u16, y: u16) -> CCell {
        self.buffer[(x + y * self.width) as usize]
    }

    pub fn get_mut_ccell(&mut self, x: u16, y: u16) -> &mut CCell {
        &mut self.buffer[(x + y * self.width) as usize]
    }

    pub fn set_ccell(&mut self, x: u16, y: u16, cell: CCell) {
        self.buffer[(x + y * self.width) as usize] = cell;
    }
}

fn make_default_buffer(width: u16, height: u16) -> Vec<CCell> {
    vec![CCell::default(); width as usize * height as usize]
}
