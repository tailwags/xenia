use crate::{Result, Syscall, fd::AsFd, syscall_result, syscall3_readonly};

#[inline]
pub fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    syscall_result(unsafe { syscall3_readonly(Syscall::WRITE, fd.as_fd(), buf, buf.len()) })
}
