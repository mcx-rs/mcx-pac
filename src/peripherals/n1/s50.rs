#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S50 {
    ptr: *mut u8,
}
unsafe impl Send for S50 {}
unsafe impl Sync for S50 {}
impl S50 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ELS_STATUS(self) -> crate::common::Reg<regs::ELS_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_CTRL(self) -> crate::common::Reg<regs::ELS_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_CMDCFG0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_CFG(self) -> crate::common::Reg<regs::ELS_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KIDX0(self) -> crate::common::Reg<regs::ELS_KIDX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KIDX1(self) -> crate::common::Reg<regs::ELS_KIDX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KPROPIN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_SRC0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_SRC0_LEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_SRC1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_SRC2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_SRC2_LEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_RES0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_RES0_LEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_INT_ENABLE(
        self,
    ) -> crate::common::Reg<regs::ELS_INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_INT_STATUS_CLR(
        self,
    ) -> crate::common::Reg<regs::ELS_INT_STATUS_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_INT_STATUS_SET(
        self,
    ) -> crate::common::Reg<regs::ELS_INT_STATUS_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_ERR_STATUS(
        self,
    ) -> crate::common::Reg<regs::ELS_ERR_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_ERR_STATUS_CLR(
        self,
    ) -> crate::common::Reg<regs::ELS_ERR_STATUS_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_VERSION(self) -> crate::common::Reg<regs::ELS_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_PRNG_DATOUT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_CMDCRC_CTRL(
        self,
    ) -> crate::common::Reg<regs::ELS_CMDCRC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_CMDCRC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_SESSION_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_DMA_FIN_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_MASTER_ID(self) -> crate::common::Reg<regs::ELS_MASTER_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KIDX2(self) -> crate::common::Reg<regs::ELS_KIDX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS0(self) -> crate::common::Reg<regs::ELS_KS0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS1(self) -> crate::common::Reg<regs::ELS_KS1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS2(self) -> crate::common::Reg<regs::ELS_KS2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS3(self) -> crate::common::Reg<regs::ELS_KS3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS4(self) -> crate::common::Reg<regs::ELS_KS4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS5(self) -> crate::common::Reg<regs::ELS_KS5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS6(self) -> crate::common::Reg<regs::ELS_KS6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS7(self) -> crate::common::Reg<regs::ELS_KS7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS8(self) -> crate::common::Reg<regs::ELS_KS8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS9(self) -> crate::common::Reg<regs::ELS_KS9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS10(self) -> crate::common::Reg<regs::ELS_KS10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS11(self) -> crate::common::Reg<regs::ELS_KS11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS12(self) -> crate::common::Reg<regs::ELS_KS12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS13(self) -> crate::common::Reg<regs::ELS_KS13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS14(self) -> crate::common::Reg<regs::ELS_KS14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS15(self) -> crate::common::Reg<regs::ELS_KS15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS16(self) -> crate::common::Reg<regs::ELS_KS16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS17(self) -> crate::common::Reg<regs::ELS_KS17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS18(self) -> crate::common::Reg<regs::ELS_KS18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KS19(self) -> crate::common::Reg<regs::ELS_KS19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_CFG(pub u32);
    impl ELS_CFG {
        #[inline(always)]
        pub const fn ADCTRL(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ADCTRL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for ELS_CFG {
        #[inline(always)]
        fn default() -> ELS_CFG {
            ELS_CFG(0)
        }
    }
    impl core::fmt::Debug for ELS_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_CFG")
                .field("ADCTRL", &self.ADCTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_CFG {
                ADCTRL: u16,
            }
            let proxy = ELS_CFG {
                ADCTRL: self.ADCTRL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CRC Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_CMDCRC_CTRL(pub u32);
    impl ELS_CMDCRC_CTRL {
        #[inline(always)]
        pub const fn CMDCRC_RST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMDCRC_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CMDCRC_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMDCRC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for ELS_CMDCRC_CTRL {
        #[inline(always)]
        fn default() -> ELS_CMDCRC_CTRL {
            ELS_CMDCRC_CTRL(0)
        }
    }
    impl core::fmt::Debug for ELS_CMDCRC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_CMDCRC_CTRL")
                .field("CMDCRC_RST", &self.CMDCRC_RST())
                .field("CMDCRC_EN", &self.CMDCRC_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_CMDCRC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_CMDCRC_CTRL {
                CMDCRC_RST: bool,
                CMDCRC_EN: bool,
            }
            let proxy = ELS_CMDCRC_CTRL {
                CMDCRC_RST: self.CMDCRC_RST(),
                CMDCRC_EN: self.CMDCRC_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_CTRL(pub u32);
    impl ELS_CTRL {
        #[inline(always)]
        pub const fn ELS_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ELS_START(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ELS_RESET(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ELS_CMD(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ELS_CMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[inline(always)]
        pub const fn BYTE_ORDER(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYTE_ORDER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for ELS_CTRL {
        #[inline(always)]
        fn default() -> ELS_CTRL {
            ELS_CTRL(0)
        }
    }
    impl core::fmt::Debug for ELS_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_CTRL")
                .field("ELS_EN", &self.ELS_EN())
                .field("ELS_START", &self.ELS_START())
                .field("ELS_RESET", &self.ELS_RESET())
                .field("ELS_CMD", &self.ELS_CMD())
                .field("BYTE_ORDER", &self.BYTE_ORDER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_CTRL {
                ELS_EN: bool,
                ELS_START: bool,
                ELS_RESET: bool,
                ELS_CMD: u8,
                BYTE_ORDER: bool,
            }
            let proxy = ELS_CTRL {
                ELS_EN: self.ELS_EN(),
                ELS_START: self.ELS_START(),
                ELS_RESET: self.ELS_RESET(),
                ELS_CMD: self.ELS_CMD(),
                BYTE_ORDER: self.BYTE_ORDER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_ERR_STATUS(pub u32);
    impl ELS_ERR_STATUS {
        #[inline(always)]
        pub const fn BUS_ERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUS_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OPN_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPN_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ALG_ERR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALG_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ITG_ERR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ITG_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FLT_ERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLT_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PRNG_ERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PRNG_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ERR_LVL(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERR_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn DTRNG_ERR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DTRNG_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for ELS_ERR_STATUS {
        #[inline(always)]
        fn default() -> ELS_ERR_STATUS {
            ELS_ERR_STATUS(0)
        }
    }
    impl core::fmt::Debug for ELS_ERR_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_ERR_STATUS")
                .field("BUS_ERR", &self.BUS_ERR())
                .field("OPN_ERR", &self.OPN_ERR())
                .field("ALG_ERR", &self.ALG_ERR())
                .field("ITG_ERR", &self.ITG_ERR())
                .field("FLT_ERR", &self.FLT_ERR())
                .field("PRNG_ERR", &self.PRNG_ERR())
                .field("ERR_LVL", &self.ERR_LVL())
                .field("DTRNG_ERR", &self.DTRNG_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_ERR_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_ERR_STATUS {
                BUS_ERR: bool,
                OPN_ERR: bool,
                ALG_ERR: bool,
                ITG_ERR: bool,
                FLT_ERR: bool,
                PRNG_ERR: bool,
                ERR_LVL: u8,
                DTRNG_ERR: bool,
            }
            let proxy = ELS_ERR_STATUS {
                BUS_ERR: self.BUS_ERR(),
                OPN_ERR: self.OPN_ERR(),
                ALG_ERR: self.ALG_ERR(),
                ITG_ERR: self.ITG_ERR(),
                FLT_ERR: self.FLT_ERR(),
                PRNG_ERR: self.PRNG_ERR(),
                ERR_LVL: self.ERR_LVL(),
                DTRNG_ERR: self.DTRNG_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Status Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_ERR_STATUS_CLR(pub u32);
    impl ELS_ERR_STATUS_CLR {
        #[inline(always)]
        pub const fn ERR_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERR_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ELS_ERR_STATUS_CLR {
        #[inline(always)]
        fn default() -> ELS_ERR_STATUS_CLR {
            ELS_ERR_STATUS_CLR(0)
        }
    }
    impl core::fmt::Debug for ELS_ERR_STATUS_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_ERR_STATUS_CLR")
                .field("ERR_CLR", &self.ERR_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_ERR_STATUS_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_ERR_STATUS_CLR {
                ERR_CLR: bool,
            }
            let proxy = ELS_ERR_STATUS_CLR {
                ERR_CLR: self.ERR_CLR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_INT_ENABLE(pub u32);
    impl ELS_INT_ENABLE {
        #[inline(always)]
        pub const fn INT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ELS_INT_ENABLE {
        #[inline(always)]
        fn default() -> ELS_INT_ENABLE {
            ELS_INT_ENABLE(0)
        }
    }
    impl core::fmt::Debug for ELS_INT_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_INT_ENABLE")
                .field("INT_EN", &self.INT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_INT_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_INT_ENABLE {
                INT_EN: bool,
            }
            let proxy = ELS_INT_ENABLE {
                INT_EN: self.INT_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_INT_STATUS_CLR(pub u32);
    impl ELS_INT_STATUS_CLR {
        #[inline(always)]
        pub const fn INT_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ELS_INT_STATUS_CLR {
        #[inline(always)]
        fn default() -> ELS_INT_STATUS_CLR {
            ELS_INT_STATUS_CLR(0)
        }
    }
    impl core::fmt::Debug for ELS_INT_STATUS_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_INT_STATUS_CLR")
                .field("INT_CLR", &self.INT_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_INT_STATUS_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_INT_STATUS_CLR {
                INT_CLR: bool,
            }
            let proxy = ELS_INT_STATUS_CLR {
                INT_CLR: self.INT_CLR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_INT_STATUS_SET(pub u32);
    impl ELS_INT_STATUS_SET {
        #[inline(always)]
        pub const fn INT_SET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ELS_INT_STATUS_SET {
        #[inline(always)]
        fn default() -> ELS_INT_STATUS_SET {
            ELS_INT_STATUS_SET(0)
        }
    }
    impl core::fmt::Debug for ELS_INT_STATUS_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_INT_STATUS_SET")
                .field("INT_SET", &self.INT_SET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_INT_STATUS_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_INT_STATUS_SET {
                INT_SET: bool,
            }
            let proxy = ELS_INT_STATUS_SET {
                INT_SET: self.INT_SET(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Keystore Index 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KIDX0(pub u32);
    impl ELS_KIDX0 {
        #[inline(always)]
        pub const fn KIDX0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_KIDX0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for ELS_KIDX0 {
        #[inline(always)]
        fn default() -> ELS_KIDX0 {
            ELS_KIDX0(0)
        }
    }
    impl core::fmt::Debug for ELS_KIDX0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KIDX0")
                .field("KIDX0", &self.KIDX0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KIDX0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KIDX0 {
                KIDX0: u8,
            }
            let proxy = ELS_KIDX0 {
                KIDX0: self.KIDX0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Keystore Index 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KIDX1(pub u32);
    impl ELS_KIDX1 {
        #[inline(always)]
        pub const fn KIDX1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_KIDX1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for ELS_KIDX1 {
        #[inline(always)]
        fn default() -> ELS_KIDX1 {
            ELS_KIDX1(0)
        }
    }
    impl core::fmt::Debug for ELS_KIDX1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KIDX1")
                .field("KIDX1", &self.KIDX1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KIDX1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KIDX1 {
                KIDX1: u8,
            }
            let proxy = ELS_KIDX1 {
                KIDX1: self.KIDX1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Keystore Index 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KIDX2(pub u32);
    impl ELS_KIDX2 {
        #[inline(always)]
        pub const fn KIDX2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_KIDX2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for ELS_KIDX2 {
        #[inline(always)]
        fn default() -> ELS_KIDX2 {
            ELS_KIDX2(0)
        }
    }
    impl core::fmt::Debug for ELS_KIDX2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KIDX2")
                .field("KIDX2", &self.KIDX2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KIDX2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KIDX2 {
                KIDX2: u8,
            }
            let proxy = ELS_KIDX2 {
                KIDX2: self.KIDX2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS0(pub u32);
    impl ELS_KS0 {
        #[inline(always)]
        pub const fn KS0_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS0_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS0_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS0_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS0_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS0_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS0_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS0_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS0_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS0_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS0_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS0_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS0_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS0_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS0_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS0_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS0_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS0_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS0_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS0_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS0_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS0_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS0_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS0_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS0_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS0_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS0_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS0_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS0_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS0 {
        #[inline(always)]
        fn default() -> ELS_KS0 {
            ELS_KS0(0)
        }
    }
    impl core::fmt::Debug for ELS_KS0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS0")
                .field("KS0_KSIZE", &self.KS0_KSIZE())
                .field("KS0_KACT", &self.KS0_KACT())
                .field("KS0_KBASE", &self.KS0_KBASE())
                .field("KS0_FGP", &self.KS0_FGP())
                .field("KS0_FRTN", &self.KS0_FRTN())
                .field("KS0_FHWO", &self.KS0_FHWO())
                .field("KS0_UKPUK", &self.KS0_UKPUK())
                .field("KS0_UTECDH", &self.KS0_UTECDH())
                .field("KS0_UCMAC", &self.KS0_UCMAC())
                .field("KS0_UKSK", &self.KS0_UKSK())
                .field("KS0_URTF", &self.KS0_URTF())
                .field("KS0_UCKDF", &self.KS0_UCKDF())
                .field("KS0_UHKDF", &self.KS0_UHKDF())
                .field("KS0_UECSG", &self.KS0_UECSG())
                .field("KS0_UECDH", &self.KS0_UECDH())
                .field("KS0_UAES", &self.KS0_UAES())
                .field("KS0_UHMAC", &self.KS0_UHMAC())
                .field("KS0_UKWK", &self.KS0_UKWK())
                .field("KS0_UKUOK", &self.KS0_UKUOK())
                .field("KS0_UTLSPMS", &self.KS0_UTLSPMS())
                .field("KS0_UTLSMS", &self.KS0_UTLSMS())
                .field("KS0_UKGSRC", &self.KS0_UKGSRC())
                .field("KS0_UHWO", &self.KS0_UHWO())
                .field("KS0_UWRPOK", &self.KS0_UWRPOK())
                .field("KS0_UDUK", &self.KS0_UDUK())
                .field("KS0_UPPROT", &self.KS0_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS0 {
                KS0_KSIZE: u8,
                KS0_KACT: bool,
                KS0_KBASE: bool,
                KS0_FGP: bool,
                KS0_FRTN: bool,
                KS0_FHWO: bool,
                KS0_UKPUK: bool,
                KS0_UTECDH: bool,
                KS0_UCMAC: bool,
                KS0_UKSK: bool,
                KS0_URTF: bool,
                KS0_UCKDF: bool,
                KS0_UHKDF: bool,
                KS0_UECSG: bool,
                KS0_UECDH: bool,
                KS0_UAES: bool,
                KS0_UHMAC: bool,
                KS0_UKWK: bool,
                KS0_UKUOK: bool,
                KS0_UTLSPMS: bool,
                KS0_UTLSMS: bool,
                KS0_UKGSRC: bool,
                KS0_UHWO: bool,
                KS0_UWRPOK: bool,
                KS0_UDUK: bool,
                KS0_UPPROT: u8,
            }
            let proxy = ELS_KS0 {
                KS0_KSIZE: self.KS0_KSIZE(),
                KS0_KACT: self.KS0_KACT(),
                KS0_KBASE: self.KS0_KBASE(),
                KS0_FGP: self.KS0_FGP(),
                KS0_FRTN: self.KS0_FRTN(),
                KS0_FHWO: self.KS0_FHWO(),
                KS0_UKPUK: self.KS0_UKPUK(),
                KS0_UTECDH: self.KS0_UTECDH(),
                KS0_UCMAC: self.KS0_UCMAC(),
                KS0_UKSK: self.KS0_UKSK(),
                KS0_URTF: self.KS0_URTF(),
                KS0_UCKDF: self.KS0_UCKDF(),
                KS0_UHKDF: self.KS0_UHKDF(),
                KS0_UECSG: self.KS0_UECSG(),
                KS0_UECDH: self.KS0_UECDH(),
                KS0_UAES: self.KS0_UAES(),
                KS0_UHMAC: self.KS0_UHMAC(),
                KS0_UKWK: self.KS0_UKWK(),
                KS0_UKUOK: self.KS0_UKUOK(),
                KS0_UTLSPMS: self.KS0_UTLSPMS(),
                KS0_UTLSMS: self.KS0_UTLSMS(),
                KS0_UKGSRC: self.KS0_UKGSRC(),
                KS0_UHWO: self.KS0_UHWO(),
                KS0_UWRPOK: self.KS0_UWRPOK(),
                KS0_UDUK: self.KS0_UDUK(),
                KS0_UPPROT: self.KS0_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS1(pub u32);
    impl ELS_KS1 {
        #[inline(always)]
        pub const fn KS1_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS1_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS1_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS1_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS1_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS1_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS1_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS1_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS1_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS1_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS1_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS1_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS1_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS1_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS1_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS1_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS1_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS1_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS1_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS1_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS1_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS1_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS1_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS1_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS1_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS1_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS1_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS1_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS1_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS1 {
        #[inline(always)]
        fn default() -> ELS_KS1 {
            ELS_KS1(0)
        }
    }
    impl core::fmt::Debug for ELS_KS1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS1")
                .field("KS1_KSIZE", &self.KS1_KSIZE())
                .field("KS1_KACT", &self.KS1_KACT())
                .field("KS1_KBASE", &self.KS1_KBASE())
                .field("KS1_FGP", &self.KS1_FGP())
                .field("KS1_FRTN", &self.KS1_FRTN())
                .field("KS1_FHWO", &self.KS1_FHWO())
                .field("KS1_UKPUK", &self.KS1_UKPUK())
                .field("KS1_UTECDH", &self.KS1_UTECDH())
                .field("KS1_UCMAC", &self.KS1_UCMAC())
                .field("KS1_UKSK", &self.KS1_UKSK())
                .field("KS1_URTF", &self.KS1_URTF())
                .field("KS1_UCKDF", &self.KS1_UCKDF())
                .field("KS1_UHKDF", &self.KS1_UHKDF())
                .field("KS1_UECSG", &self.KS1_UECSG())
                .field("KS1_UECDH", &self.KS1_UECDH())
                .field("KS1_UAES", &self.KS1_UAES())
                .field("KS1_UHMAC", &self.KS1_UHMAC())
                .field("KS1_UKWK", &self.KS1_UKWK())
                .field("KS1_UKUOK", &self.KS1_UKUOK())
                .field("KS1_UTLSPMS", &self.KS1_UTLSPMS())
                .field("KS1_UTLSMS", &self.KS1_UTLSMS())
                .field("KS1_UKGSRC", &self.KS1_UKGSRC())
                .field("KS1_UHWO", &self.KS1_UHWO())
                .field("KS1_UWRPOK", &self.KS1_UWRPOK())
                .field("KS1_UDUK", &self.KS1_UDUK())
                .field("KS1_UPPROT", &self.KS1_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS1 {
                KS1_KSIZE: u8,
                KS1_KACT: bool,
                KS1_KBASE: bool,
                KS1_FGP: bool,
                KS1_FRTN: bool,
                KS1_FHWO: bool,
                KS1_UKPUK: bool,
                KS1_UTECDH: bool,
                KS1_UCMAC: bool,
                KS1_UKSK: bool,
                KS1_URTF: bool,
                KS1_UCKDF: bool,
                KS1_UHKDF: bool,
                KS1_UECSG: bool,
                KS1_UECDH: bool,
                KS1_UAES: bool,
                KS1_UHMAC: bool,
                KS1_UKWK: bool,
                KS1_UKUOK: bool,
                KS1_UTLSPMS: bool,
                KS1_UTLSMS: bool,
                KS1_UKGSRC: bool,
                KS1_UHWO: bool,
                KS1_UWRPOK: bool,
                KS1_UDUK: bool,
                KS1_UPPROT: u8,
            }
            let proxy = ELS_KS1 {
                KS1_KSIZE: self.KS1_KSIZE(),
                KS1_KACT: self.KS1_KACT(),
                KS1_KBASE: self.KS1_KBASE(),
                KS1_FGP: self.KS1_FGP(),
                KS1_FRTN: self.KS1_FRTN(),
                KS1_FHWO: self.KS1_FHWO(),
                KS1_UKPUK: self.KS1_UKPUK(),
                KS1_UTECDH: self.KS1_UTECDH(),
                KS1_UCMAC: self.KS1_UCMAC(),
                KS1_UKSK: self.KS1_UKSK(),
                KS1_URTF: self.KS1_URTF(),
                KS1_UCKDF: self.KS1_UCKDF(),
                KS1_UHKDF: self.KS1_UHKDF(),
                KS1_UECSG: self.KS1_UECSG(),
                KS1_UECDH: self.KS1_UECDH(),
                KS1_UAES: self.KS1_UAES(),
                KS1_UHMAC: self.KS1_UHMAC(),
                KS1_UKWK: self.KS1_UKWK(),
                KS1_UKUOK: self.KS1_UKUOK(),
                KS1_UTLSPMS: self.KS1_UTLSPMS(),
                KS1_UTLSMS: self.KS1_UTLSMS(),
                KS1_UKGSRC: self.KS1_UKGSRC(),
                KS1_UHWO: self.KS1_UHWO(),
                KS1_UWRPOK: self.KS1_UWRPOK(),
                KS1_UDUK: self.KS1_UDUK(),
                KS1_UPPROT: self.KS1_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS10(pub u32);
    impl ELS_KS10 {
        #[inline(always)]
        pub const fn KS10_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS10_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS10_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS10_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS10_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS10_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS10_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS10_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS10_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS10_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS10_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS10_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS10_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS10_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS10_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS10_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS10_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS10_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS10_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS10_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS10_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS10_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS10_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS10_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS10_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS10_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS10_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS10_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS10_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS10 {
        #[inline(always)]
        fn default() -> ELS_KS10 {
            ELS_KS10(0)
        }
    }
    impl core::fmt::Debug for ELS_KS10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS10")
                .field("KS10_KSIZE", &self.KS10_KSIZE())
                .field("KS10_KACT", &self.KS10_KACT())
                .field("KS10_KBASE", &self.KS10_KBASE())
                .field("KS10_FGP", &self.KS10_FGP())
                .field("KS10_FRTN", &self.KS10_FRTN())
                .field("KS10_FHWO", &self.KS10_FHWO())
                .field("KS10_UKPUK", &self.KS10_UKPUK())
                .field("KS10_UTECDH", &self.KS10_UTECDH())
                .field("KS10_UCMAC", &self.KS10_UCMAC())
                .field("KS10_UKSK", &self.KS10_UKSK())
                .field("KS10_URTF", &self.KS10_URTF())
                .field("KS10_UCKDF", &self.KS10_UCKDF())
                .field("KS10_UHKDF", &self.KS10_UHKDF())
                .field("KS10_UECSG", &self.KS10_UECSG())
                .field("KS10_UECDH", &self.KS10_UECDH())
                .field("KS10_UAES", &self.KS10_UAES())
                .field("KS10_UHMAC", &self.KS10_UHMAC())
                .field("KS10_UKWK", &self.KS10_UKWK())
                .field("KS10_UKUOK", &self.KS10_UKUOK())
                .field("KS10_UTLSPMS", &self.KS10_UTLSPMS())
                .field("KS10_UTLSMS", &self.KS10_UTLSMS())
                .field("KS10_UKGSRC", &self.KS10_UKGSRC())
                .field("KS10_UHWO", &self.KS10_UHWO())
                .field("KS10_UWRPOK", &self.KS10_UWRPOK())
                .field("KS10_UDUK", &self.KS10_UDUK())
                .field("KS10_UPPROT", &self.KS10_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS10 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS10 {
                KS10_KSIZE: u8,
                KS10_KACT: bool,
                KS10_KBASE: bool,
                KS10_FGP: bool,
                KS10_FRTN: bool,
                KS10_FHWO: bool,
                KS10_UKPUK: bool,
                KS10_UTECDH: bool,
                KS10_UCMAC: bool,
                KS10_UKSK: bool,
                KS10_URTF: bool,
                KS10_UCKDF: bool,
                KS10_UHKDF: bool,
                KS10_UECSG: bool,
                KS10_UECDH: bool,
                KS10_UAES: bool,
                KS10_UHMAC: bool,
                KS10_UKWK: bool,
                KS10_UKUOK: bool,
                KS10_UTLSPMS: bool,
                KS10_UTLSMS: bool,
                KS10_UKGSRC: bool,
                KS10_UHWO: bool,
                KS10_UWRPOK: bool,
                KS10_UDUK: bool,
                KS10_UPPROT: u8,
            }
            let proxy = ELS_KS10 {
                KS10_KSIZE: self.KS10_KSIZE(),
                KS10_KACT: self.KS10_KACT(),
                KS10_KBASE: self.KS10_KBASE(),
                KS10_FGP: self.KS10_FGP(),
                KS10_FRTN: self.KS10_FRTN(),
                KS10_FHWO: self.KS10_FHWO(),
                KS10_UKPUK: self.KS10_UKPUK(),
                KS10_UTECDH: self.KS10_UTECDH(),
                KS10_UCMAC: self.KS10_UCMAC(),
                KS10_UKSK: self.KS10_UKSK(),
                KS10_URTF: self.KS10_URTF(),
                KS10_UCKDF: self.KS10_UCKDF(),
                KS10_UHKDF: self.KS10_UHKDF(),
                KS10_UECSG: self.KS10_UECSG(),
                KS10_UECDH: self.KS10_UECDH(),
                KS10_UAES: self.KS10_UAES(),
                KS10_UHMAC: self.KS10_UHMAC(),
                KS10_UKWK: self.KS10_UKWK(),
                KS10_UKUOK: self.KS10_UKUOK(),
                KS10_UTLSPMS: self.KS10_UTLSPMS(),
                KS10_UTLSMS: self.KS10_UTLSMS(),
                KS10_UKGSRC: self.KS10_UKGSRC(),
                KS10_UHWO: self.KS10_UHWO(),
                KS10_UWRPOK: self.KS10_UWRPOK(),
                KS10_UDUK: self.KS10_UDUK(),
                KS10_UPPROT: self.KS10_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS11(pub u32);
    impl ELS_KS11 {
        #[inline(always)]
        pub const fn KS11_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS11_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS11_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS11_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS11_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS11_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS11_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS11_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS11_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS11_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS11_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS11_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS11_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS11_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS11_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS11_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS11_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS11_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS11_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS11_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS11_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS11_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS11_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS11_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS11_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS11_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS11_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS11_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS11_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS11 {
        #[inline(always)]
        fn default() -> ELS_KS11 {
            ELS_KS11(0)
        }
    }
    impl core::fmt::Debug for ELS_KS11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS11")
                .field("KS11_KSIZE", &self.KS11_KSIZE())
                .field("KS11_KACT", &self.KS11_KACT())
                .field("KS11_KBASE", &self.KS11_KBASE())
                .field("KS11_FGP", &self.KS11_FGP())
                .field("KS11_FRTN", &self.KS11_FRTN())
                .field("KS11_FHWO", &self.KS11_FHWO())
                .field("KS11_UKPUK", &self.KS11_UKPUK())
                .field("KS11_UTECDH", &self.KS11_UTECDH())
                .field("KS11_UCMAC", &self.KS11_UCMAC())
                .field("KS11_UKSK", &self.KS11_UKSK())
                .field("KS11_URTF", &self.KS11_URTF())
                .field("KS11_UCKDF", &self.KS11_UCKDF())
                .field("KS11_UHKDF", &self.KS11_UHKDF())
                .field("KS11_UECSG", &self.KS11_UECSG())
                .field("KS11_UECDH", &self.KS11_UECDH())
                .field("KS11_UAES", &self.KS11_UAES())
                .field("KS11_UHMAC", &self.KS11_UHMAC())
                .field("KS11_UKWK", &self.KS11_UKWK())
                .field("KS11_UKUOK", &self.KS11_UKUOK())
                .field("KS11_UTLSPMS", &self.KS11_UTLSPMS())
                .field("KS11_UTLSMS", &self.KS11_UTLSMS())
                .field("KS11_UKGSRC", &self.KS11_UKGSRC())
                .field("KS11_UHWO", &self.KS11_UHWO())
                .field("KS11_UWRPOK", &self.KS11_UWRPOK())
                .field("KS11_UDUK", &self.KS11_UDUK())
                .field("KS11_UPPROT", &self.KS11_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS11 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS11 {
                KS11_KSIZE: u8,
                KS11_KACT: bool,
                KS11_KBASE: bool,
                KS11_FGP: bool,
                KS11_FRTN: bool,
                KS11_FHWO: bool,
                KS11_UKPUK: bool,
                KS11_UTECDH: bool,
                KS11_UCMAC: bool,
                KS11_UKSK: bool,
                KS11_URTF: bool,
                KS11_UCKDF: bool,
                KS11_UHKDF: bool,
                KS11_UECSG: bool,
                KS11_UECDH: bool,
                KS11_UAES: bool,
                KS11_UHMAC: bool,
                KS11_UKWK: bool,
                KS11_UKUOK: bool,
                KS11_UTLSPMS: bool,
                KS11_UTLSMS: bool,
                KS11_UKGSRC: bool,
                KS11_UHWO: bool,
                KS11_UWRPOK: bool,
                KS11_UDUK: bool,
                KS11_UPPROT: u8,
            }
            let proxy = ELS_KS11 {
                KS11_KSIZE: self.KS11_KSIZE(),
                KS11_KACT: self.KS11_KACT(),
                KS11_KBASE: self.KS11_KBASE(),
                KS11_FGP: self.KS11_FGP(),
                KS11_FRTN: self.KS11_FRTN(),
                KS11_FHWO: self.KS11_FHWO(),
                KS11_UKPUK: self.KS11_UKPUK(),
                KS11_UTECDH: self.KS11_UTECDH(),
                KS11_UCMAC: self.KS11_UCMAC(),
                KS11_UKSK: self.KS11_UKSK(),
                KS11_URTF: self.KS11_URTF(),
                KS11_UCKDF: self.KS11_UCKDF(),
                KS11_UHKDF: self.KS11_UHKDF(),
                KS11_UECSG: self.KS11_UECSG(),
                KS11_UECDH: self.KS11_UECDH(),
                KS11_UAES: self.KS11_UAES(),
                KS11_UHMAC: self.KS11_UHMAC(),
                KS11_UKWK: self.KS11_UKWK(),
                KS11_UKUOK: self.KS11_UKUOK(),
                KS11_UTLSPMS: self.KS11_UTLSPMS(),
                KS11_UTLSMS: self.KS11_UTLSMS(),
                KS11_UKGSRC: self.KS11_UKGSRC(),
                KS11_UHWO: self.KS11_UHWO(),
                KS11_UWRPOK: self.KS11_UWRPOK(),
                KS11_UDUK: self.KS11_UDUK(),
                KS11_UPPROT: self.KS11_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS12(pub u32);
    impl ELS_KS12 {
        #[inline(always)]
        pub const fn KS12_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS12_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS12_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS12_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS12_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS12_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS12_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS12_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS12_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS12_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS12_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS12_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS12_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS12_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS12_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS12_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS12_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS12_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS12_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS12_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS12_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS12_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS12_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS12_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS12_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS12_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS12_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS12_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS12_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS12 {
        #[inline(always)]
        fn default() -> ELS_KS12 {
            ELS_KS12(0)
        }
    }
    impl core::fmt::Debug for ELS_KS12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS12")
                .field("KS12_KSIZE", &self.KS12_KSIZE())
                .field("KS12_KACT", &self.KS12_KACT())
                .field("KS12_KBASE", &self.KS12_KBASE())
                .field("KS12_FGP", &self.KS12_FGP())
                .field("KS12_FRTN", &self.KS12_FRTN())
                .field("KS12_FHWO", &self.KS12_FHWO())
                .field("KS12_UKPUK", &self.KS12_UKPUK())
                .field("KS12_UTECDH", &self.KS12_UTECDH())
                .field("KS12_UCMAC", &self.KS12_UCMAC())
                .field("KS12_UKSK", &self.KS12_UKSK())
                .field("KS12_URTF", &self.KS12_URTF())
                .field("KS12_UCKDF", &self.KS12_UCKDF())
                .field("KS12_UHKDF", &self.KS12_UHKDF())
                .field("KS12_UECSG", &self.KS12_UECSG())
                .field("KS12_UECDH", &self.KS12_UECDH())
                .field("KS12_UAES", &self.KS12_UAES())
                .field("KS12_UHMAC", &self.KS12_UHMAC())
                .field("KS12_UKWK", &self.KS12_UKWK())
                .field("KS12_UKUOK", &self.KS12_UKUOK())
                .field("KS12_UTLSPMS", &self.KS12_UTLSPMS())
                .field("KS12_UTLSMS", &self.KS12_UTLSMS())
                .field("KS12_UKGSRC", &self.KS12_UKGSRC())
                .field("KS12_UHWO", &self.KS12_UHWO())
                .field("KS12_UWRPOK", &self.KS12_UWRPOK())
                .field("KS12_UDUK", &self.KS12_UDUK())
                .field("KS12_UPPROT", &self.KS12_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS12 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS12 {
                KS12_KSIZE: u8,
                KS12_KACT: bool,
                KS12_KBASE: bool,
                KS12_FGP: bool,
                KS12_FRTN: bool,
                KS12_FHWO: bool,
                KS12_UKPUK: bool,
                KS12_UTECDH: bool,
                KS12_UCMAC: bool,
                KS12_UKSK: bool,
                KS12_URTF: bool,
                KS12_UCKDF: bool,
                KS12_UHKDF: bool,
                KS12_UECSG: bool,
                KS12_UECDH: bool,
                KS12_UAES: bool,
                KS12_UHMAC: bool,
                KS12_UKWK: bool,
                KS12_UKUOK: bool,
                KS12_UTLSPMS: bool,
                KS12_UTLSMS: bool,
                KS12_UKGSRC: bool,
                KS12_UHWO: bool,
                KS12_UWRPOK: bool,
                KS12_UDUK: bool,
                KS12_UPPROT: u8,
            }
            let proxy = ELS_KS12 {
                KS12_KSIZE: self.KS12_KSIZE(),
                KS12_KACT: self.KS12_KACT(),
                KS12_KBASE: self.KS12_KBASE(),
                KS12_FGP: self.KS12_FGP(),
                KS12_FRTN: self.KS12_FRTN(),
                KS12_FHWO: self.KS12_FHWO(),
                KS12_UKPUK: self.KS12_UKPUK(),
                KS12_UTECDH: self.KS12_UTECDH(),
                KS12_UCMAC: self.KS12_UCMAC(),
                KS12_UKSK: self.KS12_UKSK(),
                KS12_URTF: self.KS12_URTF(),
                KS12_UCKDF: self.KS12_UCKDF(),
                KS12_UHKDF: self.KS12_UHKDF(),
                KS12_UECSG: self.KS12_UECSG(),
                KS12_UECDH: self.KS12_UECDH(),
                KS12_UAES: self.KS12_UAES(),
                KS12_UHMAC: self.KS12_UHMAC(),
                KS12_UKWK: self.KS12_UKWK(),
                KS12_UKUOK: self.KS12_UKUOK(),
                KS12_UTLSPMS: self.KS12_UTLSPMS(),
                KS12_UTLSMS: self.KS12_UTLSMS(),
                KS12_UKGSRC: self.KS12_UKGSRC(),
                KS12_UHWO: self.KS12_UHWO(),
                KS12_UWRPOK: self.KS12_UWRPOK(),
                KS12_UDUK: self.KS12_UDUK(),
                KS12_UPPROT: self.KS12_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS13(pub u32);
    impl ELS_KS13 {
        #[inline(always)]
        pub const fn KS13_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS13_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS13_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS13_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS13_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS13_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS13_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS13_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS13_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS13_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS13_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS13_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS13_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS13_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS13_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS13_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS13_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS13_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS13_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS13_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS13_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS13_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS13_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS13_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS13_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS13_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS13_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS13_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS13_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS13 {
        #[inline(always)]
        fn default() -> ELS_KS13 {
            ELS_KS13(0)
        }
    }
    impl core::fmt::Debug for ELS_KS13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS13")
                .field("KS13_KSIZE", &self.KS13_KSIZE())
                .field("KS13_KACT", &self.KS13_KACT())
                .field("KS13_KBASE", &self.KS13_KBASE())
                .field("KS13_FGP", &self.KS13_FGP())
                .field("KS13_FRTN", &self.KS13_FRTN())
                .field("KS13_FHWO", &self.KS13_FHWO())
                .field("KS13_UKPUK", &self.KS13_UKPUK())
                .field("KS13_UTECDH", &self.KS13_UTECDH())
                .field("KS13_UCMAC", &self.KS13_UCMAC())
                .field("KS13_UKSK", &self.KS13_UKSK())
                .field("KS13_URTF", &self.KS13_URTF())
                .field("KS13_UCKDF", &self.KS13_UCKDF())
                .field("KS13_UHKDF", &self.KS13_UHKDF())
                .field("KS13_UECSG", &self.KS13_UECSG())
                .field("KS13_UECDH", &self.KS13_UECDH())
                .field("KS13_UAES", &self.KS13_UAES())
                .field("KS13_UHMAC", &self.KS13_UHMAC())
                .field("KS13_UKWK", &self.KS13_UKWK())
                .field("KS13_UKUOK", &self.KS13_UKUOK())
                .field("KS13_UTLSPMS", &self.KS13_UTLSPMS())
                .field("KS13_UTLSMS", &self.KS13_UTLSMS())
                .field("KS13_UKGSRC", &self.KS13_UKGSRC())
                .field("KS13_UHWO", &self.KS13_UHWO())
                .field("KS13_UWRPOK", &self.KS13_UWRPOK())
                .field("KS13_UDUK", &self.KS13_UDUK())
                .field("KS13_UPPROT", &self.KS13_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS13 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS13 {
                KS13_KSIZE: u8,
                KS13_KACT: bool,
                KS13_KBASE: bool,
                KS13_FGP: bool,
                KS13_FRTN: bool,
                KS13_FHWO: bool,
                KS13_UKPUK: bool,
                KS13_UTECDH: bool,
                KS13_UCMAC: bool,
                KS13_UKSK: bool,
                KS13_URTF: bool,
                KS13_UCKDF: bool,
                KS13_UHKDF: bool,
                KS13_UECSG: bool,
                KS13_UECDH: bool,
                KS13_UAES: bool,
                KS13_UHMAC: bool,
                KS13_UKWK: bool,
                KS13_UKUOK: bool,
                KS13_UTLSPMS: bool,
                KS13_UTLSMS: bool,
                KS13_UKGSRC: bool,
                KS13_UHWO: bool,
                KS13_UWRPOK: bool,
                KS13_UDUK: bool,
                KS13_UPPROT: u8,
            }
            let proxy = ELS_KS13 {
                KS13_KSIZE: self.KS13_KSIZE(),
                KS13_KACT: self.KS13_KACT(),
                KS13_KBASE: self.KS13_KBASE(),
                KS13_FGP: self.KS13_FGP(),
                KS13_FRTN: self.KS13_FRTN(),
                KS13_FHWO: self.KS13_FHWO(),
                KS13_UKPUK: self.KS13_UKPUK(),
                KS13_UTECDH: self.KS13_UTECDH(),
                KS13_UCMAC: self.KS13_UCMAC(),
                KS13_UKSK: self.KS13_UKSK(),
                KS13_URTF: self.KS13_URTF(),
                KS13_UCKDF: self.KS13_UCKDF(),
                KS13_UHKDF: self.KS13_UHKDF(),
                KS13_UECSG: self.KS13_UECSG(),
                KS13_UECDH: self.KS13_UECDH(),
                KS13_UAES: self.KS13_UAES(),
                KS13_UHMAC: self.KS13_UHMAC(),
                KS13_UKWK: self.KS13_UKWK(),
                KS13_UKUOK: self.KS13_UKUOK(),
                KS13_UTLSPMS: self.KS13_UTLSPMS(),
                KS13_UTLSMS: self.KS13_UTLSMS(),
                KS13_UKGSRC: self.KS13_UKGSRC(),
                KS13_UHWO: self.KS13_UHWO(),
                KS13_UWRPOK: self.KS13_UWRPOK(),
                KS13_UDUK: self.KS13_UDUK(),
                KS13_UPPROT: self.KS13_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS14(pub u32);
    impl ELS_KS14 {
        #[inline(always)]
        pub const fn KS14_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS14_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS14_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS14_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS14_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS14_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS14_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS14_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS14_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS14_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS14_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS14_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS14_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS14_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS14_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS14_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS14_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS14_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS14_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS14_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS14_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS14_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS14_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS14_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS14_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS14_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS14_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS14_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS14_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS14 {
        #[inline(always)]
        fn default() -> ELS_KS14 {
            ELS_KS14(0)
        }
    }
    impl core::fmt::Debug for ELS_KS14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS14")
                .field("KS14_KSIZE", &self.KS14_KSIZE())
                .field("KS14_KACT", &self.KS14_KACT())
                .field("KS14_KBASE", &self.KS14_KBASE())
                .field("KS14_FGP", &self.KS14_FGP())
                .field("KS14_FRTN", &self.KS14_FRTN())
                .field("KS14_FHWO", &self.KS14_FHWO())
                .field("KS14_UKPUK", &self.KS14_UKPUK())
                .field("KS14_UTECDH", &self.KS14_UTECDH())
                .field("KS14_UCMAC", &self.KS14_UCMAC())
                .field("KS14_UKSK", &self.KS14_UKSK())
                .field("KS14_URTF", &self.KS14_URTF())
                .field("KS14_UCKDF", &self.KS14_UCKDF())
                .field("KS14_UHKDF", &self.KS14_UHKDF())
                .field("KS14_UECSG", &self.KS14_UECSG())
                .field("KS14_UECDH", &self.KS14_UECDH())
                .field("KS14_UAES", &self.KS14_UAES())
                .field("KS14_UHMAC", &self.KS14_UHMAC())
                .field("KS14_UKWK", &self.KS14_UKWK())
                .field("KS14_UKUOK", &self.KS14_UKUOK())
                .field("KS14_UTLSPMS", &self.KS14_UTLSPMS())
                .field("KS14_UTLSMS", &self.KS14_UTLSMS())
                .field("KS14_UKGSRC", &self.KS14_UKGSRC())
                .field("KS14_UHWO", &self.KS14_UHWO())
                .field("KS14_UWRPOK", &self.KS14_UWRPOK())
                .field("KS14_UDUK", &self.KS14_UDUK())
                .field("KS14_UPPROT", &self.KS14_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS14 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS14 {
                KS14_KSIZE: u8,
                KS14_KACT: bool,
                KS14_KBASE: bool,
                KS14_FGP: bool,
                KS14_FRTN: bool,
                KS14_FHWO: bool,
                KS14_UKPUK: bool,
                KS14_UTECDH: bool,
                KS14_UCMAC: bool,
                KS14_UKSK: bool,
                KS14_URTF: bool,
                KS14_UCKDF: bool,
                KS14_UHKDF: bool,
                KS14_UECSG: bool,
                KS14_UECDH: bool,
                KS14_UAES: bool,
                KS14_UHMAC: bool,
                KS14_UKWK: bool,
                KS14_UKUOK: bool,
                KS14_UTLSPMS: bool,
                KS14_UTLSMS: bool,
                KS14_UKGSRC: bool,
                KS14_UHWO: bool,
                KS14_UWRPOK: bool,
                KS14_UDUK: bool,
                KS14_UPPROT: u8,
            }
            let proxy = ELS_KS14 {
                KS14_KSIZE: self.KS14_KSIZE(),
                KS14_KACT: self.KS14_KACT(),
                KS14_KBASE: self.KS14_KBASE(),
                KS14_FGP: self.KS14_FGP(),
                KS14_FRTN: self.KS14_FRTN(),
                KS14_FHWO: self.KS14_FHWO(),
                KS14_UKPUK: self.KS14_UKPUK(),
                KS14_UTECDH: self.KS14_UTECDH(),
                KS14_UCMAC: self.KS14_UCMAC(),
                KS14_UKSK: self.KS14_UKSK(),
                KS14_URTF: self.KS14_URTF(),
                KS14_UCKDF: self.KS14_UCKDF(),
                KS14_UHKDF: self.KS14_UHKDF(),
                KS14_UECSG: self.KS14_UECSG(),
                KS14_UECDH: self.KS14_UECDH(),
                KS14_UAES: self.KS14_UAES(),
                KS14_UHMAC: self.KS14_UHMAC(),
                KS14_UKWK: self.KS14_UKWK(),
                KS14_UKUOK: self.KS14_UKUOK(),
                KS14_UTLSPMS: self.KS14_UTLSPMS(),
                KS14_UTLSMS: self.KS14_UTLSMS(),
                KS14_UKGSRC: self.KS14_UKGSRC(),
                KS14_UHWO: self.KS14_UHWO(),
                KS14_UWRPOK: self.KS14_UWRPOK(),
                KS14_UDUK: self.KS14_UDUK(),
                KS14_UPPROT: self.KS14_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS15(pub u32);
    impl ELS_KS15 {
        #[inline(always)]
        pub const fn KS15_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS15_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS15_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS15_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS15_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS15_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS15_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS15_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS15_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS15_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS15_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS15_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS15_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS15_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS15_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS15_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS15_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS15_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS15_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS15_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS15_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS15_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS15_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS15_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS15_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS15_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS15_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS15_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS15_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS15 {
        #[inline(always)]
        fn default() -> ELS_KS15 {
            ELS_KS15(0)
        }
    }
    impl core::fmt::Debug for ELS_KS15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS15")
                .field("KS15_KSIZE", &self.KS15_KSIZE())
                .field("KS15_KACT", &self.KS15_KACT())
                .field("KS15_KBASE", &self.KS15_KBASE())
                .field("KS15_FGP", &self.KS15_FGP())
                .field("KS15_FRTN", &self.KS15_FRTN())
                .field("KS15_FHWO", &self.KS15_FHWO())
                .field("KS15_UKPUK", &self.KS15_UKPUK())
                .field("KS15_UTECDH", &self.KS15_UTECDH())
                .field("KS15_UCMAC", &self.KS15_UCMAC())
                .field("KS15_UKSK", &self.KS15_UKSK())
                .field("KS15_URTF", &self.KS15_URTF())
                .field("KS15_UCKDF", &self.KS15_UCKDF())
                .field("KS15_UHKDF", &self.KS15_UHKDF())
                .field("KS15_UECSG", &self.KS15_UECSG())
                .field("KS15_UECDH", &self.KS15_UECDH())
                .field("KS15_UAES", &self.KS15_UAES())
                .field("KS15_UHMAC", &self.KS15_UHMAC())
                .field("KS15_UKWK", &self.KS15_UKWK())
                .field("KS15_UKUOK", &self.KS15_UKUOK())
                .field("KS15_UTLSPMS", &self.KS15_UTLSPMS())
                .field("KS15_UTLSMS", &self.KS15_UTLSMS())
                .field("KS15_UKGSRC", &self.KS15_UKGSRC())
                .field("KS15_UHWO", &self.KS15_UHWO())
                .field("KS15_UWRPOK", &self.KS15_UWRPOK())
                .field("KS15_UDUK", &self.KS15_UDUK())
                .field("KS15_UPPROT", &self.KS15_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS15 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS15 {
                KS15_KSIZE: u8,
                KS15_KACT: bool,
                KS15_KBASE: bool,
                KS15_FGP: bool,
                KS15_FRTN: bool,
                KS15_FHWO: bool,
                KS15_UKPUK: bool,
                KS15_UTECDH: bool,
                KS15_UCMAC: bool,
                KS15_UKSK: bool,
                KS15_URTF: bool,
                KS15_UCKDF: bool,
                KS15_UHKDF: bool,
                KS15_UECSG: bool,
                KS15_UECDH: bool,
                KS15_UAES: bool,
                KS15_UHMAC: bool,
                KS15_UKWK: bool,
                KS15_UKUOK: bool,
                KS15_UTLSPMS: bool,
                KS15_UTLSMS: bool,
                KS15_UKGSRC: bool,
                KS15_UHWO: bool,
                KS15_UWRPOK: bool,
                KS15_UDUK: bool,
                KS15_UPPROT: u8,
            }
            let proxy = ELS_KS15 {
                KS15_KSIZE: self.KS15_KSIZE(),
                KS15_KACT: self.KS15_KACT(),
                KS15_KBASE: self.KS15_KBASE(),
                KS15_FGP: self.KS15_FGP(),
                KS15_FRTN: self.KS15_FRTN(),
                KS15_FHWO: self.KS15_FHWO(),
                KS15_UKPUK: self.KS15_UKPUK(),
                KS15_UTECDH: self.KS15_UTECDH(),
                KS15_UCMAC: self.KS15_UCMAC(),
                KS15_UKSK: self.KS15_UKSK(),
                KS15_URTF: self.KS15_URTF(),
                KS15_UCKDF: self.KS15_UCKDF(),
                KS15_UHKDF: self.KS15_UHKDF(),
                KS15_UECSG: self.KS15_UECSG(),
                KS15_UECDH: self.KS15_UECDH(),
                KS15_UAES: self.KS15_UAES(),
                KS15_UHMAC: self.KS15_UHMAC(),
                KS15_UKWK: self.KS15_UKWK(),
                KS15_UKUOK: self.KS15_UKUOK(),
                KS15_UTLSPMS: self.KS15_UTLSPMS(),
                KS15_UTLSMS: self.KS15_UTLSMS(),
                KS15_UKGSRC: self.KS15_UKGSRC(),
                KS15_UHWO: self.KS15_UHWO(),
                KS15_UWRPOK: self.KS15_UWRPOK(),
                KS15_UDUK: self.KS15_UDUK(),
                KS15_UPPROT: self.KS15_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS16(pub u32);
    impl ELS_KS16 {
        #[inline(always)]
        pub const fn KS16_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS16_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS16_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS16_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS16_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS16_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS16_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS16_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS16_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS16_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS16_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS16_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS16_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS16_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS16_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS16_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS16_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS16_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS16_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS16_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS16_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS16_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS16_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS16_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS16_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS16_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS16_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS16_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS16_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS16 {
        #[inline(always)]
        fn default() -> ELS_KS16 {
            ELS_KS16(0)
        }
    }
    impl core::fmt::Debug for ELS_KS16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS16")
                .field("KS16_KSIZE", &self.KS16_KSIZE())
                .field("KS16_KACT", &self.KS16_KACT())
                .field("KS16_KBASE", &self.KS16_KBASE())
                .field("KS16_FGP", &self.KS16_FGP())
                .field("KS16_FRTN", &self.KS16_FRTN())
                .field("KS16_FHWO", &self.KS16_FHWO())
                .field("KS16_UKPUK", &self.KS16_UKPUK())
                .field("KS16_UTECDH", &self.KS16_UTECDH())
                .field("KS16_UCMAC", &self.KS16_UCMAC())
                .field("KS16_UKSK", &self.KS16_UKSK())
                .field("KS16_URTF", &self.KS16_URTF())
                .field("KS16_UCKDF", &self.KS16_UCKDF())
                .field("KS16_UHKDF", &self.KS16_UHKDF())
                .field("KS16_UECSG", &self.KS16_UECSG())
                .field("KS16_UECDH", &self.KS16_UECDH())
                .field("KS16_UAES", &self.KS16_UAES())
                .field("KS16_UHMAC", &self.KS16_UHMAC())
                .field("KS16_UKWK", &self.KS16_UKWK())
                .field("KS16_UKUOK", &self.KS16_UKUOK())
                .field("KS16_UTLSPMS", &self.KS16_UTLSPMS())
                .field("KS16_UTLSMS", &self.KS16_UTLSMS())
                .field("KS16_UKGSRC", &self.KS16_UKGSRC())
                .field("KS16_UHWO", &self.KS16_UHWO())
                .field("KS16_UWRPOK", &self.KS16_UWRPOK())
                .field("KS16_UDUK", &self.KS16_UDUK())
                .field("KS16_UPPROT", &self.KS16_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS16 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS16 {
                KS16_KSIZE: u8,
                KS16_KACT: bool,
                KS16_KBASE: bool,
                KS16_FGP: bool,
                KS16_FRTN: bool,
                KS16_FHWO: bool,
                KS16_UKPUK: bool,
                KS16_UTECDH: bool,
                KS16_UCMAC: bool,
                KS16_UKSK: bool,
                KS16_URTF: bool,
                KS16_UCKDF: bool,
                KS16_UHKDF: bool,
                KS16_UECSG: bool,
                KS16_UECDH: bool,
                KS16_UAES: bool,
                KS16_UHMAC: bool,
                KS16_UKWK: bool,
                KS16_UKUOK: bool,
                KS16_UTLSPMS: bool,
                KS16_UTLSMS: bool,
                KS16_UKGSRC: bool,
                KS16_UHWO: bool,
                KS16_UWRPOK: bool,
                KS16_UDUK: bool,
                KS16_UPPROT: u8,
            }
            let proxy = ELS_KS16 {
                KS16_KSIZE: self.KS16_KSIZE(),
                KS16_KACT: self.KS16_KACT(),
                KS16_KBASE: self.KS16_KBASE(),
                KS16_FGP: self.KS16_FGP(),
                KS16_FRTN: self.KS16_FRTN(),
                KS16_FHWO: self.KS16_FHWO(),
                KS16_UKPUK: self.KS16_UKPUK(),
                KS16_UTECDH: self.KS16_UTECDH(),
                KS16_UCMAC: self.KS16_UCMAC(),
                KS16_UKSK: self.KS16_UKSK(),
                KS16_URTF: self.KS16_URTF(),
                KS16_UCKDF: self.KS16_UCKDF(),
                KS16_UHKDF: self.KS16_UHKDF(),
                KS16_UECSG: self.KS16_UECSG(),
                KS16_UECDH: self.KS16_UECDH(),
                KS16_UAES: self.KS16_UAES(),
                KS16_UHMAC: self.KS16_UHMAC(),
                KS16_UKWK: self.KS16_UKWK(),
                KS16_UKUOK: self.KS16_UKUOK(),
                KS16_UTLSPMS: self.KS16_UTLSPMS(),
                KS16_UTLSMS: self.KS16_UTLSMS(),
                KS16_UKGSRC: self.KS16_UKGSRC(),
                KS16_UHWO: self.KS16_UHWO(),
                KS16_UWRPOK: self.KS16_UWRPOK(),
                KS16_UDUK: self.KS16_UDUK(),
                KS16_UPPROT: self.KS16_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS17(pub u32);
    impl ELS_KS17 {
        #[inline(always)]
        pub const fn KS17_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS17_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS17_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS17_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS17_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS17_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS17_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS17_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS17_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS17_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS17_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS17_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS17_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS17_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS17_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS17_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS17_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS17_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS17_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS17_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS17_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS17_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS17_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS17_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS17_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS17_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS17_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS17_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS17_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS17 {
        #[inline(always)]
        fn default() -> ELS_KS17 {
            ELS_KS17(0)
        }
    }
    impl core::fmt::Debug for ELS_KS17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS17")
                .field("KS17_KSIZE", &self.KS17_KSIZE())
                .field("KS17_KACT", &self.KS17_KACT())
                .field("KS17_KBASE", &self.KS17_KBASE())
                .field("KS17_FGP", &self.KS17_FGP())
                .field("KS17_FRTN", &self.KS17_FRTN())
                .field("KS17_FHWO", &self.KS17_FHWO())
                .field("KS17_UKPUK", &self.KS17_UKPUK())
                .field("KS17_UTECDH", &self.KS17_UTECDH())
                .field("KS17_UCMAC", &self.KS17_UCMAC())
                .field("KS17_UKSK", &self.KS17_UKSK())
                .field("KS17_URTF", &self.KS17_URTF())
                .field("KS17_UCKDF", &self.KS17_UCKDF())
                .field("KS17_UHKDF", &self.KS17_UHKDF())
                .field("KS17_UECSG", &self.KS17_UECSG())
                .field("KS17_UECDH", &self.KS17_UECDH())
                .field("KS17_UAES", &self.KS17_UAES())
                .field("KS17_UHMAC", &self.KS17_UHMAC())
                .field("KS17_UKWK", &self.KS17_UKWK())
                .field("KS17_UKUOK", &self.KS17_UKUOK())
                .field("KS17_UTLSPMS", &self.KS17_UTLSPMS())
                .field("KS17_UTLSMS", &self.KS17_UTLSMS())
                .field("KS17_UKGSRC", &self.KS17_UKGSRC())
                .field("KS17_UHWO", &self.KS17_UHWO())
                .field("KS17_UWRPOK", &self.KS17_UWRPOK())
                .field("KS17_UDUK", &self.KS17_UDUK())
                .field("KS17_UPPROT", &self.KS17_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS17 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS17 {
                KS17_KSIZE: u8,
                KS17_KACT: bool,
                KS17_KBASE: bool,
                KS17_FGP: bool,
                KS17_FRTN: bool,
                KS17_FHWO: bool,
                KS17_UKPUK: bool,
                KS17_UTECDH: bool,
                KS17_UCMAC: bool,
                KS17_UKSK: bool,
                KS17_URTF: bool,
                KS17_UCKDF: bool,
                KS17_UHKDF: bool,
                KS17_UECSG: bool,
                KS17_UECDH: bool,
                KS17_UAES: bool,
                KS17_UHMAC: bool,
                KS17_UKWK: bool,
                KS17_UKUOK: bool,
                KS17_UTLSPMS: bool,
                KS17_UTLSMS: bool,
                KS17_UKGSRC: bool,
                KS17_UHWO: bool,
                KS17_UWRPOK: bool,
                KS17_UDUK: bool,
                KS17_UPPROT: u8,
            }
            let proxy = ELS_KS17 {
                KS17_KSIZE: self.KS17_KSIZE(),
                KS17_KACT: self.KS17_KACT(),
                KS17_KBASE: self.KS17_KBASE(),
                KS17_FGP: self.KS17_FGP(),
                KS17_FRTN: self.KS17_FRTN(),
                KS17_FHWO: self.KS17_FHWO(),
                KS17_UKPUK: self.KS17_UKPUK(),
                KS17_UTECDH: self.KS17_UTECDH(),
                KS17_UCMAC: self.KS17_UCMAC(),
                KS17_UKSK: self.KS17_UKSK(),
                KS17_URTF: self.KS17_URTF(),
                KS17_UCKDF: self.KS17_UCKDF(),
                KS17_UHKDF: self.KS17_UHKDF(),
                KS17_UECSG: self.KS17_UECSG(),
                KS17_UECDH: self.KS17_UECDH(),
                KS17_UAES: self.KS17_UAES(),
                KS17_UHMAC: self.KS17_UHMAC(),
                KS17_UKWK: self.KS17_UKWK(),
                KS17_UKUOK: self.KS17_UKUOK(),
                KS17_UTLSPMS: self.KS17_UTLSPMS(),
                KS17_UTLSMS: self.KS17_UTLSMS(),
                KS17_UKGSRC: self.KS17_UKGSRC(),
                KS17_UHWO: self.KS17_UHWO(),
                KS17_UWRPOK: self.KS17_UWRPOK(),
                KS17_UDUK: self.KS17_UDUK(),
                KS17_UPPROT: self.KS17_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS18(pub u32);
    impl ELS_KS18 {
        #[inline(always)]
        pub const fn KS18_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS18_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS18_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS18_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS18_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS18_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS18_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS18_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS18_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS18_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS18_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS18_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS18_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS18_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS18_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS18_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS18_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS18_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS18_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS18_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS18_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS18_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS18_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS18_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS18_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS18_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS18_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS18_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS18_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS18 {
        #[inline(always)]
        fn default() -> ELS_KS18 {
            ELS_KS18(0)
        }
    }
    impl core::fmt::Debug for ELS_KS18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS18")
                .field("KS18_KSIZE", &self.KS18_KSIZE())
                .field("KS18_KACT", &self.KS18_KACT())
                .field("KS18_KBASE", &self.KS18_KBASE())
                .field("KS18_FGP", &self.KS18_FGP())
                .field("KS18_FRTN", &self.KS18_FRTN())
                .field("KS18_FHWO", &self.KS18_FHWO())
                .field("KS18_UKPUK", &self.KS18_UKPUK())
                .field("KS18_UTECDH", &self.KS18_UTECDH())
                .field("KS18_UCMAC", &self.KS18_UCMAC())
                .field("KS18_UKSK", &self.KS18_UKSK())
                .field("KS18_URTF", &self.KS18_URTF())
                .field("KS18_UCKDF", &self.KS18_UCKDF())
                .field("KS18_UHKDF", &self.KS18_UHKDF())
                .field("KS18_UECSG", &self.KS18_UECSG())
                .field("KS18_UECDH", &self.KS18_UECDH())
                .field("KS18_UAES", &self.KS18_UAES())
                .field("KS18_UHMAC", &self.KS18_UHMAC())
                .field("KS18_UKWK", &self.KS18_UKWK())
                .field("KS18_UKUOK", &self.KS18_UKUOK())
                .field("KS18_UTLSPMS", &self.KS18_UTLSPMS())
                .field("KS18_UTLSMS", &self.KS18_UTLSMS())
                .field("KS18_UKGSRC", &self.KS18_UKGSRC())
                .field("KS18_UHWO", &self.KS18_UHWO())
                .field("KS18_UWRPOK", &self.KS18_UWRPOK())
                .field("KS18_UDUK", &self.KS18_UDUK())
                .field("KS18_UPPROT", &self.KS18_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS18 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS18 {
                KS18_KSIZE: u8,
                KS18_KACT: bool,
                KS18_KBASE: bool,
                KS18_FGP: bool,
                KS18_FRTN: bool,
                KS18_FHWO: bool,
                KS18_UKPUK: bool,
                KS18_UTECDH: bool,
                KS18_UCMAC: bool,
                KS18_UKSK: bool,
                KS18_URTF: bool,
                KS18_UCKDF: bool,
                KS18_UHKDF: bool,
                KS18_UECSG: bool,
                KS18_UECDH: bool,
                KS18_UAES: bool,
                KS18_UHMAC: bool,
                KS18_UKWK: bool,
                KS18_UKUOK: bool,
                KS18_UTLSPMS: bool,
                KS18_UTLSMS: bool,
                KS18_UKGSRC: bool,
                KS18_UHWO: bool,
                KS18_UWRPOK: bool,
                KS18_UDUK: bool,
                KS18_UPPROT: u8,
            }
            let proxy = ELS_KS18 {
                KS18_KSIZE: self.KS18_KSIZE(),
                KS18_KACT: self.KS18_KACT(),
                KS18_KBASE: self.KS18_KBASE(),
                KS18_FGP: self.KS18_FGP(),
                KS18_FRTN: self.KS18_FRTN(),
                KS18_FHWO: self.KS18_FHWO(),
                KS18_UKPUK: self.KS18_UKPUK(),
                KS18_UTECDH: self.KS18_UTECDH(),
                KS18_UCMAC: self.KS18_UCMAC(),
                KS18_UKSK: self.KS18_UKSK(),
                KS18_URTF: self.KS18_URTF(),
                KS18_UCKDF: self.KS18_UCKDF(),
                KS18_UHKDF: self.KS18_UHKDF(),
                KS18_UECSG: self.KS18_UECSG(),
                KS18_UECDH: self.KS18_UECDH(),
                KS18_UAES: self.KS18_UAES(),
                KS18_UHMAC: self.KS18_UHMAC(),
                KS18_UKWK: self.KS18_UKWK(),
                KS18_UKUOK: self.KS18_UKUOK(),
                KS18_UTLSPMS: self.KS18_UTLSPMS(),
                KS18_UTLSMS: self.KS18_UTLSMS(),
                KS18_UKGSRC: self.KS18_UKGSRC(),
                KS18_UHWO: self.KS18_UHWO(),
                KS18_UWRPOK: self.KS18_UWRPOK(),
                KS18_UDUK: self.KS18_UDUK(),
                KS18_UPPROT: self.KS18_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS19(pub u32);
    impl ELS_KS19 {
        #[inline(always)]
        pub const fn KS19_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS19_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS19_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS19_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS19_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS19_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS19_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS19_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS19_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS19_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS19_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS19_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS19_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS19_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS19_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS19_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS19_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS19_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS19_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS19_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS19_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS19_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS19_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS19_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS19_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS19_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS19_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS19_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS19_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS19 {
        #[inline(always)]
        fn default() -> ELS_KS19 {
            ELS_KS19(0)
        }
    }
    impl core::fmt::Debug for ELS_KS19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS19")
                .field("KS19_KSIZE", &self.KS19_KSIZE())
                .field("KS19_KACT", &self.KS19_KACT())
                .field("KS19_KBASE", &self.KS19_KBASE())
                .field("KS19_FGP", &self.KS19_FGP())
                .field("KS19_FRTN", &self.KS19_FRTN())
                .field("KS19_FHWO", &self.KS19_FHWO())
                .field("KS19_UKPUK", &self.KS19_UKPUK())
                .field("KS19_UTECDH", &self.KS19_UTECDH())
                .field("KS19_UCMAC", &self.KS19_UCMAC())
                .field("KS19_UKSK", &self.KS19_UKSK())
                .field("KS19_URTF", &self.KS19_URTF())
                .field("KS19_UCKDF", &self.KS19_UCKDF())
                .field("KS19_UHKDF", &self.KS19_UHKDF())
                .field("KS19_UECSG", &self.KS19_UECSG())
                .field("KS19_UECDH", &self.KS19_UECDH())
                .field("KS19_UAES", &self.KS19_UAES())
                .field("KS19_UHMAC", &self.KS19_UHMAC())
                .field("KS19_UKWK", &self.KS19_UKWK())
                .field("KS19_UKUOK", &self.KS19_UKUOK())
                .field("KS19_UTLSPMS", &self.KS19_UTLSPMS())
                .field("KS19_UTLSMS", &self.KS19_UTLSMS())
                .field("KS19_UKGSRC", &self.KS19_UKGSRC())
                .field("KS19_UHWO", &self.KS19_UHWO())
                .field("KS19_UWRPOK", &self.KS19_UWRPOK())
                .field("KS19_UDUK", &self.KS19_UDUK())
                .field("KS19_UPPROT", &self.KS19_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS19 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS19 {
                KS19_KSIZE: u8,
                KS19_KACT: bool,
                KS19_KBASE: bool,
                KS19_FGP: bool,
                KS19_FRTN: bool,
                KS19_FHWO: bool,
                KS19_UKPUK: bool,
                KS19_UTECDH: bool,
                KS19_UCMAC: bool,
                KS19_UKSK: bool,
                KS19_URTF: bool,
                KS19_UCKDF: bool,
                KS19_UHKDF: bool,
                KS19_UECSG: bool,
                KS19_UECDH: bool,
                KS19_UAES: bool,
                KS19_UHMAC: bool,
                KS19_UKWK: bool,
                KS19_UKUOK: bool,
                KS19_UTLSPMS: bool,
                KS19_UTLSMS: bool,
                KS19_UKGSRC: bool,
                KS19_UHWO: bool,
                KS19_UWRPOK: bool,
                KS19_UDUK: bool,
                KS19_UPPROT: u8,
            }
            let proxy = ELS_KS19 {
                KS19_KSIZE: self.KS19_KSIZE(),
                KS19_KACT: self.KS19_KACT(),
                KS19_KBASE: self.KS19_KBASE(),
                KS19_FGP: self.KS19_FGP(),
                KS19_FRTN: self.KS19_FRTN(),
                KS19_FHWO: self.KS19_FHWO(),
                KS19_UKPUK: self.KS19_UKPUK(),
                KS19_UTECDH: self.KS19_UTECDH(),
                KS19_UCMAC: self.KS19_UCMAC(),
                KS19_UKSK: self.KS19_UKSK(),
                KS19_URTF: self.KS19_URTF(),
                KS19_UCKDF: self.KS19_UCKDF(),
                KS19_UHKDF: self.KS19_UHKDF(),
                KS19_UECSG: self.KS19_UECSG(),
                KS19_UECDH: self.KS19_UECDH(),
                KS19_UAES: self.KS19_UAES(),
                KS19_UHMAC: self.KS19_UHMAC(),
                KS19_UKWK: self.KS19_UKWK(),
                KS19_UKUOK: self.KS19_UKUOK(),
                KS19_UTLSPMS: self.KS19_UTLSPMS(),
                KS19_UTLSMS: self.KS19_UTLSMS(),
                KS19_UKGSRC: self.KS19_UKGSRC(),
                KS19_UHWO: self.KS19_UHWO(),
                KS19_UWRPOK: self.KS19_UWRPOK(),
                KS19_UDUK: self.KS19_UDUK(),
                KS19_UPPROT: self.KS19_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS2(pub u32);
    impl ELS_KS2 {
        #[inline(always)]
        pub const fn KS2_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS2_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS2_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS2_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS2_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS2_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS2_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS2_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS2_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS2_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS2_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS2_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS2_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS2_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS2_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS2_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS2_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS2_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS2_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS2_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS2_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS2_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS2_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS2_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS2_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS2_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS2_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS2_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS2_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS2 {
        #[inline(always)]
        fn default() -> ELS_KS2 {
            ELS_KS2(0)
        }
    }
    impl core::fmt::Debug for ELS_KS2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS2")
                .field("KS2_KSIZE", &self.KS2_KSIZE())
                .field("KS2_KACT", &self.KS2_KACT())
                .field("KS2_KBASE", &self.KS2_KBASE())
                .field("KS2_FGP", &self.KS2_FGP())
                .field("KS2_FRTN", &self.KS2_FRTN())
                .field("KS2_FHWO", &self.KS2_FHWO())
                .field("KS2_UKPUK", &self.KS2_UKPUK())
                .field("KS2_UTECDH", &self.KS2_UTECDH())
                .field("KS2_UCMAC", &self.KS2_UCMAC())
                .field("KS2_UKSK", &self.KS2_UKSK())
                .field("KS2_URTF", &self.KS2_URTF())
                .field("KS2_UCKDF", &self.KS2_UCKDF())
                .field("KS2_UHKDF", &self.KS2_UHKDF())
                .field("KS2_UECSG", &self.KS2_UECSG())
                .field("KS2_UECDH", &self.KS2_UECDH())
                .field("KS2_UAES", &self.KS2_UAES())
                .field("KS2_UHMAC", &self.KS2_UHMAC())
                .field("KS2_UKWK", &self.KS2_UKWK())
                .field("KS2_UKUOK", &self.KS2_UKUOK())
                .field("KS2_UTLSPMS", &self.KS2_UTLSPMS())
                .field("KS2_UTLSMS", &self.KS2_UTLSMS())
                .field("KS2_UKGSRC", &self.KS2_UKGSRC())
                .field("KS2_UHWO", &self.KS2_UHWO())
                .field("KS2_UWRPOK", &self.KS2_UWRPOK())
                .field("KS2_UDUK", &self.KS2_UDUK())
                .field("KS2_UPPROT", &self.KS2_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS2 {
                KS2_KSIZE: u8,
                KS2_KACT: bool,
                KS2_KBASE: bool,
                KS2_FGP: bool,
                KS2_FRTN: bool,
                KS2_FHWO: bool,
                KS2_UKPUK: bool,
                KS2_UTECDH: bool,
                KS2_UCMAC: bool,
                KS2_UKSK: bool,
                KS2_URTF: bool,
                KS2_UCKDF: bool,
                KS2_UHKDF: bool,
                KS2_UECSG: bool,
                KS2_UECDH: bool,
                KS2_UAES: bool,
                KS2_UHMAC: bool,
                KS2_UKWK: bool,
                KS2_UKUOK: bool,
                KS2_UTLSPMS: bool,
                KS2_UTLSMS: bool,
                KS2_UKGSRC: bool,
                KS2_UHWO: bool,
                KS2_UWRPOK: bool,
                KS2_UDUK: bool,
                KS2_UPPROT: u8,
            }
            let proxy = ELS_KS2 {
                KS2_KSIZE: self.KS2_KSIZE(),
                KS2_KACT: self.KS2_KACT(),
                KS2_KBASE: self.KS2_KBASE(),
                KS2_FGP: self.KS2_FGP(),
                KS2_FRTN: self.KS2_FRTN(),
                KS2_FHWO: self.KS2_FHWO(),
                KS2_UKPUK: self.KS2_UKPUK(),
                KS2_UTECDH: self.KS2_UTECDH(),
                KS2_UCMAC: self.KS2_UCMAC(),
                KS2_UKSK: self.KS2_UKSK(),
                KS2_URTF: self.KS2_URTF(),
                KS2_UCKDF: self.KS2_UCKDF(),
                KS2_UHKDF: self.KS2_UHKDF(),
                KS2_UECSG: self.KS2_UECSG(),
                KS2_UECDH: self.KS2_UECDH(),
                KS2_UAES: self.KS2_UAES(),
                KS2_UHMAC: self.KS2_UHMAC(),
                KS2_UKWK: self.KS2_UKWK(),
                KS2_UKUOK: self.KS2_UKUOK(),
                KS2_UTLSPMS: self.KS2_UTLSPMS(),
                KS2_UTLSMS: self.KS2_UTLSMS(),
                KS2_UKGSRC: self.KS2_UKGSRC(),
                KS2_UHWO: self.KS2_UHWO(),
                KS2_UWRPOK: self.KS2_UWRPOK(),
                KS2_UDUK: self.KS2_UDUK(),
                KS2_UPPROT: self.KS2_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS3(pub u32);
    impl ELS_KS3 {
        #[inline(always)]
        pub const fn KS3_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS3_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS3_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS3_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS3_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS3_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS3_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS3_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS3_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS3_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS3_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS3_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS3_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS3_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS3_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS3_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS3_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS3_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS3_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS3_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS3_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS3_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS3_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS3_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS3_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS3_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS3_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS3_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS3_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS3 {
        #[inline(always)]
        fn default() -> ELS_KS3 {
            ELS_KS3(0)
        }
    }
    impl core::fmt::Debug for ELS_KS3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS3")
                .field("KS3_KSIZE", &self.KS3_KSIZE())
                .field("KS3_KACT", &self.KS3_KACT())
                .field("KS3_KBASE", &self.KS3_KBASE())
                .field("KS3_FGP", &self.KS3_FGP())
                .field("KS3_FRTN", &self.KS3_FRTN())
                .field("KS3_FHWO", &self.KS3_FHWO())
                .field("KS3_UKPUK", &self.KS3_UKPUK())
                .field("KS3_UTECDH", &self.KS3_UTECDH())
                .field("KS3_UCMAC", &self.KS3_UCMAC())
                .field("KS3_UKSK", &self.KS3_UKSK())
                .field("KS3_URTF", &self.KS3_URTF())
                .field("KS3_UCKDF", &self.KS3_UCKDF())
                .field("KS3_UHKDF", &self.KS3_UHKDF())
                .field("KS3_UECSG", &self.KS3_UECSG())
                .field("KS3_UECDH", &self.KS3_UECDH())
                .field("KS3_UAES", &self.KS3_UAES())
                .field("KS3_UHMAC", &self.KS3_UHMAC())
                .field("KS3_UKWK", &self.KS3_UKWK())
                .field("KS3_UKUOK", &self.KS3_UKUOK())
                .field("KS3_UTLSPMS", &self.KS3_UTLSPMS())
                .field("KS3_UTLSMS", &self.KS3_UTLSMS())
                .field("KS3_UKGSRC", &self.KS3_UKGSRC())
                .field("KS3_UHWO", &self.KS3_UHWO())
                .field("KS3_UWRPOK", &self.KS3_UWRPOK())
                .field("KS3_UDUK", &self.KS3_UDUK())
                .field("KS3_UPPROT", &self.KS3_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS3 {
                KS3_KSIZE: u8,
                KS3_KACT: bool,
                KS3_KBASE: bool,
                KS3_FGP: bool,
                KS3_FRTN: bool,
                KS3_FHWO: bool,
                KS3_UKPUK: bool,
                KS3_UTECDH: bool,
                KS3_UCMAC: bool,
                KS3_UKSK: bool,
                KS3_URTF: bool,
                KS3_UCKDF: bool,
                KS3_UHKDF: bool,
                KS3_UECSG: bool,
                KS3_UECDH: bool,
                KS3_UAES: bool,
                KS3_UHMAC: bool,
                KS3_UKWK: bool,
                KS3_UKUOK: bool,
                KS3_UTLSPMS: bool,
                KS3_UTLSMS: bool,
                KS3_UKGSRC: bool,
                KS3_UHWO: bool,
                KS3_UWRPOK: bool,
                KS3_UDUK: bool,
                KS3_UPPROT: u8,
            }
            let proxy = ELS_KS3 {
                KS3_KSIZE: self.KS3_KSIZE(),
                KS3_KACT: self.KS3_KACT(),
                KS3_KBASE: self.KS3_KBASE(),
                KS3_FGP: self.KS3_FGP(),
                KS3_FRTN: self.KS3_FRTN(),
                KS3_FHWO: self.KS3_FHWO(),
                KS3_UKPUK: self.KS3_UKPUK(),
                KS3_UTECDH: self.KS3_UTECDH(),
                KS3_UCMAC: self.KS3_UCMAC(),
                KS3_UKSK: self.KS3_UKSK(),
                KS3_URTF: self.KS3_URTF(),
                KS3_UCKDF: self.KS3_UCKDF(),
                KS3_UHKDF: self.KS3_UHKDF(),
                KS3_UECSG: self.KS3_UECSG(),
                KS3_UECDH: self.KS3_UECDH(),
                KS3_UAES: self.KS3_UAES(),
                KS3_UHMAC: self.KS3_UHMAC(),
                KS3_UKWK: self.KS3_UKWK(),
                KS3_UKUOK: self.KS3_UKUOK(),
                KS3_UTLSPMS: self.KS3_UTLSPMS(),
                KS3_UTLSMS: self.KS3_UTLSMS(),
                KS3_UKGSRC: self.KS3_UKGSRC(),
                KS3_UHWO: self.KS3_UHWO(),
                KS3_UWRPOK: self.KS3_UWRPOK(),
                KS3_UDUK: self.KS3_UDUK(),
                KS3_UPPROT: self.KS3_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS4(pub u32);
    impl ELS_KS4 {
        #[inline(always)]
        pub const fn KS4_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS4_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS4_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS4_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS4_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS4_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS4_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS4_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS4_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS4_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS4_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS4_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS4_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS4_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS4_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS4_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS4_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS4_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS4_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS4_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS4_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS4_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS4_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS4_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS4_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS4_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS4_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS4_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS4_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS4 {
        #[inline(always)]
        fn default() -> ELS_KS4 {
            ELS_KS4(0)
        }
    }
    impl core::fmt::Debug for ELS_KS4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS4")
                .field("KS4_KSIZE", &self.KS4_KSIZE())
                .field("KS4_KACT", &self.KS4_KACT())
                .field("KS4_KBASE", &self.KS4_KBASE())
                .field("KS4_FGP", &self.KS4_FGP())
                .field("KS4_FRTN", &self.KS4_FRTN())
                .field("KS4_FHWO", &self.KS4_FHWO())
                .field("KS4_UKPUK", &self.KS4_UKPUK())
                .field("KS4_UTECDH", &self.KS4_UTECDH())
                .field("KS4_UCMAC", &self.KS4_UCMAC())
                .field("KS4_UKSK", &self.KS4_UKSK())
                .field("KS4_URTF", &self.KS4_URTF())
                .field("KS4_UCKDF", &self.KS4_UCKDF())
                .field("KS4_UHKDF", &self.KS4_UHKDF())
                .field("KS4_UECSG", &self.KS4_UECSG())
                .field("KS4_UECDH", &self.KS4_UECDH())
                .field("KS4_UAES", &self.KS4_UAES())
                .field("KS4_UHMAC", &self.KS4_UHMAC())
                .field("KS4_UKWK", &self.KS4_UKWK())
                .field("KS4_UKUOK", &self.KS4_UKUOK())
                .field("KS4_UTLSPMS", &self.KS4_UTLSPMS())
                .field("KS4_UTLSMS", &self.KS4_UTLSMS())
                .field("KS4_UKGSRC", &self.KS4_UKGSRC())
                .field("KS4_UHWO", &self.KS4_UHWO())
                .field("KS4_UWRPOK", &self.KS4_UWRPOK())
                .field("KS4_UDUK", &self.KS4_UDUK())
                .field("KS4_UPPROT", &self.KS4_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS4 {
                KS4_KSIZE: u8,
                KS4_KACT: bool,
                KS4_KBASE: bool,
                KS4_FGP: bool,
                KS4_FRTN: bool,
                KS4_FHWO: bool,
                KS4_UKPUK: bool,
                KS4_UTECDH: bool,
                KS4_UCMAC: bool,
                KS4_UKSK: bool,
                KS4_URTF: bool,
                KS4_UCKDF: bool,
                KS4_UHKDF: bool,
                KS4_UECSG: bool,
                KS4_UECDH: bool,
                KS4_UAES: bool,
                KS4_UHMAC: bool,
                KS4_UKWK: bool,
                KS4_UKUOK: bool,
                KS4_UTLSPMS: bool,
                KS4_UTLSMS: bool,
                KS4_UKGSRC: bool,
                KS4_UHWO: bool,
                KS4_UWRPOK: bool,
                KS4_UDUK: bool,
                KS4_UPPROT: u8,
            }
            let proxy = ELS_KS4 {
                KS4_KSIZE: self.KS4_KSIZE(),
                KS4_KACT: self.KS4_KACT(),
                KS4_KBASE: self.KS4_KBASE(),
                KS4_FGP: self.KS4_FGP(),
                KS4_FRTN: self.KS4_FRTN(),
                KS4_FHWO: self.KS4_FHWO(),
                KS4_UKPUK: self.KS4_UKPUK(),
                KS4_UTECDH: self.KS4_UTECDH(),
                KS4_UCMAC: self.KS4_UCMAC(),
                KS4_UKSK: self.KS4_UKSK(),
                KS4_URTF: self.KS4_URTF(),
                KS4_UCKDF: self.KS4_UCKDF(),
                KS4_UHKDF: self.KS4_UHKDF(),
                KS4_UECSG: self.KS4_UECSG(),
                KS4_UECDH: self.KS4_UECDH(),
                KS4_UAES: self.KS4_UAES(),
                KS4_UHMAC: self.KS4_UHMAC(),
                KS4_UKWK: self.KS4_UKWK(),
                KS4_UKUOK: self.KS4_UKUOK(),
                KS4_UTLSPMS: self.KS4_UTLSPMS(),
                KS4_UTLSMS: self.KS4_UTLSMS(),
                KS4_UKGSRC: self.KS4_UKGSRC(),
                KS4_UHWO: self.KS4_UHWO(),
                KS4_UWRPOK: self.KS4_UWRPOK(),
                KS4_UDUK: self.KS4_UDUK(),
                KS4_UPPROT: self.KS4_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS5(pub u32);
    impl ELS_KS5 {
        #[inline(always)]
        pub const fn KS5_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS5_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS5_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS5_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS5_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS5_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS5_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS5_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS5_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS5_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS5_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS5_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS5_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS5_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS5_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS5_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS5_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS5_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS5_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS5_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS5_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS5_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS5_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS5_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS5_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS5_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS5_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS5_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS5_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS5 {
        #[inline(always)]
        fn default() -> ELS_KS5 {
            ELS_KS5(0)
        }
    }
    impl core::fmt::Debug for ELS_KS5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS5")
                .field("KS5_KSIZE", &self.KS5_KSIZE())
                .field("KS5_KACT", &self.KS5_KACT())
                .field("KS5_KBASE", &self.KS5_KBASE())
                .field("KS5_FGP", &self.KS5_FGP())
                .field("KS5_FRTN", &self.KS5_FRTN())
                .field("KS5_FHWO", &self.KS5_FHWO())
                .field("KS5_UKPUK", &self.KS5_UKPUK())
                .field("KS5_UTECDH", &self.KS5_UTECDH())
                .field("KS5_UCMAC", &self.KS5_UCMAC())
                .field("KS5_UKSK", &self.KS5_UKSK())
                .field("KS5_URTF", &self.KS5_URTF())
                .field("KS5_UCKDF", &self.KS5_UCKDF())
                .field("KS5_UHKDF", &self.KS5_UHKDF())
                .field("KS5_UECSG", &self.KS5_UECSG())
                .field("KS5_UECDH", &self.KS5_UECDH())
                .field("KS5_UAES", &self.KS5_UAES())
                .field("KS5_UHMAC", &self.KS5_UHMAC())
                .field("KS5_UKWK", &self.KS5_UKWK())
                .field("KS5_UKUOK", &self.KS5_UKUOK())
                .field("KS5_UTLSPMS", &self.KS5_UTLSPMS())
                .field("KS5_UTLSMS", &self.KS5_UTLSMS())
                .field("KS5_UKGSRC", &self.KS5_UKGSRC())
                .field("KS5_UHWO", &self.KS5_UHWO())
                .field("KS5_UWRPOK", &self.KS5_UWRPOK())
                .field("KS5_UDUK", &self.KS5_UDUK())
                .field("KS5_UPPROT", &self.KS5_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS5 {
                KS5_KSIZE: u8,
                KS5_KACT: bool,
                KS5_KBASE: bool,
                KS5_FGP: bool,
                KS5_FRTN: bool,
                KS5_FHWO: bool,
                KS5_UKPUK: bool,
                KS5_UTECDH: bool,
                KS5_UCMAC: bool,
                KS5_UKSK: bool,
                KS5_URTF: bool,
                KS5_UCKDF: bool,
                KS5_UHKDF: bool,
                KS5_UECSG: bool,
                KS5_UECDH: bool,
                KS5_UAES: bool,
                KS5_UHMAC: bool,
                KS5_UKWK: bool,
                KS5_UKUOK: bool,
                KS5_UTLSPMS: bool,
                KS5_UTLSMS: bool,
                KS5_UKGSRC: bool,
                KS5_UHWO: bool,
                KS5_UWRPOK: bool,
                KS5_UDUK: bool,
                KS5_UPPROT: u8,
            }
            let proxy = ELS_KS5 {
                KS5_KSIZE: self.KS5_KSIZE(),
                KS5_KACT: self.KS5_KACT(),
                KS5_KBASE: self.KS5_KBASE(),
                KS5_FGP: self.KS5_FGP(),
                KS5_FRTN: self.KS5_FRTN(),
                KS5_FHWO: self.KS5_FHWO(),
                KS5_UKPUK: self.KS5_UKPUK(),
                KS5_UTECDH: self.KS5_UTECDH(),
                KS5_UCMAC: self.KS5_UCMAC(),
                KS5_UKSK: self.KS5_UKSK(),
                KS5_URTF: self.KS5_URTF(),
                KS5_UCKDF: self.KS5_UCKDF(),
                KS5_UHKDF: self.KS5_UHKDF(),
                KS5_UECSG: self.KS5_UECSG(),
                KS5_UECDH: self.KS5_UECDH(),
                KS5_UAES: self.KS5_UAES(),
                KS5_UHMAC: self.KS5_UHMAC(),
                KS5_UKWK: self.KS5_UKWK(),
                KS5_UKUOK: self.KS5_UKUOK(),
                KS5_UTLSPMS: self.KS5_UTLSPMS(),
                KS5_UTLSMS: self.KS5_UTLSMS(),
                KS5_UKGSRC: self.KS5_UKGSRC(),
                KS5_UHWO: self.KS5_UHWO(),
                KS5_UWRPOK: self.KS5_UWRPOK(),
                KS5_UDUK: self.KS5_UDUK(),
                KS5_UPPROT: self.KS5_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS6(pub u32);
    impl ELS_KS6 {
        #[inline(always)]
        pub const fn KS6_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS6_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS6_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS6_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS6_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS6_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS6_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS6_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS6_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS6_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS6_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS6_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS6_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS6_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS6_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS6_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS6_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS6_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS6_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS6_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS6_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS6_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS6_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS6_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS6_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS6_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS6_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS6_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS6_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS6 {
        #[inline(always)]
        fn default() -> ELS_KS6 {
            ELS_KS6(0)
        }
    }
    impl core::fmt::Debug for ELS_KS6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS6")
                .field("KS6_KSIZE", &self.KS6_KSIZE())
                .field("KS6_KACT", &self.KS6_KACT())
                .field("KS6_KBASE", &self.KS6_KBASE())
                .field("KS6_FGP", &self.KS6_FGP())
                .field("KS6_FRTN", &self.KS6_FRTN())
                .field("KS6_FHWO", &self.KS6_FHWO())
                .field("KS6_UKPUK", &self.KS6_UKPUK())
                .field("KS6_UTECDH", &self.KS6_UTECDH())
                .field("KS6_UCMAC", &self.KS6_UCMAC())
                .field("KS6_UKSK", &self.KS6_UKSK())
                .field("KS6_URTF", &self.KS6_URTF())
                .field("KS6_UCKDF", &self.KS6_UCKDF())
                .field("KS6_UHKDF", &self.KS6_UHKDF())
                .field("KS6_UECSG", &self.KS6_UECSG())
                .field("KS6_UECDH", &self.KS6_UECDH())
                .field("KS6_UAES", &self.KS6_UAES())
                .field("KS6_UHMAC", &self.KS6_UHMAC())
                .field("KS6_UKWK", &self.KS6_UKWK())
                .field("KS6_UKUOK", &self.KS6_UKUOK())
                .field("KS6_UTLSPMS", &self.KS6_UTLSPMS())
                .field("KS6_UTLSMS", &self.KS6_UTLSMS())
                .field("KS6_UKGSRC", &self.KS6_UKGSRC())
                .field("KS6_UHWO", &self.KS6_UHWO())
                .field("KS6_UWRPOK", &self.KS6_UWRPOK())
                .field("KS6_UDUK", &self.KS6_UDUK())
                .field("KS6_UPPROT", &self.KS6_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS6 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS6 {
                KS6_KSIZE: u8,
                KS6_KACT: bool,
                KS6_KBASE: bool,
                KS6_FGP: bool,
                KS6_FRTN: bool,
                KS6_FHWO: bool,
                KS6_UKPUK: bool,
                KS6_UTECDH: bool,
                KS6_UCMAC: bool,
                KS6_UKSK: bool,
                KS6_URTF: bool,
                KS6_UCKDF: bool,
                KS6_UHKDF: bool,
                KS6_UECSG: bool,
                KS6_UECDH: bool,
                KS6_UAES: bool,
                KS6_UHMAC: bool,
                KS6_UKWK: bool,
                KS6_UKUOK: bool,
                KS6_UTLSPMS: bool,
                KS6_UTLSMS: bool,
                KS6_UKGSRC: bool,
                KS6_UHWO: bool,
                KS6_UWRPOK: bool,
                KS6_UDUK: bool,
                KS6_UPPROT: u8,
            }
            let proxy = ELS_KS6 {
                KS6_KSIZE: self.KS6_KSIZE(),
                KS6_KACT: self.KS6_KACT(),
                KS6_KBASE: self.KS6_KBASE(),
                KS6_FGP: self.KS6_FGP(),
                KS6_FRTN: self.KS6_FRTN(),
                KS6_FHWO: self.KS6_FHWO(),
                KS6_UKPUK: self.KS6_UKPUK(),
                KS6_UTECDH: self.KS6_UTECDH(),
                KS6_UCMAC: self.KS6_UCMAC(),
                KS6_UKSK: self.KS6_UKSK(),
                KS6_URTF: self.KS6_URTF(),
                KS6_UCKDF: self.KS6_UCKDF(),
                KS6_UHKDF: self.KS6_UHKDF(),
                KS6_UECSG: self.KS6_UECSG(),
                KS6_UECDH: self.KS6_UECDH(),
                KS6_UAES: self.KS6_UAES(),
                KS6_UHMAC: self.KS6_UHMAC(),
                KS6_UKWK: self.KS6_UKWK(),
                KS6_UKUOK: self.KS6_UKUOK(),
                KS6_UTLSPMS: self.KS6_UTLSPMS(),
                KS6_UTLSMS: self.KS6_UTLSMS(),
                KS6_UKGSRC: self.KS6_UKGSRC(),
                KS6_UHWO: self.KS6_UHWO(),
                KS6_UWRPOK: self.KS6_UWRPOK(),
                KS6_UDUK: self.KS6_UDUK(),
                KS6_UPPROT: self.KS6_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS7(pub u32);
    impl ELS_KS7 {
        #[inline(always)]
        pub const fn KS7_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS7_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS7_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS7_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS7_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS7_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS7_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS7_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS7_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS7_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS7_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS7_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS7_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS7_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS7_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS7_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS7_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS7_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS7_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS7_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS7_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS7_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS7_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS7_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS7_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS7_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS7_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS7_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS7_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS7 {
        #[inline(always)]
        fn default() -> ELS_KS7 {
            ELS_KS7(0)
        }
    }
    impl core::fmt::Debug for ELS_KS7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS7")
                .field("KS7_KSIZE", &self.KS7_KSIZE())
                .field("KS7_KACT", &self.KS7_KACT())
                .field("KS7_KBASE", &self.KS7_KBASE())
                .field("KS7_FGP", &self.KS7_FGP())
                .field("KS7_FRTN", &self.KS7_FRTN())
                .field("KS7_FHWO", &self.KS7_FHWO())
                .field("KS7_UKPUK", &self.KS7_UKPUK())
                .field("KS7_UTECDH", &self.KS7_UTECDH())
                .field("KS7_UCMAC", &self.KS7_UCMAC())
                .field("KS7_UKSK", &self.KS7_UKSK())
                .field("KS7_URTF", &self.KS7_URTF())
                .field("KS7_UCKDF", &self.KS7_UCKDF())
                .field("KS7_UHKDF", &self.KS7_UHKDF())
                .field("KS7_UECSG", &self.KS7_UECSG())
                .field("KS7_UECDH", &self.KS7_UECDH())
                .field("KS7_UAES", &self.KS7_UAES())
                .field("KS7_UHMAC", &self.KS7_UHMAC())
                .field("KS7_UKWK", &self.KS7_UKWK())
                .field("KS7_UKUOK", &self.KS7_UKUOK())
                .field("KS7_UTLSPMS", &self.KS7_UTLSPMS())
                .field("KS7_UTLSMS", &self.KS7_UTLSMS())
                .field("KS7_UKGSRC", &self.KS7_UKGSRC())
                .field("KS7_UHWO", &self.KS7_UHWO())
                .field("KS7_UWRPOK", &self.KS7_UWRPOK())
                .field("KS7_UDUK", &self.KS7_UDUK())
                .field("KS7_UPPROT", &self.KS7_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS7 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS7 {
                KS7_KSIZE: u8,
                KS7_KACT: bool,
                KS7_KBASE: bool,
                KS7_FGP: bool,
                KS7_FRTN: bool,
                KS7_FHWO: bool,
                KS7_UKPUK: bool,
                KS7_UTECDH: bool,
                KS7_UCMAC: bool,
                KS7_UKSK: bool,
                KS7_URTF: bool,
                KS7_UCKDF: bool,
                KS7_UHKDF: bool,
                KS7_UECSG: bool,
                KS7_UECDH: bool,
                KS7_UAES: bool,
                KS7_UHMAC: bool,
                KS7_UKWK: bool,
                KS7_UKUOK: bool,
                KS7_UTLSPMS: bool,
                KS7_UTLSMS: bool,
                KS7_UKGSRC: bool,
                KS7_UHWO: bool,
                KS7_UWRPOK: bool,
                KS7_UDUK: bool,
                KS7_UPPROT: u8,
            }
            let proxy = ELS_KS7 {
                KS7_KSIZE: self.KS7_KSIZE(),
                KS7_KACT: self.KS7_KACT(),
                KS7_KBASE: self.KS7_KBASE(),
                KS7_FGP: self.KS7_FGP(),
                KS7_FRTN: self.KS7_FRTN(),
                KS7_FHWO: self.KS7_FHWO(),
                KS7_UKPUK: self.KS7_UKPUK(),
                KS7_UTECDH: self.KS7_UTECDH(),
                KS7_UCMAC: self.KS7_UCMAC(),
                KS7_UKSK: self.KS7_UKSK(),
                KS7_URTF: self.KS7_URTF(),
                KS7_UCKDF: self.KS7_UCKDF(),
                KS7_UHKDF: self.KS7_UHKDF(),
                KS7_UECSG: self.KS7_UECSG(),
                KS7_UECDH: self.KS7_UECDH(),
                KS7_UAES: self.KS7_UAES(),
                KS7_UHMAC: self.KS7_UHMAC(),
                KS7_UKWK: self.KS7_UKWK(),
                KS7_UKUOK: self.KS7_UKUOK(),
                KS7_UTLSPMS: self.KS7_UTLSPMS(),
                KS7_UTLSMS: self.KS7_UTLSMS(),
                KS7_UKGSRC: self.KS7_UKGSRC(),
                KS7_UHWO: self.KS7_UHWO(),
                KS7_UWRPOK: self.KS7_UWRPOK(),
                KS7_UDUK: self.KS7_UDUK(),
                KS7_UPPROT: self.KS7_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS8(pub u32);
    impl ELS_KS8 {
        #[inline(always)]
        pub const fn KS8_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS8_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS8_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS8_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS8_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS8_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS8_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS8_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS8_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS8_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS8_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS8_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS8_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS8_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS8_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS8_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS8_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS8_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS8_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS8_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS8_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS8_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS8_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS8_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS8_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS8_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS8_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS8_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS8_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS8 {
        #[inline(always)]
        fn default() -> ELS_KS8 {
            ELS_KS8(0)
        }
    }
    impl core::fmt::Debug for ELS_KS8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS8")
                .field("KS8_KSIZE", &self.KS8_KSIZE())
                .field("KS8_KACT", &self.KS8_KACT())
                .field("KS8_KBASE", &self.KS8_KBASE())
                .field("KS8_FGP", &self.KS8_FGP())
                .field("KS8_FRTN", &self.KS8_FRTN())
                .field("KS8_FHWO", &self.KS8_FHWO())
                .field("KS8_UKPUK", &self.KS8_UKPUK())
                .field("KS8_UTECDH", &self.KS8_UTECDH())
                .field("KS8_UCMAC", &self.KS8_UCMAC())
                .field("KS8_UKSK", &self.KS8_UKSK())
                .field("KS8_URTF", &self.KS8_URTF())
                .field("KS8_UCKDF", &self.KS8_UCKDF())
                .field("KS8_UHKDF", &self.KS8_UHKDF())
                .field("KS8_UECSG", &self.KS8_UECSG())
                .field("KS8_UECDH", &self.KS8_UECDH())
                .field("KS8_UAES", &self.KS8_UAES())
                .field("KS8_UHMAC", &self.KS8_UHMAC())
                .field("KS8_UKWK", &self.KS8_UKWK())
                .field("KS8_UKUOK", &self.KS8_UKUOK())
                .field("KS8_UTLSPMS", &self.KS8_UTLSPMS())
                .field("KS8_UTLSMS", &self.KS8_UTLSMS())
                .field("KS8_UKGSRC", &self.KS8_UKGSRC())
                .field("KS8_UHWO", &self.KS8_UHWO())
                .field("KS8_UWRPOK", &self.KS8_UWRPOK())
                .field("KS8_UDUK", &self.KS8_UDUK())
                .field("KS8_UPPROT", &self.KS8_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS8 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS8 {
                KS8_KSIZE: u8,
                KS8_KACT: bool,
                KS8_KBASE: bool,
                KS8_FGP: bool,
                KS8_FRTN: bool,
                KS8_FHWO: bool,
                KS8_UKPUK: bool,
                KS8_UTECDH: bool,
                KS8_UCMAC: bool,
                KS8_UKSK: bool,
                KS8_URTF: bool,
                KS8_UCKDF: bool,
                KS8_UHKDF: bool,
                KS8_UECSG: bool,
                KS8_UECDH: bool,
                KS8_UAES: bool,
                KS8_UHMAC: bool,
                KS8_UKWK: bool,
                KS8_UKUOK: bool,
                KS8_UTLSPMS: bool,
                KS8_UTLSMS: bool,
                KS8_UKGSRC: bool,
                KS8_UHWO: bool,
                KS8_UWRPOK: bool,
                KS8_UDUK: bool,
                KS8_UPPROT: u8,
            }
            let proxy = ELS_KS8 {
                KS8_KSIZE: self.KS8_KSIZE(),
                KS8_KACT: self.KS8_KACT(),
                KS8_KBASE: self.KS8_KBASE(),
                KS8_FGP: self.KS8_FGP(),
                KS8_FRTN: self.KS8_FRTN(),
                KS8_FHWO: self.KS8_FHWO(),
                KS8_UKPUK: self.KS8_UKPUK(),
                KS8_UTECDH: self.KS8_UTECDH(),
                KS8_UCMAC: self.KS8_UCMAC(),
                KS8_UKSK: self.KS8_UKSK(),
                KS8_URTF: self.KS8_URTF(),
                KS8_UCKDF: self.KS8_UCKDF(),
                KS8_UHKDF: self.KS8_UHKDF(),
                KS8_UECSG: self.KS8_UECSG(),
                KS8_UECDH: self.KS8_UECDH(),
                KS8_UAES: self.KS8_UAES(),
                KS8_UHMAC: self.KS8_UHMAC(),
                KS8_UKWK: self.KS8_UKWK(),
                KS8_UKUOK: self.KS8_UKUOK(),
                KS8_UTLSPMS: self.KS8_UTLSPMS(),
                KS8_UTLSMS: self.KS8_UTLSMS(),
                KS8_UKGSRC: self.KS8_UKGSRC(),
                KS8_UHWO: self.KS8_UHWO(),
                KS8_UWRPOK: self.KS8_UWRPOK(),
                KS8_UDUK: self.KS8_UDUK(),
                KS8_UPPROT: self.KS8_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_KS9(pub u32);
    impl ELS_KS9 {
        #[inline(always)]
        pub const fn KS9_KSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS9_KSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn KS9_KACT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_KACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn KS9_KBASE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_KBASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn KS9_FGP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_FGP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn KS9_FRTN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_FRTN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn KS9_FHWO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_FHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn KS9_UKPUK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UKPUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn KS9_UTECDH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UTECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn KS9_UCMAC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UCMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn KS9_UKSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UKSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn KS9_URTF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_URTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn KS9_UCKDF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UCKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KS9_UHKDF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UHKDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn KS9_UECSG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UECSG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn KS9_UECDH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UECDH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn KS9_UAES(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UAES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn KS9_UHMAC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UHMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn KS9_UKWK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UKWK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn KS9_UKUOK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UKUOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn KS9_UTLSPMS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UTLSPMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn KS9_UTLSMS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UTLSMS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn KS9_UKGSRC(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UKGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn KS9_UHWO(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UHWO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn KS9_UWRPOK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UWRPOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn KS9_UDUK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KS9_UDUK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn KS9_UPPROT(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KS9_UPPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ELS_KS9 {
        #[inline(always)]
        fn default() -> ELS_KS9 {
            ELS_KS9(0)
        }
    }
    impl core::fmt::Debug for ELS_KS9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_KS9")
                .field("KS9_KSIZE", &self.KS9_KSIZE())
                .field("KS9_KACT", &self.KS9_KACT())
                .field("KS9_KBASE", &self.KS9_KBASE())
                .field("KS9_FGP", &self.KS9_FGP())
                .field("KS9_FRTN", &self.KS9_FRTN())
                .field("KS9_FHWO", &self.KS9_FHWO())
                .field("KS9_UKPUK", &self.KS9_UKPUK())
                .field("KS9_UTECDH", &self.KS9_UTECDH())
                .field("KS9_UCMAC", &self.KS9_UCMAC())
                .field("KS9_UKSK", &self.KS9_UKSK())
                .field("KS9_URTF", &self.KS9_URTF())
                .field("KS9_UCKDF", &self.KS9_UCKDF())
                .field("KS9_UHKDF", &self.KS9_UHKDF())
                .field("KS9_UECSG", &self.KS9_UECSG())
                .field("KS9_UECDH", &self.KS9_UECDH())
                .field("KS9_UAES", &self.KS9_UAES())
                .field("KS9_UHMAC", &self.KS9_UHMAC())
                .field("KS9_UKWK", &self.KS9_UKWK())
                .field("KS9_UKUOK", &self.KS9_UKUOK())
                .field("KS9_UTLSPMS", &self.KS9_UTLSPMS())
                .field("KS9_UTLSMS", &self.KS9_UTLSMS())
                .field("KS9_UKGSRC", &self.KS9_UKGSRC())
                .field("KS9_UHWO", &self.KS9_UHWO())
                .field("KS9_UWRPOK", &self.KS9_UWRPOK())
                .field("KS9_UDUK", &self.KS9_UDUK())
                .field("KS9_UPPROT", &self.KS9_UPPROT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_KS9 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_KS9 {
                KS9_KSIZE: u8,
                KS9_KACT: bool,
                KS9_KBASE: bool,
                KS9_FGP: bool,
                KS9_FRTN: bool,
                KS9_FHWO: bool,
                KS9_UKPUK: bool,
                KS9_UTECDH: bool,
                KS9_UCMAC: bool,
                KS9_UKSK: bool,
                KS9_URTF: bool,
                KS9_UCKDF: bool,
                KS9_UHKDF: bool,
                KS9_UECSG: bool,
                KS9_UECDH: bool,
                KS9_UAES: bool,
                KS9_UHMAC: bool,
                KS9_UKWK: bool,
                KS9_UKUOK: bool,
                KS9_UTLSPMS: bool,
                KS9_UTLSMS: bool,
                KS9_UKGSRC: bool,
                KS9_UHWO: bool,
                KS9_UWRPOK: bool,
                KS9_UDUK: bool,
                KS9_UPPROT: u8,
            }
            let proxy = ELS_KS9 {
                KS9_KSIZE: self.KS9_KSIZE(),
                KS9_KACT: self.KS9_KACT(),
                KS9_KBASE: self.KS9_KBASE(),
                KS9_FGP: self.KS9_FGP(),
                KS9_FRTN: self.KS9_FRTN(),
                KS9_FHWO: self.KS9_FHWO(),
                KS9_UKPUK: self.KS9_UKPUK(),
                KS9_UTECDH: self.KS9_UTECDH(),
                KS9_UCMAC: self.KS9_UCMAC(),
                KS9_UKSK: self.KS9_UKSK(),
                KS9_URTF: self.KS9_URTF(),
                KS9_UCKDF: self.KS9_UCKDF(),
                KS9_UHKDF: self.KS9_UHKDF(),
                KS9_UECSG: self.KS9_UECSG(),
                KS9_UECDH: self.KS9_UECDH(),
                KS9_UAES: self.KS9_UAES(),
                KS9_UHMAC: self.KS9_UHMAC(),
                KS9_UKWK: self.KS9_UKWK(),
                KS9_UKUOK: self.KS9_UKUOK(),
                KS9_UTLSPMS: self.KS9_UTLSPMS(),
                KS9_UTLSMS: self.KS9_UTLSMS(),
                KS9_UKGSRC: self.KS9_UKGSRC(),
                KS9_UHWO: self.KS9_UHWO(),
                KS9_UWRPOK: self.KS9_UWRPOK(),
                KS9_UDUK: self.KS9_UDUK(),
                KS9_UPPROT: self.KS9_UPPROT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Master ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_MASTER_ID(pub u32);
    impl ELS_MASTER_ID {
        #[inline(always)]
        pub const fn MASTER_ID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MASTER_ID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for ELS_MASTER_ID {
        #[inline(always)]
        fn default() -> ELS_MASTER_ID {
            ELS_MASTER_ID(0)
        }
    }
    impl core::fmt::Debug for ELS_MASTER_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_MASTER_ID")
                .field("MASTER_ID", &self.MASTER_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_MASTER_ID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_MASTER_ID {
                MASTER_ID: u8,
            }
            let proxy = ELS_MASTER_ID {
                MASTER_ID: self.MASTER_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_STATUS(pub u32);
    impl ELS_STATUS {
        #[inline(always)]
        pub const fn ELS_BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ELS_IRQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ELS_ERR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PRNG_RDY(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PRNG_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ECDSA_VFY_STATUS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ECDSA_VFY_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PPROT(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PPROT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn DRBG_ENT_LVL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DRBG_ENT_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn DTRNG_BUSY(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DTRNG_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ELS_LOCKED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS_LOCKED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for ELS_STATUS {
        #[inline(always)]
        fn default() -> ELS_STATUS {
            ELS_STATUS(0)
        }
    }
    impl core::fmt::Debug for ELS_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_STATUS")
                .field("ELS_BUSY", &self.ELS_BUSY())
                .field("ELS_IRQ", &self.ELS_IRQ())
                .field("ELS_ERR", &self.ELS_ERR())
                .field("PRNG_RDY", &self.PRNG_RDY())
                .field("ECDSA_VFY_STATUS", &self.ECDSA_VFY_STATUS())
                .field("PPROT", &self.PPROT())
                .field("DRBG_ENT_LVL", &self.DRBG_ENT_LVL())
                .field("DTRNG_BUSY", &self.DTRNG_BUSY())
                .field("ELS_LOCKED", &self.ELS_LOCKED())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_STATUS {
                ELS_BUSY: bool,
                ELS_IRQ: bool,
                ELS_ERR: bool,
                PRNG_RDY: bool,
                ECDSA_VFY_STATUS: u8,
                PPROT: u8,
                DRBG_ENT_LVL: u8,
                DTRNG_BUSY: bool,
                ELS_LOCKED: bool,
            }
            let proxy = ELS_STATUS {
                ELS_BUSY: self.ELS_BUSY(),
                ELS_IRQ: self.ELS_IRQ(),
                ELS_ERR: self.ELS_ERR(),
                PRNG_RDY: self.PRNG_RDY(),
                ECDSA_VFY_STATUS: self.ECDSA_VFY_STATUS(),
                PPROT: self.PPROT(),
                DRBG_ENT_LVL: self.DRBG_ENT_LVL(),
                DTRNG_BUSY: self.DTRNG_BUSY(),
                ELS_LOCKED: self.ELS_LOCKED(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_VERSION(pub u32);
    impl ELS_VERSION {
        #[inline(always)]
        pub const fn Z(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Z(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn Y2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Y2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn Y1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Y1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn X(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn SW_Z(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SW_Z(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn SW_Y2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SW_Y2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn SW_Y1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SW_Y1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn SW_X(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SW_X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for ELS_VERSION {
        #[inline(always)]
        fn default() -> ELS_VERSION {
            ELS_VERSION(0)
        }
    }
    impl core::fmt::Debug for ELS_VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_VERSION")
                .field("Z", &self.Z())
                .field("Y2", &self.Y2())
                .field("Y1", &self.Y1())
                .field("X", &self.X())
                .field("SW_Z", &self.SW_Z())
                .field("SW_Y2", &self.SW_Y2())
                .field("SW_Y1", &self.SW_Y1())
                .field("SW_X", &self.SW_X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_VERSION {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_VERSION {
                Z: u8,
                Y2: u8,
                Y1: u8,
                X: u8,
                SW_Z: u8,
                SW_Y2: u8,
                SW_Y1: u8,
                SW_X: u8,
            }
            let proxy = ELS_VERSION {
                Z: self.Z(),
                Y2: self.Y2(),
                Y1: self.Y1(),
                X: self.X(),
                SW_Z: self.SW_Z(),
                SW_Y2: self.SW_Y2(),
                SW_Y1: self.SW_Y1(),
                SW_X: self.SW_X(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
