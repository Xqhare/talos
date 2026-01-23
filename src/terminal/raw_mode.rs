use std::{io, mem, os::fd};

use crate::error::TalosResult;


pub struct RawMode {
    original_termios: libc::termios,
    fd_stdin: fd::RawFd,
}

impl RawMode {
    pub fn enable(fd_stdin: fd::RawFd) -> TalosResult<RawMode> {
        unsafe {
            let mut termios = mem::zeroed();

            if libc::tcgetattr(fd_stdin, &mut termios) == -1 {
                return Err(io::Error::last_os_error().into())
            }

            let original_termios = termios;

            // Turn off software flow control, carriage return translation
            termios.c_iflag &= !(libc::IGNBRK | libc::BRKINT | libc::PARMRK | libc::ISTRIP | libc::INLCR | libc::IGNCR | libc::ICRNL | libc::IXON);

            // Turn off output processing
            termios.c_oflag &= !libc::OPOST;
            
            // Turn off echoing, canonical mode (line-by-line), and signals (Ctrl+C)
            termios.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
            
            // Control flags: Set 8 bits per char
            termios.c_cflag &= !(libc::CSIZE | libc::PARENB);
            termios.c_cflag |= libc::CS8;

            if libc::tcsetattr(fd_stdin, libc::TCSAFLUSH, &termios) == -1 {
                return Err(io::Error::last_os_error().into());
            }

            Ok(RawMode { original_termios, fd_stdin })
        }
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(self.fd_stdin, libc::TCSAFLUSH, &self.original_termios);
        }
    }
}
