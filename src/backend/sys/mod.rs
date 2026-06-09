use std::{io, os::fd};

#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix as os;

use crate::{TalosError, error::TalosResult};

pub fn enable_raw_mode(fd_stdin: fd::RawFd) -> TalosResult<(libc::termios, i32)> {
    os::enable_rawmode(fd_stdin)
}

pub fn terminal_size(fd_stdout: fd::RawFd) -> TalosResult<(u16, u16)> {
    match athena::system::terminal_size(fd_stdout) {
        Ok((w, h)) => Ok((w as u16, h as u16)),
        Err(e) => {
            let io_err = TryInto::<io::Error>::try_into(e);
            match io_err {
                Ok(io_err) => Err(TalosError::from(io_err)),
                Err(e) => Err(TalosError::GenericError(e.to_string())), // Should never happen
            }
        }
    }
}

pub fn disable_raw_mode(fd_stdin: fd::RawFd, original_termios: &libc::termios) {
    os::disable_rawmode(fd_stdin, original_termios);
}

pub fn register_signal_handlers() -> TalosResult<()> {
    os::register_signal_handlers()
}

pub fn check_resize() -> bool {
    os::check_resize()
}

pub fn check_terminate() -> bool {
    os::check_terminate()
}
