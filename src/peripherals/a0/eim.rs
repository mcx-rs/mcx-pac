#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EIM {
    ptr: *mut u8,
}
unsafe impl Send for EIM {}
unsafe impl Sync for EIM {}
impl EIM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn EIMCR(self) -> crate::common::Reg<regs::EIMCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHEN(self) -> crate::common::Reg<regs::EICHEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD0_WORD0(self) -> crate::common::Reg<regs::EICHD0_WORD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD0_WORD1(self) -> crate::common::Reg<regs::EICHD0_WORD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD0_WORD0(pub u32);
    impl EICHD0_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD0_WORD0 {
        #[inline(always)]
        fn default() -> EICHD0_WORD0 {
            EICHD0_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD0_WORD1(pub u32);
    impl EICHD0_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD0_WORD1 {
        #[inline(always)]
        fn default() -> EICHD0_WORD1 {
            EICHD0_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHEN(pub u32);
    impl EICHEN {
        #[inline(always)]
        pub const fn EICH0EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH0EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EICHEN {
        #[inline(always)]
        fn default() -> EICHEN {
            EICHEN(0)
        }
    }
    #[doc = "Error Injection Module Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EIMCR(pub u32);
    impl EIMCR {
        #[inline(always)]
        pub const fn GEIEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EIMCR {
        #[inline(always)]
        fn default() -> EIMCR {
            EIMCR(0)
        }
    }
}