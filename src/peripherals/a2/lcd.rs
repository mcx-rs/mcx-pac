#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LCD {
    ptr: *mut u8,
}
unsafe impl Send for LCD {}
unsafe impl Sync for LCD {}
impl LCD {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GCR(self) -> crate::common::Reg<regs::GCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn AR(self) -> crate::common::Reg<regs::AR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn FDCR(self) -> crate::common::Reg<regs::FDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn FDSR(self) -> crate::common::Reg<regs::FDSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn PEN(self, n: usize) -> crate::common::Reg<regs::PEN, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn BPEN(self, n: usize) -> crate::common::Reg<regs::BPEN, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn WF(self, n: usize) -> crate::common::Reg<regs::WF, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn WF8B(self, n: usize) -> crate::common::Reg<u8, crate::common::RW> {
        assert!(n < 48usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 1usize) as _) }
    }
}
pub mod regs {
    #[doc = "LCD Auxiliary Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AR(pub u32);
    impl AR {
        #[must_use]
        #[inline(always)]
        pub const fn BRATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BRATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BMODE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BLANK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BLANK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BLINK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BLINK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDIF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for AR {
        #[inline(always)]
        fn default() -> AR {
            AR(0)
        }
    }
    impl core::fmt::Debug for AR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AR")
                .field("BRATE", &self.BRATE())
                .field("BMODE", &self.BMODE())
                .field("BLANK", &self.BLANK())
                .field("ALT", &self.ALT())
                .field("BLINK", &self.BLINK())
                .field("LCDIF", &self.LCDIF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AR {{ BRATE: {=u8:?}, BMODE: {=bool:?}, BLANK: {=bool:?}, ALT: {=bool:?}, BLINK: {=bool:?}, LCDIF: {=bool:?} }}" , self . BRATE () , self . BMODE () , self . BLANK () , self . ALT () , self . BLINK () , self . LCDIF ())
        }
    }
    #[doc = "LCD Back Plane Enable register 0..LCD Back Plane Enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BPEN(pub u32);
    impl BPEN {
        #[must_use]
        #[inline(always)]
        pub const fn PIN_0_BPEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_0_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_32_BPEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_32_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_1_BPEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_1_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_33_BPEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_33_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_2_BPEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_2_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_34_BPEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_34_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_35_BPEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_35_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_3_BPEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_3_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_36_BPEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_36_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_4_BPEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_4_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_37_BPEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_37_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_5_BPEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_5_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_38_BPEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_38_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_6_BPEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_6_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_39_BPEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_39_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_7_BPEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_7_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_40_BPEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_40_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_8_BPEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_8_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_41_BPEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_41_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_9_BPEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_9_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_10_BPEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_10_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_42_BPEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_42_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_11_BPEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_11_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_43_BPEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_43_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_12_BPEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_12_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_44_BPEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_44_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_13_BPEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_13_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_45_BPEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_45_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_14_BPEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_14_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_46_BPEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_46_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_15_BPEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_15_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_47_BPEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_47_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_16_BPEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_16_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_17_BPEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_17_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_18_BPEN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_18_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_19_BPEN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_19_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_20_BPEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_20_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_21_BPEN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_21_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_22_BPEN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_22_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_23_BPEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_23_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_24_BPEN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_24_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_25_BPEN(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_25_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_26_BPEN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_26_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_27_BPEN(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_27_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_28_BPEN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_28_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_29_BPEN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_29_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_30_BPEN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_30_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_31_BPEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_31_BPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for BPEN {
        #[inline(always)]
        fn default() -> BPEN {
            BPEN(0)
        }
    }
    impl core::fmt::Debug for BPEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BPEN")
                .field("PIN_0_BPEN", &self.PIN_0_BPEN())
                .field("PIN_32_BPEN", &self.PIN_32_BPEN())
                .field("PIN_1_BPEN", &self.PIN_1_BPEN())
                .field("PIN_33_BPEN", &self.PIN_33_BPEN())
                .field("PIN_2_BPEN", &self.PIN_2_BPEN())
                .field("PIN_34_BPEN", &self.PIN_34_BPEN())
                .field("PIN_35_BPEN", &self.PIN_35_BPEN())
                .field("PIN_3_BPEN", &self.PIN_3_BPEN())
                .field("PIN_36_BPEN", &self.PIN_36_BPEN())
                .field("PIN_4_BPEN", &self.PIN_4_BPEN())
                .field("PIN_37_BPEN", &self.PIN_37_BPEN())
                .field("PIN_5_BPEN", &self.PIN_5_BPEN())
                .field("PIN_38_BPEN", &self.PIN_38_BPEN())
                .field("PIN_6_BPEN", &self.PIN_6_BPEN())
                .field("PIN_39_BPEN", &self.PIN_39_BPEN())
                .field("PIN_7_BPEN", &self.PIN_7_BPEN())
                .field("PIN_40_BPEN", &self.PIN_40_BPEN())
                .field("PIN_8_BPEN", &self.PIN_8_BPEN())
                .field("PIN_41_BPEN", &self.PIN_41_BPEN())
                .field("PIN_9_BPEN", &self.PIN_9_BPEN())
                .field("PIN_10_BPEN", &self.PIN_10_BPEN())
                .field("PIN_42_BPEN", &self.PIN_42_BPEN())
                .field("PIN_11_BPEN", &self.PIN_11_BPEN())
                .field("PIN_43_BPEN", &self.PIN_43_BPEN())
                .field("PIN_12_BPEN", &self.PIN_12_BPEN())
                .field("PIN_44_BPEN", &self.PIN_44_BPEN())
                .field("PIN_13_BPEN", &self.PIN_13_BPEN())
                .field("PIN_45_BPEN", &self.PIN_45_BPEN())
                .field("PIN_14_BPEN", &self.PIN_14_BPEN())
                .field("PIN_46_BPEN", &self.PIN_46_BPEN())
                .field("PIN_15_BPEN", &self.PIN_15_BPEN())
                .field("PIN_47_BPEN", &self.PIN_47_BPEN())
                .field("PIN_16_BPEN", &self.PIN_16_BPEN())
                .field("PIN_17_BPEN", &self.PIN_17_BPEN())
                .field("PIN_18_BPEN", &self.PIN_18_BPEN())
                .field("PIN_19_BPEN", &self.PIN_19_BPEN())
                .field("PIN_20_BPEN", &self.PIN_20_BPEN())
                .field("PIN_21_BPEN", &self.PIN_21_BPEN())
                .field("PIN_22_BPEN", &self.PIN_22_BPEN())
                .field("PIN_23_BPEN", &self.PIN_23_BPEN())
                .field("PIN_24_BPEN", &self.PIN_24_BPEN())
                .field("PIN_25_BPEN", &self.PIN_25_BPEN())
                .field("PIN_26_BPEN", &self.PIN_26_BPEN())
                .field("PIN_27_BPEN", &self.PIN_27_BPEN())
                .field("PIN_28_BPEN", &self.PIN_28_BPEN())
                .field("PIN_29_BPEN", &self.PIN_29_BPEN())
                .field("PIN_30_BPEN", &self.PIN_30_BPEN())
                .field("PIN_31_BPEN", &self.PIN_31_BPEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BPEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BPEN {{ PIN_0_BPEN: {=bool:?}, PIN_32_BPEN: {=bool:?}, PIN_1_BPEN: {=bool:?}, PIN_33_BPEN: {=bool:?}, PIN_2_BPEN: {=bool:?}, PIN_34_BPEN: {=bool:?}, PIN_35_BPEN: {=bool:?}, PIN_3_BPEN: {=bool:?}, PIN_36_BPEN: {=bool:?}, PIN_4_BPEN: {=bool:?}, PIN_37_BPEN: {=bool:?}, PIN_5_BPEN: {=bool:?}, PIN_38_BPEN: {=bool:?}, PIN_6_BPEN: {=bool:?}, PIN_39_BPEN: {=bool:?}, PIN_7_BPEN: {=bool:?}, PIN_40_BPEN: {=bool:?}, PIN_8_BPEN: {=bool:?}, PIN_41_BPEN: {=bool:?}, PIN_9_BPEN: {=bool:?}, PIN_10_BPEN: {=bool:?}, PIN_42_BPEN: {=bool:?}, PIN_11_BPEN: {=bool:?}, PIN_43_BPEN: {=bool:?}, PIN_12_BPEN: {=bool:?}, PIN_44_BPEN: {=bool:?}, PIN_13_BPEN: {=bool:?}, PIN_45_BPEN: {=bool:?}, PIN_14_BPEN: {=bool:?}, PIN_46_BPEN: {=bool:?}, PIN_15_BPEN: {=bool:?}, PIN_47_BPEN: {=bool:?}, PIN_16_BPEN: {=bool:?}, PIN_17_BPEN: {=bool:?}, PIN_18_BPEN: {=bool:?}, PIN_19_BPEN: {=bool:?}, PIN_20_BPEN: {=bool:?}, PIN_21_BPEN: {=bool:?}, PIN_22_BPEN: {=bool:?}, PIN_23_BPEN: {=bool:?}, PIN_24_BPEN: {=bool:?}, PIN_25_BPEN: {=bool:?}, PIN_26_BPEN: {=bool:?}, PIN_27_BPEN: {=bool:?}, PIN_28_BPEN: {=bool:?}, PIN_29_BPEN: {=bool:?}, PIN_30_BPEN: {=bool:?}, PIN_31_BPEN: {=bool:?} }}" , self . PIN_0_BPEN () , self . PIN_32_BPEN () , self . PIN_1_BPEN () , self . PIN_33_BPEN () , self . PIN_2_BPEN () , self . PIN_34_BPEN () , self . PIN_35_BPEN () , self . PIN_3_BPEN () , self . PIN_36_BPEN () , self . PIN_4_BPEN () , self . PIN_37_BPEN () , self . PIN_5_BPEN () , self . PIN_38_BPEN () , self . PIN_6_BPEN () , self . PIN_39_BPEN () , self . PIN_7_BPEN () , self . PIN_40_BPEN () , self . PIN_8_BPEN () , self . PIN_41_BPEN () , self . PIN_9_BPEN () , self . PIN_10_BPEN () , self . PIN_42_BPEN () , self . PIN_11_BPEN () , self . PIN_43_BPEN () , self . PIN_12_BPEN () , self . PIN_44_BPEN () , self . PIN_13_BPEN () , self . PIN_45_BPEN () , self . PIN_14_BPEN () , self . PIN_46_BPEN () , self . PIN_15_BPEN () , self . PIN_47_BPEN () , self . PIN_16_BPEN () , self . PIN_17_BPEN () , self . PIN_18_BPEN () , self . PIN_19_BPEN () , self . PIN_20_BPEN () , self . PIN_21_BPEN () , self . PIN_22_BPEN () , self . PIN_23_BPEN () , self . PIN_24_BPEN () , self . PIN_25_BPEN () , self . PIN_26_BPEN () , self . PIN_27_BPEN () , self . PIN_28_BPEN () , self . PIN_29_BPEN () , self . PIN_30_BPEN () , self . PIN_31_BPEN ())
        }
    }
    #[doc = "LCD Fault Detect Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDCR(pub u32);
    impl FDCR {
        #[must_use]
        #[inline(always)]
        pub const fn FDPINID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FDPINID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDBPEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDBPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDSWW(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FDSWW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDPRS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FDPRS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for FDCR {
        #[inline(always)]
        fn default() -> FDCR {
            FDCR(0)
        }
    }
    impl core::fmt::Debug for FDCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDCR")
                .field("FDPINID", &self.FDPINID())
                .field("FDBPEN", &self.FDBPEN())
                .field("FDEN", &self.FDEN())
                .field("FDSWW", &self.FDSWW())
                .field("FDPRS", &self.FDPRS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FDCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FDCR {{ FDPINID: {=u8:?}, FDBPEN: {=bool:?}, FDEN: {=bool:?}, FDSWW: {=u8:?}, FDPRS: {=u8:?} }}" , self . FDPINID () , self . FDBPEN () , self . FDEN () , self . FDSWW () , self . FDPRS ())
        }
    }
    #[doc = "LCD Fault Detect Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDSR(pub u32);
    impl FDSR {
        #[must_use]
        #[inline(always)]
        pub const fn FDCNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FDCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDCF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for FDSR {
        #[inline(always)]
        fn default() -> FDSR {
            FDSR(0)
        }
    }
    impl core::fmt::Debug for FDSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDSR")
                .field("FDCNT", &self.FDCNT())
                .field("FDCF", &self.FDCF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FDSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FDSR {{ FDCNT: {=u8:?}, FDCF: {=bool:?} }}",
                self.FDCNT(),
                self.FDCF()
            )
        }
    }
    #[doc = "LCD General Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCR(pub u32);
    impl GCR {
        #[must_use]
        #[inline(always)]
        pub const fn DUTY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DUTY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCLK(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LCLK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDLP(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDLP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDSTP(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDSTP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDDOZE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDDOZE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDCIEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDCIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCDIEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCDIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHCYCLE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHCYCLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLL1TRIM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VLL1TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLL2TRIM(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VLL2TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GCR {
        #[inline(always)]
        fn default() -> GCR {
            GCR(0)
        }
    }
    impl core::fmt::Debug for GCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GCR")
                .field("DUTY", &self.DUTY())
                .field("LCLK", &self.LCLK())
                .field("LCDLP", &self.LCDLP())
                .field("LCDEN", &self.LCDEN())
                .field("LCDSTP", &self.LCDSTP())
                .field("LCDDOZE", &self.LCDDOZE())
                .field("FDCIEN", &self.FDCIEN())
                .field("LCDIEN", &self.LCDIEN())
                .field("SHCYCLE", &self.SHCYCLE())
                .field("SHEN", &self.SHEN())
                .field("VLL1TRIM", &self.VLL1TRIM())
                .field("VLL2TRIM", &self.VLL2TRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GCR {{ DUTY: {=u8:?}, LCLK: {=u8:?}, LCDLP: {=bool:?}, LCDEN: {=bool:?}, LCDSTP: {=bool:?}, LCDDOZE: {=bool:?}, FDCIEN: {=bool:?}, LCDIEN: {=bool:?}, SHCYCLE: {=bool:?}, SHEN: {=bool:?}, VLL1TRIM: {=u8:?}, VLL2TRIM: {=u8:?} }}" , self . DUTY () , self . LCLK () , self . LCDLP () , self . LCDEN () , self . LCDSTP () , self . LCDDOZE () , self . FDCIEN () , self . LCDIEN () , self . SHCYCLE () , self . SHEN () , self . VLL1TRIM () , self . VLL2TRIM ())
        }
    }
    #[doc = "LCD Pin Enable register 0..LCD Pin Enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PEN(pub u32);
    impl PEN {
        #[must_use]
        #[inline(always)]
        pub const fn PIN_0_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_0_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_32_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_32_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_33_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_33_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_2_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_34_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_34_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_35_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_35_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_3_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_3_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_36_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_36_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_4_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_4_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_37_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_37_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_5_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_5_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_38_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_38_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_6_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_6_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_39_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_39_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_7_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_7_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_40_EN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_40_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_8_EN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_8_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_41_EN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_41_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_9_EN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_9_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_10_EN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_10_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_42_EN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_42_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_11_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_11_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_43_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_43_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_12_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_12_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_44_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_44_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_13_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_13_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_45_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_45_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_14_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_14_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_46_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_46_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_15_EN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_15_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_47_EN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_47_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_16_EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_16_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_17_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_17_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_18_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_18_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_19_EN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_19_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_20_EN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_20_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_21_EN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_21_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_22_EN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_22_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_23_EN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_23_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_24_EN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_24_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_25_EN(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_25_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_26_EN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_26_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_27_EN(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_27_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_28_EN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_28_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_29_EN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_29_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_30_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_30_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN_31_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN_31_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PEN {
        #[inline(always)]
        fn default() -> PEN {
            PEN(0)
        }
    }
    impl core::fmt::Debug for PEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PEN")
                .field("PIN_0_EN", &self.PIN_0_EN())
                .field("PIN_32_EN", &self.PIN_32_EN())
                .field("PIN_1_EN", &self.PIN_1_EN())
                .field("PIN_33_EN", &self.PIN_33_EN())
                .field("PIN_2_EN", &self.PIN_2_EN())
                .field("PIN_34_EN", &self.PIN_34_EN())
                .field("PIN_35_EN", &self.PIN_35_EN())
                .field("PIN_3_EN", &self.PIN_3_EN())
                .field("PIN_36_EN", &self.PIN_36_EN())
                .field("PIN_4_EN", &self.PIN_4_EN())
                .field("PIN_37_EN", &self.PIN_37_EN())
                .field("PIN_5_EN", &self.PIN_5_EN())
                .field("PIN_38_EN", &self.PIN_38_EN())
                .field("PIN_6_EN", &self.PIN_6_EN())
                .field("PIN_39_EN", &self.PIN_39_EN())
                .field("PIN_7_EN", &self.PIN_7_EN())
                .field("PIN_40_EN", &self.PIN_40_EN())
                .field("PIN_8_EN", &self.PIN_8_EN())
                .field("PIN_41_EN", &self.PIN_41_EN())
                .field("PIN_9_EN", &self.PIN_9_EN())
                .field("PIN_10_EN", &self.PIN_10_EN())
                .field("PIN_42_EN", &self.PIN_42_EN())
                .field("PIN_11_EN", &self.PIN_11_EN())
                .field("PIN_43_EN", &self.PIN_43_EN())
                .field("PIN_12_EN", &self.PIN_12_EN())
                .field("PIN_44_EN", &self.PIN_44_EN())
                .field("PIN_13_EN", &self.PIN_13_EN())
                .field("PIN_45_EN", &self.PIN_45_EN())
                .field("PIN_14_EN", &self.PIN_14_EN())
                .field("PIN_46_EN", &self.PIN_46_EN())
                .field("PIN_15_EN", &self.PIN_15_EN())
                .field("PIN_47_EN", &self.PIN_47_EN())
                .field("PIN_16_EN", &self.PIN_16_EN())
                .field("PIN_17_EN", &self.PIN_17_EN())
                .field("PIN_18_EN", &self.PIN_18_EN())
                .field("PIN_19_EN", &self.PIN_19_EN())
                .field("PIN_20_EN", &self.PIN_20_EN())
                .field("PIN_21_EN", &self.PIN_21_EN())
                .field("PIN_22_EN", &self.PIN_22_EN())
                .field("PIN_23_EN", &self.PIN_23_EN())
                .field("PIN_24_EN", &self.PIN_24_EN())
                .field("PIN_25_EN", &self.PIN_25_EN())
                .field("PIN_26_EN", &self.PIN_26_EN())
                .field("PIN_27_EN", &self.PIN_27_EN())
                .field("PIN_28_EN", &self.PIN_28_EN())
                .field("PIN_29_EN", &self.PIN_29_EN())
                .field("PIN_30_EN", &self.PIN_30_EN())
                .field("PIN_31_EN", &self.PIN_31_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PEN {{ PIN_0_EN: {=bool:?}, PIN_32_EN: {=bool:?}, PIN_1_EN: {=bool:?}, PIN_33_EN: {=bool:?}, PIN_2_EN: {=bool:?}, PIN_34_EN: {=bool:?}, PIN_35_EN: {=bool:?}, PIN_3_EN: {=bool:?}, PIN_36_EN: {=bool:?}, PIN_4_EN: {=bool:?}, PIN_37_EN: {=bool:?}, PIN_5_EN: {=bool:?}, PIN_38_EN: {=bool:?}, PIN_6_EN: {=bool:?}, PIN_39_EN: {=bool:?}, PIN_7_EN: {=bool:?}, PIN_40_EN: {=bool:?}, PIN_8_EN: {=bool:?}, PIN_41_EN: {=bool:?}, PIN_9_EN: {=bool:?}, PIN_10_EN: {=bool:?}, PIN_42_EN: {=bool:?}, PIN_11_EN: {=bool:?}, PIN_43_EN: {=bool:?}, PIN_12_EN: {=bool:?}, PIN_44_EN: {=bool:?}, PIN_13_EN: {=bool:?}, PIN_45_EN: {=bool:?}, PIN_14_EN: {=bool:?}, PIN_46_EN: {=bool:?}, PIN_15_EN: {=bool:?}, PIN_47_EN: {=bool:?}, PIN_16_EN: {=bool:?}, PIN_17_EN: {=bool:?}, PIN_18_EN: {=bool:?}, PIN_19_EN: {=bool:?}, PIN_20_EN: {=bool:?}, PIN_21_EN: {=bool:?}, PIN_22_EN: {=bool:?}, PIN_23_EN: {=bool:?}, PIN_24_EN: {=bool:?}, PIN_25_EN: {=bool:?}, PIN_26_EN: {=bool:?}, PIN_27_EN: {=bool:?}, PIN_28_EN: {=bool:?}, PIN_29_EN: {=bool:?}, PIN_30_EN: {=bool:?}, PIN_31_EN: {=bool:?} }}" , self . PIN_0_EN () , self . PIN_32_EN () , self . PIN_1_EN () , self . PIN_33_EN () , self . PIN_2_EN () , self . PIN_34_EN () , self . PIN_35_EN () , self . PIN_3_EN () , self . PIN_36_EN () , self . PIN_4_EN () , self . PIN_37_EN () , self . PIN_5_EN () , self . PIN_38_EN () , self . PIN_6_EN () , self . PIN_39_EN () , self . PIN_7_EN () , self . PIN_40_EN () , self . PIN_8_EN () , self . PIN_41_EN () , self . PIN_9_EN () , self . PIN_10_EN () , self . PIN_42_EN () , self . PIN_11_EN () , self . PIN_43_EN () , self . PIN_12_EN () , self . PIN_44_EN () , self . PIN_13_EN () , self . PIN_45_EN () , self . PIN_14_EN () , self . PIN_46_EN () , self . PIN_15_EN () , self . PIN_47_EN () , self . PIN_16_EN () , self . PIN_17_EN () , self . PIN_18_EN () , self . PIN_19_EN () , self . PIN_20_EN () , self . PIN_21_EN () , self . PIN_22_EN () , self . PIN_23_EN () , self . PIN_24_EN () , self . PIN_25_EN () , self . PIN_26_EN () , self . PIN_27_EN () , self . PIN_28_EN () , self . PIN_29_EN () , self . PIN_30_EN () , self . PIN_31_EN ())
        }
    }
    #[doc = "LCD Waveform 3 to 0 Register..LCD Waveform 47 to 44 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WF(pub u32);
    impl WF {
        #[must_use]
        #[inline(always)]
        pub const fn WF0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF12(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF12(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF16(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF20(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF20(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF24(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF24(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF28(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF28(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF32(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF32(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF36(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF36(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF40(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF40(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF44(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF44(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF13(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF13(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF17(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF17(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF21(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF21(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF25(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF25(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF29(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF29(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF33(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF33(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF37(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF37(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF41(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF41(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF45(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF45(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF9(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF10(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF10(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF14(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF14(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF18(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF18(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF22(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF22(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF26(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF26(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF30(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF30(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF34(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF34(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF38(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF38(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF42(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF42(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF46(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF46(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF11(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF11(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF15(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF19(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF19(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF23(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF23(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF27(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF27(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF31(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF31(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF35(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF35(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF39(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF39(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF43(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF43(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF47(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF47(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WF7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WF7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for WF {
        #[inline(always)]
        fn default() -> WF {
            WF(0)
        }
    }
    impl core::fmt::Debug for WF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WF")
                .field("WF0", &self.WF0())
                .field("WF12", &self.WF12())
                .field("WF16", &self.WF16())
                .field("WF20", &self.WF20())
                .field("WF24", &self.WF24())
                .field("WF28", &self.WF28())
                .field("WF32", &self.WF32())
                .field("WF36", &self.WF36())
                .field("WF4", &self.WF4())
                .field("WF40", &self.WF40())
                .field("WF44", &self.WF44())
                .field("WF8", &self.WF8())
                .field("WF1", &self.WF1())
                .field("WF13", &self.WF13())
                .field("WF17", &self.WF17())
                .field("WF21", &self.WF21())
                .field("WF25", &self.WF25())
                .field("WF29", &self.WF29())
                .field("WF33", &self.WF33())
                .field("WF37", &self.WF37())
                .field("WF41", &self.WF41())
                .field("WF45", &self.WF45())
                .field("WF5", &self.WF5())
                .field("WF9", &self.WF9())
                .field("WF10", &self.WF10())
                .field("WF14", &self.WF14())
                .field("WF18", &self.WF18())
                .field("WF2", &self.WF2())
                .field("WF22", &self.WF22())
                .field("WF26", &self.WF26())
                .field("WF30", &self.WF30())
                .field("WF34", &self.WF34())
                .field("WF38", &self.WF38())
                .field("WF42", &self.WF42())
                .field("WF46", &self.WF46())
                .field("WF6", &self.WF6())
                .field("WF11", &self.WF11())
                .field("WF15", &self.WF15())
                .field("WF19", &self.WF19())
                .field("WF23", &self.WF23())
                .field("WF27", &self.WF27())
                .field("WF3", &self.WF3())
                .field("WF31", &self.WF31())
                .field("WF35", &self.WF35())
                .field("WF39", &self.WF39())
                .field("WF43", &self.WF43())
                .field("WF47", &self.WF47())
                .field("WF7", &self.WF7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WF {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WF {{ WF0: {=u8:?}, WF12: {=u8:?}, WF16: {=u8:?}, WF20: {=u8:?}, WF24: {=u8:?}, WF28: {=u8:?}, WF32: {=u8:?}, WF36: {=u8:?}, WF4: {=u8:?}, WF40: {=u8:?}, WF44: {=u8:?}, WF8: {=u8:?}, WF1: {=u8:?}, WF13: {=u8:?}, WF17: {=u8:?}, WF21: {=u8:?}, WF25: {=u8:?}, WF29: {=u8:?}, WF33: {=u8:?}, WF37: {=u8:?}, WF41: {=u8:?}, WF45: {=u8:?}, WF5: {=u8:?}, WF9: {=u8:?}, WF10: {=u8:?}, WF14: {=u8:?}, WF18: {=u8:?}, WF2: {=u8:?}, WF22: {=u8:?}, WF26: {=u8:?}, WF30: {=u8:?}, WF34: {=u8:?}, WF38: {=u8:?}, WF42: {=u8:?}, WF46: {=u8:?}, WF6: {=u8:?}, WF11: {=u8:?}, WF15: {=u8:?}, WF19: {=u8:?}, WF23: {=u8:?}, WF27: {=u8:?}, WF3: {=u8:?}, WF31: {=u8:?}, WF35: {=u8:?}, WF39: {=u8:?}, WF43: {=u8:?}, WF47: {=u8:?}, WF7: {=u8:?} }}" , self . WF0 () , self . WF12 () , self . WF16 () , self . WF20 () , self . WF24 () , self . WF28 () , self . WF32 () , self . WF36 () , self . WF4 () , self . WF40 () , self . WF44 () , self . WF8 () , self . WF1 () , self . WF13 () , self . WF17 () , self . WF21 () , self . WF25 () , self . WF29 () , self . WF33 () , self . WF37 () , self . WF41 () , self . WF45 () , self . WF5 () , self . WF9 () , self . WF10 () , self . WF14 () , self . WF18 () , self . WF2 () , self . WF22 () , self . WF26 () , self . WF30 () , self . WF34 () , self . WF38 () , self . WF42 () , self . WF46 () , self . WF6 () , self . WF11 () , self . WF15 () , self . WF19 () , self . WF23 () , self . WF27 () , self . WF3 () , self . WF31 () , self . WF35 () , self . WF39 () , self . WF43 () , self . WF47 () , self . WF7 ())
        }
    }
}
