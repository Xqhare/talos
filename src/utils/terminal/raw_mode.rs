use std::io::Write;
use std::os::fd;
use std::sync::Once;

use crate::error::TalosResult;
use crate::utils::constants::ansi::{EXIT_ALT_SCREEN, SHOW_CURSOR};
use crate::utils::sys::{disable_raw_mode, enable_raw_mode};

pub struct RawMode {
    original_termios: libc::termios,
    fd_stdin: fd::RawFd,
}

impl RawMode {
    pub fn enable(fd_stdin: fd::RawFd) -> TalosResult<RawMode> {
        // Install panic hook - ALWAYS CALL BEFORE `enable_rawmode`
        install_panic_hook();
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

static INIT: Once = Once::new();

fn install_panic_hook() {
    INIT.call_once(|| {
        let def_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let mut std_err = std::io::stderr();
            let _ = std_err.write_all(EXIT_ALT_SCREEN.as_bytes());
            let _ = std_err.write_all(SHOW_CURSOR.as_bytes());
            let _ = std_err.flush();
            def_hook(info);
        }));
    });
}
