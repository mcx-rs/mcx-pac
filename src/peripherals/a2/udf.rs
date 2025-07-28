#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDF {
    ptr: *mut u8,
}
unsafe impl Send for UDF {}
unsafe impl Sync for UDF {}
impl UDF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn UDF_CTRL(self) -> crate::common::Reg<regs::UDF_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn UDF_STATUS(self) -> crate::common::Reg<regs::UDF_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn UDF_WR_DATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn UDF_RD_DATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UDF_CTRL(pub u32);
    impl UDF_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn salt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_salt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reserved21(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_reserved21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn udf_en(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_udf_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reserved25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_reserved25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reserved27(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_reserved27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn flush(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_flush(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn reserved31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_reserved31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for UDF_CTRL {
        #[inline(always)]
        fn default() -> UDF_CTRL {
            UDF_CTRL(0)
        }
    }
    impl core::fmt::Debug for UDF_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UDF_CTRL")
                .field("salt", &self.salt())
                .field("lock", &self.lock())
                .field("reserved21", &self.reserved21())
                .field("udf_en", &self.udf_en())
                .field("reserved25", &self.reserved25())
                .field("reserved27", &self.reserved27())
                .field("flush", &self.flush())
                .field("reserved31", &self.reserved31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UDF_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UDF_CTRL {{ salt: {=u16:?}, lock: {=u8:?}, reserved21: {=u8:?}, udf_en: {=u8:?}, reserved25: {=bool:?}, reserved27: {=u8:?}, flush: {=u8:?}, reserved31: {=bool:?} }}" , self . salt () , self . lock () , self . reserved21 () , self . udf_en () , self . reserved25 () , self . reserved27 () , self . flush () , self . reserved31 ())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UDF_STATUS(pub u32);
    impl UDF_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn o_status(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_o_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x03ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 5usize)) | (((val as u32) & 0x03ff_ffff) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn o_wait(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_o_wait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for UDF_STATUS {
        #[inline(always)]
        fn default() -> UDF_STATUS {
            UDF_STATUS(0)
        }
    }
    impl core::fmt::Debug for UDF_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UDF_STATUS")
                .field("o_status", &self.o_status())
                .field("rsv", &self.rsv())
                .field("o_wait", &self.o_wait())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UDF_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UDF_STATUS {{ o_status: {=u8:?}, rsv: {=u32:?}, o_wait: {=bool:?} }}",
                self.o_status(),
                self.rsv(),
                self.o_wait()
            )
        }
    }
}
