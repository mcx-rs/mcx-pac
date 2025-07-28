#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTC {
    ptr: *mut u8,
}
unsafe impl Send for RTC {}
unsafe impl Sync for RTC {}
impl RTC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn TSR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn TPR(self) -> crate::common::Reg<regs::TPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TAR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<regs::TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<regs::CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<regs::SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn LR(self) -> crate::common::Reg<regs::LR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RTC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR(pub u32);
    impl CR {
        #[must_use]
        #[inline(always)]
        pub const fn SWR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPOS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CR {
        #[inline(always)]
        fn default() -> CR {
            CR(0)
        }
    }
    impl core::fmt::Debug for CR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR")
                .field("SWR", &self.SWR())
                .field("UM", &self.UM())
                .field("LPOS", &self.LPOS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CR {{ SWR: {=bool:?}, UM: {=bool:?}, LPOS: {=bool:?} }}",
                self.SWR(),
                self.UM(),
                self.LPOS()
            )
        }
    }
    #[doc = "RTC Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[must_use]
        #[inline(always)]
        pub const fn TIIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TOIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAIE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TAIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TSIC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for IER {
        #[inline(always)]
        fn default() -> IER {
            IER(0)
        }
    }
    impl core::fmt::Debug for IER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IER")
                .field("TIIE", &self.TIIE())
                .field("TOIE", &self.TOIE())
                .field("TAIE", &self.TAIE())
                .field("TSIE", &self.TSIE())
                .field("TSIC", &self.TSIC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IER {{ TIIE: {=bool:?}, TOIE: {=bool:?}, TAIE: {=bool:?}, TSIE: {=bool:?}, TSIC: {=u8:?} }}" , self . TIIE () , self . TOIE () , self . TAIE () , self . TSIE () , self . TSIC ())
        }
    }
    #[doc = "RTC Lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LR(pub u32);
    impl LR {
        #[must_use]
        #[inline(always)]
        pub const fn TCL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LRL(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for LR {
        #[inline(always)]
        fn default() -> LR {
            LR(0)
        }
    }
    impl core::fmt::Debug for LR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LR")
                .field("TCL", &self.TCL())
                .field("CRL", &self.CRL())
                .field("SRL", &self.SRL())
                .field("LRL", &self.LRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LR {{ TCL: {=bool:?}, CRL: {=bool:?}, SRL: {=bool:?}, LRL: {=bool:?} }}",
                self.TCL(),
                self.CRL(),
                self.SRL(),
                self.LRL()
            )
        }
    }
    #[doc = "RTC Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR(pub u32);
    impl SR {
        #[must_use]
        #[inline(always)]
        pub const fn TIF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TAF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for SR {
        #[inline(always)]
        fn default() -> SR {
            SR(0)
        }
    }
    impl core::fmt::Debug for SR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SR")
                .field("TIF", &self.TIF())
                .field("TOF", &self.TOF())
                .field("TAF", &self.TAF())
                .field("TCE", &self.TCE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SR {{ TIF: {=bool:?}, TOF: {=bool:?}, TAF: {=bool:?}, TCE: {=bool:?} }}",
                self.TIF(),
                self.TOF(),
                self.TAF(),
                self.TCE()
            )
        }
    }
    #[doc = "RTC Time Compensation"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR(pub u32);
    impl TCR {
        #[must_use]
        #[inline(always)]
        pub const fn TCR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CIR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CIR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CIC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CIC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TCR {
        #[inline(always)]
        fn default() -> TCR {
            TCR(0)
        }
    }
    impl core::fmt::Debug for TCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR")
                .field("TCR", &self.TCR())
                .field("CIR", &self.CIR())
                .field("TCV", &self.TCV())
                .field("CIC", &self.CIC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TCR {{ TCR: {=u8:?}, CIR: {=u8:?}, TCV: {=u8:?}, CIC: {=u8:?} }}",
                self.TCR(),
                self.CIR(),
                self.TCV(),
                self.CIC()
            )
        }
    }
    #[doc = "RTC Time Prescaler"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TPR(pub u32);
    impl TPR {
        #[must_use]
        #[inline(always)]
        pub const fn TPR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TPR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TPR {
        #[inline(always)]
        fn default() -> TPR {
            TPR(0)
        }
    }
    impl core::fmt::Debug for TPR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TPR").field("TPR", &self.TPR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TPR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TPR {{ TPR: {=u16:?} }}", self.TPR())
        }
    }
}
