use crate::layout::Rect;

use super::CCell;

/// A canvas is a 2D array of [`CCell`](struct.CCell.html)s
///
/// # Example
/// ```rust
/// use talos::render::Canvas;
///
/// let canvas = Canvas::new(10, 20);
/// ```
pub struct Canvas {
    /// A 2D array of [`CCell`](struct.CCell.html)s
    pub buffer: Vec<CCell>,
    width: u16,
    height: u16,
    last_set_cell: Option<(u16, u16)>,
}

impl Canvas {
    /// Creates a new canvas
    ///
    /// # Arguments
    /// * `width` - The width of the canvas
    /// * `height` - The height of the canvas
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Canvas;
    ///
    /// let canvas = Canvas::new(10, 20);
    /// ```
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let buffer = make_default_buffer(width, height);
        Self {
            buffer,
            width,
            height,
            last_set_cell: None,
        }
    }

    /// Returns the height of the canvas
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Canvas;
    ///
    /// let canvas = Canvas::new(10, 20);
    /// assert_eq!(canvas.max_height(), 19);
    /// ```
    #[must_use]
    pub fn max_height(&self) -> u16 {
        self.height.saturating_sub(1)
    }

    /// Returns the width of the canvas
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Canvas;
    ///
    /// let canvas = Canvas::new(10, 20);
    /// assert_eq!(canvas.max_width(), 9);
    /// ```
    #[must_use]
    pub fn max_width(&self) -> u16 {
        self.width.saturating_sub(1)
    }

    /// Returns the size of the canvas
    ///
    /// # Example
    /// ```rust
    /// use talos::{layout::Rect, render::Canvas};
    ///
    /// let canvas = Canvas::new(10, 20);
    /// assert_eq!(canvas.size_rect(), Rect::new(0, 0, 10, 20));
    /// ```
    #[must_use]
    pub fn size_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    /// Clears the canvas
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Canvas;
    ///
    /// let mut canvas = Canvas::new(10, 20);
    /// canvas.clear();
    /// ```
    pub fn clear(&mut self) {
        self.buffer.fill(CCell::default());
    }

    /// Safely gets a cell. Returns default (space) if out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{CCell, Canvas};
    ///
    /// let canvas = Canvas::new(10, 20);
    /// let cell = canvas.get_ccell(5, 10);
    /// assert_eq!(cell, CCell::default());
    /// ```
    #[must_use]
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
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{CCell, Canvas};
    ///
    /// let mut canvas = Canvas::new(10, 20);
    /// let cell = canvas.get_mut_ccell(5, 10);
    /// *cell = CCell::default();
    /// ```
    pub fn get_mut_ccell(&mut self, x: u16, y: u16) -> &mut CCell {
        self.last_set_cell = Some((x, y));
        &mut self.buffer[(x + y * self.width) as usize]
    }

    /// Safely sets a cell. Ignores the command if coordinates are out of bounds (Clipping).
    ///
    /// Also sets the last set cell
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{CCell, Canvas};
    ///
    /// let mut canvas = Canvas::new(10, 20);
    /// canvas.set_ccell(5, 10, CCell::default());
    /// ```
    pub fn set_ccell(&mut self, x: u16, y: u16, cell: CCell) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.last_set_cell = Some((x, y));
        self.buffer[(x + y * self.width) as usize] = cell;
    }

    /// Returns the coordinate of the last set cell (x, y)
    /// Returns None if no cell has been set
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{CCell, Canvas};
    ///
    /// let mut canvas = Canvas::new(10, 20);
    /// canvas.set_ccell(5, 10, CCell::default());
    /// assert_eq!(canvas.last_cell(), Some((5, 10)));
    /// ```
    #[must_use]
    pub fn last_cell(&self) -> Option<(u16, u16)> {
        self.last_set_cell
    }
}

fn make_default_buffer(width: u16, height: u16) -> Vec<CCell> {
    vec![CCell::default(); width as usize * height as usize]
}
