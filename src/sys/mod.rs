
use std::os::fd;
mod unix;

use unix as os;

use crate::error::TalosResult;

pub fn enable_raw_mode(fd_stdin: fd::RawFd) -> TalosResult<(libc::termios, i32)> {
    os::enable_rawmode(fd_stdin)
}

pub fn terminal_size(fd_stdout: fd::RawFd) -> TalosResult<(u16, u16)> {
    os::terminal_size(fd_stdout)
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
