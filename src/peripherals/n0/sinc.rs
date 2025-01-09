#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHANNEL {
    ptr: *mut u8,
}
unsafe impl Send for CHANNEL {}
unsafe impl Sync for CHANNEL {}
impl CHANNEL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CCR(self) -> crate::common::Reg<regs::CHANNEL_CCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CDR(self) -> crate::common::Reg<regs::CHANNEL_CDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CCFR(self) -> crate::common::Reg<regs::CHANNEL_CCFR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CPROT(self) -> crate::common::Reg<regs::CHANNEL_CPROT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CBIAS(self) -> crate::common::Reg<regs::CHANNEL_CBIAS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CLOLMT(self) -> crate::common::Reg<regs::CHANNEL_CLOLMT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CHILMT(self) -> crate::common::Reg<regs::CHANNEL_CHILMT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn CRDATA(self) -> crate::common::Reg<regs::CHANNEL_CRDATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMPDATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CACFR(self) -> crate::common::Reg<regs::CHANNEL_CACFR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<regs::CHANNEL_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn CDBGR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SINC {
    ptr: *mut u8,
}
unsafe impl Send for SINC {}
unsafe impl Sync for SINC {}
impl SINC {
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
    pub const fn PARAMETER(self) -> crate::common::Reg<regs::PARAMETER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn NIE(self) -> crate::common::Reg<regs::NIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn EIE(self) -> crate::common::Reg<regs::EIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn FIFOIE(self) -> crate::common::Reg<regs::FIFOIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn NIS(self) -> crate::common::Reg<regs::NIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn EIS(self) -> crate::common::Reg<regs::EIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIFOIS(self) -> crate::common::Reg<regs::FIFOIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<regs::SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn CHANNEL(self, n: usize) -> CHANNEL {
        assert!(n < 5usize);
        unsafe { CHANNEL::from_ptr(self.ptr.add(0x38usize + n * 48usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel 0 Advanced Configuration..Channel 4 Advanced Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CACFR(pub u32);
    impl CHANNEL_CACFR {
        #[inline(always)]
        pub const fn ADMASEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ADMASEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn HPFA(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_HPFA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn IBDLY(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IBDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for CHANNEL_CACFR {
        #[inline(always)]
        fn default() -> CHANNEL_CACFR {
            CHANNEL_CACFR(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CACFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CACFR")
                .field("ADMASEL", &self.ADMASEL())
                .field("HPFA", &self.HPFA())
                .field("IBDLY", &self.IBDLY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CACFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CACFR {
                ADMASEL: u8,
                HPFA: u8,
                IBDLY: u8,
            }
            let proxy = CHANNEL_CACFR {
                ADMASEL: self.ADMASEL(),
                HPFA: self.HPFA(),
                IBDLY: self.IBDLY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Bias..Channel 4 Bias"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CBIAS(pub u32);
    impl CHANNEL_CBIAS {
        #[inline(always)]
        pub const fn BIAS(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_BIAS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CHANNEL_CBIAS {
        #[inline(always)]
        fn default() -> CHANNEL_CBIAS {
            CHANNEL_CBIAS(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CBIAS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CBIAS")
                .field("BIAS", &self.BIAS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CBIAS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CBIAS {
                BIAS: u32,
            }
            let proxy = CHANNEL_CBIAS { BIAS: self.BIAS() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Configuration..Channel 4 Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CCFR(pub u32);
    impl CHANNEL_CCFR {
        #[inline(always)]
        pub const fn PFSFT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFSFT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn RDFMT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDFMT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FIFOWMK(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFOWMK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[inline(always)]
        pub const fn IBFMT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IBFMT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn ICSEL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ICSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[inline(always)]
        pub const fn ICESEL(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ICESEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[inline(always)]
        pub const fn ITSEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ITSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn IBSEL(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IBSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn ITLVL(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ITLVL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ZCOP(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ZCOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CHANNEL_CCFR {
        #[inline(always)]
        fn default() -> CHANNEL_CCFR {
            CHANNEL_CCFR(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CCFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CCFR")
                .field("PFSFT", &self.PFSFT())
                .field("RDFMT", &self.RDFMT())
                .field("FIFOWMK", &self.FIFOWMK())
                .field("IBFMT", &self.IBFMT())
                .field("ICSEL", &self.ICSEL())
                .field("ICESEL", &self.ICESEL())
                .field("ITSEL", &self.ITSEL())
                .field("IBSEL", &self.IBSEL())
                .field("ITLVL", &self.ITLVL())
                .field("ZCOP", &self.ZCOP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CCFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CCFR {
                PFSFT: u8,
                RDFMT: bool,
                FIFOWMK: u8,
                IBFMT: u8,
                ICSEL: u8,
                ICESEL: u8,
                ITSEL: u8,
                IBSEL: u8,
                ITLVL: bool,
                ZCOP: u8,
            }
            let proxy = CHANNEL_CCFR {
                PFSFT: self.PFSFT(),
                RDFMT: self.RDFMT(),
                FIFOWMK: self.FIFOWMK(),
                IBFMT: self.IBFMT(),
                ICSEL: self.ICSEL(),
                ICESEL: self.ICESEL(),
                ITSEL: self.ITSEL(),
                IBSEL: self.IBSEL(),
                ITLVL: self.ITLVL(),
                ZCOP: self.ZCOP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Control..Channel 4 Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CCR(pub u32);
    impl CHANNEL_CCR {
        #[inline(always)]
        pub const fn CHEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DMAEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SCDEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CADEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ZCDEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn LMTEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LMTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn FIFOEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DBGSEL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DBGSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for CHANNEL_CCR {
        #[inline(always)]
        fn default() -> CHANNEL_CCR {
            CHANNEL_CCR(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CCR")
                .field("CHEN", &self.CHEN())
                .field("PFEN", &self.PFEN())
                .field("DMAEN", &self.DMAEN())
                .field("SCDEN", &self.SCDEN())
                .field("CADEN", &self.CADEN())
                .field("ZCDEN", &self.ZCDEN())
                .field("LMTEN", &self.LMTEN())
                .field("FIFOEN", &self.FIFOEN())
                .field("DBGSEL", &self.DBGSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CCR {
                CHEN: bool,
                PFEN: bool,
                DMAEN: bool,
                SCDEN: bool,
                CADEN: bool,
                ZCDEN: bool,
                LMTEN: bool,
                FIFOEN: bool,
                DBGSEL: u8,
            }
            let proxy = CHANNEL_CCR {
                CHEN: self.CHEN(),
                PFEN: self.PFEN(),
                DMAEN: self.DMAEN(),
                SCDEN: self.SCDEN(),
                CADEN: self.CADEN(),
                ZCDEN: self.ZCDEN(),
                LMTEN: self.LMTEN(),
                FIFOEN: self.FIFOEN(),
                DBGSEL: self.DBGSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Data Rate..Channel 4 Data Rate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CDR(pub u32);
    impl CHANNEL_CDR {
        #[inline(always)]
        pub const fn PFOSR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PFOSR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn PFORD(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFORD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[inline(always)]
        pub const fn PFCM(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFCM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for CHANNEL_CDR {
        #[inline(always)]
        fn default() -> CHANNEL_CDR {
            CHANNEL_CDR(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CDR")
                .field("PFOSR", &self.PFOSR())
                .field("PFORD", &self.PFORD())
                .field("PFCM", &self.PFCM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CDR {
                PFOSR: u16,
                PFORD: u8,
                PFCM: u8,
            }
            let proxy = CHANNEL_CDR {
                PFOSR: self.PFOSR(),
                PFORD: self.PFORD(),
                PFCM: self.PFCM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 High Limit..Channel 4 High Limit"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CHILMT(pub u32);
    impl CHANNEL_CHILMT {
        #[inline(always)]
        pub const fn HILMT(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_HILMT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CHANNEL_CHILMT {
        #[inline(always)]
        fn default() -> CHANNEL_CHILMT {
            CHANNEL_CHILMT(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CHILMT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CHILMT")
                .field("HILMT", &self.HILMT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CHILMT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CHILMT {
                HILMT: u32,
            }
            let proxy = CHANNEL_CHILMT {
                HILMT: self.HILMT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Low Limit..Channel 4 Low Limit"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CLOLMT(pub u32);
    impl CHANNEL_CLOLMT {
        #[inline(always)]
        pub const fn LOLMT(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_LOLMT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CHANNEL_CLOLMT {
        #[inline(always)]
        fn default() -> CHANNEL_CLOLMT {
            CHANNEL_CLOLMT(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CLOLMT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CLOLMT")
                .field("LOLMT", &self.LOLMT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CLOLMT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CLOLMT {
                LOLMT: u32,
            }
            let proxy = CHANNEL_CLOLMT {
                LOLMT: self.LOLMT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Protection..Channel 4 Protection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CPROT(pub u32);
    impl CHANNEL_CPROT {
        #[inline(always)]
        pub const fn SCDLMT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SCDLMT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn SCDCM(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn SCDOP(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SCDOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn LMTOP(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LMTOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn CADLMT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CADLMT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn CADBK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SCDBK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn LLMTBK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn WLMTBK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn HLMTBK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CHANNEL_CPROT {
        #[inline(always)]
        fn default() -> CHANNEL_CPROT {
            CHANNEL_CPROT(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CPROT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CPROT")
                .field("SCDLMT", &self.SCDLMT())
                .field("SCDCM", &self.SCDCM())
                .field("SCDOP", &self.SCDOP())
                .field("LMTOP", &self.LMTOP())
                .field("CADLMT", &self.CADLMT())
                .field("CADBK", &self.CADBK())
                .field("SCDBK", &self.SCDBK())
                .field("LLMTBK", &self.LLMTBK())
                .field("WLMTBK", &self.WLMTBK())
                .field("HLMTBK", &self.HLMTBK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CPROT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CPROT {
                SCDLMT: u8,
                SCDCM: bool,
                SCDOP: u8,
                LMTOP: u8,
                CADLMT: u8,
                CADBK: bool,
                SCDBK: bool,
                LLMTBK: bool,
                WLMTBK: bool,
                HLMTBK: bool,
            }
            let proxy = CHANNEL_CPROT {
                SCDLMT: self.SCDLMT(),
                SCDCM: self.SCDCM(),
                SCDOP: self.SCDOP(),
                LMTOP: self.LMTOP(),
                CADLMT: self.CADLMT(),
                CADBK: self.CADBK(),
                SCDBK: self.SCDBK(),
                LLMTBK: self.LLMTBK(),
                WLMTBK: self.WLMTBK(),
                HLMTBK: self.HLMTBK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Result Data..Channel 4 Result Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CRDATA(pub u32);
    impl CHANNEL_CRDATA {
        #[inline(always)]
        pub const fn RDATA(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RDATA(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CHANNEL_CRDATA {
        #[inline(always)]
        fn default() -> CHANNEL_CRDATA {
            CHANNEL_CRDATA(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CRDATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CRDATA")
                .field("RDATA", &self.RDATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CRDATA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CRDATA {
                RDATA: u32,
            }
            let proxy = CHANNEL_CRDATA {
                RDATA: self.RDATA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Channel 0 Status..Channel 4 Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CSR(pub u32);
    impl CHANNEL_CSR {
        #[inline(always)]
        pub const fn FIFOAVIL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFOAVIL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn PSRDY(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PSRDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn PFSAT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFSAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn HPFSAT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HPFSAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SFTSAT(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFTSAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn BIASSAT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIASSAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn RDRS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn SRDS(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRDS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DBGRS(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DBGRS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn CNUM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[inline(always)]
        pub const fn CNUM_OV(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CNUM_OV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for CHANNEL_CSR {
        #[inline(always)]
        fn default() -> CHANNEL_CSR {
            CHANNEL_CSR(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CSR")
                .field("FIFOAVIL", &self.FIFOAVIL())
                .field("PSRDY", &self.PSRDY())
                .field("PFSAT", &self.PFSAT())
                .field("HPFSAT", &self.HPFSAT())
                .field("SFTSAT", &self.SFTSAT())
                .field("BIASSAT", &self.BIASSAT())
                .field("RDRS", &self.RDRS())
                .field("SRDS", &self.SRDS())
                .field("DBGRS", &self.DBGRS())
                .field("CNUM", &self.CNUM())
                .field("CNUM_OV", &self.CNUM_OV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CSR {
                FIFOAVIL: u8,
                PSRDY: bool,
                PFSAT: bool,
                HPFSAT: bool,
                SFTSAT: bool,
                BIASSAT: bool,
                RDRS: bool,
                SRDS: bool,
                DBGRS: u8,
                CNUM: u8,
                CNUM_OV: bool,
            }
            let proxy = CHANNEL_CSR {
                FIFOAVIL: self.FIFOAVIL(),
                PSRDY: self.PSRDY(),
                PFSAT: self.PFSAT(),
                HPFSAT: self.HPFSAT(),
                SFTSAT: self.SFTSAT(),
                BIASSAT: self.BIASSAT(),
                RDRS: self.RDRS(),
                SRDS: self.SRDS(),
                DBGRS: self.DBGRS(),
                CNUM: self.CNUM(),
                CNUM_OV: self.CNUM_OV(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EIE(pub u32);
    impl EIE {
        #[inline(always)]
        pub const fn SCDIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SCDIE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SCDIE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SCDIE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SCDIE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCDIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WLMTIE0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WLMTIE1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn WLMTIE2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn WLMTIE3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WLMTIE4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMTIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn LLMTIE0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn LLMTIE1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn LLMTIE2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn LLMTIE3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LLMTIE4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMTIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn HLMTIE0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn HLMTIE1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn HLMTIE2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn HLMTIE3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn HLMTIE4(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMTIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for EIE {
        #[inline(always)]
        fn default() -> EIE {
            EIE(0)
        }
    }
    impl core::fmt::Debug for EIE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EIE")
                .field("SCDIE0", &self.SCDIE0())
                .field("SCDIE1", &self.SCDIE1())
                .field("SCDIE2", &self.SCDIE2())
                .field("SCDIE3", &self.SCDIE3())
                .field("SCDIE4", &self.SCDIE4())
                .field("WLMTIE0", &self.WLMTIE0())
                .field("WLMTIE1", &self.WLMTIE1())
                .field("WLMTIE2", &self.WLMTIE2())
                .field("WLMTIE3", &self.WLMTIE3())
                .field("WLMTIE4", &self.WLMTIE4())
                .field("LLMTIE0", &self.LLMTIE0())
                .field("LLMTIE1", &self.LLMTIE1())
                .field("LLMTIE2", &self.LLMTIE2())
                .field("LLMTIE3", &self.LLMTIE3())
                .field("LLMTIE4", &self.LLMTIE4())
                .field("HLMTIE0", &self.HLMTIE0())
                .field("HLMTIE1", &self.HLMTIE1())
                .field("HLMTIE2", &self.HLMTIE2())
                .field("HLMTIE3", &self.HLMTIE3())
                .field("HLMTIE4", &self.HLMTIE4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EIE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EIE {
                SCDIE0: bool,
                SCDIE1: bool,
                SCDIE2: bool,
                SCDIE3: bool,
                SCDIE4: bool,
                WLMTIE0: bool,
                WLMTIE1: bool,
                WLMTIE2: bool,
                WLMTIE3: bool,
                WLMTIE4: bool,
                LLMTIE0: bool,
                LLMTIE1: bool,
                LLMTIE2: bool,
                LLMTIE3: bool,
                LLMTIE4: bool,
                HLMTIE0: bool,
                HLMTIE1: bool,
                HLMTIE2: bool,
                HLMTIE3: bool,
                HLMTIE4: bool,
            }
            let proxy = EIE {
                SCDIE0: self.SCDIE0(),
                SCDIE1: self.SCDIE1(),
                SCDIE2: self.SCDIE2(),
                SCDIE3: self.SCDIE3(),
                SCDIE4: self.SCDIE4(),
                WLMTIE0: self.WLMTIE0(),
                WLMTIE1: self.WLMTIE1(),
                WLMTIE2: self.WLMTIE2(),
                WLMTIE3: self.WLMTIE3(),
                WLMTIE4: self.WLMTIE4(),
                LLMTIE0: self.LLMTIE0(),
                LLMTIE1: self.LLMTIE1(),
                LLMTIE2: self.LLMTIE2(),
                LLMTIE3: self.LLMTIE3(),
                LLMTIE4: self.LLMTIE4(),
                HLMTIE0: self.HLMTIE0(),
                HLMTIE1: self.HLMTIE1(),
                HLMTIE2: self.HLMTIE2(),
                HLMTIE3: self.HLMTIE3(),
                HLMTIE4: self.HLMTIE4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EIS(pub u32);
    impl EIS {
        #[inline(always)]
        pub const fn SCD0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SCD1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCD1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SCD2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCD2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SCD3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCD3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SCD4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCD4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WLMT0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WLMT1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn WLMT2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn WLMT3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WLMT4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WLMT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn LLMT0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn LLMT1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn LLMT2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn LLMT3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LLMT4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LLMT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn HLMT0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn HLMT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn HLMT2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn HLMT3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn HLMT4(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HLMT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for EIS {
        #[inline(always)]
        fn default() -> EIS {
            EIS(0)
        }
    }
    impl core::fmt::Debug for EIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EIS")
                .field("SCD0", &self.SCD0())
                .field("SCD1", &self.SCD1())
                .field("SCD2", &self.SCD2())
                .field("SCD3", &self.SCD3())
                .field("SCD4", &self.SCD4())
                .field("WLMT0", &self.WLMT0())
                .field("WLMT1", &self.WLMT1())
                .field("WLMT2", &self.WLMT2())
                .field("WLMT3", &self.WLMT3())
                .field("WLMT4", &self.WLMT4())
                .field("LLMT0", &self.LLMT0())
                .field("LLMT1", &self.LLMT1())
                .field("LLMT2", &self.LLMT2())
                .field("LLMT3", &self.LLMT3())
                .field("LLMT4", &self.LLMT4())
                .field("HLMT0", &self.HLMT0())
                .field("HLMT1", &self.HLMT1())
                .field("HLMT2", &self.HLMT2())
                .field("HLMT3", &self.HLMT3())
                .field("HLMT4", &self.HLMT4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EIS {
                SCD0: bool,
                SCD1: bool,
                SCD2: bool,
                SCD3: bool,
                SCD4: bool,
                WLMT0: bool,
                WLMT1: bool,
                WLMT2: bool,
                WLMT3: bool,
                WLMT4: bool,
                LLMT0: bool,
                LLMT1: bool,
                LLMT2: bool,
                LLMT3: bool,
                LLMT4: bool,
                HLMT0: bool,
                HLMT1: bool,
                HLMT2: bool,
                HLMT3: bool,
                HLMT4: bool,
            }
            let proxy = EIS {
                SCD0: self.SCD0(),
                SCD1: self.SCD1(),
                SCD2: self.SCD2(),
                SCD3: self.SCD3(),
                SCD4: self.SCD4(),
                WLMT0: self.WLMT0(),
                WLMT1: self.WLMT1(),
                WLMT2: self.WLMT2(),
                WLMT3: self.WLMT3(),
                WLMT4: self.WLMT4(),
                LLMT0: self.LLMT0(),
                LLMT1: self.LLMT1(),
                LLMT2: self.LLMT2(),
                LLMT3: self.LLMT3(),
                LLMT4: self.LLMT4(),
                HLMT0: self.HLMT0(),
                HLMT1: self.HLMT1(),
                HLMT2: self.HLMT2(),
                HLMT3: self.HLMT3(),
                HLMT4: self.HLMT4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FIFO And CAD Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFOIE(pub u32);
    impl FIFOIE {
        #[inline(always)]
        pub const fn FUNFIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNFIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FUNFIE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNFIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FUNFIE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNFIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FUNFIE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNFIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FUNFIE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNFIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FOVFIE0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVFIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FOVFIE1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVFIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FOVFIE2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVFIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FOVFIE3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVFIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FOVFIE4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVFIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CADIE0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn CADIE1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn CADIE2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CADIE3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn CADIE4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CADIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn SATIE0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SATIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SATIE1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SATIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SATIE2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SATIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SATIE3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SATIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn SATIE4(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SATIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for FIFOIE {
        #[inline(always)]
        fn default() -> FIFOIE {
            FIFOIE(0)
        }
    }
    impl core::fmt::Debug for FIFOIE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFOIE")
                .field("FUNFIE0", &self.FUNFIE0())
                .field("FUNFIE1", &self.FUNFIE1())
                .field("FUNFIE2", &self.FUNFIE2())
                .field("FUNFIE3", &self.FUNFIE3())
                .field("FUNFIE4", &self.FUNFIE4())
                .field("FOVFIE0", &self.FOVFIE0())
                .field("FOVFIE1", &self.FOVFIE1())
                .field("FOVFIE2", &self.FOVFIE2())
                .field("FOVFIE3", &self.FOVFIE3())
                .field("FOVFIE4", &self.FOVFIE4())
                .field("CADIE0", &self.CADIE0())
                .field("CADIE1", &self.CADIE1())
                .field("CADIE2", &self.CADIE2())
                .field("CADIE3", &self.CADIE3())
                .field("CADIE4", &self.CADIE4())
                .field("SATIE0", &self.SATIE0())
                .field("SATIE1", &self.SATIE1())
                .field("SATIE2", &self.SATIE2())
                .field("SATIE3", &self.SATIE3())
                .field("SATIE4", &self.SATIE4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFOIE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FIFOIE {
                FUNFIE0: bool,
                FUNFIE1: bool,
                FUNFIE2: bool,
                FUNFIE3: bool,
                FUNFIE4: bool,
                FOVFIE0: bool,
                FOVFIE1: bool,
                FOVFIE2: bool,
                FOVFIE3: bool,
                FOVFIE4: bool,
                CADIE0: bool,
                CADIE1: bool,
                CADIE2: bool,
                CADIE3: bool,
                CADIE4: bool,
                SATIE0: bool,
                SATIE1: bool,
                SATIE2: bool,
                SATIE3: bool,
                SATIE4: bool,
            }
            let proxy = FIFOIE {
                FUNFIE0: self.FUNFIE0(),
                FUNFIE1: self.FUNFIE1(),
                FUNFIE2: self.FUNFIE2(),
                FUNFIE3: self.FUNFIE3(),
                FUNFIE4: self.FUNFIE4(),
                FOVFIE0: self.FOVFIE0(),
                FOVFIE1: self.FOVFIE1(),
                FOVFIE2: self.FOVFIE2(),
                FOVFIE3: self.FOVFIE3(),
                FOVFIE4: self.FOVFIE4(),
                CADIE0: self.CADIE0(),
                CADIE1: self.CADIE1(),
                CADIE2: self.CADIE2(),
                CADIE3: self.CADIE3(),
                CADIE4: self.CADIE4(),
                SATIE0: self.SATIE0(),
                SATIE1: self.SATIE1(),
                SATIE2: self.SATIE2(),
                SATIE3: self.SATIE3(),
                SATIE4: self.SATIE4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FIFO And CAD Error Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFOIS(pub u32);
    impl FIFOIS {
        #[inline(always)]
        pub const fn FUNF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FUNF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FUNF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FUNF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FUNF4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FUNF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FOVF0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FOVF1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FOVF2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FOVF3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FOVF4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOVF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CAD0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn CAD1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAD1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn CAD2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAD2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CAD3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAD3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn CAD4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAD4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn SAT0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SAT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SAT2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SAT3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn SAT4(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for FIFOIS {
        #[inline(always)]
        fn default() -> FIFOIS {
            FIFOIS(0)
        }
    }
    impl core::fmt::Debug for FIFOIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFOIS")
                .field("FUNF0", &self.FUNF0())
                .field("FUNF1", &self.FUNF1())
                .field("FUNF2", &self.FUNF2())
                .field("FUNF3", &self.FUNF3())
                .field("FUNF4", &self.FUNF4())
                .field("FOVF0", &self.FOVF0())
                .field("FOVF1", &self.FOVF1())
                .field("FOVF2", &self.FOVF2())
                .field("FOVF3", &self.FOVF3())
                .field("FOVF4", &self.FOVF4())
                .field("CAD0", &self.CAD0())
                .field("CAD1", &self.CAD1())
                .field("CAD2", &self.CAD2())
                .field("CAD3", &self.CAD3())
                .field("CAD4", &self.CAD4())
                .field("SAT0", &self.SAT0())
                .field("SAT1", &self.SAT1())
                .field("SAT2", &self.SAT2())
                .field("SAT3", &self.SAT3())
                .field("SAT4", &self.SAT4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFOIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FIFOIS {
                FUNF0: bool,
                FUNF1: bool,
                FUNF2: bool,
                FUNF3: bool,
                FUNF4: bool,
                FOVF0: bool,
                FOVF1: bool,
                FOVF2: bool,
                FOVF3: bool,
                FOVF4: bool,
                CAD0: bool,
                CAD1: bool,
                CAD2: bool,
                CAD3: bool,
                CAD4: bool,
                SAT0: bool,
                SAT1: bool,
                SAT2: bool,
                SAT3: bool,
                SAT4: bool,
            }
            let proxy = FIFOIS {
                FUNF0: self.FUNF0(),
                FUNF1: self.FUNF1(),
                FUNF2: self.FUNF2(),
                FUNF3: self.FUNF3(),
                FUNF4: self.FUNF4(),
                FOVF0: self.FOVF0(),
                FOVF1: self.FOVF1(),
                FOVF2: self.FOVF2(),
                FOVF3: self.FOVF3(),
                FOVF4: self.FOVF4(),
                CAD0: self.CAD0(),
                CAD1: self.CAD1(),
                CAD2: self.CAD2(),
                CAD3: self.CAD3(),
                CAD4: self.CAD4(),
                SAT0: self.SAT0(),
                SAT1: self.SAT1(),
                SAT2: self.SAT2(),
                SAT3: self.SAT3(),
                SAT4: self.SAT4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Main Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[inline(always)]
        pub const fn STRIG0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STRIG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STRIG1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STRIG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn STRIG2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STRIG2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STRIG3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STRIG3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STRIG4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STRIG4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DOZEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn RST(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn MEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn MCLKDIV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MCLKDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn PRESCALE(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRESCALE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[inline(always)]
        pub const fn MCLK0DIS(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLK0DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn MCLK1DIS(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLK1DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn MCLK2DIS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLK2DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("STRIG0", &self.STRIG0())
                .field("STRIG1", &self.STRIG1())
                .field("STRIG2", &self.STRIG2())
                .field("STRIG3", &self.STRIG3())
                .field("STRIG4", &self.STRIG4())
                .field("DOZEN", &self.DOZEN())
                .field("RST", &self.RST())
                .field("MEN", &self.MEN())
                .field("MCLKDIV", &self.MCLKDIV())
                .field("PRESCALE", &self.PRESCALE())
                .field("MCLK0DIS", &self.MCLK0DIS())
                .field("MCLK1DIS", &self.MCLK1DIS())
                .field("MCLK2DIS", &self.MCLK2DIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCR {
                STRIG0: bool,
                STRIG1: bool,
                STRIG2: bool,
                STRIG3: bool,
                STRIG4: bool,
                DOZEN: bool,
                RST: bool,
                MEN: bool,
                MCLKDIV: u8,
                PRESCALE: u8,
                MCLK0DIS: bool,
                MCLK1DIS: bool,
                MCLK2DIS: bool,
            }
            let proxy = MCR {
                STRIG0: self.STRIG0(),
                STRIG1: self.STRIG1(),
                STRIG2: self.STRIG2(),
                STRIG3: self.STRIG3(),
                STRIG4: self.STRIG4(),
                DOZEN: self.DOZEN(),
                RST: self.RST(),
                MEN: self.MEN(),
                MCLKDIV: self.MCLKDIV(),
                PRESCALE: self.PRESCALE(),
                MCLK0DIS: self.MCLK0DIS(),
                MCLK1DIS: self.MCLK1DIS(),
                MCLK2DIS: self.MCLK2DIS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Normal Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NIE(pub u32);
    impl NIE {
        #[inline(always)]
        pub const fn COCIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COCIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn COCIE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COCIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn COCIE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COCIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn COCIE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COCIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn COCIE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COCIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHFIE0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHFIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CHFIE1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHFIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CHFIE2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHFIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CHFIE3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHFIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CHFIE4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHFIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ZCDIE0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ZCDIE1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ZCDIE2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ZCDIE3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ZCDIE4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCDIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for NIE {
        #[inline(always)]
        fn default() -> NIE {
            NIE(0)
        }
    }
    impl core::fmt::Debug for NIE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NIE")
                .field("COCIE0", &self.COCIE0())
                .field("COCIE1", &self.COCIE1())
                .field("COCIE2", &self.COCIE2())
                .field("COCIE3", &self.COCIE3())
                .field("COCIE4", &self.COCIE4())
                .field("CHFIE0", &self.CHFIE0())
                .field("CHFIE1", &self.CHFIE1())
                .field("CHFIE2", &self.CHFIE2())
                .field("CHFIE3", &self.CHFIE3())
                .field("CHFIE4", &self.CHFIE4())
                .field("ZCDIE0", &self.ZCDIE0())
                .field("ZCDIE1", &self.ZCDIE1())
                .field("ZCDIE2", &self.ZCDIE2())
                .field("ZCDIE3", &self.ZCDIE3())
                .field("ZCDIE4", &self.ZCDIE4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NIE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct NIE {
                COCIE0: bool,
                COCIE1: bool,
                COCIE2: bool,
                COCIE3: bool,
                COCIE4: bool,
                CHFIE0: bool,
                CHFIE1: bool,
                CHFIE2: bool,
                CHFIE3: bool,
                CHFIE4: bool,
                ZCDIE0: bool,
                ZCDIE1: bool,
                ZCDIE2: bool,
                ZCDIE3: bool,
                ZCDIE4: bool,
            }
            let proxy = NIE {
                COCIE0: self.COCIE0(),
                COCIE1: self.COCIE1(),
                COCIE2: self.COCIE2(),
                COCIE3: self.COCIE3(),
                COCIE4: self.COCIE4(),
                CHFIE0: self.CHFIE0(),
                CHFIE1: self.CHFIE1(),
                CHFIE2: self.CHFIE2(),
                CHFIE3: self.CHFIE3(),
                CHFIE4: self.CHFIE4(),
                ZCDIE0: self.ZCDIE0(),
                ZCDIE1: self.ZCDIE1(),
                ZCDIE2: self.ZCDIE2(),
                ZCDIE3: self.ZCDIE3(),
                ZCDIE4: self.ZCDIE4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Normal Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NIS(pub u32);
    impl NIS {
        #[inline(always)]
        pub const fn COC0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn COC1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn COC2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn COC3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn COC4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COC4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHF0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CHF1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CHF2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CHF3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CHF4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ZCD0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ZCD1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCD1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ZCD2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCD2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ZCD3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCD3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ZCD4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZCD4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for NIS {
        #[inline(always)]
        fn default() -> NIS {
            NIS(0)
        }
    }
    impl core::fmt::Debug for NIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NIS")
                .field("COC0", &self.COC0())
                .field("COC1", &self.COC1())
                .field("COC2", &self.COC2())
                .field("COC3", &self.COC3())
                .field("COC4", &self.COC4())
                .field("CHF0", &self.CHF0())
                .field("CHF1", &self.CHF1())
                .field("CHF2", &self.CHF2())
                .field("CHF3", &self.CHF3())
                .field("CHF4", &self.CHF4())
                .field("ZCD0", &self.ZCD0())
                .field("ZCD1", &self.ZCD1())
                .field("ZCD2", &self.ZCD2())
                .field("ZCD3", &self.ZCD3())
                .field("ZCD4", &self.ZCD4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct NIS {
                COC0: bool,
                COC1: bool,
                COC2: bool,
                COC3: bool,
                COC4: bool,
                CHF0: bool,
                CHF1: bool,
                CHF2: bool,
                CHF3: bool,
                CHF4: bool,
                ZCD0: bool,
                ZCD1: bool,
                ZCD2: bool,
                ZCD3: bool,
                ZCD4: bool,
            }
            let proxy = NIS {
                COC0: self.COC0(),
                COC1: self.COC1(),
                COC2: self.COC2(),
                COC3: self.COC3(),
                COC4: self.COC4(),
                CHF0: self.CHF0(),
                CHF1: self.CHF1(),
                CHF2: self.CHF2(),
                CHF3: self.CHF3(),
                CHF4: self.CHF4(),
                ZCD0: self.ZCD0(),
                ZCD1: self.ZCD1(),
                ZCD2: self.ZCD2(),
                ZCD3: self.ZCD3(),
                ZCD4: self.ZCD4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAMETER(pub u32);
    impl PARAMETER {
        #[inline(always)]
        pub const fn FIFO_DEPTH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFO_DEPTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn FLT_NUM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLT_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn PF_ORD_SEL(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PF_ORD_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
        }
    }
    impl Default for PARAMETER {
        #[inline(always)]
        fn default() -> PARAMETER {
            PARAMETER(0)
        }
    }
    impl core::fmt::Debug for PARAMETER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAMETER")
                .field("FIFO_DEPTH", &self.FIFO_DEPTH())
                .field("FLT_NUM", &self.FLT_NUM())
                .field("PF_ORD_SEL", &self.PF_ORD_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAMETER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PARAMETER {
                FIFO_DEPTH: u8,
                FLT_NUM: u8,
                PF_ORD_SEL: u8,
            }
            let proxy = PARAMETER {
                FIFO_DEPTH: self.FIFO_DEPTH(),
                FLT_NUM: self.FLT_NUM(),
                PF_ORD_SEL: self.PF_ORD_SEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR(pub u32);
    impl SR {
        #[inline(always)]
        pub const fn CIP0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CIP1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CIP2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CIP3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIP3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CIP4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIP4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHRDY0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRDY0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CHRDY1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRDY1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CHRDY2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRDY2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CHRDY3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRDY3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CHRDY4(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRDY4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FIFOEMPTY0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEMPTY0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FIFOEMPTY1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEMPTY1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FIFOEMPTY2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEMPTY2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn FIFOEMPTY3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEMPTY3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn FIFOEMPTY4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIFOEMPTY4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn MCLKRDY0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLKRDY0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn MCLKRDY1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLKRDY1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn MCLKRDY2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MCLKRDY2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("CIP0", &self.CIP0())
                .field("CIP1", &self.CIP1())
                .field("CIP2", &self.CIP2())
                .field("CIP3", &self.CIP3())
                .field("CIP4", &self.CIP4())
                .field("CHRDY0", &self.CHRDY0())
                .field("CHRDY1", &self.CHRDY1())
                .field("CHRDY2", &self.CHRDY2())
                .field("CHRDY3", &self.CHRDY3())
                .field("CHRDY4", &self.CHRDY4())
                .field("FIFOEMPTY0", &self.FIFOEMPTY0())
                .field("FIFOEMPTY1", &self.FIFOEMPTY1())
                .field("FIFOEMPTY2", &self.FIFOEMPTY2())
                .field("FIFOEMPTY3", &self.FIFOEMPTY3())
                .field("FIFOEMPTY4", &self.FIFOEMPTY4())
                .field("MCLKRDY0", &self.MCLKRDY0())
                .field("MCLKRDY1", &self.MCLKRDY1())
                .field("MCLKRDY2", &self.MCLKRDY2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SR {
                CIP0: bool,
                CIP1: bool,
                CIP2: bool,
                CIP3: bool,
                CIP4: bool,
                CHRDY0: bool,
                CHRDY1: bool,
                CHRDY2: bool,
                CHRDY3: bool,
                CHRDY4: bool,
                FIFOEMPTY0: bool,
                FIFOEMPTY1: bool,
                FIFOEMPTY2: bool,
                FIFOEMPTY3: bool,
                FIFOEMPTY4: bool,
                MCLKRDY0: bool,
                MCLKRDY1: bool,
                MCLKRDY2: bool,
            }
            let proxy = SR {
                CIP0: self.CIP0(),
                CIP1: self.CIP1(),
                CIP2: self.CIP2(),
                CIP3: self.CIP3(),
                CIP4: self.CIP4(),
                CHRDY0: self.CHRDY0(),
                CHRDY1: self.CHRDY1(),
                CHRDY2: self.CHRDY2(),
                CHRDY3: self.CHRDY3(),
                CHRDY4: self.CHRDY4(),
                FIFOEMPTY0: self.FIFOEMPTY0(),
                FIFOEMPTY1: self.FIFOEMPTY1(),
                FIFOEMPTY2: self.FIFOEMPTY2(),
                FIFOEMPTY3: self.FIFOEMPTY3(),
                FIFOEMPTY4: self.FIFOEMPTY4(),
                MCLKRDY0: self.MCLKRDY0(),
                MCLKRDY1: self.MCLKRDY1(),
                MCLKRDY2: self.MCLKRDY2(),
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
