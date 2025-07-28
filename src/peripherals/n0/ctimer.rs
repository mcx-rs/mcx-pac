#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTIMER {
    ptr: *mut u8,
}
unsafe impl Send for CTIMER {}
unsafe impl Sync for CTIMER {}
impl CTIMER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn IR(self) -> crate::common::Reg<regs::IR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<regs::TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn PC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CCR(self) -> crate::common::Reg<regs::CCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn CR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn EMR(self) -> crate::common::Reg<regs::EMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTCR(self) -> crate::common::Reg<regs::CTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn PWMC(self) -> crate::common::Reg<regs::PWMC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Capture Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR(pub u32);
    impl CCR {
        #[must_use]
        #[inline(always)]
        pub const fn CAP0RE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP0RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP0FE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP0FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP0I(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP0I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP1RE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP1RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP1FE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP1FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP1I(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP1I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP2RE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP2RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP2FE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP2FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP2I(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP3RE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP3RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP3FE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP3FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAP3I(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAP3I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for CCR {
        #[inline(always)]
        fn default() -> CCR {
            CCR(0)
        }
    }
    impl core::fmt::Debug for CCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CCR")
                .field("CAP0RE", &self.CAP0RE())
                .field("CAP0FE", &self.CAP0FE())
                .field("CAP0I", &self.CAP0I())
                .field("CAP1RE", &self.CAP1RE())
                .field("CAP1FE", &self.CAP1FE())
                .field("CAP1I", &self.CAP1I())
                .field("CAP2RE", &self.CAP2RE())
                .field("CAP2FE", &self.CAP2FE())
                .field("CAP2I", &self.CAP2I())
                .field("CAP3RE", &self.CAP3RE())
                .field("CAP3FE", &self.CAP3FE())
                .field("CAP3I", &self.CAP3I())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CCR {{ CAP0RE: {=bool:?}, CAP0FE: {=bool:?}, CAP0I: {=bool:?}, CAP1RE: {=bool:?}, CAP1FE: {=bool:?}, CAP1I: {=bool:?}, CAP2RE: {=bool:?}, CAP2FE: {=bool:?}, CAP2I: {=bool:?}, CAP3RE: {=bool:?}, CAP3FE: {=bool:?}, CAP3I: {=bool:?} }}" , self . CAP0RE () , self . CAP0FE () , self . CAP0I () , self . CAP1RE () , self . CAP1FE () , self . CAP1I () , self . CAP2RE () , self . CAP2FE () , self . CAP2I () , self . CAP3RE () , self . CAP3FE () , self . CAP3I ())
        }
    }
    #[doc = "Count Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTCR(pub u32);
    impl CTCR {
        #[must_use]
        #[inline(always)]
        pub const fn CTMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINSEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CINSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENCC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ENCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SELCC(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SELCC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
    }
    impl Default for CTCR {
        #[inline(always)]
        fn default() -> CTCR {
            CTCR(0)
        }
    }
    impl core::fmt::Debug for CTCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTCR")
                .field("CTMODE", &self.CTMODE())
                .field("CINSEL", &self.CINSEL())
                .field("ENCC", &self.ENCC())
                .field("SELCC", &self.SELCC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CTCR {{ CTMODE: {=u8:?}, CINSEL: {=u8:?}, ENCC: {=bool:?}, SELCC: {=u8:?} }}",
                self.CTMODE(),
                self.CINSEL(),
                self.ENCC(),
                self.SELCC()
            )
        }
    }
    #[doc = "External Match"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EMR(pub u32);
    impl EMR {
        #[must_use]
        #[inline(always)]
        pub const fn EM0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EM1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EM2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EM2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EM3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EM3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMC0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMC1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMC2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMC2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMC3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMC3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for EMR {
        #[inline(always)]
        fn default() -> EMR {
            EMR(0)
        }
    }
    impl core::fmt::Debug for EMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EMR")
                .field("EM0", &self.EM0())
                .field("EM1", &self.EM1())
                .field("EM2", &self.EM2())
                .field("EM3", &self.EM3())
                .field("EMC0", &self.EMC0())
                .field("EMC1", &self.EMC1())
                .field("EMC2", &self.EMC2())
                .field("EMC3", &self.EMC3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EMR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EMR {{ EM0: {=bool:?}, EM1: {=bool:?}, EM2: {=bool:?}, EM3: {=bool:?}, EMC0: {=u8:?}, EMC1: {=u8:?}, EMC2: {=u8:?}, EMC3: {=u8:?} }}" , self . EM0 () , self . EM1 () , self . EM2 () , self . EM3 () , self . EMC0 () , self . EMC1 () , self . EMC2 () , self . EMC3 ())
        }
    }
    #[doc = "Interrupt"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IR(pub u32);
    impl IR {
        #[must_use]
        #[inline(always)]
        pub const fn MR0INT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR0INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR1INT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR1INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR2INT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR2INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR3INT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR3INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CR0INT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CR0INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CR1INT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CR1INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CR2INT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CR2INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CR3INT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CR3INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for IR {
        #[inline(always)]
        fn default() -> IR {
            IR(0)
        }
    }
    impl core::fmt::Debug for IR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IR")
                .field("MR0INT", &self.MR0INT())
                .field("MR1INT", &self.MR1INT())
                .field("MR2INT", &self.MR2INT())
                .field("MR3INT", &self.MR3INT())
                .field("CR0INT", &self.CR0INT())
                .field("CR1INT", &self.CR1INT())
                .field("CR2INT", &self.CR2INT())
                .field("CR3INT", &self.CR3INT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IR {{ MR0INT: {=bool:?}, MR1INT: {=bool:?}, MR2INT: {=bool:?}, MR3INT: {=bool:?}, CR0INT: {=bool:?}, CR1INT: {=bool:?}, CR2INT: {=bool:?}, CR3INT: {=bool:?} }}" , self . MR0INT () , self . MR1INT () , self . MR2INT () , self . MR3INT () , self . CR0INT () , self . CR1INT () , self . CR2INT () , self . CR3INT ())
        }
    }
    #[doc = "Match Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[must_use]
        #[inline(always)]
        pub const fn MR0I(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR0I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR0R(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR0R(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR0S(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR0S(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR1I(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR1I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR1R(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR1R(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR1S(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR1S(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR2I(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR2R(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR2R(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR2S(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR2S(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR3I(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR3I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR3R(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR3R(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR3S(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR3S(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR0RL(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR0RL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR1RL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR1RL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR2RL(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR2RL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MR3RL(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MR3RL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MCR {
        #[inline(always)]
        fn default() -> MCR {
            MCR(0)
        }
    }
    impl core::fmt::Debug for MCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR")
                .field("MR0I", &self.MR0I())
                .field("MR0R", &self.MR0R())
                .field("MR0S", &self.MR0S())
                .field("MR1I", &self.MR1I())
                .field("MR1R", &self.MR1R())
                .field("MR1S", &self.MR1S())
                .field("MR2I", &self.MR2I())
                .field("MR2R", &self.MR2R())
                .field("MR2S", &self.MR2S())
                .field("MR3I", &self.MR3I())
                .field("MR3R", &self.MR3R())
                .field("MR3S", &self.MR3S())
                .field("MR0RL", &self.MR0RL())
                .field("MR1RL", &self.MR1RL())
                .field("MR2RL", &self.MR2RL())
                .field("MR3RL", &self.MR3RL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCR {{ MR0I: {=bool:?}, MR0R: {=bool:?}, MR0S: {=bool:?}, MR1I: {=bool:?}, MR1R: {=bool:?}, MR1S: {=bool:?}, MR2I: {=bool:?}, MR2R: {=bool:?}, MR2S: {=bool:?}, MR3I: {=bool:?}, MR3R: {=bool:?}, MR3S: {=bool:?}, MR0RL: {=bool:?}, MR1RL: {=bool:?}, MR2RL: {=bool:?}, MR3RL: {=bool:?} }}" , self . MR0I () , self . MR0R () , self . MR0S () , self . MR1I () , self . MR1R () , self . MR1S () , self . MR2I () , self . MR2R () , self . MR2S () , self . MR3I () , self . MR3R () , self . MR3S () , self . MR0RL () , self . MR1RL () , self . MR2RL () , self . MR3RL ())
        }
    }
    #[doc = "PWM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWMC(pub u32);
    impl PWMC {
        #[must_use]
        #[inline(always)]
        pub const fn PWMEN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMEN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMEN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMEN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMEN2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMEN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMEN3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMEN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PWMC {
        #[inline(always)]
        fn default() -> PWMC {
            PWMC(0)
        }
    }
    impl core::fmt::Debug for PWMC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWMC")
                .field("PWMEN0", &self.PWMEN0())
                .field("PWMEN1", &self.PWMEN1())
                .field("PWMEN2", &self.PWMEN2())
                .field("PWMEN3", &self.PWMEN3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWMC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PWMC {{ PWMEN0: {=bool:?}, PWMEN1: {=bool:?}, PWMEN2: {=bool:?}, PWMEN3: {=bool:?} }}" , self . PWMEN0 () , self . PWMEN1 () , self . PWMEN2 () , self . PWMEN3 ())
        }
    }
    #[doc = "Timer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR(pub u32);
    impl TCR {
        #[must_use]
        #[inline(always)]
        pub const fn CEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AGCEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AGCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ATCEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ATCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("CEN", &self.CEN())
                .field("CRST", &self.CRST())
                .field("AGCEN", &self.AGCEN())
                .field("ATCEN", &self.ATCEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TCR {{ CEN: {=bool:?}, CRST: {=bool:?}, AGCEN: {=bool:?}, ATCEN: {=bool:?} }}",
                self.CEN(),
                self.CRST(),
                self.AGCEN(),
                self.ATCEN()
            )
        }
    }
}
