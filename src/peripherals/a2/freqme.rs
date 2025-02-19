#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQME {
    ptr: *mut u8,
}
unsafe impl Send for FREQME {}
unsafe impl Sync for FREQME {}
impl FREQME {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL_R(self) -> crate::common::Reg<regs::CTRL_R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_W(self) -> crate::common::Reg<regs::CTRL_W, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRLSTAT(self) -> crate::common::Reg<regs::CTRLSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MIN(self) -> crate::common::Reg<regs::MIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MAX(self) -> crate::common::Reg<regs::MAX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRLSTAT(pub u32);
    impl CTRLSTAT {
        #[inline(always)]
        pub const fn REF_SCALE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_REF_SCALE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn PULSE_MODE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULSE_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn PULSE_POL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULSE_POL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LT_MIN_INT_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LT_MIN_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn GT_MAX_INT_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GT_MAX_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn RESULT_READY_INT_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESULT_READY_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn LT_MIN_STAT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LT_MIN_STAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn GT_MAX_STAT(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GT_MAX_STAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn RESULT_READY_STAT(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESULT_READY_STAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn CONTINUOUS_MODE_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONTINUOUS_MODE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn MEASURE_IN_PROGRESS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MEASURE_IN_PROGRESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRLSTAT {
        #[inline(always)]
        fn default() -> CTRLSTAT {
            CTRLSTAT(0)
        }
    }
    impl core::fmt::Debug for CTRLSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRLSTAT")
                .field("REF_SCALE", &self.REF_SCALE())
                .field("PULSE_MODE", &self.PULSE_MODE())
                .field("PULSE_POL", &self.PULSE_POL())
                .field("LT_MIN_INT_EN", &self.LT_MIN_INT_EN())
                .field("GT_MAX_INT_EN", &self.GT_MAX_INT_EN())
                .field("RESULT_READY_INT_EN", &self.RESULT_READY_INT_EN())
                .field("LT_MIN_STAT", &self.LT_MIN_STAT())
                .field("GT_MAX_STAT", &self.GT_MAX_STAT())
                .field("RESULT_READY_STAT", &self.RESULT_READY_STAT())
                .field("CONTINUOUS_MODE_EN", &self.CONTINUOUS_MODE_EN())
                .field("MEASURE_IN_PROGRESS", &self.MEASURE_IN_PROGRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRLSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRLSTAT {{ REF_SCALE: {=u8:?}, PULSE_MODE: {=bool:?}, PULSE_POL: {=bool:?}, LT_MIN_INT_EN: {=bool:?}, GT_MAX_INT_EN: {=bool:?}, RESULT_READY_INT_EN: {=bool:?}, LT_MIN_STAT: {=bool:?}, GT_MAX_STAT: {=bool:?}, RESULT_READY_STAT: {=bool:?}, CONTINUOUS_MODE_EN: {=bool:?}, MEASURE_IN_PROGRESS: {=bool:?} }}" , self . REF_SCALE () , self . PULSE_MODE () , self . PULSE_POL () , self . LT_MIN_INT_EN () , self . GT_MAX_INT_EN () , self . RESULT_READY_INT_EN () , self . LT_MIN_STAT () , self . GT_MAX_STAT () , self . RESULT_READY_STAT () , self . CONTINUOUS_MODE_EN () , self . MEASURE_IN_PROGRESS ())
        }
    }
    #[doc = "Control (in Read mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_R(pub u32);
    impl CTRL_R {
        #[inline(always)]
        pub const fn RESULT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RESULT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MEASURE_IN_PROGRESS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MEASURE_IN_PROGRESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_R {
        #[inline(always)]
        fn default() -> CTRL_R {
            CTRL_R(0)
        }
    }
    impl core::fmt::Debug for CTRL_R {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_R")
                .field("RESULT", &self.RESULT())
                .field("MEASURE_IN_PROGRESS", &self.MEASURE_IN_PROGRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_R {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CTRL_R {{ RESULT: {=u32:?}, MEASURE_IN_PROGRESS: {=bool:?} }}",
                self.RESULT(),
                self.MEASURE_IN_PROGRESS()
            )
        }
    }
    #[doc = "Control (in Write mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_W(pub u32);
    impl CTRL_W {
        #[inline(always)]
        pub const fn REF_SCALE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_REF_SCALE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn PULSE_MODE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULSE_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn PULSE_POL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULSE_POL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LT_MIN_INT_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LT_MIN_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn GT_MAX_INT_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GT_MAX_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn RESULT_READY_INT_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESULT_READY_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn CONTINUOUS_MODE_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONTINUOUS_MODE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn MEASURE_IN_PROGRESS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MEASURE_IN_PROGRESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_W {
        #[inline(always)]
        fn default() -> CTRL_W {
            CTRL_W(0)
        }
    }
    impl core::fmt::Debug for CTRL_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_W")
                .field("REF_SCALE", &self.REF_SCALE())
                .field("PULSE_MODE", &self.PULSE_MODE())
                .field("PULSE_POL", &self.PULSE_POL())
                .field("LT_MIN_INT_EN", &self.LT_MIN_INT_EN())
                .field("GT_MAX_INT_EN", &self.GT_MAX_INT_EN())
                .field("RESULT_READY_INT_EN", &self.RESULT_READY_INT_EN())
                .field("CONTINUOUS_MODE_EN", &self.CONTINUOUS_MODE_EN())
                .field("MEASURE_IN_PROGRESS", &self.MEASURE_IN_PROGRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL_W {{ REF_SCALE: {=u8:?}, PULSE_MODE: {=bool:?}, PULSE_POL: {=bool:?}, LT_MIN_INT_EN: {=bool:?}, GT_MAX_INT_EN: {=bool:?}, RESULT_READY_INT_EN: {=bool:?}, CONTINUOUS_MODE_EN: {=bool:?}, MEASURE_IN_PROGRESS: {=bool:?} }}" , self . REF_SCALE () , self . PULSE_MODE () , self . PULSE_POL () , self . LT_MIN_INT_EN () , self . GT_MAX_INT_EN () , self . RESULT_READY_INT_EN () , self . CONTINUOUS_MODE_EN () , self . MEASURE_IN_PROGRESS ())
        }
    }
    #[doc = "Maximum"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAX(pub u32);
    impl MAX {
        #[inline(always)]
        pub const fn MAX_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MAX_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for MAX {
        #[inline(always)]
        fn default() -> MAX {
            MAX(0)
        }
    }
    impl core::fmt::Debug for MAX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAX")
                .field("MAX_VALUE", &self.MAX_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MAX {{ MAX_VALUE: {=u32:?} }}", self.MAX_VALUE())
        }
    }
    #[doc = "Minimum"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MIN(pub u32);
    impl MIN {
        #[inline(always)]
        pub const fn MIN_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MIN_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for MIN {
        #[inline(always)]
        fn default() -> MIN {
            MIN(0)
        }
    }
    impl core::fmt::Debug for MIN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MIN")
                .field("MIN_VALUE", &self.MIN_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MIN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MIN {{ MIN_VALUE: {=u32:?} }}", self.MIN_VALUE())
        }
    }
}
