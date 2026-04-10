//! This module contains the widgets for the Talos library.
//!
//! Widgets are the building blocks of your UI. Talos provides a few basic widgets, such as `Block`
//! and `Text`. You can also create your own widgets by implementing the `Widget` trait.
//!
//! # Example
//!
//! ```rust,no_run
//! use talos::{
//!     Talos,
//!     layout::Rect,
//!     widgets::{Block, traits::Widget},
//! };
//!
//! fn main() -> Result<(), talos::TalosError> {
//!     let mut talos = Talos::builder().build()?;
//!
//!     talos.begin_frame();
//!     let (canvas, codex) = talos.render_ctx();
//!
//!     let rect = Rect::new(0, 0, 10, 10);
//!     let mut block = Block::new();
//!     block.render(canvas, rect, codex);
//!
//!     talos.present()?;
//!
//!     Ok(())
//! }
//! ```

mod area;
mod block;
mod internal_text;
mod number;
mod text;
pub use area::Area;
pub use block::Block;
pub use number::Number;
pub use text::Text;

/// Stateful widgets
pub mod stateful;

/// Widget traits for creating custom widgets
pub mod traits {
    use crate::{
        codex::Codex,
        layout::Rect,
        render::{Canvas, Style},
    };

    /// Widget trait
    ///
    /// A widget is a building block for your UI
    ///
    /// If you want to create a custom widget, you need to implement this trait.
    /// To learn more, take a look at the widgets included in this crate and how they implement
    /// this trait.
    pub trait Widget {
        /// Renders the widget into the specified area
        ///
        /// The area is relative to the top left corner of the canvas (1,1)
        ///
        // # Arguments
        /// * `canvas` - The canvas to render into
        /// * `area` - The area to render into. Only this area is available for the widget to draw
        ///   into.
        fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex);
        /// Sets the primary style of the widget
        fn style(&mut self, style: Style);

        /// Returns the widget with the specified style set
        ///
        /// This is a convenience method for chaining
        #[must_use]
        fn with_style(mut self, style: Style) -> Self
        where
            Self: Sized,
        {
            self.style(style);
            self
        }
    }

    impl Widget for &mut dyn Widget {
        fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
            (**self).render(canvas, area, codex);
        }
        fn style(&mut self, style: Style) {
            (**self).style(style);
        }
    }

    /// Convenience method for converting an iterator of widgets to a vector of dynamic widgets
    ///
    /// Useful for passing a vector of widgets to a widget that expects a vector of dynamic
    /// widgets like `List` or `MenuButton`.
    pub fn make_dyn_iter<'a, I, W>(iter: I) -> Vec<&'a mut dyn Widget>
    where
        I: Iterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        iter.map(|w| w as &mut dyn Widget).collect()
    }
}
