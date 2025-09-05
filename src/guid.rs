//! User and Group ID types.

use core::ffi::c_uint;

/// A group identifier as a raw integer.
pub type RawGid = c_uint;
/// A user identifier as a raw integer.
pub type RawUid = c_uint;

/// `uid_t`—A Unix user ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Uid(RawUid);

/// `gid_t`—A Unix group ID.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Gid(RawGid);

impl Uid {
    /// A `Uid` corresponding to the root user (uid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawUid` into a `Uid`.
    ///
    /// `raw` must be the value of a valid Unix user ID, and not `-1`.
    #[inline]
    pub const unsafe fn from_raw(raw: RawUid) -> Self {
        Self(raw)
    }

    /// Converts a `Uid` into a `RawUid`.
    #[inline]
    pub const fn as_raw(self) -> RawUid {
        self.0
    }

    /// Test whether this uid represents the root user ([`Uid::ROOT`]).
    #[inline]
    pub const fn is_root(self) -> bool {
        self.0 == Self::ROOT.0
    }
}

impl Gid {
    /// A `Gid` corresponding to the root group (gid 0).
    pub const ROOT: Self = Self(0);

    /// Converts a `RawGid` into a `Gid`.
    ///
    /// `raw` must be the value of a valid Unix group ID, and not `-1`.
    #[inline]
    pub const unsafe fn from_raw(raw: RawGid) -> Self {
        Self(raw)
    }

    /// Converts a `Gid` into a `RawGid`.
    #[inline]
    pub const fn as_raw(self) -> RawGid {
        self.0
    }

    /// Test whether this gid represents the root group ([`Gid::ROOT`]).
    #[inline]
    pub const fn is_root(self) -> bool {
        self.0 == Self::ROOT.0
    }
}
