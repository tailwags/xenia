//! Helper trait for passing cstrings to syscalls
//!
//! Most of the code in this module has been adapted from https://github.com/bytecodealliance/rustix
//! which is under the APACHE-2.0 license

use core::{ffi::CStr, mem::MaybeUninit, ptr, slice};
#[cfg(feature = "std")]
use std::{
    borrow::Cow,
    ffi::{CString, OsStr, OsString},
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

pub trait AsCStr {
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        Self: Sized,
        F: FnOnce(&CStr) -> crate::Result<T>;
}

impl AsCStr for &CStr {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(self)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &CString {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(self)
    }
}

#[cfg(feature = "std")]
impl AsCStr for CString {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&self)
    }
}

impl AsCStr for &str {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &String {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for String {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&CString::new(self).map_err(|_| crate::Errno::INVAL)?)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &OsStr {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &OsString {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for OsString {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&CString::new(self.into_vec()).map_err(|_| crate::Errno::INVAL)?)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &Path {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_os_str().as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &PathBuf {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_os_str().as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for PathBuf {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&CString::new(self.into_os_string().into_vec()).map_err(|_| crate::Errno::INVAL)?)
    }
}

impl AsCStr for &[u8] {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self, f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for &Vec<u8> {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self, f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for Vec<u8> {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&CString::new(self).map_err(|_| crate::Errno::INVAL)?)
    }
}

#[cfg(feature = "std")]
impl AsCStr for Cow<'_, CStr> {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        f(&self)
    }
}

#[cfg(feature = "std")]
impl AsCStr for Cow<'_, str> {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

#[cfg(feature = "std")]
impl AsCStr for Cow<'_, OsStr> {
    #[inline]
    fn try_as_c_str<T, F>(self, f: F) -> crate::Result<T>
    where
        F: FnOnce(&CStr) -> crate::Result<T>,
    {
        with_c_str(self.as_bytes(), f)
    }
}

/// Runs a closure with `bytes` passed in as a `&CStr`.
#[inline]
fn with_c_str<T, F>(bytes: &[u8], f: F) -> crate::Result<T>
where
    F: FnOnce(&CStr) -> crate::Result<T>,
{
    const SMALL_PATH_BUFFER_SIZE: usize = 256;

    // Most paths are less than `SMALL_PATH_BUFFER_SIZE` long. The rest can go
    // through the dynamic allocation path. If you're opening many files in a
    // directory with a long path, consider opening the directory and using
    // `openat` to open the files under it, which will avoid this, and is often
    // faster in the OS as well.

    // Test with `>=` so that we have room for the trailing NUL.
    if bytes.len() >= SMALL_PATH_BUFFER_SIZE {
        return with_c_str_slow_path(bytes, f);
    }

    // Taken from
    // <https://github.com/rust-lang/rust/blob/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys/common/small_c_string.rs>
    let mut buf = MaybeUninit::<[u8; SMALL_PATH_BUFFER_SIZE]>::uninit();
    let buf_ptr = buf.as_mut_ptr().cast::<u8>();

    // This helps test our safety condition below.
    debug_assert!(bytes.len() + 1 <= SMALL_PATH_BUFFER_SIZE);

    // SAFETY: `bytes.len() < SMALL_PATH_BUFFER_SIZE` which means we have space
    // for `bytes.len() + 1` `u8`s:
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, bytes.len());
        buf_ptr.add(bytes.len()).write(b'\0');
    }

    // SAFETY: We just wrote the bytes above and they will remain valid for the
    // duration of `f` because `buf` doesn't get dropped until the end of the
    // function.
    match CStr::from_bytes_with_nul(unsafe { slice::from_raw_parts(buf_ptr, bytes.len() + 1) }) {
        Ok(s) => f(s),
        Err(_) => Err(crate::Errno::INVAL),
    }
}

#[cold]
fn with_c_str_slow_path<T, F>(bytes: &[u8], f: F) -> crate::Result<T>
where
    F: FnOnce(&CStr) -> crate::Result<T>,
{
    #[cfg(feature = "std")]
    {
        f(&CString::new(bytes).map_err(|_cstr_err| crate::Errno::INVAL)?)
    }

    #[cfg(not(feature = "std"))]
    {
        const LARGE_PATH_BUFFER_SIZE: usize = linux_raw_sys::general::PATH_MAX as usize;

        // Taken from
        // <https://github.com/rust-lang/rust/blob/a00f8ba7fcac1b27341679c51bf5a3271fa82df3/library/std/src/sys/common/small_c_string.rs>
        let mut buf = MaybeUninit::<[u8; LARGE_PATH_BUFFER_SIZE]>::uninit();
        let buf_ptr = buf.as_mut_ptr().cast::<u8>();

        // This helps test our safety condition below.
        if bytes.len() + 1 > LARGE_PATH_BUFFER_SIZE {
            return Err(crate::Errno::NAMETOOLONG);
        }

        // SAFETY: `bytes.len() < LARGE_PATH_BUFFER_SIZE` which means we have
        // space for `bytes.len() + 1` `u8`s:
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), buf_ptr, bytes.len());
            buf_ptr.add(bytes.len()).write(b'\0');
        }

        // SAFETY: We just wrote the bytes above and they will remain valid for
        // the duration of `f` because `buf` doesn't get dropped until the end
        // of the function.
        match CStr::from_bytes_with_nul(unsafe { slice::from_raw_parts(buf_ptr, bytes.len() + 1) })
        {
            Ok(s) => f(s),
            Err(_) => Err(crate::Errno::INVAL),
        }
    }
}
