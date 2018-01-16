// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System bindings for the CMSIS RTOS v1 specification, provided by the cmsis_os crate (linking,
//! uh, tbd?). This is largely based on the way in which wasm32 was integrated. As much of the
//! functionality that can be implemented has been, though some parts don't make sense without
//! additional RTOS modules (e.g. filesystems or network). Lots of these functions are just stubs
//! that return errors. As with wasm32, "The hope is that with a portability lint we can ...
//! actually just remove all this and just omit parts of the standard library if we're compiling
//! for [CMSIS]".

use io;
use os::raw::c_char;

pub mod args;
#[cfg(feature = "backtrace")]
pub mod backtrace;
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fs;
pub mod memchr;
pub mod mutex;
pub mod net;
pub mod os;
pub mod os_str;
pub mod path;
pub mod pipe;
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
pub mod thread;
pub mod thread_local;
pub mod time;
pub mod stdio;

#[cfg(not(test))]
pub fn init() {
}

pub fn unsupported<T>() -> io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> io::Error {
    io::Error::new(io::ErrorKind::Other,
                   "operation not supported or nonsensical in CMSIS RTOS")
}

pub fn decode_error_kind(_code: i32) -> io::ErrorKind {
    io::ErrorKind::Other
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut n = 0;
    while *s != 0 {
        n += 1;
        s = s.offset(1);
    }
    return n
}

pub unsafe fn abort_internal() -> ! {
    ::intrinsics::abort();
}

// randomness guaranteed by a 1d20 roll.
pub fn hashmap_random_keys() -> (u64, u64) {
    (0xDEADBEEF, 0x1BADF00D)
}
