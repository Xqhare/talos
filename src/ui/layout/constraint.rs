/// A constraint for a layout
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
