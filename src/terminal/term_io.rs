use std::io::{self, Write};
use std::mem;
use std::os::fd::AsRawFd;
use crate::error::TalosResult;

use crate::constants::ansi::{CLEAR_ALL, ENTER_ALT_SCREEN, EXIT_ALT_SCREEN, HIDE_CURSOR, SHOW_CURSOR, TO_TOP_LEFT}

use super::raw_mode::RawMode;

pub struct TerminalIO {
    stdin: io::Stdin,
    stdout: io::Stdout,
    _raw_mode: RawMode,
}

impl TerminalIO {
    pub fn new(hide_cursor: bool, alternate_screen: bool) -> TalosResult<TerminalIO> {

        let stdin = io::stdin();
        let raw_mode = RawMode::enable(stdin.as_raw_fd())?;
        let mut stdout = io::stdout();

        // Enter Alternate Screen 
        // Clear Screen & Home Cursor
        // Hide Cursor
        if alternate_screen {
            write!(stdout, "{}", ENTER_ALT_SCREEN)?;
        }
        if hide_cursor {
            write!(stdout, "{}", HIDE_CURSOR)?;
        }
        write!(stdout, "{}", CLEAR_ALL)?;
        write!(stdout, "{}", TO_TOP_LEFT)?;
        stdout.flush()?;

        Ok(TerminalIO {
            stdin,
            stdout,
            _raw_mode: raw_mode,
        })
    }

    pub fn stdin(&mut self) -> &mut io::Stdin {
        &mut self.stdin
    }

    pub fn stdout(&mut self) -> &mut io::Stdout {
        &mut self.stdout
    }

    pub fn size(&self) -> TalosResult<(u16, u16)> {
        unsafe {
            let mut winsize: libc::winsize = mem::zeroed();
            if libc::ioctl(self.stdout.as_raw_fd(), libc::TIOCGWINSZ, &mut winsize) == -1 {
                return Err(io::Error::last_os_error().into())
            }
            Ok((winsize.ws_row, winsize.ws_col))
        }
    }
}

impl Drop for TerminalIO {
    fn drop(&mut self) {
        // 1. Exit Alternate Screen
        // 2. Show Cursor
        let _ = write!(self.stdout, "{}", EXIT_ALT_SCREEN);
        let _ = write!(self.stdout, "{}", SHOW_CURSOR);
        let _ = self.stdout.flush();
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
