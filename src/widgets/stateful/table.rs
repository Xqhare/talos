use crate::{
    LayoutBuilder,
    codex::Codex,
    layout::{Constraint, Direction, Rect},
    render::{CCell, Canvas, Style},
    widgets::traits::Widget,
};

/// A table of widgets
///
/// For now each row is laid out separately.
/// This means that, if a row is wider than another, the columns will not line up.
#[must_use] 
#[allow(clippy::struct_excessive_bools)]
pub struct Table<'a> {
    state: Option<&'a mut TableState>,
    rows: Vec<Vec<&'a mut dyn Widget>>,
    alternate_colour_vertically: bool,
    alternate_colour_horizontally: bool,
    style: Style,
    alternate_style: Style,
    border_style: Style,
    draw_outer_border: bool,
    draw_inner_border: bool,
}

pub struct TableState {
    pub x_offset: usize,
    pub y_offset: usize,
    pub max_rows: Option<usize>,
    pub max_columns: Option<usize>,
}

impl Default for Table<'_> {
    fn default() -> Self {
        Self::new()
    }
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
            border_style: Style::default(),
            draw_outer_border: false,
            draw_inner_border: false,
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
        self.rows = rows
            .into_iter()
            .map(|r| r.into_iter().map(|w| w as &'a mut dyn Widget).collect())
            .collect();
        self
    }

    pub fn with_alternate_style(mut self, style: Style) -> Self {
        self.alternate_style = style;
        self
    }

    pub fn with_border_style(mut self, style: Style) -> Self {
        self.border_style = style;
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

    pub fn draw_outer_border(mut self) -> Self {
        self.draw_outer_border = true;
        self
    }

    pub fn draw_inner_border(mut self) -> Self {
        self.draw_inner_border = true;
        self
    }
}

impl Widget for Table<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    // TODO: This is a mess - if anything breaks its gonna need a refactor
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let tl = codex.lookup('╔');
        let tr = codex.lookup('╗');
        let bl = codex.lookup('╚');
        let br = codex.lookup('╝');
        let h_out = codex.lookup('═');
        let v_out = codex.lookup('║');

        let h_in = codex.lookup('─');
        let v_in = codex.lookup('│');
        let cross = codex.lookup('┼');

        let left_tee = codex.lookup('╟');
        let right_tee = codex.lookup('╢');
        let top_tee = codex.lookup('╤');
        let bottom_tee = codex.lookup('╧');

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
        let row_amount = if let Some(max_rows) = self.state.as_ref().and_then(|s| s.max_rows) {
            max_rows
        } else {
            self.rows.len()
        };

        if row_amount == 0 {
            return;
        }

        for _ in 0..row_amount {
            row_layout = row_layout.add_constraint(Constraint::Min(1));
        }
        let row_layout = row_layout.build();
        let row_areas = row_layout.split(table_area);

        for (rendered_rows, (i, row)) in self
            .rows
            .iter_mut()
            .enumerate()
            .skip(self.state.as_ref().map_or(0, |s| s.y_offset))
            .enumerate()
        {
            if rendered_rows >= row_amount {
                break;
            }

            if self.draw_inner_border && rendered_rows > 0 {
                let y = row_areas[rendered_rows].y;

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

            let mut tmp = LayoutBuilder::new();
            let mut col_layout = tmp.direction(Direction::Horizontal);
            let col_amount =
                if let Some(max_columns) = self.state.as_ref().and_then(|s| s.max_columns) {
                    max_columns
                } else {
                    row.len()
                };
            let col_percentage = 100usize.saturating_div(col_amount);
            #[allow(clippy::cast_possible_truncation)]
            for _ in 0..col_amount {
                col_layout =
                    col_layout.add_constraint(Constraint::Percentage(col_percentage as u16));
            }
            let col_layout = col_layout.build();

            let col_areas = col_layout.split(row_areas[rendered_rows]);

            for (rendered_cols, (j, col)) in row
                .iter_mut()
                .enumerate()
                .skip(self.state.as_ref().map_or(0, |s| s.x_offset))
                .enumerate()
            {
                if rendered_cols >= col_amount {
                    break;
                }

                let mut cell_area = col_areas[rendered_cols];

                if self.draw_inner_border {
                    if rendered_rows > 0 {
                        cell_area.y = cell_area.y.saturating_add(1);
                        cell_area.height = cell_area.height.saturating_sub(1);
                    }

                    if rendered_cols > 0 {
                        cell_area.x = cell_area.x.saturating_add(1);
                        cell_area.width = cell_area.width.saturating_sub(1);
                    }
                }

                if self.draw_inner_border && rendered_cols > 0 {
                    let x = col_areas[rendered_cols].x;

                    let y_start = row_areas[rendered_rows].y;
                    let y_end = row_areas[rendered_rows].bottom();

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

                    if self.draw_inner_border && rendered_rows > 0 {
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
                }

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

                col.render(canvas, cell_area, codex);

            }

        }
    }
}
