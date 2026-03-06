use std::{io::Write, os::fd::BorrowedFd};

use xenia::{stdio::stdout, write};

#[repr(transparent)]
pub struct UnbufStdout<'a> {
    raw: BorrowedFd<'a>,
}

impl UnbufStdout<'_> {
    pub fn new() -> Self {
        Self { raw: stdout() }
    }
}

impl Write for UnbufStdout<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        write(self.raw, buf).map_err(|err| std::io::Error::from_raw_os_error(err.raw_os_error()))
    }

    // fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
    //     writev(self.raw, bufs).map_err(std::io::Error::from)
    // }

    // #[inline]
    // fn is_write_vectored(&self) -> bool {
    //     true
    // }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
