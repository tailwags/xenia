use bitflags::bitflags;
use linux_raw_sys::general::__kernel_mode_t;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Mode: __kernel_mode_t {

        const _ = !0;
    }
}
