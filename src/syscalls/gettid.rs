use crate::{Pid, RawPid, Syscall, syscall0_readonly};

#[inline]
pub fn gettid() -> Pid {
    unsafe { Pid::from_raw_unchecked(syscall0_readonly(Syscall::GETTID) as RawPid) }
}
