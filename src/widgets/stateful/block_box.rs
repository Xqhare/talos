use crate::{
    render::Style,
    widgets::{Block, traits::Widget},
};

/// The state of a `BlockBox`
///
/// A `BlockBox` is a block that contains another widget
///
/// Use `BlockBox::new` to create a new `BlockBox`
pub struct BlockBoxState<'a> {
    /// The block that contains or surrounds the widget
    pub block: &'a mut Block,
    /// The widget that is contained or surrounded by the block
    pub content: &'a mut dyn Widget,
}

/// A block that contains or surrounds another widget
///
/// Use `BlockBox::new` to create a new `BlockBox`
///
/// The block is rendered first, then the widget is rendered inside the block.
///
/// The Style of the block is applied to both the block and the widget, overwriting any styles set
/// on the contained Widget
///
/// # Example
/// ```rust,no_run
/// use talos::{
///     Talos,
///     layout::Rect,
///     render::{Colour, Normal, Style},
///     widgets::{stateful::BlockBox, Block, Text, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///
///     talos.begin_frame();
///     let (canvas, codex) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut block = Block::new()
///         .title("My Block", codex, true)
///         .with_fat_border()
///         .with_bg_fill();
///     let mut text = Text::new("Hello World!", codex);
///     let mut block_box = BlockBox::new(&mut block, &mut text);
///
///     block_box.render(canvas, rect, &codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
pub struct BlockBox<'a> {
    state: BlockBoxState<'a>,
    style: Style,
}

impl<'a> BlockBox<'a> {
    /// Create a new `BlockBox`
    ///
    /// # Arguments
    /// * `block` - The block that contains or surrounds the widget
    /// * `content` - The widget that is contained or surrounded by the block
    ///
    /// # Returns
    /// A new `BlockBox`
    ///
    /// # Example
    /// ```rust
    /// use talos::widgets::{Block, stateful::BlockBox, Text};
    ///
    /// let codex = talos::codex::Codex::new();
    /// let mut block = Block::new();
    /// let mut text = Text::new("Hello World!", &codex);
    /// let block_box = BlockBox::new(&mut block, &mut text);
    /// ```
    pub fn new(block: &'a mut Block, content: &'a mut dyn Widget) -> Self {
        Self {
            state: BlockBoxState { block, content },
            style: Style::default(),
        }
    }
}

impl Widget for BlockBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(
        &mut self,
        canvas: &mut crate::render::Canvas,
        area: crate::layout::Rect,
        codex: &crate::codex::Codex,
    ) {
        self.state.block.style(self.style);
        self.state.block.render(canvas, area, codex);
        self.state.content.style(self.style);
        self.state
            .content
            .render(canvas, self.state.block.inner(area), codex);
    }
}
