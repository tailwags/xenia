#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), allow(internal_features))]
#![cfg_attr(not(feature = "std"), feature(temporary_niche_types))]
#![cfg_attr(not(feature = "std"), feature(rustc_attrs))]
#![allow(clippy::missing_safety_doc)] // FIXME: remove in the future

#[cfg(feature = "alloc")]
extern crate alloc;

mod as_cstr;
mod errno;
mod guid;
mod syscalls;
mod types;

#[cfg(not(feature = "std"))]
pub mod fd;
#[cfg(feature = "std")]
pub use std::os::fd;

pub mod stdio;

pub use as_cstr::*;
pub use errno::*;
pub use guid::*;
pub use syscalls::*;
pub use types::*;

#[doc(hidden)]
pub const fn __assert_structs<A, B>() {
    assert!(core::mem::size_of::<A>() == core::mem::size_of::<B>());
    assert!(core::mem::align_of::<A>() == core::mem::align_of::<B>());
}
