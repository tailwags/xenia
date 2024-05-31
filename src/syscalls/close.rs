use crate::{fd::AsFd, syscall1_readonly, Syscall};

#[inline]
pub unsafe fn close<Fd: AsFd>(fd: Fd) {
    let _ret = syscall1_readonly(Syscall::CLOSE, fd);
}
