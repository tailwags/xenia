use core::{ffi::c_longlong, mem::MaybeUninit};

use linux_raw_sys::general::{
    __kernel_time64_t, __kernel_timespec, CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM, CLOCK_MONOTONIC,
    CLOCK_MONOTONIC_COARSE, CLOCK_MONOTONIC_RAW, CLOCK_PROCESS_CPUTIME_ID, CLOCK_REALTIME,
    CLOCK_REALTIME_ALARM, CLOCK_REALTIME_COARSE, CLOCK_SGI_CYCLE, CLOCK_TAI,
    CLOCK_THREAD_CPUTIME_ID,
};

use crate::{Syscall, syscall_result_unit, syscall2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ClockId {
    Realtime = CLOCK_REALTIME,
    Monotonic = CLOCK_MONOTONIC,
    ProcessCputimeId = CLOCK_PROCESS_CPUTIME_ID,
    ThreadCputimeId = CLOCK_THREAD_CPUTIME_ID,
    MonotonicRaw = CLOCK_MONOTONIC_RAW,
    RealtimeCoarse = CLOCK_REALTIME_COARSE,
    MonotonicCoarse = CLOCK_MONOTONIC_COARSE,
    Boottime = CLOCK_BOOTTIME,
    RealtimeAlarm = CLOCK_REALTIME_ALARM,
    BoottimeAlarm = CLOCK_BOOTTIME_ALARM,
    SgiCycle = CLOCK_SGI_CYCLE,
    Tai = CLOCK_TAI,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timespec {
    pub tv_sec: c_longlong,
    pub tv_nsec: c_longlong,
}

const _: () = crate::__assert_structs::<Timespec, linux_raw_sys::general::__kernel_timespec>();

#[inline]
pub fn clock_gettime(id: ClockId) -> crate::Result<Timespec> {
    let mut timespec = MaybeUninit::<Timespec>::uninit();

    unsafe {
        syscall_result_unit(syscall2(Syscall::CLOCK_GETTIME, id as u32, &mut timespec))?;

        Ok(timespec.assume_init())
    }
}
