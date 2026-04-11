use std::os::fd;
use crate::error::Result as TalosResult;
use crate::backend::sys::{disable_raw_mode, enable_raw_mode};

/// A guard for raw mode
pub struct RawMode {
    /// Original terminal settings to restore.
    original_termios: libc::termios,
    /// File descriptor for stdin.
    fd_stdin: i32,
}

impl RawMode {
    /// Enables raw mode
    pub fn enable(fd_stdin: fd::RawFd) -> TalosResult<RawMode> {
        let (original_termios, fd_stdin) = enable_raw_mode(fd_stdin)?;
        Ok(RawMode {
            original_termios,
            fd_stdin,
        })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode(self.fd_stdin, &self.original_termios);
    }
}
