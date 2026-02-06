use crate::{codex::{Codex, pages::SPACE_GLYPH}, layout::Rect, render::{CCell, Canvas, Style}, widgets::{Number, traits::Widget}};

/// A fillable bar
///
/// Always one Cell high
pub struct FillableBar<'a> {
    style: Style,
    state: Option<&'a mut FillableBarState>,
    show_percentage: bool,
    glow: bool,
}

/// The state of a fillable bar
///
/// The fill value is between 0.0 and 1.0
pub struct FillableBarState {
    pub fill: f32,
}

impl<'a> FillableBar<'a> {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
            state: None,
            show_percentage: false,
            glow: false,
        }
    }

    pub fn with_state(mut self, state: &'a mut FillableBarState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn show_percentage(mut self) -> Self {
        self.show_percentage = true;
        self
    }

    pub fn glow(mut self) -> Self {
        self.glow = true;
        self
    }
}

impl Widget for FillableBar<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let fill = self.state.as_ref().map(|s| s.fill).unwrap_or(0.0);
        let fill_width = (area.width as f32 * fill) as u16;

        for i in 0..area.width {
            if i < fill_width {
                canvas.set_ccell(area.x + i, area.y, CCell { char: SPACE_GLYPH, style: self.style });
            } else if i < fill_width + 1 && fill_width != 0 && self.glow {
                canvas.set_ccell(area.x + i, area.y, CCell { char: codex.lookup('░'), style: self.style });
            } else if i < fill_width + 2 && fill_width != 0 && self.glow {
                canvas.set_ccell(area.x + i, area.y, CCell { char: codex.lookup('▒'), style: self.style });
            } else if i < fill_width + 3 && fill_width != 0 && self.glow {
                canvas.set_ccell(area.x + i, area.y, CCell { char: codex.lookup('▓'), style: self.style });
            } else {
                canvas.set_ccell(area.x + i, area.y, CCell { char: codex.lookup('█'), style: self.style });
            }
        }

        if self.show_percentage {
            let percentage = (fill * 100.0).round() as u16;
            let mut number = Number::new(percentage, codex);
            let mut number_style = self.style;
            if fill_width < area.width.div_ceil(2) {
                number_style =
                    Style::builder()
                        .set_fg(self.style.get_bg().unwrap())
                        .set_bg(self.style.get_fg().unwrap())
                        .build()
            }
            number.style(number_style);
            let number_area = Rect { x: area.x + area.width.div_ceil(2), y: area.y, width: area.width, height: 1 };
            number.render(canvas, number_area, codex);
            if let Some((last_x, last_y)) = canvas.last_cell() {
                canvas.set_ccell(last_x + 1, last_y, CCell { char: codex.lookup('%'), style: number_style });
            }
        }
    }
}

