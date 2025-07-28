#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EMVSIM {
    ptr: *mut u8,
}
unsafe impl Send for EMVSIM {}
unsafe impl Sync for EMVSIM {}
impl EMVSIM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VER_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKCFG(self) -> crate::common::Reg<regs::CLKCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn DIVISOR(self) -> crate::common::Reg<regs::DIVISOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_MASK(self) -> crate::common::Reg<regs::INT_MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn RX_THD(self) -> crate::common::Reg<regs::RX_THD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_THD(self) -> crate::common::Reg<regs::TX_THD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn RX_STATUS(self) -> crate::common::Reg<regs::RX_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_STATUS(self) -> crate::common::Reg<regs::TX_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn PCSR(self) -> crate::common::Reg<regs::PCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn RX_BUF(self) -> crate::common::Reg<regs::RX_BUF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn TX_BUF(self) -> crate::common::Reg<regs::TX_BUF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_GETU(self) -> crate::common::Reg<regs::TX_GETU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn CWT_VAL(self) -> crate::common::Reg<regs::CWT_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn BWT_VAL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn BGT_VAL(self) -> crate::common::Reg<regs::BGT_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn GPCNT0_VAL(self) -> crate::common::Reg<regs::GPCNT0_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn GPCNT1_VAL(self) -> crate::common::Reg<regs::GPCNT1_VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "Block Guard Time Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BGT_VAL(pub u32);
    impl BGT_VAL {
        #[must_use]
        #[inline(always)]
        pub const fn BGT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_BGT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for BGT_VAL {
        #[inline(always)]
        fn default() -> BGT_VAL {
            BGT_VAL(0)
        }
    }
    impl core::fmt::Debug for BGT_VAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BGT_VAL").field("BGT", &self.BGT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BGT_VAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BGT_VAL {{ BGT: {=u16:?} }}", self.BGT())
        }
    }
    #[doc = "Clock Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKCFG(pub u32);
    impl CLKCFG {
        #[must_use]
        #[inline(always)]
        pub const fn CLK_PRSC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLK_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT1_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPCNT1_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT0_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPCNT0_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for CLKCFG {
        #[inline(always)]
        fn default() -> CLKCFG {
            CLKCFG(0)
        }
    }
    impl core::fmt::Debug for CLKCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKCFG")
                .field("CLK_PRSC", &self.CLK_PRSC())
                .field("GPCNT1_CLK_SEL", &self.GPCNT1_CLK_SEL())
                .field("GPCNT0_CLK_SEL", &self.GPCNT0_CLK_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CLKCFG {{ CLK_PRSC: {=u8:?}, GPCNT1_CLK_SEL: {=u8:?}, GPCNT0_CLK_SEL: {=u8:?} }}",
                self.CLK_PRSC(),
                self.GPCNT1_CLK_SEL(),
                self.GPCNT0_CLK_SEL()
            )
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn IC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ICM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ICM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ANACK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ANACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONACK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLSH_RX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLSH_RX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLSH_TX(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLSH_TX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW_RST(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KILL_CLOCKS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_KILL_CLOCKS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DOZE_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DOZE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RCV_EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RCV_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XMT_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_XMT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RCVR_11(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RCVR_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_DMA_EN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RX_DMA_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TX_DMA_EN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TX_DMA_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INV_CRC_VAL(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INV_CRC_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC_OUT_FLIP(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC_OUT_FLIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC_IN_FLIP(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC_IN_FLIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWT_EN(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LRC_EN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LRC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC_EN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XMT_CRC_LRC(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_XMT_CRC_LRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWT_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("IC", &self.IC())
                .field("ICM", &self.ICM())
                .field("ANACK", &self.ANACK())
                .field("ONACK", &self.ONACK())
                .field("FLSH_RX", &self.FLSH_RX())
                .field("FLSH_TX", &self.FLSH_TX())
                .field("SW_RST", &self.SW_RST())
                .field("KILL_CLOCKS", &self.KILL_CLOCKS())
                .field("DOZE_EN", &self.DOZE_EN())
                .field("STOP_EN", &self.STOP_EN())
                .field("RCV_EN", &self.RCV_EN())
                .field("XMT_EN", &self.XMT_EN())
                .field("RCVR_11", &self.RCVR_11())
                .field("RX_DMA_EN", &self.RX_DMA_EN())
                .field("TX_DMA_EN", &self.TX_DMA_EN())
                .field("INV_CRC_VAL", &self.INV_CRC_VAL())
                .field("CRC_OUT_FLIP", &self.CRC_OUT_FLIP())
                .field("CRC_IN_FLIP", &self.CRC_IN_FLIP())
                .field("CWT_EN", &self.CWT_EN())
                .field("LRC_EN", &self.LRC_EN())
                .field("CRC_EN", &self.CRC_EN())
                .field("XMT_CRC_LRC", &self.XMT_CRC_LRC())
                .field("BWT_EN", &self.BWT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ IC: {=bool:?}, ICM: {=bool:?}, ANACK: {=bool:?}, ONACK: {=bool:?}, FLSH_RX: {=bool:?}, FLSH_TX: {=bool:?}, SW_RST: {=bool:?}, KILL_CLOCKS: {=bool:?}, DOZE_EN: {=bool:?}, STOP_EN: {=bool:?}, RCV_EN: {=bool:?}, XMT_EN: {=bool:?}, RCVR_11: {=bool:?}, RX_DMA_EN: {=bool:?}, TX_DMA_EN: {=bool:?}, INV_CRC_VAL: {=bool:?}, CRC_OUT_FLIP: {=bool:?}, CRC_IN_FLIP: {=bool:?}, CWT_EN: {=bool:?}, LRC_EN: {=bool:?}, CRC_EN: {=bool:?}, XMT_CRC_LRC: {=bool:?}, BWT_EN: {=bool:?} }}" , self . IC () , self . ICM () , self . ANACK () , self . ONACK () , self . FLSH_RX () , self . FLSH_TX () , self . SW_RST () , self . KILL_CLOCKS () , self . DOZE_EN () , self . STOP_EN () , self . RCV_EN () , self . XMT_EN () , self . RCVR_11 () , self . RX_DMA_EN () , self . TX_DMA_EN () , self . INV_CRC_VAL () , self . CRC_OUT_FLIP () , self . CRC_IN_FLIP () , self . CWT_EN () , self . LRC_EN () , self . CRC_EN () , self . XMT_CRC_LRC () , self . BWT_EN ())
        }
    }
    #[doc = "Character Wait Time Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CWT_VAL(pub u32);
    impl CWT_VAL {
        #[must_use]
        #[inline(always)]
        pub const fn CWT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CWT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CWT_VAL {
        #[inline(always)]
        fn default() -> CWT_VAL {
            CWT_VAL(0)
        }
    }
    impl core::fmt::Debug for CWT_VAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CWT_VAL").field("CWT", &self.CWT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CWT_VAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CWT_VAL {{ CWT: {=u16:?} }}", self.CWT())
        }
    }
    #[doc = "Baud Rate Divisor"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DIVISOR(pub u32);
    impl DIVISOR {
        #[must_use]
        #[inline(always)]
        pub const fn DIVISOR_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DIVISOR_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for DIVISOR {
        #[inline(always)]
        fn default() -> DIVISOR {
            DIVISOR(0)
        }
    }
    impl core::fmt::Debug for DIVISOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DIVISOR")
                .field("DIVISOR_VALUE", &self.DIVISOR_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DIVISOR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DIVISOR {{ DIVISOR_VALUE: {=u16:?} }}",
                self.DIVISOR_VALUE()
            )
        }
    }
    #[doc = "General Purpose Counter 0 Timeout Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPCNT0_VAL(pub u32);
    impl GPCNT0_VAL {
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_GPCNT0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GPCNT0_VAL {
        #[inline(always)]
        fn default() -> GPCNT0_VAL {
            GPCNT0_VAL(0)
        }
    }
    impl core::fmt::Debug for GPCNT0_VAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPCNT0_VAL")
                .field("GPCNT0", &self.GPCNT0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPCNT0_VAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GPCNT0_VAL {{ GPCNT0: {=u16:?} }}", self.GPCNT0())
        }
    }
    #[doc = "General Purpose Counter 1 Timeout Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPCNT1_VAL(pub u32);
    impl GPCNT1_VAL {
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_GPCNT1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GPCNT1_VAL {
        #[inline(always)]
        fn default() -> GPCNT1_VAL {
            GPCNT1_VAL(0)
        }
    }
    impl core::fmt::Debug for GPCNT1_VAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPCNT1_VAL")
                .field("GPCNT1", &self.GPCNT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPCNT1_VAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GPCNT1_VAL {{ GPCNT1: {=u16:?} }}", self.GPCNT1())
        }
    }
    #[doc = "Interrupt Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_MASK(pub u32);
    impl INT_MASK {
        #[must_use]
        #[inline(always)]
        pub const fn RDT_IM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDT_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TC_IM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TC_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFO_IM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFO_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETC_IM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETC_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFE_IM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFE_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNACK_IM(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TNACK_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFF_IM(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFF_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDT_IM(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TDT_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT0_IM(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GPCNT0_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWT_ERR_IM(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWT_ERR_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RNACK_IM(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RNACK_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWT_ERR_IM(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWT_ERR_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGT_ERR_IM(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BGT_ERR_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT1_IM(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GPCNT1_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_DATA_IM(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RX_DATA_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PEF_IM(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PEF_IM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for INT_MASK {
        #[inline(always)]
        fn default() -> INT_MASK {
            INT_MASK(0)
        }
    }
    impl core::fmt::Debug for INT_MASK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_MASK")
                .field("RDT_IM", &self.RDT_IM())
                .field("TC_IM", &self.TC_IM())
                .field("RFO_IM", &self.RFO_IM())
                .field("ETC_IM", &self.ETC_IM())
                .field("TFE_IM", &self.TFE_IM())
                .field("TNACK_IM", &self.TNACK_IM())
                .field("TFF_IM", &self.TFF_IM())
                .field("TDT_IM", &self.TDT_IM())
                .field("GPCNT0_IM", &self.GPCNT0_IM())
                .field("CWT_ERR_IM", &self.CWT_ERR_IM())
                .field("RNACK_IM", &self.RNACK_IM())
                .field("BWT_ERR_IM", &self.BWT_ERR_IM())
                .field("BGT_ERR_IM", &self.BGT_ERR_IM())
                .field("GPCNT1_IM", &self.GPCNT1_IM())
                .field("RX_DATA_IM", &self.RX_DATA_IM())
                .field("PEF_IM", &self.PEF_IM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_MASK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INT_MASK {{ RDT_IM: {=bool:?}, TC_IM: {=bool:?}, RFO_IM: {=bool:?}, ETC_IM: {=bool:?}, TFE_IM: {=bool:?}, TNACK_IM: {=bool:?}, TFF_IM: {=bool:?}, TDT_IM: {=bool:?}, GPCNT0_IM: {=bool:?}, CWT_ERR_IM: {=bool:?}, RNACK_IM: {=bool:?}, BWT_ERR_IM: {=bool:?}, BGT_ERR_IM: {=bool:?}, GPCNT1_IM: {=bool:?}, RX_DATA_IM: {=bool:?}, PEF_IM: {=bool:?} }}" , self . RDT_IM () , self . TC_IM () , self . RFO_IM () , self . ETC_IM () , self . TFE_IM () , self . TNACK_IM () , self . TFF_IM () , self . TDT_IM () , self . GPCNT0_IM () , self . CWT_ERR_IM () , self . RNACK_IM () , self . BWT_ERR_IM () , self . BGT_ERR_IM () , self . GPCNT1_IM () , self . RX_DATA_IM () , self . PEF_IM ())
        }
    }
    #[doc = "Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn RX_FIFO_DEPTH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RX_FIFO_DEPTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TX_FIFO_DEPTH(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TX_FIFO_DEPTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
                .field("RX_FIFO_DEPTH", &self.RX_FIFO_DEPTH())
                .field("TX_FIFO_DEPTH", &self.TX_FIFO_DEPTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PARAM {{ RX_FIFO_DEPTH: {=u8:?}, TX_FIFO_DEPTH: {=u8:?} }}",
                self.RX_FIFO_DEPTH(),
                self.TX_FIFO_DEPTH()
            )
        }
    }
    #[doc = "Port Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCSR(pub u32);
    impl PCSR {
        #[must_use]
        #[inline(always)]
        pub const fn SAPD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SAPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SVCC_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SVCC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VCCENP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VCCENP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCSP(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPD(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPDIM(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPDIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPDIF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPDIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPDP(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPDP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPDES(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPDES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for PCSR {
        #[inline(always)]
        fn default() -> PCSR {
            PCSR(0)
        }
    }
    impl core::fmt::Debug for PCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCSR")
                .field("SAPD", &self.SAPD())
                .field("SVCC_EN", &self.SVCC_EN())
                .field("VCCENP", &self.VCCENP())
                .field("SRST", &self.SRST())
                .field("SCEN", &self.SCEN())
                .field("SCSP", &self.SCSP())
                .field("SPD", &self.SPD())
                .field("SPDIM", &self.SPDIM())
                .field("SPDIF", &self.SPDIF())
                .field("SPDP", &self.SPDP())
                .field("SPDES", &self.SPDES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PCSR {{ SAPD: {=bool:?}, SVCC_EN: {=bool:?}, VCCENP: {=bool:?}, SRST: {=bool:?}, SCEN: {=bool:?}, SCSP: {=bool:?}, SPD: {=bool:?}, SPDIM: {=bool:?}, SPDIF: {=bool:?}, SPDP: {=bool:?}, SPDES: {=bool:?} }}" , self . SAPD () , self . SVCC_EN () , self . VCCENP () , self . SRST () , self . SCEN () , self . SCSP () , self . SPD () , self . SPDIM () , self . SPDIF () , self . SPDP () , self . SPDES ())
        }
    }
    #[doc = "Receive Data Read Buffer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_BUF(pub u32);
    impl RX_BUF {
        #[must_use]
        #[inline(always)]
        pub const fn RX_BYTE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RX_BYTE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for RX_BUF {
        #[inline(always)]
        fn default() -> RX_BUF {
            RX_BUF(0)
        }
    }
    impl core::fmt::Debug for RX_BUF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_BUF")
                .field("RX_BYTE", &self.RX_BYTE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_BUF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RX_BUF {{ RX_BYTE: {=u8:?} }}", self.RX_BYTE())
        }
    }
    #[doc = "Receive Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_STATUS(pub u32);
    impl RX_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn RFO(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_DATA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RX_DATA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDTF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LRC_OK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LRC_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC_OK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWT_ERR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWT_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWT_ERR(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWT_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGT_ERR(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BGT_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PEF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEF(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_WPTR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RX_WPTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RX_CNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RX_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for RX_STATUS {
        #[inline(always)]
        fn default() -> RX_STATUS {
            RX_STATUS(0)
        }
    }
    impl core::fmt::Debug for RX_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_STATUS")
                .field("RFO", &self.RFO())
                .field("RX_DATA", &self.RX_DATA())
                .field("RDTF", &self.RDTF())
                .field("LRC_OK", &self.LRC_OK())
                .field("CRC_OK", &self.CRC_OK())
                .field("CWT_ERR", &self.CWT_ERR())
                .field("RTE", &self.RTE())
                .field("BWT_ERR", &self.BWT_ERR())
                .field("BGT_ERR", &self.BGT_ERR())
                .field("PEF", &self.PEF())
                .field("FEF", &self.FEF())
                .field("RX_WPTR", &self.RX_WPTR())
                .field("RX_CNT", &self.RX_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RX_STATUS {{ RFO: {=bool:?}, RX_DATA: {=bool:?}, RDTF: {=bool:?}, LRC_OK: {=bool:?}, CRC_OK: {=bool:?}, CWT_ERR: {=bool:?}, RTE: {=bool:?}, BWT_ERR: {=bool:?}, BGT_ERR: {=bool:?}, PEF: {=bool:?}, FEF: {=bool:?}, RX_WPTR: {=u8:?}, RX_CNT: {=u8:?} }}" , self . RFO () , self . RX_DATA () , self . RDTF () , self . LRC_OK () , self . CRC_OK () , self . CWT_ERR () , self . RTE () , self . BWT_ERR () , self . BGT_ERR () , self . PEF () , self . FEF () , self . RX_WPTR () , self . RX_CNT ())
        }
    }
    #[doc = "Receiver Threshold"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_THD(pub u32);
    impl RX_THD {
        #[must_use]
        #[inline(always)]
        pub const fn RDT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RDT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RNCK_THD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RNCK_THD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for RX_THD {
        #[inline(always)]
        fn default() -> RX_THD {
            RX_THD(0)
        }
    }
    impl core::fmt::Debug for RX_THD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_THD")
                .field("RDT", &self.RDT())
                .field("RNCK_THD", &self.RNCK_THD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_THD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RX_THD {{ RDT: {=u8:?}, RNCK_THD: {=u8:?} }}",
                self.RDT(),
                self.RNCK_THD()
            )
        }
    }
    #[doc = "Transmit Data Buffer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_BUF(pub u32);
    impl TX_BUF {
        #[must_use]
        #[inline(always)]
        pub const fn TX_BYTE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TX_BYTE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TX_BUF {
        #[inline(always)]
        fn default() -> TX_BUF {
            TX_BUF(0)
        }
    }
    impl core::fmt::Debug for TX_BUF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_BUF")
                .field("TX_BYTE", &self.TX_BYTE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_BUF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TX_BUF {{ TX_BYTE: {=u8:?} }}", self.TX_BYTE())
        }
    }
    #[doc = "Transmitter Guard ETU Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_GETU(pub u32);
    impl TX_GETU {
        #[must_use]
        #[inline(always)]
        pub const fn GETU(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GETU(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TX_GETU {
        #[inline(always)]
        fn default() -> TX_GETU {
            TX_GETU(0)
        }
    }
    impl core::fmt::Debug for TX_GETU {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_GETU")
                .field("GETU", &self.GETU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_GETU {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TX_GETU {{ GETU: {=u8:?} }}", self.GETU())
        }
    }
    #[doc = "Transmitter Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_STATUS(pub u32);
    impl TX_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn TNTE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TNTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETCF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDTF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TDTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT0_TO(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GPCNT0_TO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPCNT1_TO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GPCNT1_TO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TX_RPTR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TX_RPTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TX_CNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TX_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for TX_STATUS {
        #[inline(always)]
        fn default() -> TX_STATUS {
            TX_STATUS(0)
        }
    }
    impl core::fmt::Debug for TX_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_STATUS")
                .field("TNTE", &self.TNTE())
                .field("TFE", &self.TFE())
                .field("ETCF", &self.ETCF())
                .field("TCF", &self.TCF())
                .field("TFF", &self.TFF())
                .field("TDTF", &self.TDTF())
                .field("GPCNT0_TO", &self.GPCNT0_TO())
                .field("GPCNT1_TO", &self.GPCNT1_TO())
                .field("TX_RPTR", &self.TX_RPTR())
                .field("TX_CNT", &self.TX_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TX_STATUS {{ TNTE: {=bool:?}, TFE: {=bool:?}, ETCF: {=bool:?}, TCF: {=bool:?}, TFF: {=bool:?}, TDTF: {=bool:?}, GPCNT0_TO: {=bool:?}, GPCNT1_TO: {=bool:?}, TX_RPTR: {=u8:?}, TX_CNT: {=u8:?} }}" , self . TNTE () , self . TFE () , self . ETCF () , self . TCF () , self . TFF () , self . TDTF () , self . GPCNT0_TO () , self . GPCNT1_TO () , self . TX_RPTR () , self . TX_CNT ())
        }
    }
    #[doc = "Transmitter Threshold"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_THD(pub u32);
    impl TX_THD {
        #[must_use]
        #[inline(always)]
        pub const fn TDT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TDT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNCK_THD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNCK_THD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for TX_THD {
        #[inline(always)]
        fn default() -> TX_THD {
            TX_THD(0)
        }
    }
    impl core::fmt::Debug for TX_THD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_THD")
                .field("TDT", &self.TDT())
                .field("TNCK_THD", &self.TNCK_THD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_THD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TX_THD {{ TDT: {=u8:?}, TNCK_THD: {=u8:?} }}",
                self.TDT(),
                self.TNCK_THD()
            )
        }
    }
}
