use crate::{AsCStr, Mode, Result, Syscall, stdio::cwd, syscall_result_unit, syscall3_readonly};

#[inline]
pub fn mkdir<P: AsCStr>(path: P, mode: Mode) -> Result<()> {
    path.try_as_c_str(|path| {
        syscall_result_unit(unsafe {
            syscall3_readonly(Syscall::MKDIRAT, cwd(), path, mode.bits())
        })
    })
}
