#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod args;
#[cfg(feature = "std")]
pub mod passwd;
#[cfg(feature = "std")]
pub mod stdio;
pub mod termios;
