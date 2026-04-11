use std::os::fd;

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
use unix as os;

use crate::error::TalosResult;

/// Enable raw mode
#[inline]
pub fn enable_raw_mode(fd_stdin: fd::RawFd) -> TalosResult<(libc::termios, i32)> {
    os::enable_rawmode(fd_stdin)
}

/// Get terminal size
#[inline]
pub fn terminal_size(fd_stdout: fd::RawFd) -> TalosResult<(u16, u16)> {
    os::terminal_size(fd_stdout)
}

/// Disable raw mode
#[inline]
pub fn disable_raw_mode(fd_stdin: fd::RawFd, original_termios: &libc::termios) {
    os::disable_rawmode(fd_stdin, original_termios);
}

/// Register signal handlers
#[inline]
pub fn register_signal_handlers() -> TalosResult<()> {
    os::register_signal_handlers()
}

/// Check if terminal was resized
#[inline]
#[must_use]
pub fn check_resize() -> bool {
    os::check_resize()
}

/// Check if terminal was terminated
#[inline]
#[must_use]
pub fn check_terminate() -> bool {
    os::check_terminate()
}
