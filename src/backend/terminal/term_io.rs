use crate::backend::sys::terminal_size;
use crate::error::TalosResult;
use crate::utils::constants::ansi::{
    CLEAR_ALL, DISABLE_MOUSE_FORMATTING_CODE, DISABLE_MOUSE_REPORTING_CODE, ENTER_ALT_SCREEN,
    EXIT_ALT_SCREEN, HIDE_CURSOR, MOUSE_FORMATTING_CODE, MOUSE_REPORTING_CODE, SHOW_CURSOR,
    TO_TOP_LEFT,
};
use std::io::{self, Write};
use std::os::fd::AsRawFd;

use super::raw_mode::RawMode;

pub struct TerminalIO {
    stdin: io::Stdin,
    stdout: io::Stdout,
    raw_mode: Option<RawMode>,
}

impl TerminalIO {
    pub fn new(hide_cursor: bool, alternate_screen: bool) -> TalosResult<TerminalIO> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let raw_mode = RawMode::enable(stdin.as_raw_fd())?;

        // Enter Alternate Screen
        // Clear Screen & Home Cursor
        // Hide Cursor
        if alternate_screen {
            write!(stdout, "{ENTER_ALT_SCREEN}")?;
        }
        if hide_cursor {
            write!(stdout, "{HIDE_CURSOR}")?;
        }
        write!(stdout, "{CLEAR_ALL}")?;
        write!(stdout, "{TO_TOP_LEFT}")?;
        write!(stdout, "{MOUSE_FORMATTING_CODE}")?;
        write!(stdout, "{MOUSE_REPORTING_CODE}")?;
        stdout.flush()?;

        Ok(TerminalIO {
            stdin,
            stdout,
            raw_mode: Some(raw_mode),
        })
    }

    pub fn restore(&mut self) -> TalosResult<()> {
        write!(self.stdout, "{CLEAR_ALL}")?;
        write!(self.stdout, "{EXIT_ALT_SCREEN}")?;
        write!(self.stdout, "{SHOW_CURSOR}")?;
        write!(self.stdout, "{DISABLE_MOUSE_REPORTING_CODE}")?;
        write!(self.stdout, "{DISABLE_MOUSE_FORMATTING_CODE}")?;
        self.stdout.flush()?;

        if let Some(raw_mode) = self.raw_mode.take() {
            drop(raw_mode);
        }
        Ok(())
    }

    pub fn stdin(&mut self) -> &mut io::Stdin {
        &mut self.stdin
    }

    pub fn stdout(&mut self) -> &mut io::Stdout {
        &mut self.stdout
    }

    pub fn size(&self) -> TalosResult<(u16, u16)> {
        terminal_size(self.stdout.as_raw_fd())
    }
}

impl Drop for TerminalIO {
    fn drop(&mut self) {
        // Also wtf am I supposed to do with errors in here
        let _ = write!(self.stdout, "{CLEAR_ALL}");
        let _ = write!(self.stdout, "{EXIT_ALT_SCREEN}");
        let _ = write!(self.stdout, "{SHOW_CURSOR}");
        let _ = write!(self.stdout, "{DISABLE_MOUSE_REPORTING_CODE}");
        let _ = write!(self.stdout, "{DISABLE_MOUSE_FORMATTING_CODE}");
        let _ = self.stdout.flush();

        // Lets be explicit with dropping the raw mode - better safe than sorry
        if let Some(raw_mode) = self.raw_mode.take() {
            drop(raw_mode);
        }
    }
}

// This allows: write!(term, "Hello") instead of write!(term.stdout(), "Hello")
impl Write for TerminalIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}
