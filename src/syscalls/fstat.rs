use core::{
    ffi::{c_long, c_uint, c_ulong},
    mem::MaybeUninit,
};

use crate::{AsCStr, Result, Stat, Syscall, fd::AsFd, syscall_result_unit, syscall2, syscall4};

#[inline]
pub fn fstat<Fd: AsFd>(fd: Fd) -> Result<Stat> {
    unsafe {
        let mut stat = MaybeUninit::<Stat>::uninit();
        syscall_result_unit(syscall2(Syscall::FSTAT, fd.as_fd(), &mut stat))?;
        Ok(stat.assume_init())
    }
}
