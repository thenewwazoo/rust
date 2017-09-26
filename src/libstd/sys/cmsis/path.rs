// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi::OsStr;
use path::Prefix;

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    false
}

#[inline]
pub fn is_verbatim_sep(b: u8) -> bool {
    false
}

pub fn parse_prefix(path: &OsStr) -> Option<Prefix> {
    None
}

// This is kind of ugly; my intention is to get the point across. Is there a better option?
pub const MAIN_SEP_STR: &'static str = "\0";
pub const MAIN_SEP: char = '\0';
