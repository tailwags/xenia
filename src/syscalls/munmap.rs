use core::ffi::c_void;

use crate::{Result, Syscall, syscall_result_unit, syscall2};

/// Unmap all pages in the range `[addr, addr + len)`. `addr` must be page-aligned; `len` is
/// rounded up to a page boundary. The range may span several mappings, and unmapped pages within
/// it are not an error.
#[inline]
pub unsafe fn munmap(addr: *mut c_void, len: usize) -> Result<()> {
    syscall_result_unit(unsafe { syscall2(Syscall::MUNMAP, addr, len) })
}
