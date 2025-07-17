use crate::{Errno, Result, Syscall, fd::AsFd, syscall3_readonly};

#[inline]
pub fn write<Fd: AsFd>(fd: Fd, buf: &[u8]) -> Result<usize> {
    let res = unsafe { syscall3_readonly(Syscall::WRITE, fd, buf, buf.len()) };

    if (-4095..0).contains(&(res as isize)) {
        return Err(unsafe { Errno::from_raw(res as u16) });
    }

    Ok(res)
}
