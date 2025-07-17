use bitflags::bitflags;
use core::ffi::{CStr, c_uint};

use crate::{Result, Syscall, syscall5_readonly};

bitflags! {
    pub struct MountFlags: c_uint {
        const MOVE = linux_raw_sys::general::MS_MOVE;
        const RDONLY = linux_raw_sys::general::MS_RDONLY;

        const _ = !0;
    }
}

pub fn mount(
    source: Option<&CStr>,
    target: &CStr,
    file_system_type: Option<&CStr>,
    flags: MountFlags,
    data: Option<&CStr>,
) -> Result<()> {
    // FIXME
    unsafe {
        syscall5_readonly(
            Syscall::MOUNT,
            source,
            target,
            file_system_type,
            flags.bits(),
            data,
        );
    }

    Ok(())
}
