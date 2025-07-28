#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMU {
    ptr: *mut u8,
}
unsafe impl Send for FMU {}
unsafe impl Sync for FMU {}
impl FMU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn FSTAT(self) -> crate::common::Reg<regs::FSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn FCNFG(self) -> crate::common::Reg<regs::FCNFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn FCTRL(self) -> crate::common::Reg<regs::FCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCNFG(pub u32);
    impl FCNFG {
        #[must_use]
        #[inline(always)]
        pub const fn CCIE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSREQ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERSREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFDIE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSIEN0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERSIEN0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSIEN1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERSIEN1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FCNFG {
        #[inline(always)]
        fn default() -> FCNFG {
            FCNFG(0)
        }
    }
    impl core::fmt::Debug for FCNFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCNFG")
                .field("CCIE", &self.CCIE())
                .field("ERSREQ", &self.ERSREQ())
                .field("DFDIE", &self.DFDIE())
                .field("ERSIEN0", &self.ERSIEN0())
                .field("ERSIEN1", &self.ERSIEN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCNFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FCNFG {{ CCIE: {=bool:?}, ERSREQ: {=bool:?}, DFDIE: {=bool:?}, ERSIEN0: {=u8:?}, ERSIEN1: {=u8:?} }}" , self . CCIE () , self . ERSREQ () , self . DFDIE () , self . ERSIEN0 () , self . ERSIEN1 ())
        }
    }
    #[doc = "Flash Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u32);
    impl FCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RWSC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RWSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LSACTIVE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LSACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDFD(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDFD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ABTREQ(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ABTREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for FCTRL {
        #[inline(always)]
        fn default() -> FCTRL {
            FCTRL(0)
        }
    }
    impl core::fmt::Debug for FCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTRL")
                .field("RWSC", &self.RWSC())
                .field("LSACTIVE", &self.LSACTIVE())
                .field("FDFD", &self.FDFD())
                .field("ABTREQ", &self.ABTREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FCTRL {{ RWSC: {=u8:?}, LSACTIVE: {=bool:?}, FDFD: {=bool:?}, ABTREQ: {=bool:?} }}" , self . RWSC () , self . LSACTIVE () , self . FDFD () , self . ABTREQ ())
        }
    }
    #[doc = "Flash Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FSTAT(pub u32);
    impl FSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn FAIL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDABT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMDABT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PVIOL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PVIOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACCERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWSABT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWSABT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCIF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDPRT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDPRT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDP(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMDP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDDID(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDDID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFDIF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFDIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SALV_USED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SALV_USED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PEWEN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PEWEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PERDY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PERDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FSTAT {
        #[inline(always)]
        fn default() -> FSTAT {
            FSTAT(0)
        }
    }
    impl core::fmt::Debug for FSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FSTAT")
                .field("FAIL", &self.FAIL())
                .field("CMDABT", &self.CMDABT())
                .field("PVIOL", &self.PVIOL())
                .field("ACCERR", &self.ACCERR())
                .field("CWSABT", &self.CWSABT())
                .field("CCIF", &self.CCIF())
                .field("CMDPRT", &self.CMDPRT())
                .field("CMDP", &self.CMDP())
                .field("CMDDID", &self.CMDDID())
                .field("DFDIF", &self.DFDIF())
                .field("SALV_USED", &self.SALV_USED())
                .field("PEWEN", &self.PEWEN())
                .field("PERDY", &self.PERDY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FSTAT {{ FAIL: {=bool:?}, CMDABT: {=bool:?}, PVIOL: {=bool:?}, ACCERR: {=bool:?}, CWSABT: {=bool:?}, CCIF: {=bool:?}, CMDPRT: {=u8:?}, CMDP: {=bool:?}, CMDDID: {=u8:?}, DFDIF: {=bool:?}, SALV_USED: {=bool:?}, PEWEN: {=u8:?}, PERDY: {=bool:?} }}" , self . FAIL () , self . CMDABT () , self . PVIOL () , self . ACCERR () , self . CWSABT () , self . CCIF () , self . CMDPRT () , self . CMDP () , self . CMDDID () , self . DFDIF () , self . SALV_USED () , self . PEWEN () , self . PERDY ())
        }
    }
}
