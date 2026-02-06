use crate::{LayoutBuilder, codex::Codex, layout::{Constraint, Direction, Rect}, render::{Canvas, Style}, widgets::traits::Widget};

/// A table of widgets
///
/// For now each row is laid out separately.
/// This means that, if a row is wider than another, the columns will not line up.
pub struct Table<'a> {
    state: Option<&'a mut TableState>,
    rows: Vec<Vec<&'a mut dyn Widget>>,
    alternate_colour_vertically: bool,
    alternate_colour_horizontally: bool,
    style: Style,
    alternate_style: Style,
}

pub struct TableState {
    pub x_offset: usize,
    pub y_offset: usize,
    pub max_rows: Option<usize>,
    pub max_columns: Option<usize>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            state: None,
            rows: Vec::new(),
            alternate_colour_vertically: false,
            alternate_colour_horizontally: false,
            style: Style::default(),
            alternate_style: Style::default(),
        }
    }

    pub fn with_state(mut self, state: &'a mut TableState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn add_row(mut self, row: Vec<&'a mut dyn Widget>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn with_rows<I, R, W>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        self.rows = rows.into_iter().map(|r| r.into_iter().map(|w| w as &'a mut dyn Widget).collect()).collect();
        self
    }

    pub fn with_alternate_style(mut self, style: Style) -> Self {
        self.alternate_style = style;
        self
    }

    pub fn alternate_colour_vertically(mut self) -> Self {
        self.alternate_colour_vertically = true;
        self
    }

    pub fn alternate_colour_horizontally(mut self) -> Self {
        self.alternate_colour_horizontally = true;
        self
    }
}

impl Widget for Table<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let mut tmp = LayoutBuilder::new();
        let mut row_layout = tmp.direction(Direction::Vertical);
        let row_amount = if let Some(max_rows) = self.state.as_ref().and_then(|s| s.max_rows) { max_rows } else { self.rows.len() };
        if row_amount == 0 { return; }
        let row_percentage = 100usize.saturating_div(row_amount);
        for _ in 0..row_amount {
            row_layout = row_layout.add_constraint(Constraint::Percentage(row_percentage as u16));
        }
        let row_layout = row_layout.build();
        let row_areas = row_layout.split(area);

        let mut rendered_rows = 0;

        for (i, row) in self.rows.iter_mut().enumerate().skip(self.state.as_ref().map(|s| s.y_offset).unwrap_or(0)) {
            if rendered_rows >= row_amount { break; }
            let row_style = if self.alternate_colour_vertically && i % 2 == 1 {
                self.alternate_style
            } else {
                self.style
            };
            let mut tmp = LayoutBuilder::new();
            let mut col_layout = tmp.direction(Direction::Horizontal);
            let col_amount = if let Some(max_columns) = self.state.as_ref().and_then(|s| s.max_columns) { max_columns } else { row.len() };
            let col_percentage = 100usize.saturating_div(col_amount);
            for _ in 0..col_amount {
                col_layout = col_layout.add_constraint(Constraint::Percentage(col_percentage as u16));
            }
            let col_layout = col_layout.build();

            let col_areas = col_layout.split(row_areas[rendered_rows]);

            let mut rendered_cols = 0;
            for (j, col) in row.iter_mut().enumerate().skip(self.state.as_ref().map(|s| s.x_offset).unwrap_or(0)) {
                if rendered_cols >= col_amount { break; }
                


                let col_style = if self.alternate_colour_horizontally && j % 2 == 1 {
                    if self.alternate_colour_vertically && i % 2 == 1 {
                        self.style
                    } else {
                        self.alternate_style
                    }
                } else {
                    row_style
                };
                col.style(col_style);
                col.render(canvas, col_areas[rendered_cols], codex);
                rendered_cols += 1;
            }

            rendered_rows += 1;
        }
    }
}
