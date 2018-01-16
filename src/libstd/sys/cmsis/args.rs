// Args are meaningless here

use ffi::OsString;
use marker::PhantomData;
use vec;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {}

pub unsafe fn cleanup() {}

pub fn args() -> Args {
    Args{
        iter: Vec::new().into_iter(),
        _dont_send_or_sync_me: PhantomData,
    }
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        self.iter.as_slice()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0 as usize, None)
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        0 as usize
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        None
    }
}
