use core::{
    ffi::{c_long, c_uint, c_ulong},
    mem::MaybeUninit,
};

use crate::{AsCStr, Result, Syscall, stdio::cwd, syscall_result_unit, syscall4};

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

#[inline]
pub fn stat<P: AsCStr>(path: P) -> Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();

    path.try_as_c_str(|path| unsafe {
        syscall_result_unit(syscall4(
            Syscall::NEWFSTATAT,
            cwd(),
            path,
            &mut stat,
            0usize,
        ))?;
        Ok(stat.assume_init())
    })
}
