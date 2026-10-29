use core::ffi::c_void;

use crate::{MmapFlags, MmapProt, Result, Syscall, fd::RawFd, syscall_result, syscall6};

/// Map memory into the process address space.
///
/// `addr` is a hint — pass `null_mut()` to let the kernel choose. `len` is rounded up to a page
/// boundary by the kernel. For anonymous mappings pass `fd = -1` and `offset = 0`.
#[inline]
pub unsafe fn mmap(
    addr: *mut c_void,
    len: usize,
    prot: MmapProt,
    flags: MmapFlags,
    fd: RawFd,
    offset: i64,
) -> Result<*mut c_void> {
    syscall_result(unsafe {
        syscall6(
            Syscall::MMAP,
            addr,
            len,
            prot.bits(),
            flags.bits(),
            fd,
            offset as usize,
        )
    })
    .map(|p| p as *mut c_void)
}

/// Map anonymous memory into the process address space (convenience wrapper).
///
/// Equivalent to `mmap(addr, len, prot, flags | MmapFlags::ANONYMOUS, -1, 0)`.
#[inline]
pub unsafe fn mmap_anon(
    addr: *mut c_void,
    len: usize,
    prot: MmapProt,
    flags: MmapFlags,
) -> Result<*mut c_void> {
    unsafe { mmap(addr, len, prot, flags | MmapFlags::ANONYMOUS, -1, 0) }
}
