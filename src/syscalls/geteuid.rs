use linux_raw_sys::general::__kernel_uid_t;

use crate::{Syscall, Uid, syscall0_readonly};

#[inline]
pub fn geteuid() -> Uid {
    unsafe { Uid::from_raw(syscall0_readonly(Syscall::GETEUID) as __kernel_uid_t) }
}
