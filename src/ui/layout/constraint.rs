#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    Length(u16),
    Percentage(u16),
    Min(u16),
    Ratio(u32, u32),
    Max(u16),
}
