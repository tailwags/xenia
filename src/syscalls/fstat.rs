use core::mem::MaybeUninit;

use linux_raw_sys::general::{AT_FDCWD, stat};

use crate::{AsCStr, Result, Syscall, fd::AsFd, syscall_result_unit, syscall2, syscall4};

#[inline]
pub fn fstat<Fd: AsFd>(fd: Fd) -> Result<stat> {
    unsafe {
        let mut stat = MaybeUninit::<stat>::uninit();
        syscall_result_unit(syscall2(Syscall::FSTAT, fd.as_fd(), &mut stat))?;
        Ok(stat.assume_init())
    }
}
