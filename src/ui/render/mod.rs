//! This module contains the rendering system for the Talos library.
//!
//! The rendering system is responsible for drawing widgets to the screen. It is based on the
//! `Canvas` struct, which provides a simple API for drawing text and styling it with colors and
//! attributes.
//!
//! # Example
//!
//! ```rust,no_run
//! use talos::{
//!     Talos,
//!     layout::Rect,
//!     render::{Colour, Normal, Style},
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
//!     let style = Style::builder()
//!         .set_fg(Colour::Normal(Normal::Red))
//!         .build();
//!
//!     let mut block = Block::new();
//!     block.style(style);
//!     block.render(canvas, rect, codex);
//!
//!     talos.present()?;
//!
//!     Ok(())
//! }
//! ```

mod colour;
pub use colour::{Bright, Colour, ColourMode, Extended, Normal, TrueColour};

mod canvas;
pub use canvas::Canvas;

mod ccell;
pub use ccell::CCell;

mod style;
pub use style::Style;

/// A glyph is a internal representation of a character.
/// It uses the code pages inside the [Codex](struct.Codex.html) struct.
pub type Glyph = u16;
