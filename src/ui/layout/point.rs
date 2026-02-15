/// A point (x, y)
///
/// # Example
/// ```rust
/// use talos::layout::Point;
///
/// let point = Point { x: 10, y: 20 };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    /// The x-coordinate
    pub x: u16,
    /// The y-coordinate
    pub y: u16,
}

impl Point {
    /// Creates a new point
    ///
    /// # Arguments
    /// * `x` - The x-coordinate
    /// * `y` - The y-coordinate
    ///
    /// # Example
    /// ```
    /// use talos::ui::layout::Point;
    ///
    /// let point = Point::new(1, 1);
    /// assert_eq!(point.x, 1);
    /// assert_eq!(point.y, 1);
    /// ```
    #[must_use]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
