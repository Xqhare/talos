use std::io::{Stdin, Stdout, stdout, stdin};
use crate::error::Result as TalosResult;
use crate::backend::sys::{disable_raw_mode, enable_raw_mode, terminal_size};

/// A struct to handle terminal IO
pub struct TerminalIO {
    stdin: Stdin,
    stdout: Stdout,
    original_termios: libc::termios,
    fd_stdin: i32,
    alternate_screen: bool,
}

impl TerminalIO {
    /// Creates a new TerminalIO
    ///
    /// # Arguments
    /// * `hide_cursor` - Whether to hide the cursor
    /// * `alternate_screen` - Whether to use the alternate screen
    ///
    /// # Errors
    /// Returns an error if the terminal could not be initialized
    pub fn new(hide_cursor: bool, alternate_screen: bool) -> TalosResult<TerminalIO> {
        let stdin = stdin();
        let stdout = stdout();

        let (original_termios, fd_stdin) = enable_raw_mode(0)?;

        let mut term = TerminalIO {
            stdin,
            stdout,
            original_termios,
            fd_stdin,
            alternate_screen,
        };

        if hide_cursor {
            term.hide_cursor()?;
        }

        if alternate_screen {
            term.enter_alternate_screen()?;
        }

        Ok(term)
    }

    /// Restores the terminal to its original state
    ///
    /// # Errors
    /// Returns an error if the terminal could not be restored
    pub fn restore(&mut self) -> TalosResult<()> {
        if self.alternate_screen {
            self.exit_alternate_screen()?;
        }
        self.show_cursor()?;
        disable_raw_mode(self.fd_stdin, &self.original_termios);
        Ok(())
    }

    fn hide_cursor(&mut self) -> TalosResult<()> {
        crate::utils::write_all_bytes(&mut self.stdout, crate::utils::constants::ansi::HIDE_CURSOR.as_bytes())
    }

    fn show_cursor(&mut self) -> TalosResult<()> {
        crate::utils::write_all_bytes(&mut self.stdout, crate::utils::constants::ansi::SHOW_CURSOR.as_bytes())
    }

    fn enter_alternate_screen(&mut self) -> TalosResult<()> {
        crate::utils::write_all_bytes(&mut self.stdout, crate::utils::constants::ansi::ENTER_ALT_SCREEN.as_bytes())
    }

    fn exit_alternate_screen(&mut self) -> TalosResult<()> {
        crate::utils::write_all_bytes(&mut self.stdout, crate::utils::constants::ansi::EXIT_ALT_SCREEN.as_bytes())
    }

    /// Returns the size of the terminal
    ///
    /// # Errors
    /// Returns an error if the size could not be retrieved
    pub fn size(&self) -> TalosResult<(u16, u16)> {
        terminal_size(1)
    }

    /// Returns a reference to the stdin
    #[must_use]
    pub fn stdin(&mut self) -> &mut Stdin {
        &mut self.stdin
    }

    /// Returns a reference to the stdout
    #[must_use]
    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}
