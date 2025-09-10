use core::{ffi::c_ushort, mem::MaybeUninit};
#[cfg(feature = "std")]
use std::ffi::CString;
#[cfg(feature = "std")]
use xenia::{Errno, fd::AsRawFd, readlinkat, stat, stdio::cwd};

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

#[inline]
pub fn isatty<Fd: AsFd>(fd: Fd) -> bool {
    tcgetwinsize(fd).is_ok()
}

#[cfg(feature = "std")]
pub fn ttyname<Fd: AsFd, B: Into<Vec<u8>>>(fd: Fd, buf: B) -> xenia::Result<CString> {
    // If we are not a tty there's no point in trying any other method
    if !isatty(&fd) {
        return Err(Errno::NOTTY);
    }

    /*
    FIXME: we can def do some trickery here like rustix does to avoid allocating.

    Not the biggest priority since it's more of performance optization thinghy considering we are on std anyway
     */
    let fd_path = format!("/proc/self/fd/{}", fd.as_fd().as_raw_fd());
    let path = readlinkat(cwd(), &fd_path, buf)?;

    let fd_stat = stat(fd_path)?;
    let path_stat = stat(&path)?;

    if path_stat.st_dev != fd_stat.st_dev || path_stat.st_ino != fd_stat.st_ino {
        return Err(Errno::NODEV);
    }

    Ok(path)
}
