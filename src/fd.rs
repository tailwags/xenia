//! Owned and borrowed Linux file descriptors.

#![deny(unsafe_op_in_unsafe_fn)]

use core::{ffi::c_uint, fmt, marker::PhantomData};

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that owns the file
/// descriptor. For the duration of that lifetime, it is guaranteed that nobody will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as an argument, it is not captured or consumed, and it never has the
/// value `-1`.
///
/// This type's `.to_owned()` implementation returns another `BorrowedFd`
/// rather than an `OwnedFd`. It just makes a trivial copy of the raw file
/// descriptor, which is then borrowed under the same lifetime.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedFd<'fd> {
    fd: c_uint,
    _phantom: PhantomData<&'fd OwnedFd>,
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop. It is guaranteed that nobody else will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as a consumed argument or returned as an owned value, and it never
/// has the value `-1`.
#[repr(transparent)]
pub struct OwnedFd {
    fd: c_uint,
}

impl BorrowedFd<'_> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    pub const unsafe fn new(fd: c_uint) -> Self {
        Self {
            fd,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub const fn as_raw(&self) -> c_uint {
        self.fd
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe { crate::close(self) }
    }
}

impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedFd").field("fd", &self.fd).finish()
    }
}

impl fmt::Debug for OwnedFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFd").field("fd", &self.fd).finish()
    }
}

/// A trait to borrow the file descriptor from an underlying object.
pub trait AsFd {
    /// Borrows the file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// # use std::os::fd::{AsFd, BorrowedFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_fd(&self) -> BorrowedFd<'_>;
}

impl<T: AsFd + ?Sized> AsFd for &T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

impl<T: AsFd + ?Sized> AsFd for &mut T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        *self
    }
}

impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety: `OwnedFd` and `BorrowedFd` have the same validity
        // invariants, and the `BorrowedFd` is bounded by the lifetime
        // of `&self`.
        unsafe { BorrowedFd::new(self.fd) }
    }
}

// /// This impl allows implementing traits that require `AsFd` on Arc.
// /// ```
// /// # #[cfg(any(unix, target_os = "wasi"))] mod group_cfg {
// /// # #[cfg(target_os = "wasi")]
// /// # use std::os::wasi::io::AsFd;
// /// # #[cfg(unix)]
// /// # use std::os::unix::io::AsFd;
// /// use std::net::UdpSocket;
// /// use std::sync::Arc;
// ///
// /// trait MyTrait: AsFd {}
// /// impl MyTrait for Arc<UdpSocket> {}
// /// impl MyTrait for Box<UdpSocket> {}
// /// # }
// /// ```
// impl<T: AsFd + ?Sized> AsFd for Arc<T> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         (**self).as_fd()
//     }
// }

// impl<T: AsFd + ?Sized> AsFd for Rc<T> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         (**self).as_fd()
//     }
// }

// impl<T: AsFd + ?Sized> AsFd for Box<T> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         (**self).as_fd()
//     }
// }
