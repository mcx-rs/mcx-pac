#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OTPC {
    ptr: *mut u8,
}
unsafe impl Send for OTPC {}
unsafe impl Sync for OTPC {}
impl OTPC {
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
    pub const fn SR(self) -> crate::common::Reg<regs::SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn RWC(self) -> crate::common::Reg<regs::RWC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn RLC(self) -> crate::common::Reg<regs::RLC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn PCR(self) -> crate::common::Reg<regs::PCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn WDATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn RDATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMING1(self) -> crate::common::Reg<regs::TIMING1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMING2(self) -> crate::common::Reg<regs::TIMING2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn SECURE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn SECURE_INV(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn DBG_KEY(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn MISC_CFG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn PHANTOM_CFG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEX_CFG0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEX_CFG1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LOCK(pub u32);
    impl LOCK {
        #[must_use]
        #[inline(always)]
        pub const fn NXP_PART_CFG_LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NXP_PART_CFG_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NXP_EXT_LOCK(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NXP_EXT_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BOOT_CFG_LOCK(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BOOT_CFG_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRINCE_CFG_LOCK(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PRINCE_CFG_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSCAA_KEY_LOCK(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OSCAA_KEY_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CUST_LOCK0(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CUST_LOCK0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CUST_LOCK1(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CUST_LOCK1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CUST_LOCK2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CUST_LOCK2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CUST_LOCK3(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CUST_LOCK3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
        }
    }
    impl Default for LOCK {
        #[inline(always)]
        fn default() -> LOCK {
            LOCK(0)
        }
    }
    impl core::fmt::Debug for LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LOCK")
                .field("NXP_PART_CFG_LOCK", &self.NXP_PART_CFG_LOCK())
                .field("NXP_EXT_LOCK", &self.NXP_EXT_LOCK())
                .field("BOOT_CFG_LOCK", &self.BOOT_CFG_LOCK())
                .field("PRINCE_CFG_LOCK", &self.PRINCE_CFG_LOCK())
                .field("OSCAA_KEY_LOCK", &self.OSCAA_KEY_LOCK())
                .field("CUST_LOCK0", &self.CUST_LOCK0())
                .field("CUST_LOCK1", &self.CUST_LOCK1())
                .field("CUST_LOCK2", &self.CUST_LOCK2())
                .field("CUST_LOCK3", &self.CUST_LOCK3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LOCK {{ NXP_PART_CFG_LOCK: {=u8:?}, NXP_EXT_LOCK: {=u8:?}, BOOT_CFG_LOCK: {=u8:?}, PRINCE_CFG_LOCK: {=u8:?}, OSCAA_KEY_LOCK: {=u8:?}, CUST_LOCK0: {=u8:?}, CUST_LOCK1: {=u8:?}, CUST_LOCK2: {=u8:?}, CUST_LOCK3: {=u8:?} }}" , self . NXP_PART_CFG_LOCK () , self . NXP_EXT_LOCK () , self . BOOT_CFG_LOCK () , self . PRINCE_CFG_LOCK () , self . OSCAA_KEY_LOCK () , self . CUST_LOCK0 () , self . CUST_LOCK1 () , self . CUST_LOCK2 () , self . CUST_LOCK3 ())
        }
    }
    #[doc = "Parameters"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn NUM_FUSE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_NUM_FUSE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
                .field("NUM_FUSE", &self.NUM_FUSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PARAM {{ NUM_FUSE: {=u16:?} }}", self.NUM_FUSE())
        }
    }
    #[doc = "Power Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCR(pub u32);
    impl PCR {
        #[must_use]
        #[inline(always)]
        pub const fn HVREQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HVREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LVREQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PDREQ(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PDREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for PCR {
        #[inline(always)]
        fn default() -> PCR {
            PCR(0)
        }
    }
    impl core::fmt::Debug for PCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCR")
                .field("HVREQ", &self.HVREQ())
                .field("LVREQ", &self.LVREQ())
                .field("PDREQ", &self.PDREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PCR {{ HVREQ: {=bool:?}, LVREQ: {=bool:?}, PDREQ: {=bool:?} }}",
                self.HVREQ(),
                self.LVREQ(),
                self.PDREQ()
            )
        }
    }
    #[doc = "Reload Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RLC(pub u32);
    impl RLC {
        #[must_use]
        #[inline(always)]
        pub const fn RELOAD_SHADOWS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RELOAD_SHADOWS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RLC {
        #[inline(always)]
        fn default() -> RLC {
            RLC(0)
        }
    }
    impl core::fmt::Debug for RLC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RLC")
                .field("RELOAD_SHADOWS", &self.RELOAD_SHADOWS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RLC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RLC {{ RELOAD_SHADOWS: {=bool:?} }}",
                self.RELOAD_SHADOWS()
            )
        }
    }
    #[doc = "Read and Write Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RWC(pub u32);
    impl RWC {
        #[must_use]
        #[inline(always)]
        pub const fn ADDR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_ALL1S(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_ALL1S(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn READ_EFUSE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_READ_EFUSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn READ_UPDATE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_READ_UPDATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_UNLOCK(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_WR_UNLOCK(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for RWC {
        #[inline(always)]
        fn default() -> RWC {
            RWC(0)
        }
    }
    impl core::fmt::Debug for RWC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RWC")
                .field("ADDR", &self.ADDR())
                .field("WR_ALL1S", &self.WR_ALL1S())
                .field("READ_EFUSE", &self.READ_EFUSE())
                .field("READ_UPDATE", &self.READ_UPDATE())
                .field("WR_UNLOCK", &self.WR_UNLOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RWC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RWC {{ ADDR: {=u8:?}, WR_ALL1S: {=bool:?}, READ_EFUSE: {=bool:?}, READ_UPDATE: {=bool:?}, WR_UNLOCK: {=u16:?} }}" , self . ADDR () , self . WR_ALL1S () , self . READ_EFUSE () , self . READ_UPDATE () , self . WR_UNLOCK ())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR(pub u32);
    impl SR {
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERROR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ECC_SF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ECC_SF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ECC_DF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ECC_DF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRI_F(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRI_F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RD_FUSE_LOCK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RD_FUSE_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_FUSE_LOCK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_FUSE_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RD_REG_LOCK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RD_REG_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_REG_LOCK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_REG_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_REG_BUSY(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_REG_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_POWER_OFF(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_POWER_OFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FSM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FSM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLC(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADC(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IRC(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FSC(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("BUSY", &self.BUSY())
                .field("ERROR", &self.ERROR())
                .field("ECC_SF", &self.ECC_SF())
                .field("ECC_DF", &self.ECC_DF())
                .field("TRI_F", &self.TRI_F())
                .field("RD_FUSE_LOCK", &self.RD_FUSE_LOCK())
                .field("WR_FUSE_LOCK", &self.WR_FUSE_LOCK())
                .field("RD_REG_LOCK", &self.RD_REG_LOCK())
                .field("WR_REG_LOCK", &self.WR_REG_LOCK())
                .field("WR_REG_BUSY", &self.WR_REG_BUSY())
                .field("WR_POWER_OFF", &self.WR_POWER_OFF())
                .field("FSM", &self.FSM())
                .field("FLC", &self.FLC())
                .field("ADC", &self.ADC())
                .field("IRC", &self.IRC())
                .field("FSC", &self.FSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SR {{ BUSY: {=bool:?}, ERROR: {=bool:?}, ECC_SF: {=bool:?}, ECC_DF: {=bool:?}, TRI_F: {=bool:?}, RD_FUSE_LOCK: {=bool:?}, WR_FUSE_LOCK: {=bool:?}, RD_REG_LOCK: {=bool:?}, WR_REG_LOCK: {=bool:?}, WR_REG_BUSY: {=bool:?}, WR_POWER_OFF: {=bool:?}, FSM: {=bool:?}, FLC: {=bool:?}, ADC: {=bool:?}, IRC: {=bool:?}, FSC: {=bool:?} }}" , self . BUSY () , self . ERROR () , self . ECC_SF () , self . ECC_DF () , self . TRI_F () , self . RD_FUSE_LOCK () , self . WR_FUSE_LOCK () , self . RD_REG_LOCK () , self . WR_REG_LOCK () , self . WR_REG_BUSY () , self . WR_POWER_OFF () , self . FSM () , self . FLC () , self . ADC () , self . IRC () , self . FSC ())
        }
    }
    #[doc = "Timing1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMING1(pub u32);
    impl TIMING1 {
        #[must_use]
        #[inline(always)]
        pub const fn TADDR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRELAX(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRELAX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPD(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TIMING1 {
        #[inline(always)]
        fn default() -> TIMING1 {
            TIMING1(0)
        }
    }
    impl core::fmt::Debug for TIMING1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMING1")
                .field("TADDR", &self.TADDR())
                .field("TRELAX", &self.TRELAX())
                .field("TRD", &self.TRD())
                .field("TPS", &self.TPS())
                .field("TPD", &self.TPD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMING1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TIMING1 {{ TADDR: {=u8:?}, TRELAX: {=u8:?}, TRD: {=u8:?}, TPS: {=u8:?}, TPD: {=u8:?} }}" , self . TADDR () , self . TRELAX () , self . TRD () , self . TPS () , self . TPD ())
        }
    }
    #[doc = "Timing2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMING2(pub u32);
    impl TIMING2 {
        #[must_use]
        #[inline(always)]
        pub const fn TPGM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TPGM(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for TIMING2 {
        #[inline(always)]
        fn default() -> TIMING2 {
            TIMING2(0)
        }
    }
    impl core::fmt::Debug for TIMING2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMING2")
                .field("TPGM", &self.TPGM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMING2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TIMING2 {{ TPGM: {=u16:?} }}", self.TPGM())
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[must_use]
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAJOR(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
                self.FEATURE(),
                self.MINOR(),
                self.MAJOR()
            )
        }
    }
}
