#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UTICK {
    ptr: *mut u8,
}
unsafe impl Send for UTICK {}
unsafe impl Sync for UTICK {}
impl UTICK {
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
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPCLR(self) -> crate::common::Reg<regs::CAPCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAP(self, n: usize) -> crate::common::Reg<regs::CAP, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Capture"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAP(pub u32);
    impl CAP {
        #[inline(always)]
        pub const fn CAP_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CAP_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn VALID(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CAP {
        #[inline(always)]
        fn default() -> CAP {
            CAP(0)
        }
    }
    impl core::fmt::Debug for CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAP")
                .field("CAP_VALUE", &self.CAP_VALUE())
                .field("VALID", &self.VALID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CAP {{ CAP_VALUE: {=u32:?}, VALID: {=bool:?} }}",
                self.CAP_VALUE(),
                self.VALID()
            )
        }
    }
    #[doc = "Capture Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPCLR(pub u32);
    impl CAPCLR {
        #[inline(always)]
        pub const fn CAPCLR0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPCLR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CAPCLR1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPCLR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CAPCLR2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPCLR2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CAPCLR3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPCLR3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for CAPCLR {
        #[inline(always)]
        fn default() -> CAPCLR {
            CAPCLR(0)
        }
    }
    impl core::fmt::Debug for CAPCLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAPCLR")
                .field("CAPCLR0", &self.CAPCLR0())
                .field("CAPCLR1", &self.CAPCLR1())
                .field("CAPCLR2", &self.CAPCLR2())
                .field("CAPCLR3", &self.CAPCLR3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAPCLR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CAPCLR {{ CAPCLR0: {=bool:?}, CAPCLR1: {=bool:?}, CAPCLR2: {=bool:?}, CAPCLR3: {=bool:?} }}" , self . CAPCLR0 () , self . CAPCLR1 () , self . CAPCLR2 () , self . CAPCLR3 ())
        }
    }
    #[doc = "Capture Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG(pub u32);
    impl CFG {
        #[inline(always)]
        pub const fn CAPEN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPEN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CAPEN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPEN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CAPEN2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPEN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CAPEN3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPEN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CAPPOL0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPPOL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CAPPOL1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPPOL1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CAPPOL2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPPOL2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CAPPOL3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAPPOL3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for CFG {
        #[inline(always)]
        fn default() -> CFG {
            CFG(0)
        }
    }
    impl core::fmt::Debug for CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFG")
                .field("CAPEN0", &self.CAPEN0())
                .field("CAPEN1", &self.CAPEN1())
                .field("CAPEN2", &self.CAPEN2())
                .field("CAPEN3", &self.CAPEN3())
                .field("CAPPOL0", &self.CAPPOL0())
                .field("CAPPOL1", &self.CAPPOL1())
                .field("CAPPOL2", &self.CAPPOL2())
                .field("CAPPOL3", &self.CAPPOL3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CFG {{ CAPEN0: {=bool:?}, CAPEN1: {=bool:?}, CAPEN2: {=bool:?}, CAPEN3: {=bool:?}, CAPPOL0: {=bool:?}, CAPPOL1: {=bool:?}, CAPPOL2: {=bool:?}, CAPPOL3: {=bool:?} }}" , self . CAPEN0 () , self . CAPEN1 () , self . CAPEN2 () , self . CAPEN3 () , self . CAPPOL0 () , self . CAPPOL1 () , self . CAPPOL2 () , self . CAPPOL3 ())
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn DELAYVAL(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_DELAYVAL(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REPEAT(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REPEAT(&mut self, val: bool) {
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
                .field("DELAYVAL", &self.DELAYVAL())
                .field("REPEAT", &self.REPEAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CTRL {{ DELAYVAL: {=u32:?}, REPEAT: {=bool:?} }}",
                self.DELAYVAL(),
                self.REPEAT()
            )
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[inline(always)]
        pub const fn INTR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ACTIVE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("INTR", &self.INTR())
                .field("ACTIVE", &self.ACTIVE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "STAT {{ INTR: {=bool:?}, ACTIVE: {=bool:?} }}",
                self.INTR(),
                self.ACTIVE()
            )
        }
    }
}
