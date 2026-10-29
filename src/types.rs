use bitflags::bitflags;
use core::ffi::{c_long, c_longlong, c_uint, c_ulong};
use linux_raw_sys::general::{
    __kernel_mode_t, AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW, CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM,
    CLOCK_MONOTONIC, CLOCK_MONOTONIC_COARSE, CLOCK_MONOTONIC_RAW, CLOCK_PROCESS_CPUTIME_ID,
    CLOCK_REALTIME, CLOCK_REALTIME_ALARM, CLOCK_REALTIME_COARSE, CLOCK_SGI_CYCLE, CLOCK_TAI,
    CLOCK_THREAD_CPUTIME_ID, UTIME_NOW, UTIME_OMIT,
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
    pub struct AtFlags: c_uint {
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

bitflags! {
    pub struct OFlags: c_uint {
        const RDONLY = linux_raw_sys::general::O_RDONLY;
        const WRONLY = linux_raw_sys::general::O_WRONLY;
        const RDWR = linux_raw_sys::general::O_RDWR;
        const APPEND = linux_raw_sys::general::O_APPEND;
        const CREAT = linux_raw_sys::general::O_CREAT;
        const DSYNC = linux_raw_sys::general::O_DSYNC;
        const EXCL = linux_raw_sys::general::O_EXCL;
        const NOCTTY = linux_raw_sys::general::O_NOCTTY;
        const NONBLOCK = linux_raw_sys::general::O_NONBLOCK;

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

impl Timespec {
    pub const NOW: Self = Self {
        tv_sec: 0,
        tv_nsec: UTIME_NOW as _,
    };
    pub const OMIT: Self = Self {
        tv_sec: 0,
        tv_nsec: UTIME_OMIT as _,
    };
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

bitflags! {
    /// Memory protection flags for `mmap` and `mprotect`. Use `MmapProt::empty()` for
    /// `PROT_NONE`.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MmapProt: c_uint {
        const READ = linux_raw_sys::general::PROT_READ;
        const WRITE = linux_raw_sys::general::PROT_WRITE;
        const EXEC = linux_raw_sys::general::PROT_EXEC;
        const SEM = linux_raw_sys::general::PROT_SEM;
        const GROWSDOWN = linux_raw_sys::general::PROT_GROWSDOWN;
        const GROWSUP = linux_raw_sys::general::PROT_GROWSUP;

        const _ = !0;
    }
}

bitflags! {
    /// Mapping flags for `mmap`.
    ///
    /// The huge page size is a multi-bit field rather than a flag, so it is set with
    /// [`MmapFlags::huge_pages`] instead of a constant.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MmapFlags: c_uint {
        const SHARED = linux_raw_sys::general::MAP_SHARED;
        const PRIVATE = linux_raw_sys::general::MAP_PRIVATE;
        /// Note that this is `SHARED | PRIVATE` bitwise, so `contains(SHARED)` and
        /// `contains(PRIVATE)` are both true for it.
        const SHARED_VALIDATE = linux_raw_sys::general::MAP_SHARED_VALIDATE;
        const DROPPABLE = linux_raw_sys::general::MAP_DROPPABLE;
        const FIXED = linux_raw_sys::general::MAP_FIXED;
        const FIXED_NOREPLACE = linux_raw_sys::general::MAP_FIXED_NOREPLACE;
        const ANONYMOUS = linux_raw_sys::general::MAP_ANONYMOUS;
        const _32BIT = linux_raw_sys::general::MAP_32BIT;
        const GROWSDOWN = linux_raw_sys::general::MAP_GROWSDOWN;
        const LOCKED = linux_raw_sys::general::MAP_LOCKED;
        const NORESERVE = linux_raw_sys::general::MAP_NORESERVE;
        const POPULATE = linux_raw_sys::general::MAP_POPULATE;
        const NONBLOCK = linux_raw_sys::general::MAP_NONBLOCK;
        const STACK = linux_raw_sys::general::MAP_STACK;
        const HUGETLB = linux_raw_sys::general::MAP_HUGETLB;
        const SYNC = linux_raw_sys::general::MAP_SYNC;
        // `MAP_UNINITIALIZED` is omitted: it only has an effect on no-MMU kernels, and its bit
        // overlaps the huge page size field.

        const _ = !0;
    }
}

impl MmapFlags {
    /// `HUGETLB` with an explicit huge page size, replacing any size already set in `self`.
    pub const fn huge_pages(self, size: HugePageSize) -> Self {
        let mask = linux_raw_sys::general::MAP_HUGE_MASK << linux_raw_sys::general::MAP_HUGE_SHIFT;
        Self::from_bits_retain((self.bits() & !mask) | Self::HUGETLB.bits() | size as c_uint)
    }

    /// The huge page size set by [`MmapFlags::huge_pages`], or `None` if it is unset or unknown.
    pub const fn huge_page_size(self) -> Option<HugePageSize> {
        let shift = (self.bits() >> linux_raw_sys::general::MAP_HUGE_SHIFT)
            & linux_raw_sys::general::MAP_HUGE_MASK;
        HugePageSize::from_shift(shift)
    }
}

/// Huge page sizes for [`MmapFlags::huge_pages`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum HugePageSize {
    Size64KB = linux_raw_sys::general::MAP_HUGE_64KB,
    Size512KB = linux_raw_sys::general::MAP_HUGE_512KB,
    Size1MB = linux_raw_sys::general::MAP_HUGE_1MB,
    Size2MB = linux_raw_sys::general::MAP_HUGE_2MB,
    Size8MB = linux_raw_sys::general::MAP_HUGE_8MB,
    Size16MB = linux_raw_sys::general::MAP_HUGE_16MB,
    Size32MB = linux_raw_sys::general::MAP_HUGE_32MB,
    Size256MB = linux_raw_sys::general::MAP_HUGE_256MB,
    Size512MB = linux_raw_sys::general::MAP_HUGE_512MB,
    Size1GB = linux_raw_sys::general::MAP_HUGE_1GB,
    Size2GB = linux_raw_sys::general::MAP_HUGE_2GB,
    Size16GB = linux_raw_sys::general::MAP_HUGE_16GB,
}

impl HugePageSize {
    const ALL: [Self; 12] = [
        Self::Size64KB,
        Self::Size512KB,
        Self::Size1MB,
        Self::Size2MB,
        Self::Size8MB,
        Self::Size16MB,
        Self::Size32MB,
        Self::Size256MB,
        Self::Size512MB,
        Self::Size1GB,
        Self::Size2GB,
        Self::Size16GB,
    ];

    /// Looks up a size from its log2 in bytes (e.g. 21 for 2 MiB).
    pub const fn from_shift(shift: u32) -> Option<Self> {
        if shift > linux_raw_sys::general::MAP_HUGE_MASK {
            return None;
        }
        let mut i = 0;
        while i < Self::ALL.len() {
            if Self::ALL[i] as u32 == shift << linux_raw_sys::general::MAP_HUGE_SHIFT {
                return Some(Self::ALL[i]);
            }
            i += 1;
        }
        None
    }
}
