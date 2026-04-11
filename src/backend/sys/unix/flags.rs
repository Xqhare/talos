use crate::error::Result as TalosResult;
use std::{mem, ptr, sync::atomic::{AtomicBool, Ordering}};

static RESIZED: AtomicBool = AtomicBool::new(false);
static TERMINATED: AtomicBool = AtomicBool::new(false);

extern "C" fn signal_handler(sig: libc::c_int) {
    match sig {
        libc::SIGWINCH => RESIZED.store(true, Ordering::SeqCst),
        libc::SIGTERM | libc::SIGINT => TERMINATED.store(true, Ordering::SeqCst),
        _ => {}
    }
}

pub fn register_signal_handlers() -> TalosResult<()> {
    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sa.sa_sigaction = signal_handler as usize;
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

pub fn check_resize() -> bool {
    RESIZED.swap(false, Ordering::SeqCst)
}

pub fn check_terminate() -> bool {
    TERMINATED.load(Ordering::SeqCst)
}
