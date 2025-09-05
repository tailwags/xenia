use crate::{Result, Syscall, fd::AsFd, syscall_result_unit, syscall1_readonly};

#[inline]
pub unsafe fn close<Fd: AsFd>(fd: Fd) -> Result<()> {
    syscall_result_unit(unsafe { syscall1_readonly(Syscall::CLOSE, fd.as_fd()) })
}
