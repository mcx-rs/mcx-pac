#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRC {
    ptr: *mut u8,
}
unsafe impl Send for CRC {}
unsafe impl Sync for CRC {}
impl CRC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn DATA(self) -> crate::common::Reg<regs::DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATALL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DATALU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAHL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn DATAHU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLY(self) -> crate::common::Reg<regs::GPOLY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYLL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYLU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYHL(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn GPOLYHU(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRLHU(
        self,
    ) -> crate::common::Reg<regs::CTRL_ACCESS8BIT_CTRLHU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
}
pub mod regs {
    #[doc = "CRC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn TCRC(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FXOR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FXOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOTR(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CTRL {
        #[inline(always)]
        fn default() -> CTRL {
            CTRL(0)
        }
    }
    impl core::fmt::Debug for CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL")
                .field("TCRC", &self.TCRC())
                .field("WAS", &self.WAS())
                .field("FXOR", &self.FXOR())
                .field("TOTR", &self.TOTR())
                .field("TOT", &self.TOT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ TCRC: {=bool:?}, WAS: {=bool:?}, FXOR: {=bool:?}, TOTR: {=u8:?}, TOT: {=u8:?} }}" , self . TCRC () , self . WAS () , self . FXOR () , self . TOTR () , self . TOT ())
        }
    }
    #[doc = "CRC_CTRLHU register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_ACCESS8BIT_CTRLHU(pub u8);
    impl CTRL_ACCESS8BIT_CTRLHU {
        #[must_use]
        #[inline(always)]
        pub const fn TCRC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FXOR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FXOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOTR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOT(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for CTRL_ACCESS8BIT_CTRLHU {
        #[inline(always)]
        fn default() -> CTRL_ACCESS8BIT_CTRLHU {
            CTRL_ACCESS8BIT_CTRLHU(0)
        }
    }
    impl core::fmt::Debug for CTRL_ACCESS8BIT_CTRLHU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_ACCESS8BIT_CTRLHU")
                .field("TCRC", &self.TCRC())
                .field("WAS", &self.WAS())
                .field("FXOR", &self.FXOR())
                .field("TOTR", &self.TOTR())
                .field("TOT", &self.TOT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_ACCESS8BIT_CTRLHU {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL_ACCESS8BIT_CTRLHU {{ TCRC: {=bool:?}, WAS: {=bool:?}, FXOR: {=bool:?}, TOTR: {=u8:?}, TOT: {=u8:?} }}" , self . TCRC () , self . WAS () , self . FXOR () , self . TOTR () , self . TOT ())
        }
    }
    #[doc = "CRC Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA(pub u32);
    impl DATA {
        #[must_use]
        #[inline(always)]
        pub const fn LL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LU(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HU(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DATA {
        #[inline(always)]
        fn default() -> DATA {
            DATA(0)
        }
    }
    impl core::fmt::Debug for DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATA")
                .field("LL", &self.LL())
                .field("LU", &self.LU())
                .field("HL", &self.HL())
                .field("HU", &self.HU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DATA {{ LL: {=u8:?}, LU: {=u8:?}, HL: {=u8:?}, HU: {=u8:?} }}",
                self.LL(),
                self.LU(),
                self.HL(),
                self.HU()
            )
        }
    }
    #[doc = "CRC Polynomial"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPOLY(pub u32);
    impl GPOLY {
        #[must_use]
        #[inline(always)]
        pub const fn LOW(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LOW(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HIGH(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_HIGH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GPOLY {
        #[inline(always)]
        fn default() -> GPOLY {
            GPOLY(0)
        }
    }
    impl core::fmt::Debug for GPOLY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPOLY")
                .field("LOW", &self.LOW())
                .field("HIGH", &self.HIGH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPOLY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GPOLY {{ LOW: {=u16:?}, HIGH: {=u16:?} }}",
                self.LOW(),
                self.HIGH()
            )
        }
    }
}
