use crate::{
    AsCStr, AtFlags, Syscall, Timespec, Timestamps, fd::AsFd, syscall_result_unit,
    syscall4_readonly,
};
use bitflags::bitflags;
use linux_raw_sys::general::{AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW};

#[inline]
pub fn utimensat<Fd: AsFd, P: AsCStr>(
    fd: Fd,
    path: P,
    times: &Timestamps,
    flags: AtFlags,
) -> crate::Result<()> {
    path.try_as_c_str(|path| {
        syscall_result_unit(unsafe {
            syscall4_readonly(
                Syscall::UTIMENSAT,
                fd.as_fd(),
                path,
                &raw const *times,
                flags.bits(),
            )
        })
    })
}
