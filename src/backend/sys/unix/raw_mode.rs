use crate::error::Result as TalosResult;
use std::{mem, os::fd};

/// Enables raw mode for the given file descriptor.
pub fn enable_rawmode(fd_stdin: fd::RawFd) -> TalosResult<(libc::termios, i32)> {
    unsafe {
        let mut termios = mem::zeroed();

        if libc::tcgetattr(fd_stdin, &raw mut termios) == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        let original_termios = termios;

        termios.c_iflag &= !(libc::IGNBRK
            | libc::BRKINT
            | libc::PARMRK
            | libc::ISTRIP
            | libc::INLCR
            | libc::IGNCR
            | libc::ICRNL
            | libc::IXON);
        termios.c_oflag &= !libc::OPOST;
        termios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
        termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
        termios.c_cflag |= libc::CS8;

        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 0;

        if libc::tcsetattr(fd_stdin, libc::TCSAFLUSH, &raw const termios) == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok((original_termios, fd_stdin))
    }
}

/// Disables raw mode for the given file descriptor.
pub fn disable_rawmode(fd_stdin: fd::RawFd, original_termios: &libc::termios) {
    unsafe {
        let _ = libc::tcsetattr(fd_stdin, libc::TCSAFLUSH, &raw const *original_termios);
    }
}
