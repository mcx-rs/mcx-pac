#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMC {
    ptr: *mut u8,
}
unsafe impl Send for FMC {}
unsafe impl Sync for FMC {}
impl FMC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn REMAP(self) -> crate::common::Reg<regs::REMAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCR(self) -> crate::common::Reg<regs::FCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn FCAR(self) -> crate::common::Reg<regs::FCAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn FCTG(self) -> crate::common::Reg<regs::FCTG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[inline(always)]
    pub const fn FCLN0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn FCLN1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[inline(always)]
    pub const fn FCLN2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[inline(always)]
    pub const fn FCLN3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash Cache Access Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCAR(pub u32);
    impl FCAR {
        #[inline(always)]
        pub const fn CACHES_WAY_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CACHES_WAY_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CACHES_SET_NUM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CACHES_SET_NUM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TYPE(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for FCAR {
        #[inline(always)]
        fn default() -> FCAR {
            FCAR(0)
        }
    }
    impl core::fmt::Debug for FCAR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCAR")
                .field("CACHES_WAY_NUM", &self.CACHES_WAY_NUM())
                .field("CACHES_SET_NUM", &self.CACHES_SET_NUM())
                .field("TYPE", &self.TYPE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCAR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FCAR {{ CACHES_WAY_NUM: {=u8:?}, CACHES_SET_NUM: {=bool:?}, TYPE: {=u8:?} }}",
                self.CACHES_WAY_NUM(),
                self.CACHES_SET_NUM(),
                self.TYPE()
            )
        }
    }
    #[doc = "Flash Cache Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCCR(pub u32);
    impl FCCR {
        #[inline(always)]
        pub const fn WAY_LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_WAY_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FCCR {
        #[inline(always)]
        fn default() -> FCCR {
            FCCR(0)
        }
    }
    impl core::fmt::Debug for FCCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCCR")
                .field("WAY_LOCK", &self.WAY_LOCK())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FCCR {{ WAY_LOCK: {=u8:?}, LOCK: {=bool:?} }}",
                self.WAY_LOCK(),
                self.LOCK()
            )
        }
    }
    #[doc = "Flash Cache Tag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTG(pub u32);
    impl FCTG {
        #[inline(always)]
        pub const fn VALID(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ADDRESS(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x0003_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ADDRESS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 6usize)) | (((val as u32) & 0x0003_ffff) << 6usize);
        }
    }
    impl Default for FCTG {
        #[inline(always)]
        fn default() -> FCTG {
            FCTG(0)
        }
    }
    impl core::fmt::Debug for FCTG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTG")
                .field("VALID", &self.VALID())
                .field("ADDRESS", &self.ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FCTG {{ VALID: {=bool:?}, ADDRESS: {=u32:?} }}",
                self.VALID(),
                self.ADDRESS()
            )
        }
    }
    #[doc = "Data Remap"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REMAP(pub u32);
    impl REMAP {
        #[inline(always)]
        pub const fn REMAPLK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REMAPLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LIM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[inline(always)]
        pub const fn LIMDP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LIMDP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for REMAP {
        #[inline(always)]
        fn default() -> REMAP {
            REMAP(0)
        }
    }
    impl core::fmt::Debug for REMAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REMAP")
                .field("REMAPLK", &self.REMAPLK())
                .field("LIM", &self.LIM())
                .field("LIMDP", &self.LIMDP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REMAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REMAP {{ REMAPLK: {=bool:?}, LIM: {=u8:?}, LIMDP: {=u8:?} }}",
                self.REMAPLK(),
                self.LIM(),
                self.LIMDP()
            )
        }
    }
}
