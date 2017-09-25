
use cell::UnsafeCell;

pub struct Condvar {}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar {}
    }

    #[inline]
    pub unsafe fn init(&self) {}

    #[inline]
    pub fn notify_one(&self) {
        ::sys_common::util::dumb_print(format_args!("condvar notify_one\n"));
        unimplemented!();
    }

    #[inline]
    pub fn notify_all(&self) {
        ::sys_common::util::dumb_print(format_args!("condvar notify_all\n"));
        unimplemented!();
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        ::sys_common::util::dumb_print(format_args!("condvar wait\n"));
        unimplemented!();
    }

    #[inline]
    pub fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        ::sys_common::util::dumb_print(format_args!("condvar wait_timeout\n"));
        unimplemented!();
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        ::sys_common::util::dumb_print(format_args!("condvar destroy\n"));
        unimplemented!();
    }
}
unsafe impl Send for Condvar {}

unsafe impl Sync for Condvar {}
