#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCG {
    ptr: *mut u8,
}
unsafe impl Send for SCG {}
unsafe impl Sync for SCG {}
impl SCG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_LOCK(self) -> crate::common::Reg<regs::TRIM_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<regs::CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn RCCR(self) -> crate::common::Reg<regs::RCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn SOSCCSR(self) -> crate::common::Reg<regs::SOSCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn SOSCCFG(self) -> crate::common::Reg<regs::SOSCCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCCSR(self) -> crate::common::Reg<regs::SIRCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCTCFG(self) -> crate::common::Reg<regs::SIRCTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCTRIM(self) -> crate::common::Reg<regs::SIRCTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCSTAT(self) -> crate::common::Reg<regs::SIRCSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCCSR(self) -> crate::common::Reg<regs::FIRCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCCFG(self) -> crate::common::Reg<regs::FIRCCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCTCFG(self) -> crate::common::Reg<regs::FIRCTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCTRIM(self) -> crate::common::Reg<regs::FIRCTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCSTAT(self) -> crate::common::Reg<regs::FIRCSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC1(self) -> crate::common::Reg<regs::FIRCATC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC2(self) -> crate::common::Reg<regs::FIRCATC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC3(self) -> crate::common::Reg<regs::FIRCATC3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC4(self) -> crate::common::Reg<regs::FIRCATC4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC5(self) -> crate::common::Reg<regs::FIRCATC5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC6(self) -> crate::common::Reg<regs::FIRCATC6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC7(self) -> crate::common::Reg<regs::FIRCATC7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC8(self) -> crate::common::Reg<regs::FIRCATC8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC9(self) -> crate::common::Reg<regs::FIRCATC9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC10(self) -> crate::common::Reg<regs::FIRCATC10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCATC11(self) -> crate::common::Reg<regs::FIRCATC11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[inline(always)]
    pub const fn ROSCCSR(self) -> crate::common::Reg<regs::ROSCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[must_use]
        #[inline(always)]
        pub const fn SCS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
            f.debug_struct("CSR").field("SCS", &self.SCS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CSR {{ SCS: {=u8:?} }}", self.SCS())
        }
    }
    #[doc = "FIRC Auto-trimming Counter 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC1(pub u32);
    impl FIRCATC1 {
        #[must_use]
        #[inline(always)]
        pub const fn IDEALC_SOSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_IDEALC_SOSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAMCYC_SOSC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SAMCYC_SOSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SETCYC_SOSC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SETCYC_SOSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for FIRCATC1 {
        #[inline(always)]
        fn default() -> FIRCATC1 {
            FIRCATC1(0)
        }
    }
    impl core::fmt::Debug for FIRCATC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC1")
                .field("IDEALC_SOSC", &self.IDEALC_SOSC())
                .field("SAMCYC_SOSC", &self.SAMCYC_SOSC())
                .field("SETCYC_SOSC", &self.SETCYC_SOSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC1 {{ IDEALC_SOSC: {=u16:?}, SAMCYC_SOSC: {=u8:?}, SETCYC_SOSC: {=u8:?} }}",
                self.IDEALC_SOSC(),
                self.SAMCYC_SOSC(),
                self.SETCYC_SOSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 10"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC10(pub u32);
    impl FIRCATC10 {
        #[must_use]
        #[inline(always)]
        pub const fn FINEMAXC_SOF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_FINEMAXC_SOF(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for FIRCATC10 {
        #[inline(always)]
        fn default() -> FIRCATC10 {
            FIRCATC10(0)
        }
    }
    impl core::fmt::Debug for FIRCATC10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC10")
                .field("FINEMAXC_SOF", &self.FINEMAXC_SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC10 {{ FINEMAXC_SOF: {=u32:?} }}",
                self.FINEMAXC_SOF()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 11"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC11(pub u32);
    impl FIRCATC11 {
        #[must_use]
        #[inline(always)]
        pub const fn FINEMINC_SOF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_FINEMINC_SOF(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for FIRCATC11 {
        #[inline(always)]
        fn default() -> FIRCATC11 {
            FIRCATC11(0)
        }
    }
    impl core::fmt::Debug for FIRCATC11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC11")
                .field("FINEMINC_SOF", &self.FINEMINC_SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC11 {{ FINEMINC_SOF: {=u32:?} }}",
                self.FINEMINC_SOF()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC2(pub u32);
    impl FIRCATC2 {
        #[must_use]
        #[inline(always)]
        pub const fn COARMINC_SOSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_COARMINC_SOSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COARMAXC_SOSC(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_COARMAXC_SOSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FIRCATC2 {
        #[inline(always)]
        fn default() -> FIRCATC2 {
            FIRCATC2(0)
        }
    }
    impl core::fmt::Debug for FIRCATC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC2")
                .field("COARMINC_SOSC", &self.COARMINC_SOSC())
                .field("COARMAXC_SOSC", &self.COARMAXC_SOSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC2 {{ COARMINC_SOSC: {=u16:?}, COARMAXC_SOSC: {=u16:?} }}",
                self.COARMINC_SOSC(),
                self.COARMAXC_SOSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC3(pub u32);
    impl FIRCATC3 {
        #[must_use]
        #[inline(always)]
        pub const fn FINEMINC_SOSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FINEMINC_SOSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FINEMAXC_SOSC(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FINEMAXC_SOSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FIRCATC3 {
        #[inline(always)]
        fn default() -> FIRCATC3 {
            FIRCATC3(0)
        }
    }
    impl core::fmt::Debug for FIRCATC3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC3")
                .field("FINEMINC_SOSC", &self.FINEMINC_SOSC())
                .field("FINEMAXC_SOSC", &self.FINEMAXC_SOSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC3 {{ FINEMINC_SOSC: {=u16:?}, FINEMAXC_SOSC: {=u16:?} }}",
                self.FINEMINC_SOSC(),
                self.FINEMAXC_SOSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC4(pub u32);
    impl FIRCATC4 {
        #[must_use]
        #[inline(always)]
        pub const fn IDEALC_ROSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_IDEALC_ROSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SETCYC_ROSC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SETCYC_ROSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAMCYC_ROSC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SAMCYC_ROSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for FIRCATC4 {
        #[inline(always)]
        fn default() -> FIRCATC4 {
            FIRCATC4(0)
        }
    }
    impl core::fmt::Debug for FIRCATC4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC4")
                .field("IDEALC_ROSC", &self.IDEALC_ROSC())
                .field("SETCYC_ROSC", &self.SETCYC_ROSC())
                .field("SAMCYC_ROSC", &self.SAMCYC_ROSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC4 {{ IDEALC_ROSC: {=u16:?}, SETCYC_ROSC: {=u8:?}, SAMCYC_ROSC: {=u8:?} }}",
                self.IDEALC_ROSC(),
                self.SETCYC_ROSC(),
                self.SAMCYC_ROSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC5(pub u32);
    impl FIRCATC5 {
        #[must_use]
        #[inline(always)]
        pub const fn COARMINC_ROSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_COARMINC_ROSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COARMAXC_ROSC(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_COARMAXC_ROSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FIRCATC5 {
        #[inline(always)]
        fn default() -> FIRCATC5 {
            FIRCATC5(0)
        }
    }
    impl core::fmt::Debug for FIRCATC5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC5")
                .field("COARMINC_ROSC", &self.COARMINC_ROSC())
                .field("COARMAXC_ROSC", &self.COARMAXC_ROSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC5 {{ COARMINC_ROSC: {=u16:?}, COARMAXC_ROSC: {=u16:?} }}",
                self.COARMINC_ROSC(),
                self.COARMAXC_ROSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 6"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC6(pub u32);
    impl FIRCATC6 {
        #[must_use]
        #[inline(always)]
        pub const fn FINEMINC_ROSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FINEMINC_ROSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FINEMAXC_ROSC(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FINEMAXC_ROSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FIRCATC6 {
        #[inline(always)]
        fn default() -> FIRCATC6 {
            FIRCATC6(0)
        }
    }
    impl core::fmt::Debug for FIRCATC6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC6")
                .field("FINEMINC_ROSC", &self.FINEMINC_ROSC())
                .field("FINEMAXC_ROSC", &self.FINEMAXC_ROSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC6 {{ FINEMINC_ROSC: {=u16:?}, FINEMAXC_ROSC: {=u16:?} }}",
                self.FINEMINC_ROSC(),
                self.FINEMAXC_ROSC()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC7(pub u32);
    impl FIRCATC7 {
        #[must_use]
        #[inline(always)]
        pub const fn IDEALC_SOF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_IDEALC_SOF(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SETCYC_SOF(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SETCYC_SOF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAMCYC_SOF(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SAMCYC_SOF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FIRCATC7 {
        #[inline(always)]
        fn default() -> FIRCATC7 {
            FIRCATC7(0)
        }
    }
    impl core::fmt::Debug for FIRCATC7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC7")
                .field("IDEALC_SOF", &self.IDEALC_SOF())
                .field("SETCYC_SOF", &self.SETCYC_SOF())
                .field("SAMCYC_SOF", &self.SAMCYC_SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC7 {{ IDEALC_SOF: {=u32:?}, SETCYC_SOF: {=u8:?}, SAMCYC_SOF: {=u8:?} }}",
                self.IDEALC_SOF(),
                self.SETCYC_SOF(),
                self.SAMCYC_SOF()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 8"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC8(pub u32);
    impl FIRCATC8 {
        #[must_use]
        #[inline(always)]
        pub const fn COARMAXC_SOF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_COARMAXC_SOF(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for FIRCATC8 {
        #[inline(always)]
        fn default() -> FIRCATC8 {
            FIRCATC8(0)
        }
    }
    impl core::fmt::Debug for FIRCATC8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC8")
                .field("COARMAXC_SOF", &self.COARMAXC_SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC8 {{ COARMAXC_SOF: {=u32:?} }}",
                self.COARMAXC_SOF()
            )
        }
    }
    #[doc = "FIRC Auto-trimming Counter 9"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCATC9(pub u32);
    impl FIRCATC9 {
        #[must_use]
        #[inline(always)]
        pub const fn COARMINC_SOF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_COARMINC_SOF(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for FIRCATC9 {
        #[inline(always)]
        fn default() -> FIRCATC9 {
            FIRCATC9(0)
        }
    }
    impl core::fmt::Debug for FIRCATC9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCATC9")
                .field("COARMINC_SOF", &self.COARMINC_SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCATC9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCATC9 {{ COARMINC_SOF: {=u32:?} }}",
                self.COARMINC_SOF()
            )
        }
    }
    #[doc = "FIRC Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCCFG(pub u32);
    impl FIRCCFG {
        #[must_use]
        #[inline(always)]
        pub const fn FREQ_SEL(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FREQ_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
    }
    impl Default for FIRCCFG {
        #[inline(always)]
        fn default() -> FIRCCFG {
            FIRCCFG(0)
        }
    }
    impl core::fmt::Debug for FIRCCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCCFG")
                .field("FREQ_SEL", &self.FREQ_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FIRCCFG {{ FREQ_SEL: {=u8:?} }}", self.FREQ_SEL())
        }
    }
    #[doc = "FIRC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCCSR(pub u32);
    impl FIRCCSR {
        #[must_use]
        #[inline(always)]
        pub const fn FIRCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRC_SCLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRC_SCLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRC_FCLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRC_FCLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCTREN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCTREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCTRUP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCTRUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIM_LOCK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRIM_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COARSE_TRIM_BYPASS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COARSE_TRIM_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCERR_IE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCERR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCACC_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCACC_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCACC(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCACC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FIRCCSR {
        #[inline(always)]
        fn default() -> FIRCCSR {
            FIRCCSR(0)
        }
    }
    impl core::fmt::Debug for FIRCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCCSR")
                .field("FIRCEN", &self.FIRCEN())
                .field("FIRCSTEN", &self.FIRCSTEN())
                .field("FIRC_SCLK_PERIPH_EN", &self.FIRC_SCLK_PERIPH_EN())
                .field("FIRC_FCLK_PERIPH_EN", &self.FIRC_FCLK_PERIPH_EN())
                .field("FIRCTREN", &self.FIRCTREN())
                .field("FIRCTRUP", &self.FIRCTRUP())
                .field("TRIM_LOCK", &self.TRIM_LOCK())
                .field("COARSE_TRIM_BYPASS", &self.COARSE_TRIM_BYPASS())
                .field("LK", &self.LK())
                .field("FIRCVLD", &self.FIRCVLD())
                .field("FIRCSEL", &self.FIRCSEL())
                .field("FIRCERR", &self.FIRCERR())
                .field("FIRCERR_IE", &self.FIRCERR_IE())
                .field("FIRCACC_IE", &self.FIRCACC_IE())
                .field("FIRCACC", &self.FIRCACC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FIRCCSR {{ FIRCEN: {=bool:?}, FIRCSTEN: {=bool:?}, FIRC_SCLK_PERIPH_EN: {=bool:?}, FIRC_FCLK_PERIPH_EN: {=bool:?}, FIRCTREN: {=bool:?}, FIRCTRUP: {=bool:?}, TRIM_LOCK: {=bool:?}, COARSE_TRIM_BYPASS: {=bool:?}, LK: {=bool:?}, FIRCVLD: {=bool:?}, FIRCSEL: {=bool:?}, FIRCERR: {=bool:?}, FIRCERR_IE: {=bool:?}, FIRCACC_IE: {=bool:?}, FIRCACC: {=bool:?} }}" , self . FIRCEN () , self . FIRCSTEN () , self . FIRC_SCLK_PERIPH_EN () , self . FIRC_FCLK_PERIPH_EN () , self . FIRCTREN () , self . FIRCTRUP () , self . TRIM_LOCK () , self . COARSE_TRIM_BYPASS () , self . LK () , self . FIRCVLD () , self . FIRCSEL () , self . FIRCERR () , self . FIRCERR_IE () , self . FIRCACC_IE () , self . FIRCACC ())
        }
    }
    #[doc = "FIRC Auto-trimming Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCSTAT(pub u32);
    impl FIRCSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn TRIMFINE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMFINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMCOAR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMCOAR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for FIRCSTAT {
        #[inline(always)]
        fn default() -> FIRCSTAT {
            FIRCSTAT(0)
        }
    }
    impl core::fmt::Debug for FIRCSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCSTAT")
                .field("TRIMFINE", &self.TRIMFINE())
                .field("TRIMCOAR", &self.TRIMCOAR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCSTAT {{ TRIMFINE: {=u8:?}, TRIMCOAR: {=u8:?} }}",
                self.TRIMFINE(),
                self.TRIMCOAR()
            )
        }
    }
    #[doc = "FIRC Trim Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCTCFG(pub u32);
    impl FIRCTCFG {
        #[must_use]
        #[inline(always)]
        pub const fn TRIMSRC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMDIV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for FIRCTCFG {
        #[inline(always)]
        fn default() -> FIRCTCFG {
            FIRCTCFG(0)
        }
    }
    impl core::fmt::Debug for FIRCTCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCTCFG")
                .field("TRIMSRC", &self.TRIMSRC())
                .field("TRIMDIV", &self.TRIMDIV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCTCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCTCFG {{ TRIMSRC: {=u8:?}, TRIMDIV: {=u8:?} }}",
                self.TRIMSRC(),
                self.TRIMDIV()
            )
        }
    }
    #[doc = "FIRC Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCTRIM(pub u32);
    impl FIRCTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn TRIMFINE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMFINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMCOAR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMCOAR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMTEMP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMTEMP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMSTART(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMSTART(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for FIRCTRIM {
        #[inline(always)]
        fn default() -> FIRCTRIM {
            FIRCTRIM(0)
        }
    }
    impl core::fmt::Debug for FIRCTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCTRIM")
                .field("TRIMFINE", &self.TRIMFINE())
                .field("TRIMCOAR", &self.TRIMCOAR())
                .field("TRIMTEMP", &self.TRIMTEMP())
                .field("TRIMSTART", &self.TRIMSTART())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FIRCTRIM {{ TRIMFINE: {=u8:?}, TRIMCOAR: {=u8:?}, TRIMTEMP: {=u8:?}, TRIMSTART: {=u8:?} }}" , self . TRIMFINE () , self . TRIMCOAR () , self . TRIMTEMP () , self . TRIMSTART ())
        }
    }
    #[doc = "Parameter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn SOSCCLKPRES(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCCLKPRES(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIRCCLKPRES(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIRCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ROSCCLKPRES(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ROSCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    impl core::fmt::Debug for PARAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAM")
                .field("SOSCCLKPRES", &self.SOSCCLKPRES())
                .field("SIRCCLKPRES", &self.SIRCCLKPRES())
                .field("FIRCCLKPRES", &self.FIRCCLKPRES())
                .field("ROSCCLKPRES", &self.ROSCCLKPRES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PARAM {{ SOSCCLKPRES: {=bool:?}, SIRCCLKPRES: {=bool:?}, FIRCCLKPRES: {=bool:?}, ROSCCLKPRES: {=bool:?} }}" , self . SOSCCLKPRES () , self . SIRCCLKPRES () , self . FIRCCLKPRES () , self . ROSCCLKPRES ())
        }
    }
    #[doc = "Run Clock Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCCR(pub u32);
    impl RCCR {
        #[must_use]
        #[inline(always)]
        pub const fn SCS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for RCCR {
        #[inline(always)]
        fn default() -> RCCR {
            RCCR(0)
        }
    }
    impl core::fmt::Debug for RCCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCCR").field("SCS", &self.SCS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RCCR {{ SCS: {=u8:?} }}", self.SCS())
        }
    }
    #[doc = "ROSC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ROSCCSR(pub u32);
    impl ROSCCSR {
        #[must_use]
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ROSCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ROSCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ROSCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ROSCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ROSCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ROSCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for ROSCCSR {
        #[inline(always)]
        fn default() -> ROSCCSR {
            ROSCCSR(0)
        }
    }
    impl core::fmt::Debug for ROSCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ROSCCSR")
                .field("LK", &self.LK())
                .field("ROSCVLD", &self.ROSCVLD())
                .field("ROSCSEL", &self.ROSCSEL())
                .field("ROSCERR", &self.ROSCERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ROSCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ROSCCSR {{ LK: {=bool:?}, ROSCVLD: {=bool:?}, ROSCSEL: {=bool:?}, ROSCERR: {=bool:?} }}" , self . LK () , self . ROSCVLD () , self . ROSCSEL () , self . ROSCERR ())
        }
    }
    #[doc = "SIRC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCCSR(pub u32);
    impl SIRCCSR {
        #[must_use]
        #[inline(always)]
        pub const fn SIRCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRC_CLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRC_CLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCTREN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCTREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCTRUP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCTRUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIM_LOCK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRIM_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COARSE_TRIM_BYPASS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COARSE_TRIM_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIRCERR_IE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SIRCERR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SIRCCSR {
        #[inline(always)]
        fn default() -> SIRCCSR {
            SIRCCSR(0)
        }
    }
    impl core::fmt::Debug for SIRCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCCSR")
                .field("SIRCSTEN", &self.SIRCSTEN())
                .field("SIRC_CLK_PERIPH_EN", &self.SIRC_CLK_PERIPH_EN())
                .field("SIRCTREN", &self.SIRCTREN())
                .field("SIRCTRUP", &self.SIRCTRUP())
                .field("TRIM_LOCK", &self.TRIM_LOCK())
                .field("COARSE_TRIM_BYPASS", &self.COARSE_TRIM_BYPASS())
                .field("LK", &self.LK())
                .field("SIRCVLD", &self.SIRCVLD())
                .field("SIRCSEL", &self.SIRCSEL())
                .field("SIRCERR", &self.SIRCERR())
                .field("SIRCERR_IE", &self.SIRCERR_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SIRCCSR {{ SIRCSTEN: {=bool:?}, SIRC_CLK_PERIPH_EN: {=bool:?}, SIRCTREN: {=bool:?}, SIRCTRUP: {=bool:?}, TRIM_LOCK: {=bool:?}, COARSE_TRIM_BYPASS: {=bool:?}, LK: {=bool:?}, SIRCVLD: {=bool:?}, SIRCSEL: {=bool:?}, SIRCERR: {=bool:?}, SIRCERR_IE: {=bool:?} }}" , self . SIRCSTEN () , self . SIRC_CLK_PERIPH_EN () , self . SIRCTREN () , self . SIRCTRUP () , self . TRIM_LOCK () , self . COARSE_TRIM_BYPASS () , self . LK () , self . SIRCVLD () , self . SIRCSEL () , self . SIRCERR () , self . SIRCERR_IE ())
        }
    }
    #[doc = "SIRC Auto-trimming Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCSTAT(pub u32);
    impl SIRCSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn CCOTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CCOTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLTRIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for SIRCSTAT {
        #[inline(always)]
        fn default() -> SIRCSTAT {
            SIRCSTAT(0)
        }
    }
    impl core::fmt::Debug for SIRCSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCSTAT")
                .field("CCOTRIM", &self.CCOTRIM())
                .field("CLTRIM", &self.CLTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SIRCSTAT {{ CCOTRIM: {=u8:?}, CLTRIM: {=u8:?} }}",
                self.CCOTRIM(),
                self.CLTRIM()
            )
        }
    }
    #[doc = "SIRC Trim Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCTCFG(pub u32);
    impl SIRCTCFG {
        #[must_use]
        #[inline(always)]
        pub const fn TRIMSRC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIMDIV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIMDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for SIRCTCFG {
        #[inline(always)]
        fn default() -> SIRCTCFG {
            SIRCTCFG(0)
        }
    }
    impl core::fmt::Debug for SIRCTCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCTCFG")
                .field("TRIMSRC", &self.TRIMSRC())
                .field("TRIMDIV", &self.TRIMDIV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCTCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SIRCTCFG {{ TRIMSRC: {=u8:?}, TRIMDIV: {=u8:?} }}",
                self.TRIMSRC(),
                self.TRIMDIV()
            )
        }
    }
    #[doc = "SIRC Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCTRIM(pub u32);
    impl SIRCTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn CCOTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CCOTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLTRIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCTRIM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FVCHTRIM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FVCHTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for SIRCTRIM {
        #[inline(always)]
        fn default() -> SIRCTRIM {
            SIRCTRIM(0)
        }
    }
    impl core::fmt::Debug for SIRCTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCTRIM")
                .field("CCOTRIM", &self.CCOTRIM())
                .field("CLTRIM", &self.CLTRIM())
                .field("TCTRIM", &self.TCTRIM())
                .field("FVCHTRIM", &self.FVCHTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SIRCTRIM {{ CCOTRIM: {=u8:?}, CLTRIM: {=u8:?}, TCTRIM: {=u8:?}, FVCHTRIM: {=u8:?} }}" , self . CCOTRIM () , self . CLTRIM () , self . TCTRIM () , self . FVCHTRIM ())
        }
    }
    #[doc = "SOSC Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SOSCCFG(pub u32);
    impl SOSCCFG {
        #[must_use]
        #[inline(always)]
        pub const fn EREFS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EREFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RANGE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RANGE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SOSCCFG {
        #[inline(always)]
        fn default() -> SOSCCFG {
            SOSCCFG(0)
        }
    }
    impl core::fmt::Debug for SOSCCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SOSCCFG")
                .field("EREFS", &self.EREFS())
                .field("RANGE", &self.RANGE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SOSCCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SOSCCFG {{ EREFS: {=bool:?}, RANGE: {=u8:?} }}",
                self.EREFS(),
                self.RANGE()
            )
        }
    }
    #[doc = "SOSC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SOSCCSR(pub u32);
    impl SOSCCSR {
        #[must_use]
        #[inline(always)]
        pub const fn SOSCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSCVLD_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSCVLD_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOSC_SAFE_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOSC_SAFE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SOSCCSR {
        #[inline(always)]
        fn default() -> SOSCCSR {
            SOSCCSR(0)
        }
    }
    impl core::fmt::Debug for SOSCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SOSCCSR")
                .field("SOSCEN", &self.SOSCEN())
                .field("SOSCSTEN", &self.SOSCSTEN())
                .field("SOSCCM", &self.SOSCCM())
                .field("SOSCCMRE", &self.SOSCCMRE())
                .field("LK", &self.LK())
                .field("SOSCVLD", &self.SOSCVLD())
                .field("SOSCSEL", &self.SOSCSEL())
                .field("SOSCERR", &self.SOSCERR())
                .field("SOSCVLD_IE", &self.SOSCVLD_IE())
                .field("SOSC_SAFE_EN", &self.SOSC_SAFE_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SOSCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SOSCCSR {{ SOSCEN: {=bool:?}, SOSCSTEN: {=bool:?}, SOSCCM: {=bool:?}, SOSCCMRE: {=bool:?}, LK: {=bool:?}, SOSCVLD: {=bool:?}, SOSCSEL: {=bool:?}, SOSCERR: {=bool:?}, SOSCVLD_IE: {=bool:?}, SOSC_SAFE_EN: {=bool:?} }}" , self . SOSCEN () , self . SOSCSTEN () , self . SOSCCM () , self . SOSCCMRE () , self . LK () , self . SOSCVLD () , self . SOSCSEL () , self . SOSCERR () , self . SOSCVLD_IE () , self . SOSC_SAFE_EN ())
        }
    }
    #[doc = "Trim Lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_LOCK(pub u32);
    impl TRIM_LOCK {
        #[must_use]
        #[inline(always)]
        pub const fn TRIM_UNLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRIM_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IFR_DISABLE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IFR_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRIM_LOCK_KEY(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TRIM_LOCK_KEY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for TRIM_LOCK {
        #[inline(always)]
        fn default() -> TRIM_LOCK {
            TRIM_LOCK(0)
        }
    }
    impl core::fmt::Debug for TRIM_LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_LOCK")
                .field("TRIM_UNLOCK", &self.TRIM_UNLOCK())
                .field("IFR_DISABLE", &self.IFR_DISABLE())
                .field("TRIM_LOCK_KEY", &self.TRIM_LOCK_KEY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_LOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TRIM_LOCK {{ TRIM_UNLOCK: {=bool:?}, IFR_DISABLE: {=bool:?}, TRIM_LOCK_KEY: {=u16:?} }}" , self . TRIM_UNLOCK () , self . IFR_DISABLE () , self . TRIM_LOCK_KEY ())
        }
    }
}
