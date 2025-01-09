#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WWDT {
    ptr: *mut u8,
}
unsafe impl Send for WWDT {}
unsafe impl Sync for WWDT {}
impl WWDT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MOD(self) -> crate::common::Reg<regs::MOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn TC(self) -> crate::common::Reg<regs::TC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn FEED(self) -> crate::common::Reg<regs::FEED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn TV(self) -> crate::common::Reg<regs::TV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn WARNINT(self) -> crate::common::Reg<regs::WARNINT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn WINDOW(self) -> crate::common::Reg<regs::WINDOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Feed Sequence"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FEED(pub u32);
    impl FEED {
        #[inline(always)]
        pub const fn FEED(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FEED(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for FEED {
        #[inline(always)]
        fn default() -> FEED {
            FEED(0)
        }
    }
    impl core::fmt::Debug for FEED {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FEED").field("FEED", &self.FEED()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FEED {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FEED {
                FEED: u8,
            }
            let proxy = FEED { FEED: self.FEED() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MOD(pub u32);
    impl MOD {
        #[inline(always)]
        pub const fn WDEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WDRESET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WDTOF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDTOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WDINT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WDPROTECT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WDPROTECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DEBUG_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEBUG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for MOD {
        #[inline(always)]
        fn default() -> MOD {
            MOD(0)
        }
    }
    impl core::fmt::Debug for MOD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MOD")
                .field("WDEN", &self.WDEN())
                .field("WDRESET", &self.WDRESET())
                .field("WDTOF", &self.WDTOF())
                .field("WDINT", &self.WDINT())
                .field("WDPROTECT", &self.WDPROTECT())
                .field("LOCK", &self.LOCK())
                .field("DEBUG_EN", &self.DEBUG_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MOD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MOD {
                WDEN: bool,
                WDRESET: bool,
                WDTOF: bool,
                WDINT: bool,
                WDPROTECT: bool,
                LOCK: bool,
                DEBUG_EN: bool,
            }
            let proxy = MOD {
                WDEN: self.WDEN(),
                WDRESET: self.WDRESET(),
                WDTOF: self.WDTOF(),
                WDINT: self.WDINT(),
                WDPROTECT: self.WDPROTECT(),
                LOCK: self.LOCK(),
                DEBUG_EN: self.DEBUG_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timer Constant"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TC(pub u32);
    impl TC {
        #[inline(always)]
        pub const fn COUNT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for TC {
        #[inline(always)]
        fn default() -> TC {
            TC(0)
        }
    }
    impl core::fmt::Debug for TC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TC").field("COUNT", &self.COUNT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TC {
                COUNT: u32,
            }
            let proxy = TC {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timer Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TV(pub u32);
    impl TV {
        #[inline(always)]
        pub const fn COUNT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for TV {
        #[inline(always)]
        fn default() -> TV {
            TV(0)
        }
    }
    impl core::fmt::Debug for TV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TV").field("COUNT", &self.COUNT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TV {
                COUNT: u32,
            }
            let proxy = TV {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Warning Interrupt Compare Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WARNINT(pub u32);
    impl WARNINT {
        #[inline(always)]
        pub const fn WARNINT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_WARNINT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for WARNINT {
        #[inline(always)]
        fn default() -> WARNINT {
            WARNINT(0)
        }
    }
    impl core::fmt::Debug for WARNINT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WARNINT")
                .field("WARNINT", &self.WARNINT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WARNINT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WARNINT {
                WARNINT: u16,
            }
            let proxy = WARNINT {
                WARNINT: self.WARNINT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Window Compare Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WINDOW(pub u32);
    impl WINDOW {
        #[inline(always)]
        pub const fn WINDOW(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_WINDOW(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for WINDOW {
        #[inline(always)]
        fn default() -> WINDOW {
            WINDOW(0)
        }
    }
    impl core::fmt::Debug for WINDOW {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WINDOW")
                .field("WINDOW", &self.WINDOW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WINDOW {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WINDOW {
                WINDOW: u32,
            }
            let proxy = WINDOW {
                WINDOW: self.WINDOW(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
