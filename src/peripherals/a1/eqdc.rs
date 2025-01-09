#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EQDC {
    ptr: *mut u8,
}
unsafe impl Send for EQDC {}
unsafe impl Sync for EQDC {}
impl EQDC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<regs::CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn FILT(self) -> crate::common::Reg<regs::FILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn LASTEDGE(self) -> crate::common::Reg<regs::LASTEDGE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPER(self) -> crate::common::Reg<regs::POSDPER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPERBFR(self) -> crate::common::Reg<regs::POSDPERBFR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn UPOS(self) -> crate::common::Reg<regs::UPOS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn LPOS(self) -> crate::common::Reg<regs::LPOS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn POSD(self) -> crate::common::Reg<regs::POSD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn POSDH(self) -> crate::common::Reg<regs::POSDH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn UPOSH(self) -> crate::common::Reg<regs::UPOSH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn LPOSH(self) -> crate::common::Reg<regs::LPOSH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn LASTEDGEH(self) -> crate::common::Reg<regs::LASTEDGEH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPERH(self) -> crate::common::Reg<regs::POSDPERH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[inline(always)]
    pub const fn REVH(self) -> crate::common::Reg<regs::REVH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn REV(self) -> crate::common::Reg<regs::REV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[inline(always)]
    pub const fn UINIT(self) -> crate::common::Reg<regs::UINIT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn LINIT(self) -> crate::common::Reg<regs::LINIT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[inline(always)]
    pub const fn UMOD(self) -> crate::common::Reg<regs::UMOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn LMOD(self) -> crate::common::Reg<regs::LMOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[inline(always)]
    pub const fn UCOMP0(self) -> crate::common::Reg<regs::UCOMP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn LCOMP0(self) -> crate::common::Reg<regs::LCOMP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[inline(always)]
    pub const fn UCOMP1(self) -> crate::common::Reg<regs::UCOMP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn UPOSH1(self) -> crate::common::Reg<regs::UPOSH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn LCOMP1(self) -> crate::common::Reg<regs::LCOMP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[inline(always)]
    pub const fn LPOSH1(self) -> crate::common::Reg<regs::LPOSH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[inline(always)]
    pub const fn UCOMP2(self) -> crate::common::Reg<regs::UCOMP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn UPOSH2(self) -> crate::common::Reg<regs::UPOSH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn LCOMP2(self) -> crate::common::Reg<regs::LCOMP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
    #[inline(always)]
    pub const fn LPOSH2(self) -> crate::common::Reg<regs::LPOSH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
    #[inline(always)]
    pub const fn UCOMP3(self) -> crate::common::Reg<regs::UCOMP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn UPOSH3(self) -> crate::common::Reg<regs::UPOSH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn LCOMP3(self) -> crate::common::Reg<regs::LCOMP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[inline(always)]
    pub const fn LPOSH3(self) -> crate::common::Reg<regs::LPOSH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x36usize) as _) }
    }
    #[inline(always)]
    pub const fn INTCTRL(self) -> crate::common::Reg<regs::INTCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn WTR(self) -> crate::common::Reg<regs::WTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3ausize) as _) }
    }
    #[inline(always)]
    pub const fn IMR(self) -> crate::common::Reg<regs::IMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn TST(self) -> crate::common::Reg<regs::TST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3eusize) as _) }
    }
    #[inline(always)]
    pub const fn UVERID(self) -> crate::common::Reg<regs::UVERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn LVERID(self) -> crate::common::Reg<regs::LVERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x52usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn LDOK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DMAEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WDIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WDIRQ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn XNE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn XIP(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn XIE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn XIRQ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn PH1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PH1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REV(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SWIP(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn HNE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn HIP(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn HIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn HIRQ(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("LDOK", &self.LDOK())
                .field("DMAEN", &self.DMAEN())
                .field("WDE", &self.WDE())
                .field("WDIE", &self.WDIE())
                .field("WDIRQ", &self.WDIRQ())
                .field("XNE", &self.XNE())
                .field("XIP", &self.XIP())
                .field("XIE", &self.XIE())
                .field("XIRQ", &self.XIRQ())
                .field("PH1", &self.PH1())
                .field("REV", &self.REV())
                .field("SWIP", &self.SWIP())
                .field("HNE", &self.HNE())
                .field("HIP", &self.HIP())
                .field("HIE", &self.HIE())
                .field("HIRQ", &self.HIRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL {
                LDOK: bool,
                DMAEN: bool,
                WDE: bool,
                WDIE: bool,
                WDIRQ: bool,
                XNE: bool,
                XIP: bool,
                XIE: bool,
                XIRQ: bool,
                PH1: bool,
                REV: bool,
                SWIP: bool,
                HNE: bool,
                HIP: bool,
                HIE: bool,
                HIRQ: bool,
            }
            let proxy = CTRL {
                LDOK: self.LDOK(),
                DMAEN: self.DMAEN(),
                WDE: self.WDE(),
                WDIE: self.WDIE(),
                WDIRQ: self.WDIRQ(),
                XNE: self.XNE(),
                XIP: self.XIP(),
                XIE: self.XIE(),
                XIRQ: self.XIRQ(),
                PH1: self.PH1(),
                REV: self.REV(),
                SWIP: self.SWIP(),
                HNE: self.HNE(),
                HIP: self.HIP(),
                HIE: self.HIE(),
                HIRQ: self.HIRQ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL2(pub u32);
    impl CTRL2 {
        #[inline(always)]
        pub const fn UPDHLD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPDHLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn UPDPOS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPDPOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn OPMODE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn LDMOD(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REVMOD(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REVMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OUTCTL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUTCTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn PMEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn EMIP(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EMIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn INITPOS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INITPOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ONCE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ONCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn CMODE(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for CTRL2 {
        #[inline(always)]
        fn default() -> CTRL2 {
            CTRL2(0)
        }
    }
    impl core::fmt::Debug for CTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL2")
                .field("UPDHLD", &self.UPDHLD())
                .field("UPDPOS", &self.UPDPOS())
                .field("OPMODE", &self.OPMODE())
                .field("LDMOD", &self.LDMOD())
                .field("REVMOD", &self.REVMOD())
                .field("OUTCTL", &self.OUTCTL())
                .field("PMEN", &self.PMEN())
                .field("EMIP", &self.EMIP())
                .field("INITPOS", &self.INITPOS())
                .field("ONCE", &self.ONCE())
                .field("CMODE", &self.CMODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL2 {
                UPDHLD: bool,
                UPDPOS: bool,
                OPMODE: bool,
                LDMOD: bool,
                REVMOD: bool,
                OUTCTL: bool,
                PMEN: bool,
                EMIP: bool,
                INITPOS: bool,
                ONCE: bool,
                CMODE: u8,
            }
            let proxy = CTRL2 {
                UPDHLD: self.UPDHLD(),
                UPDPOS: self.UPDPOS(),
                OPMODE: self.OPMODE(),
                LDMOD: self.LDMOD(),
                REVMOD: self.REVMOD(),
                OUTCTL: self.OUTCTL(),
                PMEN: self.PMEN(),
                EMIP: self.EMIP(),
                INITPOS: self.INITPOS(),
                ONCE: self.ONCE(),
                CMODE: self.CMODE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Input Filter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FILT(pub u32);
    impl FILT {
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
        pub const fn FILT_CS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILT_CS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn PRSC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for FILT {
        #[inline(always)]
        fn default() -> FILT {
            FILT(0)
        }
    }
    impl core::fmt::Debug for FILT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FILT")
                .field("FILT_PER", &self.FILT_PER())
                .field("FILT_CNT", &self.FILT_CNT())
                .field("FILT_CS", &self.FILT_CS())
                .field("PRSC", &self.PRSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FILT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FILT {
                FILT_PER: u8,
                FILT_CNT: u8,
                FILT_CS: bool,
                PRSC: u8,
            }
            let proxy = FILT {
                FILT_PER: self.FILT_PER(),
                FILT_CNT: self.FILT_CNT(),
                FILT_CS: self.FILT_CS(),
                PRSC: self.PRSC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Input Monitor Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IMR(pub u32);
    impl IMR {
        #[inline(always)]
        pub const fn HOME_ENABLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOME_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INDEX_PRESET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INDEX_PRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PHB(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PHA(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FHOM_ENA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FHOM_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FIND_PRE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIND_PRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FPHB(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FPHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FPHA(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FPHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn CMPF0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMPF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CMP1F(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CMP2F(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CMP3F(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP3F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DIRH(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIRH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DIR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IMR {
        #[inline(always)]
        fn default() -> IMR {
            IMR(0)
        }
    }
    impl core::fmt::Debug for IMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IMR")
                .field("HOME_ENABLE", &self.HOME_ENABLE())
                .field("INDEX_PRESET", &self.INDEX_PRESET())
                .field("PHB", &self.PHB())
                .field("PHA", &self.PHA())
                .field("FHOM_ENA", &self.FHOM_ENA())
                .field("FIND_PRE", &self.FIND_PRE())
                .field("FPHB", &self.FPHB())
                .field("FPHA", &self.FPHA())
                .field("CMPF0", &self.CMPF0())
                .field("CMP1F", &self.CMP1F())
                .field("CMP2F", &self.CMP2F())
                .field("CMP3F", &self.CMP3F())
                .field("DIRH", &self.DIRH())
                .field("DIR", &self.DIR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IMR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IMR {
                HOME_ENABLE: bool,
                INDEX_PRESET: bool,
                PHB: bool,
                PHA: bool,
                FHOM_ENA: bool,
                FIND_PRE: bool,
                FPHB: bool,
                FPHA: bool,
                CMPF0: bool,
                CMP1F: bool,
                CMP2F: bool,
                CMP3F: bool,
                DIRH: bool,
                DIR: bool,
            }
            let proxy = IMR {
                HOME_ENABLE: self.HOME_ENABLE(),
                INDEX_PRESET: self.INDEX_PRESET(),
                PHB: self.PHB(),
                PHA: self.PHA(),
                FHOM_ENA: self.FHOM_ENA(),
                FIND_PRE: self.FIND_PRE(),
                FPHB: self.FPHB(),
                FPHA: self.FPHA(),
                CMPF0: self.CMPF0(),
                CMP1F: self.CMP1F(),
                CMP2F: self.CMP2F(),
                CMP3F: self.CMP3F(),
                DIRH: self.DIRH(),
                DIR: self.DIR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTCTRL(pub u32);
    impl INTCTRL {
        #[inline(always)]
        pub const fn SABIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SABIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SABIRQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SABIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DIRIE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIRIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIRIRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIRIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RUIE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RUIRQ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ROIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ROIRQ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn CMP0IE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CMP0IRQ(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP0IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CMP1IE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CMP1IRQ(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CMP2IE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CMP2IRQ(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn CMP3IE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP3IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn CMP3IRQ(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP3IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for INTCTRL {
        #[inline(always)]
        fn default() -> INTCTRL {
            INTCTRL(0)
        }
    }
    impl core::fmt::Debug for INTCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTCTRL")
                .field("SABIE", &self.SABIE())
                .field("SABIRQ", &self.SABIRQ())
                .field("DIRIE", &self.DIRIE())
                .field("DIRIRQ", &self.DIRIRQ())
                .field("RUIE", &self.RUIE())
                .field("RUIRQ", &self.RUIRQ())
                .field("ROIE", &self.ROIE())
                .field("ROIRQ", &self.ROIRQ())
                .field("CMP0IE", &self.CMP0IE())
                .field("CMP0IRQ", &self.CMP0IRQ())
                .field("CMP1IE", &self.CMP1IE())
                .field("CMP1IRQ", &self.CMP1IRQ())
                .field("CMP2IE", &self.CMP2IE())
                .field("CMP2IRQ", &self.CMP2IRQ())
                .field("CMP3IE", &self.CMP3IE())
                .field("CMP3IRQ", &self.CMP3IRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INTCTRL {
                SABIE: bool,
                SABIRQ: bool,
                DIRIE: bool,
                DIRIRQ: bool,
                RUIE: bool,
                RUIRQ: bool,
                ROIE: bool,
                ROIRQ: bool,
                CMP0IE: bool,
                CMP0IRQ: bool,
                CMP1IE: bool,
                CMP1IRQ: bool,
                CMP2IE: bool,
                CMP2IRQ: bool,
                CMP3IE: bool,
                CMP3IRQ: bool,
            }
            let proxy = INTCTRL {
                SABIE: self.SABIE(),
                SABIRQ: self.SABIRQ(),
                DIRIE: self.DIRIE(),
                DIRIRQ: self.DIRIRQ(),
                RUIE: self.RUIE(),
                RUIRQ: self.RUIRQ(),
                ROIE: self.ROIE(),
                ROIRQ: self.ROIRQ(),
                CMP0IE: self.CMP0IE(),
                CMP0IRQ: self.CMP0IRQ(),
                CMP1IE: self.CMP1IE(),
                CMP1IRQ: self.CMP1IRQ(),
                CMP2IE: self.CMP2IE(),
                CMP2IRQ: self.CMP2IRQ(),
                CMP3IE: self.CMP3IE(),
                CMP3IRQ: self.CMP3IRQ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Last Edge Time Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LASTEDGE(pub u32);
    impl LASTEDGE {
        #[inline(always)]
        pub const fn LASTEDGE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LASTEDGE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LASTEDGE {
        #[inline(always)]
        fn default() -> LASTEDGE {
            LASTEDGE(0)
        }
    }
    impl core::fmt::Debug for LASTEDGE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LASTEDGE")
                .field("LASTEDGE", &self.LASTEDGE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LASTEDGE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LASTEDGE {
                LASTEDGE: u16,
            }
            let proxy = LASTEDGE {
                LASTEDGE: self.LASTEDGE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Last Edge Time Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LASTEDGEH(pub u32);
    impl LASTEDGEH {
        #[inline(always)]
        pub const fn LASTEDGEH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LASTEDGEH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LASTEDGEH {
        #[inline(always)]
        fn default() -> LASTEDGEH {
            LASTEDGEH(0)
        }
    }
    impl core::fmt::Debug for LASTEDGEH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LASTEDGEH")
                .field("LASTEDGEH", &self.LASTEDGEH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LASTEDGEH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LASTEDGEH {
                LASTEDGEH: u16,
            }
            let proxy = LASTEDGEH {
                LASTEDGEH: self.LASTEDGEH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Compare Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCOMP0(pub u32);
    impl LCOMP0 {
        #[inline(always)]
        pub const fn LCOMP0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LCOMP0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LCOMP0 {
        #[inline(always)]
        fn default() -> LCOMP0 {
            LCOMP0(0)
        }
    }
    impl core::fmt::Debug for LCOMP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCOMP0")
                .field("LCOMP0", &self.LCOMP0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCOMP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LCOMP0 {
                LCOMP0: u16,
            }
            let proxy = LCOMP0 {
                LCOMP0: self.LCOMP0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Compare 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCOMP1(pub u32);
    impl LCOMP1 {
        #[inline(always)]
        pub const fn LCOMP1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LCOMP1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LCOMP1 {
        #[inline(always)]
        fn default() -> LCOMP1 {
            LCOMP1(0)
        }
    }
    impl core::fmt::Debug for LCOMP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCOMP1")
                .field("LCOMP1", &self.LCOMP1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCOMP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LCOMP1 {
                LCOMP1: u16,
            }
            let proxy = LCOMP1 {
                LCOMP1: self.LCOMP1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Compare 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCOMP2(pub u32);
    impl LCOMP2 {
        #[inline(always)]
        pub const fn LCOMP2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LCOMP2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LCOMP2 {
        #[inline(always)]
        fn default() -> LCOMP2 {
            LCOMP2(0)
        }
    }
    impl core::fmt::Debug for LCOMP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCOMP2")
                .field("LCOMP2", &self.LCOMP2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCOMP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LCOMP2 {
                LCOMP2: u16,
            }
            let proxy = LCOMP2 {
                LCOMP2: self.LCOMP2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Compare 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LCOMP3(pub u32);
    impl LCOMP3 {
        #[inline(always)]
        pub const fn LCOMP3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LCOMP3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LCOMP3 {
        #[inline(always)]
        fn default() -> LCOMP3 {
            LCOMP3(0)
        }
    }
    impl core::fmt::Debug for LCOMP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LCOMP3")
                .field("LCOMP3", &self.LCOMP3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LCOMP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LCOMP3 {
                LCOMP3: u16,
            }
            let proxy = LCOMP3 {
                LCOMP3: self.LCOMP3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Initialization Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LINIT(pub u32);
    impl LINIT {
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
    impl Default for LINIT {
        #[inline(always)]
        fn default() -> LINIT {
            LINIT(0)
        }
    }
    impl core::fmt::Debug for LINIT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LINIT").field("INIT", &self.INIT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LINIT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LINIT {
                INIT: u16,
            }
            let proxy = LINIT { INIT: self.INIT() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Modulus Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LMOD(pub u32);
    impl LMOD {
        #[inline(always)]
        pub const fn MOD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MOD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LMOD {
        #[inline(always)]
        fn default() -> LMOD {
            LMOD(0)
        }
    }
    impl core::fmt::Debug for LMOD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LMOD").field("MOD", &self.MOD()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LMOD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LMOD {
                MOD: u16,
            }
            let proxy = LMOD { MOD: self.MOD() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPOS(pub u32);
    impl LPOS {
        #[inline(always)]
        pub const fn POS(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POS(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPOS {
        #[inline(always)]
        fn default() -> LPOS {
            LPOS(0)
        }
    }
    impl core::fmt::Debug for LPOS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPOS").field("POS", &self.POS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPOS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPOS {
                POS: u16,
            }
            let proxy = LPOS { POS: self.POS() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPOSH(pub u32);
    impl LPOSH {
        #[inline(always)]
        pub const fn LPOSH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LPOSH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPOSH {
        #[inline(always)]
        fn default() -> LPOSH {
            LPOSH(0)
        }
    }
    impl core::fmt::Debug for LPOSH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPOSH")
                .field("LPOSH", &self.LPOSH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPOSH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPOSH {
                LPOSH: u16,
            }
            let proxy = LPOSH {
                LPOSH: self.LPOSH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Holder Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPOSH1(pub u32);
    impl LPOSH1 {
        #[inline(always)]
        pub const fn LPOSH1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LPOSH1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPOSH1 {
        #[inline(always)]
        fn default() -> LPOSH1 {
            LPOSH1(0)
        }
    }
    impl core::fmt::Debug for LPOSH1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPOSH1")
                .field("LPOSH1", &self.LPOSH1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPOSH1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPOSH1 {
                LPOSH1: u16,
            }
            let proxy = LPOSH1 {
                LPOSH1: self.LPOSH1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Holder Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPOSH2(pub u32);
    impl LPOSH2 {
        #[inline(always)]
        pub const fn LPOSH2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LPOSH2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPOSH2 {
        #[inline(always)]
        fn default() -> LPOSH2 {
            LPOSH2(0)
        }
    }
    impl core::fmt::Debug for LPOSH2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPOSH2")
                .field("LPOSH2", &self.LPOSH2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPOSH2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPOSH2 {
                LPOSH2: u16,
            }
            let proxy = LPOSH2 {
                LPOSH2: self.LPOSH2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower Position Holder Register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPOSH3(pub u32);
    impl LPOSH3 {
        #[inline(always)]
        pub const fn LPOSH3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LPOSH3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPOSH3 {
        #[inline(always)]
        fn default() -> LPOSH3 {
            LPOSH3(0)
        }
    }
    impl core::fmt::Debug for LPOSH3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPOSH3")
                .field("LPOSH3", &self.LPOSH3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPOSH3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPOSH3 {
                LPOSH3: u16,
            }
            let proxy = LPOSH3 {
                LPOSH3: self.LPOSH3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Lower VERID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LVERID(pub u32);
    impl LVERID {
        #[inline(always)]
        pub const fn LVERID(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LVERID(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LVERID {
        #[inline(always)]
        fn default() -> LVERID {
            LVERID(0)
        }
    }
    impl core::fmt::Debug for LVERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LVERID")
                .field("LVERID", &self.LVERID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LVERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LVERID {
                LVERID: u16,
            }
            let proxy = LVERID {
                LVERID: self.LVERID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Position Difference Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POSD(pub u32);
    impl POSD {
        #[inline(always)]
        pub const fn POSD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for POSD {
        #[inline(always)]
        fn default() -> POSD {
            POSD(0)
        }
    }
    impl core::fmt::Debug for POSD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POSD").field("POSD", &self.POSD()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POSD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct POSD {
                POSD: u16,
            }
            let proxy = POSD { POSD: self.POSD() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Position Difference Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POSDH(pub u32);
    impl POSDH {
        #[inline(always)]
        pub const fn POSDH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSDH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for POSDH {
        #[inline(always)]
        fn default() -> POSDH {
            POSDH(0)
        }
    }
    impl core::fmt::Debug for POSDH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POSDH")
                .field("POSDH", &self.POSDH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POSDH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct POSDH {
                POSDH: u16,
            }
            let proxy = POSDH {
                POSDH: self.POSDH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Position Difference Period Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POSDPER(pub u32);
    impl POSDPER {
        #[inline(always)]
        pub const fn POSDPER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSDPER(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for POSDPER {
        #[inline(always)]
        fn default() -> POSDPER {
            POSDPER(0)
        }
    }
    impl core::fmt::Debug for POSDPER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POSDPER")
                .field("POSDPER", &self.POSDPER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POSDPER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct POSDPER {
                POSDPER: u16,
            }
            let proxy = POSDPER {
                POSDPER: self.POSDPER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Position Difference Period Buffer Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POSDPERBFR(pub u32);
    impl POSDPERBFR {
        #[inline(always)]
        pub const fn POSDPERBFR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSDPERBFR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for POSDPERBFR {
        #[inline(always)]
        fn default() -> POSDPERBFR {
            POSDPERBFR(0)
        }
    }
    impl core::fmt::Debug for POSDPERBFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POSDPERBFR")
                .field("POSDPERBFR", &self.POSDPERBFR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POSDPERBFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct POSDPERBFR {
                POSDPERBFR: u16,
            }
            let proxy = POSDPERBFR {
                POSDPERBFR: self.POSDPERBFR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Position Difference Period Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POSDPERH(pub u32);
    impl POSDPERH {
        #[inline(always)]
        pub const fn POSDPERH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSDPERH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for POSDPERH {
        #[inline(always)]
        fn default() -> POSDPERH {
            POSDPERH(0)
        }
    }
    impl core::fmt::Debug for POSDPERH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POSDPERH")
                .field("POSDPERH", &self.POSDPERH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POSDPERH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct POSDPERH {
                POSDPERH: u16,
            }
            let proxy = POSDPERH {
                POSDPERH: self.POSDPERH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Revolution Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REV(pub u32);
    impl REV {
        #[inline(always)]
        pub const fn REV(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REV(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for REV {
        #[inline(always)]
        fn default() -> REV {
            REV(0)
        }
    }
    impl core::fmt::Debug for REV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REV").field("REV", &self.REV()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REV {
                REV: u16,
            }
            let proxy = REV { REV: self.REV() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Revolution Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REVH(pub u32);
    impl REVH {
        #[inline(always)]
        pub const fn REVH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REVH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for REVH {
        #[inline(always)]
        fn default() -> REVH {
            REVH(0)
        }
    }
    impl core::fmt::Debug for REVH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REVH").field("REVH", &self.REVH()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REVH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REVH {
                REVH: u16,
            }
            let proxy = REVH { REVH: self.REVH() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TST(pub u32);
    impl TST {
        #[inline(always)]
        pub const fn TEST_COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEST_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TEST_PERIOD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEST_PERIOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn QDN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn TCE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn TEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for TST {
        #[inline(always)]
        fn default() -> TST {
            TST(0)
        }
    }
    impl core::fmt::Debug for TST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TST")
                .field("TEST_COUNT", &self.TEST_COUNT())
                .field("TEST_PERIOD", &self.TEST_PERIOD())
                .field("QDN", &self.QDN())
                .field("TCE", &self.TCE())
                .field("TEN", &self.TEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TST {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TST {
                TEST_COUNT: u8,
                TEST_PERIOD: u8,
                QDN: bool,
                TCE: bool,
                TEN: bool,
            }
            let proxy = TST {
                TEST_COUNT: self.TEST_COUNT(),
                TEST_PERIOD: self.TEST_PERIOD(),
                QDN: self.QDN(),
                TCE: self.TCE(),
                TEN: self.TEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Compare Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UCOMP0(pub u32);
    impl UCOMP0 {
        #[inline(always)]
        pub const fn UCOMP0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UCOMP0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UCOMP0 {
        #[inline(always)]
        fn default() -> UCOMP0 {
            UCOMP0(0)
        }
    }
    impl core::fmt::Debug for UCOMP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UCOMP0")
                .field("UCOMP0", &self.UCOMP0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UCOMP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UCOMP0 {
                UCOMP0: u16,
            }
            let proxy = UCOMP0 {
                UCOMP0: self.UCOMP0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Compare 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UCOMP1(pub u32);
    impl UCOMP1 {
        #[inline(always)]
        pub const fn UCOMP1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UCOMP1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UCOMP1 {
        #[inline(always)]
        fn default() -> UCOMP1 {
            UCOMP1(0)
        }
    }
    impl core::fmt::Debug for UCOMP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UCOMP1")
                .field("UCOMP1", &self.UCOMP1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UCOMP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UCOMP1 {
                UCOMP1: u16,
            }
            let proxy = UCOMP1 {
                UCOMP1: self.UCOMP1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Compare 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UCOMP2(pub u32);
    impl UCOMP2 {
        #[inline(always)]
        pub const fn UCOMP2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UCOMP2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UCOMP2 {
        #[inline(always)]
        fn default() -> UCOMP2 {
            UCOMP2(0)
        }
    }
    impl core::fmt::Debug for UCOMP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UCOMP2")
                .field("UCOMP2", &self.UCOMP2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UCOMP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UCOMP2 {
                UCOMP2: u16,
            }
            let proxy = UCOMP2 {
                UCOMP2: self.UCOMP2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Compare 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UCOMP3(pub u32);
    impl UCOMP3 {
        #[inline(always)]
        pub const fn UCOMP3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UCOMP3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UCOMP3 {
        #[inline(always)]
        fn default() -> UCOMP3 {
            UCOMP3(0)
        }
    }
    impl core::fmt::Debug for UCOMP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UCOMP3")
                .field("UCOMP3", &self.UCOMP3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UCOMP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UCOMP3 {
                UCOMP3: u16,
            }
            let proxy = UCOMP3 {
                UCOMP3: self.UCOMP3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Initialization Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UINIT(pub u32);
    impl UINIT {
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
    impl Default for UINIT {
        #[inline(always)]
        fn default() -> UINIT {
            UINIT(0)
        }
    }
    impl core::fmt::Debug for UINIT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UINIT").field("INIT", &self.INIT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UINIT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UINIT {
                INIT: u16,
            }
            let proxy = UINIT { INIT: self.INIT() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Modulus Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UMOD(pub u32);
    impl UMOD {
        #[inline(always)]
        pub const fn MOD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MOD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UMOD {
        #[inline(always)]
        fn default() -> UMOD {
            UMOD(0)
        }
    }
    impl core::fmt::Debug for UMOD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UMOD").field("MOD", &self.MOD()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UMOD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UMOD {
                MOD: u16,
            }
            let proxy = UMOD { MOD: self.MOD() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Counter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPOS(pub u32);
    impl UPOS {
        #[inline(always)]
        pub const fn POS(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POS(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UPOS {
        #[inline(always)]
        fn default() -> UPOS {
            UPOS(0)
        }
    }
    impl core::fmt::Debug for UPOS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPOS").field("POS", &self.POS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPOS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UPOS {
                POS: u16,
            }
            let proxy = UPOS { POS: self.POS() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Hold Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPOSH(pub u32);
    impl UPOSH {
        #[inline(always)]
        pub const fn POSH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_POSH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UPOSH {
        #[inline(always)]
        fn default() -> UPOSH {
            UPOSH(0)
        }
    }
    impl core::fmt::Debug for UPOSH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPOSH").field("POSH", &self.POSH()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPOSH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UPOSH {
                POSH: u16,
            }
            let proxy = UPOSH { POSH: self.POSH() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Holder Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPOSH1(pub u32);
    impl UPOSH1 {
        #[inline(always)]
        pub const fn UPOSH1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UPOSH1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UPOSH1 {
        #[inline(always)]
        fn default() -> UPOSH1 {
            UPOSH1(0)
        }
    }
    impl core::fmt::Debug for UPOSH1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPOSH1")
                .field("UPOSH1", &self.UPOSH1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPOSH1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UPOSH1 {
                UPOSH1: u16,
            }
            let proxy = UPOSH1 {
                UPOSH1: self.UPOSH1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Holder Register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPOSH2(pub u32);
    impl UPOSH2 {
        #[inline(always)]
        pub const fn UPOSH2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UPOSH2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UPOSH2 {
        #[inline(always)]
        fn default() -> UPOSH2 {
            UPOSH2(0)
        }
    }
    impl core::fmt::Debug for UPOSH2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPOSH2")
                .field("UPOSH2", &self.UPOSH2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPOSH2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UPOSH2 {
                UPOSH2: u16,
            }
            let proxy = UPOSH2 {
                UPOSH2: self.UPOSH2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper Position Holder Register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPOSH3(pub u32);
    impl UPOSH3 {
        #[inline(always)]
        pub const fn UPOSH3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UPOSH3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UPOSH3 {
        #[inline(always)]
        fn default() -> UPOSH3 {
            UPOSH3(0)
        }
    }
    impl core::fmt::Debug for UPOSH3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPOSH3")
                .field("UPOSH3", &self.UPOSH3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPOSH3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UPOSH3 {
                UPOSH3: u16,
            }
            let proxy = UPOSH3 {
                UPOSH3: self.UPOSH3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Upper VERID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UVERID(pub u32);
    impl UVERID {
        #[inline(always)]
        pub const fn UVERID(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_UVERID(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UVERID {
        #[inline(always)]
        fn default() -> UVERID {
            UVERID(0)
        }
    }
    impl core::fmt::Debug for UVERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UVERID")
                .field("UVERID", &self.UVERID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UVERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UVERID {
                UVERID: u16,
            }
            let proxy = UVERID {
                UVERID: self.UVERID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Watchdog Timeout Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WTR(pub u32);
    impl WTR {
        #[inline(always)]
        pub const fn WDOG(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_WDOG(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for WTR {
        #[inline(always)]
        fn default() -> WTR {
            WTR(0)
        }
    }
    impl core::fmt::Debug for WTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WTR").field("WDOG", &self.WDOG()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WTR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WTR {
                WDOG: u16,
            }
            let proxy = WTR { WDOG: self.WDOG() };
            defmt::write!(f, "{}", proxy)
        }
    }
}
