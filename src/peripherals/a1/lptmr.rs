#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR {
    ptr: *mut u8,
}
unsafe impl Send for LPTMR {}
unsafe impl Sync for LPTMR {}
impl LPTMR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<regs::CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PSR(self) -> crate::common::Reg<regs::PSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CMR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CNR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[must_use]
        #[inline(always)]
        pub const fn TEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TMS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPP(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TPP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDRE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for CSR {
        #[inline(always)]
        fn default() -> CSR {
            CSR(0)
        }
    }
    impl core::fmt::Debug for CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSR")
                .field("TEN", &self.TEN())
                .field("TMS", &self.TMS())
                .field("TFC", &self.TFC())
                .field("TPP", &self.TPP())
                .field("TPS", &self.TPS())
                .field("TIE", &self.TIE())
                .field("TCF", &self.TCF())
                .field("TDRE", &self.TDRE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CSR {{ TEN: {=bool:?}, TMS: {=bool:?}, TFC: {=bool:?}, TPP: {=bool:?}, TPS: {=u8:?}, TIE: {=bool:?}, TCF: {=bool:?}, TDRE: {=bool:?} }}" , self . TEN () , self . TMS () , self . TFC () , self . TPP () , self . TPS () , self . TIE () , self . TCF () , self . TDRE ())
        }
    }
    #[doc = "Prescaler and Glitch Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PSR(pub u32);
    impl PSR {
        #[must_use]
        #[inline(always)]
        pub const fn PCS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PBYP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PBYP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRESCALE(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PRESCALE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
    }
    impl Default for PSR {
        #[inline(always)]
        fn default() -> PSR {
            PSR(0)
        }
    }
    impl core::fmt::Debug for PSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PSR")
                .field("PCS", &self.PCS())
                .field("PBYP", &self.PBYP())
                .field("PRESCALE", &self.PRESCALE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PSR {{ PCS: {=u8:?}, PBYP: {=bool:?}, PRESCALE: {=u8:?} }}",
                self.PCS(),
                self.PBYP(),
                self.PRESCALE()
            )
        }
    }
}
