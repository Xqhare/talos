use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{CCell, Canvas, Style},
    widgets::{Number, traits::Widget},
};

/// A stateful widget that displays a progress bar.
///
/// The `FillableBar` widget can be used to display a progress bar. The state of the fillable bar is
/// managed by a `FillableBarState` struct, which must be passed to the `with_state` method.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     input::{Event, KeyCode, KeyEvent},
///     layout::Rect,
///     widgets::{
///         stateful::{FillableBar, FillableBarState},
///         traits::Widget,
///     },
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///     let (canvas, codex) = talos.render_ctx();
///
///     let mut fillable_bar_state = FillableBarState { fill: 0.5 };
///
///     let mut fillable_bar = FillableBar::new()
///         .with_state(&mut fillable_bar_state)
///         .show_percentage()
///         .glow();
///
///     let rect = Rect::new(0, 0, 20, 1);
///     fillable_bar.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[must_use]
pub struct FillableBar<'a> {
    /// The style of the bar.
    style: Style,
    /// The state of the bar.
    state: Option<&'a mut FillableBarState>,
    /// Whether to show the percentage.
    show_percentage: bool,
    /// Whether to make the bar glow.
    glow: bool,
    /// Whether the bar is vertical.
    vertical: bool,
}

/// The state of a fillable bar
///
/// The fill value is between 0.0 and 1.0
#[derive(Default)]
#[non_exhaustive]
pub struct FillableBarState {
    /// The fill value
    pub fill: f32,
}

impl Default for FillableBar<'_> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> FillableBar<'a> {
    /// Creates a new, empty fillable bar
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::FillableBar};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut fillable_bar_state = FillableBarState { fill: 0.5 };
    /// let fillable_bar = FillableBar::new().with_state(&mut fillable_bar_state);
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            state: None,
            show_percentage: false,
            glow: false,
            vertical: false,
        }
    }

    /// Sets the state of the fillable bar
    #[inline]
    #[must_use]
    pub fn with_state(mut self, state: &'a mut FillableBarState) -> Self {
        self.state = Some(state);
        self
    }

    /// Show the percentage in text in the middle of the bar
    #[inline]
    #[must_use]
    pub fn show_percentage(mut self) -> Self {
        self.show_percentage = true;
        self
    }

    /// Makes the bar glow or fade
    #[inline]
    #[must_use]
    pub fn glow(mut self) -> Self {
        self.glow = true;
        self
    }

    /// Makes the bar vertical
    #[inline]
    #[must_use]
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

impl Widget for FillableBar<'_> {
    #[inline]
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[inline]
    #[expect(clippy::too_many_lines, reason = "Render functions are naturally long")]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let fill = self.state.as_ref().map_or(0.0, |s| s.fill);
        // BODGE: flip bg and fg
        let fg = self.style.get_fg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::White));
        let bg = self.style.get_bg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::Black));
        self.style = Style::builder()
            .set_fg(bg)
            .set_bg(fg)
            .build();
        if self.vertical {
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Coords are also truncated")]
            let fill_height = (f32::from(area.height) * fill) as u16;
            let empty_height = area.height.saturating_sub(fill_height);

            for y_off in 0..area.height {
                let y = area.y.saturating_add(y_off);

                // Determine the character and style for this entire row
                let (char, cell_style) = if y_off < empty_height {
                    (SPACE_GLYPH, self.style)
                } else {
                    let depth = y_off.saturating_sub(empty_height);
                    // Glow Logic
                    if self.glow && depth == 0 && fill_height > 0 && fill < 1.0 {
                        (codex.lookup('\u{2591}'), self.style) // ░
                    } else if self.glow && depth == 1 && fill_height > 1 && fill < 1.0 {
                        (codex.lookup('\u{2592}'), self.style) // ▒
                    } else if self.glow && depth == 2 && fill_height > 2 && fill < 1.0 {
                        (codex.lookup('\u{2593}'), self.style) // ▓
                    } else {
                        (codex.lookup('\u{2588}'), self.style) // █
                    }
                };

                // 2. Iterate over the Width (This allows the bar to be 2, 3, or N cells wide)
                for x_off in 0..area.width {
                    canvas.set_ccell(
                        area.x.saturating_add(x_off),
                        y,
                        CCell {
                            char,
                            style: cell_style,
                        },
                    );
                }
            }

            if self.show_percentage {
                #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Coords are also truncated")]
                let percentage = (fill * 100.0).round() as u16;
                let mut number = Number::new(&percentage, codex);

                // Text Color Logic (Inverted if on top of filled part)
                let mut number_style = self.style;
                if fill_height > area.height.saturating_div(2) {
                    let fg = self.style.get_fg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::White));
                    let bg = self.style.get_bg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::Black));
                    number_style = Style::builder().set_fg(bg).set_bg(fg).build();
                }
                number.style(number_style);

                // 3. Render Text at Bottom, spanning the full width
                let number_area = Rect {
                    x: area.width.saturating_div(2).saturating_add(area.x),
                    y: area.height.saturating_div(2).saturating_add(area.y),
                    width: area.width,
                    height: 1,
                };

                number.render(canvas, number_area, codex);

                // Add '%' sign if it fits
                if let Some((last_x, last_y)) = canvas.last_cell()
                    && last_x.saturating_add(1) < area.right()
                {
                    canvas.set_ccell(
                        last_x.saturating_add(1),
                        last_y,
                        CCell {
                            char: codex.lookup('%'),
                            style: number_style,
                        },
                    );
                }
            }
        } else {
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Coords are also truncated")]
            let fill_width = (f32::from(area.width) * fill) as u16;

            for x_off in 0..area.width {
                let x = area.x.saturating_add(x_off);

                let (char, cell_style) = if x_off < fill_width {
                    let depth = fill_width.saturating_sub(1).saturating_sub(x_off);

                    if self.glow && depth == 0 && fill < 1.0 {
                        (codex.lookup('\u{2591}'), self.style) // ░
                    } else if self.glow && depth == 1 && fill < 1.0 {
                        (codex.lookup('\u{2592}'), self.style) // ▒
                    } else if self.glow && depth == 2 && fill < 1.0 {
                        (codex.lookup('\u{2593}'), self.style) // ▓
                    } else {
                        (codex.lookup('\u{2588}'), self.style) // █
                    }
                } else {
                    (SPACE_GLYPH, self.style)
                };

                for y_off in 0..area.height {
                    canvas.set_ccell(
                        x,
                        area.y.saturating_add(y_off),
                        CCell {
                            char,
                            style: cell_style,
                        },
                    );
                }
            }

            if self.show_percentage {
                #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation, reason = "Coords are also truncated")]
                let percentage = (fill * 100.0).round() as u16;
                let mut number = Number::new(&percentage, codex);

                let mut number_style = self.style;
                if fill_width > area.width.saturating_div(2) {
                    let fg = self.style.get_fg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::White));
                    let bg = self.style.get_bg().unwrap_or(crate::render::Colour::Normal(crate::render::Normal::Black));
                    number_style = Style::builder().set_fg(bg).set_bg(fg).build();
                }
                number.style(number_style);

                let number_area = Rect {
                    x: area.x.saturating_add(area.width.div_ceil(2).saturating_sub(1)), // Rough horizontal center
                    y: area.y.saturating_add(area.height.div_ceil(2).saturating_sub(1)), // Vertical center
                    width: area.width,
                    height: 1,
                };

                number.render(canvas, number_area, codex);
                if let Some((last_x, last_y)) = canvas.last_cell()
                    && last_x.saturating_add(1) < area.right()
                {
                    canvas.set_ccell(
                        last_x.saturating_add(1),
                        last_y,
                        CCell {
                            char: codex.lookup('%'),
                            style: number_style,
                        },
                    );
                }
            }
        }
    }
}
