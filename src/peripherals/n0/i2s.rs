#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2S {
    ptr: *mut u8,
}
unsafe impl Send for I2S {}
unsafe impl Sync for I2S {}
impl I2S {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<regs::VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TCSR(self) -> crate::common::Reg<regs::TCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR1(self) -> crate::common::Reg<regs::TCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn TCR2(self) -> crate::common::Reg<regs::TCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR3(self) -> crate::common::Reg<regs::TCR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR4(self) -> crate::common::Reg<regs::TCR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR5(self) -> crate::common::Reg<regs::TCR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn TDR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TFR(self, n: usize) -> crate::common::Reg<regs::TFR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TMR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn RCSR(self) -> crate::common::Reg<regs::RCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR1(self) -> crate::common::Reg<regs::RCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn RCR2(self) -> crate::common::Reg<regs::RCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR3(self) -> crate::common::Reg<regs::RCR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR4(self) -> crate::common::Reg<regs::RCR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR5(self) -> crate::common::Reg<regs::RCR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn RDR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RFR(self, n: usize) -> crate::common::Reg<regs::RFR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RMR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
}
pub mod regs {
    #[doc = "MCLK Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DIVEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIVEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn MOE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("DIV", &self.DIV())
                .field("DIVEN", &self.DIVEN())
                .field("MSEL", &self.MSEL())
                .field("MOE", &self.MOE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCR {
                DIV: u8,
                DIVEN: bool,
                MSEL: u8,
                MOE: bool,
            }
            let proxy = MCR {
                DIV: self.DIV(),
                DIVEN: self.DIVEN(),
                MSEL: self.MSEL(),
                MOE: self.MOE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn DATALINE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATALINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FIFO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn FRAME(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRAME(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
                .field("DATALINE", &self.DATALINE())
                .field("FIFO", &self.FIFO())
                .field("FRAME", &self.FRAME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PARAM {
                DATALINE: u8,
                FIFO: u8,
                FRAME: u8,
            }
            let proxy = PARAM {
                DATALINE: self.DATALINE(),
                FIFO: self.FIFO(),
                FRAME: self.FRAME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR1(pub u32);
    impl RCR1 {
        #[inline(always)]
        pub const fn RFW(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for RCR1 {
        #[inline(always)]
        fn default() -> RCR1 {
            RCR1(0)
        }
    }
    impl core::fmt::Debug for RCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR1").field("RFW", &self.RFW()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCR1 {
                RFW: u8,
            }
            let proxy = RCR1 { RFW: self.RFW() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR2(pub u32);
    impl RCR2 {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn BYP(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn BCD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn BCP(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn BCI(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn BCS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn SYNC(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for RCR2 {
        #[inline(always)]
        fn default() -> RCR2 {
            RCR2(0)
        }
    }
    impl core::fmt::Debug for RCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR2")
                .field("DIV", &self.DIV())
                .field("BYP", &self.BYP())
                .field("BCD", &self.BCD())
                .field("BCP", &self.BCP())
                .field("MSEL", &self.MSEL())
                .field("BCI", &self.BCI())
                .field("BCS", &self.BCS())
                .field("SYNC", &self.SYNC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCR2 {
                DIV: u8,
                BYP: bool,
                BCD: bool,
                BCP: bool,
                MSEL: u8,
                BCI: bool,
                BCS: bool,
                SYNC: u8,
            }
            let proxy = RCR2 {
                DIV: self.DIV(),
                BYP: self.BYP(),
                BCD: self.BCD(),
                BCP: self.BCP(),
                MSEL: self.MSEL(),
                BCI: self.BCI(),
                BCS: self.BCS(),
                SYNC: self.SYNC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Configuration 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR3(pub u32);
    impl RCR3 {
        #[inline(always)]
        pub const fn WDFL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WDFL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn RCE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn CFR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CFR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for RCR3 {
        #[inline(always)]
        fn default() -> RCR3 {
            RCR3(0)
        }
    }
    impl core::fmt::Debug for RCR3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR3")
                .field("WDFL", &self.WDFL())
                .field("RCE", &self.RCE())
                .field("CFR", &self.CFR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCR3 {
                WDFL: u8,
                RCE: u8,
                CFR: u8,
            }
            let proxy = RCR3 {
                WDFL: self.WDFL(),
                RCE: self.RCE(),
                CFR: self.CFR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Configuration 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR4(pub u32);
    impl RCR4 {
        #[inline(always)]
        pub const fn FSD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FSP(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ONDEM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ONDEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FSE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn MF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn SYWD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYWD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn FRSZ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn FPACK(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FPACK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn FCOMB(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FCOMB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn FCONT(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FCONT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for RCR4 {
        #[inline(always)]
        fn default() -> RCR4 {
            RCR4(0)
        }
    }
    impl core::fmt::Debug for RCR4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR4")
                .field("FSD", &self.FSD())
                .field("FSP", &self.FSP())
                .field("ONDEM", &self.ONDEM())
                .field("FSE", &self.FSE())
                .field("MF", &self.MF())
                .field("SYWD", &self.SYWD())
                .field("FRSZ", &self.FRSZ())
                .field("FPACK", &self.FPACK())
                .field("FCOMB", &self.FCOMB())
                .field("FCONT", &self.FCONT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCR4 {
                FSD: bool,
                FSP: bool,
                ONDEM: bool,
                FSE: bool,
                MF: bool,
                SYWD: u8,
                FRSZ: u8,
                FPACK: u8,
                FCOMB: u8,
                FCONT: bool,
            }
            let proxy = RCR4 {
                FSD: self.FSD(),
                FSP: self.FSP(),
                ONDEM: self.ONDEM(),
                FSE: self.FSE(),
                MF: self.MF(),
                SYWD: self.SYWD(),
                FRSZ: self.FRSZ(),
                FPACK: self.FPACK(),
                FCOMB: self.FCOMB(),
                FCONT: self.FCONT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Configuration 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR5(pub u32);
    impl RCR5 {
        #[inline(always)]
        pub const fn FBT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FBT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn W0W(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_W0W(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn WNW(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WNW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for RCR5 {
        #[inline(always)]
        fn default() -> RCR5 {
            RCR5(0)
        }
    }
    impl core::fmt::Debug for RCR5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR5")
                .field("FBT", &self.FBT())
                .field("W0W", &self.W0W())
                .field("WNW", &self.WNW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCR5 {
                FBT: u8,
                W0W: u8,
                WNW: u8,
            }
            let proxy = RCR5 {
                FBT: self.FBT(),
                W0W: self.W0W(),
                WNW: self.WNW(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCSR(pub u32);
    impl RCSR {
        #[inline(always)]
        pub const fn FRDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FWDE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FWIE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FEIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WSIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FRF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FWF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FEF(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn SEF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn WSF(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn SR(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn FR(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn BCE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn DBGE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn STOPE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOPE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn RE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RCSR {
        #[inline(always)]
        fn default() -> RCSR {
            RCSR(0)
        }
    }
    impl core::fmt::Debug for RCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCSR")
                .field("FRDE", &self.FRDE())
                .field("FWDE", &self.FWDE())
                .field("FRIE", &self.FRIE())
                .field("FWIE", &self.FWIE())
                .field("FEIE", &self.FEIE())
                .field("SEIE", &self.SEIE())
                .field("WSIE", &self.WSIE())
                .field("FRF", &self.FRF())
                .field("FWF", &self.FWF())
                .field("FEF", &self.FEF())
                .field("SEF", &self.SEF())
                .field("WSF", &self.WSF())
                .field("SR", &self.SR())
                .field("FR", &self.FR())
                .field("BCE", &self.BCE())
                .field("DBGE", &self.DBGE())
                .field("STOPE", &self.STOPE())
                .field("RE", &self.RE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RCSR {
                FRDE: bool,
                FWDE: bool,
                FRIE: bool,
                FWIE: bool,
                FEIE: bool,
                SEIE: bool,
                WSIE: bool,
                FRF: bool,
                FWF: bool,
                FEF: bool,
                SEF: bool,
                WSF: bool,
                SR: bool,
                FR: bool,
                BCE: bool,
                DBGE: bool,
                STOPE: bool,
                RE: bool,
            }
            let proxy = RCSR {
                FRDE: self.FRDE(),
                FWDE: self.FWDE(),
                FRIE: self.FRIE(),
                FWIE: self.FWIE(),
                FEIE: self.FEIE(),
                SEIE: self.SEIE(),
                WSIE: self.WSIE(),
                FRF: self.FRF(),
                FWF: self.FWF(),
                FEF: self.FEF(),
                SEF: self.SEF(),
                WSF: self.WSF(),
                SR: self.SR(),
                FR: self.FR(),
                BCE: self.BCE(),
                DBGE: self.DBGE(),
                STOPE: self.STOPE(),
                RE: self.RE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Receive FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RFR(pub u32);
    impl RFR {
        #[inline(always)]
        pub const fn RFP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RCP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn WFP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WFP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for RFR {
        #[inline(always)]
        fn default() -> RFR {
            RFR(0)
        }
    }
    impl core::fmt::Debug for RFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RFR")
                .field("RFP", &self.RFP())
                .field("RCP", &self.RCP())
                .field("WFP", &self.WFP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RFR {
                RFP: u8,
                RCP: bool,
                WFP: u8,
            }
            let proxy = RFR {
                RFP: self.RFP(),
                RCP: self.RCP(),
                WFP: self.WFP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR1(pub u32);
    impl TCR1 {
        #[inline(always)]
        pub const fn TFW(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TFW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for TCR1 {
        #[inline(always)]
        fn default() -> TCR1 {
            TCR1(0)
        }
    }
    impl core::fmt::Debug for TCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR1").field("TFW", &self.TFW()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCR1 {
                TFW: u8,
            }
            let proxy = TCR1 { TFW: self.TFW() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR2(pub u32);
    impl TCR2 {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn BYP(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn BCD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn BCP(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn BCI(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn BCS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn SYNC(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for TCR2 {
        #[inline(always)]
        fn default() -> TCR2 {
            TCR2(0)
        }
    }
    impl core::fmt::Debug for TCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR2")
                .field("DIV", &self.DIV())
                .field("BYP", &self.BYP())
                .field("BCD", &self.BCD())
                .field("BCP", &self.BCP())
                .field("MSEL", &self.MSEL())
                .field("BCI", &self.BCI())
                .field("BCS", &self.BCS())
                .field("SYNC", &self.SYNC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCR2 {
                DIV: u8,
                BYP: bool,
                BCD: bool,
                BCP: bool,
                MSEL: u8,
                BCI: bool,
                BCS: bool,
                SYNC: u8,
            }
            let proxy = TCR2 {
                DIV: self.DIV(),
                BYP: self.BYP(),
                BCD: self.BCD(),
                BCP: self.BCP(),
                MSEL: self.MSEL(),
                BCI: self.BCI(),
                BCS: self.BCS(),
                SYNC: self.SYNC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Configuration 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR3(pub u32);
    impl TCR3 {
        #[inline(always)]
        pub const fn WDFL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WDFL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn TCE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn CFR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CFR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for TCR3 {
        #[inline(always)]
        fn default() -> TCR3 {
            TCR3(0)
        }
    }
    impl core::fmt::Debug for TCR3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR3")
                .field("WDFL", &self.WDFL())
                .field("TCE", &self.TCE())
                .field("CFR", &self.CFR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCR3 {
                WDFL: u8,
                TCE: u8,
                CFR: u8,
            }
            let proxy = TCR3 {
                WDFL: self.WDFL(),
                TCE: self.TCE(),
                CFR: self.CFR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Configuration 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR4(pub u32);
    impl TCR4 {
        #[inline(always)]
        pub const fn FSD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FSP(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ONDEM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ONDEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FSE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn MF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHMOD(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SYWD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYWD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn FRSZ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn FPACK(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FPACK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn FCOMB(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FCOMB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn FCONT(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FCONT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for TCR4 {
        #[inline(always)]
        fn default() -> TCR4 {
            TCR4(0)
        }
    }
    impl core::fmt::Debug for TCR4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR4")
                .field("FSD", &self.FSD())
                .field("FSP", &self.FSP())
                .field("ONDEM", &self.ONDEM())
                .field("FSE", &self.FSE())
                .field("MF", &self.MF())
                .field("CHMOD", &self.CHMOD())
                .field("SYWD", &self.SYWD())
                .field("FRSZ", &self.FRSZ())
                .field("FPACK", &self.FPACK())
                .field("FCOMB", &self.FCOMB())
                .field("FCONT", &self.FCONT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCR4 {
                FSD: bool,
                FSP: bool,
                ONDEM: bool,
                FSE: bool,
                MF: bool,
                CHMOD: bool,
                SYWD: u8,
                FRSZ: u8,
                FPACK: u8,
                FCOMB: u8,
                FCONT: bool,
            }
            let proxy = TCR4 {
                FSD: self.FSD(),
                FSP: self.FSP(),
                ONDEM: self.ONDEM(),
                FSE: self.FSE(),
                MF: self.MF(),
                CHMOD: self.CHMOD(),
                SYWD: self.SYWD(),
                FRSZ: self.FRSZ(),
                FPACK: self.FPACK(),
                FCOMB: self.FCOMB(),
                FCONT: self.FCONT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Configuration 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR5(pub u32);
    impl TCR5 {
        #[inline(always)]
        pub const fn FBT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FBT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn W0W(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_W0W(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn WNW(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WNW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for TCR5 {
        #[inline(always)]
        fn default() -> TCR5 {
            TCR5(0)
        }
    }
    impl core::fmt::Debug for TCR5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR5")
                .field("FBT", &self.FBT())
                .field("W0W", &self.W0W())
                .field("WNW", &self.WNW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCR5 {
                FBT: u8,
                W0W: u8,
                WNW: u8,
            }
            let proxy = TCR5 {
                FBT: self.FBT(),
                W0W: self.W0W(),
                WNW: self.WNW(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCSR(pub u32);
    impl TCSR {
        #[inline(always)]
        pub const fn FRDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FWDE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FWIE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FEIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WSIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FRF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FWF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FEF(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn SEF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn WSF(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn SR(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn FR(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn BCE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn DBGE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn STOPE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOPE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn TE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TCSR {
        #[inline(always)]
        fn default() -> TCSR {
            TCSR(0)
        }
    }
    impl core::fmt::Debug for TCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCSR")
                .field("FRDE", &self.FRDE())
                .field("FWDE", &self.FWDE())
                .field("FRIE", &self.FRIE())
                .field("FWIE", &self.FWIE())
                .field("FEIE", &self.FEIE())
                .field("SEIE", &self.SEIE())
                .field("WSIE", &self.WSIE())
                .field("FRF", &self.FRF())
                .field("FWF", &self.FWF())
                .field("FEF", &self.FEF())
                .field("SEF", &self.SEF())
                .field("WSF", &self.WSF())
                .field("SR", &self.SR())
                .field("FR", &self.FR())
                .field("BCE", &self.BCE())
                .field("DBGE", &self.DBGE())
                .field("STOPE", &self.STOPE())
                .field("TE", &self.TE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TCSR {
                FRDE: bool,
                FWDE: bool,
                FRIE: bool,
                FWIE: bool,
                FEIE: bool,
                SEIE: bool,
                WSIE: bool,
                FRF: bool,
                FWF: bool,
                FEF: bool,
                SEF: bool,
                WSF: bool,
                SR: bool,
                FR: bool,
                BCE: bool,
                DBGE: bool,
                STOPE: bool,
                TE: bool,
            }
            let proxy = TCSR {
                FRDE: self.FRDE(),
                FWDE: self.FWDE(),
                FRIE: self.FRIE(),
                FWIE: self.FWIE(),
                FEIE: self.FEIE(),
                SEIE: self.SEIE(),
                WSIE: self.WSIE(),
                FRF: self.FRF(),
                FWF: self.FWF(),
                FEF: self.FEF(),
                SEF: self.SEF(),
                WSF: self.WSF(),
                SR: self.SR(),
                FR: self.FR(),
                BCE: self.BCE(),
                DBGE: self.DBGE(),
                STOPE: self.STOPE(),
                TE: self.TE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TFR(pub u32);
    impl TFR {
        #[inline(always)]
        pub const fn RFP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn WFP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WFP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn WCP(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TFR {
        #[inline(always)]
        fn default() -> TFR {
            TFR(0)
        }
    }
    impl core::fmt::Debug for TFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TFR")
                .field("RFP", &self.RFP())
                .field("WFP", &self.WFP())
                .field("WCP", &self.WCP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TFR {
                RFP: u8,
                WFP: u8,
                WCP: bool,
            }
            let proxy = TFR {
                RFP: self.RFP(),
                WFP: self.WFP(),
                WCP: self.WCP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for VERID {
        #[inline(always)]
        fn default() -> VERID {
            VERID(0)
        }
    }
    impl core::fmt::Debug for VERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERID")
                .field("FEATURE", &self.FEATURE())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VERID {
                FEATURE: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERID {
                FEATURE: self.FEATURE(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
