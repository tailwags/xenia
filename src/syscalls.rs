#![allow(unused)] // FIXME: remove when all syscalls types are used

use core::{
    arch::asm,
    ffi::{CStr, c_int},
    mem::MaybeUninit,
    ptr::null,
};

macro_rules! syscall_modules {
    ($($module:ident),* $(,)?) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}

syscall_modules! {
    chdir, chroot, close, execve, exit_group,  geteuid, getpid, mkdir, mount, stat, fstat, uname, write, umask, ioctl,
    readlinkat
}

use crate::{
    Errno, Result,
    fd::{AsRawFd, BorrowedFd},
};

use linux_raw_sys::general::{
    __NR_chdir, __NR_chroot, __NR_close, __NR_execve, __NR_exit_group, __NR_fstat, __NR_geteuid,
    __NR_getpid, __NR_getuid, __NR_ioctl, __NR_mkdirat, __NR_mount, __NR_newfstatat,
    __NR_readlinkat, __NR_umask, __NR_uname, __NR_write,
};

#[repr(transparent)]
pub struct Syscall(usize);

impl Syscall {
    pub const CHDIR: Self = Self::from_raw(__NR_chdir);
    pub const WRITE: Self = Self::from_raw(__NR_write);
    pub const CHROOT: Self = Self::from_raw(__NR_chroot);
    pub const CLOSE: Self = Self::from_raw(__NR_close);
    pub const EXECVE: Self = Self::from_raw(__NR_execve);
    pub const UNAME: Self = Self::from_raw(__NR_uname);
    pub const EXIT_GROUP: Self = Self::from_raw(__NR_exit_group);
    pub const GETPID: Self = Self::from_raw(__NR_getpid);
    pub const MKDIRAT: Self = Self::from_raw(__NR_mkdirat);
    pub const NEWFSTATAT: Self = Self::from_raw(__NR_newfstatat);
    pub const FSTAT: Self = Self::from_raw(__NR_fstat);
    pub const MOUNT: Self = Self::from_raw(__NR_mount);
    pub const GETUID: Self = Self::from_raw(__NR_getuid);
    pub const GETEUID: Self = Self::from_raw(__NR_geteuid);
    pub const UMASK: Self = Self::from_raw(__NR_umask);
    pub const IOCTL: Self = Self::from_raw(__NR_ioctl);
    pub const READLINKAT: Self = Self::from_raw(__NR_readlinkat);

    const fn from_raw(nr: u32) -> Self {
        Self(nr as usize)
    }

    #[inline]
    pub(crate) const fn as_raw(self) -> usize {
        self.0
    }
}

pub(crate) unsafe trait SyscallArg: sealed::Sealed {
    fn as_arg(&self) -> usize;
}

unsafe impl SyscallArg for BorrowedFd<'_> {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_raw_fd() as usize
    }
}

unsafe impl<T> SyscallArg for &mut MaybeUninit<T> {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T> SyscallArg for *mut T {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for &CStr {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T> SyscallArg for *const T {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for u32 {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for i32 {
    #[inline]
    fn as_arg(&self) -> usize {
        *self as usize
    }
}

unsafe impl SyscallArg for usize {
    #[inline]
    fn as_arg(&self) -> usize {
        *self
    }
}

unsafe impl SyscallArg for &[u8] {
    #[inline]
    fn as_arg(&self) -> usize {
        self.as_ptr() as usize
    }
}

unsafe impl<T: SyscallArg> SyscallArg for Option<T> {
    fn as_arg(&self) -> usize {
        match self {
            Some(a) => a.as_arg(),
            None => null::<T>() as usize,
        }
    }
}

mod sealed {
    use crate::fd::BorrowedFd;
    use core::{ffi::CStr, mem::MaybeUninit};

    pub(crate) trait Sealed {}

    impl Sealed for BorrowedFd<'_> {}
    impl<T> Sealed for &mut MaybeUninit<T> {}
    impl Sealed for &CStr {}
    impl<T> Sealed for *const T {}
    impl<T> Sealed for *mut T {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for isize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for usize {}
    impl Sealed for &[u8] {}
    impl Sealed for &mut [u8] {}
    impl<T: Sealed> Sealed for Option<T> {}
}

#[inline]
pub(crate) fn syscall_result(ret: usize) -> Result<usize> {
    if (-4095..0).contains(&(ret as isize)) {
        Err(unsafe { Errno::from_raw(ret as u16) })
    } else {
        Ok(ret)
    }
}

#[inline]
pub(crate) fn syscall_result_unit(ret: usize) -> Result<()> {
    if (-4095..0).contains(&(ret as isize)) {
        Err(unsafe { Errno::from_raw(ret as u16) })
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn syscall_result_c_int(ret: usize) -> Result<c_int> {
    if (-4095..0).contains(&(ret as isize)) {
        Err(unsafe { Errno::from_raw(ret as u16) })
    } else {
        Ok(ret as c_int)
    }
}

#[inline]
pub(crate) unsafe fn syscall0_readonly(syscall: Syscall) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly),
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall1<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall1_readonly<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall1_noreturn<Arg0: SyscallArg>(syscall: Syscall, arg0: Arg0) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall.as_raw(),
            in("rdi") arg0.as_arg(),
            options(nostack, noreturn)
        )
    }
}

#[inline]
pub(crate) unsafe fn syscall2<Arg0: SyscallArg, Arg1: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall2_readonly<Arg0: SyscallArg, Arg1: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall3<Arg0: SyscallArg, Arg1: SyscallArg, Arg2: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall3_readonly<Arg0: SyscallArg, Arg1: SyscallArg, Arg2: SyscallArg>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall4<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall4_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall5<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall5_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall6<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
    Arg5: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            in("r9") arg5.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[inline]
pub(crate) unsafe fn syscall6_readonly<
    Arg0: SyscallArg,
    Arg1: SyscallArg,
    Arg2: SyscallArg,
    Arg3: SyscallArg,
    Arg4: SyscallArg,
    Arg5: SyscallArg,
>(
    syscall: Syscall,
    arg0: Arg0,
    arg1: Arg1,
    arg2: Arg2,
    arg3: Arg3,
    arg4: Arg4,
    arg5: Arg5,
) -> usize {
    unsafe {
        let ret;
        asm!(
            "syscall",
            inlateout("rax") syscall.as_raw() => ret,
            in("rdi") arg0.as_arg(),
            in("rsi") arg1.as_arg(),
            in("rdx") arg2.as_arg(),
            in("r10") arg3.as_arg(),
            in("r8") arg4.as_arg(),
            in("r9") arg5.as_arg(),
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        ret
    }
}
