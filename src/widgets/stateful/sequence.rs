use std::cmp::min;

use crate::{
    LayoutBuilder,
    codex::Codex,
    layout::{Constraint, Direction, Layout, Rect},
    render::{Canvas, Style},
    widgets::{
        Area, Block,
        traits::{Widget, make_dyn_iter},
    },
};

/// The state for the `Sequence` widget
pub struct SequenceState {
    /// The current scroll offset
    pub scroll_offset: usize,
}

/// A sequence of widgets
///
/// The `Sequence` widget can be used to display a sequence of widgets. The widgets are displayed
/// either horizontally or vertically.
///
/// A custom Layout can be provided to control the layout of the widgets.
/// If no Layout is provided, the widgets attempts to draw all items inside the given space.
///
/// This widget supports internal scrolling - See the `scroll_offset` field in the `ListState`
///
/// # Example
/// ```rust,no_run
/// use talos::{Talos, widgets::{traits::Widget, stateful::{Sequence, SequenceState}}};
///
/// let mut talos = Talos::builder().build().unwrap();
/// let (_, codex) = talos.render_ctx();
/// let mut sequence_state = SequenceState {
///     scroll_offset: 0,
/// };
/// let items: Vec<&mut dyn Widget> = Vec::new();
/// let sequence = Sequence::new(items, sequence_state.iter_mut());
/// # assert!(true);
/// ```
pub struct Sequence<'a> {
    items: Vec<&'a mut dyn Widget>,
    state: SequenceState,
    style: Style,
    horizontal: bool,
    layout: Option<Layout>,
    draw_border: bool,
    draw_fat_border: bool,
}

impl<'a> Sequence<'a> {
    /// Creates a sequence of widgets
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{traits::Widget, stateful::{Sequence, SequenceState}}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut sequence_state = SequenceState {
    ///     scroll_offset: 0,
    /// };
    /// let items: Vec<&mut dyn Widget> = Vec::new();
    /// let sequence = Sequence::new(items, sequence_state.iter_mut());
    /// # assert!(true);
    /// ```
    pub fn new<I, W>(items: I, state: SequenceState) -> Self
    where
        I: Iterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        Self {
            items: make_dyn_iter(items),
            state,
            style: Style::default(),
            horizontal: true,
            layout: None,
            draw_border: false,
            draw_fat_border: false,
        }
    }

    /// Sets the direction of the `Sequence`
    ///
    /// Default is horizontal
    ///
    /// This will draw the widgets horizontally, left to right.
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// Sets the direction of the `Sequence`
    ///
    /// Default is horizontal
    ///
    /// This will draw the widgets vertically, top to bottom
    pub fn vertical(mut self) -> Self {
        self.horizontal = false;
        self
    }

    /// Draws a border around the `Sequence`
    pub fn draw_border(mut self) -> Self {
        self.draw_border = true;
        self
    }

    /// Draws a fat (double line) border around the `Sequence`
    pub fn draw_fat_border(mut self) -> Self {
        self.draw_border = true;
        self.draw_fat_border = true;
        self
    }

    /// Uses a custom `Layout`
    ///
    /// Only as many widgets are drawn as the `Layout` allows.
    /// When the `scroll_offset` field in the `ListState` is used, the layout must be able to
    /// handle that:
    ///
    /// The first `Rect` of the `Layout` is always used for the first widget rendered, the
    /// second `Rect` for the second widget and so on.
    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = Some(layout);
        self
    }

    fn make_layout(&self, area: Rect) -> Vec<Rect> {
        if let Some(layout) = &self.layout {
            layout.split(area)
        } else {
            let items_amount = self.items.len().saturating_sub(self.state.scroll_offset);
            let mut layout = LayoutBuilder::new();
            let display_items;
            if self.horizontal {
                layout.direction(Direction::Horizontal);
                display_items = min(items_amount, area.width as usize);
            } else {
                layout.direction(Direction::Vertical);
                display_items = min(items_amount, area.height as usize);
            }
            for _ in 0..display_items {
                layout.add_constraint(Constraint::Min(1));
            }
            layout.build().split(area)
        }
    }
}

impl Widget for Sequence<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let mut area = area;
        if self.draw_border {
            let mut block = Block::new().with_bg_fill().with_style(self.style);
            block.set_fat_border(self.draw_fat_border);
            block.render(canvas, area, codex);
            area = block.inner(area);
        } else {
            Area::new()
                .with_style(self.style)
                .render(canvas, area, codex);
        }
        let layout = self.make_layout(area);
        for (index, rect) in layout.iter().enumerate() {
            if index + self.state.scroll_offset < self.items.len() {
                self.items[index + self.state.scroll_offset].render(canvas, *rect, codex);
            }
        }
    }
}
