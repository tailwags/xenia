use crate::{Result, Syscall, syscall_result_c_int, syscall5};

/// Perform operations on a process or thread.
#[inline]
pub unsafe fn prctl(option: i32, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<i32> {
    syscall_result_c_int(unsafe { syscall5(Syscall::PRCTL, option, a2, a3, a4, a5) })
}
