use core::{ffi::c_longlong, mem::MaybeUninit};

use linux_raw_sys::general::{
    __kernel_time64_t, __kernel_timespec, CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM, CLOCK_MONOTONIC,
    CLOCK_MONOTONIC_COARSE, CLOCK_MONOTONIC_RAW, CLOCK_PROCESS_CPUTIME_ID, CLOCK_REALTIME,
    CLOCK_REALTIME_ALARM, CLOCK_REALTIME_COARSE, CLOCK_SGI_CYCLE, CLOCK_TAI,
    CLOCK_THREAD_CPUTIME_ID,
};

use crate::{ClockId, Syscall, Timespec, syscall_result_unit, syscall2};

#[inline]
pub fn clock_gettime(id: ClockId) -> crate::Result<Timespec> {
    let mut timespec = MaybeUninit::<Timespec>::uninit();

    unsafe {
        syscall_result_unit(syscall2(Syscall::CLOCK_GETTIME, id as u32, &mut timespec))?;

        Ok(timespec.assume_init())
    }
}
