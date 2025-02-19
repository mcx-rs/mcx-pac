#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TSI {
    ptr: *mut u8,
}
unsafe impl Send for TSI {}
unsafe impl Sync for TSI {}
impl TSI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CONFIG_MUTUAL(self) -> crate::common::Reg<regs::CONFIG_MUTUAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn TSHD(self) -> crate::common::Reg<regs::TSHD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GENCS(self) -> crate::common::Reg<regs::GENCS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MUL(self) -> crate::common::Reg<regs::MUL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SINC(self) -> crate::common::Reg<regs::SINC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SSC0(self) -> crate::common::Reg<regs::SSC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn SSC1(self) -> crate::common::Reg<regs::SSC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn SSC2(self) -> crate::common::Reg<regs::SSC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn BASELINE(self) -> crate::common::Reg<regs::BASELINE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CHMERGE(self) -> crate::common::Reg<regs::CHMERGE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIELD(self) -> crate::common::Reg<regs::SHIELD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn DATA(self) -> crate::common::Reg<regs::DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn MISC(self) -> crate::common::Reg<regs::MISC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIG(self) -> crate::common::Reg<regs::TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
}
pub mod regs {
    #[doc = "TSI Baseline"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BASELINE(pub u32);
    impl BASELINE {
        #[inline(always)]
        pub const fn BASELINE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_BASELINE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn BASE_TRACE_DEBOUNCE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BASE_TRACE_DEBOUNCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn BASE_TRACE_EN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BASE_TRACE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn THESHOLD_RATIO(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_THESHOLD_RATIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[inline(always)]
        pub const fn THRESHOLD_TRACE_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_THRESHOLD_TRACE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for BASELINE {
        #[inline(always)]
        fn default() -> BASELINE {
            BASELINE(0)
        }
    }
    impl core::fmt::Debug for BASELINE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BASELINE")
                .field("BASELINE", &self.BASELINE())
                .field("BASE_TRACE_DEBOUNCE", &self.BASE_TRACE_DEBOUNCE())
                .field("BASE_TRACE_EN", &self.BASE_TRACE_EN())
                .field("THESHOLD_RATIO", &self.THESHOLD_RATIO())
                .field("THRESHOLD_TRACE_EN", &self.THRESHOLD_TRACE_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BASELINE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BASELINE {{ BASELINE: {=u16:?}, BASE_TRACE_DEBOUNCE: {=u8:?}, BASE_TRACE_EN: {=bool:?}, THESHOLD_RATIO: {=u8:?}, THRESHOLD_TRACE_EN: {=bool:?} }}" , self . BASELINE () , self . BASE_TRACE_DEBOUNCE () , self . BASE_TRACE_EN () , self . THESHOLD_RATIO () , self . THRESHOLD_TRACE_EN ())
        }
    }
    #[doc = "TSI Channel Merge"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHMERGE(pub u32);
    impl CHMERGE {
        #[inline(always)]
        pub const fn CHANNEL_ENABLE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CHANNEL_ENABLE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for CHMERGE {
        #[inline(always)]
        fn default() -> CHMERGE {
            CHMERGE(0)
        }
    }
    impl core::fmt::Debug for CHMERGE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHMERGE")
                .field("CHANNEL_ENABLE", &self.CHANNEL_ENABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHMERGE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CHMERGE {{ CHANNEL_ENABLE: {=u32:?} }}",
                self.CHANNEL_ENABLE()
            )
        }
    }
    #[doc = "TSI CONFIG (TSI_CONFIG) for Self-Capacitor"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIG(pub u32);
    impl CONFIG {
        #[inline(always)]
        pub const fn MODE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TSICH(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSICH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[inline(always)]
        pub const fn S_NOISE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_S_NOISE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn S_XCH(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_S_XCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn S_XIN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_S_XIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn S_CTRIM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_S_CTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[inline(always)]
        pub const fn S_SEN(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_S_SEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn S_XDN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_S_XDN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[inline(always)]
        pub const fn S_XIN_ADD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_S_XIN_ADD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CONFIG {
        #[inline(always)]
        fn default() -> CONFIG {
            CONFIG(0)
        }
    }
    impl core::fmt::Debug for CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIG")
                .field("MODE", &self.MODE())
                .field("TSICH", &self.TSICH())
                .field("S_NOISE", &self.S_NOISE())
                .field("S_XCH", &self.S_XCH())
                .field("S_XIN", &self.S_XIN())
                .field("S_CTRIM", &self.S_CTRIM())
                .field("S_SEN", &self.S_SEN())
                .field("S_XDN", &self.S_XDN())
                .field("S_XIN_ADD", &self.S_XIN_ADD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CONFIG {{ MODE: {=bool:?}, TSICH: {=u8:?}, S_NOISE: {=bool:?}, S_XCH: {=u8:?}, S_XIN: {=bool:?}, S_CTRIM: {=u8:?}, S_SEN: {=bool:?}, S_XDN: {=u8:?}, S_XIN_ADD: {=bool:?} }}" , self . MODE () , self . TSICH () , self . S_NOISE () , self . S_XCH () , self . S_XIN () , self . S_CTRIM () , self . S_SEN () , self . S_XDN () , self . S_XIN_ADD ())
        }
    }
    #[doc = "TSI CONFIG (TSI_CONFIG) for Mutual-Capacitor"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIG_MUTUAL(pub u32);
    impl CONFIG_MUTUAL {
        #[inline(always)]
        pub const fn MODE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn M_NMIRROR(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_NMIRROR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn M_PMIRRORR(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_PMIRRORR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[inline(always)]
        pub const fn M_PMIRRORL(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_PMIRRORL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[inline(always)]
        pub const fn M_SEL_RX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_SEL_RX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn M_SEL_TX(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_SEL_TX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[inline(always)]
        pub const fn M_CNT_EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M_CNT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn M_TX_PD_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M_TX_PD_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn M_SEN_BOOST(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_SEN_BOOST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[inline(always)]
        pub const fn M_PRE_RES(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_PRE_RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
        }
        #[inline(always)]
        pub const fn M_PRE_CURRENT(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_PRE_CURRENT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for CONFIG_MUTUAL {
        #[inline(always)]
        fn default() -> CONFIG_MUTUAL {
            CONFIG_MUTUAL(0)
        }
    }
    impl core::fmt::Debug for CONFIG_MUTUAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIG_MUTUAL")
                .field("MODE", &self.MODE())
                .field("M_NMIRROR", &self.M_NMIRROR())
                .field("M_PMIRRORR", &self.M_PMIRRORR())
                .field("M_PMIRRORL", &self.M_PMIRRORL())
                .field("M_SEL_RX", &self.M_SEL_RX())
                .field("M_SEL_TX", &self.M_SEL_TX())
                .field("M_CNT_EN", &self.M_CNT_EN())
                .field("M_TX_PD_EN", &self.M_TX_PD_EN())
                .field("M_SEN_BOOST", &self.M_SEN_BOOST())
                .field("M_PRE_RES", &self.M_PRE_RES())
                .field("M_PRE_CURRENT", &self.M_PRE_CURRENT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIG_MUTUAL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CONFIG_MUTUAL {{ MODE: {=bool:?}, M_NMIRROR: {=u8:?}, M_PMIRRORR: {=u8:?}, M_PMIRRORL: {=u8:?}, M_SEL_RX: {=u8:?}, M_SEL_TX: {=u8:?}, M_CNT_EN: {=bool:?}, M_TX_PD_EN: {=bool:?}, M_SEN_BOOST: {=u8:?}, M_PRE_RES: {=u8:?}, M_PRE_CURRENT: {=u8:?} }}" , self . MODE () , self . M_NMIRROR () , self . M_PMIRRORR () , self . M_PMIRRORL () , self . M_SEL_RX () , self . M_SEL_TX () , self . M_CNT_EN () , self . M_TX_PD_EN () , self . M_SEN_BOOST () , self . M_PRE_RES () , self . M_PRE_CURRENT ())
        }
    }
    #[doc = "TSI Data and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA(pub u32);
    impl DATA {
        #[inline(always)]
        pub const fn TSICNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TSICNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn EOSF(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EOSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn OVERRUNF(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVERRUNF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn OUTRGF(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUTRGF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("TSICNT", &self.TSICNT())
                .field("EOSF", &self.EOSF())
                .field("OVERRUNF", &self.OVERRUNF())
                .field("OUTRGF", &self.OUTRGF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DATA {{ TSICNT: {=u16:?}, EOSF: {=bool:?}, OVERRUNF: {=bool:?}, OUTRGF: {=bool:?} }}" , self . TSICNT () , self . EOSF () , self . OVERRUNF () , self . OUTRGF ())
        }
    }
    #[doc = "TSI General Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GENCS(pub u32);
    impl GENCS {
        #[inline(always)]
        pub const fn DMAEN_EOS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAEN_EOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DMAEN_OUTRG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAEN_OUTRG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STPE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STPE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TSIEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SWTS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn CTRIM_FINE(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTRIM_FINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[inline(always)]
        pub const fn DVOLT(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DVOLT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[inline(always)]
        pub const fn DEBOUNCE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DEBOUNCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn S_PROX_EN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_S_PROX_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn SETCLK(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[inline(always)]
        pub const fn ESOR(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn OUTRG_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUTRG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for GENCS {
        #[inline(always)]
        fn default() -> GENCS {
            GENCS(0)
        }
    }
    impl core::fmt::Debug for GENCS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GENCS")
                .field("DMAEN_EOS", &self.DMAEN_EOS())
                .field("DMAEN_OUTRG", &self.DMAEN_OUTRG())
                .field("STM", &self.STM())
                .field("STPE", &self.STPE())
                .field("TSIEN", &self.TSIEN())
                .field("SWTS", &self.SWTS())
                .field("CTRIM_FINE", &self.CTRIM_FINE())
                .field("DVOLT", &self.DVOLT())
                .field("DEBOUNCE", &self.DEBOUNCE())
                .field("S_PROX_EN", &self.S_PROX_EN())
                .field("SETCLK", &self.SETCLK())
                .field("ESOR", &self.ESOR())
                .field("OUTRG_EN", &self.OUTRG_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GENCS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GENCS {{ DMAEN_EOS: {=bool:?}, DMAEN_OUTRG: {=bool:?}, STM: {=bool:?}, STPE: {=bool:?}, TSIEN: {=bool:?}, SWTS: {=bool:?}, CTRIM_FINE: {=u8:?}, DVOLT: {=u8:?}, DEBOUNCE: {=u8:?}, S_PROX_EN: {=bool:?}, SETCLK: {=u8:?}, ESOR: {=bool:?}, OUTRG_EN: {=bool:?} }}" , self . DMAEN_EOS () , self . DMAEN_OUTRG () , self . STM () , self . STPE () , self . TSIEN () , self . SWTS () , self . CTRIM_FINE () , self . DVOLT () , self . DEBOUNCE () , self . S_PROX_EN () , self . SETCLK () , self . ESOR () , self . OUTRG_EN ())
        }
    }
    #[doc = "TSI Miscellaneous"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISC(pub u32);
    impl MISC {
        #[inline(always)]
        pub const fn OSC_CLK_SEL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn TEST_FINGER(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEST_FINGER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn TEST_FINGER_EN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEST_FINGER_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn CLKDIVIDER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKDIVIDER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for MISC {
        #[inline(always)]
        fn default() -> MISC {
            MISC(0)
        }
    }
    impl core::fmt::Debug for MISC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MISC")
                .field("OSC_CLK_SEL", &self.OSC_CLK_SEL())
                .field("TEST_FINGER", &self.TEST_FINGER())
                .field("TEST_FINGER_EN", &self.TEST_FINGER_EN())
                .field("CLKDIVIDER", &self.CLKDIVIDER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MISC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MISC {{ OSC_CLK_SEL: {=bool:?}, TEST_FINGER: {=u8:?}, TEST_FINGER_EN: {=bool:?}, CLKDIVIDER: {=u8:?} }}" , self . OSC_CLK_SEL () , self . TEST_FINGER () , self . TEST_FINGER_EN () , self . CLKDIVIDER ())
        }
    }
    #[doc = "TSI Mutual-Capacitance"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MUL(pub u32);
    impl MUL {
        #[inline(always)]
        pub const fn M_VPRE_CHOOSE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M_VPRE_CHOOSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn M_MODE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn M_TRIM_CAP(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_TRIM_CAP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[inline(always)]
        pub const fn M_TX_USED(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_TX_USED(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
        #[inline(always)]
        pub const fn M_TRIM(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_M_TRIM(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MUL {
        #[inline(always)]
        fn default() -> MUL {
            MUL(0)
        }
    }
    impl core::fmt::Debug for MUL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MUL")
                .field("M_VPRE_CHOOSE", &self.M_VPRE_CHOOSE())
                .field("M_MODE", &self.M_MODE())
                .field("M_TRIM_CAP", &self.M_TRIM_CAP())
                .field("M_TX_USED", &self.M_TX_USED())
                .field("M_TRIM", &self.M_TRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MUL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MUL {{ M_VPRE_CHOOSE: {=bool:?}, M_MODE: {=bool:?}, M_TRIM_CAP: {=u8:?}, M_TX_USED: {=u8:?}, M_TRIM: {=u16:?} }}" , self . M_VPRE_CHOOSE () , self . M_MODE () , self . M_TRIM_CAP () , self . M_TX_USED () , self . M_TRIM ())
        }
    }
    #[doc = "TSI Shield"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIELD(pub u32);
    impl SHIELD {
        #[inline(always)]
        pub const fn SHIELD_ENABLE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SHIELD_ENABLE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn M_SEN_RES(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_M_SEN_RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 25usize)) | (((val as u32) & 0x3f) << 25usize);
        }
    }
    impl Default for SHIELD {
        #[inline(always)]
        fn default() -> SHIELD {
            SHIELD(0)
        }
    }
    impl core::fmt::Debug for SHIELD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIELD")
                .field("SHIELD_ENABLE", &self.SHIELD_ENABLE())
                .field("M_SEN_RES", &self.M_SEN_RES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIELD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SHIELD {{ SHIELD_ENABLE: {=u8:?}, M_SEN_RES: {=u8:?} }}",
                self.SHIELD_ENABLE(),
                self.M_SEN_RES()
            )
        }
    }
    #[doc = "TSI SINC Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SINC(pub u32);
    impl SINC {
        #[inline(always)]
        pub const fn SSC_CONTROL_OUT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SSC_CONTROL_OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SINC_VALID(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SINC_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SINC_OVERFLOW_FLAG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SINC_OVERFLOW_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SWITCH_ENABLE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWITCH_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DECIMATION(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DECIMATION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn ORDER(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ORDER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn CUTOFF(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CUTOFF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for SINC {
        #[inline(always)]
        fn default() -> SINC {
            SINC(0)
        }
    }
    impl core::fmt::Debug for SINC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SINC")
                .field("SSC_CONTROL_OUT", &self.SSC_CONTROL_OUT())
                .field("SINC_VALID", &self.SINC_VALID())
                .field("SINC_OVERFLOW_FLAG", &self.SINC_OVERFLOW_FLAG())
                .field("SWITCH_ENABLE", &self.SWITCH_ENABLE())
                .field("DECIMATION", &self.DECIMATION())
                .field("ORDER", &self.ORDER())
                .field("CUTOFF", &self.CUTOFF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SINC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SINC {{ SSC_CONTROL_OUT: {=bool:?}, SINC_VALID: {=bool:?}, SINC_OVERFLOW_FLAG: {=bool:?}, SWITCH_ENABLE: {=bool:?}, DECIMATION: {=u8:?}, ORDER: {=bool:?}, CUTOFF: {=u8:?} }}" , self . SSC_CONTROL_OUT () , self . SINC_VALID () , self . SINC_OVERFLOW_FLAG () , self . SWITCH_ENABLE () , self . DECIMATION () , self . ORDER () , self . CUTOFF ())
        }
    }
    #[doc = "TSI SSC 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSC0(pub u32);
    impl SSC0 {
        #[inline(always)]
        pub const fn SSC_PRESCALE_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSC_PRESCALE_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn BASE_NOCHARGE_NUM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BASE_NOCHARGE_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn CHARGE_NUM(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHARGE_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn SSC_CONTROL_REVERSE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SSC_CONTROL_REVERSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SSC_MODE(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSC_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[inline(always)]
        pub const fn PRBS_OUTSEL(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRBS_OUTSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for SSC0 {
        #[inline(always)]
        fn default() -> SSC0 {
            SSC0(0)
        }
    }
    impl core::fmt::Debug for SSC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSC0")
                .field("SSC_PRESCALE_NUM", &self.SSC_PRESCALE_NUM())
                .field("BASE_NOCHARGE_NUM", &self.BASE_NOCHARGE_NUM())
                .field("CHARGE_NUM", &self.CHARGE_NUM())
                .field("SSC_CONTROL_REVERSE", &self.SSC_CONTROL_REVERSE())
                .field("SSC_MODE", &self.SSC_MODE())
                .field("PRBS_OUTSEL", &self.PRBS_OUTSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSC0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSC0 {{ SSC_PRESCALE_NUM: {=u8:?}, BASE_NOCHARGE_NUM: {=u8:?}, CHARGE_NUM: {=u8:?}, SSC_CONTROL_REVERSE: {=bool:?}, SSC_MODE: {=u8:?}, PRBS_OUTSEL: {=u8:?} }}" , self . SSC_PRESCALE_NUM () , self . BASE_NOCHARGE_NUM () , self . CHARGE_NUM () , self . SSC_CONTROL_REVERSE () , self . SSC_MODE () , self . PRBS_OUTSEL ())
        }
    }
    #[doc = "TSI SSC 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSC1(pub u32);
    impl SSC1 {
        #[inline(always)]
        pub const fn PRBS_SEED_LO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRBS_SEED_LO(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn PRBS_SEED_HI(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRBS_SEED_HI(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn PRBS_WEIGHT_LO(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRBS_WEIGHT_LO(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn PRBS_WEIGHT_HI(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRBS_WEIGHT_HI(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SSC1 {
        #[inline(always)]
        fn default() -> SSC1 {
            SSC1(0)
        }
    }
    impl core::fmt::Debug for SSC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSC1")
                .field("PRBS_SEED_LO", &self.PRBS_SEED_LO())
                .field("PRBS_SEED_HI", &self.PRBS_SEED_HI())
                .field("PRBS_WEIGHT_LO", &self.PRBS_WEIGHT_LO())
                .field("PRBS_WEIGHT_HI", &self.PRBS_WEIGHT_HI())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSC1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSC1 {{ PRBS_SEED_LO: {=u8:?}, PRBS_SEED_HI: {=u8:?}, PRBS_WEIGHT_LO: {=u8:?}, PRBS_WEIGHT_HI: {=u8:?} }}" , self . PRBS_SEED_LO () , self . PRBS_SEED_HI () , self . PRBS_WEIGHT_LO () , self . PRBS_WEIGHT_HI ())
        }
    }
    #[doc = "TSI SSC 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSC2(pub u32);
    impl SSC2 {
        #[inline(always)]
        pub const fn MOVE_REPEAT_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MOVE_REPEAT_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn MOVE_STEPS_NUM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MOVE_STEPS_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn MOVE_NOCHARGE_MAX(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MOVE_NOCHARGE_MAX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[inline(always)]
        pub const fn MOVE_NOCHARGE_MIN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MOVE_NOCHARGE_MIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for SSC2 {
        #[inline(always)]
        fn default() -> SSC2 {
            SSC2(0)
        }
    }
    impl core::fmt::Debug for SSC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSC2")
                .field("MOVE_REPEAT_NUM", &self.MOVE_REPEAT_NUM())
                .field("MOVE_STEPS_NUM", &self.MOVE_STEPS_NUM())
                .field("MOVE_NOCHARGE_MAX", &self.MOVE_NOCHARGE_MAX())
                .field("MOVE_NOCHARGE_MIN", &self.MOVE_NOCHARGE_MIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSC2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSC2 {{ MOVE_REPEAT_NUM: {=u8:?}, MOVE_STEPS_NUM: {=u8:?}, MOVE_NOCHARGE_MAX: {=u8:?}, MOVE_NOCHARGE_MIN: {=u8:?} }}" , self . MOVE_REPEAT_NUM () , self . MOVE_STEPS_NUM () , self . MOVE_NOCHARGE_MAX () , self . MOVE_NOCHARGE_MIN ())
        }
    }
    #[doc = "TSI AUTO TRIG"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIG(pub u32);
    impl TRIG {
        #[inline(always)]
        pub const fn TRIG_PERIOD_COUNTER(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TRIG_PERIOD_COUNTER(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn TRIG_CLK_DIVIDER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIG_CLK_DIVIDER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[inline(always)]
        pub const fn TRIG_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn TRIG_CLK_SEL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIG_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TRIG {
        #[inline(always)]
        fn default() -> TRIG {
            TRIG(0)
        }
    }
    impl core::fmt::Debug for TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIG")
                .field("TRIG_PERIOD_COUNTER", &self.TRIG_PERIOD_COUNTER())
                .field("TRIG_CLK_DIVIDER", &self.TRIG_CLK_DIVIDER())
                .field("TRIG_EN", &self.TRIG_EN())
                .field("TRIG_CLK_SEL", &self.TRIG_CLK_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TRIG {{ TRIG_PERIOD_COUNTER: {=u32:?}, TRIG_CLK_DIVIDER: {=u8:?}, TRIG_EN: {=bool:?}, TRIG_CLK_SEL: {=bool:?} }}" , self . TRIG_PERIOD_COUNTER () , self . TRIG_CLK_DIVIDER () , self . TRIG_EN () , self . TRIG_CLK_SEL ())
        }
    }
    #[doc = "TSI Threshold"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TSHD(pub u32);
    impl TSHD {
        #[inline(always)]
        pub const fn THRESL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_THRESL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn THRESH(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_THRESH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for TSHD {
        #[inline(always)]
        fn default() -> TSHD {
            TSHD(0)
        }
    }
    impl core::fmt::Debug for TSHD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TSHD")
                .field("THRESL", &self.THRESL())
                .field("THRESH", &self.THRESH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TSHD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TSHD {{ THRESL: {=u16:?}, THRESH: {=u16:?} }}",
                self.THRESL(),
                self.THRESH()
            )
        }
    }
}
