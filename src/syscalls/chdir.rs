use core::ffi::CStr;

use crate::{Result, Syscall, syscall1_readonly};

pub fn chdir(path: &CStr) -> Result<()> {
    // FIXME
    let _ret = unsafe { syscall1_readonly(Syscall::CHDIR, path) };

    Ok(())
}
