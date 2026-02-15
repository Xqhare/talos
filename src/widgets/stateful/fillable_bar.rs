//! A stateful widget that displays a progress bar.
//!
//! The `FillableBar` widget can be used to display a progress bar. The state of the fillable bar is
//! managed by a `FillableBarState` struct, which must be passed to the `with_state` method.
//!
//! # Example
//!
//! ```rust,no_run
//! use talos::{
//!     Talos,
//!     input::{Event, KeyCode, KeyEvent},
//!     layout::Rect,
//!     widgets::{
//!         stateful::{FillableBar, FillableBarState},
//!         traits::Widget,
//!     },
//! };
//!
//! fn main() -> Result<(), talos::TalosError> {
//!     let mut talos = Talos::builder().build()?;
//!     let (canvas, codex) = talos.render_ctx();
//!
//!     let mut fillable_bar_state = FillableBarState { fill: 0.5 };
//!
//!     let mut fillable_bar = FillableBar::new()
//!         .with_state(&mut fillable_bar_state)
//!         .show_percentage()
//!         .glow();
//!
//!     let rect = Rect::new(0, 0, 20, 1);
//!     fillable_bar.render(canvas, rect, codex);
//!
//!     talos.present()?;
//!
//!     Ok(())
//! }
//! ```

use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{CCell, Canvas, Style},
    widgets::{Number, traits::Widget},
};

/// A fillable bar
///
/// Always tries to fill all given space
///
/// If using a a bar in the vertical dircection, and you want to enable the `percentage`, make sure
/// to set the width to at least 4.
///
/// # Example
/// ```rust,no_run
/// use talos::{Talos, widgets::stateful::{FillableBar, FillableBarState}};
///
/// let mut talos = Talos::builder().build().unwrap();
/// let (_, codex) = talos.render_ctx();
/// let mut fillable_bar_state = FillableBarState { fill: 0.5 };
/// let fillable_bar = FillableBar::new().with_state(&mut fillable_bar_state);
/// # assert!(true);
/// ```
#[must_use]
pub struct FillableBar<'a> {
    style: Style,
    state: Option<&'a mut FillableBarState>,
    show_percentage: bool,
    glow: bool,
    vertical: bool,
}

/// The state of a fillable bar
///
/// The fill value is between 0.0 and 1.0
#[derive(Default)]
pub struct FillableBarState {
    /// The fill value
    pub fill: f32,
}

impl Default for FillableBar<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> FillableBar<'a> {
    /// Creates a new, empty fillable bar
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{FillableBar, FillableBarState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut fillable_bar_state = FillableBarState { fill: 0.5 };
    /// let fillable_bar = FillableBar::new().with_state(&mut fillable_bar_state);
    /// # assert!(true);
    /// ```
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
    pub fn with_state(mut self, state: &'a mut FillableBarState) -> Self {
        self.state = Some(state);
        self
    }

    /// Show the percentage in text in the middle of the bar
    pub fn show_percentage(mut self) -> Self {
        self.show_percentage = true;
        self
    }

    /// Makes the bar glow or fade
    pub fn glow(mut self) -> Self {
        self.glow = true;
        self
    }

    /// Makes the bar vertical
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

impl Widget for FillableBar<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let fill = self.state.as_ref().map_or(0.0, |s| s.fill);
        // BODGE: flip bg and fg
        self.style = Style::builder()
            .set_fg(self.style.get_bg().unwrap())
            .set_bg(self.style.get_fg().unwrap())
            .build();
        if self.vertical {
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let fill_height = (f32::from(area.height) * fill) as u16;
            let empty_height = area.height.saturating_sub(fill_height);

            for y_off in 0..area.height {
                let y = area.y + y_off;

                // Determine the character and style for this entire row
                let (char, cell_style) = if y_off < empty_height {
                    (SPACE_GLYPH, self.style)
                } else {
                    let depth = y_off - empty_height;
                    // Glow Logic
                    if self.glow && depth == 0 && fill_height > 0 && fill < 1.0 {
                        (codex.lookup('░'), self.style)
                    } else if self.glow && depth == 1 && fill_height > 1 && fill < 1.0 {
                        (codex.lookup('▒'), self.style)
                    } else if self.glow && depth == 2 && fill_height > 2 && fill < 1.0 {
                        (codex.lookup('▓'), self.style)
                    } else {
                        (codex.lookup('█'), self.style)
                    }
                };

                // 2. Iterate over the Width (This allows the bar to be 2, 3, or N cells wide)
                for x_off in 0..area.width {
                    canvas.set_ccell(
                        area.x + x_off,
                        y,
                        CCell {
                            char,
                            style: cell_style,
                        },
                    );
                }
            }

            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            if self.show_percentage {
                let percentage = (fill * 100.0).round() as u16;
                let mut number = Number::new(&percentage, codex);

                // Text Color Logic (Inverted if on top of filled part)
                let mut number_style = self.style;
                if fill_height > area.height / 2 {
                    let fg = self.style.get_fg().unwrap();
                    let bg = self.style.get_bg().unwrap();
                    number_style = Style::builder().set_fg(bg).set_bg(fg).build();
                }
                number.style(number_style);

                // 3. Render Text at Bottom, spanning the full width
                let number_area = Rect {
                    x: area.width / 2 + area.x,
                    y: area.height / 2 + area.y,
                    width: area.width,
                    height: 1,
                };

                number.render(canvas, number_area, codex);

                // Add '%' sign if it fits
                if let Some((last_x, last_y)) = canvas.last_cell()
                    && last_x + 1 < area.right()
                {
                    canvas.set_ccell(
                        last_x + 1,
                        last_y,
                        CCell {
                            char: codex.lookup('%'),
                            style: number_style,
                        },
                    );
                }
            }
        } else {
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let fill_width = (f32::from(area.width) * fill) as u16;

            for x_off in 0..area.width {
                let x = area.x + x_off;

                let (char, cell_style) = if x_off < fill_width {
                    let depth = fill_width.saturating_sub(1).saturating_sub(x_off);

                    if self.glow && depth == 0 && fill < 1.0 {
                        (codex.lookup('░'), self.style)
                    } else if self.glow && depth == 1 && fill < 1.0 {
                        (codex.lookup('▒'), self.style)
                    } else if self.glow && depth == 2 && fill < 1.0 {
                        (codex.lookup('▓'), self.style)
                    } else {
                        (codex.lookup('█'), self.style)
                    }
                } else {
                    (SPACE_GLYPH, self.style)
                };

                for y_off in 0..area.height {
                    canvas.set_ccell(
                        x,
                        area.y + y_off,
                        CCell {
                            char,
                            style: cell_style,
                        },
                    );
                }
            }

            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            if self.show_percentage {
                let percentage = (fill * 100.0).round() as u16;
                let mut number = Number::new(&percentage, codex);

                let mut number_style = self.style;
                if fill_width > area.width / 2 {
                    let fg = self.style.get_fg().unwrap();
                    let bg = self.style.get_bg().unwrap();
                    number_style = Style::builder().set_fg(bg).set_bg(fg).build();
                }
                number.style(number_style);

                let number_area = Rect {
                    x: area.x + area.width.div_ceil(2).saturating_sub(1), // Rough horizontal center
                    y: area.y + area.height.div_ceil(2).saturating_sub(1), // Vertical center
                    width: area.width,
                    height: 1,
                };

                number.render(canvas, number_area, codex);
                if let Some((last_x, last_y)) = canvas.last_cell()
                    && last_x + 1 < area.right()
                {
                    canvas.set_ccell(
                        last_x + 1,
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
