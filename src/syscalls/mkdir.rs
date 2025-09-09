use linux_raw_sys::general::AT_FDCWD;

use crate::{AsCStr, Mode, Result, Syscall, syscall_result_unit, syscall3_readonly};

#[inline]
pub fn mkdir<P: AsCStr>(path: P, mode: Mode) -> Result<()> {
    path.try_as_c_str(|path| {
        syscall_result_unit(unsafe {
            syscall3_readonly(Syscall::MKDIRAT, AT_FDCWD, path, mode.bits())
        })
    })
}
