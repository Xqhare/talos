use crate::error::Result as TalosResult;
use std::{mem, ptr, sync::atomic::{AtomicBool, Ordering}};

/// Flag indicating that the terminal was resized.
static RESIZED: AtomicBool = AtomicBool::new(false);
/// Flag indicating that the process was terminated.
static TERMINATED: AtomicBool = AtomicBool::new(false);

/// C-compatible signal handler.
extern "C" fn signal_handler(sig: libc::c_int) {
    match sig {
        libc::SIGWINCH => RESIZED.store(true, Ordering::SeqCst),
        libc::SIGTERM | libc::SIGINT => TERMINATED.store(true, Ordering::SeqCst),
        _ => {}
    }
}

/// Registers signal handlers for SIGWINCH, SIGTERM, and SIGINT.
pub fn register_signal_handlers() -> TalosResult<()> {
    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sa.sa_sigaction = signal_handler as *const () as usize;
        sa.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&raw mut sa.sa_mask);

        if libc::sigaction(libc::SIGWINCH, &raw const sa, ptr::null_mut()) == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        if libc::sigaction(libc::SIGTERM, &raw const sa, ptr::null_mut()) == -1 {
            return Err(std::io::Error::last_os_error().into());
        }

        if libc::sigaction(libc::SIGINT, &raw const sa, ptr::null_mut()) == -1 {
            return Err(std::io::Error::last_os_error().into());
        }
    }
    Ok(())
}

/// Checks if the terminal was resized since the last call.
#[must_use]
pub fn check_resize() -> bool {
    RESIZED.swap(false, Ordering::SeqCst)
}

/// Checks if the process was terminated.
#[must_use]
pub fn check_terminate() -> bool {
    TERMINATED.load(Ordering::SeqCst)
}
