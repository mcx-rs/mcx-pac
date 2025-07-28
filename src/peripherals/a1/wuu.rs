#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
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
        #[must_use]
        #[inline(always)]
        pub const fn WUDE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUDE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUDE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUDE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUDE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUDE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("WUDE4", &self.WUDE4())
                .field("WUDE6", &self.WUDE6())
                .field("WUDE8", &self.WUDE8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DE {{ WUDE4: {=bool:?}, WUDE6: {=bool:?}, WUDE8: {=bool:?} }}",
                self.WUDE4(),
                self.WUDE6(),
                self.WUDE8()
            )
        }
    }
    #[doc = "Pin Filter DMA/Trigger Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FDC(pub u32);
    impl FDC {
        #[must_use]
        #[inline(always)]
        pub const fn FILTC1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTC2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTC2(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "FDC {{ FILTC1: {=u8:?}, FILTC2: {=u8:?} }}",
                self.FILTC1(),
                self.FILTC2()
            )
        }
    }
    #[doc = "Pin Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FILT(pub u32);
    impl FILT {
        #[must_use]
        #[inline(always)]
        pub const fn FILTSEL1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTE1(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTF1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FILTF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTE2(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTF2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FILTF2(&mut self, val: bool) {
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
            defmt :: write ! (f , "FILT {{ FILTSEL1: {=u8:?}, FILTE1: {=u8:?}, FILTF1: {=bool:?}, FILTSEL2: {=u8:?}, FILTE2: {=u8:?}, FILTF2: {=bool:?} }}" , self . FILTSEL1 () , self . FILTE1 () , self . FILTF1 () , self . FILTSEL2 () , self . FILTE2 () , self . FILTF2 ())
        }
    }
    #[doc = "Pin Filter Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FMC(pub u32);
    impl FMC {
        #[must_use]
        #[inline(always)]
        pub const fn FILTM1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FILTM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTM2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FILTM2(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "FMC {{ FILTM1: {=bool:?}, FILTM2: {=bool:?} }}",
                self.FILTM1(),
                self.FILTM2()
            )
        }
    }
    #[doc = "Module Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ME(pub u32);
    impl ME {
        #[must_use]
        #[inline(always)]
        pub const fn WUME0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUME0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUME2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUME2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUME6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUME6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUME8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUME8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("WUME2", &self.WUME2())
                .field("WUME6", &self.WUME6())
                .field("WUME8", &self.WUME8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ME {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ME {{ WUME0: {=bool:?}, WUME2: {=bool:?}, WUME6: {=bool:?}, WUME8: {=bool:?} }}",
                self.WUME0(),
                self.WUME2(),
                self.WUME6(),
                self.WUME8()
            )
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn FILTERS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTERS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DMAS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MODULES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MODULES(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PINS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PINS(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "PARAM {{ FILTERS: {=u8:?}, DMAS: {=u8:?}, MODULES: {=u8:?}, PINS: {=u8:?} }}",
                self.FILTERS(),
                self.DMAS(),
                self.MODULES(),
                self.PINS()
            )
        }
    }
    #[doc = "Pin DMA/Trigger Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDC1(pub u32);
    impl PDC1 {
        #[must_use]
        #[inline(always)]
        pub const fn Reserved0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved15(&mut self, val: u8) {
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
                .field("Reserved0", &self.Reserved0())
                .field("Reserved1", &self.Reserved1())
                .field("WUPDC2", &self.WUPDC2())
                .field("Reserved3", &self.Reserved3())
                .field("Reserved4", &self.Reserved4())
                .field("Reserved5", &self.Reserved5())
                .field("WUPDC6", &self.WUPDC6())
                .field("WUPDC7", &self.WUPDC7())
                .field("WUPDC8", &self.WUPDC8())
                .field("WUPDC9", &self.WUPDC9())
                .field("WUPDC10", &self.WUPDC10())
                .field("WUPDC11", &self.WUPDC11())
                .field("WUPDC12", &self.WUPDC12())
                .field("Reserved13", &self.Reserved13())
                .field("Reserved14", &self.Reserved14())
                .field("Reserved15", &self.Reserved15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDC1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PDC1 {{ Reserved0: {=u8:?}, Reserved1: {=u8:?}, WUPDC2: {=u8:?}, Reserved3: {=u8:?}, Reserved4: {=u8:?}, Reserved5: {=u8:?}, WUPDC6: {=u8:?}, WUPDC7: {=u8:?}, WUPDC8: {=u8:?}, WUPDC9: {=u8:?}, WUPDC10: {=u8:?}, WUPDC11: {=u8:?}, WUPDC12: {=u8:?}, Reserved13: {=u8:?}, Reserved14: {=u8:?}, Reserved15: {=u8:?} }}" , self . Reserved0 () , self . Reserved1 () , self . WUPDC2 () , self . Reserved3 () , self . Reserved4 () , self . Reserved5 () , self . WUPDC6 () , self . WUPDC7 () , self . WUPDC8 () , self . WUPDC9 () , self . WUPDC10 () , self . WUPDC11 () , self . WUPDC12 () , self . Reserved13 () , self . Reserved14 () , self . Reserved15 ())
        }
    }
    #[doc = "Pin DMA/Trigger Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDC2(pub u32);
    impl PDC2 {
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC17(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC18(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC19(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC20(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved21(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC22(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC24(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC25(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC25(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC26(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC27(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC28(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC29(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC29(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC30(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPDC31(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPDC31(&mut self, val: u8) {
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
                .field("Reserved21", &self.Reserved21())
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
            defmt :: write ! (f , "PDC2 {{ WUPDC16: {=u8:?}, WUPDC17: {=u8:?}, WUPDC18: {=u8:?}, WUPDC19: {=u8:?}, WUPDC20: {=u8:?}, Reserved21: {=u8:?}, WUPDC22: {=u8:?}, WUPDC23: {=u8:?}, WUPDC24: {=u8:?}, WUPDC25: {=u8:?}, WUPDC26: {=u8:?}, WUPDC27: {=u8:?}, WUPDC28: {=u8:?}, WUPDC29: {=u8:?}, WUPDC30: {=u8:?}, WUPDC31: {=u8:?} }}" , self . WUPDC16 () , self . WUPDC17 () , self . WUPDC18 () , self . WUPDC19 () , self . WUPDC20 () , self . Reserved21 () , self . WUPDC22 () , self . WUPDC23 () , self . WUPDC24 () , self . WUPDC25 () , self . WUPDC26 () , self . WUPDC27 () , self . WUPDC28 () , self . WUPDC29 () , self . WUPDC30 () , self . WUPDC31 ())
        }
    }
    #[doc = "Pin Enable 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PE1(pub u32);
    impl PE1 {
        #[must_use]
        #[inline(always)]
        pub const fn Reserved0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved15(&mut self, val: u8) {
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
                .field("Reserved0", &self.Reserved0())
                .field("Reserved1", &self.Reserved1())
                .field("WUPE2", &self.WUPE2())
                .field("Reserved3", &self.Reserved3())
                .field("Reserved4", &self.Reserved4())
                .field("Reserved5", &self.Reserved5())
                .field("WUPE6", &self.WUPE6())
                .field("WUPE7", &self.WUPE7())
                .field("WUPE8", &self.WUPE8())
                .field("WUPE9", &self.WUPE9())
                .field("WUPE10", &self.WUPE10())
                .field("WUPE11", &self.WUPE11())
                .field("WUPE12", &self.WUPE12())
                .field("Reserved13", &self.Reserved13())
                .field("Reserved14", &self.Reserved14())
                .field("Reserved15", &self.Reserved15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PE1 {{ Reserved0: {=u8:?}, Reserved1: {=u8:?}, WUPE2: {=u8:?}, Reserved3: {=u8:?}, Reserved4: {=u8:?}, Reserved5: {=u8:?}, WUPE6: {=u8:?}, WUPE7: {=u8:?}, WUPE8: {=u8:?}, WUPE9: {=u8:?}, WUPE10: {=u8:?}, WUPE11: {=u8:?}, WUPE12: {=u8:?}, Reserved13: {=u8:?}, Reserved14: {=u8:?}, Reserved15: {=u8:?} }}" , self . Reserved0 () , self . Reserved1 () , self . WUPE2 () , self . Reserved3 () , self . Reserved4 () , self . Reserved5 () , self . WUPE6 () , self . WUPE7 () , self . WUPE8 () , self . WUPE9 () , self . WUPE10 () , self . WUPE11 () , self . WUPE12 () , self . Reserved13 () , self . Reserved14 () , self . Reserved15 ())
        }
    }
    #[doc = "Pin Enable 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PE2(pub u32);
    impl PE2 {
        #[must_use]
        #[inline(always)]
        pub const fn WUPE16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE17(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE18(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE19(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE20(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved21(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE22(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE23(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE24(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE25(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE25(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE26(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE27(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE28(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE28(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE29(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE29(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE30(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPE31(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUPE31(&mut self, val: u8) {
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
                .field("Reserved21", &self.Reserved21())
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
            defmt :: write ! (f , "PE2 {{ WUPE16: {=u8:?}, WUPE17: {=u8:?}, WUPE18: {=u8:?}, WUPE19: {=u8:?}, WUPE20: {=u8:?}, Reserved21: {=u8:?}, WUPE22: {=u8:?}, WUPE23: {=u8:?}, WUPE24: {=u8:?}, WUPE25: {=u8:?}, WUPE26: {=u8:?}, WUPE27: {=u8:?}, WUPE28: {=u8:?}, WUPE29: {=u8:?}, WUPE30: {=u8:?}, WUPE31: {=u8:?} }}" , self . WUPE16 () , self . WUPE17 () , self . WUPE18 () , self . WUPE19 () , self . WUPE20 () , self . Reserved21 () , self . WUPE22 () , self . WUPE23 () , self . WUPE24 () , self . WUPE25 () , self . WUPE26 () , self . WUPE27 () , self . WUPE28 () , self . WUPE29 () , self . WUPE30 () , self . WUPE31 ())
        }
    }
    #[doc = "Pin Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PF(pub u32);
    impl PF {
        #[must_use]
        #[inline(always)]
        pub const fn Reserved0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUF31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUF31(&mut self, val: bool) {
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
                .field("Reserved0", &self.Reserved0())
                .field("Reserved1", &self.Reserved1())
                .field("WUF2", &self.WUF2())
                .field("Reserved3", &self.Reserved3())
                .field("Reserved4", &self.Reserved4())
                .field("Reserved5", &self.Reserved5())
                .field("WUF6", &self.WUF6())
                .field("WUF7", &self.WUF7())
                .field("WUF8", &self.WUF8())
                .field("WUF9", &self.WUF9())
                .field("WUF10", &self.WUF10())
                .field("WUF11", &self.WUF11())
                .field("WUF12", &self.WUF12())
                .field("Reserved13", &self.Reserved13())
                .field("Reserved14", &self.Reserved14())
                .field("Reserved15", &self.Reserved15())
                .field("WUF16", &self.WUF16())
                .field("WUF17", &self.WUF17())
                .field("WUF18", &self.WUF18())
                .field("WUF19", &self.WUF19())
                .field("WUF20", &self.WUF20())
                .field("Reserved21", &self.Reserved21())
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
            defmt :: write ! (f , "PF {{ Reserved0: {=bool:?}, Reserved1: {=bool:?}, WUF2: {=bool:?}, Reserved3: {=bool:?}, Reserved4: {=bool:?}, Reserved5: {=bool:?}, WUF6: {=bool:?}, WUF7: {=bool:?}, WUF8: {=bool:?}, WUF9: {=bool:?}, WUF10: {=bool:?}, WUF11: {=bool:?}, WUF12: {=bool:?}, Reserved13: {=bool:?}, Reserved14: {=bool:?}, Reserved15: {=bool:?}, WUF16: {=bool:?}, WUF17: {=bool:?}, WUF18: {=bool:?}, WUF19: {=bool:?}, WUF20: {=bool:?}, Reserved21: {=bool:?}, WUF22: {=bool:?}, WUF23: {=bool:?}, WUF24: {=bool:?}, WUF25: {=bool:?}, WUF26: {=bool:?}, WUF27: {=bool:?}, WUF28: {=bool:?}, WUF29: {=bool:?}, WUF30: {=bool:?}, WUF31: {=bool:?} }}" , self . Reserved0 () , self . Reserved1 () , self . WUF2 () , self . Reserved3 () , self . Reserved4 () , self . Reserved5 () , self . WUF6 () , self . WUF7 () , self . WUF8 () , self . WUF9 () , self . WUF10 () , self . WUF11 () , self . WUF12 () , self . Reserved13 () , self . Reserved14 () , self . Reserved15 () , self . WUF16 () , self . WUF17 () , self . WUF18 () , self . WUF19 () , self . WUF20 () , self . Reserved21 () , self . WUF22 () , self . WUF23 () , self . WUF24 () , self . WUF25 () , self . WUF26 () , self . WUF27 () , self . WUF28 () , self . WUF29 () , self . WUF30 () , self . WUF31 ())
        }
    }
    #[doc = "Pin Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMC(pub u32);
    impl PMC {
        #[must_use]
        #[inline(always)]
        pub const fn Reserved0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Reserved21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUPMC31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WUPMC31(&mut self, val: bool) {
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
                .field("Reserved0", &self.Reserved0())
                .field("Reserved1", &self.Reserved1())
                .field("WUPMC2", &self.WUPMC2())
                .field("Reserved3", &self.Reserved3())
                .field("Reserved4", &self.Reserved4())
                .field("Reserved5", &self.Reserved5())
                .field("WUPMC6", &self.WUPMC6())
                .field("WUPMC7", &self.WUPMC7())
                .field("WUPMC8", &self.WUPMC8())
                .field("WUPMC9", &self.WUPMC9())
                .field("WUPMC10", &self.WUPMC10())
                .field("WUPMC11", &self.WUPMC11())
                .field("WUPMC12", &self.WUPMC12())
                .field("Reserved13", &self.Reserved13())
                .field("Reserved14", &self.Reserved14())
                .field("Reserved15", &self.Reserved15())
                .field("WUPMC16", &self.WUPMC16())
                .field("WUPMC17", &self.WUPMC17())
                .field("WUPMC18", &self.WUPMC18())
                .field("WUPMC19", &self.WUPMC19())
                .field("WUPMC20", &self.WUPMC20())
                .field("Reserved21", &self.Reserved21())
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
            defmt :: write ! (f , "PMC {{ Reserved0: {=bool:?}, Reserved1: {=bool:?}, WUPMC2: {=bool:?}, Reserved3: {=bool:?}, Reserved4: {=bool:?}, Reserved5: {=bool:?}, WUPMC6: {=bool:?}, WUPMC7: {=bool:?}, WUPMC8: {=bool:?}, WUPMC9: {=bool:?}, WUPMC10: {=bool:?}, WUPMC11: {=bool:?}, WUPMC12: {=bool:?}, Reserved13: {=bool:?}, Reserved14: {=bool:?}, Reserved15: {=bool:?}, WUPMC16: {=bool:?}, WUPMC17: {=bool:?}, WUPMC18: {=bool:?}, WUPMC19: {=bool:?}, WUPMC20: {=bool:?}, Reserved21: {=bool:?}, WUPMC22: {=bool:?}, WUPMC23: {=bool:?}, WUPMC24: {=bool:?}, WUPMC25: {=bool:?}, WUPMC26: {=bool:?}, WUPMC27: {=bool:?}, WUPMC28: {=bool:?}, WUPMC29: {=bool:?}, WUPMC30: {=bool:?}, WUPMC31: {=bool:?} }}" , self . Reserved0 () , self . Reserved1 () , self . WUPMC2 () , self . Reserved3 () , self . Reserved4 () , self . Reserved5 () , self . WUPMC6 () , self . WUPMC7 () , self . WUPMC8 () , self . WUPMC9 () , self . WUPMC10 () , self . WUPMC11 () , self . WUPMC12 () , self . Reserved13 () , self . Reserved14 () , self . Reserved15 () , self . WUPMC16 () , self . WUPMC17 () , self . WUPMC18 () , self . WUPMC19 () , self . WUPMC20 () , self . Reserved21 () , self . WUPMC22 () , self . WUPMC23 () , self . WUPMC24 () , self . WUPMC25 () , self . WUPMC26 () , self . WUPMC27 () , self . WUPMC28 () , self . WUPMC29 () , self . WUPMC30 () , self . WUPMC31 ())
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
