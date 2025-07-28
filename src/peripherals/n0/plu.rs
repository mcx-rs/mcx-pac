#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LUT {
    ptr: *mut u8,
}
unsafe impl Send for LUT {}
unsafe impl Sync for LUT {}
impl LUT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn INP_MUX(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLU {
    ptr: *mut u8,
}
unsafe impl Send for PLU {}
unsafe impl Sync for PLU {}
impl PLU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn LUT(self, n: usize) -> LUT {
        assert!(n < 26usize);
        unsafe { LUT::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn LUT_TRUTH(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 26usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTPUTS(self) -> crate::common::Reg<regs::OUTPUTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKEINT_CTRL(self) -> crate::common::Reg<regs::WAKEINT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTPUT_MUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OUTPUT_MUX, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LUT_INP_MUX(pub u32);
    impl LUT_INP_MUX {
        #[must_use]
        #[inline(always)]
        pub const fn LUTn_INPx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LUTn_INPx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for LUT_INP_MUX {
        #[inline(always)]
        fn default() -> LUT_INP_MUX {
            LUT_INP_MUX(0)
        }
    }
    impl core::fmt::Debug for LUT_INP_MUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LUT_INP_MUX")
                .field("LUTn_INPx", &self.LUTn_INPx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LUT_INP_MUX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LUT_INP_MUX {{ LUTn_INPx: {=u8:?} }}", self.LUTn_INPx())
        }
    }
    #[doc = "PLU outputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUTS(pub u32);
    impl OUTPUTS {
        #[must_use]
        #[inline(always)]
        pub const fn OUTPUT_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OUTPUT_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for OUTPUTS {
        #[inline(always)]
        fn default() -> OUTPUTS {
            OUTPUTS(0)
        }
    }
    impl core::fmt::Debug for OUTPUTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTPUTS")
                .field("OUTPUT_STATE", &self.OUTPUT_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTPUTS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OUTPUTS {{ OUTPUT_STATE: {=u8:?} }}",
                self.OUTPUT_STATE()
            )
        }
    }
    #[doc = "PLU output multiplexer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUT_MUX(pub u32);
    impl OUTPUT_MUX {
        #[must_use]
        #[inline(always)]
        pub const fn OUTPUT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OUTPUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for OUTPUT_MUX {
        #[inline(always)]
        fn default() -> OUTPUT_MUX {
            OUTPUT_MUX(0)
        }
    }
    impl core::fmt::Debug for OUTPUT_MUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTPUT_MUX")
                .field("OUTPUT", &self.OUTPUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTPUT_MUX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OUTPUT_MUX {{ OUTPUT: {=u8:?} }}", self.OUTPUT())
        }
    }
    #[doc = "Wakeup interrupt control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKEINT_CTRL(pub u32);
    impl WAKEINT_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn MASK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTER_MODE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTER_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTER_CLKSEL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTER_CLKSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LATCH_ENABLE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LATCH_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INTR_CLEAR(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INTR_CLEAR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for WAKEINT_CTRL {
        #[inline(always)]
        fn default() -> WAKEINT_CTRL {
            WAKEINT_CTRL(0)
        }
    }
    impl core::fmt::Debug for WAKEINT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKEINT_CTRL")
                .field("MASK", &self.MASK())
                .field("FILTER_MODE", &self.FILTER_MODE())
                .field("FILTER_CLKSEL", &self.FILTER_CLKSEL())
                .field("LATCH_ENABLE", &self.LATCH_ENABLE())
                .field("INTR_CLEAR", &self.INTR_CLEAR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKEINT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WAKEINT_CTRL {{ MASK: {=u8:?}, FILTER_MODE: {=u8:?}, FILTER_CLKSEL: {=u8:?}, LATCH_ENABLE: {=bool:?}, INTR_CLEAR: {=bool:?} }}" , self . MASK () , self . FILTER_MODE () , self . FILTER_CLKSEL () , self . LATCH_ENABLE () , self . INTR_CLEAR ())
        }
    }
}
