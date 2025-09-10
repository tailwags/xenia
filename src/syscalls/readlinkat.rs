use core::ffi::CStr;
use std::ffi::CString;

use crate::{AsCStr, Syscall, fd::AsFd, syscall_result, syscall4};

#[inline]
pub fn readlinkat<Fd: AsFd, P: AsCStr, B: Into<Vec<u8>>>(
    fd: Fd,
    path: P,
    buffer: B,
) -> crate::Result<CString> {
    path.try_as_c_str(|path| readlinkat_inner(fd, path, buffer.into()))
}

#[inline]
fn readlinkat_inner<Fd: AsFd>(fd: Fd, path: &CStr, mut buffer: Vec<u8>) -> crate::Result<CString> {
    buffer.clear();
    buffer.reserve(256);

    loop {
        let buf = buffer.spare_capacity_mut();

        let bytes_read = syscall_result(unsafe {
            syscall4(
                Syscall::READLINKAT,
                fd.as_fd(),
                path,
                buf.as_mut_ptr(),
                buf.len(),
            )
        })?;

        if bytes_read < buffer.capacity() {
            unsafe {
                buffer.set_len(bytes_read);
            }

            return Ok(unsafe { CString::from_vec_unchecked(buffer) });
        }

        buffer.reserve(buffer.capacity() + 1);
    }
}
