#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VBAT {
    ptr: *mut u8,
}
unsafe impl Send for VBAT {}
unsafe impl Sync for VBAT {}
impl VBAT {
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
    pub const fn FROCTLA(self) -> crate::common::Reg<regs::FROCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn FROLCKA(self) -> crate::common::Reg<regs::FROLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FROCLKE(self) -> crate::common::Reg<regs::FROCLKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKEUP(self, n: usize) -> WAKEUP {
        assert!(n < 2usize);
        unsafe { WAKEUP::from_ptr(self.ptr.add(0x0700usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKLCKA(self) -> crate::common::Reg<regs::WAKLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEUP {
    ptr: *mut u8,
}
unsafe impl Send for WAKEUP {}
unsafe impl Sync for WAKEUP {}
impl WAKEUP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn WAKEUPA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "FRO16K Clock Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCLKE(pub u32);
    impl FROCLKE {
        #[inline(always)]
        pub const fn CLKE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for FROCLKE {
        #[inline(always)]
        fn default() -> FROCLKE {
            FROCLKE(0)
        }
    }
    impl core::fmt::Debug for FROCLKE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROCLKE")
                .field("CLKE", &self.CLKE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROCLKE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROCLKE {{ CLKE: {=u8:?} }}", self.CLKE())
        }
    }
    #[doc = "FRO16K Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCTLA(pub u32);
    impl FROCTLA {
        #[inline(always)]
        pub const fn FRO_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROCTLA {
        #[inline(always)]
        fn default() -> FROCTLA {
            FROCTLA(0)
        }
    }
    impl core::fmt::Debug for FROCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROCTLA")
                .field("FRO_EN", &self.FRO_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROCTLA {{ FRO_EN: {=bool:?} }}", self.FRO_EN())
        }
    }
    #[doc = "FRO16K Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROLCKA(pub u32);
    impl FROLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROLCKA {
        #[inline(always)]
        fn default() -> FROLCKA {
            FROLCKA(0)
        }
    }
    impl core::fmt::Debug for FROLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
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
    #[doc = "Wakeup Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKLCKA(pub u32);
    impl WAKLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WAKLCKA {
        #[inline(always)]
        fn default() -> WAKLCKA {
            WAKLCKA(0)
        }
    }
    impl core::fmt::Debug for WAKLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WAKLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
}
