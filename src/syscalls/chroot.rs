use crate::{AsCStr, Result, Syscall, syscall_result_unit, syscall1_readonly};

#[inline]
pub fn chroot<P: AsCStr>(path: P) -> Result<()> {
    path.try_as_c_str(|path| {
        syscall_result_unit(unsafe { syscall1_readonly(Syscall::CHROOT, path) })
    })
}
