use crate::{Pid, RawPid, Syscall, syscall0_readonly};

#[inline]
pub fn getpid() -> Pid {
    unsafe { Pid::from_raw_unchecked(syscall0_readonly(Syscall::GETPID) as RawPid) }
}
