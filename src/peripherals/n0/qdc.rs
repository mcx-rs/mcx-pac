#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QDC {
    ptr: *mut u8,
}
unsafe impl Send for QDC {}
unsafe impl Sync for QDC {}
impl QDC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn FILT(self) -> crate::common::Reg<regs::FILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn WTR(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn POSD(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn POSDH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn REV(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn REVH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn UPOS(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn LPOS(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn UPOSH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn LPOSH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn UINIT(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn LINIT(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IMR(self) -> crate::common::Reg<regs::IMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[inline(always)]
    pub const fn TST(self) -> crate::common::Reg<regs::TST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<regs::CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
    #[inline(always)]
    pub const fn UMOD(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn LMOD(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x22usize) as _) }
    }
    #[inline(always)]
    pub const fn UCOMP(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn LCOMP(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x26usize) as _) }
    }
    #[inline(always)]
    pub const fn LASTEDGE(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn LASTEDGEH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2ausize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPER(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPERBFR(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[inline(always)]
    pub const fn POSDPERH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL3(self) -> crate::common::Reg<regs::CTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x32usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u16);
    impl CTRL {
        #[inline(always)]
        pub const fn CMPIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMPIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CMPIRQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMPIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DIRQ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn XNE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn XIP(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn XIE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn XIRQ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_XIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn PH1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PH1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REV(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SWIP(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn HNE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn HIP(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn HIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn HIRQ(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
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
                .field("CMPIE", &self.CMPIE())
                .field("CMPIRQ", &self.CMPIRQ())
                .field("WDE", &self.WDE())
                .field("DIE", &self.DIE())
                .field("DIRQ", &self.DIRQ())
                .field("XNE", &self.XNE())
                .field("XIP", &self.XIP())
                .field("XIE", &self.XIE())
                .field("XIRQ", &self.XIRQ())
                .field("PH1", &self.PH1())
                .field("REV", &self.REV())
                .field("SWIP", &self.SWIP())
                .field("HNE", &self.HNE())
                .field("HIP", &self.HIP())
                .field("HIE", &self.HIE())
                .field("HIRQ", &self.HIRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ CMPIE: {=bool:?}, CMPIRQ: {=bool:?}, WDE: {=bool:?}, DIE: {=bool:?}, DIRQ: {=bool:?}, XNE: {=bool:?}, XIP: {=bool:?}, XIE: {=bool:?}, XIRQ: {=bool:?}, PH1: {=bool:?}, REV: {=bool:?}, SWIP: {=bool:?}, HNE: {=bool:?}, HIP: {=bool:?}, HIE: {=bool:?}, HIRQ: {=bool:?} }}" , self . CMPIE () , self . CMPIRQ () , self . WDE () , self . DIE () , self . DIRQ () , self . XNE () , self . XIP () , self . XIE () , self . XIRQ () , self . PH1 () , self . REV () , self . SWIP () , self . HNE () , self . HIP () , self . HIE () , self . HIRQ ())
        }
    }
    #[doc = "Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL2(pub u16);
    impl CTRL2 {
        #[inline(always)]
        pub const fn UPDHLD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPDHLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn UPDPOS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPDPOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MOD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RUIE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RUIRQ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ROIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ROIRQ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REVMOD(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REVMOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OUTCTL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUTCTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SABIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SABIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SABIRQ(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SABIRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn INITPOS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INITPOS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
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
                .field("UPDHLD", &self.UPDHLD())
                .field("UPDPOS", &self.UPDPOS())
                .field("MOD", &self.MOD())
                .field("DIR", &self.DIR())
                .field("RUIE", &self.RUIE())
                .field("RUIRQ", &self.RUIRQ())
                .field("ROIE", &self.ROIE())
                .field("ROIRQ", &self.ROIRQ())
                .field("REVMOD", &self.REVMOD())
                .field("OUTCTL", &self.OUTCTL())
                .field("SABIE", &self.SABIE())
                .field("SABIRQ", &self.SABIRQ())
                .field("INITPOS", &self.INITPOS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL2 {{ UPDHLD: {=bool:?}, UPDPOS: {=bool:?}, MOD: {=bool:?}, DIR: {=bool:?}, RUIE: {=bool:?}, RUIRQ: {=bool:?}, ROIE: {=bool:?}, ROIRQ: {=bool:?}, REVMOD: {=bool:?}, OUTCTL: {=bool:?}, SABIE: {=bool:?}, SABIRQ: {=bool:?}, INITPOS: {=bool:?} }}" , self . UPDHLD () , self . UPDPOS () , self . MOD () , self . DIR () , self . RUIE () , self . RUIRQ () , self . ROIE () , self . ROIRQ () , self . REVMOD () , self . OUTCTL () , self . SABIE () , self . SABIRQ () , self . INITPOS ())
        }
    }
    #[doc = "Control 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL3(pub u16);
    impl CTRL3 {
        #[inline(always)]
        pub const fn PMEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PRSC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
    }
    impl Default for CTRL3 {
        #[inline(always)]
        fn default() -> CTRL3 {
            CTRL3(0)
        }
    }
    impl core::fmt::Debug for CTRL3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL3")
                .field("PMEN", &self.PMEN())
                .field("PRSC", &self.PRSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CTRL3 {{ PMEN: {=bool:?}, PRSC: {=u8:?} }}",
                self.PMEN(),
                self.PRSC()
            )
        }
    }
    #[doc = "Input Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FILT(pub u16);
    impl FILT {
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn FILT_PRSC(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PRSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for FILT {
        #[inline(always)]
        fn default() -> FILT {
            FILT(0)
        }
    }
    impl core::fmt::Debug for FILT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FILT")
                .field("FILT_PER", &self.FILT_PER())
                .field("FILT_CNT", &self.FILT_CNT())
                .field("FILT_PRSC", &self.FILT_PRSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FILT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FILT {{ FILT_PER: {=u8:?}, FILT_CNT: {=u8:?}, FILT_PRSC: {=u8:?} }}",
                self.FILT_PER(),
                self.FILT_CNT(),
                self.FILT_PRSC()
            )
        }
    }
    #[doc = "Input Monitor"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IMR(pub u16);
    impl IMR {
        #[inline(always)]
        pub const fn HOME(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INDEX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INDEX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PHB(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PHA(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FHOM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FHOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FIND(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FPHB(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FPHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FPHA(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FPHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for IMR {
        #[inline(always)]
        fn default() -> IMR {
            IMR(0)
        }
    }
    impl core::fmt::Debug for IMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IMR")
                .field("HOME", &self.HOME())
                .field("INDEX", &self.INDEX())
                .field("PHB", &self.PHB())
                .field("PHA", &self.PHA())
                .field("FHOM", &self.FHOM())
                .field("FIND", &self.FIND())
                .field("FPHB", &self.FPHB())
                .field("FPHA", &self.FPHA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IMR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IMR {{ HOME: {=bool:?}, INDEX: {=bool:?}, PHB: {=bool:?}, PHA: {=bool:?}, FHOM: {=bool:?}, FIND: {=bool:?}, FPHB: {=bool:?}, FPHA: {=bool:?} }}" , self . HOME () , self . INDEX () , self . PHB () , self . PHA () , self . FHOM () , self . FIND () , self . FPHB () , self . FPHA ())
        }
    }
    #[doc = "Test"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TST(pub u16);
    impl TST {
        #[inline(always)]
        pub const fn TEST_COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEST_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TEST_PERIOD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEST_PERIOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn QDN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn TCE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn TEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for TST {
        #[inline(always)]
        fn default() -> TST {
            TST(0)
        }
    }
    impl core::fmt::Debug for TST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TST")
                .field("TEST_COUNT", &self.TEST_COUNT())
                .field("TEST_PERIOD", &self.TEST_PERIOD())
                .field("QDN", &self.QDN())
                .field("TCE", &self.TCE())
                .field("TEN", &self.TEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TST {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TST {{ TEST_COUNT: {=u8:?}, TEST_PERIOD: {=u8:?}, QDN: {=bool:?}, TCE: {=bool:?}, TEN: {=bool:?} }}" , self . TEST_COUNT () , self . TEST_PERIOD () , self . QDN () , self . TCE () , self . TEN ())
        }
    }
}
