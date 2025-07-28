#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMC {
    ptr: *mut u8,
}
unsafe impl Send for FMC {}
unsafe impl Sync for FMC {}
impl FMC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn REMAP(self) -> crate::common::Reg<regs::REMAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Data Remap"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REMAP(pub u32);
    impl REMAP {
        #[must_use]
        #[inline(always)]
        pub const fn REMAPLK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REMAPLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LIM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LIMDP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LIMDP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for REMAP {
        #[inline(always)]
        fn default() -> REMAP {
            REMAP(0)
        }
    }
    impl core::fmt::Debug for REMAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REMAP")
                .field("REMAPLK", &self.REMAPLK())
                .field("LIM", &self.LIM())
                .field("LIMDP", &self.LIMDP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REMAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REMAP {{ REMAPLK: {=bool:?}, LIM: {=u8:?}, LIMDP: {=u8:?} }}",
                self.REMAPLK(),
                self.LIM(),
                self.LIMDP()
            )
        }
    }
}
