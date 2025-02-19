#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
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
    pub const fn REG0_TOP(self) -> crate::common::Reg<regs::REG0_TOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn REG1_TOP(self) -> crate::common::Reg<regs::REG1_TOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn POLSEL(self) -> crate::common::Reg<regs::POLSEL, crate::common::RW> {
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
    impl core::fmt::Debug for POLSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("POLSEL")
                .field("REG0_POLICY", &self.REG0_POLICY())
                .field("REG1_POLICY", &self.REG1_POLICY())
                .field("REG2_POLICY", &self.REG2_POLICY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for POLSEL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "POLSEL {{ REG0_POLICY: {=u8:?}, REG1_POLICY: {=u8:?}, REG2_POLICY: {=u8:?} }}",
                self.REG0_POLICY(),
                self.REG1_POLICY(),
                self.REG2_POLICY()
            )
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
    impl core::fmt::Debug for REG0_TOP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REG0_TOP")
                .field("REG0_TOP", &self.REG0_TOP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REG0_TOP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "REG0_TOP {{ REG0_TOP: {=u32:?} }}", self.REG0_TOP())
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
    impl core::fmt::Debug for REG1_TOP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REG1_TOP")
                .field("REG1_TOP", &self.REG1_TOP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REG1_TOP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "REG1_TOP {{ REG1_TOP: {=u32:?} }}", self.REG1_TOP())
        }
    }
}
