use core::{
    ffi::{c_long, c_uint, c_ulong},
    mem::MaybeUninit,
};

use crate::{AsCStr, Result, Stat, Syscall, stdio::cwd, syscall_result_unit, syscall4};

#[inline]
pub fn stat<P: AsCStr>(path: P) -> Result<Stat> {
    let mut stat = MaybeUninit::<Stat>::uninit();

    path.try_as_c_str(|path| unsafe {
        syscall_result_unit(syscall4(
            Syscall::NEWFSTATAT,
            cwd(),
            path,
            &mut stat,
            0usize,
        ))?;
        Ok(stat.assume_init())
    })
}
