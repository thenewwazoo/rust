
use cty;
use cmsis_os as c;
use io;
use mem;
use sys_common::thread::*;
use std::ffi::CStr;
use time::Duration;
use ptr;

pub const DEFAULT_MIN_STACK_SIZE: usize = c::CMSIS_CFG_DEFAULT_STACK as usize;

pub struct Thread {
    id: c::osThreadId,
}

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    pub unsafe fn new<'a>(_stack: usize, p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        let p = box p;
        let def = c::osThreadDef_t {
            pthread: thread_start,
            tpriority: c::osPriority::Normal,
            stacksize: _stack as u32,
            name: ptr::null_mut() as *const _,
        };
        let id = c::osThreadCreate(&def, &p as *const _ as *mut _);
        if id.is_null() {
            Err(io::Err::new(
                io::ErrorKind::Other,
                "failed to create thread",
            ))
        } else {
            Ok(Thread { id: id })
        }
    }

    extern "C" fn thread_start(main: *mut cty::c_void) {
        unsafe {
            start_thread(main);
        }
    }

    pub fn yield_now() {
        let ret = c::osThreadYield();
        assert_debug_eq!(ret, c::osStatus::OK);
    }

    pub fn set_name(name: &CStr) {
        let id = c::osThreadGetId();
        if !id.is_null() {
            id.name = name.as_ptr() as *const cty::c_char;
        }
    }

    pub fn sleep(dur: Duration) {
        let millis = (dur.as_secs() * 1000 + dur.subsec_nanos() / 1000) as u32;
        let ret = c::osDelay(millis);
        assert_debug_eq!(ret, c::osStatus::osEventTimeout);
    }

    pub fn join(self) {
        let ret = c::osThreadTerminate(self.id);
        mem::forget(self);
        assert!(ret == c::osStatus::osOK, "failed to join thread: {:?}", ret);
    }

    pub fn id(&self) -> c::osThreadId {
        self.id
    }

    pub fn into_id(self) -> c::osThreadId {
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
