#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWM {
    ptr: *mut u8,
}
unsafe impl Send for PWM {}
unsafe impl Sync for PWM {}
impl PWM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SM(self, n: usize) -> SM {
        assert!(n < 4usize);
        unsafe { SM::from_ptr(self.ptr.add(0x0usize + n * 96usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTEN(self) -> crate::common::Reg<regs::OUTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn MASK(self) -> crate::common::Reg<regs::MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0182usize) as _) }
    }
    #[inline(always)]
    pub const fn SWCOUT(self) -> crate::common::Reg<regs::SWCOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn DTSRCSEL(self) -> crate::common::Reg<regs::DTSRCSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0186usize) as _) }
    }
    #[inline(always)]
    pub const fn MCTRL(self) -> crate::common::Reg<regs::MCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn MCTRL2(self) -> crate::common::Reg<regs::MCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018ausize) as _) }
    }
    #[inline(always)]
    pub const fn FCTRL(self) -> crate::common::Reg<regs::FCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn FSTS(self) -> crate::common::Reg<regs::FSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018eusize) as _) }
    }
    #[inline(always)]
    pub const fn FFILT(self) -> crate::common::Reg<regs::FFILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn FTST(self) -> crate::common::Reg<regs::FTST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0192usize) as _) }
    }
    #[inline(always)]
    pub const fn FCTRL2(self) -> crate::common::Reg<regs::FCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SM {
    ptr: *mut u8,
}
unsafe impl Send for SM {}
unsafe impl Sync for SM {}
impl SM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CNT(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn INIT(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<regs::SM_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::SM_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn FRACVAL1(self) -> crate::common::Reg<regs::SM_FRACVAL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn VAL1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn FRACVAL2(self) -> crate::common::Reg<regs::SM_FRACVAL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL2(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn FRACVAL3(self) -> crate::common::Reg<regs::SM_FRACVAL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL3(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn FRACVAL4(self) -> crate::common::Reg<regs::SM_FRACVAL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL4(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[inline(always)]
    pub const fn FRACVAL5(self) -> crate::common::Reg<regs::SM_FRACVAL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn VAL5(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[inline(always)]
    pub const fn FRCTRL(self) -> crate::common::Reg<regs::SM_FRCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn OCTRL(self) -> crate::common::Reg<regs::SM_OCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[inline(always)]
    pub const fn STS(self) -> crate::common::Reg<regs::SM_STS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<regs::SM_INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[inline(always)]
    pub const fn DMAEN(self) -> crate::common::Reg<regs::SM_DMAEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn TCTRL(self) -> crate::common::Reg<regs::SM_TCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[inline(always)]
    pub const fn DISMAP(self, n: usize) -> crate::common::Reg<u16, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize + n * 2usize) as _) }
    }
    #[inline(always)]
    pub const fn DTCNT0(self) -> crate::common::Reg<regs::SM_DTCNT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn DTCNT1(self) -> crate::common::Reg<regs::SM_DTCNT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCTRLA(self) -> crate::common::Reg<regs::SM_CAPTCTRLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCOMPA(self) -> crate::common::Reg<regs::SM_CAPTCOMPA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCTRLB(self) -> crate::common::Reg<regs::SM_CAPTCTRLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCOMPB(self) -> crate::common::Reg<regs::SM_CAPTCOMPB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3ausize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCTRLX(self) -> crate::common::Reg<regs::SM_CAPTCTRLX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCOMPX(self) -> crate::common::Reg<regs::SM_CAPTCOMPX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL0(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL0CYC(self) -> crate::common::Reg<regs::SM_CVAL0CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL1(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL1CYC(self) -> crate::common::Reg<regs::SM_CVAL1CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x46usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL2(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL2CYC(self) -> crate::common::Reg<regs::SM_CVAL2CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4ausize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL3(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL3CYC(self) -> crate::common::Reg<regs::SM_CVAL3CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4eusize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL4(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL4CYC(self) -> crate::common::Reg<regs::SM_CVAL4CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x52usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL5(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL5CYC(self) -> crate::common::Reg<regs::SM_CVAL5CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x56usize) as _) }
    }
    #[inline(always)]
    pub const fn PHASEDLY(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTFILTA(self) -> crate::common::Reg<regs::SM_CAPTFILTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5ausize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTFILTB(self) -> crate::common::Reg<regs::SM_CAPTFILTB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTFILTX(self) -> crate::common::Reg<regs::SM_CAPTFILTX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5eusize) as _) }
    }
}
pub mod regs {
    #[doc = "PWM Source Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DTSRCSEL(pub u16);
    impl DTSRCSEL {
        #[must_use]
        #[inline(always)]
        pub const fn SM0SEL45(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM0SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM0SEL23(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM0SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM1SEL45(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM1SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM1SEL23(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM1SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM2SEL45(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM2SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM2SEL23(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM2SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM3SEL45(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM3SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM3SEL23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM3SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for DTSRCSEL {
        #[inline(always)]
        fn default() -> DTSRCSEL {
            DTSRCSEL(0)
        }
    }
    impl core::fmt::Debug for DTSRCSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DTSRCSEL")
                .field("SM0SEL45", &self.SM0SEL45())
                .field("SM0SEL23", &self.SM0SEL23())
                .field("SM1SEL45", &self.SM1SEL45())
                .field("SM1SEL23", &self.SM1SEL23())
                .field("SM2SEL45", &self.SM2SEL45())
                .field("SM2SEL23", &self.SM2SEL23())
                .field("SM3SEL45", &self.SM3SEL45())
                .field("SM3SEL23", &self.SM3SEL23())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DTSRCSEL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DTSRCSEL {{ SM0SEL45: {=u8:?}, SM0SEL23: {=u8:?}, SM1SEL45: {=u8:?}, SM1SEL23: {=u8:?}, SM2SEL45: {=u8:?}, SM2SEL23: {=u8:?}, SM3SEL45: {=u8:?}, SM3SEL23: {=u8:?} }}" , self . SM0SEL45 () , self . SM0SEL23 () , self . SM1SEL45 () , self . SM1SEL23 () , self . SM2SEL45 () , self . SM2SEL23 () , self . SM3SEL45 () , self . SM3SEL23 ())
        }
    }
    #[doc = "Fault Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u16);
    impl FCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FSAFE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FSAFE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FAUTO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FAUTO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLVL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for FCTRL {
        #[inline(always)]
        fn default() -> FCTRL {
            FCTRL(0)
        }
    }
    impl core::fmt::Debug for FCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTRL")
                .field("FIE", &self.FIE())
                .field("FSAFE", &self.FSAFE())
                .field("FAUTO", &self.FAUTO())
                .field("FLVL", &self.FLVL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FCTRL {{ FIE: {=u8:?}, FSAFE: {=u8:?}, FAUTO: {=u8:?}, FLVL: {=u8:?} }}",
                self.FIE(),
                self.FSAFE(),
                self.FAUTO(),
                self.FLVL()
            )
        }
    }
    #[doc = "Fault Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL2(pub u16);
    impl FCTRL2 {
        #[must_use]
        #[inline(always)]
        pub const fn NOCOMB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NOCOMB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for FCTRL2 {
        #[inline(always)]
        fn default() -> FCTRL2 {
            FCTRL2(0)
        }
    }
    impl core::fmt::Debug for FCTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTRL2")
                .field("NOCOMB", &self.NOCOMB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTRL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FCTRL2 {{ NOCOMB: {=u8:?} }}", self.NOCOMB())
        }
    }
    #[doc = "Fault Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FFILT(pub u16);
    impl FFILT {
        #[must_use]
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GSTR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GSTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for FFILT {
        #[inline(always)]
        fn default() -> FFILT {
            FFILT(0)
        }
    }
    impl core::fmt::Debug for FFILT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FFILT")
                .field("FILT_PER", &self.FILT_PER())
                .field("FILT_CNT", &self.FILT_CNT())
                .field("GSTR", &self.GSTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FFILT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FFILT {{ FILT_PER: {=u8:?}, FILT_CNT: {=u8:?}, GSTR: {=bool:?} }}",
                self.FILT_PER(),
                self.FILT_CNT(),
                self.GSTR()
            )
        }
    }
    #[doc = "Fault Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FSTS(pub u16);
    impl FSTS {
        #[must_use]
        #[inline(always)]
        pub const fn FFLAG(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FFLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FFULL(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FFULL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FFPIN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FFPIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FHALF(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FHALF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for FSTS {
        #[inline(always)]
        fn default() -> FSTS {
            FSTS(0)
        }
    }
    impl core::fmt::Debug for FSTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FSTS")
                .field("FFLAG", &self.FFLAG())
                .field("FFULL", &self.FFULL())
                .field("FFPIN", &self.FFPIN())
                .field("FHALF", &self.FHALF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FSTS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FSTS {{ FFLAG: {=u8:?}, FFULL: {=u8:?}, FFPIN: {=u8:?}, FHALF: {=u8:?} }}",
                self.FFLAG(),
                self.FFULL(),
                self.FFPIN(),
                self.FHALF()
            )
        }
    }
    #[doc = "Fault Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FTST(pub u16);
    impl FTST {
        #[must_use]
        #[inline(always)]
        pub const fn FTEST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FTEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
    }
    impl Default for FTST {
        #[inline(always)]
        fn default() -> FTST {
            FTST(0)
        }
    }
    impl core::fmt::Debug for FTST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FTST")
                .field("FTEST", &self.FTEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FTST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FTST {{ FTEST: {=bool:?} }}", self.FTEST())
        }
    }
    #[doc = "Mask Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASK(pub u16);
    impl MASK {
        #[must_use]
        #[inline(always)]
        pub const fn MASKX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASKX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASKB(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASKB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASKA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASKA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UPDATE_MASK(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_UPDATE_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for MASK {
        #[inline(always)]
        fn default() -> MASK {
            MASK(0)
        }
    }
    impl core::fmt::Debug for MASK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MASK")
                .field("MASKX", &self.MASKX())
                .field("MASKB", &self.MASKB())
                .field("MASKA", &self.MASKA())
                .field("UPDATE_MASK", &self.UPDATE_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MASK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MASK {{ MASKX: {=u8:?}, MASKB: {=u8:?}, MASKA: {=u8:?}, UPDATE_MASK: {=u8:?} }}",
                self.MASKX(),
                self.MASKB(),
                self.MASKA(),
                self.UPDATE_MASK()
            )
        }
    }
    #[doc = "Master Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTRL(pub u16);
    impl MCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn LDOK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LDOK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLDOK(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLDOK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RUN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RUN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPOL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPOL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for MCTRL {
        #[inline(always)]
        fn default() -> MCTRL {
            MCTRL(0)
        }
    }
    impl core::fmt::Debug for MCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCTRL")
                .field("LDOK", &self.LDOK())
                .field("CLDOK", &self.CLDOK())
                .field("RUN", &self.RUN())
                .field("IPOL", &self.IPOL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MCTRL {{ LDOK: {=u8:?}, CLDOK: {=u8:?}, RUN: {=u8:?}, IPOL: {=u8:?} }}",
                self.LDOK(),
                self.CLDOK(),
                self.RUN(),
                self.IPOL()
            )
        }
    }
    #[doc = "Master Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTRL2(pub u16);
    impl MCTRL2 {
        #[must_use]
        #[inline(always)]
        pub const fn WRPROT(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WRPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STRETCH_CNT_PRSC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_STRETCH_CNT_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
    }
    impl Default for MCTRL2 {
        #[inline(always)]
        fn default() -> MCTRL2 {
            MCTRL2(0)
        }
    }
    impl core::fmt::Debug for MCTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCTRL2")
                .field("WRPROT", &self.WRPROT())
                .field("STRETCH_CNT_PRSC", &self.STRETCH_CNT_PRSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCTRL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MCTRL2 {{ WRPROT: {=u8:?}, STRETCH_CNT_PRSC: {=u8:?} }}",
                self.WRPROT(),
                self.STRETCH_CNT_PRSC()
            )
        }
    }
    #[doc = "Output Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTEN(pub u16);
    impl OUTEN {
        #[must_use]
        #[inline(always)]
        pub const fn PWMX_EN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMX_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMB_EN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMB_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMA_EN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMA_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
    }
    impl Default for OUTEN {
        #[inline(always)]
        fn default() -> OUTEN {
            OUTEN(0)
        }
    }
    impl core::fmt::Debug for OUTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTEN")
                .field("PWMX_EN", &self.PWMX_EN())
                .field("PWMB_EN", &self.PWMB_EN())
                .field("PWMA_EN", &self.PWMA_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OUTEN {{ PWMX_EN: {=u8:?}, PWMB_EN: {=u8:?}, PWMA_EN: {=u8:?} }}",
                self.PWMX_EN(),
                self.PWMB_EN(),
                self.PWMA_EN()
            )
        }
    }
    #[doc = "Capture Compare A Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCOMPA(pub u16);
    impl SM_CAPTCOMPA {
        #[must_use]
        #[inline(always)]
        pub const fn EDGCMPA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCMPA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCNTA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for SM_CAPTCOMPA {
        #[inline(always)]
        fn default() -> SM_CAPTCOMPA {
            SM_CAPTCOMPA(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCOMPA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCOMPA")
                .field("EDGCMPA", &self.EDGCMPA())
                .field("EDGCNTA", &self.EDGCNTA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCOMPA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTCOMPA {{ EDGCMPA: {=u8:?}, EDGCNTA: {=u8:?} }}",
                self.EDGCMPA(),
                self.EDGCNTA()
            )
        }
    }
    #[doc = "Capture Compare B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCOMPB(pub u16);
    impl SM_CAPTCOMPB {
        #[must_use]
        #[inline(always)]
        pub const fn EDGCMPB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCMPB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTB(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCNTB(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for SM_CAPTCOMPB {
        #[inline(always)]
        fn default() -> SM_CAPTCOMPB {
            SM_CAPTCOMPB(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCOMPB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCOMPB")
                .field("EDGCMPB", &self.EDGCMPB())
                .field("EDGCNTB", &self.EDGCNTB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCOMPB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTCOMPB {{ EDGCMPB: {=u8:?}, EDGCNTB: {=u8:?} }}",
                self.EDGCMPB(),
                self.EDGCNTB()
            )
        }
    }
    #[doc = "Capture Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCOMPX(pub u16);
    impl SM_CAPTCOMPX {
        #[must_use]
        #[inline(always)]
        pub const fn EDGCMPX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCMPX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGCNTX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for SM_CAPTCOMPX {
        #[inline(always)]
        fn default() -> SM_CAPTCOMPX {
            SM_CAPTCOMPX(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCOMPX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCOMPX")
                .field("EDGCMPX", &self.EDGCMPX())
                .field("EDGCNTX", &self.EDGCNTX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCOMPX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTCOMPX {{ EDGCMPX: {=u8:?}, EDGCNTX: {=u8:?} }}",
                self.EDGCMPX(),
                self.EDGCNTX()
            )
        }
    }
    #[doc = "Capture Control A Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCTRLA(pub u16);
    impl SM_CAPTCTRLA {
        #[must_use]
        #[inline(always)]
        pub const fn ARMA(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ARMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONESHOTA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONESHOTA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGA0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGA1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INP_SELA(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INP_SELA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTA_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EDGCNTA_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFAWM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFAWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA0CNT(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CA0CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA1CNT(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CA1CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for SM_CAPTCTRLA {
        #[inline(always)]
        fn default() -> SM_CAPTCTRLA {
            SM_CAPTCTRLA(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCTRLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCTRLA")
                .field("ARMA", &self.ARMA())
                .field("ONESHOTA", &self.ONESHOTA())
                .field("EDGA0", &self.EDGA0())
                .field("EDGA1", &self.EDGA1())
                .field("INP_SELA", &self.INP_SELA())
                .field("EDGCNTA_EN", &self.EDGCNTA_EN())
                .field("CFAWM", &self.CFAWM())
                .field("CA0CNT", &self.CA0CNT())
                .field("CA1CNT", &self.CA1CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCTRLA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_CAPTCTRLA {{ ARMA: {=bool:?}, ONESHOTA: {=bool:?}, EDGA0: {=u8:?}, EDGA1: {=u8:?}, INP_SELA: {=bool:?}, EDGCNTA_EN: {=bool:?}, CFAWM: {=u8:?}, CA0CNT: {=u8:?}, CA1CNT: {=u8:?} }}" , self . ARMA () , self . ONESHOTA () , self . EDGA0 () , self . EDGA1 () , self . INP_SELA () , self . EDGCNTA_EN () , self . CFAWM () , self . CA0CNT () , self . CA1CNT ())
        }
    }
    #[doc = "Capture Control B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCTRLB(pub u16);
    impl SM_CAPTCTRLB {
        #[must_use]
        #[inline(always)]
        pub const fn ARMB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ARMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONESHOTB(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONESHOTB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGB0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGB0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGB1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGB1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INP_SELB(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INP_SELB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTB_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EDGCNTB_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFBWM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFBWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB0CNT(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CB0CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB1CNT(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CB1CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for SM_CAPTCTRLB {
        #[inline(always)]
        fn default() -> SM_CAPTCTRLB {
            SM_CAPTCTRLB(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCTRLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCTRLB")
                .field("ARMB", &self.ARMB())
                .field("ONESHOTB", &self.ONESHOTB())
                .field("EDGB0", &self.EDGB0())
                .field("EDGB1", &self.EDGB1())
                .field("INP_SELB", &self.INP_SELB())
                .field("EDGCNTB_EN", &self.EDGCNTB_EN())
                .field("CFBWM", &self.CFBWM())
                .field("CB0CNT", &self.CB0CNT())
                .field("CB1CNT", &self.CB1CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCTRLB {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_CAPTCTRLB {{ ARMB: {=bool:?}, ONESHOTB: {=bool:?}, EDGB0: {=u8:?}, EDGB1: {=u8:?}, INP_SELB: {=bool:?}, EDGCNTB_EN: {=bool:?}, CFBWM: {=u8:?}, CB0CNT: {=u8:?}, CB1CNT: {=u8:?} }}" , self . ARMB () , self . ONESHOTB () , self . EDGB0 () , self . EDGB1 () , self . INP_SELB () , self . EDGCNTB_EN () , self . CFBWM () , self . CB0CNT () , self . CB1CNT ())
        }
    }
    #[doc = "Capture Control X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCTRLX(pub u16);
    impl SM_CAPTCTRLX {
        #[must_use]
        #[inline(always)]
        pub const fn ARMX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ARMX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONESHOTX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONESHOTX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGX0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGX0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGX1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EDGX1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INP_SELX(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INP_SELX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDGCNTX_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EDGCNTX_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFXWM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFXWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CX0CNT(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CX0CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CX1CNT(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CX1CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for SM_CAPTCTRLX {
        #[inline(always)]
        fn default() -> SM_CAPTCTRLX {
            SM_CAPTCTRLX(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTCTRLX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTCTRLX")
                .field("ARMX", &self.ARMX())
                .field("ONESHOTX", &self.ONESHOTX())
                .field("EDGX0", &self.EDGX0())
                .field("EDGX1", &self.EDGX1())
                .field("INP_SELX", &self.INP_SELX())
                .field("EDGCNTX_EN", &self.EDGCNTX_EN())
                .field("CFXWM", &self.CFXWM())
                .field("CX0CNT", &self.CX0CNT())
                .field("CX1CNT", &self.CX1CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTCTRLX {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_CAPTCTRLX {{ ARMX: {=bool:?}, ONESHOTX: {=bool:?}, EDGX0: {=u8:?}, EDGX1: {=u8:?}, INP_SELX: {=bool:?}, EDGCNTX_EN: {=bool:?}, CFXWM: {=u8:?}, CX0CNT: {=u8:?}, CX1CNT: {=u8:?} }}" , self . ARMX () , self . ONESHOTX () , self . EDGX0 () , self . EDGX1 () , self . INP_SELX () , self . EDGCNTX_EN () , self . CFXWM () , self . CX0CNT () , self . CX1CNT ())
        }
    }
    #[doc = "Capture PWM_A Input Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTFILTA(pub u16);
    impl SM_CAPTFILTA {
        #[must_use]
        #[inline(always)]
        pub const fn CAPTA_FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTA_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAPTA_FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTA_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for SM_CAPTFILTA {
        #[inline(always)]
        fn default() -> SM_CAPTFILTA {
            SM_CAPTFILTA(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTFILTA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTFILTA")
                .field("CAPTA_FILT_PER", &self.CAPTA_FILT_PER())
                .field("CAPTA_FILT_CNT", &self.CAPTA_FILT_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTFILTA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTFILTA {{ CAPTA_FILT_PER: {=u8:?}, CAPTA_FILT_CNT: {=u8:?} }}",
                self.CAPTA_FILT_PER(),
                self.CAPTA_FILT_CNT()
            )
        }
    }
    #[doc = "Capture PWM_B Input Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTFILTB(pub u16);
    impl SM_CAPTFILTB {
        #[must_use]
        #[inline(always)]
        pub const fn CAPTB_FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTB_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAPTB_FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTB_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for SM_CAPTFILTB {
        #[inline(always)]
        fn default() -> SM_CAPTFILTB {
            SM_CAPTFILTB(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTFILTB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTFILTB")
                .field("CAPTB_FILT_PER", &self.CAPTB_FILT_PER())
                .field("CAPTB_FILT_CNT", &self.CAPTB_FILT_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTFILTB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTFILTB {{ CAPTB_FILT_PER: {=u8:?}, CAPTB_FILT_CNT: {=u8:?} }}",
                self.CAPTB_FILT_PER(),
                self.CAPTB_FILT_CNT()
            )
        }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTFILTX(pub u16);
    impl SM_CAPTFILTX {
        #[must_use]
        #[inline(always)]
        pub const fn CAPTX_FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTX_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAPTX_FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTX_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for SM_CAPTFILTX {
        #[inline(always)]
        fn default() -> SM_CAPTFILTX {
            SM_CAPTFILTX(0)
        }
    }
    impl core::fmt::Debug for SM_CAPTFILTX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CAPTFILTX")
                .field("CAPTX_FILT_PER", &self.CAPTX_FILT_PER())
                .field("CAPTX_FILT_CNT", &self.CAPTX_FILT_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CAPTFILTX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_CAPTFILTX {{ CAPTX_FILT_PER: {=u8:?}, CAPTX_FILT_CNT: {=u8:?} }}",
                self.CAPTX_FILT_PER(),
                self.CAPTX_FILT_CNT()
            )
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CTRL(pub u16);
    impl SM_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DBLEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBLEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBLX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBLX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LDMOD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LDMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPLIT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPLIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRSC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COMPMODE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COMPMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FULL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HALF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HALF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LDFQ(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LDFQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for SM_CTRL {
        #[inline(always)]
        fn default() -> SM_CTRL {
            SM_CTRL(0)
        }
    }
    impl core::fmt::Debug for SM_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CTRL")
                .field("DBLEN", &self.DBLEN())
                .field("DBLX", &self.DBLX())
                .field("LDMOD", &self.LDMOD())
                .field("SPLIT", &self.SPLIT())
                .field("PRSC", &self.PRSC())
                .field("COMPMODE", &self.COMPMODE())
                .field("DT", &self.DT())
                .field("FULL", &self.FULL())
                .field("HALF", &self.HALF())
                .field("LDFQ", &self.LDFQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_CTRL {{ DBLEN: {=bool:?}, DBLX: {=bool:?}, LDMOD: {=bool:?}, SPLIT: {=bool:?}, PRSC: {=u8:?}, COMPMODE: {=bool:?}, DT: {=u8:?}, FULL: {=bool:?}, HALF: {=bool:?}, LDFQ: {=u8:?} }}" , self . DBLEN () , self . DBLX () , self . LDMOD () , self . SPLIT () , self . PRSC () , self . COMPMODE () , self . DT () , self . FULL () , self . HALF () , self . LDFQ ())
        }
    }
    #[doc = "Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CTRL2(pub u16);
    impl SM_CTRL2 {
        #[must_use]
        #[inline(always)]
        pub const fn CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RELOAD_SEL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RELOAD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FORCE_SEL(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FORCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u16) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FORCE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FORCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRCEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INIT_SEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INIT_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMX_INIT(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMX_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWM45_INIT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWM45_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWM23_INIT(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWM23_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INDEP(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INDEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBGEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for SM_CTRL2 {
        #[inline(always)]
        fn default() -> SM_CTRL2 {
            SM_CTRL2(0)
        }
    }
    impl core::fmt::Debug for SM_CTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CTRL2")
                .field("CLK_SEL", &self.CLK_SEL())
                .field("RELOAD_SEL", &self.RELOAD_SEL())
                .field("FORCE_SEL", &self.FORCE_SEL())
                .field("FORCE", &self.FORCE())
                .field("FRCEN", &self.FRCEN())
                .field("INIT_SEL", &self.INIT_SEL())
                .field("PWMX_INIT", &self.PWMX_INIT())
                .field("PWM45_INIT", &self.PWM45_INIT())
                .field("PWM23_INIT", &self.PWM23_INIT())
                .field("INDEP", &self.INDEP())
                .field("DBGEN", &self.DBGEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CTRL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_CTRL2 {{ CLK_SEL: {=u8:?}, RELOAD_SEL: {=bool:?}, FORCE_SEL: {=u8:?}, FORCE: {=bool:?}, FRCEN: {=bool:?}, INIT_SEL: {=u8:?}, PWMX_INIT: {=bool:?}, PWM45_INIT: {=bool:?}, PWM23_INIT: {=bool:?}, INDEP: {=bool:?}, DBGEN: {=bool:?} }}" , self . CLK_SEL () , self . RELOAD_SEL () , self . FORCE_SEL () , self . FORCE () , self . FRCEN () , self . INIT_SEL () , self . PWMX_INIT () , self . PWM45_INIT () , self . PWM23_INIT () , self . INDEP () , self . DBGEN ())
        }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL0CYC(pub u16);
    impl SM_CVAL0CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL0CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL0CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL0CYC {
        #[inline(always)]
        fn default() -> SM_CVAL0CYC {
            SM_CVAL0CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL0CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL0CYC")
                .field("CVAL0CYC", &self.CVAL0CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL0CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL0CYC {{ CVAL0CYC: {=u8:?} }}", self.CVAL0CYC())
        }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL1CYC(pub u16);
    impl SM_CVAL1CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL1CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL1CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL1CYC {
        #[inline(always)]
        fn default() -> SM_CVAL1CYC {
            SM_CVAL1CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL1CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL1CYC")
                .field("CVAL1CYC", &self.CVAL1CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL1CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL1CYC {{ CVAL1CYC: {=u8:?} }}", self.CVAL1CYC())
        }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL2CYC(pub u16);
    impl SM_CVAL2CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL2CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL2CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL2CYC {
        #[inline(always)]
        fn default() -> SM_CVAL2CYC {
            SM_CVAL2CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL2CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL2CYC")
                .field("CVAL2CYC", &self.CVAL2CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL2CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL2CYC {{ CVAL2CYC: {=u8:?} }}", self.CVAL2CYC())
        }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL3CYC(pub u16);
    impl SM_CVAL3CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL3CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL3CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL3CYC {
        #[inline(always)]
        fn default() -> SM_CVAL3CYC {
            SM_CVAL3CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL3CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL3CYC")
                .field("CVAL3CYC", &self.CVAL3CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL3CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL3CYC {{ CVAL3CYC: {=u8:?} }}", self.CVAL3CYC())
        }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL4CYC(pub u16);
    impl SM_CVAL4CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL4CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL4CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL4CYC {
        #[inline(always)]
        fn default() -> SM_CVAL4CYC {
            SM_CVAL4CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL4CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL4CYC")
                .field("CVAL4CYC", &self.CVAL4CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL4CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL4CYC {{ CVAL4CYC: {=u8:?} }}", self.CVAL4CYC())
        }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL5CYC(pub u16);
    impl SM_CVAL5CYC {
        #[must_use]
        #[inline(always)]
        pub const fn CVAL5CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CVAL5CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
    }
    impl Default for SM_CVAL5CYC {
        #[inline(always)]
        fn default() -> SM_CVAL5CYC {
            SM_CVAL5CYC(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL5CYC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL5CYC")
                .field("CVAL5CYC", &self.CVAL5CYC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL5CYC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_CVAL5CYC {{ CVAL5CYC: {=u8:?} }}", self.CVAL5CYC())
        }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DISMAP(pub u16);
    impl SM_DISMAP {
        #[must_use]
        #[inline(always)]
        pub const fn DIS0A(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DIS0A(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS0B(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DIS0B(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS0X(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DIS0X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
    }
    impl Default for SM_DISMAP {
        #[inline(always)]
        fn default() -> SM_DISMAP {
            SM_DISMAP(0)
        }
    }
    impl core::fmt::Debug for SM_DISMAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_DISMAP")
                .field("DIS0A", &self.DIS0A())
                .field("DIS0B", &self.DIS0B())
                .field("DIS0X", &self.DIS0X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_DISMAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SM_DISMAP {{ DIS0A: {=u8:?}, DIS0B: {=u8:?}, DIS0X: {=u8:?} }}",
                self.DIS0A(),
                self.DIS0B(),
                self.DIS0X()
            )
        }
    }
    #[doc = "DMA Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DMAEN(pub u16);
    impl SM_DMAEN {
        #[must_use]
        #[inline(always)]
        pub const fn CX0DE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CX0DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CX1DE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CX1DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB0DE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CB0DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB1DE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CB1DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA0DE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CA0DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA1DE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CA1DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAPTDE(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAPTDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FAND(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FAND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VALDE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VALDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for SM_DMAEN {
        #[inline(always)]
        fn default() -> SM_DMAEN {
            SM_DMAEN(0)
        }
    }
    impl core::fmt::Debug for SM_DMAEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_DMAEN")
                .field("CX0DE", &self.CX0DE())
                .field("CX1DE", &self.CX1DE())
                .field("CB0DE", &self.CB0DE())
                .field("CB1DE", &self.CB1DE())
                .field("CA0DE", &self.CA0DE())
                .field("CA1DE", &self.CA1DE())
                .field("CAPTDE", &self.CAPTDE())
                .field("FAND", &self.FAND())
                .field("VALDE", &self.VALDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_DMAEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_DMAEN {{ CX0DE: {=bool:?}, CX1DE: {=bool:?}, CB0DE: {=bool:?}, CB1DE: {=bool:?}, CA0DE: {=bool:?}, CA1DE: {=bool:?}, CAPTDE: {=u8:?}, FAND: {=bool:?}, VALDE: {=bool:?} }}" , self . CX0DE () , self . CX1DE () , self . CB0DE () , self . CB1DE () , self . CA0DE () , self . CA1DE () , self . CAPTDE () , self . FAND () , self . VALDE ())
        }
    }
    #[doc = "Deadtime Count Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DTCNT0(pub u16);
    impl SM_DTCNT0 {
        #[must_use]
        #[inline(always)]
        pub const fn DTCNT0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DTCNT0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for SM_DTCNT0 {
        #[inline(always)]
        fn default() -> SM_DTCNT0 {
            SM_DTCNT0(0)
        }
    }
    impl core::fmt::Debug for SM_DTCNT0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_DTCNT0")
                .field("DTCNT0", &self.DTCNT0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_DTCNT0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_DTCNT0 {{ DTCNT0: {=u16:?} }}", self.DTCNT0())
        }
    }
    #[doc = "Deadtime Count Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DTCNT1(pub u16);
    impl SM_DTCNT1 {
        #[must_use]
        #[inline(always)]
        pub const fn DTCNT1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DTCNT1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
    }
    impl Default for SM_DTCNT1 {
        #[inline(always)]
        fn default() -> SM_DTCNT1 {
            SM_DTCNT1(0)
        }
    }
    impl core::fmt::Debug for SM_DTCNT1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_DTCNT1")
                .field("DTCNT1", &self.DTCNT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_DTCNT1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_DTCNT1 {{ DTCNT1: {=u16:?} }}", self.DTCNT1())
        }
    }
    #[doc = "Fractional Value Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRACVAL1(pub u16);
    impl SM_FRACVAL1 {
        #[must_use]
        #[inline(always)]
        pub const fn FRACVAL1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRACVAL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
        }
    }
    impl Default for SM_FRACVAL1 {
        #[inline(always)]
        fn default() -> SM_FRACVAL1 {
            SM_FRACVAL1(0)
        }
    }
    impl core::fmt::Debug for SM_FRACVAL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRACVAL1")
                .field("FRACVAL1", &self.FRACVAL1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRACVAL1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_FRACVAL1 {{ FRACVAL1: {=u8:?} }}", self.FRACVAL1())
        }
    }
    #[doc = "Fractional Value Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRACVAL2(pub u16);
    impl SM_FRACVAL2 {
        #[must_use]
        #[inline(always)]
        pub const fn FRACVAL2(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRACVAL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
        }
    }
    impl Default for SM_FRACVAL2 {
        #[inline(always)]
        fn default() -> SM_FRACVAL2 {
            SM_FRACVAL2(0)
        }
    }
    impl core::fmt::Debug for SM_FRACVAL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRACVAL2")
                .field("FRACVAL2", &self.FRACVAL2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRACVAL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_FRACVAL2 {{ FRACVAL2: {=u8:?} }}", self.FRACVAL2())
        }
    }
    #[doc = "Fractional Value Register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRACVAL3(pub u16);
    impl SM_FRACVAL3 {
        #[must_use]
        #[inline(always)]
        pub const fn FRACVAL3(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRACVAL3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
        }
    }
    impl Default for SM_FRACVAL3 {
        #[inline(always)]
        fn default() -> SM_FRACVAL3 {
            SM_FRACVAL3(0)
        }
    }
    impl core::fmt::Debug for SM_FRACVAL3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRACVAL3")
                .field("FRACVAL3", &self.FRACVAL3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRACVAL3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_FRACVAL3 {{ FRACVAL3: {=u8:?} }}", self.FRACVAL3())
        }
    }
    #[doc = "Fractional Value Register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRACVAL4(pub u16);
    impl SM_FRACVAL4 {
        #[must_use]
        #[inline(always)]
        pub const fn FRACVAL4(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRACVAL4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
        }
    }
    impl Default for SM_FRACVAL4 {
        #[inline(always)]
        fn default() -> SM_FRACVAL4 {
            SM_FRACVAL4(0)
        }
    }
    impl core::fmt::Debug for SM_FRACVAL4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRACVAL4")
                .field("FRACVAL4", &self.FRACVAL4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRACVAL4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_FRACVAL4 {{ FRACVAL4: {=u8:?} }}", self.FRACVAL4())
        }
    }
    #[doc = "Fractional Value Register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRACVAL5(pub u16);
    impl SM_FRACVAL5 {
        #[must_use]
        #[inline(always)]
        pub const fn FRACVAL5(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRACVAL5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
        }
    }
    impl Default for SM_FRACVAL5 {
        #[inline(always)]
        fn default() -> SM_FRACVAL5 {
            SM_FRACVAL5(0)
        }
    }
    impl core::fmt::Debug for SM_FRACVAL5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRACVAL5")
                .field("FRACVAL5", &self.FRACVAL5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRACVAL5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SM_FRACVAL5 {{ FRACVAL5: {=u8:?} }}", self.FRACVAL5())
        }
    }
    #[doc = "Fractional Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_FRCTRL(pub u16);
    impl SM_FRCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FRAC1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRAC1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRAC23_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRAC23_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRAC45_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRAC45_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TEST(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for SM_FRCTRL {
        #[inline(always)]
        fn default() -> SM_FRCTRL {
            SM_FRCTRL(0)
        }
    }
    impl core::fmt::Debug for SM_FRCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_FRCTRL")
                .field("FRAC1_EN", &self.FRAC1_EN())
                .field("FRAC23_EN", &self.FRAC23_EN())
                .field("FRAC45_EN", &self.FRAC45_EN())
                .field("TEST", &self.TEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_FRCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_FRCTRL {{ FRAC1_EN: {=bool:?}, FRAC23_EN: {=bool:?}, FRAC45_EN: {=bool:?}, TEST: {=bool:?} }}" , self . FRAC1_EN () , self . FRAC23_EN () , self . FRAC45_EN () , self . TEST ())
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_INTEN(pub u16);
    impl SM_INTEN {
        #[must_use]
        #[inline(always)]
        pub const fn CMPIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMPIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CX0IE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CX0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CX1IE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CX1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB0IE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CB0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CB1IE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CB1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA0IE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CA0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CA1IE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CA1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REIE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for SM_INTEN {
        #[inline(always)]
        fn default() -> SM_INTEN {
            SM_INTEN(0)
        }
    }
    impl core::fmt::Debug for SM_INTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_INTEN")
                .field("CMPIE", &self.CMPIE())
                .field("CX0IE", &self.CX0IE())
                .field("CX1IE", &self.CX1IE())
                .field("CB0IE", &self.CB0IE())
                .field("CB1IE", &self.CB1IE())
                .field("CA0IE", &self.CA0IE())
                .field("CA1IE", &self.CA1IE())
                .field("RIE", &self.RIE())
                .field("REIE", &self.REIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_INTEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_INTEN {{ CMPIE: {=u8:?}, CX0IE: {=bool:?}, CX1IE: {=bool:?}, CB0IE: {=bool:?}, CB1IE: {=bool:?}, CA0IE: {=bool:?}, CA1IE: {=bool:?}, RIE: {=bool:?}, REIE: {=bool:?} }}" , self . CMPIE () , self . CX0IE () , self . CX1IE () , self . CB0IE () , self . CB1IE () , self . CA0IE () , self . CA1IE () , self . RIE () , self . REIE ())
        }
    }
    #[doc = "Output Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_OCTRL(pub u16);
    impl SM_OCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn PWMXFS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMXFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMBFS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMBFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMAFS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWMAFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POLX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POLX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POLB(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POLB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POLA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POLA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMX_IN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMX_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMB_IN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMB_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWMA_IN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWMA_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for SM_OCTRL {
        #[inline(always)]
        fn default() -> SM_OCTRL {
            SM_OCTRL(0)
        }
    }
    impl core::fmt::Debug for SM_OCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_OCTRL")
                .field("PWMXFS", &self.PWMXFS())
                .field("PWMBFS", &self.PWMBFS())
                .field("PWMAFS", &self.PWMAFS())
                .field("POLX", &self.POLX())
                .field("POLB", &self.POLB())
                .field("POLA", &self.POLA())
                .field("PWMX_IN", &self.PWMX_IN())
                .field("PWMB_IN", &self.PWMB_IN())
                .field("PWMA_IN", &self.PWMA_IN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_OCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_OCTRL {{ PWMXFS: {=u8:?}, PWMBFS: {=u8:?}, PWMAFS: {=u8:?}, POLX: {=bool:?}, POLB: {=bool:?}, POLA: {=bool:?}, PWMX_IN: {=bool:?}, PWMB_IN: {=bool:?}, PWMA_IN: {=bool:?} }}" , self . PWMXFS () , self . PWMBFS () , self . PWMAFS () , self . POLX () , self . POLB () , self . POLA () , self . PWMX_IN () , self . PWMB_IN () , self . PWMA_IN ())
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_STS(pub u16);
    impl SM_STS {
        #[must_use]
        #[inline(always)]
        pub const fn CMPF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMPF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFX0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFX0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFX1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFX1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFB0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFB0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFB1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFB1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFA0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFA1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFA1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REF(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RUF(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for SM_STS {
        #[inline(always)]
        fn default() -> SM_STS {
            SM_STS(0)
        }
    }
    impl core::fmt::Debug for SM_STS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_STS")
                .field("CMPF", &self.CMPF())
                .field("CFX0", &self.CFX0())
                .field("CFX1", &self.CFX1())
                .field("CFB0", &self.CFB0())
                .field("CFB1", &self.CFB1())
                .field("CFA0", &self.CFA0())
                .field("CFA1", &self.CFA1())
                .field("RF", &self.RF())
                .field("REF", &self.REF())
                .field("RUF", &self.RUF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_STS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_STS {{ CMPF: {=u8:?}, CFX0: {=bool:?}, CFX1: {=bool:?}, CFB0: {=bool:?}, CFB1: {=bool:?}, CFA0: {=bool:?}, CFA1: {=bool:?}, RF: {=bool:?}, REF: {=bool:?}, RUF: {=bool:?} }}" , self . CMPF () , self . CFX0 () , self . CFX1 () , self . CFB0 () , self . CFB1 () , self . CFA0 () , self . CFA1 () , self . RF () , self . REF () , self . RUF ())
        }
    }
    #[doc = "Output Trigger Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_TCTRL(pub u16);
    impl SM_TCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn OUT_TRIG_EN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OUT_TRIG_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRGFRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRGFRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWBOT1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWBOT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWAOT0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWAOT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for SM_TCTRL {
        #[inline(always)]
        fn default() -> SM_TCTRL {
            SM_TCTRL(0)
        }
    }
    impl core::fmt::Debug for SM_TCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_TCTRL")
                .field("OUT_TRIG_EN", &self.OUT_TRIG_EN())
                .field("TRGFRQ", &self.TRGFRQ())
                .field("PWBOT1", &self.PWBOT1())
                .field("PWAOT0", &self.PWAOT0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_TCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SM_TCTRL {{ OUT_TRIG_EN: {=u8:?}, TRGFRQ: {=bool:?}, PWBOT1: {=bool:?}, PWAOT0: {=bool:?} }}" , self . OUT_TRIG_EN () , self . TRGFRQ () , self . PWBOT1 () , self . PWAOT0 ())
        }
    }
    #[doc = "Software Controlled Output Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWCOUT(pub u16);
    impl SWCOUT {
        #[must_use]
        #[inline(always)]
        pub const fn SM0OUT45(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM0OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM0OUT23(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM0OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM1OUT45(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM1OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM1OUT23(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM1OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM2OUT45(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM2OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM2OUT23(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM2OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM3OUT45(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM3OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SM3OUT23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SM3OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for SWCOUT {
        #[inline(always)]
        fn default() -> SWCOUT {
            SWCOUT(0)
        }
    }
    impl core::fmt::Debug for SWCOUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWCOUT")
                .field("SM0OUT45", &self.SM0OUT45())
                .field("SM0OUT23", &self.SM0OUT23())
                .field("SM1OUT45", &self.SM1OUT45())
                .field("SM1OUT23", &self.SM1OUT23())
                .field("SM2OUT45", &self.SM2OUT45())
                .field("SM2OUT23", &self.SM2OUT23())
                .field("SM3OUT45", &self.SM3OUT45())
                .field("SM3OUT23", &self.SM3OUT23())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWCOUT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SWCOUT {{ SM0OUT45: {=bool:?}, SM0OUT23: {=bool:?}, SM1OUT45: {=bool:?}, SM1OUT23: {=bool:?}, SM2OUT45: {=bool:?}, SM2OUT23: {=bool:?}, SM3OUT45: {=bool:?}, SM3OUT23: {=bool:?} }}" , self . SM0OUT45 () , self . SM0OUT23 () , self . SM1OUT45 () , self . SM1OUT23 () , self . SM2OUT45 () , self . SM2OUT23 () , self . SM3OUT45 () , self . SM3OUT23 ())
        }
    }
}
