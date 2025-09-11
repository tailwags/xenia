use bitflags::bitflags;
use core::ffi::{CStr, c_uint};

use crate::{MountFlags, Result, Syscall, syscall_result_unit, syscall5_readonly};

#[inline]
pub fn mount(
    source: Option<&CStr>,
    target: &CStr,
    file_system_type: Option<&CStr>,
    flags: MountFlags,
    data: Option<&CStr>,
) -> Result<()> {
    syscall_result_unit(unsafe {
        syscall5_readonly(
            Syscall::MOUNT,
            source,
            target,
            file_system_type,
            flags.bits(),
            data,
        )
    })
}
