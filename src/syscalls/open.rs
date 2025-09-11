use crate::{
    AsCStr, Mode, OFlags, Syscall, fd::OwnedFd, syscall_result_owned_fd, syscall3_readonly,
};

#[inline]
pub fn open<P: AsCStr>(path: P, flags: OFlags, mode: Mode) -> crate::Result<OwnedFd> {
    unsafe {
        path.try_as_c_str(|path| {
            syscall_result_owned_fd(syscall3_readonly(
                Syscall::OPEN,
                path,
                flags.bits(),
                mode.bits(),
            ))
        })
    }
}
