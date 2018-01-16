
use alloc::boxed::FnBox;
use cty;
use cmsis_os;
use io;
use mem;
use sys_common::thread::*;
use ffi::CStr;
use time::Duration;
use ptr;

pub const DEFAULT_MIN_STACK_SIZE: usize = cmsis_os::CMSIS_CFG_DEFAULT_STACK as usize;

pub struct Thread {
    id: cmsis_os::osThreadId,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        let p = Box::new(p);
        let def = cmsis_os::osThreadDef_t {
            pthread: thread_start,
            tpriority: cmsis_os::osPriority::Normal,
            stacksize: stack as u32,
            name: ptr::null_mut() as *const _,
        };
        let id = cmsis_os::osThreadCreate(&def, &*p as *const _ as *mut _);
        if id.is_null() {
            Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to create thread",
                    ))
        } else {
            Ok(Thread { id: id })
        }

        extern fn thread_start(main: *mut cty::c_void) {
            unsafe {
                start_thread(main);
            }
        }
    }

    pub fn yield_now() {
        let ret = cmsis_os::osThreadYield();
        debug_assert_eq!(ret, cmsis_os::osStatus::OK);
    }

    pub fn set_name(name: &CStr) {
        let id = cmsis_os::osThreadGetId();
        if !id.is_null() {
            id.name = name.as_ptr() as *const cty::c_char;
        }
    }

    pub fn sleep(dur: Duration) {
        let millis = (dur.as_secs() * 1000 + dur.subsec_nanos() as u64 / 1000) as u32;
        let ret = cmsis_os::osDelay(millis);
        debug_assert_eq!(ret, cmsis_os::osStatus::osEventTimeout);
    }

    pub fn join(self) {
        let ret = cmsis_os::osThreadTerminate(self.id);
        mem::forget(self);
        debug_assert!(ret == cmsis_os::osStatus::osOK, "failed to join thread: {:?}", ret);
    }

    pub fn id(&self) -> cmsis_os::osThreadId {
        self.id
    }

    pub fn into_id(self) -> cmsis_os::osThreadId {
        let id = self.id;
        mem::forget(self);
        id
    }
}

#[cfg_attr(test, allow(dead_code))]
pub mod guard {
    pub unsafe fn current() -> Option<usize> {
        None
    }
    pub unsafe fn init() -> Option<usize> {
        None
    }
}
