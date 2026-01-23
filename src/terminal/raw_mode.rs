use std::os::fd;
use std::sync::Once;

use crate::constants::ansi::{EXIT_ALT_SCREEN, SHOW_CURSOR};
use crate::error::TalosResult;

use crate::sys::os::enable_rawmode;
use crate::sys::unix::drop_rawmode;

pub struct RawMode {
    original_termios: libc::termios,
    fd_stdin: fd::RawFd,
}

impl RawMode {
    pub fn enable(fd_stdin: fd::RawFd) -> TalosResult<RawMode> {
        // Install panic hook - ALWAYS CALL BEFORE `enable_rawmode`
        install_panic_hook();
        let (original_termios, fd_stdin) = enable_rawmode(fd_stdin)?;
        Ok(RawMode { original_termios, fd_stdin })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        drop_rawmode(self.fd_stdin, &self.original_termios);
    }
}

static INIT: Once = Once::new();

fn install_panic_hook() {
    INIT.call_once(|| {
        let def_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = print!("{}", EXIT_ALT_SCREEN);
            let _ = print!("{}", SHOW_CURSOR);
            def_hook(info);
        }));
    });
}
