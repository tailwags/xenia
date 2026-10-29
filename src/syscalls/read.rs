use crate::{Result, Syscall, fd::AsFd, syscall_result, syscall3};

/// Read bytes from a file descriptor into a buffer.
#[inline]
pub fn read<Fd: AsFd>(fd: Fd, buf: &mut [u8]) -> Result<usize> {
    let len = buf.len();
    syscall_result(unsafe { syscall3(Syscall::READ, fd.as_fd(), buf, len) })
}
