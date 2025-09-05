use core::ffi::CStr;

use crate::{Result, Syscall, syscall_result_unit, syscall1_readonly};

#[inline]
pub fn chdir(path: &CStr) -> Result<()> {
    syscall_result_unit(unsafe { syscall1_readonly(Syscall::CHDIR, path) })
}
