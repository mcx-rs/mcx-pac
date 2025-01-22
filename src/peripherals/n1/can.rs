#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN {
    ptr: *mut u8,
}
unsafe impl Send for CAN {}
unsafe impl Sync for CAN {}
impl CAN {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL1(self) -> crate::common::Reg<regs::CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER(self) -> crate::common::Reg<regs::TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn RXMGMASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn RX14MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn RX15MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn ECR(self) -> crate::common::Reg<regs::ECR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn ESR1(self) -> crate::common::Reg<regs::ESR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn IMASK1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn IFLAG1(self) -> crate::common::Reg<regs::IFLAG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<regs::CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ESR2(self) -> crate::common::Reg<regs::ESR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn CRCR(self) -> crate::common::Reg<regs::CRCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn RXFGMASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn RXFIR(self) -> crate::common::Reg<regs::RXFIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn CBT(self) -> crate::common::Reg<regs::CBT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn MB(self, n: usize) -> MB {
        assert!(n < 32usize);
        unsafe { MB::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn MB_16B(self, n: usize) -> MB_16B {
        assert!(n < 21usize);
        unsafe { MB_16B::from_ptr(self.ptr.add(0x80usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn MB_32B(self, n: usize) -> MB_32B {
        assert!(n < 12usize);
        unsafe { MB_32B::from_ptr(self.ptr.add(0x80usize + n * 40usize) as _) }
    }
    #[inline(always)]
    pub const fn MB_64B(self, n: usize) -> MB_64B {
        assert!(n < 7usize);
        unsafe { MB_64B::from_ptr(self.ptr.add(0x80usize + n * 72usize) as _) }
    }
    #[inline(always)]
    pub const fn MB_8B(self, n: usize) -> MB_8B {
        assert!(n < 32usize);
        unsafe { MB_8B::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn RXIMR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL1_PN(self) -> crate::common::Reg<regs::CTRL1_PN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2_PN(self) -> crate::common::Reg<regs::CTRL2_PN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b04usize) as _) }
    }
    #[inline(always)]
    pub const fn WU_MTC(self) -> crate::common::Reg<regs::WU_MTC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b08usize) as _) }
    }
    #[inline(always)]
    pub const fn FLT_ID1(self) -> crate::common::Reg<regs::FLT_ID1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b0cusize) as _) }
    }
    #[inline(always)]
    pub const fn FLT_DLC(self) -> crate::common::Reg<regs::FLT_DLC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b10usize) as _) }
    }
    #[inline(always)]
    pub const fn PL1_LO(self) -> crate::common::Reg<regs::PL1_LO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b14usize) as _) }
    }
    #[inline(always)]
    pub const fn PL1_HI(self) -> crate::common::Reg<regs::PL1_HI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b18usize) as _) }
    }
    #[inline(always)]
    pub const fn FLT_ID2_IDMASK(
        self,
    ) -> crate::common::Reg<regs::FLT_ID2_IDMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PL2_PLMASK_LO(self) -> crate::common::Reg<regs::PL2_PLMASK_LO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b20usize) as _) }
    }
    #[inline(always)]
    pub const fn PL2_PLMASK_HI(self) -> crate::common::Reg<regs::PL2_PLMASK_HI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b24usize) as _) }
    }
    #[inline(always)]
    pub const fn WMB(self, n: usize) -> WMB {
        assert!(n < 4usize);
        unsafe { WMB::from_ptr(self.ptr.add(0x0b40usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn EPRS(self) -> crate::common::Reg<regs::EPRS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bf0usize) as _) }
    }
    #[inline(always)]
    pub const fn ENCBT(self) -> crate::common::Reg<regs::ENCBT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bf4usize) as _) }
    }
    #[inline(always)]
    pub const fn EDCBT(self) -> crate::common::Reg<regs::EDCBT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bf8usize) as _) }
    }
    #[inline(always)]
    pub const fn ETDC(self) -> crate::common::Reg<regs::ETDC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0bfcusize) as _) }
    }
    #[inline(always)]
    pub const fn FDCTRL(self) -> crate::common::Reg<regs::FDCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn FDCBT(self) -> crate::common::Reg<regs::FDCBT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[inline(always)]
    pub const fn FDCRC(self) -> crate::common::Reg<regs::FDCRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c08usize) as _) }
    }
    #[inline(always)]
    pub const fn ERFCR(self) -> crate::common::Reg<regs::ERFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c0cusize) as _) }
    }
    #[inline(always)]
    pub const fn ERFIER(self) -> crate::common::Reg<regs::ERFIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c10usize) as _) }
    }
    #[inline(always)]
    pub const fn ERFSR(self) -> crate::common::Reg<regs::ERFSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c14usize) as _) }
    }
    #[inline(always)]
    pub const fn ERFFEL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3000usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MB {
    ptr: *mut u8,
}
unsafe impl Send for MB {}
unsafe impl Sync for MB {}
impl MB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::MB_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::MB_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD0(self) -> crate::common::Reg<regs::MB_WORD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD1(self) -> crate::common::Reg<regs::MB_WORD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MB_16B {
    ptr: *mut u8,
}
unsafe impl Send for MB_16B {}
unsafe impl Sync for MB_16B {}
impl MB_16B {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::MB_16B_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::MB_16B_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MB_32B {
    ptr: *mut u8,
}
unsafe impl Send for MB_32B {}
unsafe impl Sync for MB_32B {}
impl MB_32B {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::MB_32B_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::MB_32B_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MB_64B {
    ptr: *mut u8,
}
unsafe impl Send for MB_64B {}
unsafe impl Sync for MB_64B {}
impl MB_64B {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::MB_64B_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::MB_64B_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MB_8B {
    ptr: *mut u8,
}
unsafe impl Send for MB_8B {}
unsafe impl Sync for MB_8B {}
impl MB_8B {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::MB_8B_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::MB_8B_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn WORD(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WMB {
    ptr: *mut u8,
}
unsafe impl Send for WMB {}
unsafe impl Sync for WMB {}
impl WMB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<regs::WMB_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::WMB_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn D03(self) -> crate::common::Reg<regs::WMB_D03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn D47(self) -> crate::common::Reg<regs::WMB_D47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "CAN Bit Timing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CBT(pub u32);
    impl CBT {
        #[inline(always)]
        pub const fn EPSEG2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPSEG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn EPSEG1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPSEG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[inline(always)]
        pub const fn EPROPSEG(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_EPROPSEG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
        #[inline(always)]
        pub const fn ERJW(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERJW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn EPRESDIV(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_EPRESDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
        }
        #[inline(always)]
        pub const fn BTF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CBT {
        #[inline(always)]
        fn default() -> CBT {
            CBT(0)
        }
    }
    impl core::fmt::Debug for CBT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CBT")
                .field("EPSEG2", &self.EPSEG2())
                .field("EPSEG1", &self.EPSEG1())
                .field("EPROPSEG", &self.EPROPSEG())
                .field("ERJW", &self.ERJW())
                .field("EPRESDIV", &self.EPRESDIV())
                .field("BTF", &self.BTF())
                .finish()
        }
    }
    #[doc = "Cyclic Redundancy Check"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CRCR(pub u32);
    impl CRCR {
        #[inline(always)]
        pub const fn TXCRC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TXCRC(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[inline(always)]
        pub const fn MBCRC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MBCRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for CRCR {
        #[inline(always)]
        fn default() -> CRCR {
            CRCR(0)
        }
    }
    impl core::fmt::Debug for CRCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CRCR")
                .field("TXCRC", &self.TXCRC())
                .field("MBCRC", &self.MBCRC())
                .finish()
        }
    }
    #[doc = "Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL1(pub u32);
    impl CTRL1 {
        #[inline(always)]
        pub const fn PROPSEG(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PROPSEG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn LOM(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn LBUF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LBUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TSYN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSYN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn BOFFREC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOFFREC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SMP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RWRNMSK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RWRNMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TWRNMSK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TWRNMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn LPB(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ERRMSK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERRMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn BOFFMSK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOFFMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PSEG2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PSEG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn PSEG1(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PSEG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
        #[inline(always)]
        pub const fn RJW(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RJW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn PRESDIV(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRESDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for CTRL1 {
        #[inline(always)]
        fn default() -> CTRL1 {
            CTRL1(0)
        }
    }
    impl core::fmt::Debug for CTRL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL1")
                .field("PROPSEG", &self.PROPSEG())
                .field("LOM", &self.LOM())
                .field("LBUF", &self.LBUF())
                .field("TSYN", &self.TSYN())
                .field("BOFFREC", &self.BOFFREC())
                .field("SMP", &self.SMP())
                .field("RWRNMSK", &self.RWRNMSK())
                .field("TWRNMSK", &self.TWRNMSK())
                .field("LPB", &self.LPB())
                .field("ERRMSK", &self.ERRMSK())
                .field("BOFFMSK", &self.BOFFMSK())
                .field("PSEG2", &self.PSEG2())
                .field("PSEG1", &self.PSEG1())
                .field("RJW", &self.RJW())
                .field("PRESDIV", &self.PRESDIV())
                .finish()
        }
    }
    #[doc = "Pretended Networking Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL1_PN(pub u32);
    impl CTRL1_PN {
        #[inline(always)]
        pub const fn FCS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn IDFS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IDFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PLFS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn NMATCH(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_NMATCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn WUMF_MSK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUMF_MSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WTOF_MSK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WTOF_MSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for CTRL1_PN {
        #[inline(always)]
        fn default() -> CTRL1_PN {
            CTRL1_PN(0)
        }
    }
    impl core::fmt::Debug for CTRL1_PN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL1_PN")
                .field("FCS", &self.FCS())
                .field("IDFS", &self.IDFS())
                .field("PLFS", &self.PLFS())
                .field("NMATCH", &self.NMATCH())
                .field("WUMF_MSK", &self.WUMF_MSK())
                .field("WTOF_MSK", &self.WTOF_MSK())
                .finish()
        }
    }
    #[doc = "Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL2(pub u32);
    impl CTRL2 {
        #[inline(always)]
        pub const fn EDFLTDIS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDFLTDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ISOCANFDEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISOCANFDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn BTE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PREXCEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PREXCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn EACEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EACEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn RRS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn MRP(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MRP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn TASD(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TASD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[inline(always)]
        pub const fn RFFN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFFN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn BOFFDONEMSK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOFFDONEMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ERRMSK_FAST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERRMSK_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("EDFLTDIS", &self.EDFLTDIS())
                .field("ISOCANFDEN", &self.ISOCANFDEN())
                .field("BTE", &self.BTE())
                .field("PREXCEN", &self.PREXCEN())
                .field("EACEN", &self.EACEN())
                .field("RRS", &self.RRS())
                .field("MRP", &self.MRP())
                .field("TASD", &self.TASD())
                .field("RFFN", &self.RFFN())
                .field("BOFFDONEMSK", &self.BOFFDONEMSK())
                .field("ERRMSK_FAST", &self.ERRMSK_FAST())
                .finish()
        }
    }
    #[doc = "Pretended Networking Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL2_PN(pub u32);
    impl CTRL2_PN {
        #[inline(always)]
        pub const fn MATCHTO(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHTO(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CTRL2_PN {
        #[inline(always)]
        fn default() -> CTRL2_PN {
            CTRL2_PN(0)
        }
    }
    impl core::fmt::Debug for CTRL2_PN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL2_PN")
                .field("MATCHTO", &self.MATCHTO())
                .finish()
        }
    }
    #[doc = "Error Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ECR(pub u32);
    impl ECR {
        #[inline(always)]
        pub const fn TXERRCNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXERRCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RXERRCNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXERRCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn TXERRCNT_FAST(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXERRCNT_FAST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn RXERRCNT_FAST(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXERRCNT_FAST(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for ECR {
        #[inline(always)]
        fn default() -> ECR {
            ECR(0)
        }
    }
    impl core::fmt::Debug for ECR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ECR")
                .field("TXERRCNT", &self.TXERRCNT())
                .field("RXERRCNT", &self.RXERRCNT())
                .field("TXERRCNT_FAST", &self.TXERRCNT_FAST())
                .field("RXERRCNT_FAST", &self.RXERRCNT_FAST())
                .finish()
        }
    }
    #[doc = "Enhanced Data Phase CAN Bit Timing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EDCBT(pub u32);
    impl EDCBT {
        #[inline(always)]
        pub const fn DTSEG1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DTSEG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn DTSEG2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DTSEG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn DRJW(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DRJW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
    }
    impl Default for EDCBT {
        #[inline(always)]
        fn default() -> EDCBT {
            EDCBT(0)
        }
    }
    impl core::fmt::Debug for EDCBT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EDCBT")
                .field("DTSEG1", &self.DTSEG1())
                .field("DTSEG2", &self.DTSEG2())
                .field("DRJW", &self.DRJW())
                .finish()
        }
    }
    #[doc = "Enhanced Nominal CAN Bit Timing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENCBT(pub u32);
    impl ENCBT {
        #[inline(always)]
        pub const fn NTSEG1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_NTSEG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NTSEG2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NTSEG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
        }
        #[inline(always)]
        pub const fn NRJW(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NRJW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
    }
    impl Default for ENCBT {
        #[inline(always)]
        fn default() -> ENCBT {
            ENCBT(0)
        }
    }
    impl core::fmt::Debug for ENCBT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENCBT")
                .field("NTSEG1", &self.NTSEG1())
                .field("NTSEG2", &self.NTSEG2())
                .field("NRJW", &self.NRJW())
                .finish()
        }
    }
    #[doc = "Enhanced CAN Bit Timing Prescalers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EPRS(pub u32);
    impl EPRS {
        #[inline(always)]
        pub const fn ENPRESDIV(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ENPRESDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[inline(always)]
        pub const fn EDPRESDIV(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_EDPRESDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for EPRS {
        #[inline(always)]
        fn default() -> EPRS {
            EPRS(0)
        }
    }
    impl core::fmt::Debug for EPRS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EPRS")
                .field("ENPRESDIV", &self.ENPRESDIV())
                .field("EDPRESDIV", &self.EDPRESDIV())
                .finish()
        }
    }
    #[doc = "Enhanced RX FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERFCR(pub u32);
    impl ERFCR {
        #[inline(always)]
        pub const fn ERFWM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERFWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn NFE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NFE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn NEXIF(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NEXIF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[inline(always)]
        pub const fn DMALW(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMALW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[inline(always)]
        pub const fn ERFEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ERFCR {
        #[inline(always)]
        fn default() -> ERFCR {
            ERFCR(0)
        }
    }
    impl core::fmt::Debug for ERFCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERFCR")
                .field("ERFWM", &self.ERFWM())
                .field("NFE", &self.NFE())
                .field("NEXIF", &self.NEXIF())
                .field("DMALW", &self.DMALW())
                .field("ERFEN", &self.ERFEN())
                .finish()
        }
    }
    #[doc = "Enhanced RX FIFO Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERFIER(pub u32);
    impl ERFIER {
        #[inline(always)]
        pub const fn ERFDAIE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFDAIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ERFWMIIE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFWMIIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn ERFOVFIE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFOVFIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ERFUFWIE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFUFWIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ERFIER {
        #[inline(always)]
        fn default() -> ERFIER {
            ERFIER(0)
        }
    }
    impl core::fmt::Debug for ERFIER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERFIER")
                .field("ERFDAIE", &self.ERFDAIE())
                .field("ERFWMIIE", &self.ERFWMIIE())
                .field("ERFOVFIE", &self.ERFOVFIE())
                .field("ERFUFWIE", &self.ERFUFWIE())
                .finish()
        }
    }
    #[doc = "Enhanced RX FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERFSR(pub u32);
    impl ERFSR {
        #[inline(always)]
        pub const fn ERFEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERFEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn ERFF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ERFE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ERFCLR(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFCLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ERFDA(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFDA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ERFWMI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFWMI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn ERFOVF(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFOVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ERFUFW(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERFUFW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ERFSR {
        #[inline(always)]
        fn default() -> ERFSR {
            ERFSR(0)
        }
    }
    impl core::fmt::Debug for ERFSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERFSR")
                .field("ERFEL", &self.ERFEL())
                .field("ERFF", &self.ERFF())
                .field("ERFE", &self.ERFE())
                .field("ERFCLR", &self.ERFCLR())
                .field("ERFDA", &self.ERFDA())
                .field("ERFWMI", &self.ERFWMI())
                .field("ERFOVF", &self.ERFOVF())
                .field("ERFUFW", &self.ERFUFW())
                .finish()
        }
    }
    #[doc = "Error and Status 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ESR1(pub u32);
    impl ESR1 {
        #[inline(always)]
        pub const fn WAKINT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ERRINT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERRINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn BOFFINT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOFFINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RX(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FLTCONF(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLTCONF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn TX(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn IDLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RXWRN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXWRN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TXWRN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXWRN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn STFERR(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STFERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FRMERR(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRMERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CRCERR(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ACKERR(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACKERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn BIT0ERR(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIT0ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn BIT1ERR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIT1ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn RWRNINT(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RWRNINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TWRNINT(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TWRNINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn SYNCH(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYNCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn BOFFDONEINT(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOFFDONEINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ERRINT_FAST(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERRINT_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn ERROVR(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROVR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn STFERR_FAST(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STFERR_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn FRMERR_FAST(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRMERR_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn CRCERR_FAST(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRCERR_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn BIT0ERR_FAST(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIT0ERR_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn BIT1ERR_FAST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIT1ERR_FAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ESR1 {
        #[inline(always)]
        fn default() -> ESR1 {
            ESR1(0)
        }
    }
    impl core::fmt::Debug for ESR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ESR1")
                .field("WAKINT", &self.WAKINT())
                .field("ERRINT", &self.ERRINT())
                .field("BOFFINT", &self.BOFFINT())
                .field("RX", &self.RX())
                .field("FLTCONF", &self.FLTCONF())
                .field("TX", &self.TX())
                .field("IDLE", &self.IDLE())
                .field("RXWRN", &self.RXWRN())
                .field("TXWRN", &self.TXWRN())
                .field("STFERR", &self.STFERR())
                .field("FRMERR", &self.FRMERR())
                .field("CRCERR", &self.CRCERR())
                .field("ACKERR", &self.ACKERR())
                .field("BIT0ERR", &self.BIT0ERR())
                .field("BIT1ERR", &self.BIT1ERR())
                .field("RWRNINT", &self.RWRNINT())
                .field("TWRNINT", &self.TWRNINT())
                .field("SYNCH", &self.SYNCH())
                .field("BOFFDONEINT", &self.BOFFDONEINT())
                .field("ERRINT_FAST", &self.ERRINT_FAST())
                .field("ERROVR", &self.ERROVR())
                .field("STFERR_FAST", &self.STFERR_FAST())
                .field("FRMERR_FAST", &self.FRMERR_FAST())
                .field("CRCERR_FAST", &self.CRCERR_FAST())
                .field("BIT0ERR_FAST", &self.BIT0ERR_FAST())
                .field("BIT1ERR_FAST", &self.BIT1ERR_FAST())
                .finish()
        }
    }
    #[doc = "Error and Status 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ESR2(pub u32);
    impl ESR2 {
        #[inline(always)]
        pub const fn IMB(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn VPS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VPS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn LPTM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LPTM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for ESR2 {
        #[inline(always)]
        fn default() -> ESR2 {
            ESR2(0)
        }
    }
    impl core::fmt::Debug for ESR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ESR2")
                .field("IMB", &self.IMB())
                .field("VPS", &self.VPS())
                .field("LPTM", &self.LPTM())
                .finish()
        }
    }
    #[doc = "Enhanced Transceiver Delay Compensation"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ETDC(pub u32);
    impl ETDC {
        #[inline(always)]
        pub const fn ETDCVAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETDCVAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn ETDCFAIL(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ETDCFAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ETDCOFF(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETDCOFF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[inline(always)]
        pub const fn TDMDIS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDMDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ETDCEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ETDCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ETDC {
        #[inline(always)]
        fn default() -> ETDC {
            ETDC(0)
        }
    }
    impl core::fmt::Debug for ETDC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ETDC")
                .field("ETDCVAL", &self.ETDCVAL())
                .field("ETDCFAIL", &self.ETDCFAIL())
                .field("ETDCOFF", &self.ETDCOFF())
                .field("TDMDIS", &self.TDMDIS())
                .field("ETDCEN", &self.ETDCEN())
                .finish()
        }
    }
    #[doc = "CAN FD Bit Timing"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDCBT(pub u32);
    impl FDCBT {
        #[inline(always)]
        pub const fn FPSEG2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FPSEG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn FPSEG1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FPSEG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[inline(always)]
        pub const fn FPROPSEG(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FPROPSEG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[inline(always)]
        pub const fn FRJW(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRJW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn FPRESDIV(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FPRESDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for FDCBT {
        #[inline(always)]
        fn default() -> FDCBT {
            FDCBT(0)
        }
    }
    impl core::fmt::Debug for FDCBT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDCBT")
                .field("FPSEG2", &self.FPSEG2())
                .field("FPSEG1", &self.FPSEG1())
                .field("FPROPSEG", &self.FPROPSEG())
                .field("FRJW", &self.FRJW())
                .field("FPRESDIV", &self.FPRESDIV())
                .finish()
        }
    }
    #[doc = "CAN FD CRC"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDCRC(pub u32);
    impl FDCRC {
        #[inline(always)]
        pub const fn FD_TXCRC(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FD_TXCRC(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn FD_MBCRC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FD_MBCRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for FDCRC {
        #[inline(always)]
        fn default() -> FDCRC {
            FDCRC(0)
        }
    }
    impl core::fmt::Debug for FDCRC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDCRC")
                .field("FD_TXCRC", &self.FD_TXCRC())
                .field("FD_MBCRC", &self.FD_MBCRC())
                .finish()
        }
    }
    #[doc = "CAN FD Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDCTRL(pub u32);
    impl FDCTRL {
        #[inline(always)]
        pub const fn TDCVAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TDCVAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn TDCOFF(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TDCOFF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn TDCFAIL(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDCFAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn TDCEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn MBDSR0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MBDSR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn FDRATE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FDRATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FDCTRL {
        #[inline(always)]
        fn default() -> FDCTRL {
            FDCTRL(0)
        }
    }
    impl core::fmt::Debug for FDCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDCTRL")
                .field("TDCVAL", &self.TDCVAL())
                .field("TDCOFF", &self.TDCOFF())
                .field("TDCFAIL", &self.TDCFAIL())
                .field("TDCEN", &self.TDCEN())
                .field("MBDSR0", &self.MBDSR0())
                .field("FDRATE", &self.FDRATE())
                .finish()
        }
    }
    #[doc = "Pretended Networking Data Length Code (DLC) Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLT_DLC(pub u32);
    impl FLT_DLC {
        #[inline(always)]
        pub const fn FLT_DLC_HI(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLT_DLC_HI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FLT_DLC_LO(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLT_DLC_LO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FLT_DLC {
        #[inline(always)]
        fn default() -> FLT_DLC {
            FLT_DLC(0)
        }
    }
    impl core::fmt::Debug for FLT_DLC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLT_DLC")
                .field("FLT_DLC_HI", &self.FLT_DLC_HI())
                .field("FLT_DLC_LO", &self.FLT_DLC_LO())
                .finish()
        }
    }
    #[doc = "Pretended Networking ID Filter 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLT_ID1(pub u32);
    impl FLT_ID1 {
        #[inline(always)]
        pub const fn FLT_ID1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FLT_ID1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn FLT_RTR(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLT_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn FLT_IDE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLT_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for FLT_ID1 {
        #[inline(always)]
        fn default() -> FLT_ID1 {
            FLT_ID1(0)
        }
    }
    impl core::fmt::Debug for FLT_ID1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLT_ID1")
                .field("FLT_ID1", &self.FLT_ID1())
                .field("FLT_RTR", &self.FLT_RTR())
                .field("FLT_IDE", &self.FLT_IDE())
                .finish()
        }
    }
    #[doc = "Pretended Networking ID Filter 2 or ID Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLT_ID2_IDMASK(pub u32);
    impl FLT_ID2_IDMASK {
        #[inline(always)]
        pub const fn FLT_ID2_IDMASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FLT_ID2_IDMASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn RTR_MSK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR_MSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn IDE_MSK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE_MSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for FLT_ID2_IDMASK {
        #[inline(always)]
        fn default() -> FLT_ID2_IDMASK {
            FLT_ID2_IDMASK(0)
        }
    }
    impl core::fmt::Debug for FLT_ID2_IDMASK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLT_ID2_IDMASK")
                .field("FLT_ID2_IDMASK", &self.FLT_ID2_IDMASK())
                .field("RTR_MSK", &self.RTR_MSK())
                .field("IDE_MSK", &self.IDE_MSK())
                .finish()
        }
    }
    #[doc = "Interrupt Flags 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IFLAG1(pub u32);
    impl IFLAG1 {
        #[inline(always)]
        pub const fn BUF0I(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUF0I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BUF4TO1I(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BUF4TO1I(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[inline(always)]
        pub const fn BUF5I(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUF5I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn BUF6I(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUF6I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn BUF7I(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUF7I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn BUF31TO8I(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_BUF31TO8I(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IFLAG1 {
        #[inline(always)]
        fn default() -> IFLAG1 {
            IFLAG1(0)
        }
    }
    impl core::fmt::Debug for IFLAG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IFLAG1")
                .field("BUF0I", &self.BUF0I())
                .field("BUF4TO1I", &self.BUF4TO1I())
                .field("BUF5I", &self.BUF5I())
                .field("BUF6I", &self.BUF6I())
                .field("BUF7I", &self.BUF7I())
                .field("BUF31TO8I", &self.BUF31TO8I())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 CS Register..Message Buffer 20 CS Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_16B_CS(pub u32);
    impl MB_16B_CS {
        #[inline(always)]
        pub const fn TIME_STAMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIME_STAMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ESI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn BRS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MB_16B_CS {
        #[inline(always)]
        fn default() -> MB_16B_CS {
            MB_16B_CS(0)
        }
    }
    impl core::fmt::Debug for MB_16B_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_16B_CS")
                .field("TIME_STAMP", &self.TIME_STAMP())
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .field("CODE", &self.CODE())
                .field("ESI", &self.ESI())
                .field("BRS", &self.BRS())
                .field("EDL", &self.EDL())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 ID Register..Message Buffer 20 ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_16B_ID(pub u32);
    impl MB_16B_ID {
        #[inline(always)]
        pub const fn EXT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EXT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STD(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[inline(always)]
        pub const fn PRIO(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for MB_16B_ID {
        #[inline(always)]
        fn default() -> MB_16B_ID {
            MB_16B_ID(0)
        }
    }
    impl core::fmt::Debug for MB_16B_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_16B_ID")
                .field("EXT", &self.EXT())
                .field("STD", &self.STD())
                .field("PRIO", &self.PRIO())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD_16B Register..Message Buffer 20 WORD_16B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_16B_WORD(pub u32);
    impl MB_16B_WORD {
        #[inline(always)]
        pub const fn DATA_BYTE_11(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_15(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_10(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_14(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_13(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_13(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_9(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_12(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_16B_WORD {
        #[inline(always)]
        fn default() -> MB_16B_WORD {
            MB_16B_WORD(0)
        }
    }
    impl core::fmt::Debug for MB_16B_WORD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_16B_WORD")
                .field("DATA_BYTE_11", &self.DATA_BYTE_11())
                .field("DATA_BYTE_15", &self.DATA_BYTE_15())
                .field("DATA_BYTE_3", &self.DATA_BYTE_3())
                .field("DATA_BYTE_7", &self.DATA_BYTE_7())
                .field("DATA_BYTE_10", &self.DATA_BYTE_10())
                .field("DATA_BYTE_14", &self.DATA_BYTE_14())
                .field("DATA_BYTE_2", &self.DATA_BYTE_2())
                .field("DATA_BYTE_6", &self.DATA_BYTE_6())
                .field("DATA_BYTE_1", &self.DATA_BYTE_1())
                .field("DATA_BYTE_13", &self.DATA_BYTE_13())
                .field("DATA_BYTE_5", &self.DATA_BYTE_5())
                .field("DATA_BYTE_9", &self.DATA_BYTE_9())
                .field("DATA_BYTE_0", &self.DATA_BYTE_0())
                .field("DATA_BYTE_12", &self.DATA_BYTE_12())
                .field("DATA_BYTE_4", &self.DATA_BYTE_4())
                .field("DATA_BYTE_8", &self.DATA_BYTE_8())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 CS Register..Message Buffer 11 CS Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_32B_CS(pub u32);
    impl MB_32B_CS {
        #[inline(always)]
        pub const fn TIME_STAMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIME_STAMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ESI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn BRS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MB_32B_CS {
        #[inline(always)]
        fn default() -> MB_32B_CS {
            MB_32B_CS(0)
        }
    }
    impl core::fmt::Debug for MB_32B_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_32B_CS")
                .field("TIME_STAMP", &self.TIME_STAMP())
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .field("CODE", &self.CODE())
                .field("ESI", &self.ESI())
                .field("BRS", &self.BRS())
                .field("EDL", &self.EDL())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 ID Register..Message Buffer 11 ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_32B_ID(pub u32);
    impl MB_32B_ID {
        #[inline(always)]
        pub const fn EXT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EXT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STD(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[inline(always)]
        pub const fn PRIO(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for MB_32B_ID {
        #[inline(always)]
        fn default() -> MB_32B_ID {
            MB_32B_ID(0)
        }
    }
    impl core::fmt::Debug for MB_32B_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_32B_ID")
                .field("EXT", &self.EXT())
                .field("STD", &self.STD())
                .field("PRIO", &self.PRIO())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD_32B Register..Message Buffer 11 WORD_32B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_32B_WORD(pub u32);
    impl MB_32B_WORD {
        #[inline(always)]
        pub const fn DATA_BYTE_11(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_15(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_19(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_19(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_23(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_23(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_27(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_27(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_31(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_31(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_10(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_14(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_18(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_18(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_22(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_22(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_26(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_26(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_30(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_30(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_13(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_13(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_17(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_17(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_21(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_21(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_25(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_25(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_29(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_29(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_9(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_12(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_16(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_20(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_20(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_28(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_32B_WORD {
        #[inline(always)]
        fn default() -> MB_32B_WORD {
            MB_32B_WORD(0)
        }
    }
    impl core::fmt::Debug for MB_32B_WORD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_32B_WORD")
                .field("DATA_BYTE_11", &self.DATA_BYTE_11())
                .field("DATA_BYTE_15", &self.DATA_BYTE_15())
                .field("DATA_BYTE_19", &self.DATA_BYTE_19())
                .field("DATA_BYTE_23", &self.DATA_BYTE_23())
                .field("DATA_BYTE_27", &self.DATA_BYTE_27())
                .field("DATA_BYTE_3", &self.DATA_BYTE_3())
                .field("DATA_BYTE_31", &self.DATA_BYTE_31())
                .field("DATA_BYTE_7", &self.DATA_BYTE_7())
                .field("DATA_BYTE_10", &self.DATA_BYTE_10())
                .field("DATA_BYTE_14", &self.DATA_BYTE_14())
                .field("DATA_BYTE_18", &self.DATA_BYTE_18())
                .field("DATA_BYTE_2", &self.DATA_BYTE_2())
                .field("DATA_BYTE_22", &self.DATA_BYTE_22())
                .field("DATA_BYTE_26", &self.DATA_BYTE_26())
                .field("DATA_BYTE_30", &self.DATA_BYTE_30())
                .field("DATA_BYTE_6", &self.DATA_BYTE_6())
                .field("DATA_BYTE_1", &self.DATA_BYTE_1())
                .field("DATA_BYTE_13", &self.DATA_BYTE_13())
                .field("DATA_BYTE_17", &self.DATA_BYTE_17())
                .field("DATA_BYTE_21", &self.DATA_BYTE_21())
                .field("DATA_BYTE_25", &self.DATA_BYTE_25())
                .field("DATA_BYTE_29", &self.DATA_BYTE_29())
                .field("DATA_BYTE_5", &self.DATA_BYTE_5())
                .field("DATA_BYTE_9", &self.DATA_BYTE_9())
                .field("DATA_BYTE_0", &self.DATA_BYTE_0())
                .field("DATA_BYTE_12", &self.DATA_BYTE_12())
                .field("DATA_BYTE_16", &self.DATA_BYTE_16())
                .field("DATA_BYTE_20", &self.DATA_BYTE_20())
                .field("DATA_BYTE_24", &self.DATA_BYTE_24())
                .field("DATA_BYTE_28", &self.DATA_BYTE_28())
                .field("DATA_BYTE_4", &self.DATA_BYTE_4())
                .field("DATA_BYTE_8", &self.DATA_BYTE_8())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 CS Register..Message Buffer 6 CS Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_64B_CS(pub u32);
    impl MB_64B_CS {
        #[inline(always)]
        pub const fn TIME_STAMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIME_STAMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ESI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn BRS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MB_64B_CS {
        #[inline(always)]
        fn default() -> MB_64B_CS {
            MB_64B_CS(0)
        }
    }
    impl core::fmt::Debug for MB_64B_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_64B_CS")
                .field("TIME_STAMP", &self.TIME_STAMP())
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .field("CODE", &self.CODE())
                .field("ESI", &self.ESI())
                .field("BRS", &self.BRS())
                .field("EDL", &self.EDL())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 ID Register..Message Buffer 6 ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_64B_ID(pub u32);
    impl MB_64B_ID {
        #[inline(always)]
        pub const fn EXT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EXT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STD(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[inline(always)]
        pub const fn PRIO(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for MB_64B_ID {
        #[inline(always)]
        fn default() -> MB_64B_ID {
            MB_64B_ID(0)
        }
    }
    impl core::fmt::Debug for MB_64B_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_64B_ID")
                .field("EXT", &self.EXT())
                .field("STD", &self.STD())
                .field("PRIO", &self.PRIO())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD_64B Register..Message Buffer 6 WORD_64B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_64B_WORD(pub u32);
    impl MB_64B_WORD {
        #[inline(always)]
        pub const fn DATA_BYTE_11(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_15(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_19(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_19(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_23(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_23(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_27(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_27(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_31(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_31(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_35(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_35(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_39(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_39(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_43(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_43(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_47(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_47(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_51(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_51(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_55(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_55(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_59(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_59(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_63(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_63(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_10(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_14(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_18(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_18(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_22(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_22(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_26(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_26(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_30(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_30(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_34(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_34(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_38(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_38(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_42(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_42(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_46(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_46(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_50(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_50(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_54(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_54(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_58(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_58(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_62(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_62(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_13(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_13(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_17(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_17(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_21(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_21(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_25(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_25(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_29(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_29(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_33(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_33(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_37(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_37(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_41(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_41(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_45(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_45(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_49(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_49(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_53(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_53(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_57(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_57(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_61(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_61(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_9(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_12(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_16(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_20(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_20(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_28(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_32(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_32(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_36(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_36(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_40(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_40(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_44(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_44(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_48(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_48(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_52(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_52(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_56(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_56(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_60(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_60(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_64B_WORD {
        #[inline(always)]
        fn default() -> MB_64B_WORD {
            MB_64B_WORD(0)
        }
    }
    impl core::fmt::Debug for MB_64B_WORD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_64B_WORD")
                .field("DATA_BYTE_11", &self.DATA_BYTE_11())
                .field("DATA_BYTE_15", &self.DATA_BYTE_15())
                .field("DATA_BYTE_19", &self.DATA_BYTE_19())
                .field("DATA_BYTE_23", &self.DATA_BYTE_23())
                .field("DATA_BYTE_27", &self.DATA_BYTE_27())
                .field("DATA_BYTE_3", &self.DATA_BYTE_3())
                .field("DATA_BYTE_31", &self.DATA_BYTE_31())
                .field("DATA_BYTE_35", &self.DATA_BYTE_35())
                .field("DATA_BYTE_39", &self.DATA_BYTE_39())
                .field("DATA_BYTE_43", &self.DATA_BYTE_43())
                .field("DATA_BYTE_47", &self.DATA_BYTE_47())
                .field("DATA_BYTE_51", &self.DATA_BYTE_51())
                .field("DATA_BYTE_55", &self.DATA_BYTE_55())
                .field("DATA_BYTE_59", &self.DATA_BYTE_59())
                .field("DATA_BYTE_63", &self.DATA_BYTE_63())
                .field("DATA_BYTE_7", &self.DATA_BYTE_7())
                .field("DATA_BYTE_10", &self.DATA_BYTE_10())
                .field("DATA_BYTE_14", &self.DATA_BYTE_14())
                .field("DATA_BYTE_18", &self.DATA_BYTE_18())
                .field("DATA_BYTE_2", &self.DATA_BYTE_2())
                .field("DATA_BYTE_22", &self.DATA_BYTE_22())
                .field("DATA_BYTE_26", &self.DATA_BYTE_26())
                .field("DATA_BYTE_30", &self.DATA_BYTE_30())
                .field("DATA_BYTE_34", &self.DATA_BYTE_34())
                .field("DATA_BYTE_38", &self.DATA_BYTE_38())
                .field("DATA_BYTE_42", &self.DATA_BYTE_42())
                .field("DATA_BYTE_46", &self.DATA_BYTE_46())
                .field("DATA_BYTE_50", &self.DATA_BYTE_50())
                .field("DATA_BYTE_54", &self.DATA_BYTE_54())
                .field("DATA_BYTE_58", &self.DATA_BYTE_58())
                .field("DATA_BYTE_6", &self.DATA_BYTE_6())
                .field("DATA_BYTE_62", &self.DATA_BYTE_62())
                .field("DATA_BYTE_1", &self.DATA_BYTE_1())
                .field("DATA_BYTE_13", &self.DATA_BYTE_13())
                .field("DATA_BYTE_17", &self.DATA_BYTE_17())
                .field("DATA_BYTE_21", &self.DATA_BYTE_21())
                .field("DATA_BYTE_25", &self.DATA_BYTE_25())
                .field("DATA_BYTE_29", &self.DATA_BYTE_29())
                .field("DATA_BYTE_33", &self.DATA_BYTE_33())
                .field("DATA_BYTE_37", &self.DATA_BYTE_37())
                .field("DATA_BYTE_41", &self.DATA_BYTE_41())
                .field("DATA_BYTE_45", &self.DATA_BYTE_45())
                .field("DATA_BYTE_49", &self.DATA_BYTE_49())
                .field("DATA_BYTE_5", &self.DATA_BYTE_5())
                .field("DATA_BYTE_53", &self.DATA_BYTE_53())
                .field("DATA_BYTE_57", &self.DATA_BYTE_57())
                .field("DATA_BYTE_61", &self.DATA_BYTE_61())
                .field("DATA_BYTE_9", &self.DATA_BYTE_9())
                .field("DATA_BYTE_0", &self.DATA_BYTE_0())
                .field("DATA_BYTE_12", &self.DATA_BYTE_12())
                .field("DATA_BYTE_16", &self.DATA_BYTE_16())
                .field("DATA_BYTE_20", &self.DATA_BYTE_20())
                .field("DATA_BYTE_24", &self.DATA_BYTE_24())
                .field("DATA_BYTE_28", &self.DATA_BYTE_28())
                .field("DATA_BYTE_32", &self.DATA_BYTE_32())
                .field("DATA_BYTE_36", &self.DATA_BYTE_36())
                .field("DATA_BYTE_4", &self.DATA_BYTE_4())
                .field("DATA_BYTE_40", &self.DATA_BYTE_40())
                .field("DATA_BYTE_44", &self.DATA_BYTE_44())
                .field("DATA_BYTE_48", &self.DATA_BYTE_48())
                .field("DATA_BYTE_52", &self.DATA_BYTE_52())
                .field("DATA_BYTE_56", &self.DATA_BYTE_56())
                .field("DATA_BYTE_60", &self.DATA_BYTE_60())
                .field("DATA_BYTE_8", &self.DATA_BYTE_8())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 CS Register..Message Buffer 31 CS Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_8B_CS(pub u32);
    impl MB_8B_CS {
        #[inline(always)]
        pub const fn TIME_STAMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIME_STAMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ESI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn BRS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MB_8B_CS {
        #[inline(always)]
        fn default() -> MB_8B_CS {
            MB_8B_CS(0)
        }
    }
    impl core::fmt::Debug for MB_8B_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_8B_CS")
                .field("TIME_STAMP", &self.TIME_STAMP())
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .field("CODE", &self.CODE())
                .field("ESI", &self.ESI())
                .field("BRS", &self.BRS())
                .field("EDL", &self.EDL())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 ID Register..Message Buffer 31 ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_8B_ID(pub u32);
    impl MB_8B_ID {
        #[inline(always)]
        pub const fn EXT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EXT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STD(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[inline(always)]
        pub const fn PRIO(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for MB_8B_ID {
        #[inline(always)]
        fn default() -> MB_8B_ID {
            MB_8B_ID(0)
        }
    }
    impl core::fmt::Debug for MB_8B_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_8B_ID")
                .field("EXT", &self.EXT())
                .field("STD", &self.STD())
                .field("PRIO", &self.PRIO())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD_8B Register..Message Buffer 31 WORD_8B Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_8B_WORD(pub u32);
    impl MB_8B_WORD {
        #[inline(always)]
        pub const fn DATA_BYTE_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_8B_WORD {
        #[inline(always)]
        fn default() -> MB_8B_WORD {
            MB_8B_WORD(0)
        }
    }
    impl core::fmt::Debug for MB_8B_WORD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_8B_WORD")
                .field("DATA_BYTE_3", &self.DATA_BYTE_3())
                .field("DATA_BYTE_7", &self.DATA_BYTE_7())
                .field("DATA_BYTE_2", &self.DATA_BYTE_2())
                .field("DATA_BYTE_6", &self.DATA_BYTE_6())
                .field("DATA_BYTE_1", &self.DATA_BYTE_1())
                .field("DATA_BYTE_5", &self.DATA_BYTE_5())
                .field("DATA_BYTE_0", &self.DATA_BYTE_0())
                .field("DATA_BYTE_4", &self.DATA_BYTE_4())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 CS Register..Message Buffer 31 CS Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_CS(pub u32);
    impl MB_CS {
        #[inline(always)]
        pub const fn TIME_STAMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIME_STAMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ESI(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn BRS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MB_CS {
        #[inline(always)]
        fn default() -> MB_CS {
            MB_CS(0)
        }
    }
    impl core::fmt::Debug for MB_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_CS")
                .field("TIME_STAMP", &self.TIME_STAMP())
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .field("CODE", &self.CODE())
                .field("ESI", &self.ESI())
                .field("BRS", &self.BRS())
                .field("EDL", &self.EDL())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 ID Register..Message Buffer 31 ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_ID(pub u32);
    impl MB_ID {
        #[inline(always)]
        pub const fn EXT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EXT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STD(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[inline(always)]
        pub const fn PRIO(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for MB_ID {
        #[inline(always)]
        fn default() -> MB_ID {
            MB_ID(0)
        }
    }
    impl core::fmt::Debug for MB_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_ID")
                .field("EXT", &self.EXT())
                .field("STD", &self.STD())
                .field("PRIO", &self.PRIO())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD0 Register..Message Buffer 31 WORD0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_WORD0(pub u32);
    impl MB_WORD0 {
        #[inline(always)]
        pub const fn DATA_BYTE_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_WORD0 {
        #[inline(always)]
        fn default() -> MB_WORD0 {
            MB_WORD0(0)
        }
    }
    impl core::fmt::Debug for MB_WORD0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_WORD0")
                .field("DATA_BYTE_3", &self.DATA_BYTE_3())
                .field("DATA_BYTE_2", &self.DATA_BYTE_2())
                .field("DATA_BYTE_1", &self.DATA_BYTE_1())
                .field("DATA_BYTE_0", &self.DATA_BYTE_0())
                .finish()
        }
    }
    #[doc = "Message Buffer 0 WORD1 Register..Message Buffer 31 WORD1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MB_WORD1(pub u32);
    impl MB_WORD1 {
        #[inline(always)]
        pub const fn DATA_BYTE_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA_BYTE_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA_BYTE_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MB_WORD1 {
        #[inline(always)]
        fn default() -> MB_WORD1 {
            MB_WORD1(0)
        }
    }
    impl core::fmt::Debug for MB_WORD1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MB_WORD1")
                .field("DATA_BYTE_7", &self.DATA_BYTE_7())
                .field("DATA_BYTE_6", &self.DATA_BYTE_6())
                .field("DATA_BYTE_5", &self.DATA_BYTE_5())
                .field("DATA_BYTE_4", &self.DATA_BYTE_4())
                .finish()
        }
    }
    #[doc = "Module Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[inline(always)]
        pub const fn MAXMB(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAXMB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[inline(always)]
        pub const fn IDAM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IDAM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn FDEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn LPRIOEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPRIOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PNET_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PNET_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DMA(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn IRMQ(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRMQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SRXDIS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRXDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn WAKSRC(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPMACK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPMACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn WRNEN(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WRNEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SLFWAK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLFWAK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn FRZACK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRZACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SOFTRST(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOFTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn WAKMSK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn NOTRDY(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NOTRDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn RFEN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn FRZ(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn MDIS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("MAXMB", &self.MAXMB())
                .field("IDAM", &self.IDAM())
                .field("FDEN", &self.FDEN())
                .field("AEN", &self.AEN())
                .field("LPRIOEN", &self.LPRIOEN())
                .field("PNET_EN", &self.PNET_EN())
                .field("DMA", &self.DMA())
                .field("IRMQ", &self.IRMQ())
                .field("SRXDIS", &self.SRXDIS())
                .field("WAKSRC", &self.WAKSRC())
                .field("LPMACK", &self.LPMACK())
                .field("WRNEN", &self.WRNEN())
                .field("SLFWAK", &self.SLFWAK())
                .field("FRZACK", &self.FRZACK())
                .field("SOFTRST", &self.SOFTRST())
                .field("WAKMSK", &self.WAKMSK())
                .field("NOTRDY", &self.NOTRDY())
                .field("HALT", &self.HALT())
                .field("RFEN", &self.RFEN())
                .field("FRZ", &self.FRZ())
                .field("MDIS", &self.MDIS())
                .finish()
        }
    }
    #[doc = "Pretended Networking Payload High Filter 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PL1_HI(pub u32);
    impl PL1_HI {
        #[inline(always)]
        pub const fn Data_byte_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PL1_HI {
        #[inline(always)]
        fn default() -> PL1_HI {
            PL1_HI(0)
        }
    }
    impl core::fmt::Debug for PL1_HI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PL1_HI")
                .field("Data_byte_7", &self.Data_byte_7())
                .field("Data_byte_6", &self.Data_byte_6())
                .field("Data_byte_5", &self.Data_byte_5())
                .field("Data_byte_4", &self.Data_byte_4())
                .finish()
        }
    }
    #[doc = "Pretended Networking Payload Low Filter 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PL1_LO(pub u32);
    impl PL1_LO {
        #[inline(always)]
        pub const fn Data_byte_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PL1_LO {
        #[inline(always)]
        fn default() -> PL1_LO {
            PL1_LO(0)
        }
    }
    impl core::fmt::Debug for PL1_LO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PL1_LO")
                .field("Data_byte_3", &self.Data_byte_3())
                .field("Data_byte_2", &self.Data_byte_2())
                .field("Data_byte_1", &self.Data_byte_1())
                .field("Data_byte_0", &self.Data_byte_0())
                .finish()
        }
    }
    #[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PL2_PLMASK_HI(pub u32);
    impl PL2_PLMASK_HI {
        #[inline(always)]
        pub const fn Data_byte_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PL2_PLMASK_HI {
        #[inline(always)]
        fn default() -> PL2_PLMASK_HI {
            PL2_PLMASK_HI(0)
        }
    }
    impl core::fmt::Debug for PL2_PLMASK_HI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PL2_PLMASK_HI")
                .field("Data_byte_7", &self.Data_byte_7())
                .field("Data_byte_6", &self.Data_byte_6())
                .field("Data_byte_5", &self.Data_byte_5())
                .field("Data_byte_4", &self.Data_byte_4())
                .finish()
        }
    }
    #[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PL2_PLMASK_LO(pub u32);
    impl PL2_PLMASK_LO {
        #[inline(always)]
        pub const fn Data_byte_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PL2_PLMASK_LO {
        #[inline(always)]
        fn default() -> PL2_PLMASK_LO {
            PL2_PLMASK_LO(0)
        }
    }
    impl core::fmt::Debug for PL2_PLMASK_LO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PL2_PLMASK_LO")
                .field("Data_byte_3", &self.Data_byte_3())
                .field("Data_byte_2", &self.Data_byte_2())
                .field("Data_byte_1", &self.Data_byte_1())
                .field("Data_byte_0", &self.Data_byte_0())
                .finish()
        }
    }
    #[doc = "Legacy RX FIFO Information"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RXFIR(pub u32);
    impl RXFIR {
        #[inline(always)]
        pub const fn IDHIT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IDHIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for RXFIR {
        #[inline(always)]
        fn default() -> RXFIR {
            RXFIR(0)
        }
    }
    impl core::fmt::Debug for RXFIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RXFIR")
                .field("IDHIT", &self.IDHIT())
                .finish()
        }
    }
    #[doc = "Free-Running Timer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER(pub u32);
    impl TIMER {
        #[inline(always)]
        pub const fn TIMER(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIMER(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TIMER {
        #[inline(always)]
        fn default() -> TIMER {
            TIMER(0)
        }
    }
    impl core::fmt::Debug for TIMER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER")
                .field("TIMER", &self.TIMER())
                .finish()
        }
    }
    #[doc = "Wake-Up Message Buffer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WMB_CS(pub u32);
    impl WMB_CS {
        #[inline(always)]
        pub const fn DLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn RTR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IDE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SRR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for WMB_CS {
        #[inline(always)]
        fn default() -> WMB_CS {
            WMB_CS(0)
        }
    }
    impl core::fmt::Debug for WMB_CS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WMB_CS")
                .field("DLC", &self.DLC())
                .field("RTR", &self.RTR())
                .field("IDE", &self.IDE())
                .field("SRR", &self.SRR())
                .finish()
        }
    }
    #[doc = "Wake-Up Message Buffer for Data 0-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WMB_D03(pub u32);
    impl WMB_D03 {
        #[inline(always)]
        pub const fn Data_byte_3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for WMB_D03 {
        #[inline(always)]
        fn default() -> WMB_D03 {
            WMB_D03(0)
        }
    }
    impl core::fmt::Debug for WMB_D03 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WMB_D03")
                .field("Data_byte_3", &self.Data_byte_3())
                .field("Data_byte_2", &self.Data_byte_2())
                .field("Data_byte_1", &self.Data_byte_1())
                .field("Data_byte_0", &self.Data_byte_0())
                .finish()
        }
    }
    #[doc = "Wake-Up Message Buffer Register Data 4-7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WMB_D47(pub u32);
    impl WMB_D47 {
        #[inline(always)]
        pub const fn Data_byte_7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn Data_byte_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn Data_byte_5(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn Data_byte_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_Data_byte_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for WMB_D47 {
        #[inline(always)]
        fn default() -> WMB_D47 {
            WMB_D47(0)
        }
    }
    impl core::fmt::Debug for WMB_D47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WMB_D47")
                .field("Data_byte_7", &self.Data_byte_7())
                .field("Data_byte_6", &self.Data_byte_6())
                .field("Data_byte_5", &self.Data_byte_5())
                .field("Data_byte_4", &self.Data_byte_4())
                .finish()
        }
    }
    #[doc = "Wake-Up Message Buffer for ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WMB_ID(pub u32);
    impl WMB_ID {
        #[inline(always)]
        pub const fn ID(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ID(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for WMB_ID {
        #[inline(always)]
        fn default() -> WMB_ID {
            WMB_ID(0)
        }
    }
    impl core::fmt::Debug for WMB_ID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WMB_ID").field("ID", &self.ID()).finish()
        }
    }
    #[doc = "Pretended Networking Wake-Up Match"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WU_MTC(pub u32);
    impl WU_MTC {
        #[inline(always)]
        pub const fn MCOUNTER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MCOUNTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn WUMF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUMF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WTOF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WTOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for WU_MTC {
        #[inline(always)]
        fn default() -> WU_MTC {
            WU_MTC(0)
        }
    }
    impl core::fmt::Debug for WU_MTC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WU_MTC")
                .field("MCOUNTER", &self.MCOUNTER())
                .field("WUMF", &self.WUMF())
                .field("WTOF", &self.WTOF())
                .finish()
        }
    }
}
