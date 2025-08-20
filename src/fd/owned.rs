//! Owned and borrowed Unix-like file descriptors.

// FIXME: all commented out stuff
// TODO: Figure out if we can do alloc stuff

use crate::close;

use super::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

use core::{fmt, marker::PhantomData, mem::ManuallyDrop};

type ValidRawFd = core::num::niche_types::NotAllOnes<RawFd>;

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
/// This type does not have a [`ToOwned`][crate::borrow::ToOwned]
/// implementation. Calling `.to_owned()` on a variable of this type will call
/// it on `&BorrowedFd` and use `Clone::clone()` like `ToOwned` does for all
/// types implementing `Clone`. The result will be descriptor borrowed under
/// the same lifetime.
///
/// To obtain an [`OwnedFd`], you can use [`BorrowedFd::try_clone_to_owned`]
/// instead, but this is not supported on all platforms.
#[derive(Copy, Clone)]
#[repr(transparent)]
#[rustc_nonnull_optimization_guaranteed]

pub struct BorrowedFd<'fd> {
    fd: ValidRawFd,
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
///
/// You can use [`AsFd::as_fd`] to obtain a [`BorrowedFd`].
#[repr(transparent)]
#[rustc_nonnull_optimization_guaranteed]

pub struct OwnedFd {
    fd: ValidRawFd,
}

impl BorrowedFd<'_> {
    /// Returns a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    #[track_caller]
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        Self {
            fd: ValidRawFd::new(fd).expect("fd != -1"),
            _phantom: PhantomData,
        }
    }
}

impl OwnedFd {
    // /// Creates a new `OwnedFd` instance that shares the same underlying file
    // /// description as the existing `OwnedFd` instance.

    // pub fn try_clone(&self) -> crate::io::Result<Self> {
    //     self.as_fd().try_clone_to_owned()
    // }
}

impl BorrowedFd<'_> {
    // /// Creates a new `OwnedFd` instance that shares the same underlying file
    // /// description as the existing `BorrowedFd` instance.

    // pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedFd> {
    //     // We want to atomically duplicate this file descriptor and set the
    //     // CLOEXEC flag, and currently that's done via F_DUPFD_CLOEXEC. This
    //     // is a POSIX flag that was added to Linux in 2.6.24.
    //     #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
    //     let cmd = libc::F_DUPFD_CLOEXEC;

    //     // For ESP-IDF, F_DUPFD is used instead, because the CLOEXEC semantics
    //     // will never be supported, as this is a bare metal framework with
    //     // no capabilities for multi-process execution. While F_DUPFD is also
    //     // not supported yet, it might be (currently it returns ENOSYS).
    //     #[cfg(any(target_os = "espidf", target_os = "vita"))]
    //     let cmd = libc::F_DUPFD;

    //     // Avoid using file descriptors below 3 as they are used for stdio
    //     let fd = cvt(unsafe { libc::fcntl(self.as_raw_fd(), cmd, 3) })?;
    //     Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    // }
}

impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_inner()
    }
}

impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_inner()
    }
}

impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        ManuallyDrop::new(self).fd.as_inner()
    }
}

impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// [ownership][io-safety]. The resource must not require any cleanup other than `close`.
    ///
    /// [io-safety]: io#io-safety
    #[inline]
    #[track_caller]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self {
            fd: ValidRawFd::new(fd).expect("fd != -1"),
        }
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // Note that errors are ignored when closing a file descriptor. According to POSIX 2024,
            // we can and indeed should retry `close` on `EINTR`
            // (https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/close.html),
            // but it is not clear yet how well widely-used implementations are conforming with this
            // mandate since older versions of POSIX left the state of the FD after an `EINTR`
            // unspecified. Ignoring errors is "fine" because some of the major Unices (in
            // particular, Linux) do make sure to always close the FD, even when `close()` is
            // interrupted, and the scenario is rare to begin with. If we retried on a
            // not-POSIX-compliant implementation, the consequences could be really bad since we may
            // close the wrong FD. Helpful link to an epic discussion by POSIX workgroup that led to
            // the latest POSIX wording: http://austingroupbugs.net/view.php?id=529

            // crate::sys::fs::debug_assert_fd_is_open(self.fd.as_inner());

            let _ = close(self);
        }
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

// macro_rules! impl_is_terminal {
//     ($($t:ty),*$(,)?) => {$(
//         #[unstable(feature = "sealed", issue = "none")]
//         impl crate::sealed::Sealed for $t {}

//         impl crate::io::IsTerminal for $t {
//             #[inline]
//             fn is_terminal(&self) -> bool {
//                 crate::sys::io::is_terminal(self)
//             }
//         }
//     )*}
// }

// impl_is_terminal!(BorrowedFd<'_>, OwnedFd);

/// A trait to borrow the file descriptor from an underlying object.
///
/// This is only available on unix platforms and must be imported in order to
/// call the method. Windows platforms have a corresponding `AsHandle` and
/// `AsSocket` set of traits.

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
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

// impl AsFd for fs::File {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         self.as_inner().as_fd()
//     }
// }

// impl From<fs::File> for OwnedFd {
//     /// Takes ownership of a [`File`](fs::File)'s underlying file descriptor.
//     #[inline]
//     fn from(file: fs::File) -> OwnedFd {
//         file.into_inner().into_inner().into_inner()
//     }
// }

// impl From<OwnedFd> for fs::File {
//     /// Returns a [`File`](fs::File) that takes ownership of the given
//     /// file descriptor.
//     #[inline]
//     fn from(owned_fd: OwnedFd) -> Self {
//         Self::from_inner(FromInner::from_inner(FromInner::from_inner(owned_fd)))
//     }
// }

// / This impl allows implementing traits that require `AsFd` on Arc.
// / ```
// / # #[cfg(any(unix, target_os = "wasi"))] mod group_cfg {
// / # #[cfg(target_os = "wasi")]
// / # use std::os::wasi::io::AsFd;
// / # #[cfg(unix)]
// / # use std::os::unix::io::AsFd;
// / use std::net::UdpSocket;
// / use std::sync::Arc;
// /
// / trait MyTrait: AsFd {}
// / impl MyTrait for Arc<UdpSocket> {}
// / impl MyTrait for Box<UdpSocket> {}
// / # }
// / ```
// /
// impl<T: AsFd + ?Sized> AsFd for crate::sync::Arc<T> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         (**self).as_fd()
//     }
// }

// impl<T: AsFd + ?Sized> AsFd for crate::rc::Rc<T> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         (**self).as_fd()
//     }
// }

// #[unstable(feature = "unique_rc_arc", issue = "112566")]
// impl<T: AsFd + ?Sized> AsFd for crate::rc::UniqueRc<T> {
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

// impl AsFd for io::Stdin {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         unsafe { BorrowedFd::borrow_raw(0) }
//     }
// }

// impl<'a> AsFd for io::StdinLock<'a> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         // SAFETY: user code should not close stdin out from under the standard library
//         unsafe { BorrowedFd::borrow_raw(0) }
//     }
// }

// impl AsFd for io::Stdout {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         unsafe { BorrowedFd::borrow_raw(1) }
//     }
// }

// impl<'a> AsFd for io::StdoutLock<'a> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         // SAFETY: user code should not close stdout out from under the standard library
//         unsafe { BorrowedFd::borrow_raw(1) }
//     }
// }

// impl AsFd for io::Stderr {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         unsafe { BorrowedFd::borrow_raw(2) }
//     }
// }

// impl<'a> AsFd for io::StderrLock<'a> {
//     #[inline]
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         // SAFETY: user code should not close stderr out from under the standard library
//         unsafe { BorrowedFd::borrow_raw(2) }
//     }
// }
