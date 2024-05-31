use linux_raw_sys::general::STDOUT_FILENO;

use crate::fd::BorrowedFd;

pub const fn stdout() -> BorrowedFd<'static> {
    unsafe { BorrowedFd::new(STDOUT_FILENO) }
}
