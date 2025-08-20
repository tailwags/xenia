//! Owned and borrowed Unix-like file descriptors.
//!
//! This module is supported on Unix platforms and WASI, which both use a
//! similar file descriptor system for referencing OS resources.
//!
//! Derive from rust std at revision e8a792daf500b5ff8097896ddb6cc037abe92487

// `RawFd`, `AsRawFd`, etc.
mod raw;

// `OwnedFd`, `AsFd`, etc.
mod owned;

pub use owned::*;
pub use raw::*;
