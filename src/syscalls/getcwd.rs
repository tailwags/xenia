use core::ffi::CStr;
use std::ffi::CString;

use crate::{Errno, Syscall, syscall_result, syscall2};

#[inline]
pub fn getcwd<B: Into<Vec<u8>>>(buffer: B) -> crate::Result<CString> {
    let mut buffer = buffer.into();
    buffer.clear();
    buffer.reserve(256);

    loop {
        let buf = buffer.spare_capacity_mut();

        match syscall_result(unsafe { syscall2(Syscall::GETCWD, buf.as_mut_ptr(), buf.len()) }) {
            Err(Errno::RANGE) => {
                buffer.reserve(buffer.capacity() + 1);
            }
            Ok(_) => unsafe {
                buffer.set_len(
                    CStr::from_ptr(buffer.as_ptr().cast())
                        .to_bytes_with_nul()
                        .len(),
                );

                return Ok(CString::from_vec_with_nul_unchecked(buffer));
            },
            Err(errno) => return Err(errno),
        }
    }
}
