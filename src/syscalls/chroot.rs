use core::ffi::CStr;

use crate::{Result, Syscall, syscall1_readonly, syscall_result_unit};

pub fn chroot(path: &CStr) -> Result<()> {
    syscall_result_unit(unsafe { syscall1_readonly(Syscall::CHROOT, path) })
}
