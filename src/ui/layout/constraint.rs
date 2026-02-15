/// A constraint for a layout
///
/// # Example
/// ```rust
/// use talos::layout::Constraint;
///
/// let length_constraint = Constraint::Length(10);
/// let percentage_constraint = Constraint::Percentage(50);
/// let min_constraint = Constraint::Min(5);
/// let ratio_constraint = Constraint::Ratio(1, 2);
/// let max_constraint = Constraint::Max(20);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    /// Constrains to a specific length
    Length(u16),
    /// Constrains to a percentage of the available space
    Percentage(u16),
    /// Constrains to a minimum length
    Min(u16),
    /// Constrains to a ratio of the available space
    Ratio(u32, u32),
    /// Constrains to a maximum length
    Max(u16),
}
