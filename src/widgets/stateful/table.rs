use crate::{
    LayoutBuilder,
    layout::{Constraint, Direction, Layout, Rect},
    render::{CCell, Canvas, Style},
    widgets::traits::Widget,
};

/// A stateful widget that displays a scrollable table of items.
///
/// The `Table` widget can be used to display a scrollable table of items. The state of the table is
/// managed by a `TableState` struct, which must be passed to the `with_state` method.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     input::{Event, KeyCode, KeyEvent},
///     layout::Rect,
///     widgets::{
///         stateful::{Table, TableState},
///         Text,
///         traits::Widget,
///     },
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///     let (canvas, thoth) = talos.render_ctx();
///
///     let mut table_state = TableState {
///         x_offset: 0,
///         y_offset: 0,
///         max_rows: None,
///         max_columns: None,
///     };
///
///     let rows = vec![
///         vec![
///             Box::new(Text::new("Row 1, Col 1", thoth)) as Box<dyn Widget>,
///             Box::new(Text::new("Row 1, Col 2", thoth)) as Box<dyn Widget>,
///         ],
///         vec![
///             Box::new(Text::new("Row 2, Col 1", thoth)) as Box<dyn Widget>,
///             Box::new(Text::new("Row 2, Col 2", thoth)) as Box<dyn Widget>,
///         ],
///     ];
///
///     let mut table = Table::new(&mut table_state)
///         .with_rows(rows)
///         .with_row_height(2);
///
///     let rect = Rect::new(0, 0, 40, 10);
///     table.render(canvas, rect, thoth);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[must_use]
#[allow(clippy::struct_excessive_bools)]
pub struct Table<'a> {
    state: &'a mut TableState,
    rows: Vec<Vec<Box<dyn Widget + 'a>>>,
    alternate_colour_vertically: bool,
    alternate_colour_horizontally: bool,
    style: Style,
    alternate_style: Style,
    border_style: Style,
    header_style: Style,
    header_row: Option<usize>,
    col_layout: Option<Layout>,
    row_height: Option<u16>,
    draw_outer_border: bool,
    draw_inner_border: InnerBorder,
}

/// The state of the table
///
/// # Example
/// ```rust,no_run
/// use talos::{Talos, widgets::{stateful::{Table, TableState}, Text}};
///
/// let mut talos = Talos::builder().build().unwrap();
/// let (_, thoth) = talos.render_ctx();
/// let table_state = TableState {
///     x_offset: 0,
///     y_offset: 0,
///     max_rows: None,
///     max_columns: None,
/// };
/// # assert!(true);
/// ```
#[derive(Default, Debug, Clone, Copy)]
pub struct TableState {
    /// The x offset of the table - used for scrolling
    pub x_offset: usize,
    /// The y offset of the table - used for scrolling
    pub y_offset: usize,
    /// The maximum number of rows to display at once
    ///
    /// If `None`, the table will try and fit all rows into the available space
    pub max_rows: Option<usize>,
    /// The maximum number of columns to display at once
    ///
    /// If `None`, the table will try and fit all columns into the available space
    pub max_columns: Option<usize>,
}

impl TableState {
    /// Creates a new TableState
    ///
    /// # Fields
    /// * `x_offset` - The x offset of the table - used for scrolling; Initalised to `0`.
    /// * `y_offset` - The y offset of the table - used for scrolling; Initalised to `0`
    /// * `max_rows` - The maximum number of rows to display at once; Initalised to `None`
    /// * `max_columns` - The maximum number of columns to display at once; Initalised to `None`
    pub fn new() -> Self {
        Self {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        }
    }
}

/// The inner border of the table
pub enum InnerBorder {
    /// All borders, between rows and columns
    All,
    /// Only borders between rows
    Rows,
    /// Only borders between columns
    Columns,
    /// No borders, Default
    None,
}

impl Default for InnerBorder {
    fn default() -> Self {
        Self::None
    }
}

impl<'a> Table<'a> {
    /// Creates a new, empty table
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Table, TableState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut state = TableState {
    ///     x_offset: 0,
    ///     y_offset: 0,
    ///     max_rows: None,
    ///     max_columns: None,
    /// };
    /// let table = Table::new(&mut state);
    /// # assert!(true);
    /// ```
    pub fn new(state: &'a mut TableState) -> Self {
        Self {
            state,
            rows: Vec::new(),
            alternate_colour_vertically: false,
            alternate_colour_horizontally: false,
            style: Style::default(),
            alternate_style: Style::default(),
            border_style: Style::default(),
            header_style: Style::default(),
            header_row: None,
            col_layout: None,
            row_height: None,
            draw_outer_border: false,
            draw_inner_border: InnerBorder::default(),
        }
    }

    /// Sets the layout of the columns
    ///
    /// Use this for fine grained control over how the columns are rendered.
    ///
    /// The `Direction` of the layout is ignored and always set to [`Direction::Horizontal`]
    pub fn with_col_layout(mut self, layout: Layout) -> Self {
        let mut layout = layout;
        layout.direction = Direction::Horizontal;
        self.col_layout = Some(layout);
        self
    }

    /// Sets the height of each row in the table
    ///
    /// If not set, the table will try and fit all rows into the available space
    pub fn with_row_height(mut self, height: u16) -> Self {
        self.row_height = Some(height);
        self
    }

    /// Sets the style of the table header
    ///
    /// To use, set `header_row` using [`Table::with_header_row`]
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Table, TableState}, render::{Style, Colour, Normal}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut state = TableState {
    ///     x_offset: 0,
    ///     y_offset: 0,
    ///     max_rows: None,
    ///     max_columns: None,
    /// };
    /// let table = Table::new(&mut state).with_header_style(Style::builder().set_fg(Colour::Normal(Normal::Red)).build());
    /// # assert!(true);
    /// ```
    pub fn with_header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }

    /// Sets the row to use as the table header
    ///
    /// Please provide a valid index of a row in the table
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Table, TableState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut state = TableState {
    ///     x_offset: 0,
    ///     y_offset: 0,
    ///     max_rows: None,
    ///     max_columns: None,
    /// };
    /// let table = Table::new(&mut state).with_header_row(0);
    /// # assert!(true);
    /// ```
    pub fn with_header_row(mut self, row: usize) -> Self {
        self.header_row = Some(row);
        self
    }

    /// Adds a row to the table
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{stateful::{Table, TableState}, Text, traits::Widget}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut table_state = TableState {
    ///     x_offset: 0,
    ///     y_offset: 0,
    ///     max_rows: None,
    ///     max_columns: None,
    /// };
    /// let row = vec![Box::new(Text::new("Hello", thoth)) as Box<dyn Widget>];
    /// let table = Table::new(&mut table_state)
    ///     .add_row(row);
    /// # assert!(true);
    /// ```
    pub fn add_row(mut self, row: Vec<Box<dyn Widget + 'a>>) -> Self {
        self.rows.push(row);
        self
    }

    /// Sets the rows of the table
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{stateful::{Table, TableState}, Text, traits::Widget}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut table_state = TableState {
    ///     x_offset: 0,
    ///     y_offset: 0,
    ///     max_rows: None,
    ///     max_columns: None,
    /// };
    /// let rows = vec![
    ///     vec![Box::new(Text::new("Hello", thoth)) as Box<dyn Widget>],
    ///     vec![Box::new(Text::new("World", thoth)) as Box<dyn Widget>],
    /// ];
    /// let table = Table::new(&mut table_state)
    ///     .with_rows(rows);
    /// # assert!(true);
    /// ```
    pub fn with_rows<I, R>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = Box<dyn Widget + 'a>>,
    {
        self.rows = rows.into_iter().map(|r| r.into_iter().collect()).collect();
        self
    }

    /// Sets the style of the table to be used when drawing the table and either
    /// `alternate_colour_vertically` or `alternate_colour_horizontally` is set to true.
    ///
    /// If both are `true`, the table will be drawn in a checkered pattern.
    pub fn with_alternate_style(mut self, style: Style) -> Self {
        self.alternate_style = style;
        self
    }

    /// Sets the style of the table border
    pub fn with_border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Makes the Table use alternating colouring vertically
    ///
    /// If this and `alternate_colour_horizontally` are both `true`, the table will be drawn in a checkered pattern
    pub fn alternate_colour_vertically(mut self) -> Self {
        self.alternate_colour_vertically = true;
        self
    }

    /// Makes the Table use alternating colouring horizontally
    ///
    /// If this and `alternate_colour_vertically` are both `true`, the table will be drawn in a checkered pattern
    pub fn alternate_colour_horizontally(mut self) -> Self {
        self.alternate_colour_horizontally = true;
        self
    }

    /// Draws a border around the table
    pub fn draw_outer_border(mut self) -> Self {
        self.draw_outer_border = true;
        self
    }

    /// Draws a border inside the table.
    ///
    /// Choose between `InnerBorder::All`, `InnerBorder::Rows` or `InnerBorder::Columns`
    ///
    /// - `InnerBorder::All`: All borders, between rows and columns
    /// - `InnerBorder::Rows`: Only borders between rows
    /// - `InnerBorder::Columns`: Only borders between columns
    /// - `InnerBorder::None`: No borders; Default
    pub fn draw_inner_border(mut self, border: InnerBorder) -> Self {
        self.draw_inner_border = border;
        self
    }

    /// Gets the state of the table
    pub fn get_state(&mut self) -> &mut TableState {
        &mut self.state
    }

    /// Returns a grid of `Rect`s representing the individual cells of the table.
    ///
    /// This method follows the same layout logic as [`Table::render`] but does not perform any drawing.
    pub fn inner(&self, area: Rect) -> Vec<Vec<Rect>> {
        let table_area = if self.draw_outer_border {
            Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            }
        } else {
            area
        };

        if table_area.width == 0 || table_area.height == 0 {
            return Vec::new();
        }

        let mut tmp = LayoutBuilder::new();
        let mut row_layout_builder = tmp.direction(Direction::Vertical);
        let row_amount = if let Some(max_rows) = self.state.max_rows {
            max_rows
        } else {
            std::cmp::min(self.rows.len(), table_area.height as usize)
        };

        if row_amount == 0 {
            return Vec::new();
        }

        let has_inner_row_border =
            matches!(self.draw_inner_border, InnerBorder::All | InnerBorder::Rows);

        for i in 0..row_amount {
            let constraint = if let Some(h) = self.row_height {
                if has_inner_row_border && i > 0 {
                    Constraint::Length(h.saturating_add(1))
                } else {
                    Constraint::Length(h)
                }
            } else {
                Constraint::Min(1)
            };
            row_layout_builder = row_layout_builder.add_constraint(constraint);
        }
        let row_layout = row_layout_builder.build();
        let row_areas = row_layout.split(table_area);

        let mut result = Vec::with_capacity(row_amount);

        for (rendered_rows, (_i, row)) in self
            .rows
            .iter()
            .enumerate()
            .skip(self.state.y_offset)
            .enumerate()
        {
            if rendered_rows >= row_amount || rendered_rows >= row_areas.len() {
                break;
            }

            let row_area = row_areas[rendered_rows];
            if row_area.top() >= table_area.bottom() {
                break;
            }

            let (col_amount, col_layout) = {
                let col_amount = if let Some(max_columns) = self.state.max_columns {
                    max_columns
                } else {
                    row.len()
                };
                if let Some(col_layout) = &self.col_layout {
                    (col_amount, col_layout.clone())
                } else {
                    let mut tmp = LayoutBuilder::new();
                    let mut col_layout_builder = tmp.direction(Direction::Horizontal);
                    let col_percentage = 100usize.saturating_div(col_amount);
                    #[allow(clippy::cast_possible_truncation)]
                    for _ in 0..col_amount {
                        col_layout_builder = col_layout_builder
                            .add_constraint(Constraint::Percentage(col_percentage as u16));
                    }
                    (col_amount, col_layout_builder.build())
                }
            };

            let col_areas = col_layout.split(row_area);
            let mut row_results = Vec::with_capacity(col_amount);

            for (rendered_cols, (_j, _col)) in
                row.iter().enumerate().skip(self.state.x_offset).enumerate()
            {
                if rendered_cols >= col_amount {
                    break;
                }

                let mut cell_area = col_areas[rendered_cols];

                if matches!(self.draw_inner_border, InnerBorder::All | InnerBorder::Rows)
                    && rendered_rows > 0
                {
                    cell_area.y = cell_area.y.saturating_add(1);
                    cell_area.height = cell_area.height.saturating_sub(1);
                }

                if matches!(
                    self.draw_inner_border,
                    InnerBorder::All | InnerBorder::Columns
                ) && rendered_cols > 0
                {
                    cell_area.x = cell_area.x.saturating_add(1);
                    cell_area.width = cell_area.width.saturating_sub(1);
                }

                row_results.push(cell_area);
            }
            result.push(row_results);
        }

        result
    }
}

impl Widget for Table<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, thoth: &thoth::Thoth) {
        let tl = crate::render::Grapheme::new("╔");
        let tr = crate::render::Grapheme::new("╗");
        let bl = crate::render::Grapheme::new("╚");
        let br = crate::render::Grapheme::new("╝");
        let h_out = crate::render::Grapheme::new("═");
        let v_out = crate::render::Grapheme::new("║");

        let h_in = crate::render::Grapheme::new("─");
        let v_in = crate::render::Grapheme::new("│");
        let cross = crate::render::Grapheme::new("┼");

        let left_tee = crate::render::Grapheme::new("╟");
        let right_tee = crate::render::Grapheme::new("╢");
        let top_tee = crate::render::Grapheme::new("╤");
        let bottom_tee = crate::render::Grapheme::new("╧");

        if self.draw_outer_border {
            let left = area.left();
            let right = area.right().saturating_sub(1);
            let top = area.top();
            let bottom = area.bottom().saturating_sub(1);

            canvas.set_ccell(
                left,
                top,
                CCell {
                    char: tl,
                    style: self.border_style,
                },
            );
            canvas.set_ccell(
                right,
                top,
                CCell {
                    char: tr,
                    style: self.border_style,
                },
            );
            canvas.set_ccell(
                left,
                bottom,
                CCell {
                    char: bl,
                    style: self.border_style,
                },
            );
            canvas.set_ccell(
                right,
                bottom,
                CCell {
                    char: br,
                    style: self.border_style,
                },
            );

            for x in (left + 1)..right {
                canvas.set_ccell(
                    x,
                    top,
                    CCell {
                        char: h_out,
                        style: self.border_style,
                    },
                );
                canvas.set_ccell(
                    x,
                    bottom,
                    CCell {
                        char: h_out,
                        style: self.border_style,
                    },
                );
            }
            for y in (top + 1)..bottom {
                canvas.set_ccell(
                    left,
                    y,
                    CCell {
                        char: v_out,
                        style: self.border_style,
                    },
                );
                canvas.set_ccell(
                    right,
                    y,
                    CCell {
                        char: v_out,
                        style: self.border_style,
                    },
                );
            }
        }

        let table_area = if self.draw_outer_border {
            Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            }
        } else {
            area
        };

        if table_area.width == 0 || table_area.height == 0 {
            return;
        }

        let mut tmp = LayoutBuilder::new();
        let mut row_layout = tmp.direction(Direction::Vertical);
        let row_amount = if let Some(max_rows) = self.state.max_rows {
            max_rows
        } else {
            // Only add at most table_area.height constraints as we can't show more anyway
            std::cmp::min(self.rows.len(), table_area.height as usize)
        };

        if row_amount == 0 {
            return;
        }

        let has_inner_row_border =
            matches!(self.draw_inner_border, InnerBorder::All | InnerBorder::Rows);

        for i in 0..row_amount {
            let constraint = if let Some(h) = self.row_height {
                if has_inner_row_border && i > 0 {
                    Constraint::Length(h.saturating_add(1))
                } else {
                    Constraint::Length(h)
                }
            } else {
                Constraint::Min(1)
            };
            row_layout = row_layout.add_constraint(constraint);
        }
        let row_layout = row_layout.build();
        let row_areas = row_layout.split(table_area);

        for (rendered_rows, (i, row)) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(self.state.y_offset)
            .enumerate()
        {
            if rendered_rows >= row_amount || rendered_rows >= row_areas.len() {
                break;
            }

            let row_area = row_areas[rendered_rows];
            if row_area.top() >= table_area.bottom() {
                break;
            }

            if matches!(self.draw_inner_border, InnerBorder::All | InnerBorder::Rows)
                && rendered_rows > 0
            {
                let y = row_area.y;

                for x in table_area.left()..table_area.right() {
                    canvas.set_ccell(
                        x,
                        y,
                        CCell {
                            char: h_in,
                            style: self.border_style,
                        },
                    );
                }

                if self.draw_outer_border {
                    canvas.set_ccell(
                        area.left(),
                        y,
                        CCell {
                            char: left_tee,
                            style: self.border_style,
                        },
                    );
                    canvas.set_ccell(
                        area.right().saturating_sub(1),
                        y,
                        CCell {
                            char: right_tee,
                            style: self.border_style,
                        },
                    );
                }
            }

            let row_style = if self.alternate_colour_vertically && i % 2 == 1 {
                self.alternate_style
            } else {
                self.style
            };

            let (col_amount, col_layout) = {
                let col_amount = if let Some(max_columns) = self.state.max_columns {
                    max_columns
                } else {
                    row.len()
                };
                if let Some(col_layout) = &self.col_layout {
                    (col_amount, col_layout)
                } else {
                    let mut tmp = LayoutBuilder::new();
                    let mut col_layout = tmp.direction(Direction::Horizontal);
                    let col_percentage = 100usize.saturating_div(col_amount);
                    #[allow(clippy::cast_possible_truncation)]
                    for _ in 0..col_amount {
                        col_layout = col_layout
                            .add_constraint(Constraint::Percentage(col_percentage as u16));
                    }
                    (col_amount, &col_layout.build())
                }
            };

            let col_areas = col_layout.split(row_area);

            for (rendered_cols, (j, col)) in row
                .iter_mut()
                .enumerate()
                .skip(self.state.x_offset)
                .enumerate()
            {
                if rendered_cols >= col_amount {
                    break;
                }

                let mut cell_area = col_areas[rendered_cols];

                if matches!(self.draw_inner_border, InnerBorder::All | InnerBorder::Rows)
                    && rendered_rows > 0
                {
                    cell_area.y = cell_area.y.saturating_add(1);
                    cell_area.height = cell_area.height.saturating_sub(1);
                }

                if matches!(
                    self.draw_inner_border,
                    InnerBorder::All | InnerBorder::Columns
                ) && rendered_cols > 0
                {
                    let x = cell_area.x;

                    let y_start = row_area.y;
                    let y_end = row_area.bottom();

                    for y in y_start..y_end {
                        canvas.set_ccell(
                            x,
                            y,
                            CCell {
                                char: v_in,
                                style: self.border_style,
                            },
                        );
                    }

                    if matches!(self.draw_inner_border, InnerBorder::All) && rendered_rows > 0 {
                        canvas.set_ccell(
                            x,
                            y_start,
                            CCell {
                                char: cross,
                                style: self.border_style,
                            },
                        );
                    }

                    if self.draw_outer_border {
                        if rendered_rows == 0 {
                            canvas.set_ccell(
                                x,
                                area.top(),
                                CCell {
                                    char: top_tee,
                                    style: self.border_style,
                                },
                            );
                        }
                        if rendered_rows == row_amount - 1 {
                            canvas.set_ccell(
                                x,
                                area.bottom().saturating_sub(1),
                                CCell {
                                    char: bottom_tee,
                                    style: self.border_style,
                                },
                            );
                        }
                    }

                    cell_area.x = cell_area.x.saturating_add(1);
                    cell_area.width = cell_area.width.saturating_sub(1);
                }

                let mut col_style = if self.alternate_colour_horizontally && j % 2 == 1 {
                    if self.alternate_colour_vertically && i % 2 == 1 {
                        self.style
                    } else {
                        self.alternate_style
                    }
                } else {
                    row_style
                };
                if let Some(header_row) = self.header_row
                    && i == header_row
                {
                    col_style = self.header_style;
                }
                col.style(col_style);

                col.render(canvas, cell_area, thoth);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Text;

    #[test]
    fn test_table_inner_border_rows() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 10);
        let r1: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R1C1", &thoth)),
            Box::new(Text::new("R1C2", &thoth)),
        ];
        let r2: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R2C1", &thoth)),
            Box::new(Text::new("R2C2", &thoth)),
        ];
        let rows = vec![r1, r2];

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .draw_inner_border(InnerBorder::Rows);

        table.render(&mut canvas, Rect::new(0, 0, 20, 10), &thoth);

        // Horizontal border should be at y=5 (because 2 rows in 10 lines -> split(10) -> [0..5, 5..10])
        let h_in = crate::render::Grapheme::new("─");
        assert_eq!(canvas.get_ccell(0, 5).char, h_in);
        assert_eq!(canvas.get_ccell(19, 5).char, h_in);

        // Vertical border should NOT be present
        let v_in = crate::render::Grapheme::new("│");
        assert_ne!(canvas.get_ccell(10, 0).char, v_in);
    }

    #[test]
    fn test_table_inner_border_columns() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 10);
        let r1: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R1C1", &thoth)),
            Box::new(Text::new("R1C2", &thoth)),
        ];
        let rows = vec![r1];

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .draw_inner_border(InnerBorder::Columns);

        table.render(&mut canvas, Rect::new(0, 0, 20, 10), &thoth);

        // Vertical border should be at x=10
        let v_in = crate::render::Grapheme::new("│");
        assert_eq!(canvas.get_ccell(10, 0).char, v_in);

        // Horizontal border should NOT be present
        let h_in = crate::render::Grapheme::new("─");
        assert_ne!(canvas.get_ccell(0, 5).char, h_in);
    }

    #[test]
    fn test_table_inner_border_all() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 10);
        let r1: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R1C1", &thoth)),
            Box::new(Text::new("R1C2", &thoth)),
        ];
        let r2: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R2C1", &thoth)),
            Box::new(Text::new("R2C2", &thoth)),
        ];
        let rows = vec![r1, r2];

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .draw_inner_border(InnerBorder::All);

        table.render(&mut canvas, Rect::new(0, 0, 20, 10), &thoth);

        let h_in = crate::render::Grapheme::new("─");
        let v_in = crate::render::Grapheme::new("│");
        let cross = crate::render::Grapheme::new("┼");

        assert_eq!(canvas.get_ccell(0, 5).char, h_in);
        assert_eq!(canvas.get_ccell(10, 0).char, v_in);
        assert_eq!(canvas.get_ccell(10, 5).char, cross);
    }

    #[test]
    fn test_table_outer_border() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 10);
        let r1: Vec<Box<dyn Widget>> = vec![Box::new(Text::new("R1C1", &thoth))];
        let rows = vec![r1];

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .draw_outer_border();

        table.render(&mut canvas, Rect::new(0, 0, 20, 10), &thoth);

        // Outer border corners
        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("╔"));
        assert_eq!(canvas.get_ccell(19, 0).char, crate::render::Grapheme::new("╗"));
        assert_eq!(canvas.get_ccell(0, 9).char, crate::render::Grapheme::new("╚"));
        assert_eq!(canvas.get_ccell(19, 9).char, crate::render::Grapheme::new("╝"));
    }

    #[test]
    fn test_table_custom_row_height() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 10);
        let r1: Vec<Box<dyn Widget>> = vec![Box::new(Text::new("R1", &thoth))];
        let r2: Vec<Box<dyn Widget>> = vec![Box::new(Text::new("R2", &thoth))];
        let rows = vec![r1, r2];

        let mut table = Table::new(&mut table_state)
            .with_rows(rows)
            .with_row_height(2)
            .draw_inner_border(InnerBorder::Rows);

        table.render(&mut canvas, Rect::new(0, 0, 20, 10), &thoth);

        // Row 0: height 2 (y=0, y=1)
        // Row 1: border at y=2, height 2 (y=3, y=4)
        let h_in = crate::render::Grapheme::new("─");
        assert_eq!(canvas.get_ccell(0, 2).char, h_in);

        // R1 should be in y=0 or y=1
        // R2 should be in y=3 or y=4
        // Text::new("R1") will render "R1" starting at (0,0)
        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("R"));
        assert_eq!(canvas.get_ccell(1, 0).char, crate::render::Grapheme::new("1"));

        assert_eq!(canvas.get_ccell(0, 3).char, crate::render::Grapheme::new("R"));
        assert_eq!(canvas.get_ccell(1, 3).char, crate::render::Grapheme::new("2"));
    }

    #[test]
    fn test_table_inner() {
        let mut table_state = TableState {
            x_offset: 0,
            y_offset: 0,
            max_rows: None,
            max_columns: None,
        };
        let thoth = thoth::Thoth::new().unwrap();
        let r1: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R1C1", &thoth)),
            Box::new(Text::new("R1C2", &thoth)),
        ];
        let r2: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("R2C1", &thoth)),
            Box::new(Text::new("R2C2", &thoth)),
        ];
        let rows = vec![r1, r2];

        let table = Table::new(&mut table_state)
            .with_rows(rows)
            .draw_inner_border(InnerBorder::All)
            .draw_outer_border();

        let area = Rect::new(0, 0, 21, 11);
        let inner_areas = table.inner(area);

        // Outer border takes 1 pixel on each side.
        // Table area is (1, 1, 19, 9)
        // 2 rows in 9 lines: 9 / 2 = 4. [0..5, 5..9] relative to table_area.
        // Relative to absolute: Row 0 is y=1..6, Row 1 is y=6..10
        // Row 1 starts with a border at y=6.

        // Col layout: 100 / 2 = 50%. 19 * 0.5 = 9. [0..9, 9..19] relative to table_area.
        // Relative to absolute: Col 0 is x=1..10, Col 1 is x=10..20
        // Col 1 starts with a border at x=10.

        assert_eq!(inner_areas.len(), 2);
        assert_eq!(inner_areas[0].len(), 2);
        assert_eq!(inner_areas[1].len(), 2);

        // Row 0, Col 0: x=1, y=1, w=9, h=5
        assert_eq!(inner_areas[0][0], Rect::new(1, 1, 9, 5));
        // Row 0, Col 1: x=10+1=11, y=1, w=9-1=8, h=5
        assert_eq!(inner_areas[0][1], Rect::new(11, 1, 8, 5));

        // Row 1, Col 0: x=1, y=6+1=7, w=9, h=4-1=3
        assert_eq!(inner_areas[1][0], Rect::new(1, 7, 9, 3));
        // Row 1, Col 1: x=10+1=11, y=6+1=7, w=9-1=8, h=4-1=3
        assert_eq!(inner_areas[1][1], Rect::new(11, 7, 8, 3));
    }
}
