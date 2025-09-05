use linux_raw_sys::general::__kernel_pid_t;

use crate::{Syscall, syscall0_readonly};

#[inline]
pub fn getpid() -> __kernel_pid_t {
    unsafe { syscall0_readonly(Syscall::GETPID) as __kernel_pid_t }
}
