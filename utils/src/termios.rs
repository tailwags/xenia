use core::{ffi::c_ushort, mem::MaybeUninit};

#[cfg(feature = "alloc")]
use alloc::{ffi::CString, vec::Vec};
#[cfg(feature = "alloc")]
use core::ffi::CStr;
#[cfg(feature = "alloc")]
use xenia::{
    Errno,
    fd::{AsRawFd, RawFd},
    fstat, readlinkat, stat,
    stdio::cwd,
};

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

const _: () = xenia::__assert_structs::<Winsize, linux_raw_sys::general::winsize>();

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

#[cfg(feature = "alloc")]
const PROC_SELF_FD: &[u8] = b"/proc/self/fd/";

/// Room for the prefix, the 10 digits of any `RawFd` and the nul terminator
#[cfg(feature = "alloc")]
const PROC_SELF_FD_PATH_LEN: usize = PROC_SELF_FD.len() + 10 + 1;

#[cfg(feature = "alloc")]
pub fn ttyname<Fd: AsFd, B: Into<Vec<u8>>>(fd: Fd, buf: B) -> xenia::Result<CString> {
    // If we are not a tty there's no point in trying any other method
    if !isatty(&fd) {
        return Err(Errno::NOTTY);
    }

    let mut fd_path = [0; PROC_SELF_FD_PATH_LEN];
    let fd_path = proc_self_fd_path(fd.as_fd().as_raw_fd(), &mut fd_path);
    let path = readlinkat(cwd(), fd_path, buf)?;

    let fd_stat = fstat(&fd)?;
    let path_stat = stat(&path)?;

    if path_stat.st_dev != fd_stat.st_dev || path_stat.st_ino != fd_stat.st_ino {
        return Err(Errno::NODEV);
    }

    Ok(path)
}

/// Formats `/proc/self/fd/<fd>` into `buf` without allocating.
#[cfg(feature = "alloc")]
fn proc_self_fd_path(fd: RawFd, buf: &mut [u8; PROC_SELF_FD_PATH_LEN]) -> &CStr {
    let (prefix, digits) = buf.split_at_mut(PROC_SELF_FD.len());
    prefix.copy_from_slice(PROC_SELF_FD);

    // Open file descriptors are never negative
    let mut fd = fd.cast_unsigned();
    let mut len = 0;

    loop {
        digits[len] = b'0' + (fd % 10) as u8;
        len += 1;
        fd /= 10;

        if fd == 0 {
            break;
        }
    }

    digits[..len].reverse();
    digits[len] = 0;

    unsafe { CStr::from_bytes_with_nul_unchecked(&buf[..PROC_SELF_FD.len() + len + 1]) }
}
