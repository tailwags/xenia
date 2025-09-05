use linux_raw_sys::general::__kernel_uid_t;

use crate::{Syscall, syscall0_readonly};

#[inline]
pub fn geteuid() -> __kernel_uid_t {
    unsafe { syscall0_readonly(Syscall::GETEUID) as __kernel_uid_t }
}
