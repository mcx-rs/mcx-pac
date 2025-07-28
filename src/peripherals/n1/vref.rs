#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VREF {
    ptr: *mut u8,
}
unsafe impl Send for VREF {}
unsafe impl Sync for VREF {}
impl VREF {
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
    pub const fn CSR(self) -> crate::common::Reg<regs::CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn UTRIM(self) -> crate::common::Reg<regs::UTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[must_use]
        #[inline(always)]
        pub const fn HCBGEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HCBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPBGEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPBG_BUF_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPBG_BUF_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHOPEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHOPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ICOMPEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ICOMPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REGEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HI_PWR_LV(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HI_PWR_LV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BUF21EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUF21EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREFST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VREFST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CSR {
        #[inline(always)]
        fn default() -> CSR {
            CSR(0)
        }
    }
    impl core::fmt::Debug for CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSR")
                .field("HCBGEN", &self.HCBGEN())
                .field("LPBGEN", &self.LPBGEN())
                .field("LPBG_BUF_EN", &self.LPBG_BUF_EN())
                .field("CHOPEN", &self.CHOPEN())
                .field("ICOMPEN", &self.ICOMPEN())
                .field("REGEN", &self.REGEN())
                .field("HI_PWR_LV", &self.HI_PWR_LV())
                .field("BUF21EN", &self.BUF21EN())
                .field("VREFST", &self.VREFST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CSR {{ HCBGEN: {=bool:?}, LPBGEN: {=bool:?}, LPBG_BUF_EN: {=bool:?}, CHOPEN: {=bool:?}, ICOMPEN: {=bool:?}, REGEN: {=bool:?}, HI_PWR_LV: {=bool:?}, BUF21EN: {=bool:?}, VREFST: {=bool:?} }}" , self . HCBGEN () , self . LPBGEN () , self . LPBG_BUF_EN () , self . CHOPEN () , self . ICOMPEN () , self . REGEN () , self . HI_PWR_LV () , self . BUF21EN () , self . VREFST ())
        }
    }
    #[doc = "User Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UTRIM(pub u32);
    impl UTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn TRIM2V1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIM2V1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREFTRIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VREFTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for UTRIM {
        #[inline(always)]
        fn default() -> UTRIM {
            UTRIM(0)
        }
    }
    impl core::fmt::Debug for UTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UTRIM")
                .field("TRIM2V1", &self.TRIM2V1())
                .field("VREFTRIM", &self.VREFTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UTRIM {{ TRIM2V1: {=u8:?}, VREFTRIM: {=u8:?} }}",
                self.TRIM2V1(),
                self.VREFTRIM()
            )
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
