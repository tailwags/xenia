use core::mem::MaybeUninit;

use crate::{Result, Rusage, Syscall, UsageWho, syscall_result_unit, syscall2};

/// Get resource usage statistics.
#[inline]
pub fn getrusage(who: UsageWho) -> Result<Rusage> {
    let mut usage = MaybeUninit::<Rusage>::uninit();

    unsafe {
        syscall_result_unit(syscall2(Syscall::GETRUSAGE, who as i32, &mut usage))?;

        Ok(usage.assume_init())
    }
}
