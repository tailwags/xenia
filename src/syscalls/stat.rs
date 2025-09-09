use core::mem::MaybeUninit;

use linux_raw_sys::general::{AT_FDCWD, stat};

use crate::{AsCStr, Result, Syscall, syscall_result_unit, syscall4};

#[inline]
pub fn stat<P: AsCStr>(path: P) -> Result<stat> {
    let mut stat = MaybeUninit::<stat>::uninit();

    path.try_as_c_str(|path| unsafe {
        syscall_result_unit(syscall4(
            Syscall::NEWFSTATAT,
            AT_FDCWD,
            path,
            &mut stat,
            0usize,
        ))?;
        Ok(stat.assume_init())
    })
}
