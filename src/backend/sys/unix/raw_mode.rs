use crate::error::TalosResult;
use std::{io, mem, os::fd};

pub fn enable_rawmode(fd_stdin: fd::RawFd) -> TalosResult<(libc::termios, i32)> {
    unsafe {
        let mut termios = mem::zeroed();

        if libc::tcgetattr(fd_stdin, &raw mut termios) == -1 {
            return Err(io::Error::last_os_error().into());
        }

        let original_termios = termios;

        // Turn off software flow control, carriage return translation
        termios.c_iflag &= !(libc::IGNBRK
            | libc::BRKINT
            | libc::PARMRK
            | libc::ISTRIP
            | libc::INLCR
            | libc::IGNCR
            | libc::ICRNL
            | libc::IXON);

        // Turn off output processing
        termios.c_oflag &= !libc::OPOST;

        // Turn off echoing, canonical mode (line-by-line), and signals (Ctrl+C)
        termios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::IEXTEN);

        // Control flags: Set 8 bits per char
        termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
        termios.c_cflag |= libc::CS8;

        // Non blocking read - I hope
        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 0;

        if libc::tcsetattr(fd_stdin, libc::TCSAFLUSH, &raw const termios) == -1 {
            return Err(io::Error::last_os_error().into());
        }

        Ok((original_termios, fd_stdin))
    }
}

pub fn disable_rawmode(fd_stdin: fd::RawFd, original_termios: &libc::termios) {
    unsafe {
        libc::tcsetattr(fd_stdin, libc::TCSAFLUSH, original_termios);
    }
}
