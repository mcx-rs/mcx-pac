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
    pub const fn EDFR(self) -> crate::common::Reg<regs::EDFR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn EDIER(self) -> crate::common::Reg<regs::EDIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn EDCR(self) -> crate::common::Reg<regs::EDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
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
    #[doc = "EFT Detect Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EDCR(pub u32);
    impl EDCR {
        #[inline(always)]
        pub const fn EDHC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDHC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EDLC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDLC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for EDCR {
        #[inline(always)]
        fn default() -> EDCR {
            EDCR(0)
        }
    }
    impl core::fmt::Debug for EDCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EDCR")
                .field("EDHC", &self.EDHC())
                .field("EDLC", &self.EDLC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EDCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EDCR {
                EDHC: bool,
                EDLC: bool,
            }
            let proxy = EDCR {
                EDHC: self.EDHC(),
                EDLC: self.EDLC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EFT Detect Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EDFR(pub u32);
    impl EDFR {
        #[inline(always)]
        pub const fn EDF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EDF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EDF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn EDF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn EDF4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EDF5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn EDF6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn EDF7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn EDF8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn EDF9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn EDF10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn EDF11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn EDF12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EDF13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EDF14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn EDF15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn EDF16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn EDF17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn EDF18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn EDF19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EDF20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn EDF21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn EDF22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn EDF23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn EDF24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn EDF25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn EDF26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn EDF27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn EDF28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn EDF29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn EDF30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDF31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDF31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EDFR {
        #[inline(always)]
        fn default() -> EDFR {
            EDFR(0)
        }
    }
    impl core::fmt::Debug for EDFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EDFR")
                .field("EDF0", &self.EDF0())
                .field("EDF1", &self.EDF1())
                .field("EDF2", &self.EDF2())
                .field("EDF3", &self.EDF3())
                .field("EDF4", &self.EDF4())
                .field("EDF5", &self.EDF5())
                .field("EDF6", &self.EDF6())
                .field("EDF7", &self.EDF7())
                .field("EDF8", &self.EDF8())
                .field("EDF9", &self.EDF9())
                .field("EDF10", &self.EDF10())
                .field("EDF11", &self.EDF11())
                .field("EDF12", &self.EDF12())
                .field("EDF13", &self.EDF13())
                .field("EDF14", &self.EDF14())
                .field("EDF15", &self.EDF15())
                .field("EDF16", &self.EDF16())
                .field("EDF17", &self.EDF17())
                .field("EDF18", &self.EDF18())
                .field("EDF19", &self.EDF19())
                .field("EDF20", &self.EDF20())
                .field("EDF21", &self.EDF21())
                .field("EDF22", &self.EDF22())
                .field("EDF23", &self.EDF23())
                .field("EDF24", &self.EDF24())
                .field("EDF25", &self.EDF25())
                .field("EDF26", &self.EDF26())
                .field("EDF27", &self.EDF27())
                .field("EDF28", &self.EDF28())
                .field("EDF29", &self.EDF29())
                .field("EDF30", &self.EDF30())
                .field("EDF31", &self.EDF31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EDFR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EDFR {
                EDF0: bool,
                EDF1: bool,
                EDF2: bool,
                EDF3: bool,
                EDF4: bool,
                EDF5: bool,
                EDF6: bool,
                EDF7: bool,
                EDF8: bool,
                EDF9: bool,
                EDF10: bool,
                EDF11: bool,
                EDF12: bool,
                EDF13: bool,
                EDF14: bool,
                EDF15: bool,
                EDF16: bool,
                EDF17: bool,
                EDF18: bool,
                EDF19: bool,
                EDF20: bool,
                EDF21: bool,
                EDF22: bool,
                EDF23: bool,
                EDF24: bool,
                EDF25: bool,
                EDF26: bool,
                EDF27: bool,
                EDF28: bool,
                EDF29: bool,
                EDF30: bool,
                EDF31: bool,
            }
            let proxy = EDFR {
                EDF0: self.EDF0(),
                EDF1: self.EDF1(),
                EDF2: self.EDF2(),
                EDF3: self.EDF3(),
                EDF4: self.EDF4(),
                EDF5: self.EDF5(),
                EDF6: self.EDF6(),
                EDF7: self.EDF7(),
                EDF8: self.EDF8(),
                EDF9: self.EDF9(),
                EDF10: self.EDF10(),
                EDF11: self.EDF11(),
                EDF12: self.EDF12(),
                EDF13: self.EDF13(),
                EDF14: self.EDF14(),
                EDF15: self.EDF15(),
                EDF16: self.EDF16(),
                EDF17: self.EDF17(),
                EDF18: self.EDF18(),
                EDF19: self.EDF19(),
                EDF20: self.EDF20(),
                EDF21: self.EDF21(),
                EDF22: self.EDF22(),
                EDF23: self.EDF23(),
                EDF24: self.EDF24(),
                EDF25: self.EDF25(),
                EDF26: self.EDF26(),
                EDF27: self.EDF27(),
                EDF28: self.EDF28(),
                EDF29: self.EDF29(),
                EDF30: self.EDF30(),
                EDF31: self.EDF31(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EDIER(pub u32);
    impl EDIER {
        #[inline(always)]
        pub const fn EDIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EDIE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EDIE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn EDIE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn EDIE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EDIE5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn EDIE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn EDIE7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn EDIE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn EDIE9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn EDIE10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn EDIE11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn EDIE12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EDIE13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EDIE14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn EDIE15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn EDIE16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn EDIE17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn EDIE18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn EDIE19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EDIE20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn EDIE21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn EDIE22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn EDIE23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn EDIE24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn EDIE25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn EDIE26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn EDIE27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn EDIE28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn EDIE29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn EDIE30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EDIE31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EDIE31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EDIER {
        #[inline(always)]
        fn default() -> EDIER {
            EDIER(0)
        }
    }
    impl core::fmt::Debug for EDIER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EDIER")
                .field("EDIE0", &self.EDIE0())
                .field("EDIE1", &self.EDIE1())
                .field("EDIE2", &self.EDIE2())
                .field("EDIE3", &self.EDIE3())
                .field("EDIE4", &self.EDIE4())
                .field("EDIE5", &self.EDIE5())
                .field("EDIE6", &self.EDIE6())
                .field("EDIE7", &self.EDIE7())
                .field("EDIE8", &self.EDIE8())
                .field("EDIE9", &self.EDIE9())
                .field("EDIE10", &self.EDIE10())
                .field("EDIE11", &self.EDIE11())
                .field("EDIE12", &self.EDIE12())
                .field("EDIE13", &self.EDIE13())
                .field("EDIE14", &self.EDIE14())
                .field("EDIE15", &self.EDIE15())
                .field("EDIE16", &self.EDIE16())
                .field("EDIE17", &self.EDIE17())
                .field("EDIE18", &self.EDIE18())
                .field("EDIE19", &self.EDIE19())
                .field("EDIE20", &self.EDIE20())
                .field("EDIE21", &self.EDIE21())
                .field("EDIE22", &self.EDIE22())
                .field("EDIE23", &self.EDIE23())
                .field("EDIE24", &self.EDIE24())
                .field("EDIE25", &self.EDIE25())
                .field("EDIE26", &self.EDIE26())
                .field("EDIE27", &self.EDIE27())
                .field("EDIE28", &self.EDIE28())
                .field("EDIE29", &self.EDIE29())
                .field("EDIE30", &self.EDIE30())
                .field("EDIE31", &self.EDIE31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EDIER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EDIER {
                EDIE0: bool,
                EDIE1: bool,
                EDIE2: bool,
                EDIE3: bool,
                EDIE4: bool,
                EDIE5: bool,
                EDIE6: bool,
                EDIE7: bool,
                EDIE8: bool,
                EDIE9: bool,
                EDIE10: bool,
                EDIE11: bool,
                EDIE12: bool,
                EDIE13: bool,
                EDIE14: bool,
                EDIE15: bool,
                EDIE16: bool,
                EDIE17: bool,
                EDIE18: bool,
                EDIE19: bool,
                EDIE20: bool,
                EDIE21: bool,
                EDIE22: bool,
                EDIE23: bool,
                EDIE24: bool,
                EDIE25: bool,
                EDIE26: bool,
                EDIE27: bool,
                EDIE28: bool,
                EDIE29: bool,
                EDIE30: bool,
                EDIE31: bool,
            }
            let proxy = EDIER {
                EDIE0: self.EDIE0(),
                EDIE1: self.EDIE1(),
                EDIE2: self.EDIE2(),
                EDIE3: self.EDIE3(),
                EDIE4: self.EDIE4(),
                EDIE5: self.EDIE5(),
                EDIE6: self.EDIE6(),
                EDIE7: self.EDIE7(),
                EDIE8: self.EDIE8(),
                EDIE9: self.EDIE9(),
                EDIE10: self.EDIE10(),
                EDIE11: self.EDIE11(),
                EDIE12: self.EDIE12(),
                EDIE13: self.EDIE13(),
                EDIE14: self.EDIE14(),
                EDIE15: self.EDIE15(),
                EDIE16: self.EDIE16(),
                EDIE17: self.EDIE17(),
                EDIE18: self.EDIE18(),
                EDIE19: self.EDIE19(),
                EDIE20: self.EDIE20(),
                EDIE21: self.EDIE21(),
                EDIE22: self.EDIE22(),
                EDIE23: self.EDIE23(),
                EDIE24: self.EDIE24(),
                EDIE25: self.EDIE25(),
                EDIE26: self.EDIE26(),
                EDIE27: self.EDIE27(),
                EDIE28: self.EDIE28(),
                EDIE29: self.EDIE29(),
                EDIE30: self.EDIE30(),
                EDIE31: self.EDIE31(),
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
