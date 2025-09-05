use core::ffi::{CStr, c_char};

use crate::{Errno, Syscall, syscall3_readonly};

#[inline]
pub unsafe fn execve(
    path: &CStr,
    args: *const *const c_char,
    env_vars: *const *const c_char,
) -> Errno {
    unsafe { Errno::from_raw(syscall3_readonly(Syscall::EXECVE, path, args, env_vars) as u16) }
}
