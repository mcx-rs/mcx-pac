#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GDET {
    ptr: *mut u8,
}
unsafe impl Send for GDET {}
unsafe impl Sync for GDET {}
impl GDET {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GDET_CONF_0(self) -> crate::common::Reg<regs::GDET_CONF_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_1(self) -> crate::common::Reg<regs::GDET_CONF_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_ENABLE1(self) -> crate::common::Reg<regs::GDET_ENABLE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_2(self) -> crate::common::Reg<regs::GDET_CONF_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_3(self) -> crate::common::Reg<regs::GDET_CONF_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_4(self) -> crate::common::Reg<regs::GDET_CONF_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_5(self) -> crate::common::Reg<regs::GDET_CONF_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_RESET(self) -> crate::common::Reg<regs::GDET_RESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_TEST(self) -> crate::common::Reg<regs::GDET_TEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc4usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_DLY_CTRL(self) -> crate::common::Reg<regs::GDET_DLY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fccusize) as _) }
    }
}
pub mod regs {
    #[doc = "GDET Configuration 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_0(pub u32);
    impl GDET_CONF_0 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_3_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_3_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_3_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_3_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for GDET_CONF_0 {
        #[inline(always)]
        fn default() -> GDET_CONF_0 {
            GDET_CONF_0(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_0")
                .field("FIELD_3_0", &self.FIELD_3_0())
                .field("field_3_0", &self.field_3_0())
                .field("SBZ", &self.SBZ())
                .field("sbz", &self.sbz())
                .field("RFU", &self.RFU())
                .field("rfu", &self.rfu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_0 {{ FIELD_3_0: {=u8:?}, field_3_0: {=u8:?}, SBZ: {=bool:?}, sbz: {=bool:?}, RFU: {=u32:?}, rfu: {=u32:?} }}" , self . FIELD_3_0 () , self . field_3_0 () , self . SBZ () , self . sbz () , self . RFU () , self . rfu ())
        }
    }
    #[doc = "GDET Configuration 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_1(pub u32);
    impl GDET_CONF_1 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_1_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_1_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_3_2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_3_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_3_2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_3_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIELD_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_field_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIELD_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_field_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBZ5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
    }
    impl Default for GDET_CONF_1 {
        #[inline(always)]
        fn default() -> GDET_CONF_1 {
            GDET_CONF_1(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_1")
                .field("FIELD_1_0", &self.FIELD_1_0())
                .field("field_1_0", &self.field_1_0())
                .field("FIELD_3_2", &self.FIELD_3_2())
                .field("field_3_2", &self.field_3_2())
                .field("SBZ1", &self.SBZ1())
                .field("sbz1", &self.sbz1())
                .field("SBZ2", &self.SBZ2())
                .field("sbz2", &self.sbz2())
                .field("SBZ3", &self.SBZ3())
                .field("sbz3", &self.sbz3())
                .field("FIELD_7", &self.FIELD_7())
                .field("field_7", &self.field_7())
                .field("FIELD_8", &self.FIELD_8())
                .field("field_8", &self.field_8())
                .field("SBZ4", &self.SBZ4())
                .field("sbz4", &self.sbz4())
                .field("SBZ5", &self.SBZ5())
                .field("sbz5", &self.sbz5())
                .field("RFU", &self.RFU())
                .field("rfu", &self.rfu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_1 {{ FIELD_1_0: {=u8:?}, field_1_0: {=u8:?}, FIELD_3_2: {=u8:?}, field_3_2: {=u8:?}, SBZ1: {=bool:?}, sbz1: {=bool:?}, SBZ2: {=bool:?}, sbz2: {=bool:?}, SBZ3: {=bool:?}, sbz3: {=bool:?}, FIELD_7: {=bool:?}, field_7: {=bool:?}, FIELD_8: {=bool:?}, field_8: {=bool:?}, SBZ4: {=bool:?}, sbz4: {=bool:?}, SBZ5: {=bool:?}, sbz5: {=bool:?}, RFU: {=u32:?}, rfu: {=u32:?} }}" , self . FIELD_1_0 () , self . field_1_0 () , self . FIELD_3_2 () , self . field_3_2 () , self . SBZ1 () , self . sbz1 () , self . SBZ2 () , self . sbz2 () , self . SBZ3 () , self . sbz3 () , self . FIELD_7 () , self . field_7 () , self . FIELD_8 () , self . field_8 () , self . SBZ4 () , self . sbz4 () , self . SBZ5 () , self . sbz5 () , self . RFU () , self . rfu ())
        }
    }
    #[doc = "GDET Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_2(pub u32);
    impl GDET_CONF_2 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu1(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_rfu1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_21_16(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_21_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_21_16(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_21_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RFU2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rfu2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_29_24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_29_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_29_24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_29_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU3(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RFU3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu3(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rfu3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for GDET_CONF_2 {
        #[inline(always)]
        fn default() -> GDET_CONF_2 {
            GDET_CONF_2(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_2")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("field_6_0", &self.field_6_0())
                .field("RFU1", &self.RFU1())
                .field("rfu1", &self.rfu1())
                .field("FIELD_21_16", &self.FIELD_21_16())
                .field("field_21_16", &self.field_21_16())
                .field("RFU2", &self.RFU2())
                .field("rfu2", &self.rfu2())
                .field("FIELD_29_24", &self.FIELD_29_24())
                .field("field_29_24", &self.field_29_24())
                .field("RFU3", &self.RFU3())
                .field("rfu3", &self.rfu3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_2 {{ FIELD_6_0: {=u8:?}, field_6_0: {=u8:?}, RFU1: {=u16:?}, rfu1: {=u16:?}, FIELD_21_16: {=u8:?}, field_21_16: {=u8:?}, RFU2: {=u8:?}, rfu2: {=u8:?}, FIELD_29_24: {=u8:?}, field_29_24: {=u8:?}, RFU3: {=u8:?}, rfu3: {=u8:?} }}" , self . FIELD_6_0 () , self . field_6_0 () , self . RFU1 () , self . rfu1 () , self . FIELD_21_16 () , self . field_21_16 () , self . RFU2 () , self . rfu2 () , self . FIELD_29_24 () , self . field_29_24 () , self . RFU3 () , self . rfu3 ())
        }
    }
    #[doc = "GDET Configuration 3 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_3(pub u32);
    impl GDET_CONF_3 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for GDET_CONF_3 {
        #[inline(always)]
        fn default() -> GDET_CONF_3 {
            GDET_CONF_3(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_3")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("field_6_0", &self.field_6_0())
                .field("RFU1", &self.RFU1())
                .field("rfu1", &self.rfu1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_3 {{ FIELD_6_0: {=u8:?}, field_6_0: {=u8:?}, RFU1: {=u32:?}, rfu1: {=u32:?} }}" , self . FIELD_6_0 () , self . field_6_0 () , self . RFU1 () , self . rfu1 ())
        }
    }
    #[doc = "GDET Configuration 4 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_4(pub u32);
    impl GDET_CONF_4 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for GDET_CONF_4 {
        #[inline(always)]
        fn default() -> GDET_CONF_4 {
            GDET_CONF_4(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_4")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("field_6_0", &self.field_6_0())
                .field("RFU1", &self.RFU1())
                .field("rfu1", &self.rfu1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_4 {{ FIELD_6_0: {=u8:?}, field_6_0: {=u8:?}, RFU1: {=u32:?}, rfu1: {=u32:?} }}" , self . FIELD_6_0 () , self . field_6_0 () , self . RFU1 () , self . rfu1 ())
        }
    }
    #[doc = "GDET Configuration 5 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_5(pub u32);
    impl GDET_CONF_5 {
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_5_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_5_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_5_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_5_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIELD_11_6(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIELD_11_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn field_11_6(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_field_11_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu1(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu1(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for GDET_CONF_5 {
        #[inline(always)]
        fn default() -> GDET_CONF_5 {
            GDET_CONF_5(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_5")
                .field("FIELD_5_0", &self.FIELD_5_0())
                .field("field_5_0", &self.field_5_0())
                .field("FIELD_11_6", &self.FIELD_11_6())
                .field("field_11_6", &self.field_11_6())
                .field("RFU1", &self.RFU1())
                .field("rfu1", &self.rfu1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_5 {{ FIELD_5_0: {=u8:?}, field_5_0: {=u8:?}, FIELD_11_6: {=u8:?}, field_11_6: {=u8:?}, RFU1: {=u32:?}, rfu1: {=u32:?} }}" , self . FIELD_5_0 () , self . field_5_0 () , self . FIELD_11_6 () , self . field_11_6 () , self . RFU1 () , self . rfu1 ())
        }
    }
    #[doc = "GDET Delay Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_DLY_CTRL(pub u32);
    impl GDET_DLY_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn VOL_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VOL_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn vol_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_vol_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW_VOL_CTRL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW_VOL_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sw_vol_ctrl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sw_vol_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for GDET_DLY_CTRL {
        #[inline(always)]
        fn default() -> GDET_DLY_CTRL {
            GDET_DLY_CTRL(0)
        }
    }
    impl core::fmt::Debug for GDET_DLY_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_DLY_CTRL")
                .field("VOL_SEL", &self.VOL_SEL())
                .field("vol_sel", &self.vol_sel())
                .field("SW_VOL_CTRL", &self.SW_VOL_CTRL())
                .field("sw_vol_ctrl", &self.sw_vol_ctrl())
                .field("RFU", &self.RFU())
                .field("rfu", &self.rfu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_DLY_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_DLY_CTRL {{ VOL_SEL: {=u8:?}, vol_sel: {=u8:?}, SW_VOL_CTRL: {=bool:?}, sw_vol_ctrl: {=bool:?}, RFU: {=u32:?}, rfu: {=u32:?} }}" , self . VOL_SEL () , self . vol_sel () , self . SW_VOL_CTRL () , self . sw_vol_ctrl () , self . RFU () , self . rfu ())
        }
    }
    #[doc = "GDET Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_ENABLE1(pub u32);
    impl GDET_ENABLE1 {
        #[must_use]
        #[inline(always)]
        pub const fn EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for GDET_ENABLE1 {
        #[inline(always)]
        fn default() -> GDET_ENABLE1 {
            GDET_ENABLE1(0)
        }
    }
    impl core::fmt::Debug for GDET_ENABLE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_ENABLE1")
                .field("EN1", &self.EN1())
                .field("en1", &self.en1())
                .field("RFU", &self.RFU())
                .field("rfu", &self.rfu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_ENABLE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_ENABLE1 {{ EN1: {=bool:?}, en1: {=bool:?}, RFU: {=u32:?}, rfu: {=u32:?} }}",
                self.EN1(),
                self.en1(),
                self.RFU(),
                self.rfu()
            )
        }
    }
    #[doc = "GDET Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_RESET(pub u32);
    impl GDET_RESET {
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rfu1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SFT_RST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SFT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sft_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sft_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU2(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu2(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for GDET_RESET {
        #[inline(always)]
        fn default() -> GDET_RESET {
            GDET_RESET(0)
        }
    }
    impl core::fmt::Debug for GDET_RESET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_RESET")
                .field("RFU1", &self.RFU1())
                .field("rfu1", &self.rfu1())
                .field("SFT_RST", &self.SFT_RST())
                .field("sft_rst", &self.sft_rst())
                .field("RFU2", &self.RFU2())
                .field("rfu2", &self.rfu2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_RESET {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_RESET {{ RFU1: {=u8:?}, rfu1: {=u8:?}, SFT_RST: {=bool:?}, sft_rst: {=bool:?}, RFU2: {=u32:?}, rfu2: {=u32:?} }}" , self . RFU1 () , self . rfu1 () , self . SFT_RST () , self . sft_rst () , self . RFU2 () , self . rfu2 ())
        }
    }
    #[doc = "GDET Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_TEST(pub u32);
    impl GDET_TEST {
        #[must_use]
        #[inline(always)]
        pub const fn SBZ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn sbz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_sbz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rfu(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rfu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for GDET_TEST {
        #[inline(always)]
        fn default() -> GDET_TEST {
            GDET_TEST(0)
        }
    }
    impl core::fmt::Debug for GDET_TEST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_TEST")
                .field("SBZ", &self.SBZ())
                .field("sbz", &self.sbz())
                .field("RFU", &self.RFU())
                .field("rfu", &self.rfu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_TEST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_TEST {{ SBZ: {=bool:?}, sbz: {=bool:?}, RFU: {=u32:?}, rfu: {=u32:?} }}",
                self.SBZ(),
                self.sbz(),
                self.RFU(),
                self.rfu()
            )
        }
    }
}
