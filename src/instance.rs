use core::ops::Deref;

#[repr(transparent)]
pub struct Instance<T, const N: u8> {
    ptr: core::ptr::NonNull<T>,
}

impl<T, const N: u8> Deref for Instance<T, N> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, const N: u8> Instance<T, N> {
    #[inline]
    pub const unsafe fn new(ptr: *const T) -> Self {
        Self {
            ptr: core::ptr::NonNull::new_unchecked(ptr as *mut _),
        }
    }
}

unsafe impl<T, const N: u8> Send for Instance<T, N> {}

pub const SOLE_INSTANCE: u8 = 255u8;
