//! A simple, (almost) no dependency, TUI immediate mode unix toolkit.
//!
//! It can be used to create a simple TUI for any kind of application.
//!
//! # Philosophy
//!
//! Talos is an immediate mode TUI library. This means that you are responsible for drawing the entire UI
//! on every frame. This is in contrast to retained mode libraries, where you create UI elements and the
//! library is responsible for drawing them.
//!
//! This approach has a few advantages:
//!
//! * **Simplicity:** The API is simple and easy to understand. You don't have to worry about complex
//! UI state management.
//! * **Flexibility:** You have complete control over the rendering process. This makes it easy to
//! create custom UI elements and effects.
//!
//! # Getting Started
//!
//! To get started with Talos, you need to add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! talos = { git = "https://github.com/Xqhare/talos" }
//! ```
//!
//! Then, you can create a new `Talos` instance and start drawing your UI:
//!
//! ```rust,no_run
//! use talos::{
//!     Talos,
//!     input::{Event, KeyCode, KeyEvent},
//!     layout::Rect,
//!     render::{Colour, Normal, Style},
//!     widgets::{Block, Text, traits::Widget},
//! };
//!
//! // A simple helper to make the loop cleaner
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() -> Result<(), talos::TalosError> {
//!     // 1. Initialize Talos
//!     let mut talos = Talos::builder().build()?;
//!
//!     let mut running = true;
//!
//!     while running {
//!         // 2. Handle Input
//!         if let Some(events) = talos.poll_input()? {
//!             for event in events {
//!                 match event {
//!                     // Quit on 'q' or Esc
//!                     Event::KeyEvent(KeyEvent {
//!                         code: KeyCode::Char('q'),
//!                         ..
//!                     })
//!                     | Event::KeyEvent(KeyEvent {
//!                         code: KeyCode::Esc, ..
//!                     }) => {
//!                         running = false;
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!
//!         // 3. Render Frame
//!         talos.begin_frame();
//!         let (canvas, codex) = talos.render_ctx();
//!
//!         let big_area = Rect::new(1, 1, canvas.max_width(), canvas.max_height());
//!
//!         let style = Style::builder()
//!             .set_fg(Colour::Normal(Normal::Yellow))
//!             .set_bg(Colour::Normal(Normal::Blue))
//!             .build();
//!
//!         let mut large_block: Block = Block::new()
//!             .title("", codex, false)
//!             .with_fat_border()
//!             .with_bg_fill();
//!
//!         large_block.style(style);
//!         large_block.render(canvas, big_area, codex);
//!
//!         let right_area = Rect::new(canvas.max_width().saturating_sub(60), 5, 30, 5);
//!
//!         let style = Style::builder()
//!             .set_fg(Colour::Normal(Normal::White))
//!             .set_bg(Colour::Normal(Normal::Red))
//!             .build();
//!
//!         let mut right_block: Block = Block::new()
//!             .with_fat_border()
//!             .title("Right", codex, false)
//!             .with_beautify_border_breaks()
//!             .with_bg_fill();
//!
//!         right_block.style(style);
//!         right_block.render(canvas, right_area, codex);
//!
//!         let drawing_over_right = Rect::new(canvas.max_width().saturating_sub(40), 8, 30, 5);
//!
//!         let style = Style::builder()
//!             .set_fg(Colour::Normal(Normal::White))
//!             .set_bg(Colour::Normal(Normal::Green))
//!             .build();
//!
//!         let mut next_right_block: Block = Block::new()
//!             .title("Over Right", codex, false)
//!             .with_bg_fill();
//!
//!         next_right_block.style(style);
//!         next_right_block.render(canvas, drawing_over_right, codex);
//!
//!         // Let's draw a white & black block in the middle
//!         let area = Rect::new(15, 15, 50, 10);
//!
//!         let style = Style::builder()
//!             .set_fg(Colour::Normal(Normal::Black))
//!             .set_bg(Colour::Normal(Normal::White))
//!             .build();
//!
//!         let mut block: Block = Block::new()
//!             .title(" Hello Talos ", codex, false)
//!             .with_bg_fill();
//!
//!         block.style(style);
//!         block.render(canvas, area, codex);
//!
//!         // Lets add some styled text to the block
//!         let block_inner = block.inner(area);
//!
//!         let text_style = Style::builder()
//!             .set_bg(Colour::Normal(Normal::White))
//!             .set_fg(Colour::Normal(Normal::Blue))
//!             .set_bold(true)
//!             .build();
//!
//!         let mut text = Text::new("Look mom! Text inside a block!", codex)
//!             .align_center()
//!             .align_vertically();
//!
//!         text.style(text_style);
//!         text.render(canvas, block_inner, codex);
//!
//!         // 4. Present to Terminal
//!         talos.present()?;
//!
//!         // Cap framerate slightly to save CPU
//!         thread::sleep(Duration::from_millis(16));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Main Components
//!
//! ## `Talos`
//!
//! The `Talos` struct is the main entry point of the library. It is responsible for initializing the
//! terminal, handling input, and rendering the UI.
//!
//! ## Layout
//!
//! The layout system is based on the `LayoutBuilder`. It allows you to divide the screen into
//! different areas, and then place widgets in them.
//!
//! ## Widgets
//!
//! Widgets are the building blocks of your UI. Talos provides a few basic widgets, such as `Block`
//! and `Text`. You can also create your own widgets by implementing the `Widget` trait.
//!
//! ## Rendering
//!
//! The rendering process is handled by the `Canvas`. It provides a simple API for drawing text and
//! styling it with colors and attributes.
//!
//! ### Colours and Backgrounds
//!
//! When using [`Normal::Black`](render/enum.Normal.html), be aware that many terminals render this
//! as a dark gray rather than a true black. To use the terminal's true default background, set the
//! background of your [`Style`](render/struct.Style.html) to `None`.

// Uncomment below when appropriate
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
// Uncomment below when approaching stable
// #![warn(missing_docs)]

use std::io::Write;

use codex::Codex;
use input::Parser;
use input::poll_input_bytes;
use ui::render::CCell;
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::write_all_bytes;

mod builder;
pub use builder::{LayoutBuilder, ParserBuilder, TalosBuilder};
pub mod error;
pub use error::{TalosError, TalosResult};

use crate::backend::TerminalIO;
use crate::backend::sys::check_resize;
use crate::backend::sys::check_terminate;
use crate::input::Event;
use crate::ui::render::Canvas;
use crate::utils::move_render_cursor;

mod backend;
/// Codex
pub mod codex;
mod content;
mod ui;

/// Input
pub mod input;
pub use ui::layout;
pub use ui::render;
mod utils;
/// Widgets
pub mod widgets;

type Width = u16;
type Height = u16;

/// The main struct of the library
///
/// # Example
/// ```rust,no_run
/// use talos::Talos;
///
/// let talos = Talos::builder().build();
/// assert!(talos.is_ok());
/// ```
///
/// For more information on building the struct, see [`TalosBuilder`](struct.TalosBuilder.html).
pub struct Talos {
    terminal: TerminalIO,
    canvas: Canvas,
    codex: Codex,
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
    previous_buffer: Vec<CCell>,

    output_buffer: Vec<u8>,

    // Input-Parser
    parser: Parser,
}

/// Return type of `Talos::present`
pub enum Present {
    /// The terminal was resized, and because of that nothing was drawn
    Resized,
    /// The terminal was not resized - the canvas was drawn
    Presented,
}

impl Talos {
    /// Returns a new `TalosBuilder`
    ///
    /// This builder can be used to configure `Talos`.
    ///
    /// For an exhaustive list of options, see the [`TalosBuilder`](struct.TalosBuilder.html) struct.
    /// Most options are shown in the example below.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder()
    /// .with_cursor() // Show the Terminal cursor
    /// .with_alternate_screen() // Use the alternate screen
    /// .without_panic_handler() // Disable the panic handler
    /// .build();
    /// assert!(talos.is_ok());
    /// ```
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    /// Returns a mutable reference to the canvas
    ///
    /// Consider using `Talos::render_ctx` instead
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let canvas = talos.canvas_mut();
    /// # assert!(true);
    /// ```
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    /// Returns a tuple containing the canvas and the codex in the form `(canvas, codex)`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (canvas, codex) = talos.render_ctx();
    /// # assert!(true);
    /// ```
    pub fn render_ctx(&mut self) -> (&mut Canvas, &Codex) {
        (&mut self.canvas, &self.codex)
    }

    /// Clear the canvas.
    /// Call at the beginning of every frame.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// talos.begin_frame();
    /// # assert!(true);
    /// ```
    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    /// Present the canvas to the terminal
    /// The new size is stored in `self.size`.
    ///
    /// # Returns
    /// Returns whether the terminal was resized.
    /// If the terminal was resized, `present` will not draw anything to the terminal.
    /// Returns `Ok(Present::Resized)` if the terminal was resized.
    /// Returns `Ok(Present::Presented)` if the terminal was not resized.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// assert!(talos.present().is_ok());
    /// ```
    pub fn present(&mut self) -> TalosResult<Present> {
        let resized = self.handle_signals()?;
        if resized {
            return Ok(Present::Resized);
        }

        self.output_buffer.clear();

        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;

        let mut prev_x_cell: u16 = u16::MAX;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    if x.wrapping_sub(prev_x_cell) != 1 {
                        move_render_cursor(&mut self.output_buffer, x, y)?;
                    }

                    // Write styled char
                    ccell.style.generate(&mut self.output_buffer);
                    write_all_bytes(
                        &mut self.output_buffer,
                        self.codex.resolve(ccell.char).as_bytes(),
                    )?;
                    prev_x_cell = x;
                }
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            return Ok(Present::Presented);
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        // Pointer swapping of the buffers
        std::mem::swap(&mut self.previous_buffer, &mut self.canvas.buffer);

        Ok(Present::Presented)
    }

    /// Returns a mutable reference to the codex
    ///
    /// Consider using `Talos::render_ctx` instead
    /// A mutable reference is only needed to add more pages to the codex.
    ///
    /// For more on adding pages to the codex, see the documentation of `Codex`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let codex = talos.codex_mut();
    /// # assert!(true);
    /// ```
    pub fn codex_mut(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns all input events since the last call.
    /// If there is no input, returns None.
    ///
    /// Eagerly evaluates all bytes read, and returns an `Event::Unknown` if
    /// the bytes cannot be parsed.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    /// use talos::input::{Event, KeyEvent, KeyCode};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// # let mut c = 0;
    /// let mut run = true;
    /// while run {
    ///     if let Ok(Some(events)) = talos.poll_input() {
    ///         for event in events {
    ///             match event {
    ///                 Event::KeyEvent(KeyEvent {
    ///                     code: KeyCode::Char('q'),
    ///                     ..
    ///                 }) => {
    ///                     run = false;
    ///                 }
    ///                 _ => {}
    ///             }
    ///         }
    ///     }
    ///     # c += 1;
    ///     # if c > 10 { break; }
    /// }
    /// # assert!(true);
    /// ```
    pub fn poll_input(&mut self) -> TalosResult<Option<&[Event]>> {
        let _ = self.handle_signals()?;

        self.parser.event_buffer.clear();

        if let Some(bytes) = poll_input_bytes(
            &mut self.terminal.stdin(),
            &mut self.parser.poll_input_buffer,
            self.parser.max_poll_input_buffer,
            self.parser.buffer_linear_growth_step,
        )? {
            self.parser
                .parser
                .parse(bytes, &mut self.parser.event_buffer)?;
        } else {
            self.parser.parser.flush(&mut self.parser.event_buffer);
        }

        Ok(Some(self.parser.event_buffer.as_slice()))
    }

    /// Handles signals from the OS
    ///
    /// Returns `true` whether the terminal was resized
    /// Returns `false` if the terminal was not resized
    /// ENDS THE PROCESS if the terminate signal was received
    ///
    /// If the terminal was terminated, the terminal is restored and the process exits.
    ///
    /// # Errors
    /// Returns an error if internal I/O errors occur
    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            // We need to shut down now - No state will be saved, just restore the terminal
            self.terminal.restore()?;
            std::process::exit(0);
        }

        if check_resize() {
            let (rows, cols) = self.terminal.size()?;
            self.size = (cols, rows);

            self.canvas = Canvas::new(self.size.0, self.size.1);
            let len = (self.size.0 as usize) * (self.size.1 as usize);
            self.previous_buffer = vec![CCell::default(); len];
            self.output_buffer.clear();
            self.output_buffer.reserve(len * 10);
            write_all_bytes(&mut self.terminal.stdout(), CLEAR_ALL.as_bytes())?;
            self.terminal.stdout().flush()?;
            return Ok(true);
        }

        Ok(false)
    }
}
