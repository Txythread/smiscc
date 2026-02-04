use std::ffi::CString;
use std::sync::atomic::{AtomicU64, Ordering};

#[repr(C)]
struct OSLog;

#[link(name = "signpost_shim")]
unsafe extern "C" {
    fn create_log(subsystem: *const i8, category: *const i8) -> *mut OSLog;
    fn emit_signpost(log: *mut OSLog, spid: u64, message: *const i8);
}


static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct InstrumentsLog {
    log: *mut OSLog,
}

impl InstrumentsLog {
    /// Create a new instruments log with subsystem/category
    pub fn new(subsystem: &str, category: &str) -> InstrumentsLog {
        let cs = CString::new(subsystem).unwrap();
        let cc = CString::new(category).unwrap();
        unsafe {
            let log = create_log(cs.as_ptr(), cc.as_ptr());
            InstrumentsLog { log }
        }
    }

    /// Emit a single signpost event with a message
    pub fn mark(&mut self, message: &str) {
        let cmsg = CString::new(message).unwrap();
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        unsafe {
            emit_signpost(self.log, 0, cmsg.as_ptr())
        }
    }
}
