use core::mem::MaybeUninit;

use crate::{Result, SysInfo, Syscall, syscall_result_unit, syscall1};

/// Return information about overall system statistics.
#[inline]
pub fn sysinfo() -> Result<SysInfo> {
    let mut info = MaybeUninit::<SysInfo>::uninit();

    unsafe {
        syscall_result_unit(syscall1(Syscall::SYSINFO, &mut info))?;

        Ok(info.assume_init())
    }
}
