use crate::layout::Rect;

use super::CCell;

pub struct Canvas {
    pub buffer: Vec<CCell>,
    width: u16,
    height: u16,
    last_set_cell: Option<(u16, u16)>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = make_default_buffer(width, height);
        Self {
            buffer,
            width,
            height,
            last_set_cell: None,
        }
    }

    pub fn max_height(&self) -> u16 {
        self.height.saturating_sub(1)
    }

    pub fn max_width(&self) -> u16 {
        self.width.saturating_sub(1)
    }

    pub fn size_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.buffer.fill(CCell::default());
    }

    /// Safely gets a cell. Returns default (space) if out of bounds.
    pub fn get_ccell(&self, x: u16, y: u16) -> CCell {
        if x >= self.width || y >= self.height {
            return CCell::default();
        }
        self.buffer[(x + y * self.width) as usize]
    }

    /// Unsafe access for performance-critical loops (like internal renderers)
    /// Panics if out of bounds.
    ///
    /// Also sets the last set cell
    pub fn get_mut_ccell(&mut self, x: u16, y: u16) -> &mut CCell {
        self.last_set_cell = Some((x, y));
        &mut self.buffer[(x + y * self.width) as usize]
    }

    /// Safely sets a cell. Ignores the command if coordinates are out of bounds (Clipping).
    ///
    /// Also sets the last set cell
    pub fn set_ccell(&mut self, x: u16, y: u16, cell: CCell) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.last_set_cell = Some((x, y));
        self.buffer[(x + y * self.width) as usize] = cell;
    }

    /// Returns the coordinate of the last set cell (x, y)
    /// Returns None if no cell has been set
    pub fn last_cell(&self) -> Option<(u16, u16)> {
        self.last_set_cell
    }
}

fn make_default_buffer(width: u16, height: u16) -> Vec<CCell> {
    vec![CCell::default(); width as usize * height as usize]
}
