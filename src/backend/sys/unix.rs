/// Raw mode handling for Unix systems
pub mod raw_mode;
/// Signal flags handling for Unix systems
pub mod flags;

use std::{io, mem, os::fd};

use crate::error::TalosResult;

/// Get terminal size for Unix systems
#[inline]
pub fn terminal_size(fd_stdout: fd::RawFd) -> TalosResult<(u16, u16)> {
    unsafe {
        let mut winsize: libc::winsize = mem::zeroed();
        if libc::ioctl(fd_stdout, libc::TIOCGWINSZ, &mut winsize) == -1 {
            return Err(io::Error::last_os_error().into());
        }
        Ok((winsize.ws_row, winsize.ws_col))
    }
}
