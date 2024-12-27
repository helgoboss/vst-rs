use std::panic::{catch_unwind, AssertUnwindSafe};

/// Should be placed around each C entry point to not crash the DAW on a panic.
pub fn firewall<R, F>(f: F) -> Option<R> where F: FnOnce() -> R {
    catch_unwind(AssertUnwindSafe(f)).ok()
}