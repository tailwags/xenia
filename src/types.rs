use bitflags::bitflags;
use core::ffi::{c_long, c_longlong, c_uint, c_ulong};
use linux_raw_sys::general::{
    __kernel_mode_t, AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW, CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM,
    CLOCK_MONOTONIC, CLOCK_MONOTONIC_COARSE, CLOCK_MONOTONIC_RAW, CLOCK_PROCESS_CPUTIME_ID,
    CLOCK_REALTIME, CLOCK_REALTIME_ALARM, CLOCK_REALTIME_COARSE, CLOCK_SGI_CYCLE, CLOCK_TAI,
    CLOCK_THREAD_CPUTIME_ID,
};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Mode: __kernel_mode_t {

        const _ = !0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AtFlags: u32 {
        const EMPTY_PATH = AT_EMPTY_PATH;
        const SYMLINK_NOFOLLOW = AT_SYMLINK_NOFOLLOW;
        const _ = !0;
    }
}

bitflags! {
    pub struct MountFlags: c_uint {
        const MOVE = linux_raw_sys::general::MS_MOVE;
        const RDONLY = linux_raw_sys::general::MS_RDONLY;

        const _ = !0;
    }
}

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

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamps {
    pub last_access: Timespec,
    pub last_modification: Timespec,
}

const _: () = crate::__assert_structs::<Timestamps, [Timespec; 2]>();

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_uint,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atime: c_ulong,
    pub st_atime_nsec: c_ulong,
    pub st_mtime: c_ulong,
    pub st_mtime_nsec: c_ulong,
    pub st_ctime: c_ulong,
    pub st_ctime_nsec: c_ulong,
    __unused: [c_long; 3usize],
}

const _: () = crate::__assert_structs::<Stat, linux_raw_sys::general::stat>();
