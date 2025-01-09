#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WUU {
    ptr: *mut u8,
}
unsafe impl Send for WUU {}
unsafe impl Sync for WUU {}
impl WUU {
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
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn PE1(self) -> crate::common::Reg<regs::PE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PE2(self) -> crate::common::Reg<regs::PE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn ME(self) -> crate::common::Reg<regs::ME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DE(self) -> crate::common::Reg<regs::DE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PF(self) -> crate::common::Reg<regs::PF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn FILT(self) -> crate::common::Reg<regs::FILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn PDC1(self) -> crate::common::Reg<regs::PDC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn PDC2(self) -> crate::common::Reg<regs::PDC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn FDC(self) -> crate::common::Reg<regs::FDC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn PMC(self) -> crate::common::Reg<regs::PMC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn FMC(self) -> crate::common::Reg<regs::FMC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
}
pub mod regs {
    #[doc = "Module DMA/Trigger Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DE(pub u32);
    impl DE {
        #[inline(always)]
        pub const fn WUDE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WUDE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WUDE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WUDE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WUDE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WUDE5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn WUDE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn WUDE7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WUDE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WUDE9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUDE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for DE {
        #[inline(always)]
        fn default() -> DE {
            DE(0)
        }
    }
    impl core::fmt::Debug for DE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DE")
                .field("WUDE0", &self.WUDE0())
                .field("WUDE1", &self.WUDE1())
                .field("WUDE2", &self.WUDE2())
                .field("WUDE3", &self.WUDE3())
                .field("WUDE4", &self.WUDE4())
                .field("WUDE5", &self.WUDE5())
                .field("WUDE6", &self.WUDE6())
                .field("WUDE7", &self.WUDE7())
                .field("WUDE8", &self.WUDE8())
                .field("WUDE9", &self.WUDE9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DE {
                WUDE0: bool,
                WUDE1: bool,
                WUDE2: bool,
                WUDE3: bool,
                WUDE4: bool,
                WUDE5: bool,
                WUDE6: bool,
                WUDE7: bool,
                WUDE8: bool,
                WUDE9: bool,
            }
            let proxy = DE {
                WUDE0: self.WUDE0(),
                WUDE1: self.WUDE1(),
                WUDE2: self.WUDE2(),
                WUDE3: self.WUDE3(),
                WUDE4: self.WUDE4(),
                WUDE5: self.WUDE5(),
                WUDE6: self.WUDE6(),
                WUDE7: self.WUDE7(),
                WUDE8: self.WUDE8(),
                WUDE9: self.WUDE9(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Filter DMA/Trigger Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDC(pub u32);
    impl FDC {
        #[inline(always)]
        pub const fn FILTC1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn FILTC2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTC2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for FDC {
        #[inline(always)]
        fn default() -> FDC {
            FDC(0)
        }
    }
    impl core::fmt::Debug for FDC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FDC")
                .field("FILTC1", &self.FILTC1())
                .field("FILTC2", &self.FILTC2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FDC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FDC {
                FILTC1: u8,
                FILTC2: u8,
            }
            let proxy = FDC {
                FILTC1: self.FILTC1(),
                FILTC2: self.FILTC2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FILT(pub u32);
    impl FILT {
        #[inline(always)]
        pub const fn FILTSEL1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn FILTE1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[inline(always)]
        pub const fn FILTF1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FILTSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn FILTE2(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[inline(always)]
        pub const fn FILTF2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("FILTSEL1", &self.FILTSEL1())
                .field("FILTE1", &self.FILTE1())
                .field("FILTF1", &self.FILTF1())
                .field("FILTSEL2", &self.FILTSEL2())
                .field("FILTE2", &self.FILTE2())
                .field("FILTF2", &self.FILTF2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FILT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FILT {
                FILTSEL1: u8,
                FILTE1: u8,
                FILTF1: bool,
                FILTSEL2: u8,
                FILTE2: u8,
                FILTF2: bool,
            }
            let proxy = FILT {
                FILTSEL1: self.FILTSEL1(),
                FILTE1: self.FILTE1(),
                FILTF1: self.FILTF1(),
                FILTSEL2: self.FILTSEL2(),
                FILTE2: self.FILTE2(),
                FILTF2: self.FILTF2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Filter Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FMC(pub u32);
    impl FMC {
        #[inline(always)]
        pub const fn FILTM1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FILTM2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTM2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for FMC {
        #[inline(always)]
        fn default() -> FMC {
            FMC(0)
        }
    }
    impl core::fmt::Debug for FMC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FMC")
                .field("FILTM1", &self.FILTM1())
                .field("FILTM2", &self.FILTM2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FMC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FMC {
                FILTM1: bool,
                FILTM2: bool,
            }
            let proxy = FMC {
                FILTM1: self.FILTM1(),
                FILTM2: self.FILTM2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Module Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ME(pub u32);
    impl ME {
        #[inline(always)]
        pub const fn WUME0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WUME1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WUME2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WUME3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WUME4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WUME5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn WUME6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn WUME7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WUME8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WUME9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUME9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for ME {
        #[inline(always)]
        fn default() -> ME {
            ME(0)
        }
    }
    impl core::fmt::Debug for ME {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ME")
                .field("WUME0", &self.WUME0())
                .field("WUME1", &self.WUME1())
                .field("WUME2", &self.WUME2())
                .field("WUME3", &self.WUME3())
                .field("WUME4", &self.WUME4())
                .field("WUME5", &self.WUME5())
                .field("WUME6", &self.WUME6())
                .field("WUME7", &self.WUME7())
                .field("WUME8", &self.WUME8())
                .field("WUME9", &self.WUME9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ME {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ME {
                WUME0: bool,
                WUME1: bool,
                WUME2: bool,
                WUME3: bool,
                WUME4: bool,
                WUME5: bool,
                WUME6: bool,
                WUME7: bool,
                WUME8: bool,
                WUME9: bool,
            }
            let proxy = ME {
                WUME0: self.WUME0(),
                WUME1: self.WUME1(),
                WUME2: self.WUME2(),
                WUME3: self.WUME3(),
                WUME4: self.WUME4(),
                WUME5: self.WUME5(),
                WUME6: self.WUME6(),
                WUME7: self.WUME7(),
                WUME8: self.WUME8(),
                WUME9: self.WUME9(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn FILTERS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTERS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DMAS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMAS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn MODULES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MODULES(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn PINS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    impl core::fmt::Debug for PARAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAM")
                .field("FILTERS", &self.FILTERS())
                .field("DMAS", &self.DMAS())
                .field("MODULES", &self.MODULES())
                .field("PINS", &self.PINS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PARAM {
                FILTERS: u8,
                DMAS: u8,
                MODULES: u8,
                PINS: u8,
            }
            let proxy = PARAM {
                FILTERS: self.FILTERS(),
                DMAS: self.DMAS(),
                MODULES: self.MODULES(),
                PINS: self.PINS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin DMA/Trigger Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDC1(pub u32);
    impl PDC1 {
        #[inline(always)]
        pub const fn WUPDC0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn WUPDC1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn WUPDC2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn WUPDC3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn WUPDC4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn WUPDC5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn WUPDC6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn WUPDC7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn WUPDC8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn WUPDC9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn WUPDC10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn WUPDC11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn WUPDC12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn WUPDC13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn WUPDC14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn WUPDC15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PDC1 {
        #[inline(always)]
        fn default() -> PDC1 {
            PDC1(0)
        }
    }
    impl core::fmt::Debug for PDC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDC1")
                .field("WUPDC0", &self.WUPDC0())
                .field("WUPDC1", &self.WUPDC1())
                .field("WUPDC2", &self.WUPDC2())
                .field("WUPDC3", &self.WUPDC3())
                .field("WUPDC4", &self.WUPDC4())
                .field("WUPDC5", &self.WUPDC5())
                .field("WUPDC6", &self.WUPDC6())
                .field("WUPDC7", &self.WUPDC7())
                .field("WUPDC8", &self.WUPDC8())
                .field("WUPDC9", &self.WUPDC9())
                .field("WUPDC10", &self.WUPDC10())
                .field("WUPDC11", &self.WUPDC11())
                .field("WUPDC12", &self.WUPDC12())
                .field("WUPDC13", &self.WUPDC13())
                .field("WUPDC14", &self.WUPDC14())
                .field("WUPDC15", &self.WUPDC15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDC1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PDC1 {
                WUPDC0: u8,
                WUPDC1: u8,
                WUPDC2: u8,
                WUPDC3: u8,
                WUPDC4: u8,
                WUPDC5: u8,
                WUPDC6: u8,
                WUPDC7: u8,
                WUPDC8: u8,
                WUPDC9: u8,
                WUPDC10: u8,
                WUPDC11: u8,
                WUPDC12: u8,
                WUPDC13: u8,
                WUPDC14: u8,
                WUPDC15: u8,
            }
            let proxy = PDC1 {
                WUPDC0: self.WUPDC0(),
                WUPDC1: self.WUPDC1(),
                WUPDC2: self.WUPDC2(),
                WUPDC3: self.WUPDC3(),
                WUPDC4: self.WUPDC4(),
                WUPDC5: self.WUPDC5(),
                WUPDC6: self.WUPDC6(),
                WUPDC7: self.WUPDC7(),
                WUPDC8: self.WUPDC8(),
                WUPDC9: self.WUPDC9(),
                WUPDC10: self.WUPDC10(),
                WUPDC11: self.WUPDC11(),
                WUPDC12: self.WUPDC12(),
                WUPDC13: self.WUPDC13(),
                WUPDC14: self.WUPDC14(),
                WUPDC15: self.WUPDC15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin DMA/Trigger Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDC2(pub u32);
    impl PDC2 {
        #[inline(always)]
        pub const fn WUPDC16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn WUPDC17(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn WUPDC18(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn WUPDC19(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn WUPDC20(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn WUPDC21(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn WUPDC22(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn WUPDC23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn WUPDC24(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn WUPDC25(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC25(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn WUPDC26(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn WUPDC27(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn WUPDC28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC28(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn WUPDC29(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC29(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn WUPDC30(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn WUPDC31(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPDC31(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PDC2 {
        #[inline(always)]
        fn default() -> PDC2 {
            PDC2(0)
        }
    }
    impl core::fmt::Debug for PDC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDC2")
                .field("WUPDC16", &self.WUPDC16())
                .field("WUPDC17", &self.WUPDC17())
                .field("WUPDC18", &self.WUPDC18())
                .field("WUPDC19", &self.WUPDC19())
                .field("WUPDC20", &self.WUPDC20())
                .field("WUPDC21", &self.WUPDC21())
                .field("WUPDC22", &self.WUPDC22())
                .field("WUPDC23", &self.WUPDC23())
                .field("WUPDC24", &self.WUPDC24())
                .field("WUPDC25", &self.WUPDC25())
                .field("WUPDC26", &self.WUPDC26())
                .field("WUPDC27", &self.WUPDC27())
                .field("WUPDC28", &self.WUPDC28())
                .field("WUPDC29", &self.WUPDC29())
                .field("WUPDC30", &self.WUPDC30())
                .field("WUPDC31", &self.WUPDC31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDC2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PDC2 {
                WUPDC16: u8,
                WUPDC17: u8,
                WUPDC18: u8,
                WUPDC19: u8,
                WUPDC20: u8,
                WUPDC21: u8,
                WUPDC22: u8,
                WUPDC23: u8,
                WUPDC24: u8,
                WUPDC25: u8,
                WUPDC26: u8,
                WUPDC27: u8,
                WUPDC28: u8,
                WUPDC29: u8,
                WUPDC30: u8,
                WUPDC31: u8,
            }
            let proxy = PDC2 {
                WUPDC16: self.WUPDC16(),
                WUPDC17: self.WUPDC17(),
                WUPDC18: self.WUPDC18(),
                WUPDC19: self.WUPDC19(),
                WUPDC20: self.WUPDC20(),
                WUPDC21: self.WUPDC21(),
                WUPDC22: self.WUPDC22(),
                WUPDC23: self.WUPDC23(),
                WUPDC24: self.WUPDC24(),
                WUPDC25: self.WUPDC25(),
                WUPDC26: self.WUPDC26(),
                WUPDC27: self.WUPDC27(),
                WUPDC28: self.WUPDC28(),
                WUPDC29: self.WUPDC29(),
                WUPDC30: self.WUPDC30(),
                WUPDC31: self.WUPDC31(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Enable 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PE1(pub u32);
    impl PE1 {
        #[inline(always)]
        pub const fn WUPE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn WUPE1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn WUPE2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn WUPE3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn WUPE4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn WUPE5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn WUPE6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn WUPE7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn WUPE8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn WUPE9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn WUPE10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn WUPE11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn WUPE12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn WUPE13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn WUPE14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn WUPE15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PE1 {
        #[inline(always)]
        fn default() -> PE1 {
            PE1(0)
        }
    }
    impl core::fmt::Debug for PE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PE1")
                .field("WUPE0", &self.WUPE0())
                .field("WUPE1", &self.WUPE1())
                .field("WUPE2", &self.WUPE2())
                .field("WUPE3", &self.WUPE3())
                .field("WUPE4", &self.WUPE4())
                .field("WUPE5", &self.WUPE5())
                .field("WUPE6", &self.WUPE6())
                .field("WUPE7", &self.WUPE7())
                .field("WUPE8", &self.WUPE8())
                .field("WUPE9", &self.WUPE9())
                .field("WUPE10", &self.WUPE10())
                .field("WUPE11", &self.WUPE11())
                .field("WUPE12", &self.WUPE12())
                .field("WUPE13", &self.WUPE13())
                .field("WUPE14", &self.WUPE14())
                .field("WUPE15", &self.WUPE15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PE1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PE1 {
                WUPE0: u8,
                WUPE1: u8,
                WUPE2: u8,
                WUPE3: u8,
                WUPE4: u8,
                WUPE5: u8,
                WUPE6: u8,
                WUPE7: u8,
                WUPE8: u8,
                WUPE9: u8,
                WUPE10: u8,
                WUPE11: u8,
                WUPE12: u8,
                WUPE13: u8,
                WUPE14: u8,
                WUPE15: u8,
            }
            let proxy = PE1 {
                WUPE0: self.WUPE0(),
                WUPE1: self.WUPE1(),
                WUPE2: self.WUPE2(),
                WUPE3: self.WUPE3(),
                WUPE4: self.WUPE4(),
                WUPE5: self.WUPE5(),
                WUPE6: self.WUPE6(),
                WUPE7: self.WUPE7(),
                WUPE8: self.WUPE8(),
                WUPE9: self.WUPE9(),
                WUPE10: self.WUPE10(),
                WUPE11: self.WUPE11(),
                WUPE12: self.WUPE12(),
                WUPE13: self.WUPE13(),
                WUPE14: self.WUPE14(),
                WUPE15: self.WUPE15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Enable 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PE2(pub u32);
    impl PE2 {
        #[inline(always)]
        pub const fn WUPE16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn WUPE17(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn WUPE18(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn WUPE19(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn WUPE20(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn WUPE21(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn WUPE22(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn WUPE23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn WUPE24(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn WUPE25(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE25(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn WUPE26(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn WUPE27(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn WUPE28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE28(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn WUPE29(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE29(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn WUPE30(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn WUPE31(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WUPE31(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for PE2 {
        #[inline(always)]
        fn default() -> PE2 {
            PE2(0)
        }
    }
    impl core::fmt::Debug for PE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PE2")
                .field("WUPE16", &self.WUPE16())
                .field("WUPE17", &self.WUPE17())
                .field("WUPE18", &self.WUPE18())
                .field("WUPE19", &self.WUPE19())
                .field("WUPE20", &self.WUPE20())
                .field("WUPE21", &self.WUPE21())
                .field("WUPE22", &self.WUPE22())
                .field("WUPE23", &self.WUPE23())
                .field("WUPE24", &self.WUPE24())
                .field("WUPE25", &self.WUPE25())
                .field("WUPE26", &self.WUPE26())
                .field("WUPE27", &self.WUPE27())
                .field("WUPE28", &self.WUPE28())
                .field("WUPE29", &self.WUPE29())
                .field("WUPE30", &self.WUPE30())
                .field("WUPE31", &self.WUPE31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PE2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PE2 {
                WUPE16: u8,
                WUPE17: u8,
                WUPE18: u8,
                WUPE19: u8,
                WUPE20: u8,
                WUPE21: u8,
                WUPE22: u8,
                WUPE23: u8,
                WUPE24: u8,
                WUPE25: u8,
                WUPE26: u8,
                WUPE27: u8,
                WUPE28: u8,
                WUPE29: u8,
                WUPE30: u8,
                WUPE31: u8,
            }
            let proxy = PE2 {
                WUPE16: self.WUPE16(),
                WUPE17: self.WUPE17(),
                WUPE18: self.WUPE18(),
                WUPE19: self.WUPE19(),
                WUPE20: self.WUPE20(),
                WUPE21: self.WUPE21(),
                WUPE22: self.WUPE22(),
                WUPE23: self.WUPE23(),
                WUPE24: self.WUPE24(),
                WUPE25: self.WUPE25(),
                WUPE26: self.WUPE26(),
                WUPE27: self.WUPE27(),
                WUPE28: self.WUPE28(),
                WUPE29: self.WUPE29(),
                WUPE30: self.WUPE30(),
                WUPE31: self.WUPE31(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PF(pub u32);
    impl PF {
        #[inline(always)]
        pub const fn WUF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WUF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WUF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WUF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WUF4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WUF5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn WUF6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn WUF7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WUF8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WUF9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn WUF10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn WUF11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WUF12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn WUF13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn WUF14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn WUF15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn WUF16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WUF17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn WUF18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn WUF19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn WUF20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn WUF21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn WUF22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn WUF23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn WUF24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn WUF25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn WUF26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn WUF27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn WUF28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn WUF29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn WUF30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn WUF31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUF31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PF {
        #[inline(always)]
        fn default() -> PF {
            PF(0)
        }
    }
    impl core::fmt::Debug for PF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PF")
                .field("WUF0", &self.WUF0())
                .field("WUF1", &self.WUF1())
                .field("WUF2", &self.WUF2())
                .field("WUF3", &self.WUF3())
                .field("WUF4", &self.WUF4())
                .field("WUF5", &self.WUF5())
                .field("WUF6", &self.WUF6())
                .field("WUF7", &self.WUF7())
                .field("WUF8", &self.WUF8())
                .field("WUF9", &self.WUF9())
                .field("WUF10", &self.WUF10())
                .field("WUF11", &self.WUF11())
                .field("WUF12", &self.WUF12())
                .field("WUF13", &self.WUF13())
                .field("WUF14", &self.WUF14())
                .field("WUF15", &self.WUF15())
                .field("WUF16", &self.WUF16())
                .field("WUF17", &self.WUF17())
                .field("WUF18", &self.WUF18())
                .field("WUF19", &self.WUF19())
                .field("WUF20", &self.WUF20())
                .field("WUF21", &self.WUF21())
                .field("WUF22", &self.WUF22())
                .field("WUF23", &self.WUF23())
                .field("WUF24", &self.WUF24())
                .field("WUF25", &self.WUF25())
                .field("WUF26", &self.WUF26())
                .field("WUF27", &self.WUF27())
                .field("WUF28", &self.WUF28())
                .field("WUF29", &self.WUF29())
                .field("WUF30", &self.WUF30())
                .field("WUF31", &self.WUF31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PF {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PF {
                WUF0: bool,
                WUF1: bool,
                WUF2: bool,
                WUF3: bool,
                WUF4: bool,
                WUF5: bool,
                WUF6: bool,
                WUF7: bool,
                WUF8: bool,
                WUF9: bool,
                WUF10: bool,
                WUF11: bool,
                WUF12: bool,
                WUF13: bool,
                WUF14: bool,
                WUF15: bool,
                WUF16: bool,
                WUF17: bool,
                WUF18: bool,
                WUF19: bool,
                WUF20: bool,
                WUF21: bool,
                WUF22: bool,
                WUF23: bool,
                WUF24: bool,
                WUF25: bool,
                WUF26: bool,
                WUF27: bool,
                WUF28: bool,
                WUF29: bool,
                WUF30: bool,
                WUF31: bool,
            }
            let proxy = PF {
                WUF0: self.WUF0(),
                WUF1: self.WUF1(),
                WUF2: self.WUF2(),
                WUF3: self.WUF3(),
                WUF4: self.WUF4(),
                WUF5: self.WUF5(),
                WUF6: self.WUF6(),
                WUF7: self.WUF7(),
                WUF8: self.WUF8(),
                WUF9: self.WUF9(),
                WUF10: self.WUF10(),
                WUF11: self.WUF11(),
                WUF12: self.WUF12(),
                WUF13: self.WUF13(),
                WUF14: self.WUF14(),
                WUF15: self.WUF15(),
                WUF16: self.WUF16(),
                WUF17: self.WUF17(),
                WUF18: self.WUF18(),
                WUF19: self.WUF19(),
                WUF20: self.WUF20(),
                WUF21: self.WUF21(),
                WUF22: self.WUF22(),
                WUF23: self.WUF23(),
                WUF24: self.WUF24(),
                WUF25: self.WUF25(),
                WUF26: self.WUF26(),
                WUF27: self.WUF27(),
                WUF28: self.WUF28(),
                WUF29: self.WUF29(),
                WUF30: self.WUF30(),
                WUF31: self.WUF31(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMC(pub u32);
    impl PMC {
        #[inline(always)]
        pub const fn WUPMC0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WUPMC1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn WUPMC2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WUPMC3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn WUPMC4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WUPMC5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn WUPMC6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn WUPMC7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WUPMC8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WUPMC9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn WUPMC10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn WUPMC11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn WUPMC12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn WUPMC13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn WUPMC14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn WUPMC15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn WUPMC16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WUPMC17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn WUPMC18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn WUPMC19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn WUPMC20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn WUPMC21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn WUPMC22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn WUPMC23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn WUPMC24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn WUPMC25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn WUPMC26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn WUPMC27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn WUPMC28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn WUPMC29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn WUPMC30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn WUPMC31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WUPMC31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PMC {
        #[inline(always)]
        fn default() -> PMC {
            PMC(0)
        }
    }
    impl core::fmt::Debug for PMC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMC")
                .field("WUPMC0", &self.WUPMC0())
                .field("WUPMC1", &self.WUPMC1())
                .field("WUPMC2", &self.WUPMC2())
                .field("WUPMC3", &self.WUPMC3())
                .field("WUPMC4", &self.WUPMC4())
                .field("WUPMC5", &self.WUPMC5())
                .field("WUPMC6", &self.WUPMC6())
                .field("WUPMC7", &self.WUPMC7())
                .field("WUPMC8", &self.WUPMC8())
                .field("WUPMC9", &self.WUPMC9())
                .field("WUPMC10", &self.WUPMC10())
                .field("WUPMC11", &self.WUPMC11())
                .field("WUPMC12", &self.WUPMC12())
                .field("WUPMC13", &self.WUPMC13())
                .field("WUPMC14", &self.WUPMC14())
                .field("WUPMC15", &self.WUPMC15())
                .field("WUPMC16", &self.WUPMC16())
                .field("WUPMC17", &self.WUPMC17())
                .field("WUPMC18", &self.WUPMC18())
                .field("WUPMC19", &self.WUPMC19())
                .field("WUPMC20", &self.WUPMC20())
                .field("WUPMC21", &self.WUPMC21())
                .field("WUPMC22", &self.WUPMC22())
                .field("WUPMC23", &self.WUPMC23())
                .field("WUPMC24", &self.WUPMC24())
                .field("WUPMC25", &self.WUPMC25())
                .field("WUPMC26", &self.WUPMC26())
                .field("WUPMC27", &self.WUPMC27())
                .field("WUPMC28", &self.WUPMC28())
                .field("WUPMC29", &self.WUPMC29())
                .field("WUPMC30", &self.WUPMC30())
                .field("WUPMC31", &self.WUPMC31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PMC {
                WUPMC0: bool,
                WUPMC1: bool,
                WUPMC2: bool,
                WUPMC3: bool,
                WUPMC4: bool,
                WUPMC5: bool,
                WUPMC6: bool,
                WUPMC7: bool,
                WUPMC8: bool,
                WUPMC9: bool,
                WUPMC10: bool,
                WUPMC11: bool,
                WUPMC12: bool,
                WUPMC13: bool,
                WUPMC14: bool,
                WUPMC15: bool,
                WUPMC16: bool,
                WUPMC17: bool,
                WUPMC18: bool,
                WUPMC19: bool,
                WUPMC20: bool,
                WUPMC21: bool,
                WUPMC22: bool,
                WUPMC23: bool,
                WUPMC24: bool,
                WUPMC25: bool,
                WUPMC26: bool,
                WUPMC27: bool,
                WUPMC28: bool,
                WUPMC29: bool,
                WUPMC30: bool,
                WUPMC31: bool,
            }
            let proxy = PMC {
                WUPMC0: self.WUPMC0(),
                WUPMC1: self.WUPMC1(),
                WUPMC2: self.WUPMC2(),
                WUPMC3: self.WUPMC3(),
                WUPMC4: self.WUPMC4(),
                WUPMC5: self.WUPMC5(),
                WUPMC6: self.WUPMC6(),
                WUPMC7: self.WUPMC7(),
                WUPMC8: self.WUPMC8(),
                WUPMC9: self.WUPMC9(),
                WUPMC10: self.WUPMC10(),
                WUPMC11: self.WUPMC11(),
                WUPMC12: self.WUPMC12(),
                WUPMC13: self.WUPMC13(),
                WUPMC14: self.WUPMC14(),
                WUPMC15: self.WUPMC15(),
                WUPMC16: self.WUPMC16(),
                WUPMC17: self.WUPMC17(),
                WUPMC18: self.WUPMC18(),
                WUPMC19: self.WUPMC19(),
                WUPMC20: self.WUPMC20(),
                WUPMC21: self.WUPMC21(),
                WUPMC22: self.WUPMC22(),
                WUPMC23: self.WUPMC23(),
                WUPMC24: self.WUPMC24(),
                WUPMC25: self.WUPMC25(),
                WUPMC26: self.WUPMC26(),
                WUPMC27: self.WUPMC27(),
                WUPMC28: self.WUPMC28(),
                WUPMC29: self.WUPMC29(),
                WUPMC30: self.WUPMC30(),
                WUPMC31: self.WUPMC31(),
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
