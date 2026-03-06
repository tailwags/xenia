use core::{
    ffi::{CStr, c_char, c_int},
    ptr,
};

pub struct Args {
    next: *const *const c_char,
    end: *const *const c_char,
}

unsafe impl Send for Args {}
unsafe impl Sync for Args {}

impl Args {
    pub unsafe fn new(argc: c_int, argv: *const *const c_char) -> Self {
        if argc <= 0 || argv.is_null() {
            return Self::empty();
        }
        let max_end = unsafe { argv.add(argc as usize) };
        let mut end = argv;
        while end < max_end && !unsafe { *end }.is_null() {
            end = unsafe { end.add(1) };
        }
        Self { next: argv, end }
    }

    pub const fn empty() -> Self {
        let dangling = ptr::NonNull::dangling().as_ptr();
        Self {
            next: dangling,
            end: dangling,
        }
    }
}

impl Iterator for Args {
    type Item = &'static CStr;

    fn next(&mut self) -> Option<Self::Item> {
        if ptr::eq(self.next, self.end) {
            return None;
        }
        let arg_ptr = unsafe { *self.next };
        self.next = unsafe { self.next.add(1) };
        Some(unsafe { CStr::from_ptr(arg_ptr) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.next) as usize }
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<Self::Item> {
        if ptr::eq(self.next, self.end) {
            return None;
        }
        self.end = unsafe { self.end.sub(1) };
        Some(unsafe { CStr::from_ptr(*self.end) })
    }
}

impl core::iter::FusedIterator for Args {}
