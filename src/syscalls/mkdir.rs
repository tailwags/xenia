use core::ffi::CStr;

use linux_raw_sys::general::AT_FDCWD;

use crate::{Mode, Result, Syscall, syscall_result_unit, syscall3_readonly};

#[inline]
pub fn mkdir(path: &CStr, mode: Mode) -> Result<()> {
    syscall_result_unit(unsafe { syscall3_readonly(Syscall::MKDIRAT, AT_FDCWD, path, mode.bits()) })
}
