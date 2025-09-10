use core::{ffi::c_ushort, mem::MaybeUninit};

use linux_raw_sys::ioctl;
use xenia::fd::AsFd;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

#[inline]
pub fn tcgetwinsize<Fd: AsFd>(fd: Fd) -> xenia::Result<Winsize> {
    unsafe {
        let mut winsize = MaybeUninit::<Winsize>::uninit();

        xenia::ioctl(fd, ioctl::TIOCGWINSZ, winsize.as_mut_ptr().cast())?;

        Ok(winsize.assume_init())
    }
}
