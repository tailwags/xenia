use xenia::{Errno, stdio::stdout, write};

#[derive(Debug, Clone, Copy, Default)]
pub struct UnbufStdout;

#[cfg(feature = "std")]
impl std::io::Write for UnbufStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        write(stdout(), buf).map_err(|err| std::io::Error::from_raw_os_error(err.raw_os_error()))
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

impl core::fmt::Write for UnbufStdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut buf = s.as_bytes();

        while !buf.is_empty() {
            match write(stdout(), buf) {
                Ok(0) => return Err(core::fmt::Error),
                Ok(n) => buf = &buf[n..],
                Err(Errno::INTR) => continue,
                Err(_) => return Err(core::fmt::Error),
            }
        }

        Ok(())
    }
}
