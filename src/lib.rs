#![no_std]
#![allow(internal_features)]
#![feature(temporary_niche_types)]
#![feature(rustc_attrs)]

mod errno;
mod nr;
mod syscalls;

pub mod fd;
pub mod stdio;

pub use errno::*;
pub use nr::Syscall;
pub use syscalls::*;
