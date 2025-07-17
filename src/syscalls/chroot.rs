use core::ffi::CStr;

use crate::{Result, Syscall, syscall1_readonly};

pub fn chroot(path: &CStr) -> Result<()> {
    // FIXME
    unsafe {
        syscall1_readonly(Syscall::CHROOT, path);
    }
    Ok(())
}
