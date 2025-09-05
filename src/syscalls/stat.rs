use core::{ffi::CStr, mem::MaybeUninit};

use linux_raw_sys::general::{AT_FDCWD, stat};

use crate::{Result, Syscall, syscall_result_unit, syscall4};

#[inline]
pub fn stat(path: &CStr) -> Result<stat> {
    let mut stat = MaybeUninit::<stat>::uninit();

    unsafe {
        syscall_result_unit(syscall4(
            Syscall::NEWFSTATAT,
            AT_FDCWD,
            path,
            &mut stat,
            0usize,
        ))?;
        Ok(stat.assume_init())
    }
}
