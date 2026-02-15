use std::cmp::{max, min};

/// A rectangle (x, y, width, height)
///
/// x and y are the coordinates of the top left corner of the rectangle
///
/// Coordinates of a rectangle are stored as relative to the top left corner (1,1)
///
/// # Example
/// ```rust
/// use talos::layout::Rect;
///
/// let rect = Rect {
///    x: 10,
///    y: 20,
///    width: 30,
///    height: 40,
/// };
/// ```
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
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(0, 0, 10, 20);
    /// assert_eq!(rect.area(), 200);
    /// ```
    #[must_use]
    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    /// Returns the x-coordinate of the left edge
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert_eq!(rect.left(), 10);
    /// ```
    #[must_use]
    pub fn left(self) -> u16 {
        self.x
    }

    /// Returns the x-coordinate of the right edge (x + width)
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert_eq!(rect.right(), 40);
    /// ```
    #[must_use]
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Returns the y-coordinate of the top edge
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert_eq!(rect.top(), 20);
    /// ```
    #[must_use]
    pub fn top(self) -> u16 {
        self.y
    }

    /// Returns the y-coordinate of the bottom edge (y + height)
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert_eq!(rect.bottom(), 60);
    /// ```
    #[must_use]
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Returns true if the given point is inside the rectangle
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect = Rect::new(10, 20, 30, 40);
    /// assert!(rect.contains(15, 25));
    /// ```
    #[must_use]
    pub fn contains(self, x: u16, y: u16) -> bool {
        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }

    /// Returns true if this rectangle intersects with another
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect1 = Rect::new(10, 20, 30, 40);
    /// let rect2 = Rect::new(20, 30, 30, 40);
    /// assert!(rect1.intersects(rect2));
    /// ```
    #[must_use]
    pub fn intersects(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    /// Returns the intersection of two rectangles, or `Rect::default()` if they don't intersect
    ///
    /// # Example
    /// ```rust
    /// use talos::layout::Rect;
    ///
    /// let rect1 = Rect::new(10, 20, 30, 40);
    /// let rect2 = Rect::new(20, 30, 30, 40);
    /// let intersection = rect1.intersection(rect2);
    /// assert_eq!(intersection, Rect::new(20, 30, 20, 30));
    /// ```
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
