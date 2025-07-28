#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMCR {
    ptr: *mut u8,
}
unsafe impl Send for PMCR {}
unsafe impl Sync for PMCR {}
impl PMCR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn PMCR(self) -> crate::common::Reg<regs::PMCR_PMCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PMECTR(self, n: usize) -> PMECTR {
        assert!(n < 3usize);
        unsafe { PMECTR::from_ptr(self.ptr.add(0x18usize + n * 8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMECTR {
    ptr: *mut u8,
}
unsafe impl Send for PMECTR {}
unsafe impl Sync for PMECTR {}
impl PMECTR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn HI(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn LO(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSPM {
    ptr: *mut u8,
}
unsafe impl Send for SYSPM {}
unsafe impl Sync for SYSPM {}
impl SYSPM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn PMCR(self, n: usize) -> PMCR {
        assert!(n < 1usize);
        unsafe { PMCR::from_ptr(self.ptr.add(0x0usize + n * 48usize) as _) }
    }
}
pub mod regs {
    #[doc = "Performance Monitor Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMCR_PMCR(pub u32);
    impl PMCR_PMCR {
        #[must_use]
        #[inline(always)]
        pub const fn MENB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MENB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SSC(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMODE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RECTR1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RECTR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RECTR2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RECTR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RECTR3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RECTR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SELEVT1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SELEVT1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 11usize)) | (((val as u32) & 0x7f) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SELEVT2(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SELEVT2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SELEVT3(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SELEVT3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for PMCR_PMCR {
        #[inline(always)]
        fn default() -> PMCR_PMCR {
            PMCR_PMCR(0)
        }
    }
    impl core::fmt::Debug for PMCR_PMCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMCR_PMCR")
                .field("MENB", &self.MENB())
                .field("SSC", &self.SSC())
                .field("CMODE", &self.CMODE())
                .field("RECTR1", &self.RECTR1())
                .field("RECTR2", &self.RECTR2())
                .field("RECTR3", &self.RECTR3())
                .field("SELEVT1", &self.SELEVT1())
                .field("SELEVT2", &self.SELEVT2())
                .field("SELEVT3", &self.SELEVT3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMCR_PMCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PMCR_PMCR {{ MENB: {=bool:?}, SSC: {=u8:?}, CMODE: {=u8:?}, RECTR1: {=bool:?}, RECTR2: {=bool:?}, RECTR3: {=bool:?}, SELEVT1: {=u8:?}, SELEVT2: {=u8:?}, SELEVT3: {=u8:?} }}" , self . MENB () , self . SSC () , self . CMODE () , self . RECTR1 () , self . RECTR2 () , self . RECTR3 () , self . SELEVT1 () , self . SELEVT2 () , self . SELEVT3 ())
        }
    }
}
