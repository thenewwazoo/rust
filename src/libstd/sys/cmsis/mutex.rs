

use cell::UnsafeCell;
use mem;

use cmsis_os;

pub struct Mutex {
    inner: cmsis_os::osMutexDef_t,
    lock: UnsafeCell<cmsis_os::osMutexId>,
}

impl Mutex {
    pub const fn new() -> Self {
        Mutex {
            inner: cmsis_os::osMutexDef_t { dummy: 0 },
            lock: mem::uninitialized(),
        }
    }

    pub unsafe fn init(&mut self) {
        let id = self.lock.get();
        id = cmsis_os::osMutexCreate(&self.inner);
    }

    pub unsafe fn lock(&self) {
        let r = cmsis_os::osMutexWait(self.lock.get(), 0);
        debug_assert_eq!(r, cmsis_os::osStatus::osOK);
    }

    pub unsafe fn unlock(&self) {
        let r = cmsis_os::osMutexRelease(self.lock.get(), 0);
        debug_assert_eq!(r, cmsis_os::osStatus::osOK);
    }

    pub unsafe fn try_lock(&self) -> bool {
        let r = cmsis_os::osMutexWait(self.lock.get(), 0);
        match r {
            cmsis_os::osStatus::osOK => true,
            _ => false,
        }
    }

    pub unsafe fn destroy(&self) {
        let r = cmsis_os::osMutexDelete(self.lock.get(), 0);
        debug_assert_eq!(r, cmsis_os::osStatus::osOK);
    }
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub struct ReentrantMutex {
    pub mutex: Mutex,
    pub owner: UnsafeCell<cmsis_os::osThreadId>,
    pub own_count: UnsafeCell<usize>,
}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {
            mutex: Mutex::new(),
            owner: mem::uninitialized(),
            own_count: mem::uninitialized(),
        }
    }

    pub unsafe fn init(&mut self) {
        let me = cmsis_os::osThreadGetId();

        self.mutex.init();
        self.owner = UnsafeCell::new(me);
        self.own_count = UnsafeCell::new(0);
    }

    pub unsafe fn lock(&self) {
        let me = cmsis_os::osThreadGetId();
        if *self.own_count.get() > 0 && self.owner.get().name == me.name {
            *self.own_count.get() += 1;
        } else {
            self.mutex.lock();
            self.owner.get() = me;
            *self.own_count.get() = 1;
        }
    }

    pub unsafe fn try_lock(&self) -> bool {
        let me = cmsis_os::osThreadGetId();
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
        let me = cmsis_os::osThreadGetId();
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
