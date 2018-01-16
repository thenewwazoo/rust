
// This is a probably-placeholder. The CMSIS RTOS v1 spec does not provide for condition variables,
// but they can likely be implemented using the appropriate OS primitives. This implementation is
// left as an exercise for the reader.

use sys::mutex::Mutex;
use time::Duration;

pub struct Condvar;

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar
    }

    #[inline]
    pub unsafe fn init(&self) {}

    #[inline]
    pub fn notify_one(&self) {
    }

    #[inline]
    pub fn notify_all(&self) {
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
    }

    #[inline]
    pub fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        false
    }

    #[inline]
    pub unsafe fn destroy(&self) {
    }
}
