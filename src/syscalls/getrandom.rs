use crate::{GetRandomFlags, Result, Syscall, syscall_result, syscall3};

/// Fill `buf` with random bytes from the kernel entropy pool.
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> Result<usize> {
    let len = buf.len();
    syscall_result(unsafe { syscall3(Syscall::GETRANDOM, buf, len, flags.bits()) })
}
