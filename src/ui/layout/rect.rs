use std::cmp::{max, min};

/// A rectangle (x, y, width, height)
///
/// x and y are the coordinates of the top left corner of the rectangle
///
/// Coordinates of a rectangle are stored as relative to the top left corner (1,1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    /// The x-coordinate of the top left corner
    pub x: u16,
    /// The y-coordinate of the top left corner
    pub y: u16,
    /// The width of the rectangle
    pub width: u16,
    /// The height of the rectangle
    pub height: u16,
}

impl Rect {
    /// Creates a new rectangle
    ///
    /// # Arguments
    /// * `x` - The x-coordinate of the top left corner
    /// * `y` - The y-coordinate of the top left corner
    /// * `width` - The width of the rectangle
    /// * `height` - The height of the rectangle
    ///
    /// # Example
    /// ```
    /// use talos::ui::layout::Rect;
    ///
    /// let rect = Rect::new(1, 1, 10, 10);
    /// assert_eq!(rect.x, 1);
    /// assert_eq!(rect.y, 1);
    /// assert_eq!(rect.width, 10);
    /// assert_eq!(rect.height, 10);
    /// ```
    #[must_use]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the area of the rectangle
    #[must_use]
    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    /// Returns the x-coordinate of the left edge
    #[must_use]
    pub fn left(self) -> u16 {
        self.x
    }

    /// Returns the x-coordinate of the right edge (x + width)
    #[must_use]
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the y-coordinate of the top edge
    #[must_use]
    pub fn top(self) -> u16 {
        self.y
    }

    /// Returns the y-coordinate of the bottom edge (y + height)
    #[must_use]
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns true if the given point is inside the rectangle
    #[must_use]
    pub fn contains(self, x: u16, y: u16) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    /// Returns true if this rectangle intersects with another
    #[must_use]
    pub fn intersects(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    /// Returns the intersection of two rectangles, or `Rect::default()` if they don't intersect
    #[must_use]
    pub fn intersection(self, other: Rect) -> Rect {
        let x1 = max(self.left(), other.left());
        let y1 = max(self.top(), other.top());
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());

        if x2 > x1 && y2 > y1 {
            Rect {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
            }
        } else {
            Rect::default()
        }
    }
}
