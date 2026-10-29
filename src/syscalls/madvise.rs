use core::ffi::c_void;

use crate::{Advice, Result, Syscall, syscall_result_unit, syscall3};

/// Give advice about use of memory.
#[inline]
pub unsafe fn madvise(addr: *mut c_void, len: usize, advice: Advice) -> Result<()> {
    syscall_result_unit(unsafe { syscall3(Syscall::MADVISE, addr, len, advice as u32) })
}
