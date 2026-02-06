use crate::error::TalosResult;
use std::sync::Once;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, mem, ptr};

static RESIZE_NEEDED: AtomicBool = AtomicBool::new(false);
static TERMINATE_NEEDED: AtomicBool = AtomicBool::new(false);
static HANDLER_REGISTERED: Once = Once::new();

extern "C" fn signal_handler(sig: libc::c_int) {
    match sig {
        libc::SIGWINCH => RESIZE_NEEDED.store(true, Ordering::Relaxed),
        libc::SIGTERM | libc::SIGINT => TERMINATE_NEEDED.store(true, Ordering::Relaxed),
        _ => {}
    }
}

pub fn register_signal_handlers() -> TalosResult<()> {
    let mut out = Ok(());

    HANDLER_REGISTERED.call_once(|| {
        unsafe {
            let mut sa: libc::sigaction = mem::zeroed();

            sa.sa_sigaction = signal_handler as *const () as usize;
            sa.sa_flags = libc::SA_RESTART;

            // Register SIGWINCH
            if libc::sigaction(libc::SIGWINCH, &raw const sa, ptr::null_mut()) == -1 {
                out = Err(io::Error::last_os_error().into());
            }

            // Register SIGTERM (Kill request)
            if libc::sigaction(libc::SIGTERM, &raw const sa, ptr::null_mut()) == -1 {
                out = Err(io::Error::last_os_error().into());
            }

            // Register SIGINT (Keyboard Interrupt via kill -INT)
            if libc::sigaction(libc::SIGINT, &raw const sa, ptr::null_mut()) == -1 {
                out = Err(io::Error::last_os_error().into());
            }
        }
    });

    out
}

pub fn check_resize() -> bool {
    RESIZE_NEEDED.swap(false, Ordering::Relaxed)
}

pub fn check_terminate() -> bool {
    TERMINATE_NEEDED.load(Ordering::Relaxed)
}
