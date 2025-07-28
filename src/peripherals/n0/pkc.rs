#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PKC {
    ptr: *mut u8,
}
unsafe impl Send for PKC {}
unsafe impl Sync for PKC {}
impl PKC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn PKC_STATUS(self) -> crate::common::Reg<regs::PKC_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_CTRL(self) -> crate::common::Reg<regs::PKC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_CFG(self) -> crate::common::Reg<regs::PKC_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_MODE1(self) -> crate::common::Reg<regs::PKC_MODE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_XYPTR1(self) -> crate::common::Reg<regs::PKC_XYPTR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_ZRPTR1(self) -> crate::common::Reg<regs::PKC_ZRPTR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_LEN1(self) -> crate::common::Reg<regs::PKC_LEN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_MODE2(self) -> crate::common::Reg<regs::PKC_MODE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_XYPTR2(self) -> crate::common::Reg<regs::PKC_XYPTR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_ZRPTR2(self) -> crate::common::Reg<regs::PKC_ZRPTR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_LEN2(self) -> crate::common::Reg<regs::PKC_LEN2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_UPTR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_UPTRT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_ULEN(self) -> crate::common::Reg<regs::PKC_ULEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_MCDATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_VERSION(self) -> crate::common::Reg<regs::PKC_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_SOFT_RST(self) -> crate::common::Reg<regs::PKC_SOFT_RST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fb0usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_ACCESS_ERR(
        self,
    ) -> crate::common::Reg<regs::PKC_ACCESS_ERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_ACCESS_ERR_CLR(
        self,
    ) -> crate::common::Reg<regs::PKC_ACCESS_ERR_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc4usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_CLR_ENABLE(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_CLR_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd8usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_SET_ENABLE(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_SET_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fdcusize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_ENABLE(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_CLR_STATUS(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_CLR_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_INT_SET_STATUS(
        self,
    ) -> crate::common::Reg<regs::PKC_INT_SET_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[inline(always)]
    pub const fn PKC_MODULE_ID(self) -> crate::common::Reg<regs::PKC_MODULE_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_ACCESS_ERR(pub u32);
    impl PKC_ACCESS_ERR {
        #[must_use]
        #[inline(always)]
        pub const fn APB_NOTAV(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_APB_NOTAV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn APB_WRGMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_APB_WRGMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn APB_MASTER(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_APB_MASTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AHB(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PKCC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PKCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDET(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTRL(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UCRC(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UCRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for PKC_ACCESS_ERR {
        #[inline(always)]
        fn default() -> PKC_ACCESS_ERR {
            PKC_ACCESS_ERR(0)
        }
    }
    impl core::fmt::Debug for PKC_ACCESS_ERR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_ACCESS_ERR")
                .field("APB_NOTAV", &self.APB_NOTAV())
                .field("APB_WRGMD", &self.APB_WRGMD())
                .field("APB_MASTER", &self.APB_MASTER())
                .field("AHB", &self.AHB())
                .field("PKCC", &self.PKCC())
                .field("FDET", &self.FDET())
                .field("CTRL", &self.CTRL())
                .field("UCRC", &self.UCRC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_ACCESS_ERR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_ACCESS_ERR {{ APB_NOTAV: {=bool:?}, APB_WRGMD: {=bool:?}, APB_MASTER: {=u8:?}, AHB: {=bool:?}, PKCC: {=bool:?}, FDET: {=bool:?}, CTRL: {=bool:?}, UCRC: {=bool:?} }}" , self . APB_NOTAV () , self . APB_WRGMD () , self . APB_MASTER () , self . AHB () , self . PKCC () , self . FDET () , self . CTRL () , self . UCRC ())
        }
    }
    #[doc = "Clear Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_ACCESS_ERR_CLR(pub u32);
    impl PKC_ACCESS_ERR_CLR {
        #[must_use]
        #[inline(always)]
        pub const fn ERR_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERR_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_ACCESS_ERR_CLR {
        #[inline(always)]
        fn default() -> PKC_ACCESS_ERR_CLR {
            PKC_ACCESS_ERR_CLR(0)
        }
    }
    impl core::fmt::Debug for PKC_ACCESS_ERR_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_ACCESS_ERR_CLR")
                .field("ERR_CLR", &self.ERR_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_ACCESS_ERR_CLR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_ACCESS_ERR_CLR {{ ERR_CLR: {=bool:?} }}",
                self.ERR_CLR()
            )
        }
    }
    #[doc = "Configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_CFG(pub u32);
    impl PKC_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn IDLEOP(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IDLEOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFU1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFU2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFU2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLKRND(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CLKRND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REDMULNOISE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REDMULNOISE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RNDDLY(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RNDDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBXNOISE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBXNOISE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALPNOISE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALPNOISE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMULNOISE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FMULNOISE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for PKC_CFG {
        #[inline(always)]
        fn default() -> PKC_CFG {
            PKC_CFG(0)
        }
    }
    impl core::fmt::Debug for PKC_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_CFG")
                .field("IDLEOP", &self.IDLEOP())
                .field("RFU1", &self.RFU1())
                .field("RFU2", &self.RFU2())
                .field("CLKRND", &self.CLKRND())
                .field("REDMULNOISE", &self.REDMULNOISE())
                .field("RNDDLY", &self.RNDDLY())
                .field("SBXNOISE", &self.SBXNOISE())
                .field("ALPNOISE", &self.ALPNOISE())
                .field("FMULNOISE", &self.FMULNOISE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_CFG {{ IDLEOP: {=bool:?}, RFU1: {=bool:?}, RFU2: {=bool:?}, CLKRND: {=bool:?}, REDMULNOISE: {=bool:?}, RNDDLY: {=u8:?}, SBXNOISE: {=bool:?}, ALPNOISE: {=bool:?}, FMULNOISE: {=bool:?} }}" , self . IDLEOP () , self . RFU1 () , self . RFU2 () , self . CLKRND () , self . REDMULNOISE () , self . RNDDLY () , self . SBXNOISE () , self . ALPNOISE () , self . FMULNOISE ())
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_CTRL(pub u32);
    impl PKC_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOD1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOD1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOD2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOD2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOM1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOM2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOM2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOU(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GF2CONV(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GF2CONV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLRCACHE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CLRCACHE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CACHE_EN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CACHE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REDMUL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_REDMUL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for PKC_CTRL {
        #[inline(always)]
        fn default() -> PKC_CTRL {
            PKC_CTRL(0)
        }
    }
    impl core::fmt::Debug for PKC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_CTRL")
                .field("RESET", &self.RESET())
                .field("STOP", &self.STOP())
                .field("GOD1", &self.GOD1())
                .field("GOD2", &self.GOD2())
                .field("GOM1", &self.GOM1())
                .field("GOM2", &self.GOM2())
                .field("GOU", &self.GOU())
                .field("GF2CONV", &self.GF2CONV())
                .field("CLRCACHE", &self.CLRCACHE())
                .field("CACHE_EN", &self.CACHE_EN())
                .field("REDMUL", &self.REDMUL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_CTRL {{ RESET: {=bool:?}, STOP: {=bool:?}, GOD1: {=bool:?}, GOD2: {=bool:?}, GOM1: {=bool:?}, GOM2: {=bool:?}, GOU: {=bool:?}, GF2CONV: {=bool:?}, CLRCACHE: {=bool:?}, CACHE_EN: {=bool:?}, REDMUL: {=u8:?} }}" , self . RESET () , self . STOP () , self . GOD1 () , self . GOD2 () , self . GOM1 () , self . GOM2 () , self . GOU () , self . GF2CONV () , self . CLRCACHE () , self . CACHE_EN () , self . REDMUL ())
        }
    }
    #[doc = "Interrupt enable clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_CLR_ENABLE(pub u32);
    impl PKC_INT_CLR_ENABLE {
        #[must_use]
        #[inline(always)]
        pub const fn EN_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EN_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_CLR_ENABLE {
        #[inline(always)]
        fn default() -> PKC_INT_CLR_ENABLE {
            PKC_INT_CLR_ENABLE(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_CLR_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_CLR_ENABLE")
                .field("EN_PDONE", &self.EN_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_CLR_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_CLR_ENABLE {{ EN_PDONE: {=bool:?} }}",
                self.EN_PDONE()
            )
        }
    }
    #[doc = "Interrupt status clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_CLR_STATUS(pub u32);
    impl PKC_INT_CLR_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn INT_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_CLR_STATUS {
        #[inline(always)]
        fn default() -> PKC_INT_CLR_STATUS {
            PKC_INT_CLR_STATUS(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_CLR_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_CLR_STATUS")
                .field("INT_PDONE", &self.INT_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_CLR_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_CLR_STATUS {{ INT_PDONE: {=bool:?} }}",
                self.INT_PDONE()
            )
        }
    }
    #[doc = "Interrupt enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_ENABLE(pub u32);
    impl PKC_INT_ENABLE {
        #[must_use]
        #[inline(always)]
        pub const fn EN_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EN_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_ENABLE {
        #[inline(always)]
        fn default() -> PKC_INT_ENABLE {
            PKC_INT_ENABLE(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_ENABLE")
                .field("EN_PDONE", &self.EN_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_ENABLE {{ EN_PDONE: {=bool:?} }}",
                self.EN_PDONE()
            )
        }
    }
    #[doc = "Interrupt enable set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_SET_ENABLE(pub u32);
    impl PKC_INT_SET_ENABLE {
        #[must_use]
        #[inline(always)]
        pub const fn EN_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EN_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_SET_ENABLE {
        #[inline(always)]
        fn default() -> PKC_INT_SET_ENABLE {
            PKC_INT_SET_ENABLE(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_SET_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_SET_ENABLE")
                .field("EN_PDONE", &self.EN_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_SET_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_SET_ENABLE {{ EN_PDONE: {=bool:?} }}",
                self.EN_PDONE()
            )
        }
    }
    #[doc = "Interrupt status set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_SET_STATUS(pub u32);
    impl PKC_INT_SET_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn INT_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_SET_STATUS {
        #[inline(always)]
        fn default() -> PKC_INT_SET_STATUS {
            PKC_INT_SET_STATUS(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_SET_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_SET_STATUS")
                .field("INT_PDONE", &self.INT_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_SET_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_SET_STATUS {{ INT_PDONE: {=bool:?} }}",
                self.INT_PDONE()
            )
        }
    }
    #[doc = "Interrupt status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_INT_STATUS(pub u32);
    impl PKC_INT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn INT_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_INT_STATUS {
        #[inline(always)]
        fn default() -> PKC_INT_STATUS {
            PKC_INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for PKC_INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_INT_STATUS")
                .field("INT_PDONE", &self.INT_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_INT_STATUS {{ INT_PDONE: {=bool:?} }}",
                self.INT_PDONE()
            )
        }
    }
    #[doc = "Length register, parameter set 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_LEN1(pub u32);
    impl PKC_LEN1 {
        #[must_use]
        #[inline(always)]
        pub const fn LEN(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LEN(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCLEN(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MCLEN(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_LEN1 {
        #[inline(always)]
        fn default() -> PKC_LEN1 {
            PKC_LEN1(0)
        }
    }
    impl core::fmt::Debug for PKC_LEN1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_LEN1")
                .field("LEN", &self.LEN())
                .field("MCLEN", &self.MCLEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_LEN1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_LEN1 {{ LEN: {=u16:?}, MCLEN: {=u16:?} }}",
                self.LEN(),
                self.MCLEN()
            )
        }
    }
    #[doc = "Length register, parameter set 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_LEN2(pub u32);
    impl PKC_LEN2 {
        #[must_use]
        #[inline(always)]
        pub const fn LEN(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LEN(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCLEN(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MCLEN(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_LEN2 {
        #[inline(always)]
        fn default() -> PKC_LEN2 {
            PKC_LEN2(0)
        }
    }
    impl core::fmt::Debug for PKC_LEN2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_LEN2")
                .field("LEN", &self.LEN())
                .field("MCLEN", &self.MCLEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_LEN2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_LEN2 {{ LEN: {=u16:?}, MCLEN: {=u16:?} }}",
                self.LEN(),
                self.MCLEN()
            )
        }
    }
    #[doc = "Mode register, parameter set 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_MODE1(pub u32);
    impl PKC_MODE1 {
        #[must_use]
        #[inline(always)]
        pub const fn MODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PKC_MODE1 {
        #[inline(always)]
        fn default() -> PKC_MODE1 {
            PKC_MODE1(0)
        }
    }
    impl core::fmt::Debug for PKC_MODE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_MODE1")
                .field("MODE", &self.MODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_MODE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PKC_MODE1 {{ MODE: {=u8:?} }}", self.MODE())
        }
    }
    #[doc = "Mode register, parameter set 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_MODE2(pub u32);
    impl PKC_MODE2 {
        #[must_use]
        #[inline(always)]
        pub const fn MODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PKC_MODE2 {
        #[inline(always)]
        fn default() -> PKC_MODE2 {
            PKC_MODE2(0)
        }
    }
    impl core::fmt::Debug for PKC_MODE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_MODE2")
                .field("MODE", &self.MODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_MODE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PKC_MODE2 {{ MODE: {=u8:?} }}", self.MODE())
        }
    }
    #[doc = "Module ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_MODULE_ID(pub u32);
    impl PKC_MODULE_ID {
        #[must_use]
        #[inline(always)]
        pub const fn SIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MINOR_REV(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MINOR_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAJOR_REV(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAJOR_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ID(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ID(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_MODULE_ID {
        #[inline(always)]
        fn default() -> PKC_MODULE_ID {
            PKC_MODULE_ID(0)
        }
    }
    impl core::fmt::Debug for PKC_MODULE_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_MODULE_ID")
                .field("SIZE", &self.SIZE())
                .field("MINOR_REV", &self.MINOR_REV())
                .field("MAJOR_REV", &self.MAJOR_REV())
                .field("ID", &self.ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_MODULE_ID {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_MODULE_ID {{ SIZE: {=u8:?}, MINOR_REV: {=u8:?}, MAJOR_REV: {=u8:?}, ID: {=u16:?} }}" , self . SIZE () , self . MINOR_REV () , self . MAJOR_REV () , self . ID ())
        }
    }
    #[doc = "Software reset"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_SOFT_RST(pub u32);
    impl PKC_SOFT_RST {
        #[must_use]
        #[inline(always)]
        pub const fn SOFT_RST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOFT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PKC_SOFT_RST {
        #[inline(always)]
        fn default() -> PKC_SOFT_RST {
            PKC_SOFT_RST(0)
        }
    }
    impl core::fmt::Debug for PKC_SOFT_RST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_SOFT_RST")
                .field("SOFT_RST", &self.SOFT_RST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_SOFT_RST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PKC_SOFT_RST {{ SOFT_RST: {=bool:?} }}", self.SOFT_RST())
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_STATUS(pub u32);
    impl PKC_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn ACTIV(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACTIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CARRY(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CARRY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ZERO(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ZERO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOANY(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOANY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCKED(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCKED(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
    }
    impl Default for PKC_STATUS {
        #[inline(always)]
        fn default() -> PKC_STATUS {
            PKC_STATUS(0)
        }
    }
    impl core::fmt::Debug for PKC_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_STATUS")
                .field("ACTIV", &self.ACTIV())
                .field("CARRY", &self.CARRY())
                .field("ZERO", &self.ZERO())
                .field("GOANY", &self.GOANY())
                .field("LOCKED", &self.LOCKED())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_STATUS {{ ACTIV: {=bool:?}, CARRY: {=bool:?}, ZERO: {=bool:?}, GOANY: {=bool:?}, LOCKED: {=u8:?} }}" , self . ACTIV () , self . CARRY () , self . ZERO () , self . GOANY () , self . LOCKED ())
        }
    }
    #[doc = "Universal pointer length"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_ULEN(pub u32);
    impl PKC_ULEN {
        #[must_use]
        #[inline(always)]
        pub const fn LEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PKC_ULEN {
        #[inline(always)]
        fn default() -> PKC_ULEN {
            PKC_ULEN(0)
        }
    }
    impl core::fmt::Debug for PKC_ULEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_ULEN")
                .field("LEN", &self.LEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_ULEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PKC_ULEN {{ LEN: {=u8:?} }}", self.LEN())
        }
    }
    #[doc = "PKC version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_VERSION(pub u32);
    impl PKC_VERSION {
        #[must_use]
        #[inline(always)]
        pub const fn MULSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MULSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCAVAIL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCAVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UPAVAIL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UPAVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UPCACHEAVAIL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UPCACHEAVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GF2AVAIL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GF2AVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PARAMNUM(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PARAMNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBX0AVAIL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBX0AVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBX1AVAIL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBX1AVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBX2AVAIL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBX2AVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBX3AVAIL(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBX3AVAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCRECONF_SIZE(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MCRECONF_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for PKC_VERSION {
        #[inline(always)]
        fn default() -> PKC_VERSION {
            PKC_VERSION(0)
        }
    }
    impl core::fmt::Debug for PKC_VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_VERSION")
                .field("MULSIZE", &self.MULSIZE())
                .field("MCAVAIL", &self.MCAVAIL())
                .field("UPAVAIL", &self.UPAVAIL())
                .field("UPCACHEAVAIL", &self.UPCACHEAVAIL())
                .field("GF2AVAIL", &self.GF2AVAIL())
                .field("PARAMNUM", &self.PARAMNUM())
                .field("SBX0AVAIL", &self.SBX0AVAIL())
                .field("SBX1AVAIL", &self.SBX1AVAIL())
                .field("SBX2AVAIL", &self.SBX2AVAIL())
                .field("SBX3AVAIL", &self.SBX3AVAIL())
                .field("MCRECONF_SIZE", &self.MCRECONF_SIZE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_VERSION {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PKC_VERSION {{ MULSIZE: {=u8:?}, MCAVAIL: {=bool:?}, UPAVAIL: {=bool:?}, UPCACHEAVAIL: {=bool:?}, GF2AVAIL: {=bool:?}, PARAMNUM: {=u8:?}, SBX0AVAIL: {=bool:?}, SBX1AVAIL: {=bool:?}, SBX2AVAIL: {=bool:?}, SBX3AVAIL: {=bool:?}, MCRECONF_SIZE: {=u8:?} }}" , self . MULSIZE () , self . MCAVAIL () , self . UPAVAIL () , self . UPCACHEAVAIL () , self . GF2AVAIL () , self . PARAMNUM () , self . SBX0AVAIL () , self . SBX1AVAIL () , self . SBX2AVAIL () , self . SBX3AVAIL () , self . MCRECONF_SIZE ())
        }
    }
    #[doc = "X+Y pointer register, parameter set 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_XYPTR1(pub u32);
    impl PKC_XYPTR1 {
        #[must_use]
        #[inline(always)]
        pub const fn XPTR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_XPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn YPTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_YPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_XYPTR1 {
        #[inline(always)]
        fn default() -> PKC_XYPTR1 {
            PKC_XYPTR1(0)
        }
    }
    impl core::fmt::Debug for PKC_XYPTR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_XYPTR1")
                .field("XPTR", &self.XPTR())
                .field("YPTR", &self.YPTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_XYPTR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_XYPTR1 {{ XPTR: {=u16:?}, YPTR: {=u16:?} }}",
                self.XPTR(),
                self.YPTR()
            )
        }
    }
    #[doc = "X+Y pointer register, parameter set 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_XYPTR2(pub u32);
    impl PKC_XYPTR2 {
        #[must_use]
        #[inline(always)]
        pub const fn XPTR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_XPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn YPTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_YPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_XYPTR2 {
        #[inline(always)]
        fn default() -> PKC_XYPTR2 {
            PKC_XYPTR2(0)
        }
    }
    impl core::fmt::Debug for PKC_XYPTR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_XYPTR2")
                .field("XPTR", &self.XPTR())
                .field("YPTR", &self.YPTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_XYPTR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_XYPTR2 {{ XPTR: {=u16:?}, YPTR: {=u16:?} }}",
                self.XPTR(),
                self.YPTR()
            )
        }
    }
    #[doc = "Z+R pointer register, parameter set 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_ZRPTR1(pub u32);
    impl PKC_ZRPTR1 {
        #[must_use]
        #[inline(always)]
        pub const fn ZPTR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ZPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_ZRPTR1 {
        #[inline(always)]
        fn default() -> PKC_ZRPTR1 {
            PKC_ZRPTR1(0)
        }
    }
    impl core::fmt::Debug for PKC_ZRPTR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_ZRPTR1")
                .field("ZPTR", &self.ZPTR())
                .field("RPTR", &self.RPTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_ZRPTR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_ZRPTR1 {{ ZPTR: {=u16:?}, RPTR: {=u16:?} }}",
                self.ZPTR(),
                self.RPTR()
            )
        }
    }
    #[doc = "Z+R pointer register, parameter set 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKC_ZRPTR2(pub u32);
    impl PKC_ZRPTR2 {
        #[must_use]
        #[inline(always)]
        pub const fn ZPT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ZPT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RPTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKC_ZRPTR2 {
        #[inline(always)]
        fn default() -> PKC_ZRPTR2 {
            PKC_ZRPTR2(0)
        }
    }
    impl core::fmt::Debug for PKC_ZRPTR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKC_ZRPTR2")
                .field("ZPT", &self.ZPT())
                .field("RPTR", &self.RPTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKC_ZRPTR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PKC_ZRPTR2 {{ ZPT: {=u16:?}, RPTR: {=u16:?} }}",
                self.ZPT(),
                self.RPTR()
            )
        }
    }
}
