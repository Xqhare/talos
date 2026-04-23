#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
use std::io::Write;

use codex::Codex;
use input::Parser;
use input::poll_input_bytes;
use ui::render::{CCell, Style};
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::constants::ansi::{SHOW_CURSOR, HIDE_CURSOR};
use utils::constants::ansi::{BEGIN_SYNC_UPDATE, END_SYNC_UPDATE};
use utils::write_all_bytes;

mod builder;
pub use builder::{LayoutBuilder, ParserBuilder, TalosBuilder};
pub mod error;
pub use error::{TalosError, TalosResult};

/// Style and Layout Atlases
pub mod atlases;

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
    hide_cursor: bool,
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

        // Removing the next line will cause some weird side effects (Bleeding the `selected` style of
        // `List` to previous elements for example);
        // Doing this is not ideal (performance-wise) but it works
        Style::default().generate(&mut self.output_buffer);

        let mut prev_x_cell: u16 = u16::MAX;
        let mut current_terminal_style = Style::default();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    if x.wrapping_sub(prev_x_cell) != 1 {
                        move_render_cursor(&mut self.output_buffer, x, y)?;
                    }

                    // Only generate style if it differs from the current terminal style
                    if ccell.style != current_terminal_style {
                        ccell
                            .style
                            .generate_diff(current_terminal_style, &mut self.output_buffer);
                        current_terminal_style = ccell.style;
                    }

                    write_all_bytes(
                        &mut self.output_buffer,
                        self.codex.resolve(ccell.char).as_bytes(),
                    )?;
                    prev_x_cell = x;
                }
            }
        }

        if let Some((x, y)) = self.canvas.cursor {
            move_render_cursor(&mut self.output_buffer, x, y)?;
            write_all_bytes(&mut self.output_buffer, SHOW_CURSOR.as_bytes())?;
        } else if self.hide_cursor {
            write_all_bytes(&mut self.output_buffer, HIDE_CURSOR.as_bytes())?;
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            return Ok(Present::Presented);
        }

        self.terminal
            .stdout()
            .write_all(BEGIN_SYNC_UPDATE.as_bytes())?;
        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal
            .stdout()
            .write_all(END_SYNC_UPDATE.as_bytes())?;
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

    /// Returns the codex
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let codex = talos.codex();
    /// # assert!(true);
    /// ```
    pub fn codex(&self) -> &Codex {
        &self.codex
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
