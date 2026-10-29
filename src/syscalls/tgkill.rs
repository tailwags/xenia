use crate::{Pid, Result, Signal, Syscall, syscall_result_unit, syscall3_readonly};

/// Send `signal` to thread `pid` in thread group `tgid`. Passing `None` sends no signal but
/// still performs the existence and permission checks.
#[inline]
pub fn tgkill(tgid: Pid, pid: Pid, signal: Option<Signal>) -> Result<()> {
    syscall_result_unit(unsafe { syscall3_readonly(Syscall::TGKILL, tgid, pid, signal) })
}
