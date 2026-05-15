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

use crate::codex::Codex;
use crate::layout::Rect;
use std::collections::BTreeMap;

/// A glyph is a internal representation of a character.
/// It uses the code pages inside the [Codex](struct.Codex.html) struct.
pub type Glyph = u16;

/// A map of interactive regions and their IDs.
#[derive(Default, Debug, Clone)]
pub struct InteractionMap {
    regions: BTreeMap<String, Rect>,
}

impl InteractionMap {
    /// Creates a new, empty `InteractionMap`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a region with an ID.
    pub fn register(&mut self, id: impl Into<String>, area: Rect) {
        self.regions.insert(id.into(), area);
    }

    /// Returns the ID of the region at the specified coordinates.
    pub fn get_at(&self, x: u16, y: u16) -> Option<&String> {
        for (id, rect) in &self.regions {
            if rect.contains(x, y) {
                return Some(id);
            }
        }
        None
    }

    /// Clears all regions from the map.
    pub fn clear(&mut self) {
        self.regions.clear();
    }
}

/// A context for rendering widgets.
///
/// This struct consolidates the various objects needed to render a widget, such as the [Canvas]
/// and the [Codex].
pub struct RenderContext<'a> {
    /// The canvas to render into.
    pub canvas: &'a mut Canvas,
    /// The codex to use for character mapping.
    pub codex: &'a Codex,
    /// The interaction map to register regions into.
    pub interactions: &'a mut InteractionMap,
}

impl<'a> RenderContext<'a> {
    /// Creates a new `RenderContext`.
    pub fn new(canvas: &'a mut Canvas, codex: &'a Codex, interactions: &'a mut InteractionMap) -> Self {
        Self { canvas, codex, interactions }
    }
}
