use core::ffi::c_int;

use crate::{Syscall, syscall1_noreturn};

#[inline]
pub fn exit_group(exit_code: c_int) -> ! {
    unsafe { syscall1_noreturn(Syscall::EXIT_GROUP, exit_code) }
}
