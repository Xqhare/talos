use crate::layout::{Constraint, Direction, Layout};

/// A builder for a Layout
///
/// # Example
/// ```rust
/// use talos::{
///     layout::{Constraint, Direction},
///     LayoutBuilder,
/// };
///
/// let layout = LayoutBuilder::new()
///     .direction(Direction::Horizontal)
///     .margin(1)
///     .add_constraint(Constraint::Percentage(50))
///     .add_constraint(Constraint::Percentage(50))
///     .build();
/// ```
#[must_use]
pub struct LayoutBuilder {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: u16,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutBuilder {
    /// Creates a new `LayoutBuilder`
    ///
    /// # Example
    /// ```
    /// use talos::builder::LayoutBuilder;
    ///
    /// let layout_builder = LayoutBuilder::new();
    /// ```
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {
            direction: Direction::Horizontal,
            constraints: Vec::new(),
            margin: 0,
        }
    }
    /// Sets the direction of the layout
    ///
    /// # Example
    /// ```rust
    /// use talos::{layout::Direction, LayoutBuilder};
    ///
    /// let mut builder = LayoutBuilder::new();
    /// builder.direction(Direction::Vertical);
    /// ```
    pub fn direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }
    /// Sets the margin of the layout
    ///
    /// # Example
    /// ```rust
    /// use talos::LayoutBuilder;
    ///
    /// let mut builder = LayoutBuilder::new();
    /// builder.margin(1);
    /// ```
    pub fn margin(&mut self, margin: u16) -> &mut Self {
        self.margin = margin;
        self
    }

    /// Adds a constraint to the layout - there is no limit
    ///
    /// # Example
    /// ```rust
    /// use talos::{layout::Constraint, LayoutBuilder};
    ///
    /// let mut builder = LayoutBuilder::new();
    /// builder.add_constraint(Constraint::Percentage(50));
    /// ```
    pub fn add_constraint(&mut self, constraint: Constraint) -> &mut Self {
        self.constraints.push(constraint);
        self
    }

    /// Builds the layout
    ///
    /// # Example
    /// ```rust
    /// use talos::LayoutBuilder;
    ///
    /// let layout = LayoutBuilder::new().build();
    /// ```
    pub fn build(&self) -> Layout {
        Layout::new(self.direction, self.constraints.clone(), self.margin)
    }
}
