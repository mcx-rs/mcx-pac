#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART {
    ptr: *mut u8,
}
unsafe impl Send for LPUART {}
unsafe impl Sync for LPUART {}
impl LPUART {
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
    pub const fn GLOBAL(self) -> crate::common::Reg<regs::GLOBAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PINCFG(self) -> crate::common::Reg<regs::PINCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn BAUD(self) -> crate::common::Reg<regs::BAUD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DATA(self) -> crate::common::Reg<regs::DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH(self) -> crate::common::Reg<regs::MATCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MODIR(self) -> crate::common::Reg<regs::MODIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn FIFO(self) -> crate::common::Reg<regs::FIFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn WATER(self) -> crate::common::Reg<regs::WATER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn DATARO(self) -> crate::common::Reg<regs::DATARO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<regs::MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn REIR(self) -> crate::common::Reg<regs::REIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn TEIR(self) -> crate::common::Reg<regs::TEIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn HDCR(self) -> crate::common::Reg<regs::HDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn TOCR(self) -> crate::common::Reg<regs::TOCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn TOSR(self) -> crate::common::Reg<regs::TOSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMEOUT(self, n: usize) -> crate::common::Reg<regs::TIMEOUT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TCBR(self, n: usize) -> crate::common::Reg<regs::TCBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TDBR(self, n: usize) -> crate::common::Reg<regs::TDBR, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Baud Rate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BAUD(pub u32);
    impl BAUD {
        #[inline(always)]
        pub const fn SBR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SBR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[inline(always)]
        pub const fn SBNS(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBNS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn RXEDGIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEDGIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn LBKDIE(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LBKDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn RESYNCDIS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESYNCDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn BOTHEDGE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BOTHEDGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn MATCFG(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MATCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn RIDMAE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RIDMAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn RDMAE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDMAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TDMAE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDMAE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn OSR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OSR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[inline(always)]
        pub const fn M10(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn MAEN2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAEN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn MAEN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAEN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for BAUD {
        #[inline(always)]
        fn default() -> BAUD {
            BAUD(0)
        }
    }
    impl core::fmt::Debug for BAUD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BAUD")
                .field("SBR", &self.SBR())
                .field("SBNS", &self.SBNS())
                .field("RXEDGIE", &self.RXEDGIE())
                .field("LBKDIE", &self.LBKDIE())
                .field("RESYNCDIS", &self.RESYNCDIS())
                .field("BOTHEDGE", &self.BOTHEDGE())
                .field("MATCFG", &self.MATCFG())
                .field("RIDMAE", &self.RIDMAE())
                .field("RDMAE", &self.RDMAE())
                .field("TDMAE", &self.TDMAE())
                .field("OSR", &self.OSR())
                .field("M10", &self.M10())
                .field("MAEN2", &self.MAEN2())
                .field("MAEN1", &self.MAEN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BAUD {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BAUD {{ SBR: {=u16:?}, SBNS: {=bool:?}, RXEDGIE: {=bool:?}, LBKDIE: {=bool:?}, RESYNCDIS: {=bool:?}, BOTHEDGE: {=bool:?}, MATCFG: {=u8:?}, RIDMAE: {=bool:?}, RDMAE: {=bool:?}, TDMAE: {=bool:?}, OSR: {=u8:?}, M10: {=bool:?}, MAEN2: {=bool:?}, MAEN1: {=bool:?} }}" , self . SBR () , self . SBNS () , self . RXEDGIE () , self . LBKDIE () , self . RESYNCDIS () , self . BOTHEDGE () , self . MATCFG () , self . RIDMAE () , self . RDMAE () , self . TDMAE () , self . OSR () , self . M10 () , self . MAEN2 () , self . MAEN1 ())
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn PT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ILT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ILT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WAKE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn M(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RSRC(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DOZEEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn LOOPS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOOPS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn IDLECFG(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_IDLECFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn M7(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_M7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn MA2IE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MA2IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn MA1IE(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MA1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn SBK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn RWU(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RWU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn TE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ILIE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ILIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn RIE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TCIE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TIE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PEIE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn FEIE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn NEIE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ORIE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ORIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn TXINV(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXINV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn TXDIR(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXDIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn R9T8(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R9T8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn R8T9(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R8T9(&mut self, val: bool) {
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
                .field("PT", &self.PT())
                .field("PE", &self.PE())
                .field("ILT", &self.ILT())
                .field("WAKE", &self.WAKE())
                .field("M", &self.M())
                .field("RSRC", &self.RSRC())
                .field("DOZEEN", &self.DOZEEN())
                .field("LOOPS", &self.LOOPS())
                .field("IDLECFG", &self.IDLECFG())
                .field("M7", &self.M7())
                .field("MA2IE", &self.MA2IE())
                .field("MA1IE", &self.MA1IE())
                .field("SBK", &self.SBK())
                .field("RWU", &self.RWU())
                .field("RE", &self.RE())
                .field("TE", &self.TE())
                .field("ILIE", &self.ILIE())
                .field("RIE", &self.RIE())
                .field("TCIE", &self.TCIE())
                .field("TIE", &self.TIE())
                .field("PEIE", &self.PEIE())
                .field("FEIE", &self.FEIE())
                .field("NEIE", &self.NEIE())
                .field("ORIE", &self.ORIE())
                .field("TXINV", &self.TXINV())
                .field("TXDIR", &self.TXDIR())
                .field("R9T8", &self.R9T8())
                .field("R8T9", &self.R8T9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ PT: {=bool:?}, PE: {=bool:?}, ILT: {=bool:?}, WAKE: {=bool:?}, M: {=bool:?}, RSRC: {=bool:?}, DOZEEN: {=bool:?}, LOOPS: {=bool:?}, IDLECFG: {=u8:?}, M7: {=bool:?}, MA2IE: {=bool:?}, MA1IE: {=bool:?}, SBK: {=bool:?}, RWU: {=bool:?}, RE: {=bool:?}, TE: {=bool:?}, ILIE: {=bool:?}, RIE: {=bool:?}, TCIE: {=bool:?}, TIE: {=bool:?}, PEIE: {=bool:?}, FEIE: {=bool:?}, NEIE: {=bool:?}, ORIE: {=bool:?}, TXINV: {=bool:?}, TXDIR: {=bool:?}, R9T8: {=bool:?}, R8T9: {=bool:?} }}" , self . PT () , self . PE () , self . ILT () , self . WAKE () , self . M () , self . RSRC () , self . DOZEEN () , self . LOOPS () , self . IDLECFG () , self . M7 () , self . MA2IE () , self . MA1IE () , self . SBK () , self . RWU () , self . RE () , self . TE () , self . ILIE () , self . RIE () , self . TCIE () , self . TIE () , self . PEIE () , self . FEIE () , self . NEIE () , self . ORIE () , self . TXINV () , self . TXDIR () , self . R9T8 () , self . R8T9 ())
        }
    }
    #[doc = "Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA(pub u32);
    impl DATA {
        #[inline(always)]
        pub const fn R0T0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R0T0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn R1T1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R1T1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn R2T2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R2T2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn R3T3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R3T3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn R4T4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R4T4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn R5T5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R5T5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn R6T6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R6T6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn R7T7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R7T7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn R8T8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R8T8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn R9T9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_R9T9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LINBRK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LINBRK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn IDLINE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDLINE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn RXEMPT(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FRETSC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRETSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PARITYE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PARITYE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn NOISY(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NOISY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("R0T0", &self.R0T0())
                .field("R1T1", &self.R1T1())
                .field("R2T2", &self.R2T2())
                .field("R3T3", &self.R3T3())
                .field("R4T4", &self.R4T4())
                .field("R5T5", &self.R5T5())
                .field("R6T6", &self.R6T6())
                .field("R7T7", &self.R7T7())
                .field("R8T8", &self.R8T8())
                .field("R9T9", &self.R9T9())
                .field("LINBRK", &self.LINBRK())
                .field("IDLINE", &self.IDLINE())
                .field("RXEMPT", &self.RXEMPT())
                .field("FRETSC", &self.FRETSC())
                .field("PARITYE", &self.PARITYE())
                .field("NOISY", &self.NOISY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DATA {{ R0T0: {=bool:?}, R1T1: {=bool:?}, R2T2: {=bool:?}, R3T3: {=bool:?}, R4T4: {=bool:?}, R5T5: {=bool:?}, R6T6: {=bool:?}, R7T7: {=bool:?}, R8T8: {=bool:?}, R9T9: {=bool:?}, LINBRK: {=bool:?}, IDLINE: {=bool:?}, RXEMPT: {=bool:?}, FRETSC: {=bool:?}, PARITYE: {=bool:?}, NOISY: {=bool:?} }}" , self . R0T0 () , self . R1T1 () , self . R2T2 () , self . R3T3 () , self . R4T4 () , self . R5T5 () , self . R6T6 () , self . R7T7 () , self . R8T8 () , self . R9T9 () , self . LINBRK () , self . IDLINE () , self . RXEMPT () , self . FRETSC () , self . PARITYE () , self . NOISY ())
        }
    }
    #[doc = "Data Read-Only"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATARO(pub u32);
    impl DATARO {
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DATARO {
        #[inline(always)]
        fn default() -> DATARO {
            DATARO(0)
        }
    }
    impl core::fmt::Debug for DATARO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATARO")
                .field("DATA", &self.DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATARO {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DATARO {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "FIFO"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIFO(pub u32);
    impl FIFO {
        #[inline(always)]
        pub const fn RXFIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXFIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn RXFE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TXFIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXFIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[inline(always)]
        pub const fn TXFE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RXUFE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXUFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TXOFE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXOFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RXIDEN(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[inline(always)]
        pub const fn RXFLUSH(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXFLUSH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn TXFLUSH(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXFLUSH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn RXUF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TXOF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RXEMPT(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TXEMPT(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXEMPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for FIFO {
        #[inline(always)]
        fn default() -> FIFO {
            FIFO(0)
        }
    }
    impl core::fmt::Debug for FIFO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIFO")
                .field("RXFIFOSIZE", &self.RXFIFOSIZE())
                .field("RXFE", &self.RXFE())
                .field("TXFIFOSIZE", &self.TXFIFOSIZE())
                .field("TXFE", &self.TXFE())
                .field("RXUFE", &self.RXUFE())
                .field("TXOFE", &self.TXOFE())
                .field("RXIDEN", &self.RXIDEN())
                .field("RXFLUSH", &self.RXFLUSH())
                .field("TXFLUSH", &self.TXFLUSH())
                .field("RXUF", &self.RXUF())
                .field("TXOF", &self.TXOF())
                .field("RXEMPT", &self.RXEMPT())
                .field("TXEMPT", &self.TXEMPT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIFO {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FIFO {{ RXFIFOSIZE: {=u8:?}, RXFE: {=bool:?}, TXFIFOSIZE: {=u8:?}, TXFE: {=bool:?}, RXUFE: {=bool:?}, TXOFE: {=bool:?}, RXIDEN: {=u8:?}, RXFLUSH: {=bool:?}, TXFLUSH: {=bool:?}, RXUF: {=bool:?}, TXOF: {=bool:?}, RXEMPT: {=bool:?}, TXEMPT: {=bool:?} }}" , self . RXFIFOSIZE () , self . RXFE () , self . TXFIFOSIZE () , self . TXFE () , self . RXUFE () , self . TXOFE () , self . RXIDEN () , self . RXFLUSH () , self . TXFLUSH () , self . RXUF () , self . TXOF () , self . RXEMPT () , self . TXEMPT ())
        }
    }
    #[doc = "Global"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GLOBAL(pub u32);
    impl GLOBAL {
        #[inline(always)]
        pub const fn RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for GLOBAL {
        #[inline(always)]
        fn default() -> GLOBAL {
            GLOBAL(0)
        }
    }
    impl core::fmt::Debug for GLOBAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GLOBAL").field("RST", &self.RST()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GLOBAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GLOBAL {{ RST: {=bool:?} }}", self.RST())
        }
    }
    #[doc = "Half Duplex Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HDCR(pub u32);
    impl HDCR {
        #[inline(always)]
        pub const fn TXSTALL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RXSEL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RXWRMSK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXWRMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RXMSK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXMSK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RTSEXT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RTSEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for HDCR {
        #[inline(always)]
        fn default() -> HDCR {
            HDCR(0)
        }
    }
    impl core::fmt::Debug for HDCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HDCR")
                .field("TXSTALL", &self.TXSTALL())
                .field("RXSEL", &self.RXSEL())
                .field("RXWRMSK", &self.RXWRMSK())
                .field("RXMSK", &self.RXMSK())
                .field("RTSEXT", &self.RTSEXT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HDCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "HDCR {{ TXSTALL: {=bool:?}, RXSEL: {=bool:?}, RXWRMSK: {=bool:?}, RXMSK: {=bool:?}, RTSEXT: {=u8:?} }}" , self . TXSTALL () , self . RXSEL () , self . RXWRMSK () , self . RXMSK () , self . RTSEXT ())
        }
    }
    #[doc = "Match Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH(pub u32);
    impl MATCH {
        #[inline(always)]
        pub const fn MA1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MA1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[inline(always)]
        pub const fn MA2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MA2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for MATCH {
        #[inline(always)]
        fn default() -> MATCH {
            MATCH(0)
        }
    }
    impl core::fmt::Debug for MATCH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCH")
                .field("MA1", &self.MA1())
                .field("MA2", &self.MA2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MATCH {{ MA1: {=u16:?}, MA2: {=u16:?} }}",
                self.MA1(),
                self.MA2()
            )
        }
    }
    #[doc = "MODEM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[inline(always)]
        pub const fn CTS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DSR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DSR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RIN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DCD(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DTR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RTS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("CTS", &self.CTS())
                .field("DSR", &self.DSR())
                .field("RIN", &self.RIN())
                .field("DCD", &self.DCD())
                .field("DTR", &self.DTR())
                .field("RTS", &self.RTS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCR {{ CTS: {=bool:?}, DSR: {=bool:?}, RIN: {=bool:?}, DCD: {=bool:?}, DTR: {=bool:?}, RTS: {=bool:?} }}" , self . CTS () , self . DSR () , self . RIN () , self . DCD () , self . DTR () , self . RTS ())
        }
    }
    #[doc = "MODEM IrDA"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MODIR(pub u32);
    impl MODIR {
        #[inline(always)]
        pub const fn TXCTSE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXCTSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TXRTSE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXRTSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TXRTSPOL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXRTSPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RXRTSE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXRTSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TXCTSC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXCTSC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TXCTSSRC(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXCTSSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RTSWATER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RTSWATER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn TNP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TNP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn IREN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for MODIR {
        #[inline(always)]
        fn default() -> MODIR {
            MODIR(0)
        }
    }
    impl core::fmt::Debug for MODIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MODIR")
                .field("TXCTSE", &self.TXCTSE())
                .field("TXRTSE", &self.TXRTSE())
                .field("TXRTSPOL", &self.TXRTSPOL())
                .field("RXRTSE", &self.RXRTSE())
                .field("TXCTSC", &self.TXCTSC())
                .field("TXCTSSRC", &self.TXCTSSRC())
                .field("RTSWATER", &self.RTSWATER())
                .field("TNP", &self.TNP())
                .field("IREN", &self.IREN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MODIR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MODIR {{ TXCTSE: {=bool:?}, TXRTSE: {=bool:?}, TXRTSPOL: {=bool:?}, RXRTSE: {=bool:?}, TXCTSC: {=bool:?}, TXCTSSRC: {=bool:?}, RTSWATER: {=u8:?}, TNP: {=u8:?}, IREN: {=bool:?} }}" , self . TXCTSE () , self . TXRTSE () , self . TXRTSPOL () , self . RXRTSE () , self . TXCTSC () , self . TXCTSSRC () , self . RTSWATER () , self . TNP () , self . IREN ())
        }
    }
    #[doc = "MODEM Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSR(pub u32);
    impl MSR {
        #[inline(always)]
        pub const fn DCTS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DDSR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DDSR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DRI(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DRI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DDCD(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DDCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DSR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DSR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RIN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DCD(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for MSR {
        #[inline(always)]
        fn default() -> MSR {
            MSR(0)
        }
    }
    impl core::fmt::Debug for MSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSR")
                .field("DCTS", &self.DCTS())
                .field("DDSR", &self.DDSR())
                .field("DRI", &self.DRI())
                .field("DDCD", &self.DDCD())
                .field("CTS", &self.CTS())
                .field("DSR", &self.DSR())
                .field("RIN", &self.RIN())
                .field("DCD", &self.DCD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MSR {{ DCTS: {=bool:?}, DDSR: {=bool:?}, DRI: {=bool:?}, DDCD: {=bool:?}, CTS: {=bool:?}, DSR: {=bool:?}, RIN: {=bool:?}, DCD: {=bool:?} }}" , self . DCTS () , self . DDSR () , self . DRI () , self . DDCD () , self . CTS () , self . DSR () , self . RIN () , self . DCD ())
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn TXFIFO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXFIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RXFIFO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXFIFO(&mut self, val: u8) {
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
                .field("TXFIFO", &self.TXFIFO())
                .field("RXFIFO", &self.RXFIFO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PARAM {{ TXFIFO: {=u8:?}, RXFIFO: {=u8:?} }}",
                self.TXFIFO(),
                self.RXFIFO()
            )
        }
    }
    #[doc = "Pin Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINCFG(pub u32);
    impl PINCFG {
        #[inline(always)]
        pub const fn TRGSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRGSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for PINCFG {
        #[inline(always)]
        fn default() -> PINCFG {
            PINCFG(0)
        }
    }
    impl core::fmt::Debug for PINCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PINCFG")
                .field("TRGSEL", &self.TRGSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PINCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PINCFG {{ TRGSEL: {=u8:?} }}", self.TRGSEL())
        }
    }
    #[doc = "Receiver Extended Idle"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REIR(pub u32);
    impl REIR {
        #[inline(always)]
        pub const fn IDTIME(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IDTIME(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for REIR {
        #[inline(always)]
        fn default() -> REIR {
            REIR(0)
        }
    }
    impl core::fmt::Debug for REIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REIR")
                .field("IDTIME", &self.IDTIME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REIR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "REIR {{ IDTIME: {=u16:?} }}", self.IDTIME())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[inline(always)]
        pub const fn LBKFE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LBKFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn AME(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MSF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TSF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn MA2F(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MA2F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn MA1F(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MA1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn NF(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn OR(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn IDLE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IDLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn RDRF(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TC(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TDRE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn RAF(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn LBKDE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LBKDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn BRK13(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BRK13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn RWUID(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RWUID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn RXINV(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXINV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn MSBF(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MSBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn RXEDGIF(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEDGIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn LBKDIF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LBKDIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for STAT {
        #[inline(always)]
        fn default() -> STAT {
            STAT(0)
        }
    }
    impl core::fmt::Debug for STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STAT")
                .field("LBKFE", &self.LBKFE())
                .field("AME", &self.AME())
                .field("MSF", &self.MSF())
                .field("TSF", &self.TSF())
                .field("MA2F", &self.MA2F())
                .field("MA1F", &self.MA1F())
                .field("PF", &self.PF())
                .field("FE", &self.FE())
                .field("NF", &self.NF())
                .field("OR", &self.OR())
                .field("IDLE", &self.IDLE())
                .field("RDRF", &self.RDRF())
                .field("TC", &self.TC())
                .field("TDRE", &self.TDRE())
                .field("RAF", &self.RAF())
                .field("LBKDE", &self.LBKDE())
                .field("BRK13", &self.BRK13())
                .field("RWUID", &self.RWUID())
                .field("RXINV", &self.RXINV())
                .field("MSBF", &self.MSBF())
                .field("RXEDGIF", &self.RXEDGIF())
                .field("LBKDIF", &self.LBKDIF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STAT {{ LBKFE: {=bool:?}, AME: {=bool:?}, MSF: {=bool:?}, TSF: {=bool:?}, MA2F: {=bool:?}, MA1F: {=bool:?}, PF: {=bool:?}, FE: {=bool:?}, NF: {=bool:?}, OR: {=bool:?}, IDLE: {=bool:?}, RDRF: {=bool:?}, TC: {=bool:?}, TDRE: {=bool:?}, RAF: {=bool:?}, LBKDE: {=bool:?}, BRK13: {=bool:?}, RWUID: {=bool:?}, RXINV: {=bool:?}, MSBF: {=bool:?}, RXEDGIF: {=bool:?}, LBKDIF: {=bool:?} }}" , self . LBKFE () , self . AME () , self . MSF () , self . TSF () , self . MA2F () , self . MA1F () , self . PF () , self . FE () , self . NF () , self . OR () , self . IDLE () , self . RDRF () , self . TC () , self . TDRE () , self . RAF () , self . LBKDE () , self . BRK13 () , self . RWUID () , self . RXINV () , self . MSBF () , self . RXEDGIF () , self . LBKDIF ())
        }
    }
    #[doc = "Transmit Command Burst"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCBR(pub u32);
    impl TCBR {
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TCBR {
        #[inline(always)]
        fn default() -> TCBR {
            TCBR(0)
        }
    }
    impl core::fmt::Debug for TCBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCBR").field("DATA", &self.DATA()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCBR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TCBR {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "Transmit Data Burst"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TDBR(pub u32);
    impl TDBR {
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TDBR {
        #[inline(always)]
        fn default() -> TDBR {
            TDBR(0)
        }
    }
    impl core::fmt::Debug for TDBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TDBR")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .field("DATA2", &self.DATA2())
                .field("DATA3", &self.DATA3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TDBR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TDBR {{ DATA0: {=u8:?}, DATA1: {=u8:?}, DATA2: {=u8:?}, DATA3: {=u8:?} }}",
                self.DATA0(),
                self.DATA1(),
                self.DATA2(),
                self.DATA3()
            )
        }
    }
    #[doc = "Transmitter Extended Idle"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TEIR(pub u32);
    impl TEIR {
        #[inline(always)]
        pub const fn IDTIME(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IDTIME(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for TEIR {
        #[inline(always)]
        fn default() -> TEIR {
            TEIR(0)
        }
    }
    impl core::fmt::Debug for TEIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TEIR")
                .field("IDTIME", &self.IDTIME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TEIR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TEIR {{ IDTIME: {=u16:?} }}", self.IDTIME())
        }
    }
    #[doc = "Timeout N"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMEOUT(pub u32);
    impl TIMEOUT {
        #[inline(always)]
        pub const fn TIMEOUT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TIMEOUT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[inline(always)]
        pub const fn CFG(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for TIMEOUT {
        #[inline(always)]
        fn default() -> TIMEOUT {
            TIMEOUT(0)
        }
    }
    impl core::fmt::Debug for TIMEOUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMEOUT")
                .field("TIMEOUT", &self.TIMEOUT())
                .field("CFG", &self.CFG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMEOUT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TIMEOUT {{ TIMEOUT: {=u16:?}, CFG: {=u8:?} }}",
                self.TIMEOUT(),
                self.CFG()
            )
        }
    }
    #[doc = "Timeout Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TOCR(pub u32);
    impl TOCR {
        #[inline(always)]
        pub const fn TOEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TOIE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for TOCR {
        #[inline(always)]
        fn default() -> TOCR {
            TOCR(0)
        }
    }
    impl core::fmt::Debug for TOCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TOCR")
                .field("TOEN", &self.TOEN())
                .field("TOIE", &self.TOIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TOCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TOCR {{ TOEN: {=u8:?}, TOIE: {=u8:?} }}",
                self.TOEN(),
                self.TOIE()
            )
        }
    }
    #[doc = "Timeout Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TOSR(pub u32);
    impl TOSR {
        #[inline(always)]
        pub const fn TOZ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TOF(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TOF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for TOSR {
        #[inline(always)]
        fn default() -> TOSR {
            TOSR(0)
        }
    }
    impl core::fmt::Debug for TOSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TOSR")
                .field("TOZ", &self.TOZ())
                .field("TOF", &self.TOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TOSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TOSR {{ TOZ: {=u8:?}, TOF: {=u8:?} }}",
                self.TOZ(),
                self.TOF()
            )
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
            defmt::write!(
                f,
                "VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
                self.FEATURE(),
                self.MINOR(),
                self.MAJOR()
            )
        }
    }
    #[doc = "Watermark"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WATER(pub u32);
    impl WATER {
        #[inline(always)]
        pub const fn TXWATER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXWATER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCOUNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn RXWATER(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXWATER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn RXCOUNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for WATER {
        #[inline(always)]
        fn default() -> WATER {
            WATER(0)
        }
    }
    impl core::fmt::Debug for WATER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WATER")
                .field("TXWATER", &self.TXWATER())
                .field("TXCOUNT", &self.TXCOUNT())
                .field("RXWATER", &self.RXWATER())
                .field("RXCOUNT", &self.RXCOUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WATER {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WATER {{ TXWATER: {=u8:?}, TXCOUNT: {=u8:?}, RXWATER: {=u8:?}, RXCOUNT: {=u8:?} }}" , self . TXWATER () , self . TXCOUNT () , self . RXWATER () , self . RXCOUNT ())
        }
    }
}
