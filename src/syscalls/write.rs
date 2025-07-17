use crate::{Result, Syscall, fd::AsFd, syscall3_readonly, syscall_result};

#[inline]
pub fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    syscall_result(unsafe { syscall3_readonly(Syscall::WRITE, fd, buf, buf.len()) })
}
