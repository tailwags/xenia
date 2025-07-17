use crate::{Result, Syscall, fd::AsFd, syscall1_readonly, syscall_result_unit};

#[inline]
pub unsafe fn close<Fd: AsFd>(fd: Fd) -> Result<()> {
    syscall_result_unit(unsafe { syscall1_readonly(Syscall::CLOSE, fd) })
}
