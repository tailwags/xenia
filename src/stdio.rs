use crate::fd::{BorrowedFd, RawFd};

pub const STDIN_FILENO: RawFd = linux_raw_sys::general::STDIN_FILENO as _;
pub const STDOUT_FILENO: RawFd = linux_raw_sys::general::STDOUT_FILENO as _;
pub const STDERR_FILENO: RawFd = linux_raw_sys::general::STDERR_FILENO as _;
pub const CWD: RawFd = linux_raw_sys::general::AT_FDCWD as _;

pub const fn stdin() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::borrow_raw(STDIN_FILENO) }
}

pub const fn stdout() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::borrow_raw(STDOUT_FILENO) }
}

pub const fn stderr() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::borrow_raw(STDERR_FILENO) }
}

pub const fn cwd() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::borrow_raw(CWD) }
}
