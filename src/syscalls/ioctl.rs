use core::ffi::{c_int, c_uint, c_void};

use crate::{Syscall, fd::AsFd, syscall_result_c_int, syscall3};

#[inline]
pub unsafe fn ioctl<Fd: AsFd>(fd: Fd, cmd: c_uint, arg: *mut c_void) -> crate::Result<c_int> {
    syscall_result_c_int(unsafe { syscall3(Syscall::IOCTL, fd.as_fd(), cmd, arg) })
}
