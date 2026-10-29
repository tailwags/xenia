use core::ffi::c_void;

use crate::{MmapProt, Result, Syscall, syscall_result_unit, syscall3};

/// Change protection on a region of memory.
#[inline]
pub unsafe fn mprotect(addr: *mut c_void, len: usize, prot: MmapProt) -> Result<()> {
    syscall_result_unit(unsafe { syscall3(Syscall::MPROTECT, addr, len, prot.bits()) })
}
