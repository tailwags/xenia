use core::{ffi::c_int, num::NonZero};

pub type RawPid = c_int;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Pid(NonZero<RawPid>);

impl Pid {
    /// Converts a raw process ID, returning `None` if it is not positive.
    pub const fn from_raw(raw: RawPid) -> Option<Self> {
        if raw > 0 {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        } else {
            None
        }
    }

    /// # Safety
    ///
    /// `raw` must be a positive process ID.
    pub const unsafe fn from_raw_unchecked(raw: RawPid) -> Self {
        Self(unsafe { NonZero::new_unchecked(raw) })
    }

    pub const fn as_raw(&self) -> RawPid {
        self.0.get()
    }
}
