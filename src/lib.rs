#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), allow(internal_features))]
#![cfg_attr(not(feature = "std"), feature(temporary_niche_types))]
#![cfg_attr(not(feature = "std"), feature(rustc_attrs))]

mod errno;
mod guid;
mod nr;
mod syscalls;

#[cfg(not(feature = "std"))]
pub mod fd;
#[cfg(feature = "std")]
pub use std::os::fd;

pub mod stdio;

pub use errno::*;
pub use guid::*;
pub use nr::Syscall;
pub use syscalls::*;
