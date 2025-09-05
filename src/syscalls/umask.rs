use crate::{Mode, Syscall, syscall1_readonly};

#[inline]
pub fn umask(mode: Mode) -> Mode {
    unsafe { Mode::from_bits_retain(syscall1_readonly(Syscall::UMASK, mode.bits()) as u32) }
}
