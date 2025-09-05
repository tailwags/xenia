#![cfg_attr(not(feature = "std"), no_std)]
#![allow(internal_features)]
#![feature(temporary_niche_types)]
#![feature(rustc_attrs)]

mod errno;
mod nr;
mod syscalls;

#[cfg(not(feature = "std"))]
pub mod fd;
#[cfg(feature = "std")]
pub use std::os::fd;

pub mod stdio;

pub use errno::*;
pub use nr::Syscall;
pub use syscalls::*;
