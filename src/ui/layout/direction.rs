/// The direction of a layout
///
/// # Example
/// ```rust
/// use talos::layout::Direction;
///
/// let vertical = Direction::Vertical;
/// let horizontal = Direction::Horizontal;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Vertical layout direction
    Vertical,
    /// Horizontal layout direction
    Horizontal,
}
