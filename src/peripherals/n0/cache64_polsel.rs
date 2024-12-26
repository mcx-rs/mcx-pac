#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CACHE64_POLSEL {
    ptr: *mut u8,
}
unsafe impl Send for CACHE64_POLSEL {}
unsafe impl Sync for CACHE64_POLSEL {}
impl CACHE64_POLSEL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn REG0_TOP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn REG1_TOP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn POLSEL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Policy Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct POLSEL(pub u32);
    impl POLSEL {
        #[inline(always)]
        pub const fn REG0_POLICY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_REG0_POLICY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn REG1_POLICY(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_REG1_POLICY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn REG2_POLICY(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_REG2_POLICY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for POLSEL {
        #[inline(always)]
        fn default() -> POLSEL {
            POLSEL(0)
        }
    }
    #[doc = "Region 0 Top Boundary"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REG0_TOP(pub u32);
    impl REG0_TOP {
        #[inline(always)]
        pub const fn REG0_TOP(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x0007_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_REG0_TOP(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0007_ffff << 10usize)) | (((val as u32) & 0x0007_ffff) << 10usize);
        }
    }
    impl Default for REG0_TOP {
        #[inline(always)]
        fn default() -> REG0_TOP {
            REG0_TOP(0)
        }
    }
    #[doc = "Region 1 Top Boundary"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REG1_TOP(pub u32);
    impl REG1_TOP {
        #[inline(always)]
        pub const fn REG1_TOP(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x0007_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_REG1_TOP(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0007_ffff << 10usize)) | (((val as u32) & 0x0007_ffff) << 10usize);
        }
    }
    impl Default for REG1_TOP {
        #[inline(always)]
        fn default() -> REG1_TOP {
            REG1_TOP(0)
        }
    }
}
