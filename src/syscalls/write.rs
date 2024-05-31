use crate::{fd::AsFd, syscall3_readonly, Result, Syscall};

#[inline]
pub fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    let res = unsafe { syscall3_readonly(Syscall::WRITE, fd, buf, buf.len()) };

    Ok(res)
}
