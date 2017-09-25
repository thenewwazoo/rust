
use cmsis_os as c;
use time::Duration;
use sys_common::mul_div_u64;

const NANOS_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime {
    t: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    t: u32,
}

impl Instant {
    pub fn now() -> Instant {
        Instant { t: c::osKernelSysTick() }
    }

    pub fn sub_instant(&self, other: &Instant) -> Duration {
        let diff = (self.t as u64).checked_sub(other.t as u64).expect(
            "specified instant was later than self",
        );
        let nanos = mul_div_u64(diff, NANOS_PER_SEC, c::osKernelSysTickFrequency as u64);
        Duration::new(nanos / NANOS_PER_SEC, (nanos % NANOS_PER_SEC) as u32)
    }

    pub fn add_duration(&self, other: &Duration) -> Instant {
        let freq = c::osKernelSysTickFrequency as u64;
        let t = other
            .as_secs()
            .checked_mul(freq)
            .and_then(|i| (self.t as u64).checked_add(i))
            .and_then(|i| {
                i.checked_add(mul_div_u64(
                    other.subsec_nanos() as u64,
                    freq,
                    NANOS_PER_SEC,
                ))
            })
            .expect("overflow when adding duration to time");
        Instant { t: t as u32 }
    }

    pub fn sub_duration(&self, other: &Duration) -> Instant {
        let freq = c::osKernelSysTickFrequency as u64;
        let t = other
            .as_secs()
            .checked_mul(freq)
            .and_then(|i| (self.t as u64).checked_sub(i))
            .and_then(|i| {
                i.checked_sub(mul_div_u64(
                    other.subsec_nanos() as u64,
                    freq,
                    NANOS_PER_SEC,
                ))
            })
            .expect("overflow when subtracting duration from time");
        Instant { t: t as c::LARGE_INTEGER }
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        SystemTime { t: c::osKernelSysTick() }
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        if self.t >= other.t {
            Ok(ticks2dur((self.t - other.t) as u64))
        } else {
            Ok(ticks2dur((other.t - self.t) as u64))
        }
    }

    pub fn add_duration(&self, other: &Duration) -> SystemTime {
        let ticks = self.t.checked_add(dur2ticks(other)).expect(
            "overflow when adding duration to time",
        );
        SystemTime { t: ticks }
    }

    pub fn sub_duration(&self, other: &Duration) -> SystemTime {
        let ticks = self.t.checked_sub(dur2ticks(other)).expect(
            "overflow when subtracting from time",
        );
        SystemTime { t: ticks }
    }
}

impl PartialEq for SystemTime {
    fn eq(&self, other: &SystemTime) -> bool {
        self.t == other.t
    }
}

impl Eq for SystemTime {}

impl PartialOrd for SystemTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SystemTime {
    fn cmp(&self, other: &SystemTime) -> Ordering {
        self.t.cmp(&other.t)
    }
}

fn dur2ticks(d: &Duration) -> i64 {
    d.as_secs()
        .checked_mul(c::osKernelSysTickFrequency)
        .and_then(|i| i.checked_add(d.subsec_nanos() as u64 / 100))
        .and_then(|i| i.try_into().ok())
        .expect("overflow when converting duration to ticks")
}

fn ticks2dur(ticks: u64) -> Duration {
    Duration::new(
        ticks / c::osKernelSysTickFrequency,
        ((ticks % c::osKernelSysTickFrequency) * 100) as u32,
    )
}

pub const UNIX_EPOCH: SystemTime = SystemTime(0);
