use crate::fd::{BorrowedFd, RawFd};

pub const STDIN_FILENO: RawFd = linux_raw_sys::general::STDIN_FILENO as _;
pub const STDOUT_FILENO: RawFd = linux_raw_sys::general::STDOUT_FILENO as _;
pub const STDERR_FILENO: RawFd = linux_raw_sys::general::STDERR_FILENO as _;

pub const fn stdout() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::borrow_raw(STDOUT_FILENO) }
}
