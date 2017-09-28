
#![allow(dead_code)] // runtime init functions not used during testing

use ffi::OsString;
use marker::PhantomData;
use vec;

/// One-time global initialization.
pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}

/// One-time global cleanup.
pub unsafe fn cleanup() {}

pub fn args() -> Args {
    unimplemented!();
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}
