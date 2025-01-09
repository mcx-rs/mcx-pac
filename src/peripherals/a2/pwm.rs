#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn CNT(self) -> crate::common::Reg<regs::SM_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn INIT(self) -> crate::common::Reg<regs::SM_INIT, crate::common::RW> {
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
    pub const fn VAL0(self) -> crate::common::Reg<regs::SM_VAL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn VAL1(self) -> crate::common::Reg<regs::SM_VAL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn VAL2(self) -> crate::common::Reg<regs::SM_VAL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL3(self) -> crate::common::Reg<regs::SM_VAL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn VAL4(self) -> crate::common::Reg<regs::SM_VAL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[inline(always)]
    pub const fn VAL5(self) -> crate::common::Reg<regs::SM_VAL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
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
    pub const fn DISMAP(self, n: usize) -> crate::common::Reg<regs::SM_DISMAP, crate::common::RW> {
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
    pub const fn CAPTCTRLX(self) -> crate::common::Reg<regs::SM_CAPTCTRLX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTCOMPX(self) -> crate::common::Reg<regs::SM_CAPTCOMPX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL0(self) -> crate::common::Reg<regs::SM_CVAL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL0CYC(self) -> crate::common::Reg<regs::SM_CVAL0CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL1(self) -> crate::common::Reg<regs::SM_CVAL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn CVAL1CYC(self) -> crate::common::Reg<regs::SM_CVAL1CYC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x46usize) as _) }
    }
    #[inline(always)]
    pub const fn PHASEDLY(self) -> crate::common::Reg<regs::SM_PHASEDLY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
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
    pub struct DTSRCSEL(pub u32);
    impl DTSRCSEL {
        #[inline(always)]
        pub const fn SM0SEL45(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM0SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn SM0SEL23(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM0SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn SM1SEL45(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM1SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn SM1SEL23(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM1SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn SM2SEL45(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM2SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn SM2SEL23(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM2SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn SM3SEL45(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM3SEL45(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn SM3SEL23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SM3SEL23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
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
            #[derive(defmt :: Format)]
            struct DTSRCSEL {
                SM0SEL45: u8,
                SM0SEL23: u8,
                SM1SEL45: u8,
                SM1SEL23: u8,
                SM2SEL45: u8,
                SM2SEL23: u8,
                SM3SEL45: u8,
                SM3SEL23: u8,
            }
            let proxy = DTSRCSEL {
                SM0SEL45: self.SM0SEL45(),
                SM0SEL23: self.SM0SEL23(),
                SM1SEL45: self.SM1SEL45(),
                SM1SEL23: self.SM1SEL23(),
                SM2SEL45: self.SM2SEL45(),
                SM2SEL23: self.SM2SEL23(),
                SM3SEL45: self.SM3SEL45(),
                SM3SEL23: self.SM3SEL23(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u32);
    impl FCTRL {
        #[inline(always)]
        pub const fn FIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FSAFE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FSAFE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn FAUTO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FAUTO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn FLVL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
            #[derive(defmt :: Format)]
            struct FCTRL {
                FIE: u8,
                FSAFE: u8,
                FAUTO: u8,
                FLVL: u8,
            }
            let proxy = FCTRL {
                FIE: self.FIE(),
                FSAFE: self.FSAFE(),
                FAUTO: self.FAUTO(),
                FLVL: self.FLVL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL2(pub u32);
    impl FCTRL2 {
        #[inline(always)]
        pub const fn NOCOMB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NOCOMB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            #[derive(defmt :: Format)]
            struct FCTRL2 {
                NOCOMB: u8,
            }
            let proxy = FCTRL2 {
                NOCOMB: self.NOCOMB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FFILT(pub u32);
    impl FFILT {
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn GSTR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GSTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            #[derive(defmt :: Format)]
            struct FFILT {
                FILT_PER: u8,
                FILT_CNT: u8,
                GSTR: bool,
            }
            let proxy = FFILT {
                FILT_PER: self.FILT_PER(),
                FILT_CNT: self.FILT_CNT(),
                GSTR: self.GSTR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FSTS(pub u32);
    impl FSTS {
        #[inline(always)]
        pub const fn FFLAG(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FFLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FFULL(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FFULL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn FFPIN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FFPIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn FHALF(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FHALF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
            #[derive(defmt :: Format)]
            struct FSTS {
                FFLAG: u8,
                FFULL: u8,
                FFPIN: u8,
                FHALF: u8,
            }
            let proxy = FSTS {
                FFLAG: self.FFLAG(),
                FFULL: self.FFULL(),
                FFPIN: self.FFPIN(),
                FHALF: self.FHALF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FTST(pub u32);
    impl FTST {
        #[inline(always)]
        pub const fn FTEST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FTEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            #[derive(defmt :: Format)]
            struct FTST {
                FTEST: bool,
            }
            let proxy = FTST {
                FTEST: self.FTEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Mask Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASK(pub u32);
    impl MASK {
        #[inline(always)]
        pub const fn MASKX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MASKX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn MASKB(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MASKB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn MASKA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MASKA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn UPDATE_MASK(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_UPDATE_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
            #[derive(defmt :: Format)]
            struct MASK {
                MASKX: u8,
                MASKB: u8,
                MASKA: u8,
                UPDATE_MASK: u8,
            }
            let proxy = MASK {
                MASKX: self.MASKX(),
                MASKB: self.MASKB(),
                MASKA: self.MASKA(),
                UPDATE_MASK: self.UPDATE_MASK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTRL(pub u32);
    impl MCTRL {
        #[inline(always)]
        pub const fn LDOK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LDOK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn CLDOK(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLDOK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn RUN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RUN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn IPOL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IPOL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
            #[derive(defmt :: Format)]
            struct MCTRL {
                LDOK: u8,
                CLDOK: u8,
                RUN: u8,
                IPOL: u8,
            }
            let proxy = MCTRL {
                LDOK: self.LDOK(),
                CLDOK: self.CLDOK(),
                RUN: self.RUN(),
                IPOL: self.IPOL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTRL2(pub u32);
    impl MCTRL2 {
        #[inline(always)]
        pub const fn WRPROT(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WRPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn STRETCH_CNT_PRSC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_STRETCH_CNT_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
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
            #[derive(defmt :: Format)]
            struct MCTRL2 {
                WRPROT: u8,
                STRETCH_CNT_PRSC: u8,
            }
            let proxy = MCTRL2 {
                WRPROT: self.WRPROT(),
                STRETCH_CNT_PRSC: self.STRETCH_CNT_PRSC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTEN(pub u32);
    impl OUTEN {
        #[inline(always)]
        pub const fn PWMX_EN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMX_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn PWMB_EN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMB_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn PWMA_EN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMA_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
            #[derive(defmt :: Format)]
            struct OUTEN {
                PWMX_EN: u8,
                PWMB_EN: u8,
                PWMA_EN: u8,
            }
            let proxy = OUTEN {
                PWMX_EN: self.PWMX_EN(),
                PWMB_EN: self.PWMB_EN(),
                PWMA_EN: self.PWMA_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Compare X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCOMPX(pub u32);
    impl SM_CAPTCOMPX {
        #[inline(always)]
        pub const fn EDGCMPX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EDGCMPX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn EDGCNTX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EDGCNTX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
            #[derive(defmt :: Format)]
            struct SM_CAPTCOMPX {
                EDGCMPX: u8,
                EDGCNTX: u8,
            }
            let proxy = SM_CAPTCOMPX {
                EDGCMPX: self.EDGCMPX(),
                EDGCNTX: self.EDGCNTX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Control X Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTCTRLX(pub u32);
    impl SM_CAPTCTRLX {
        #[inline(always)]
        pub const fn ARMX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ARMX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ONESHOTX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ONESHOTX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EDGX0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_EDGX0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn EDGX1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_EDGX1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn INP_SELX(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INP_SELX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn EDGCNTX_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDGCNTX_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn CFXWM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CFXWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn CX0CNT(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CX0CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[inline(always)]
        pub const fn CX1CNT(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CX1CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
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
            #[derive(defmt :: Format)]
            struct SM_CAPTCTRLX {
                ARMX: bool,
                ONESHOTX: bool,
                EDGX0: u8,
                EDGX1: u8,
                INP_SELX: bool,
                EDGCNTX_EN: bool,
                CFXWM: u8,
                CX0CNT: u8,
                CX1CNT: u8,
            }
            let proxy = SM_CAPTCTRLX {
                ARMX: self.ARMX(),
                ONESHOTX: self.ONESHOTX(),
                EDGX0: self.EDGX0(),
                EDGX1: self.EDGX1(),
                INP_SELX: self.INP_SELX(),
                EDGCNTX_EN: self.EDGCNTX_EN(),
                CFXWM: self.CFXWM(),
                CX0CNT: self.CX0CNT(),
                CX1CNT: self.CX1CNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CAPTFILTX(pub u32);
    impl SM_CAPTFILTX {
        #[inline(always)]
        pub const fn CAPTX_FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAPTX_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CAPTX_FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAPTX_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
            #[derive(defmt :: Format)]
            struct SM_CAPTFILTX {
                CAPTX_FILT_PER: u8,
                CAPTX_FILT_CNT: u8,
            }
            let proxy = SM_CAPTFILTX {
                CAPTX_FILT_PER: self.CAPTX_FILT_PER(),
                CAPTX_FILT_CNT: self.CAPTX_FILT_CNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CNT(pub u32);
    impl SM_CNT {
        #[inline(always)]
        pub const fn CNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_CNT {
        #[inline(always)]
        fn default() -> SM_CNT {
            SM_CNT(0)
        }
    }
    impl core::fmt::Debug for SM_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CNT").field("CNT", &self.CNT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CNT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_CNT {
                CNT: u16,
            }
            let proxy = SM_CNT { CNT: self.CNT() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CTRL(pub u32);
    impl SM_CTRL {
        #[inline(always)]
        pub const fn DBLEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBLEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DBLX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBLX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LDMOD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SPLIT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn PRSC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[inline(always)]
        pub const fn COMPMODE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COMPMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn DT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn FULL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn HALF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn LDFQ(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LDFQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
            #[derive(defmt :: Format)]
            struct SM_CTRL {
                DBLEN: bool,
                DBLX: bool,
                LDMOD: bool,
                SPLIT: bool,
                PRSC: u8,
                COMPMODE: bool,
                DT: u8,
                FULL: bool,
                HALF: bool,
                LDFQ: u8,
            }
            let proxy = SM_CTRL {
                DBLEN: self.DBLEN(),
                DBLX: self.DBLX(),
                LDMOD: self.LDMOD(),
                SPLIT: self.SPLIT(),
                PRSC: self.PRSC(),
                COMPMODE: self.COMPMODE(),
                DT: self.DT(),
                FULL: self.FULL(),
                HALF: self.HALF(),
                LDFQ: self.LDFQ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CTRL2(pub u32);
    impl SM_CTRL2 {
        #[inline(always)]
        pub const fn CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn RELOAD_SEL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RELOAD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FORCE_SEL(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FORCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[inline(always)]
        pub const fn FORCE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FORCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FRCEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn INIT_SEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INIT_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn PWMX_INIT(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWMX_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn PWM45_INIT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM45_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn PWM23_INIT(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM23_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn INDEP(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INDEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DBGEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            #[derive(defmt :: Format)]
            struct SM_CTRL2 {
                CLK_SEL: u8,
                RELOAD_SEL: bool,
                FORCE_SEL: u8,
                FORCE: bool,
                FRCEN: bool,
                INIT_SEL: u8,
                PWMX_INIT: bool,
                PWM45_INIT: bool,
                PWM23_INIT: bool,
                INDEP: bool,
                DBGEN: bool,
            }
            let proxy = SM_CTRL2 {
                CLK_SEL: self.CLK_SEL(),
                RELOAD_SEL: self.RELOAD_SEL(),
                FORCE_SEL: self.FORCE_SEL(),
                FORCE: self.FORCE(),
                FRCEN: self.FRCEN(),
                INIT_SEL: self.INIT_SEL(),
                PWMX_INIT: self.PWMX_INIT(),
                PWM45_INIT: self.PWM45_INIT(),
                PWM23_INIT: self.PWM23_INIT(),
                INDEP: self.INDEP(),
                DBGEN: self.DBGEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Value 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL0(pub u32);
    impl SM_CVAL0 {
        #[inline(always)]
        pub const fn CAPTVAL0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPTVAL0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_CVAL0 {
        #[inline(always)]
        fn default() -> SM_CVAL0 {
            SM_CVAL0(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL0")
                .field("CAPTVAL0", &self.CAPTVAL0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_CVAL0 {
                CAPTVAL0: u16,
            }
            let proxy = SM_CVAL0 {
                CAPTVAL0: self.CAPTVAL0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL0CYC(pub u32);
    impl SM_CVAL0CYC {
        #[inline(always)]
        pub const fn CVAL0CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CVAL0CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            #[derive(defmt :: Format)]
            struct SM_CVAL0CYC {
                CVAL0CYC: u8,
            }
            let proxy = SM_CVAL0CYC {
                CVAL0CYC: self.CVAL0CYC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Value 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL1(pub u32);
    impl SM_CVAL1 {
        #[inline(always)]
        pub const fn CAPTVAL1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPTVAL1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_CVAL1 {
        #[inline(always)]
        fn default() -> SM_CVAL1 {
            SM_CVAL1(0)
        }
    }
    impl core::fmt::Debug for SM_CVAL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_CVAL1")
                .field("CAPTVAL1", &self.CAPTVAL1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_CVAL1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_CVAL1 {
                CAPTVAL1: u16,
            }
            let proxy = SM_CVAL1 {
                CAPTVAL1: self.CAPTVAL1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_CVAL1CYC(pub u32);
    impl SM_CVAL1CYC {
        #[inline(always)]
        pub const fn CVAL1CYC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CVAL1CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            #[derive(defmt :: Format)]
            struct SM_CVAL1CYC {
                CVAL1CYC: u8,
            }
            let proxy = SM_CVAL1CYC {
                CVAL1CYC: self.CVAL1CYC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DISMAP(pub u32);
    impl SM_DISMAP {
        #[inline(always)]
        pub const fn DIS0A(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIS0A(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn DIS0B(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIS0B(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn DIS0X(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIS0X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
            #[derive(defmt :: Format)]
            struct SM_DISMAP {
                DIS0A: u8,
                DIS0B: u8,
                DIS0X: u8,
            }
            let proxy = SM_DISMAP {
                DIS0A: self.DIS0A(),
                DIS0B: self.DIS0B(),
                DIS0X: self.DIS0X(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DMAEN(pub u32);
    impl SM_DMAEN {
        #[inline(always)]
        pub const fn CX0DE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CX0DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CX1DE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CX1DE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CAPTDE(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAPTDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn FAND(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FAND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VALDE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VALDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("CAPTDE", &self.CAPTDE())
                .field("FAND", &self.FAND())
                .field("VALDE", &self.VALDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_DMAEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_DMAEN {
                CX0DE: bool,
                CX1DE: bool,
                CAPTDE: u8,
                FAND: bool,
                VALDE: bool,
            }
            let proxy = SM_DMAEN {
                CX0DE: self.CX0DE(),
                CX1DE: self.CX1DE(),
                CAPTDE: self.CAPTDE(),
                FAND: self.FAND(),
                VALDE: self.VALDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Deadtime Count Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DTCNT0(pub u32);
    impl SM_DTCNT0 {
        #[inline(always)]
        pub const fn DTCNT0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DTCNT0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
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
            #[derive(defmt :: Format)]
            struct SM_DTCNT0 {
                DTCNT0: u16,
            }
            let proxy = SM_DTCNT0 {
                DTCNT0: self.DTCNT0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Deadtime Count Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_DTCNT1(pub u32);
    impl SM_DTCNT1 {
        #[inline(always)]
        pub const fn DTCNT1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DTCNT1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
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
            #[derive(defmt :: Format)]
            struct SM_DTCNT1 {
                DTCNT1: u16,
            }
            let proxy = SM_DTCNT1 {
                DTCNT1: self.DTCNT1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Initial Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_INIT(pub u32);
    impl SM_INIT {
        #[inline(always)]
        pub const fn INIT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_INIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_INIT {
        #[inline(always)]
        fn default() -> SM_INIT {
            SM_INIT(0)
        }
    }
    impl core::fmt::Debug for SM_INIT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_INIT")
                .field("INIT", &self.INIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_INIT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_INIT {
                INIT: u16,
            }
            let proxy = SM_INIT { INIT: self.INIT() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_INTEN(pub u32);
    impl SM_INTEN {
        #[inline(always)]
        pub const fn CMPIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMPIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CX0IE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CX0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CX1IE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CX1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REIE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("RIE", &self.RIE())
                .field("REIE", &self.REIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_INTEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_INTEN {
                CMPIE: u8,
                CX0IE: bool,
                CX1IE: bool,
                RIE: bool,
                REIE: bool,
            }
            let proxy = SM_INTEN {
                CMPIE: self.CMPIE(),
                CX0IE: self.CX0IE(),
                CX1IE: self.CX1IE(),
                RIE: self.RIE(),
                REIE: self.REIE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_OCTRL(pub u32);
    impl SM_OCTRL {
        #[inline(always)]
        pub const fn PWMXFS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMXFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PWMBFS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMBFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PWMAFS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWMAFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn POLX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POLX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn POLB(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POLB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn POLA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POLA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn PWMX_IN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWMX_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PWMB_IN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWMB_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PWMA_IN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWMA_IN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            #[derive(defmt :: Format)]
            struct SM_OCTRL {
                PWMXFS: u8,
                PWMBFS: u8,
                PWMAFS: u8,
                POLX: bool,
                POLB: bool,
                POLA: bool,
                PWMX_IN: bool,
                PWMB_IN: bool,
                PWMA_IN: bool,
            }
            let proxy = SM_OCTRL {
                PWMXFS: self.PWMXFS(),
                PWMBFS: self.PWMBFS(),
                PWMAFS: self.PWMAFS(),
                POLX: self.POLX(),
                POLB: self.POLB(),
                POLA: self.POLA(),
                PWMX_IN: self.PWMX_IN(),
                PWMB_IN: self.PWMB_IN(),
                PWMA_IN: self.PWMA_IN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Phase Delay Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_PHASEDLY(pub u32);
    impl SM_PHASEDLY {
        #[inline(always)]
        pub const fn PHASEDLY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PHASEDLY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_PHASEDLY {
        #[inline(always)]
        fn default() -> SM_PHASEDLY {
            SM_PHASEDLY(0)
        }
    }
    impl core::fmt::Debug for SM_PHASEDLY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_PHASEDLY")
                .field("PHASEDLY", &self.PHASEDLY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_PHASEDLY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_PHASEDLY {
                PHASEDLY: u16,
            }
            let proxy = SM_PHASEDLY {
                PHASEDLY: self.PHASEDLY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_STS(pub u32);
    impl SM_STS {
        #[inline(always)]
        pub const fn CMPF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMPF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CFX0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFX0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CFX1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFX1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REF(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn RUF(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("RF", &self.RF())
                .field("REF", &self.REF())
                .field("RUF", &self.RUF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_STS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_STS {
                CMPF: u8,
                CFX0: bool,
                CFX1: bool,
                RF: bool,
                REF: bool,
                RUF: bool,
            }
            let proxy = SM_STS {
                CMPF: self.CMPF(),
                CFX0: self.CFX0(),
                CFX1: self.CFX1(),
                RF: self.RF(),
                REF: self.REF(),
                RUF: self.RUF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Trigger Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_TCTRL(pub u32);
    impl SM_TCTRL {
        #[inline(always)]
        pub const fn OUT_TRIG_EN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUT_TRIG_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn TRGFRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRGFRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PWBOT1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWBOT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PWAOT0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWAOT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
            #[derive(defmt :: Format)]
            struct SM_TCTRL {
                OUT_TRIG_EN: u8,
                TRGFRQ: bool,
                PWBOT1: bool,
                PWAOT0: bool,
            }
            let proxy = SM_TCTRL {
                OUT_TRIG_EN: self.OUT_TRIG_EN(),
                TRGFRQ: self.TRGFRQ(),
                PWBOT1: self.PWBOT1(),
                PWAOT0: self.PWAOT0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL0(pub u32);
    impl SM_VAL0 {
        #[inline(always)]
        pub const fn VAL0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL0 {
        #[inline(always)]
        fn default() -> SM_VAL0 {
            SM_VAL0(0)
        }
    }
    impl core::fmt::Debug for SM_VAL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL0")
                .field("VAL0", &self.VAL0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL0 {
                VAL0: u16,
            }
            let proxy = SM_VAL0 { VAL0: self.VAL0() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL1(pub u32);
    impl SM_VAL1 {
        #[inline(always)]
        pub const fn VAL1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL1 {
        #[inline(always)]
        fn default() -> SM_VAL1 {
            SM_VAL1(0)
        }
    }
    impl core::fmt::Debug for SM_VAL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL1")
                .field("VAL1", &self.VAL1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL1 {
                VAL1: u16,
            }
            let proxy = SM_VAL1 { VAL1: self.VAL1() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL2(pub u32);
    impl SM_VAL2 {
        #[inline(always)]
        pub const fn VAL2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL2 {
        #[inline(always)]
        fn default() -> SM_VAL2 {
            SM_VAL2(0)
        }
    }
    impl core::fmt::Debug for SM_VAL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL2")
                .field("VAL2", &self.VAL2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL2 {
                VAL2: u16,
            }
            let proxy = SM_VAL2 { VAL2: self.VAL2() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL3(pub u32);
    impl SM_VAL3 {
        #[inline(always)]
        pub const fn VAL3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL3 {
        #[inline(always)]
        fn default() -> SM_VAL3 {
            SM_VAL3(0)
        }
    }
    impl core::fmt::Debug for SM_VAL3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL3")
                .field("VAL3", &self.VAL3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL3 {
                VAL3: u16,
            }
            let proxy = SM_VAL3 { VAL3: self.VAL3() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL4(pub u32);
    impl SM_VAL4 {
        #[inline(always)]
        pub const fn VAL4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL4 {
        #[inline(always)]
        fn default() -> SM_VAL4 {
            SM_VAL4(0)
        }
    }
    impl core::fmt::Debug for SM_VAL4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL4")
                .field("VAL4", &self.VAL4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL4 {
                VAL4: u16,
            }
            let proxy = SM_VAL4 { VAL4: self.VAL4() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Value Register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SM_VAL5(pub u32);
    impl SM_VAL5 {
        #[inline(always)]
        pub const fn VAL5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_VAL5(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SM_VAL5 {
        #[inline(always)]
        fn default() -> SM_VAL5 {
            SM_VAL5(0)
        }
    }
    impl core::fmt::Debug for SM_VAL5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SM_VAL5")
                .field("VAL5", &self.VAL5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SM_VAL5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SM_VAL5 {
                VAL5: u16,
            }
            let proxy = SM_VAL5 { VAL5: self.VAL5() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Software Controlled Output Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWCOUT(pub u32);
    impl SWCOUT {
        #[inline(always)]
        pub const fn SM0OUT45(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM0OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SM0OUT23(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM0OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SM1OUT45(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM1OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SM1OUT23(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM1OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SM2OUT45(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM2OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn SM2OUT23(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM2OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SM3OUT45(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM3OUT45(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SM3OUT23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SM3OUT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            #[derive(defmt :: Format)]
            struct SWCOUT {
                SM0OUT45: bool,
                SM0OUT23: bool,
                SM1OUT45: bool,
                SM1OUT23: bool,
                SM2OUT45: bool,
                SM2OUT23: bool,
                SM3OUT45: bool,
                SM3OUT23: bool,
            }
            let proxy = SWCOUT {
                SM0OUT45: self.SM0OUT45(),
                SM0OUT23: self.SM0OUT23(),
                SM1OUT45: self.SM1OUT45(),
                SM1OUT23: self.SM1OUT23(),
                SM2OUT45: self.SM2OUT45(),
                SM2OUT23: self.SM2OUT23(),
                SM3OUT45: self.SM3OUT45(),
                SM3OUT23: self.SM3OUT23(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
