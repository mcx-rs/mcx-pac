#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CACHE64_CTRL {
    ptr: *mut u8,
}
unsafe impl Send for CACHE64_CTRL {}
unsafe impl Sync for CACHE64_CTRL {}
impl CACHE64_CTRL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CCR(self) -> crate::common::Reg<regs::CCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[inline(always)]
    pub const fn CLCR(self) -> crate::common::Reg<regs::CLCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[inline(always)]
    pub const fn CSAR(self) -> crate::common::Reg<regs::CSAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[inline(always)]
    pub const fn CCVR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Cache Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR(pub u32);
    impl CCR {
        #[must_use]
        #[inline(always)]
        pub const fn ENCACHE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ENCACHE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENWRBUF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ENWRBUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRCWT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRCWT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRCNOALLC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRCNOALLC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INVW0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INVW0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUSHW0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PUSHW0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INVW1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INVW1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUSHW1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PUSHW1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GO(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CCR {
        #[inline(always)]
        fn default() -> CCR {
            CCR(0)
        }
    }
    impl core::fmt::Debug for CCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CCR")
                .field("ENCACHE", &self.ENCACHE())
                .field("ENWRBUF", &self.ENWRBUF())
                .field("FRCWT", &self.FRCWT())
                .field("FRCNOALLC", &self.FRCNOALLC())
                .field("INVW0", &self.INVW0())
                .field("PUSHW0", &self.PUSHW0())
                .field("INVW1", &self.INVW1())
                .field("PUSHW1", &self.PUSHW1())
                .field("GO", &self.GO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CCR {{ ENCACHE: {=bool:?}, ENWRBUF: {=bool:?}, FRCWT: {=bool:?}, FRCNOALLC: {=bool:?}, INVW0: {=bool:?}, PUSHW0: {=bool:?}, INVW1: {=bool:?}, PUSHW1: {=bool:?}, GO: {=bool:?} }}" , self . ENCACHE () , self . ENWRBUF () , self . FRCWT () , self . FRCNOALLC () , self . INVW0 () , self . PUSHW0 () , self . INVW1 () , self . PUSHW1 () , self . GO ())
        }
    }
    #[doc = "Cache Line Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLCR(pub u32);
    impl CLCR {
        #[must_use]
        #[inline(always)]
        pub const fn LGO(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LGO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CACHEADDR(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CACHEADDR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 2usize)) | (((val as u32) & 0x07ff) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WSEL(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDSEL(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCIVB(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCIVB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCIMB(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCIMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCWAY(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCWAY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCMD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LCMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LADSEL(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LADSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LACC(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LACC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for CLCR {
        #[inline(always)]
        fn default() -> CLCR {
            CLCR(0)
        }
    }
    impl core::fmt::Debug for CLCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLCR")
                .field("LGO", &self.LGO())
                .field("CACHEADDR", &self.CACHEADDR())
                .field("WSEL", &self.WSEL())
                .field("TDSEL", &self.TDSEL())
                .field("LCIVB", &self.LCIVB())
                .field("LCIMB", &self.LCIMB())
                .field("LCWAY", &self.LCWAY())
                .field("LCMD", &self.LCMD())
                .field("LADSEL", &self.LADSEL())
                .field("LACC", &self.LACC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CLCR {{ LGO: {=bool:?}, CACHEADDR: {=u16:?}, WSEL: {=bool:?}, TDSEL: {=bool:?}, LCIVB: {=bool:?}, LCIMB: {=bool:?}, LCWAY: {=bool:?}, LCMD: {=u8:?}, LADSEL: {=bool:?}, LACC: {=bool:?} }}" , self . LGO () , self . CACHEADDR () , self . WSEL () , self . TDSEL () , self . LCIVB () , self . LCIMB () , self . LCWAY () , self . LCMD () , self . LADSEL () , self . LACC ())
        }
    }
    #[doc = "Cache Search Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSAR(pub u32);
    impl CSAR {
        #[must_use]
        #[inline(always)]
        pub const fn LGO(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LGO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PHYADDR(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_PHYADDR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for CSAR {
        #[inline(always)]
        fn default() -> CSAR {
            CSAR(0)
        }
    }
    impl core::fmt::Debug for CSAR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSAR")
                .field("LGO", &self.LGO())
                .field("PHYADDR", &self.PHYADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSAR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CSAR {{ LGO: {=bool:?}, PHYADDR: {=u32:?} }}",
                self.LGO(),
                self.PHYADDR()
            )
        }
    }
}
