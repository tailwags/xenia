use crate::{AsCStr, Syscall, Timespec, fd::AsFd, syscall_result_unit, syscall4_readonly};
use bitflags::bitflags;
use linux_raw_sys::general::{AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamps {
    pub last_access: Timespec,
    pub last_modification: Timespec,
}

const _: () = crate::__assert_structs::<Timestamps, [Timespec; 2]>();

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AtFlags: u32 {
        const EMPTY_PATH = AT_EMPTY_PATH;
        const SYMLINK_NOFOLLOW = AT_SYMLINK_NOFOLLOW;
        const _ = !0;
    }
}

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
