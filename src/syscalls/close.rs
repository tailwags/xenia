use crate::{Syscall, fd::AsFd, syscall1_readonly};

#[inline]
pub unsafe fn close<Fd: AsFd>(fd: Fd) {
    unsafe {
        let _ret = syscall1_readonly(Syscall::CLOSE, fd);
    }
}
