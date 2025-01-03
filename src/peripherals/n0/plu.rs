#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
        #[inline(always)]
        pub const fn LUTn_INPx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LUTn_INPx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for LUT_INP_MUX {
        #[inline(always)]
        fn default() -> LUT_INP_MUX {
            LUT_INP_MUX(0)
        }
    }
    #[doc = "PLU outputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUTS(pub u32);
    impl OUTPUTS {
        #[inline(always)]
        pub const fn OUTPUT_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUTPUT_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for OUTPUTS {
        #[inline(always)]
        fn default() -> OUTPUTS {
            OUTPUTS(0)
        }
    }
    #[doc = "PLU output multiplexer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUT_MUX(pub u32);
    impl OUTPUT_MUX {
        #[inline(always)]
        pub const fn OUTPUT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OUTPUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for OUTPUT_MUX {
        #[inline(always)]
        fn default() -> OUTPUT_MUX {
            OUTPUT_MUX(0)
        }
    }
    #[doc = "Wakeup interrupt control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKEINT_CTRL(pub u32);
    impl WAKEINT_CTRL {
        #[inline(always)]
        pub const fn MASK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILTER_MODE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTER_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn FILTER_CLKSEL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTER_CLKSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn LATCH_ENABLE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LATCH_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn INTR_CLEAR(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_CLEAR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for WAKEINT_CTRL {
        #[inline(always)]
        fn default() -> WAKEINT_CTRL {
            WAKEINT_CTRL(0)
        }
    }
}
