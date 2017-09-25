
use cell::UnsafeCell;
use libc;
use mem;

use cmsis_os as c;

pub struct Mutex {
    inner: c::osMutexDef_t,
    lock: UnsafeCell<c::osMutexId>,
}

impl Mutex {
    pub const fn new() -> Self {
        Mutex {
            inner: c::osMutexDef_t { dummy: 0 },
            lock: mem::uninitialized()
        }
    }

    pub unsafe fn init(&mut self) {
        let id = self.lock.get();
        id = c::osMutexCreate(&self.inner);
    }

    pub unsafe fn lock(&self) {
        let r = c::osMutexWait(self.lock.get(), 0);
        debug_assert_eq!(r, c::osStatus::osOK);
    }

    pub unsafe fn unlock(&self) {
        let r = c::osMutexRelease(self.lock.get(), 0);
        debug_assert_eq!(r, c::osStatus::osOK);
    }

    pub unsafe fn try_lock(&self) -> bool {
        let r = c::osMutexWait(self.lock.get(), 0);
        match r {
            c::osStatus::osOK => true,
            _ => false,
        }
    }

    pub unsafe fn destroy(&self) {
        let r = c::osMutexDelete(self.lock.get(), 0);
        debug_assert_eq!(r, c::osStatus::osOK);
    }
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub struct ReentrantMutex {
    pub mutex: Mutex,
    pub owner: UnsafeCell<c::osThreadId>,
    pub own_count: UnsafeCell<usize>,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {
            mutex: Mutex::new(),
            owner: mem::uninitialized(),
            own_count: mem::uninitialized()
    }

    pub unsafe fn init(&mut self) {
        let me = c::osThreadGetId();

        self.mutex.init();
        self.owner = UnsafeCell::new(me);
        self.own_count = UnsafeCell::new(0);
    }

    pub unsafe fn lock(&self) {
        let me = c::osThreadGetId();
        if *self.own_count.get() > 0 && self.owner.get().name == me.name {
            *self.own_count.get() += 1;
        } else {
            self.mutex.lock();
            self.owner.get() = me;
            *self.own_count.get() = 1;
        }
    }

    pub unsafe fn try_lock(&self) -> bool {
        let me = c::osThreadGetId();
        if *self.own_count.get() > 0 && self.owner.get().name == me.name {
            *self.own_count.get() += 1;
            true
        } else {
            if self.mutex.try_lock() {
                self.owner.get() = me;
                *self.own_count.get() = 1;
                true
            } else {
                false
            }
        }
    }

    pub unsafe fn unlock(&self) {
        let me = c::osThreadGetId();
        if *self.own_count.get() > 0 && self.owner.get().name == me.name {
            *self.own_count.get() -= 1;
            if *self.own_count.get() == 0 {
                self.owner.get() = mem::uninitialized();
                self.mutex.unlock();
            }
        }
    }

    pub unsafe fn destroy(&self) {
        self.mutex.destroy();
        self.owner.get() = mem::uninitialized();
        self.own_count.get() = mem::uninitialized();
    }
}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}
