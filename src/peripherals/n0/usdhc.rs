#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USDHC {
    ptr: *mut u8,
}
unsafe impl Send for USDHC {}
unsafe impl Sync for USDHC {}
impl USDHC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn DS_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn BLK_ATT(self) -> crate::common::Reg<regs::BLK_ATT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_ARG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_XFR_TYP(self) -> crate::common::Reg<regs::CMD_XFR_TYP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_RSP0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_RSP1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_RSP2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_RSP3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn DATA_BUFF_ACC_PORT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn PRES_STATE(self) -> crate::common::Reg<regs::PRES_STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn PROT_CTRL(self) -> crate::common::Reg<regs::PROT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SYS_CTRL(self) -> crate::common::Reg<regs::SYS_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn INT_STATUS(self) -> crate::common::Reg<regs::INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_STATUS_EN(self) -> crate::common::Reg<regs::INT_STATUS_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_SIGNAL_EN(self) -> crate::common::Reg<regs::INT_SIGNAL_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn AUTOCMD12_ERR_STATUS(
        self,
    ) -> crate::common::Reg<regs::AUTOCMD12_ERR_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn HOST_CTRL_CAP(self) -> crate::common::Reg<regs::HOST_CTRL_CAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn WTMK_LVL(self) -> crate::common::Reg<regs::WTMK_LVL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn MIX_CTRL(self) -> crate::common::Reg<regs::MIX_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn FORCE_EVENT(self) -> crate::common::Reg<regs::FORCE_EVENT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn ADMA_ERR_STATUS(
        self,
    ) -> crate::common::Reg<regs::ADMA_ERR_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn ADMA_SYS_ADDR(self) -> crate::common::Reg<regs::ADMA_SYS_ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn DLL_CTRL(self) -> crate::common::Reg<regs::DLL_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn DLL_STATUS(self) -> crate::common::Reg<regs::DLL_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn CLK_TUNE_CTRL_STATUS(
        self,
    ) -> crate::common::Reg<regs::CLK_TUNE_CTRL_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn VEND_SPEC(self) -> crate::common::Reg<regs::VEND_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn MMC_BOOT(self) -> crate::common::Reg<regs::MMC_BOOT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn VEND_SPEC2(self) -> crate::common::Reg<regs::VEND_SPEC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn TUNING_CTRL(self) -> crate::common::Reg<regs::TUNING_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADMA Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADMA_ERR_STATUS(pub u32);
    impl ADMA_ERR_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn ADMAES(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADMAES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADMALME(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADMALME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADMADCE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADMADCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for ADMA_ERR_STATUS {
        #[inline(always)]
        fn default() -> ADMA_ERR_STATUS {
            ADMA_ERR_STATUS(0)
        }
    }
    impl core::fmt::Debug for ADMA_ERR_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADMA_ERR_STATUS")
                .field("ADMAES", &self.ADMAES())
                .field("ADMALME", &self.ADMALME())
                .field("ADMADCE", &self.ADMADCE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADMA_ERR_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ADMA_ERR_STATUS {{ ADMAES: {=u8:?}, ADMALME: {=bool:?}, ADMADCE: {=bool:?} }}",
                self.ADMAES(),
                self.ADMALME(),
                self.ADMADCE()
            )
        }
    }
    #[doc = "ADMA System Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADMA_SYS_ADDR(pub u32);
    impl ADMA_SYS_ADDR {
        #[must_use]
        #[inline(always)]
        pub const fn ADS_ADDR(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ADS_ADDR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for ADMA_SYS_ADDR {
        #[inline(always)]
        fn default() -> ADMA_SYS_ADDR {
            ADMA_SYS_ADDR(0)
        }
    }
    impl core::fmt::Debug for ADMA_SYS_ADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADMA_SYS_ADDR")
                .field("ADS_ADDR", &self.ADS_ADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADMA_SYS_ADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ADMA_SYS_ADDR {{ ADS_ADDR: {=u32:?} }}", self.ADS_ADDR())
        }
    }
    #[doc = "Auto CMD12 Error Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AUTOCMD12_ERR_STATUS(pub u32);
    impl AUTOCMD12_ERR_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn AC12NE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12NE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12TOE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12TOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12CE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12CE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12EBE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12EBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12IE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CNIBAC12E(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CNIBAC12E(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXECUTE_TUNING(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXECUTE_TUNING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMP_CLK_SEL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMP_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for AUTOCMD12_ERR_STATUS {
        #[inline(always)]
        fn default() -> AUTOCMD12_ERR_STATUS {
            AUTOCMD12_ERR_STATUS(0)
        }
    }
    impl core::fmt::Debug for AUTOCMD12_ERR_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AUTOCMD12_ERR_STATUS")
                .field("AC12NE", &self.AC12NE())
                .field("AC12TOE", &self.AC12TOE())
                .field("AC12CE", &self.AC12CE())
                .field("AC12EBE", &self.AC12EBE())
                .field("AC12IE", &self.AC12IE())
                .field("CNIBAC12E", &self.CNIBAC12E())
                .field("EXECUTE_TUNING", &self.EXECUTE_TUNING())
                .field("SMP_CLK_SEL", &self.SMP_CLK_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AUTOCMD12_ERR_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AUTOCMD12_ERR_STATUS {{ AC12NE: {=bool:?}, AC12TOE: {=bool:?}, AC12CE: {=bool:?}, AC12EBE: {=bool:?}, AC12IE: {=bool:?}, CNIBAC12E: {=bool:?}, EXECUTE_TUNING: {=bool:?}, SMP_CLK_SEL: {=bool:?} }}" , self . AC12NE () , self . AC12TOE () , self . AC12CE () , self . AC12EBE () , self . AC12IE () , self . CNIBAC12E () , self . EXECUTE_TUNING () , self . SMP_CLK_SEL ())
        }
    }
    #[doc = "Block Attributes"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BLK_ATT(pub u32);
    impl BLK_ATT {
        #[must_use]
        #[inline(always)]
        pub const fn BLKSIZE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_BLKSIZE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BLKCNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_BLKCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BLK_ATT {
        #[inline(always)]
        fn default() -> BLK_ATT {
            BLK_ATT(0)
        }
    }
    impl core::fmt::Debug for BLK_ATT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BLK_ATT")
                .field("BLKSIZE", &self.BLKSIZE())
                .field("BLKCNT", &self.BLKCNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BLK_ATT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BLK_ATT {{ BLKSIZE: {=u16:?}, BLKCNT: {=u16:?} }}",
                self.BLKSIZE(),
                self.BLKCNT()
            )
        }
    }
    #[doc = "CLK Tuning Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_TUNE_CTRL_STATUS(pub u32);
    impl CLK_TUNE_CTRL_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn DLY_CELL_SET_POST(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLY_CELL_SET_POST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLY_CELL_SET_OUT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLY_CELL_SET_OUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLY_CELL_SET_PRE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLY_CELL_SET_PRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NXT_ERR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NXT_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAP_SEL_POST(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TAP_SEL_POST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAP_SEL_OUT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TAP_SEL_OUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAP_SEL_PRE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TAP_SEL_PRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRE_ERR(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PRE_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CLK_TUNE_CTRL_STATUS {
        #[inline(always)]
        fn default() -> CLK_TUNE_CTRL_STATUS {
            CLK_TUNE_CTRL_STATUS(0)
        }
    }
    impl core::fmt::Debug for CLK_TUNE_CTRL_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLK_TUNE_CTRL_STATUS")
                .field("DLY_CELL_SET_POST", &self.DLY_CELL_SET_POST())
                .field("DLY_CELL_SET_OUT", &self.DLY_CELL_SET_OUT())
                .field("DLY_CELL_SET_PRE", &self.DLY_CELL_SET_PRE())
                .field("NXT_ERR", &self.NXT_ERR())
                .field("TAP_SEL_POST", &self.TAP_SEL_POST())
                .field("TAP_SEL_OUT", &self.TAP_SEL_OUT())
                .field("TAP_SEL_PRE", &self.TAP_SEL_PRE())
                .field("PRE_ERR", &self.PRE_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLK_TUNE_CTRL_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CLK_TUNE_CTRL_STATUS {{ DLY_CELL_SET_POST: {=u8:?}, DLY_CELL_SET_OUT: {=u8:?}, DLY_CELL_SET_PRE: {=u8:?}, NXT_ERR: {=bool:?}, TAP_SEL_POST: {=u8:?}, TAP_SEL_OUT: {=u8:?}, TAP_SEL_PRE: {=u8:?}, PRE_ERR: {=bool:?} }}" , self . DLY_CELL_SET_POST () , self . DLY_CELL_SET_OUT () , self . DLY_CELL_SET_PRE () , self . NXT_ERR () , self . TAP_SEL_POST () , self . TAP_SEL_OUT () , self . TAP_SEL_PRE () , self . PRE_ERR ())
        }
    }
    #[doc = "Command Transfer Type"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_XFR_TYP(pub u32);
    impl CMD_XFR_TYP {
        #[must_use]
        #[inline(always)]
        pub const fn DMAEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BCEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDR_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTDSEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSBSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MSBSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NIBBLE_POS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NIBBLE_POS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC23EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC23EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSPTYP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSPTYP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCCEN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CICEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CICEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPSEL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDTYP(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDTYP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDINX(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDINX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for CMD_XFR_TYP {
        #[inline(always)]
        fn default() -> CMD_XFR_TYP {
            CMD_XFR_TYP(0)
        }
    }
    impl core::fmt::Debug for CMD_XFR_TYP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMD_XFR_TYP")
                .field("DMAEN", &self.DMAEN())
                .field("BCEN", &self.BCEN())
                .field("AC12EN", &self.AC12EN())
                .field("DDR_EN", &self.DDR_EN())
                .field("DTDSEL", &self.DTDSEL())
                .field("MSBSEL", &self.MSBSEL())
                .field("NIBBLE_POS", &self.NIBBLE_POS())
                .field("AC23EN", &self.AC23EN())
                .field("RSPTYP", &self.RSPTYP())
                .field("CCCEN", &self.CCCEN())
                .field("CICEN", &self.CICEN())
                .field("DPSEL", &self.DPSEL())
                .field("CMDTYP", &self.CMDTYP())
                .field("CMDINX", &self.CMDINX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMD_XFR_TYP {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CMD_XFR_TYP {{ DMAEN: {=bool:?}, BCEN: {=bool:?}, AC12EN: {=bool:?}, DDR_EN: {=bool:?}, DTDSEL: {=bool:?}, MSBSEL: {=bool:?}, NIBBLE_POS: {=bool:?}, AC23EN: {=bool:?}, RSPTYP: {=u8:?}, CCCEN: {=bool:?}, CICEN: {=bool:?}, DPSEL: {=bool:?}, CMDTYP: {=u8:?}, CMDINX: {=u8:?} }}" , self . DMAEN () , self . BCEN () , self . AC12EN () , self . DDR_EN () , self . DTDSEL () , self . MSBSEL () , self . NIBBLE_POS () , self . AC23EN () , self . RSPTYP () , self . CCCEN () , self . CICEN () , self . DPSEL () , self . CMDTYP () , self . CMDINX ())
        }
    }
    #[doc = "DLL (Delay Line) Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLL_CTRL(pub u32);
    impl DLL_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_ENABLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_RESET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_FORCE_UPD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_FORCE_UPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_DLY_TARGET0(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_DLY_TARGET0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_GATE_UPDATE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_GATE_UPDATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_OVERRIDE_VAL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_OVERRIDE_VAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_DLY_TARGET1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_DLY_TARGET1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_SLV_UPDATE_INT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_SLV_UPDATE_INT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_CTRL_REF_UPDATE_INT(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_CTRL_REF_UPDATE_INT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for DLL_CTRL {
        #[inline(always)]
        fn default() -> DLL_CTRL {
            DLL_CTRL(0)
        }
    }
    impl core::fmt::Debug for DLL_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DLL_CTRL")
                .field("DLL_CTRL_ENABLE", &self.DLL_CTRL_ENABLE())
                .field("DLL_CTRL_RESET", &self.DLL_CTRL_RESET())
                .field("DLL_CTRL_SLV_FORCE_UPD", &self.DLL_CTRL_SLV_FORCE_UPD())
                .field("DLL_CTRL_SLV_DLY_TARGET0", &self.DLL_CTRL_SLV_DLY_TARGET0())
                .field("DLL_CTRL_GATE_UPDATE", &self.DLL_CTRL_GATE_UPDATE())
                .field("DLL_CTRL_SLV_OVERRIDE", &self.DLL_CTRL_SLV_OVERRIDE())
                .field(
                    "DLL_CTRL_SLV_OVERRIDE_VAL",
                    &self.DLL_CTRL_SLV_OVERRIDE_VAL(),
                )
                .field("DLL_CTRL_SLV_DLY_TARGET1", &self.DLL_CTRL_SLV_DLY_TARGET1())
                .field("DLL_CTRL_SLV_UPDATE_INT", &self.DLL_CTRL_SLV_UPDATE_INT())
                .field("DLL_CTRL_REF_UPDATE_INT", &self.DLL_CTRL_REF_UPDATE_INT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DLL_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DLL_CTRL {{ DLL_CTRL_ENABLE: {=bool:?}, DLL_CTRL_RESET: {=bool:?}, DLL_CTRL_SLV_FORCE_UPD: {=bool:?}, DLL_CTRL_SLV_DLY_TARGET0: {=u8:?}, DLL_CTRL_GATE_UPDATE: {=bool:?}, DLL_CTRL_SLV_OVERRIDE: {=bool:?}, DLL_CTRL_SLV_OVERRIDE_VAL: {=u8:?}, DLL_CTRL_SLV_DLY_TARGET1: {=u8:?}, DLL_CTRL_SLV_UPDATE_INT: {=u8:?}, DLL_CTRL_REF_UPDATE_INT: {=u8:?} }}" , self . DLL_CTRL_ENABLE () , self . DLL_CTRL_RESET () , self . DLL_CTRL_SLV_FORCE_UPD () , self . DLL_CTRL_SLV_DLY_TARGET0 () , self . DLL_CTRL_GATE_UPDATE () , self . DLL_CTRL_SLV_OVERRIDE () , self . DLL_CTRL_SLV_OVERRIDE_VAL () , self . DLL_CTRL_SLV_DLY_TARGET1 () , self . DLL_CTRL_SLV_UPDATE_INT () , self . DLL_CTRL_REF_UPDATE_INT ())
        }
    }
    #[doc = "DLL Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLL_STATUS(pub u32);
    impl DLL_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn DLL_STS_SLV_LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_STS_SLV_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_STS_REF_LOCK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLL_STS_REF_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_STS_SLV_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_STS_SLV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLL_STS_REF_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLL_STS_REF_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
        }
    }
    impl Default for DLL_STATUS {
        #[inline(always)]
        fn default() -> DLL_STATUS {
            DLL_STATUS(0)
        }
    }
    impl core::fmt::Debug for DLL_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DLL_STATUS")
                .field("DLL_STS_SLV_LOCK", &self.DLL_STS_SLV_LOCK())
                .field("DLL_STS_REF_LOCK", &self.DLL_STS_REF_LOCK())
                .field("DLL_STS_SLV_SEL", &self.DLL_STS_SLV_SEL())
                .field("DLL_STS_REF_SEL", &self.DLL_STS_REF_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DLL_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DLL_STATUS {{ DLL_STS_SLV_LOCK: {=bool:?}, DLL_STS_REF_LOCK: {=bool:?}, DLL_STS_SLV_SEL: {=u8:?}, DLL_STS_REF_SEL: {=u8:?} }}" , self . DLL_STS_SLV_LOCK () , self . DLL_STS_REF_LOCK () , self . DLL_STS_SLV_SEL () , self . DLL_STS_REF_SEL ())
        }
    }
    #[doc = "Force Event"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FORCE_EVENT(pub u32);
    impl FORCE_EVENT {
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12NE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12NE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12TOE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12TOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12CE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12CE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12EBE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12EBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12IE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCNIBAC12E(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCNIBAC12E(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCTOE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCTOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCCE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCEBE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCEBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCIE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTDTOE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTDTOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTDCE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTDCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTDEBE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTDEBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTAC12E(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTAC12E(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTTNE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTTNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTDMAE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTDMAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEVTCINT(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEVTCINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FORCE_EVENT {
        #[inline(always)]
        fn default() -> FORCE_EVENT {
            FORCE_EVENT(0)
        }
    }
    impl core::fmt::Debug for FORCE_EVENT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FORCE_EVENT")
                .field("FEVTAC12NE", &self.FEVTAC12NE())
                .field("FEVTAC12TOE", &self.FEVTAC12TOE())
                .field("FEVTAC12CE", &self.FEVTAC12CE())
                .field("FEVTAC12EBE", &self.FEVTAC12EBE())
                .field("FEVTAC12IE", &self.FEVTAC12IE())
                .field("FEVTCNIBAC12E", &self.FEVTCNIBAC12E())
                .field("FEVTCTOE", &self.FEVTCTOE())
                .field("FEVTCCE", &self.FEVTCCE())
                .field("FEVTCEBE", &self.FEVTCEBE())
                .field("FEVTCIE", &self.FEVTCIE())
                .field("FEVTDTOE", &self.FEVTDTOE())
                .field("FEVTDCE", &self.FEVTDCE())
                .field("FEVTDEBE", &self.FEVTDEBE())
                .field("FEVTAC12E", &self.FEVTAC12E())
                .field("FEVTTNE", &self.FEVTTNE())
                .field("FEVTDMAE", &self.FEVTDMAE())
                .field("FEVTCINT", &self.FEVTCINT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FORCE_EVENT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FORCE_EVENT {{ FEVTAC12NE: {=bool:?}, FEVTAC12TOE: {=bool:?}, FEVTAC12CE: {=bool:?}, FEVTAC12EBE: {=bool:?}, FEVTAC12IE: {=bool:?}, FEVTCNIBAC12E: {=bool:?}, FEVTCTOE: {=bool:?}, FEVTCCE: {=bool:?}, FEVTCEBE: {=bool:?}, FEVTCIE: {=bool:?}, FEVTDTOE: {=bool:?}, FEVTDCE: {=bool:?}, FEVTDEBE: {=bool:?}, FEVTAC12E: {=bool:?}, FEVTTNE: {=bool:?}, FEVTDMAE: {=bool:?}, FEVTCINT: {=bool:?} }}" , self . FEVTAC12NE () , self . FEVTAC12TOE () , self . FEVTAC12CE () , self . FEVTAC12EBE () , self . FEVTAC12IE () , self . FEVTCNIBAC12E () , self . FEVTCTOE () , self . FEVTCCE () , self . FEVTCEBE () , self . FEVTCIE () , self . FEVTDTOE () , self . FEVTDCE () , self . FEVTDEBE () , self . FEVTAC12E () , self . FEVTTNE () , self . FEVTDMAE () , self . FEVTCINT ())
        }
    }
    #[doc = "Host Controller Capabilities"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HOST_CTRL_CAP(pub u32);
    impl HOST_CTRL_CAP {
        #[must_use]
        #[inline(always)]
        pub const fn SDR50_SUPPORT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SDR50_SUPPORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SDR104_SUPPORT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SDR104_SUPPORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDR50_SUPPORT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDR50_SUPPORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USE_TUNING_SDR50(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USE_TUNING_SDR50(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADMAS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADMAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HSS(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HSS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAS(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRS(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VS33(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VS33(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VS30(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VS30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VS18(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VS18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for HOST_CTRL_CAP {
        #[inline(always)]
        fn default() -> HOST_CTRL_CAP {
            HOST_CTRL_CAP(0)
        }
    }
    impl core::fmt::Debug for HOST_CTRL_CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HOST_CTRL_CAP")
                .field("SDR50_SUPPORT", &self.SDR50_SUPPORT())
                .field("SDR104_SUPPORT", &self.SDR104_SUPPORT())
                .field("DDR50_SUPPORT", &self.DDR50_SUPPORT())
                .field("USE_TUNING_SDR50", &self.USE_TUNING_SDR50())
                .field("MBL", &self.MBL())
                .field("ADMAS", &self.ADMAS())
                .field("HSS", &self.HSS())
                .field("DMAS", &self.DMAS())
                .field("SRS", &self.SRS())
                .field("VS33", &self.VS33())
                .field("VS30", &self.VS30())
                .field("VS18", &self.VS18())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HOST_CTRL_CAP {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HOST_CTRL_CAP {{ SDR50_SUPPORT: {=bool:?}, SDR104_SUPPORT: {=bool:?}, DDR50_SUPPORT: {=bool:?}, USE_TUNING_SDR50: {=bool:?}, MBL: {=u8:?}, ADMAS: {=bool:?}, HSS: {=bool:?}, DMAS: {=bool:?}, SRS: {=bool:?}, VS33: {=bool:?}, VS30: {=bool:?}, VS18: {=bool:?} }}" , self . SDR50_SUPPORT () , self . SDR104_SUPPORT () , self . DDR50_SUPPORT () , self . USE_TUNING_SDR50 () , self . MBL () , self . ADMAS () , self . HSS () , self . DMAS () , self . SRS () , self . VS33 () , self . VS30 () , self . VS18 ())
        }
    }
    #[doc = "Interrupt Signal Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_SIGNAL_EN(pub u32);
    impl INT_SIGNAL_EN {
        #[must_use]
        #[inline(always)]
        pub const fn CCIEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCIEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGEIEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BGEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DINTIEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DINTIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWRIEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWRIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BRRIEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BRRIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINSIEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINSIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRMIEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRMIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINTIEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINTIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTEIEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPIEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TPIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTOEIEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CTOEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCEIEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CEBEIEN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CEBEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CIEIEN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CIEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTOEIEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTOEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCEIEN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBEIEN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12EIEN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12EIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNEIEN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TNEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAEIEN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for INT_SIGNAL_EN {
        #[inline(always)]
        fn default() -> INT_SIGNAL_EN {
            INT_SIGNAL_EN(0)
        }
    }
    impl core::fmt::Debug for INT_SIGNAL_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_SIGNAL_EN")
                .field("CCIEN", &self.CCIEN())
                .field("TCIEN", &self.TCIEN())
                .field("BGEIEN", &self.BGEIEN())
                .field("DINTIEN", &self.DINTIEN())
                .field("BWRIEN", &self.BWRIEN())
                .field("BRRIEN", &self.BRRIEN())
                .field("CINSIEN", &self.CINSIEN())
                .field("CRMIEN", &self.CRMIEN())
                .field("CINTIEN", &self.CINTIEN())
                .field("RTEIEN", &self.RTEIEN())
                .field("TPIEN", &self.TPIEN())
                .field("CTOEIEN", &self.CTOEIEN())
                .field("CCEIEN", &self.CCEIEN())
                .field("CEBEIEN", &self.CEBEIEN())
                .field("CIEIEN", &self.CIEIEN())
                .field("DTOEIEN", &self.DTOEIEN())
                .field("DCEIEN", &self.DCEIEN())
                .field("DEBEIEN", &self.DEBEIEN())
                .field("AC12EIEN", &self.AC12EIEN())
                .field("TNEIEN", &self.TNEIEN())
                .field("DMAEIEN", &self.DMAEIEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_SIGNAL_EN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INT_SIGNAL_EN {{ CCIEN: {=bool:?}, TCIEN: {=bool:?}, BGEIEN: {=bool:?}, DINTIEN: {=bool:?}, BWRIEN: {=bool:?}, BRRIEN: {=bool:?}, CINSIEN: {=bool:?}, CRMIEN: {=bool:?}, CINTIEN: {=bool:?}, RTEIEN: {=bool:?}, TPIEN: {=bool:?}, CTOEIEN: {=bool:?}, CCEIEN: {=bool:?}, CEBEIEN: {=bool:?}, CIEIEN: {=bool:?}, DTOEIEN: {=bool:?}, DCEIEN: {=bool:?}, DEBEIEN: {=bool:?}, AC12EIEN: {=bool:?}, TNEIEN: {=bool:?}, DMAEIEN: {=bool:?} }}" , self . CCIEN () , self . TCIEN () , self . BGEIEN () , self . DINTIEN () , self . BWRIEN () , self . BRRIEN () , self . CINSIEN () , self . CRMIEN () , self . CINTIEN () , self . RTEIEN () , self . TPIEN () , self . CTOEIEN () , self . CCEIEN () , self . CEBEIEN () , self . CIEIEN () , self . DTOEIEN () , self . DCEIEN () , self . DEBEIEN () , self . AC12EIEN () , self . TNEIEN () , self . DMAEIEN ())
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_STATUS(pub u32);
    impl INT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn CC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DINT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BRR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRM(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TP(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERR_INT_STATUS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERR_INT_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTOE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CTOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CEBE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CEBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CIE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTOE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12E(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12E(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for INT_STATUS {
        #[inline(always)]
        fn default() -> INT_STATUS {
            INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_STATUS")
                .field("CC", &self.CC())
                .field("TC", &self.TC())
                .field("BGE", &self.BGE())
                .field("DINT", &self.DINT())
                .field("BWR", &self.BWR())
                .field("BRR", &self.BRR())
                .field("CINS", &self.CINS())
                .field("CRM", &self.CRM())
                .field("CINT", &self.CINT())
                .field("RTE", &self.RTE())
                .field("TP", &self.TP())
                .field("ERR_INT_STATUS", &self.ERR_INT_STATUS())
                .field("CTOE", &self.CTOE())
                .field("CCE", &self.CCE())
                .field("CEBE", &self.CEBE())
                .field("CIE", &self.CIE())
                .field("DTOE", &self.DTOE())
                .field("DCE", &self.DCE())
                .field("DEBE", &self.DEBE())
                .field("AC12E", &self.AC12E())
                .field("TNE", &self.TNE())
                .field("DMAE", &self.DMAE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INT_STATUS {{ CC: {=bool:?}, TC: {=bool:?}, BGE: {=bool:?}, DINT: {=bool:?}, BWR: {=bool:?}, BRR: {=bool:?}, CINS: {=bool:?}, CRM: {=bool:?}, CINT: {=bool:?}, RTE: {=bool:?}, TP: {=bool:?}, ERR_INT_STATUS: {=bool:?}, CTOE: {=bool:?}, CCE: {=bool:?}, CEBE: {=bool:?}, CIE: {=bool:?}, DTOE: {=bool:?}, DCE: {=bool:?}, DEBE: {=bool:?}, AC12E: {=bool:?}, TNE: {=bool:?}, DMAE: {=bool:?} }}" , self . CC () , self . TC () , self . BGE () , self . DINT () , self . BWR () , self . BRR () , self . CINS () , self . CRM () , self . CINT () , self . RTE () , self . TP () , self . ERR_INT_STATUS () , self . CTOE () , self . CCE () , self . CEBE () , self . CIE () , self . DTOE () , self . DCE () , self . DEBE () , self . AC12E () , self . TNE () , self . DMAE ())
        }
    }
    #[doc = "Interrupt Status Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_STATUS_EN(pub u32);
    impl INT_STATUS_EN {
        #[must_use]
        #[inline(always)]
        pub const fn CCSEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCSEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGESEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BGESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DINTSEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DINTSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWRSEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWRSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BRRSEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BRRSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINSSEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINSSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRMSEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRMSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINTSEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINTSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTESEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPSEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TPSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTOESEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CTOESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCESEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CEBESEN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CEBESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CIESEN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CIESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTOESEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTOESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCESEN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBESEN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12ESEN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12ESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNESEN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TNESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAESEN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAESEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for INT_STATUS_EN {
        #[inline(always)]
        fn default() -> INT_STATUS_EN {
            INT_STATUS_EN(0)
        }
    }
    impl core::fmt::Debug for INT_STATUS_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_STATUS_EN")
                .field("CCSEN", &self.CCSEN())
                .field("TCSEN", &self.TCSEN())
                .field("BGESEN", &self.BGESEN())
                .field("DINTSEN", &self.DINTSEN())
                .field("BWRSEN", &self.BWRSEN())
                .field("BRRSEN", &self.BRRSEN())
                .field("CINSSEN", &self.CINSSEN())
                .field("CRMSEN", &self.CRMSEN())
                .field("CINTSEN", &self.CINTSEN())
                .field("RTESEN", &self.RTESEN())
                .field("TPSEN", &self.TPSEN())
                .field("CTOESEN", &self.CTOESEN())
                .field("CCESEN", &self.CCESEN())
                .field("CEBESEN", &self.CEBESEN())
                .field("CIESEN", &self.CIESEN())
                .field("DTOESEN", &self.DTOESEN())
                .field("DCESEN", &self.DCESEN())
                .field("DEBESEN", &self.DEBESEN())
                .field("AC12ESEN", &self.AC12ESEN())
                .field("TNESEN", &self.TNESEN())
                .field("DMAESEN", &self.DMAESEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_STATUS_EN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INT_STATUS_EN {{ CCSEN: {=bool:?}, TCSEN: {=bool:?}, BGESEN: {=bool:?}, DINTSEN: {=bool:?}, BWRSEN: {=bool:?}, BRRSEN: {=bool:?}, CINSSEN: {=bool:?}, CRMSEN: {=bool:?}, CINTSEN: {=bool:?}, RTESEN: {=bool:?}, TPSEN: {=bool:?}, CTOESEN: {=bool:?}, CCESEN: {=bool:?}, CEBESEN: {=bool:?}, CIESEN: {=bool:?}, DTOESEN: {=bool:?}, DCESEN: {=bool:?}, DEBESEN: {=bool:?}, AC12ESEN: {=bool:?}, TNESEN: {=bool:?}, DMAESEN: {=bool:?} }}" , self . CCSEN () , self . TCSEN () , self . BGESEN () , self . DINTSEN () , self . BWRSEN () , self . BRRSEN () , self . CINSSEN () , self . CRMSEN () , self . CINTSEN () , self . RTESEN () , self . TPSEN () , self . CTOESEN () , self . CCESEN () , self . CEBESEN () , self . CIESEN () , self . DTOESEN () , self . DCESEN () , self . DEBESEN () , self . AC12ESEN () , self . TNESEN () , self . DMAESEN ())
        }
    }
    #[doc = "Mixer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MIX_CTRL(pub u32);
    impl MIX_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DMAEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BCEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC12EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DDR_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DDR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTDSEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSBSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MSBSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NIBBLE_POS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NIBBLE_POS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AC23EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC23EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXE_TUNE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXE_TUNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMP_CLK_SEL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMP_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AUTO_TUNE_EN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AUTO_TUNE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FBCLK_SEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FBCLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MIX_CTRL {
        #[inline(always)]
        fn default() -> MIX_CTRL {
            MIX_CTRL(0)
        }
    }
    impl core::fmt::Debug for MIX_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MIX_CTRL")
                .field("DMAEN", &self.DMAEN())
                .field("BCEN", &self.BCEN())
                .field("AC12EN", &self.AC12EN())
                .field("DDR_EN", &self.DDR_EN())
                .field("DTDSEL", &self.DTDSEL())
                .field("MSBSEL", &self.MSBSEL())
                .field("NIBBLE_POS", &self.NIBBLE_POS())
                .field("AC23EN", &self.AC23EN())
                .field("EXE_TUNE", &self.EXE_TUNE())
                .field("SMP_CLK_SEL", &self.SMP_CLK_SEL())
                .field("AUTO_TUNE_EN", &self.AUTO_TUNE_EN())
                .field("FBCLK_SEL", &self.FBCLK_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MIX_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MIX_CTRL {{ DMAEN: {=bool:?}, BCEN: {=bool:?}, AC12EN: {=bool:?}, DDR_EN: {=bool:?}, DTDSEL: {=bool:?}, MSBSEL: {=bool:?}, NIBBLE_POS: {=bool:?}, AC23EN: {=bool:?}, EXE_TUNE: {=bool:?}, SMP_CLK_SEL: {=bool:?}, AUTO_TUNE_EN: {=bool:?}, FBCLK_SEL: {=bool:?} }}" , self . DMAEN () , self . BCEN () , self . AC12EN () , self . DDR_EN () , self . DTDSEL () , self . MSBSEL () , self . NIBBLE_POS () , self . AC23EN () , self . EXE_TUNE () , self . SMP_CLK_SEL () , self . AUTO_TUNE_EN () , self . FBCLK_SEL ())
        }
    }
    #[doc = "eMMC Boot"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MMC_BOOT(pub u32);
    impl MMC_BOOT {
        #[must_use]
        #[inline(always)]
        pub const fn DTOCV_ACK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DTOCV_ACK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BOOT_ACK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BOOT_ACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BOOT_MODE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BOOT_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BOOT_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BOOT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AUTO_SABG_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AUTO_SABG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_TIME_OUT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DISABLE_TIME_OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BOOT_BLK_CNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_BOOT_BLK_CNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MMC_BOOT {
        #[inline(always)]
        fn default() -> MMC_BOOT {
            MMC_BOOT(0)
        }
    }
    impl core::fmt::Debug for MMC_BOOT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MMC_BOOT")
                .field("DTOCV_ACK", &self.DTOCV_ACK())
                .field("BOOT_ACK", &self.BOOT_ACK())
                .field("BOOT_MODE", &self.BOOT_MODE())
                .field("BOOT_EN", &self.BOOT_EN())
                .field("AUTO_SABG_EN", &self.AUTO_SABG_EN())
                .field("DISABLE_TIME_OUT", &self.DISABLE_TIME_OUT())
                .field("BOOT_BLK_CNT", &self.BOOT_BLK_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MMC_BOOT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MMC_BOOT {{ DTOCV_ACK: {=u8:?}, BOOT_ACK: {=bool:?}, BOOT_MODE: {=bool:?}, BOOT_EN: {=bool:?}, AUTO_SABG_EN: {=bool:?}, DISABLE_TIME_OUT: {=bool:?}, BOOT_BLK_CNT: {=u16:?} }}" , self . DTOCV_ACK () , self . BOOT_ACK () , self . BOOT_MODE () , self . BOOT_EN () , self . AUTO_SABG_EN () , self . DISABLE_TIME_OUT () , self . BOOT_BLK_CNT ())
        }
    }
    #[doc = "Present State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PRES_STATE(pub u32);
    impl PRES_STATE {
        #[must_use]
        #[inline(always)]
        pub const fn CIHB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CIHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDIHB(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDIHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLA(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DLA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SDSTB(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SDSTB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WTA(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WTA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTA(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BWEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BWEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BREN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSCD(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CINST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CINST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLSL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CLSL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DLSL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DLSL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PRES_STATE {
        #[inline(always)]
        fn default() -> PRES_STATE {
            PRES_STATE(0)
        }
    }
    impl core::fmt::Debug for PRES_STATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PRES_STATE")
                .field("CIHB", &self.CIHB())
                .field("CDIHB", &self.CDIHB())
                .field("DLA", &self.DLA())
                .field("SDSTB", &self.SDSTB())
                .field("WTA", &self.WTA())
                .field("RTA", &self.RTA())
                .field("BWEN", &self.BWEN())
                .field("BREN", &self.BREN())
                .field("RTR", &self.RTR())
                .field("TSCD", &self.TSCD())
                .field("CINST", &self.CINST())
                .field("CLSL", &self.CLSL())
                .field("DLSL", &self.DLSL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PRES_STATE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PRES_STATE {{ CIHB: {=bool:?}, CDIHB: {=bool:?}, DLA: {=bool:?}, SDSTB: {=bool:?}, WTA: {=bool:?}, RTA: {=bool:?}, BWEN: {=bool:?}, BREN: {=bool:?}, RTR: {=bool:?}, TSCD: {=bool:?}, CINST: {=bool:?}, CLSL: {=bool:?}, DLSL: {=u8:?} }}" , self . CIHB () , self . CDIHB () , self . DLA () , self . SDSTB () , self . WTA () , self . RTA () , self . BWEN () , self . BREN () , self . RTR () , self . TSCD () , self . CINST () , self . CLSL () , self . DLSL ())
        }
    }
    #[doc = "Protocol Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PROT_CTRL(pub u32);
    impl PROT_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DTW(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DTW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn D3CD(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_D3CD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMODE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMASEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMASEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SABGREQ(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SABGREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CREQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWCTL(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWCTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IABG(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IABG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RD_DONE_NO_8CLK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RD_DONE_NO_8CLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WECINT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WECINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WECINS(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WECINS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WECRM(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WECRM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BURST_LEN_EN(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BURST_LEN_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NON_EXACT_BLK_RD(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NON_EXACT_BLK_RD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for PROT_CTRL {
        #[inline(always)]
        fn default() -> PROT_CTRL {
            PROT_CTRL(0)
        }
    }
    impl core::fmt::Debug for PROT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PROT_CTRL")
                .field("DTW", &self.DTW())
                .field("D3CD", &self.D3CD())
                .field("EMODE", &self.EMODE())
                .field("DMASEL", &self.DMASEL())
                .field("SABGREQ", &self.SABGREQ())
                .field("CREQ", &self.CREQ())
                .field("RWCTL", &self.RWCTL())
                .field("IABG", &self.IABG())
                .field("RD_DONE_NO_8CLK", &self.RD_DONE_NO_8CLK())
                .field("WECINT", &self.WECINT())
                .field("WECINS", &self.WECINS())
                .field("WECRM", &self.WECRM())
                .field("BURST_LEN_EN", &self.BURST_LEN_EN())
                .field("NON_EXACT_BLK_RD", &self.NON_EXACT_BLK_RD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PROT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PROT_CTRL {{ DTW: {=u8:?}, D3CD: {=bool:?}, EMODE: {=u8:?}, DMASEL: {=u8:?}, SABGREQ: {=bool:?}, CREQ: {=bool:?}, RWCTL: {=bool:?}, IABG: {=bool:?}, RD_DONE_NO_8CLK: {=bool:?}, WECINT: {=bool:?}, WECINS: {=bool:?}, WECRM: {=bool:?}, BURST_LEN_EN: {=u8:?}, NON_EXACT_BLK_RD: {=bool:?} }}" , self . DTW () , self . D3CD () , self . EMODE () , self . DMASEL () , self . SABGREQ () , self . CREQ () , self . RWCTL () , self . IABG () , self . RD_DONE_NO_8CLK () , self . WECINT () , self . WECINS () , self . WECRM () , self . BURST_LEN_EN () , self . NON_EXACT_BLK_RD ())
        }
    }
    #[doc = "System Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYS_CTRL(pub u32);
    impl SYS_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DVS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DVS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SDCLKFS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SDCLKFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DTOCV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DTOCV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RST_FIFO(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RST_FIFO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPP_RST_N(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IPP_RST_N(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTA(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTC(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTD(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INITA(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INITA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTT(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for SYS_CTRL {
        #[inline(always)]
        fn default() -> SYS_CTRL {
            SYS_CTRL(0)
        }
    }
    impl core::fmt::Debug for SYS_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYS_CTRL")
                .field("DVS", &self.DVS())
                .field("SDCLKFS", &self.SDCLKFS())
                .field("DTOCV", &self.DTOCV())
                .field("RST_FIFO", &self.RST_FIFO())
                .field("IPP_RST_N", &self.IPP_RST_N())
                .field("RSTA", &self.RSTA())
                .field("RSTC", &self.RSTC())
                .field("RSTD", &self.RSTD())
                .field("INITA", &self.INITA())
                .field("RSTT", &self.RSTT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYS_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SYS_CTRL {{ DVS: {=u8:?}, SDCLKFS: {=u8:?}, DTOCV: {=u8:?}, RST_FIFO: {=bool:?}, IPP_RST_N: {=bool:?}, RSTA: {=bool:?}, RSTC: {=bool:?}, RSTD: {=bool:?}, INITA: {=bool:?}, RSTT: {=bool:?} }}" , self . DVS () , self . SDCLKFS () , self . DTOCV () , self . RST_FIFO () , self . IPP_RST_N () , self . RSTA () , self . RSTC () , self . RSTD () , self . INITA () , self . RSTT ())
        }
    }
    #[doc = "Tuning Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TUNING_CTRL(pub u32);
    impl TUNING_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_START_TAP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNING_START_TAP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS_CMD_CHK_FOR_STD_TUNING(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS_CMD_CHK_FOR_STD_TUNING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_COUNTER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNING_COUNTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_STEP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNING_STEP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_WINDOW(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNING_WINDOW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STD_TUNING_EN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STD_TUNING_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for TUNING_CTRL {
        #[inline(always)]
        fn default() -> TUNING_CTRL {
            TUNING_CTRL(0)
        }
    }
    impl core::fmt::Debug for TUNING_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TUNING_CTRL")
                .field("TUNING_START_TAP", &self.TUNING_START_TAP())
                .field(
                    "DIS_CMD_CHK_FOR_STD_TUNING",
                    &self.DIS_CMD_CHK_FOR_STD_TUNING(),
                )
                .field("TUNING_COUNTER", &self.TUNING_COUNTER())
                .field("TUNING_STEP", &self.TUNING_STEP())
                .field("TUNING_WINDOW", &self.TUNING_WINDOW())
                .field("STD_TUNING_EN", &self.STD_TUNING_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TUNING_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TUNING_CTRL {{ TUNING_START_TAP: {=u8:?}, DIS_CMD_CHK_FOR_STD_TUNING: {=bool:?}, TUNING_COUNTER: {=u8:?}, TUNING_STEP: {=u8:?}, TUNING_WINDOW: {=u8:?}, STD_TUNING_EN: {=bool:?} }}" , self . TUNING_START_TAP () , self . DIS_CMD_CHK_FOR_STD_TUNING () , self . TUNING_COUNTER () , self . TUNING_STEP () , self . TUNING_WINDOW () , self . STD_TUNING_EN ())
        }
    }
    #[doc = "Vendor Specific Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VEND_SPEC(pub u32);
    impl VEND_SPEC {
        #[must_use]
        #[inline(always)]
        pub const fn AC12_WR_CHKBUSY_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AC12_WR_CHKBUSY_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRC_SDCLK_ON(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRC_SDCLK_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC_CHK_DIS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC_CHK_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMD_BYTE_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMD_BYTE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for VEND_SPEC {
        #[inline(always)]
        fn default() -> VEND_SPEC {
            VEND_SPEC(0)
        }
    }
    impl core::fmt::Debug for VEND_SPEC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VEND_SPEC")
                .field("AC12_WR_CHKBUSY_EN", &self.AC12_WR_CHKBUSY_EN())
                .field("FRC_SDCLK_ON", &self.FRC_SDCLK_ON())
                .field("CRC_CHK_DIS", &self.CRC_CHK_DIS())
                .field("CMD_BYTE_EN", &self.CMD_BYTE_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VEND_SPEC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VEND_SPEC {{ AC12_WR_CHKBUSY_EN: {=bool:?}, FRC_SDCLK_ON: {=bool:?}, CRC_CHK_DIS: {=bool:?}, CMD_BYTE_EN: {=bool:?} }}" , self . AC12_WR_CHKBUSY_EN () , self . FRC_SDCLK_ON () , self . CRC_CHK_DIS () , self . CMD_BYTE_EN ())
        }
    }
    #[doc = "Vendor Specific 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VEND_SPEC2(pub u32);
    impl VEND_SPEC2 {
        #[must_use]
        #[inline(always)]
        pub const fn CARD_INT_D3_TEST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CARD_INT_D3_TEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_BIT_EN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNING_BIT_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNING_CMD_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TUNING_CMD_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACMD23_ARGU2_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACMD23_ARGU2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EN_32K_CLK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EN_32K_CLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for VEND_SPEC2 {
        #[inline(always)]
        fn default() -> VEND_SPEC2 {
            VEND_SPEC2(0)
        }
    }
    impl core::fmt::Debug for VEND_SPEC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VEND_SPEC2")
                .field("CARD_INT_D3_TEST", &self.CARD_INT_D3_TEST())
                .field("TUNING_BIT_EN", &self.TUNING_BIT_EN())
                .field("TUNING_CMD_EN", &self.TUNING_CMD_EN())
                .field("ACMD23_ARGU2_EN", &self.ACMD23_ARGU2_EN())
                .field("EN_32K_CLK", &self.EN_32K_CLK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VEND_SPEC2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VEND_SPEC2 {{ CARD_INT_D3_TEST: {=bool:?}, TUNING_BIT_EN: {=u8:?}, TUNING_CMD_EN: {=bool:?}, ACMD23_ARGU2_EN: {=bool:?}, EN_32K_CLK: {=bool:?} }}" , self . CARD_INT_D3_TEST () , self . TUNING_BIT_EN () , self . TUNING_CMD_EN () , self . ACMD23_ARGU2_EN () , self . EN_32K_CLK ())
        }
    }
    #[doc = "Watermark Level"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WTMK_LVL(pub u32);
    impl WTMK_LVL {
        #[must_use]
        #[inline(always)]
        pub const fn RD_WML(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RD_WML(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RD_BRST_LEN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RD_BRST_LEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_WML(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WR_WML(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_BRST_LEN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WR_BRST_LEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for WTMK_LVL {
        #[inline(always)]
        fn default() -> WTMK_LVL {
            WTMK_LVL(0)
        }
    }
    impl core::fmt::Debug for WTMK_LVL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WTMK_LVL")
                .field("RD_WML", &self.RD_WML())
                .field("RD_BRST_LEN", &self.RD_BRST_LEN())
                .field("WR_WML", &self.WR_WML())
                .field("WR_BRST_LEN", &self.WR_BRST_LEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WTMK_LVL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WTMK_LVL {{ RD_WML: {=u8:?}, RD_BRST_LEN: {=u8:?}, WR_WML: {=u8:?}, WR_BRST_LEN: {=u8:?} }}" , self . RD_WML () , self . RD_BRST_LEN () , self . WR_WML () , self . WR_BRST_LEN ())
        }
    }
}
