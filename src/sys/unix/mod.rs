
mod raw_mode;
pub use raw_mode::{enable_rawmode, disable_rawmode};

mod flags;
pub use flags::{register_signal_handlers, check_resize, check_terminate};

use std::{io, mem, os::fd};

use crate::error::TalosResult;

pub fn terminal_size(fd_stdout: fd::RawFd) -> TalosResult<(u16, u16)> {
    unsafe {
        let mut winsize: libc::winsize = mem::zeroed();
        if libc::ioctl(fd_stdout, libc::TIOCGWINSZ, &mut winsize) == -1 {
            return Err(io::Error::last_os_error().into())
        }
        Ok((winsize.ws_row, winsize.ws_col))
    }
}
