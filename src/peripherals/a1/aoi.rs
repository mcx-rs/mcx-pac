#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AOI {
    ptr: *mut u8,
}
unsafe impl Send for AOI {}
unsafe impl Sync for AOI {}
impl AOI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn BFCRT(self, n: usize) -> BFCRT {
        assert!(n < 4usize);
        unsafe { BFCRT::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BFCRT {
    ptr: *mut u8,
}
unsafe impl Send for BFCRT {}
unsafe impl Sync for BFCRT {}
impl BFCRT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn BFCRT01(self) -> crate::common::Reg<regs::BFCRT_BFCRT01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn BFCRT23(self) -> crate::common::Reg<regs::BFCRT_BFCRT23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
}
pub mod regs {
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0..Boolean Function Term 0 and 1 Configuration for EVENT3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BFCRT_BFCRT01(pub u16);
    impl BFCRT_BFCRT01 {
        #[must_use]
        #[inline(always)]
        pub const fn PT1_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT1_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT1_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT1_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT1_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT1_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT1_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT1_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT0_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT0_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT0_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT0_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT0_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT0_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT0_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT0_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for BFCRT_BFCRT01 {
        #[inline(always)]
        fn default() -> BFCRT_BFCRT01 {
            BFCRT_BFCRT01(0)
        }
    }
    impl core::fmt::Debug for BFCRT_BFCRT01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BFCRT_BFCRT01")
                .field("PT1_DC", &self.PT1_DC())
                .field("PT1_CC", &self.PT1_CC())
                .field("PT1_BC", &self.PT1_BC())
                .field("PT1_AC", &self.PT1_AC())
                .field("PT0_DC", &self.PT0_DC())
                .field("PT0_CC", &self.PT0_CC())
                .field("PT0_BC", &self.PT0_BC())
                .field("PT0_AC", &self.PT0_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BFCRT_BFCRT01 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BFCRT_BFCRT01 {{ PT1_DC: {=u8:?}, PT1_CC: {=u8:?}, PT1_BC: {=u8:?}, PT1_AC: {=u8:?}, PT0_DC: {=u8:?}, PT0_CC: {=u8:?}, PT0_BC: {=u8:?}, PT0_AC: {=u8:?} }}" , self . PT1_DC () , self . PT1_CC () , self . PT1_BC () , self . PT1_AC () , self . PT0_DC () , self . PT0_CC () , self . PT0_BC () , self . PT0_AC ())
        }
    }
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0..Boolean Function Term 2 and 3 Configuration for EVENT3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BFCRT_BFCRT23(pub u16);
    impl BFCRT_BFCRT23 {
        #[must_use]
        #[inline(always)]
        pub const fn PT3_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT3_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT3_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT3_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT3_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT3_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT3_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT3_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT2_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT2_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT2_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT2_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT2_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT2_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT2_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PT2_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for BFCRT_BFCRT23 {
        #[inline(always)]
        fn default() -> BFCRT_BFCRT23 {
            BFCRT_BFCRT23(0)
        }
    }
    impl core::fmt::Debug for BFCRT_BFCRT23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BFCRT_BFCRT23")
                .field("PT3_DC", &self.PT3_DC())
                .field("PT3_CC", &self.PT3_CC())
                .field("PT3_BC", &self.PT3_BC())
                .field("PT3_AC", &self.PT3_AC())
                .field("PT2_DC", &self.PT2_DC())
                .field("PT2_CC", &self.PT2_CC())
                .field("PT2_BC", &self.PT2_BC())
                .field("PT2_AC", &self.PT2_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BFCRT_BFCRT23 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BFCRT_BFCRT23 {{ PT3_DC: {=u8:?}, PT3_CC: {=u8:?}, PT3_BC: {=u8:?}, PT3_AC: {=u8:?}, PT2_DC: {=u8:?}, PT2_CC: {=u8:?}, PT2_BC: {=u8:?}, PT2_AC: {=u8:?} }}" , self . PT3_DC () , self . PT3_CC () , self . PT3_BC () , self . PT3_AC () , self . PT2_DC () , self . PT2_CC () , self . PT2_BC () , self . PT2_AC ())
        }
    }
}
