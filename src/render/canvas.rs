use crate::constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};

use super::{CCell, Style};

pub struct Canvas {
    pub buffer: Vec<Vec<CCell>>,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let w = width as usize;
        let h = height as usize;
        Self {
            buffer: vec![
                vec![
                    CCell { 
                        char: ' ', 
                        style: Style::default() 
                    }; 
                    w]; 
                h],
            width,
            height,
        }
    }
    
    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = CCell::default();
            }
        }
    }
}

