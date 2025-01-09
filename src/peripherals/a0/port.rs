#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORT {
    ptr: *mut u8,
}
unsafe impl Send for PORT {}
unsafe impl Sync for PORT {}
impl PORT {
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
    pub const fn GPCLR(self) -> crate::common::Reg<regs::GPCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn GPCHR(self) -> crate::common::Reg<regs::GPCHR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CALIB0(self) -> crate::common::Reg<regs::CALIB0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn CALIB1(self) -> crate::common::Reg<regs::CALIB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn PCR(self, n: usize) -> crate::common::Reg<regs::PCR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Calibration 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CALIB0(pub u32);
    impl CALIB0 {
        #[inline(always)]
        pub const fn NCAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NCAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn PCAL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PCAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for CALIB0 {
        #[inline(always)]
        fn default() -> CALIB0 {
            CALIB0(0)
        }
    }
    impl core::fmt::Debug for CALIB0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CALIB0")
                .field("NCAL", &self.NCAL())
                .field("PCAL", &self.PCAL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CALIB0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CALIB0 {
                NCAL: u8,
                PCAL: u8,
            }
            let proxy = CALIB0 {
                NCAL: self.NCAL(),
                PCAL: self.PCAL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Calibration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CALIB1(pub u32);
    impl CALIB1 {
        #[inline(always)]
        pub const fn NCAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NCAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn PCAL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PCAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for CALIB1 {
        #[inline(always)]
        fn default() -> CALIB1 {
            CALIB1(0)
        }
    }
    impl core::fmt::Debug for CALIB1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CALIB1")
                .field("NCAL", &self.NCAL())
                .field("PCAL", &self.PCAL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CALIB1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CALIB1 {
                NCAL: u8,
                PCAL: u8,
            }
            let proxy = CALIB1 {
                NCAL: self.NCAL(),
                PCAL: self.PCAL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIG(pub u32);
    impl CONFIG {
        #[inline(always)]
        pub const fn RANGE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CONFIG {
        #[inline(always)]
        fn default() -> CONFIG {
            CONFIG(0)
        }
    }
    impl core::fmt::Debug for CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIG")
                .field("RANGE", &self.RANGE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONFIG {
                RANGE: bool,
            }
            let proxy = CONFIG {
                RANGE: self.RANGE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Global Pin Control High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPCHR(pub u32);
    impl GPCHR {
        #[inline(always)]
        pub const fn GPWD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GPWD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn GPWE16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn GPWE17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn GPWE18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn GPWE19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn GPWE20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn GPWE21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn GPWE22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn GPWE23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn GPWE24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn GPWE25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn GPWE26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn GPWE27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn GPWE28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn GPWE29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn GPWE30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn GPWE31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GPCHR {
        #[inline(always)]
        fn default() -> GPCHR {
            GPCHR(0)
        }
    }
    impl core::fmt::Debug for GPCHR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPCHR")
                .field("GPWD", &self.GPWD())
                .field("GPWE16", &self.GPWE16())
                .field("GPWE17", &self.GPWE17())
                .field("GPWE18", &self.GPWE18())
                .field("GPWE19", &self.GPWE19())
                .field("GPWE20", &self.GPWE20())
                .field("GPWE21", &self.GPWE21())
                .field("GPWE22", &self.GPWE22())
                .field("GPWE23", &self.GPWE23())
                .field("GPWE24", &self.GPWE24())
                .field("GPWE25", &self.GPWE25())
                .field("GPWE26", &self.GPWE26())
                .field("GPWE27", &self.GPWE27())
                .field("GPWE28", &self.GPWE28())
                .field("GPWE29", &self.GPWE29())
                .field("GPWE30", &self.GPWE30())
                .field("GPWE31", &self.GPWE31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPCHR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPCHR {
                GPWD: u16,
                GPWE16: bool,
                GPWE17: bool,
                GPWE18: bool,
                GPWE19: bool,
                GPWE20: bool,
                GPWE21: bool,
                GPWE22: bool,
                GPWE23: bool,
                GPWE24: bool,
                GPWE25: bool,
                GPWE26: bool,
                GPWE27: bool,
                GPWE28: bool,
                GPWE29: bool,
                GPWE30: bool,
                GPWE31: bool,
            }
            let proxy = GPCHR {
                GPWD: self.GPWD(),
                GPWE16: self.GPWE16(),
                GPWE17: self.GPWE17(),
                GPWE18: self.GPWE18(),
                GPWE19: self.GPWE19(),
                GPWE20: self.GPWE20(),
                GPWE21: self.GPWE21(),
                GPWE22: self.GPWE22(),
                GPWE23: self.GPWE23(),
                GPWE24: self.GPWE24(),
                GPWE25: self.GPWE25(),
                GPWE26: self.GPWE26(),
                GPWE27: self.GPWE27(),
                GPWE28: self.GPWE28(),
                GPWE29: self.GPWE29(),
                GPWE30: self.GPWE30(),
                GPWE31: self.GPWE31(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Global Pin Control Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPCLR(pub u32);
    impl GPCLR {
        #[inline(always)]
        pub const fn GPWD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GPWD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn GPWE0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn GPWE1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn GPWE2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn GPWE3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn GPWE4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn GPWE5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn GPWE6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn GPWE7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn GPWE8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn GPWE9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn GPWE10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn GPWE11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn GPWE12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn GPWE13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn GPWE14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn GPWE15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPWE15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GPCLR {
        #[inline(always)]
        fn default() -> GPCLR {
            GPCLR(0)
        }
    }
    impl core::fmt::Debug for GPCLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPCLR")
                .field("GPWD", &self.GPWD())
                .field("GPWE0", &self.GPWE0())
                .field("GPWE1", &self.GPWE1())
                .field("GPWE2", &self.GPWE2())
                .field("GPWE3", &self.GPWE3())
                .field("GPWE4", &self.GPWE4())
                .field("GPWE5", &self.GPWE5())
                .field("GPWE6", &self.GPWE6())
                .field("GPWE7", &self.GPWE7())
                .field("GPWE8", &self.GPWE8())
                .field("GPWE9", &self.GPWE9())
                .field("GPWE10", &self.GPWE10())
                .field("GPWE11", &self.GPWE11())
                .field("GPWE12", &self.GPWE12())
                .field("GPWE13", &self.GPWE13())
                .field("GPWE14", &self.GPWE14())
                .field("GPWE15", &self.GPWE15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPCLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GPCLR {
                GPWD: u16,
                GPWE0: bool,
                GPWE1: bool,
                GPWE2: bool,
                GPWE3: bool,
                GPWE4: bool,
                GPWE5: bool,
                GPWE6: bool,
                GPWE7: bool,
                GPWE8: bool,
                GPWE9: bool,
                GPWE10: bool,
                GPWE11: bool,
                GPWE12: bool,
                GPWE13: bool,
                GPWE14: bool,
                GPWE15: bool,
            }
            let proxy = GPCLR {
                GPWD: self.GPWD(),
                GPWE0: self.GPWE0(),
                GPWE1: self.GPWE1(),
                GPWE2: self.GPWE2(),
                GPWE3: self.GPWE3(),
                GPWE4: self.GPWE4(),
                GPWE5: self.GPWE5(),
                GPWE6: self.GPWE6(),
                GPWE7: self.GPWE7(),
                GPWE8: self.GPWE8(),
                GPWE9: self.GPWE9(),
                GPWE10: self.GPWE10(),
                GPWE11: self.GPWE11(),
                GPWE12: self.GPWE12(),
                GPWE13: self.GPWE13(),
                GPWE14: self.GPWE14(),
                GPWE15: self.GPWE15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Control 0..Pin Control 31"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCR(pub u32);
    impl PCR {
        #[inline(always)]
        pub const fn PS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PS(&mut self, val: bool) {
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
        pub const fn PV(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SRE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn PFE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ODE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DSE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DSE1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn IBE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn INV(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PCR {
        #[inline(always)]
        fn default() -> PCR {
            PCR(0)
        }
    }
    impl core::fmt::Debug for PCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCR")
                .field("PS", &self.PS())
                .field("PE", &self.PE())
                .field("PV", &self.PV())
                .field("SRE", &self.SRE())
                .field("PFE", &self.PFE())
                .field("ODE", &self.ODE())
                .field("DSE", &self.DSE())
                .field("DSE1", &self.DSE1())
                .field("MUX", &self.MUX())
                .field("IBE", &self.IBE())
                .field("INV", &self.INV())
                .field("LK", &self.LK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PCR {
                PS: bool,
                PE: bool,
                PV: bool,
                SRE: bool,
                PFE: bool,
                ODE: bool,
                DSE: bool,
                DSE1: bool,
                MUX: u8,
                IBE: bool,
                INV: bool,
                LK: bool,
            }
            let proxy = PCR {
                PS: self.PS(),
                PE: self.PE(),
                PV: self.PV(),
                SRE: self.SRE(),
                PFE: self.PFE(),
                ODE: self.ODE(),
                DSE: self.DSE(),
                DSE1: self.DSE1(),
                MUX: self.MUX(),
                IBE: self.IBE(),
                INV: self.INV(),
                LK: self.LK(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct VERID {
                FEATURE: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERID {
                FEATURE: self.FEATURE(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
