#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AHBSC {
    ptr: *mut u8,
}
unsafe impl Send for AHBSC {}
unsafe impl Sync for AHBSC {}
impl AHBSC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn FLASH00_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLASH00_MEM_RULE, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASH02_MEM_RULE(
        self,
    ) -> crate::common::Reg<regs::FLASH02_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASH03_MEM_RULE(
        self,
    ) -> crate::common::Reg<regs::FLASH03_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn ROM_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ROM_MEM_RULE, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMX_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAMX_MEM_RULE, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMA_MEM_RULE(self) -> crate::common::Reg<regs::RAMA_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMB_MEM_RULE(self) -> crate::common::Reg<regs::RAMB_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMC_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAMC_MEM_RULE, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMD_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAMD_MEM_RULE, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAME_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAME_MEM_RULE, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP0_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP0_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP0_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP0_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP0_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP0_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP0_MEM_RULE3(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP0_MEM_RULE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP1_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP1_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP1_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP1_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[inline(always)]
    pub const fn APB_PERIPHERAL_GROUP1_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::APB_PERIPHERAL_GROUP1_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP0_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP0_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP0_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP0_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP0_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP0_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP0_MEM_RULE3(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP0_MEM_RULE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0(
        self,
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1(
        self,
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2(
        self,
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP1_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP1_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP1_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP1_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0(
        self,
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1(
        self,
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1, crate::common::RW>
    {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2(
        self,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP2_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP2_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP2_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP2_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP3_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP3_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP3_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP3_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP3_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP3_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP3_MEM_RULE3(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP3_MEM_RULE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP4_MEM_RULE0(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP4_MEM_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP4_MEM_RULE1(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP4_MEM_RULE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP4_MEM_RULE2(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP4_MEM_RULE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn AIPS_BRIDGE_GROUP4_MEM_RULE3(
        self,
    ) -> crate::common::Reg<regs::AIPS_BRIDGE_GROUP4_MEM_RULE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHB_SECURE_CTRL_PERIPHERAL_RULE0(
        self,
    ) -> crate::common::Reg<regs::AHB_SECURE_CTRL_PERIPHERAL_RULE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_VIO_ADDR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_VIO_MISC_INFO(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SEC_VIO_MISC_INFO, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_VIO_INFO_VALID(
        self,
    ) -> crate::common::Reg<regs::SEC_VIO_INFO_VALID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f00usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_GPIO_MASK(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SEC_GPIO_MASK, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MASTER_SEC_LEVEL(
        self,
    ) -> crate::common::Reg<regs::MASTER_SEC_LEVEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MASTER_SEC_ANTI_POL_REG(
        self,
    ) -> crate::common::Reg<regs::MASTER_SEC_ANTI_POL_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd4usize) as _) }
    }
    #[inline(always)]
    pub const fn CPU0_LOCK_REG(self) -> crate::common::Reg<regs::CPU0_LOCK_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[inline(always)]
    pub const fn MISC_CTRL_DP_REG(
        self,
    ) -> crate::common::Reg<regs::MISC_CTRL_DP_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[inline(always)]
    pub const fn MISC_CTRL_REG(self) -> crate::common::Reg<regs::MISC_CTRL_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0(pub u32);
    impl AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH15(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCT0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SCT0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_FLEXCOMM0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LP_FLEXCOMM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_FLEXCOMM1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LP_FLEXCOMM1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_FLEXCOMM2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LP_FLEXCOMM2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_FLEXCOMM3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LP_FLEXCOMM3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO0_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO0_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {
            AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0")
                .field("eDMA0_CH15", &self.eDMA0_CH15())
                .field("SCT0", &self.SCT0())
                .field("LP_FLEXCOMM0", &self.LP_FLEXCOMM0())
                .field("LP_FLEXCOMM1", &self.LP_FLEXCOMM1())
                .field("LP_FLEXCOMM2", &self.LP_FLEXCOMM2())
                .field("LP_FLEXCOMM3", &self.LP_FLEXCOMM3())
                .field("GPIO0_ALIAS0", &self.GPIO0_ALIAS0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE0 {{ eDMA0_CH15: {=u8:?}, SCT0: {=u8:?}, LP_FLEXCOMM0: {=u8:?}, LP_FLEXCOMM1: {=u8:?}, LP_FLEXCOMM2: {=u8:?}, LP_FLEXCOMM3: {=u8:?}, GPIO0_ALIAS0: {=u8:?} }}" , self . eDMA0_CH15 () , self . SCT0 () , self . LP_FLEXCOMM0 () , self . LP_FLEXCOMM1 () , self . LP_FLEXCOMM2 () , self . LP_FLEXCOMM3 () , self . GPIO0_ALIAS0 ())
        }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1(pub u32);
    impl AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn GPIO0_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO0_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO1_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO1_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO1_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO1_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO2_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO2_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO2_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO2_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO3_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO3_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO3_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO3_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO4_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO4_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {
            AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1")
                .field("GPIO0_ALIAS1", &self.GPIO0_ALIAS1())
                .field("GPIO1_ALIAS0", &self.GPIO1_ALIAS0())
                .field("GPIO1_ALIAS1", &self.GPIO1_ALIAS1())
                .field("GPIO2_ALIAS0", &self.GPIO2_ALIAS0())
                .field("GPIO2_ALIAS1", &self.GPIO2_ALIAS1())
                .field("GPIO3_ALIAS0", &self.GPIO3_ALIAS0())
                .field("GPIO3_ALIAS1", &self.GPIO3_ALIAS1())
                .field("GPIO4_ALIAS0", &self.GPIO4_ALIAS0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE1 {{ GPIO0_ALIAS1: {=u8:?}, GPIO1_ALIAS0: {=u8:?}, GPIO1_ALIAS1: {=u8:?}, GPIO2_ALIAS0: {=u8:?}, GPIO2_ALIAS1: {=u8:?}, GPIO3_ALIAS0: {=u8:?}, GPIO3_ALIAS1: {=u8:?}, GPIO4_ALIAS0: {=u8:?} }}" , self . GPIO0_ALIAS1 () , self . GPIO1_ALIAS0 () , self . GPIO1_ALIAS1 () , self . GPIO2_ALIAS0 () , self . GPIO2_ALIAS1 () , self . GPIO3_ALIAS0 () , self . GPIO3_ALIAS1 () , self . GPIO4_ALIAS0 ())
        }
    }
    #[doc = "AHB Peripheral 0 Slave Port 12 Slave Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2(pub u32);
    impl AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn GPIO4_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO4_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {
            AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2")
                .field("GPIO4_ALIAS1", &self.GPIO4_ALIAS1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHB_PERIPHERAL0_SLAVE_PORT_P12_SLAVE_RULE2 {{ GPIO4_ALIAS1: {=u8:?} }}",
                self.GPIO4_ALIAS1()
            )
        }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0(pub u32);
    impl AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH15(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEMA42(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEMA42(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAILBOX(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAILBOX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PKC_RAM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PKC_RAM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM4(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM5(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {
            AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0")
                .field("eDMA1_CH15", &self.eDMA1_CH15())
                .field("SEMA42", &self.SEMA42())
                .field("MAILBOX", &self.MAILBOX())
                .field("PKC_RAM", &self.PKC_RAM())
                .field("FLEXCOMM4", &self.FLEXCOMM4())
                .field("FLEXCOMM5", &self.FLEXCOMM5())
                .field("FLEXCOMM6", &self.FLEXCOMM6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE0 {{ eDMA1_CH15: {=u8:?}, SEMA42: {=u8:?}, MAILBOX: {=u8:?}, PKC_RAM: {=u8:?}, FLEXCOMM4: {=u8:?}, FLEXCOMM5: {=u8:?}, FLEXCOMM6: {=u8:?} }}" , self . eDMA1_CH15 () , self . SEMA42 () , self . MAILBOX () , self . PKC_RAM () , self . FLEXCOMM4 () , self . FLEXCOMM5 () , self . FLEXCOMM6 ())
        }
    }
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1(pub u32);
    impl AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM8(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXCOMM9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXCOMM9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_FS_OTG_RAM(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USB_FS_OTG_RAM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CDOG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBUG_MAILBOX(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DEBUG_MAILBOX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPU(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NPU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {
            AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1")
                .field("FLEXCOMM7", &self.FLEXCOMM7())
                .field("FLEXCOMM8", &self.FLEXCOMM8())
                .field("FLEXCOMM9", &self.FLEXCOMM9())
                .field("USB_FS_OTG_RAM", &self.USB_FS_OTG_RAM())
                .field("CDOG0", &self.CDOG0())
                .field("CDOG1", &self.CDOG1())
                .field("DEBUG_MAILBOX", &self.DEBUG_MAILBOX())
                .field("NPU", &self.NPU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE1 {{ FLEXCOMM7: {=u8:?}, FLEXCOMM8: {=u8:?}, FLEXCOMM9: {=u8:?}, USB_FS_OTG_RAM: {=u8:?}, CDOG0: {=u8:?}, CDOG1: {=u8:?}, DEBUG_MAILBOX: {=u8:?}, NPU: {=u8:?} }}" , self . FLEXCOMM7 () , self . FLEXCOMM8 () , self . FLEXCOMM9 () , self . USB_FS_OTG_RAM () , self . CDOG0 () , self . CDOG1 () , self . DEBUG_MAILBOX () , self . NPU ())
        }
    }
    #[doc = "AHB Secure Control Peripheral Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_SECURE_CTRL_PERIPHERAL_RULE0(pub u32);
    impl AHB_SECURE_CTRL_PERIPHERAL_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for AHB_SECURE_CTRL_PERIPHERAL_RULE0 {
        #[inline(always)]
        fn default() -> AHB_SECURE_CTRL_PERIPHERAL_RULE0 {
            AHB_SECURE_CTRL_PERIPHERAL_RULE0(0)
        }
    }
    impl core::fmt::Debug for AHB_SECURE_CTRL_PERIPHERAL_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_SECURE_CTRL_PERIPHERAL_RULE0")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_SECURE_CTRL_PERIPHERAL_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHB_SECURE_CTRL_PERIPHERAL_RULE0 {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 ())
        }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP0_MEM_RULE0(pub u32);
    impl AIPS_BRIDGE_GROUP0_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn GPIO5_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO5_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPIO5_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GPIO5_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PORT5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMU0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FMU0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCG0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SCG0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPC0(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SPC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WUU0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WUU0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP0_MEM_RULE0 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP0_MEM_RULE0 {
            AIPS_BRIDGE_GROUP0_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP0_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP0_MEM_RULE0")
                .field("GPIO5_ALIAS0", &self.GPIO5_ALIAS0())
                .field("GPIO5_ALIAS1", &self.GPIO5_ALIAS1())
                .field("PORT5", &self.PORT5())
                .field("FMU0", &self.FMU0())
                .field("SCG0", &self.SCG0())
                .field("SPC0", &self.SPC0())
                .field("WUU0", &self.WUU0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP0_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP0_MEM_RULE0 {{ GPIO5_ALIAS0: {=u8:?}, GPIO5_ALIAS1: {=u8:?}, PORT5: {=u8:?}, FMU0: {=u8:?}, SCG0: {=u8:?}, SPC0: {=u8:?}, WUU0: {=u8:?} }}" , self . GPIO5_ALIAS0 () , self . GPIO5_ALIAS1 () , self . PORT5 () , self . FMU0 () , self . SCG0 () , self . SPC0 () , self . WUU0 ())
        }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP0_MEM_RULE1(pub u32);
    impl AIPS_BRIDGE_GROUP0_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn LPTMR0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPTMR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPTMR1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPTMR1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RTC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RTC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMU_TEST(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FMU_TEST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP0_MEM_RULE1 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP0_MEM_RULE1 {
            AIPS_BRIDGE_GROUP0_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP0_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP0_MEM_RULE1")
                .field("LPTMR0", &self.LPTMR0())
                .field("LPTMR1", &self.LPTMR1())
                .field("RTC", &self.RTC())
                .field("FMU_TEST", &self.FMU_TEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP0_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP0_MEM_RULE1 {{ LPTMR0: {=u8:?}, LPTMR1: {=u8:?}, RTC: {=u8:?}, FMU_TEST: {=u8:?} }}" , self . LPTMR0 () , self . LPTMR1 () , self . RTC () , self . FMU_TEST ())
        }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP0_MEM_RULE2(pub u32);
    impl AIPS_BRIDGE_GROUP0_MEM_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn TSI(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TSI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMP0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMP0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMP1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMP1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMP2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMP2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ELS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ELS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ELS_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ELS_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ELS_ALIAS2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ELS_ALIAS2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ELS_ALIAS3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ELS_ALIAS3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP0_MEM_RULE2 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP0_MEM_RULE2 {
            AIPS_BRIDGE_GROUP0_MEM_RULE2(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP0_MEM_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP0_MEM_RULE2")
                .field("TSI", &self.TSI())
                .field("CMP0", &self.CMP0())
                .field("CMP1", &self.CMP1())
                .field("CMP2", &self.CMP2())
                .field("ELS", &self.ELS())
                .field("ELS_ALIAS1", &self.ELS_ALIAS1())
                .field("ELS_ALIAS2", &self.ELS_ALIAS2())
                .field("ELS_ALIAS3", &self.ELS_ALIAS3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP0_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP0_MEM_RULE2 {{ TSI: {=u8:?}, CMP0: {=u8:?}, CMP1: {=u8:?}, CMP2: {=u8:?}, ELS: {=u8:?}, ELS_ALIAS1: {=u8:?}, ELS_ALIAS2: {=u8:?}, ELS_ALIAS3: {=u8:?} }}" , self . TSI () , self . CMP0 () , self . CMP1 () , self . CMP2 () , self . ELS () , self . ELS_ALIAS1 () , self . ELS_ALIAS2 () , self . ELS_ALIAS3 ())
        }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP0_MEM_RULE3(pub u32);
    impl AIPS_BRIDGE_GROUP0_MEM_RULE3 {
        #[must_use]
        #[inline(always)]
        pub const fn DIGTMP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DIGTMP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VBAT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VBAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRNG(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRNG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EIM0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EIM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERM0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INTM0(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INTM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP0_MEM_RULE3 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP0_MEM_RULE3 {
            AIPS_BRIDGE_GROUP0_MEM_RULE3(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP0_MEM_RULE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP0_MEM_RULE3")
                .field("DIGTMP", &self.DIGTMP())
                .field("VBAT", &self.VBAT())
                .field("TRNG", &self.TRNG())
                .field("EIM0", &self.EIM0())
                .field("ERM0", &self.ERM0())
                .field("INTM0", &self.INTM0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP0_MEM_RULE3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP0_MEM_RULE3 {{ DIGTMP: {=u8:?}, VBAT: {=u8:?}, TRNG: {=u8:?}, EIM0: {=u8:?}, ERM0: {=u8:?}, INTM0: {=u8:?} }}" , self . DIGTMP () , self . VBAT () , self . TRNG () , self . EIM0 () , self . ERM0 () , self . INTM0 ())
        }
    }
    #[doc = "AIPS Bridge Group 1 Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP1_MEM_RULE0(pub u32);
    impl AIPS_BRIDGE_GROUP1_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_MP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_MP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH4(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH5(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP1_MEM_RULE0 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP1_MEM_RULE0 {
            AIPS_BRIDGE_GROUP1_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP1_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP1_MEM_RULE0")
                .field("eDMA0_MP", &self.eDMA0_MP())
                .field("eDMA0_CH0", &self.eDMA0_CH0())
                .field("eDMA0_CH1", &self.eDMA0_CH1())
                .field("eDMA0_CH2", &self.eDMA0_CH2())
                .field("eDMA0_CH3", &self.eDMA0_CH3())
                .field("eDMA0_CH4", &self.eDMA0_CH4())
                .field("eDMA0_CH5", &self.eDMA0_CH5())
                .field("eDMA0_CH6", &self.eDMA0_CH6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP1_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP1_MEM_RULE0 {{ eDMA0_MP: {=u8:?}, eDMA0_CH0: {=u8:?}, eDMA0_CH1: {=u8:?}, eDMA0_CH2: {=u8:?}, eDMA0_CH3: {=u8:?}, eDMA0_CH4: {=u8:?}, eDMA0_CH5: {=u8:?}, eDMA0_CH6: {=u8:?} }}" , self . eDMA0_MP () , self . eDMA0_CH0 () , self . eDMA0_CH1 () , self . eDMA0_CH2 () , self . eDMA0_CH3 () , self . eDMA0_CH4 () , self . eDMA0_CH5 () , self . eDMA0_CH6 ())
        }
    }
    #[doc = "AIPS Bridge Group 1 Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP1_MEM_RULE1(pub u32);
    impl AIPS_BRIDGE_GROUP1_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH8(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH10(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH11(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH12(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH13(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0_CH14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0_CH14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP1_MEM_RULE1 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP1_MEM_RULE1 {
            AIPS_BRIDGE_GROUP1_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP1_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP1_MEM_RULE1")
                .field("eDMA0_CH7", &self.eDMA0_CH7())
                .field("eDMA0_CH8", &self.eDMA0_CH8())
                .field("eDMA0_CH9", &self.eDMA0_CH9())
                .field("eDMA0_CH10", &self.eDMA0_CH10())
                .field("eDMA0_CH11", &self.eDMA0_CH11())
                .field("eDMA0_CH12", &self.eDMA0_CH12())
                .field("eDMA0_CH13", &self.eDMA0_CH13())
                .field("eDMA0_CH14", &self.eDMA0_CH14())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP1_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP1_MEM_RULE1 {{ eDMA0_CH7: {=u8:?}, eDMA0_CH8: {=u8:?}, eDMA0_CH9: {=u8:?}, eDMA0_CH10: {=u8:?}, eDMA0_CH11: {=u8:?}, eDMA0_CH12: {=u8:?}, eDMA0_CH13: {=u8:?}, eDMA0_CH14: {=u8:?} }}" , self . eDMA0_CH7 () , self . eDMA0_CH8 () , self . eDMA0_CH9 () , self . eDMA0_CH10 () , self . eDMA0_CH11 () , self . eDMA0_CH12 () , self . eDMA0_CH13 () , self . eDMA0_CH14 ())
        }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP2_MEM_RULE0(pub u32);
    impl AIPS_BRIDGE_GROUP2_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_MP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_MP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH4(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH5(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP2_MEM_RULE0 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP2_MEM_RULE0 {
            AIPS_BRIDGE_GROUP2_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP2_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP2_MEM_RULE0")
                .field("eDMA1_MP", &self.eDMA1_MP())
                .field("eDMA1_CH0", &self.eDMA1_CH0())
                .field("eDMA1_CH1", &self.eDMA1_CH1())
                .field("eDMA1_CH2", &self.eDMA1_CH2())
                .field("eDMA1_CH3", &self.eDMA1_CH3())
                .field("eDMA1_CH4", &self.eDMA1_CH4())
                .field("eDMA1_CH5", &self.eDMA1_CH5())
                .field("eDMA1_CH6", &self.eDMA1_CH6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP2_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP2_MEM_RULE0 {{ eDMA1_MP: {=u8:?}, eDMA1_CH0: {=u8:?}, eDMA1_CH1: {=u8:?}, eDMA1_CH2: {=u8:?}, eDMA1_CH3: {=u8:?}, eDMA1_CH4: {=u8:?}, eDMA1_CH5: {=u8:?}, eDMA1_CH6: {=u8:?} }}" , self . eDMA1_MP () , self . eDMA1_CH0 () , self . eDMA1_CH1 () , self . eDMA1_CH2 () , self . eDMA1_CH3 () , self . eDMA1_CH4 () , self . eDMA1_CH5 () , self . eDMA1_CH6 ())
        }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP2_MEM_RULE1(pub u32);
    impl AIPS_BRIDGE_GROUP2_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH7(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP2_MEM_RULE1 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP2_MEM_RULE1 {
            AIPS_BRIDGE_GROUP2_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP2_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP2_MEM_RULE1")
                .field("eDMA1_CH7", &self.eDMA1_CH7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP2_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AIPS_BRIDGE_GROUP2_MEM_RULE1 {{ eDMA1_CH7: {=u8:?} }}",
                self.eDMA1_CH7()
            )
        }
    }
    #[doc = "AIPS Bridge Group 3 Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP3_MEM_RULE0(pub u32);
    impl AIPS_BRIDGE_GROUP3_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn EWM0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EWM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPCAC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPCAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXSPI_CMX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXSPI_CMX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SFA(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SFA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP3_MEM_RULE0 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP3_MEM_RULE0 {
            AIPS_BRIDGE_GROUP3_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP3_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP3_MEM_RULE0")
                .field("EWM0", &self.EWM0())
                .field("LPCAC", &self.LPCAC())
                .field("FLEXSPI_CMX", &self.FLEXSPI_CMX())
                .field("SFA", &self.SFA())
                .field("MBC", &self.MBC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP3_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE0 {{ EWM0: {=u8:?}, LPCAC: {=u8:?}, FLEXSPI_CMX: {=u8:?}, SFA: {=u8:?}, MBC: {=u8:?} }}" , self . EWM0 () , self . LPCAC () , self . FLEXSPI_CMX () , self . SFA () , self . MBC ())
        }
    }
    #[doc = "AIPS Bridge Group 3 Memory Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP3_MEM_RULE1(pub u32);
    impl AIPS_BRIDGE_GROUP3_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn FLEXSPI(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXSPI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OTPC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OTPC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPX(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NPX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn QDC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_QDC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP3_MEM_RULE1 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP3_MEM_RULE1 {
            AIPS_BRIDGE_GROUP3_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP3_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP3_MEM_RULE1")
                .field("FLEXSPI", &self.FLEXSPI())
                .field("OTPC", &self.OTPC())
                .field("CRC", &self.CRC())
                .field("NPX", &self.NPX())
                .field("PWM", &self.PWM())
                .field("QDC", &self.QDC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP3_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE1 {{ FLEXSPI: {=u8:?}, OTPC: {=u8:?}, CRC: {=u8:?}, NPX: {=u8:?}, PWM: {=u8:?}, QDC: {=u8:?} }}" , self . FLEXSPI () , self . OTPC () , self . CRC () , self . NPX () , self . PWM () , self . QDC ())
        }
    }
    #[doc = "AIPS Bridge Group 3 Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP3_MEM_RULE2(pub u32);
    impl AIPS_BRIDGE_GROUP3_MEM_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn PWM1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PWM1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn QDC1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_QDC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVTG(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVTG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN0_RULE0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN0_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN0_RULE1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN0_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN0_RULE2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN0_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN0_RULE3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN0_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP3_MEM_RULE2 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP3_MEM_RULE2 {
            AIPS_BRIDGE_GROUP3_MEM_RULE2(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP3_MEM_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP3_MEM_RULE2")
                .field("PWM1", &self.PWM1())
                .field("QDC1", &self.QDC1())
                .field("EVTG", &self.EVTG())
                .field("CAN0_RULE0", &self.CAN0_RULE0())
                .field("CAN0_RULE1", &self.CAN0_RULE1())
                .field("CAN0_RULE2", &self.CAN0_RULE2())
                .field("CAN0_RULE3", &self.CAN0_RULE3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP3_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE2 {{ PWM1: {=u8:?}, QDC1: {=u8:?}, EVTG: {=u8:?}, CAN0_RULE0: {=u8:?}, CAN0_RULE1: {=u8:?}, CAN0_RULE2: {=u8:?}, CAN0_RULE3: {=u8:?} }}" , self . PWM1 () , self . QDC1 () , self . EVTG () , self . CAN0_RULE0 () , self . CAN0_RULE1 () , self . CAN0_RULE2 () , self . CAN0_RULE3 ())
        }
    }
    #[doc = "AIPS Bridge Group 3 Rule 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP3_MEM_RULE3(pub u32);
    impl AIPS_BRIDGE_GROUP3_MEM_RULE3 {
        #[must_use]
        #[inline(always)]
        pub const fn CAN1_RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN1_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN1_RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN1_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN1_RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN1_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAN1_RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAN1_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBDCD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USBDCD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBFS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USBFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP3_MEM_RULE3 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP3_MEM_RULE3 {
            AIPS_BRIDGE_GROUP3_MEM_RULE3(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP3_MEM_RULE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP3_MEM_RULE3")
                .field("CAN1_RULE0", &self.CAN1_RULE0())
                .field("CAN1_RULE1", &self.CAN1_RULE1())
                .field("CAN1_RULE2", &self.CAN1_RULE2())
                .field("CAN1_RULE3", &self.CAN1_RULE3())
                .field("USBDCD", &self.USBDCD())
                .field("USBFS", &self.USBFS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP3_MEM_RULE3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE3 {{ CAN1_RULE0: {=u8:?}, CAN1_RULE1: {=u8:?}, CAN1_RULE2: {=u8:?}, CAN1_RULE3: {=u8:?}, USBDCD: {=u8:?}, USBFS: {=u8:?} }}" , self . CAN1_RULE0 () , self . CAN1_RULE1 () , self . CAN1_RULE2 () , self . CAN1_RULE3 () , self . USBDCD () , self . USBFS ())
        }
    }
    #[doc = "AIPS Bridge Group 4 Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP4_MEM_RULE0(pub u32);
    impl AIPS_BRIDGE_GROUP4_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn ENET(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMVSIM0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMVSIM0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMVSIM1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EMVSIM1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLEXIO(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FLEXIO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAI0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SAI0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAI1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SAI1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP4_MEM_RULE0 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP4_MEM_RULE0 {
            AIPS_BRIDGE_GROUP4_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP4_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP4_MEM_RULE0")
                .field("ENET", &self.ENET())
                .field("EMVSIM0", &self.EMVSIM0())
                .field("EMVSIM1", &self.EMVSIM1())
                .field("FLEXIO", &self.FLEXIO())
                .field("SAI0", &self.SAI0())
                .field("SAI1", &self.SAI1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP4_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP4_MEM_RULE0 {{ ENET: {=u8:?}, EMVSIM0: {=u8:?}, EMVSIM1: {=u8:?}, FLEXIO: {=u8:?}, SAI0: {=u8:?}, SAI1: {=u8:?} }}" , self . ENET () , self . EMVSIM0 () , self . EMVSIM1 () , self . FLEXIO () , self . SAI0 () , self . SAI1 ())
        }
    }
    #[doc = "AIPS Bridge Group 4 Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP4_MEM_RULE1(pub u32);
    impl AIPS_BRIDGE_GROUP4_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn SINC0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SINC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn uSDHC0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_uSDHC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBHSPHY(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USBHSPHY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBHS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USBHS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MICD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MICD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADC0(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADC1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAC0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DAC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP4_MEM_RULE1 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP4_MEM_RULE1 {
            AIPS_BRIDGE_GROUP4_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP4_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP4_MEM_RULE1")
                .field("SINC0", &self.SINC0())
                .field("uSDHC0", &self.uSDHC0())
                .field("USBHSPHY", &self.USBHSPHY())
                .field("USBHS", &self.USBHS())
                .field("MICD", &self.MICD())
                .field("ADC0", &self.ADC0())
                .field("ADC1", &self.ADC1())
                .field("DAC0", &self.DAC0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP4_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP4_MEM_RULE1 {{ SINC0: {=u8:?}, uSDHC0: {=u8:?}, USBHSPHY: {=u8:?}, USBHS: {=u8:?}, MICD: {=u8:?}, ADC0: {=u8:?}, ADC1: {=u8:?}, DAC0: {=u8:?} }}" , self . SINC0 () , self . uSDHC0 () , self . USBHSPHY () , self . USBHS () , self . MICD () , self . ADC0 () , self . ADC1 () , self . DAC0 ())
        }
    }
    #[doc = "AIPS Bridge Group 4 Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP4_MEM_RULE2(pub u32);
    impl AIPS_BRIDGE_GROUP4_MEM_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn OPAMP0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OPAMP0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREF(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VREF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OPAMP1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OPAMP1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HPDAC0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HPDAC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OPAMP2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OPAMP2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PORT0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PORT1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP4_MEM_RULE2 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP4_MEM_RULE2 {
            AIPS_BRIDGE_GROUP4_MEM_RULE2(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP4_MEM_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP4_MEM_RULE2")
                .field("OPAMP0", &self.OPAMP0())
                .field("VREF", &self.VREF())
                .field("DAC", &self.DAC())
                .field("OPAMP1", &self.OPAMP1())
                .field("HPDAC0", &self.HPDAC0())
                .field("OPAMP2", &self.OPAMP2())
                .field("PORT0", &self.PORT0())
                .field("PORT1", &self.PORT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP4_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP4_MEM_RULE2 {{ OPAMP0: {=u8:?}, VREF: {=u8:?}, DAC: {=u8:?}, OPAMP1: {=u8:?}, HPDAC0: {=u8:?}, OPAMP2: {=u8:?}, PORT0: {=u8:?}, PORT1: {=u8:?} }}" , self . OPAMP0 () , self . VREF () , self . DAC () , self . OPAMP1 () , self . HPDAC0 () , self . OPAMP2 () , self . PORT0 () , self . PORT1 ())
        }
    }
    #[doc = "AIPS Bridge Group 4 Rule 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AIPS_BRIDGE_GROUP4_MEM_RULE3(pub u32);
    impl AIPS_BRIDGE_GROUP4_MEM_RULE3 {
        #[must_use]
        #[inline(always)]
        pub const fn PORT2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PORT3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PORT4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PORT4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MTR0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MTR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ATX0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ATX0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for AIPS_BRIDGE_GROUP4_MEM_RULE3 {
        #[inline(always)]
        fn default() -> AIPS_BRIDGE_GROUP4_MEM_RULE3 {
            AIPS_BRIDGE_GROUP4_MEM_RULE3(0)
        }
    }
    impl core::fmt::Debug for AIPS_BRIDGE_GROUP4_MEM_RULE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AIPS_BRIDGE_GROUP4_MEM_RULE3")
                .field("PORT2", &self.PORT2())
                .field("PORT3", &self.PORT3())
                .field("PORT4", &self.PORT4())
                .field("MTR0", &self.MTR0())
                .field("ATX0", &self.ATX0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP4_MEM_RULE3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP4_MEM_RULE3 {{ PORT2: {=u8:?}, PORT3: {=u8:?}, PORT4: {=u8:?}, MTR0: {=u8:?}, ATX0: {=u8:?} }}" , self . PORT2 () , self . PORT3 () , self . PORT4 () , self . MTR0 () , self . ATX0 ())
        }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP0_MEM_RULE0(pub u32);
    impl APB_PERIPHERAL_GROUP0_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn SYSCON(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SYSCON(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PINT0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PINT0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INPUTMUX(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INPUTMUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP0_MEM_RULE0 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP0_MEM_RULE0 {
            APB_PERIPHERAL_GROUP0_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP0_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP0_MEM_RULE0")
                .field("SYSCON", &self.SYSCON())
                .field("PINT0", &self.PINT0())
                .field("INPUTMUX", &self.INPUTMUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP0_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP0_MEM_RULE0 {{ SYSCON: {=u8:?}, PINT0: {=u8:?}, INPUTMUX: {=u8:?} }}" , self . SYSCON () , self . PINT0 () , self . INPUTMUX ())
        }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP0_MEM_RULE1(pub u32);
    impl APB_PERIPHERAL_GROUP0_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn CTIMER0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTIMER0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTIMER1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTIMER1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTIMER2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTIMER2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTIMER3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTIMER3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP0_MEM_RULE1 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP0_MEM_RULE1 {
            APB_PERIPHERAL_GROUP0_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP0_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP0_MEM_RULE1")
                .field("CTIMER0", &self.CTIMER0())
                .field("CTIMER1", &self.CTIMER1())
                .field("CTIMER2", &self.CTIMER2())
                .field("CTIMER3", &self.CTIMER3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP0_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP0_MEM_RULE1 {{ CTIMER0: {=u8:?}, CTIMER1: {=u8:?}, CTIMER2: {=u8:?}, CTIMER3: {=u8:?} }}" , self . CTIMER0 () , self . CTIMER1 () , self . CTIMER2 () , self . CTIMER3 ())
        }
    }
    #[doc = "APB Bridge Group 0 Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP0_MEM_RULE2(pub u32);
    impl APB_PERIPHERAL_GROUP0_MEM_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn CTIMER4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTIMER4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FREQME0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FREQME0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UTCIK0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_UTCIK0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MRT0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MRT0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSTIMER0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OSTIMER0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WWDT0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WWDT1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP0_MEM_RULE2 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP0_MEM_RULE2 {
            APB_PERIPHERAL_GROUP0_MEM_RULE2(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP0_MEM_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP0_MEM_RULE2")
                .field("CTIMER4", &self.CTIMER4())
                .field("FREQME0", &self.FREQME0())
                .field("UTCIK0", &self.UTCIK0())
                .field("MRT0", &self.MRT0())
                .field("OSTIMER0", &self.OSTIMER0())
                .field("WWDT0", &self.WWDT0())
                .field("WWDT1", &self.WWDT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP0_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP0_MEM_RULE2 {{ CTIMER4: {=u8:?}, FREQME0: {=u8:?}, UTCIK0: {=u8:?}, MRT0: {=u8:?}, OSTIMER0: {=u8:?}, WWDT0: {=u8:?}, WWDT1: {=u8:?} }}" , self . CTIMER4 () , self . FREQME0 () , self . UTCIK0 () , self . MRT0 () , self . OSTIMER0 () , self . WWDT0 () , self . WWDT1 ())
        }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP0_MEM_RULE3(pub u32);
    impl APB_PERIPHERAL_GROUP0_MEM_RULE3 {
        #[must_use]
        #[inline(always)]
        pub const fn CACHE64_POLSEL0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CACHE64_POLSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP0_MEM_RULE3 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP0_MEM_RULE3 {
            APB_PERIPHERAL_GROUP0_MEM_RULE3(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP0_MEM_RULE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP0_MEM_RULE3")
                .field("CACHE64_POLSEL0", &self.CACHE64_POLSEL0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP0_MEM_RULE3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APB_PERIPHERAL_GROUP0_MEM_RULE3 {{ CACHE64_POLSEL0: {=u8:?} }}",
                self.CACHE64_POLSEL0()
            )
        }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP1_MEM_RULE0(pub u32);
    impl APB_PERIPHERAL_GROUP1_MEM_RULE0 {
        #[must_use]
        #[inline(always)]
        pub const fn I3C0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_I3C0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I3C1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_I3C1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GDET(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GDET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ITRC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ITRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP1_MEM_RULE0 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP1_MEM_RULE0 {
            APB_PERIPHERAL_GROUP1_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP1_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP1_MEM_RULE0")
                .field("I3C0", &self.I3C0())
                .field("I3C1", &self.I3C1())
                .field("GDET", &self.GDET())
                .field("ITRC", &self.ITRC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP1_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP1_MEM_RULE0 {{ I3C0: {=u8:?}, I3C1: {=u8:?}, GDET: {=u8:?}, ITRC: {=u8:?} }}" , self . I3C0 () , self . I3C1 () , self . GDET () , self . ITRC ())
        }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP1_MEM_RULE1(pub u32);
    impl APB_PERIPHERAL_GROUP1_MEM_RULE1 {
        #[must_use]
        #[inline(always)]
        pub const fn PKC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PKC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUF_ALIAS0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PUF_ALIAS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUF_ALIAS1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PUF_ALIAS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUF_ALIAS2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PUF_ALIAS2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUF_ALIAS3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PUF_ALIAS3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP1_MEM_RULE1 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP1_MEM_RULE1 {
            APB_PERIPHERAL_GROUP1_MEM_RULE1(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP1_MEM_RULE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP1_MEM_RULE1")
                .field("PKC", &self.PKC())
                .field("PUF_ALIAS0", &self.PUF_ALIAS0())
                .field("PUF_ALIAS1", &self.PUF_ALIAS1())
                .field("PUF_ALIAS2", &self.PUF_ALIAS2())
                .field("PUF_ALIAS3", &self.PUF_ALIAS3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP1_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP1_MEM_RULE1 {{ PKC: {=u8:?}, PUF_ALIAS0: {=u8:?}, PUF_ALIAS1: {=u8:?}, PUF_ALIAS2: {=u8:?}, PUF_ALIAS3: {=u8:?} }}" , self . PKC () , self . PUF_ALIAS0 () , self . PUF_ALIAS1 () , self . PUF_ALIAS2 () , self . PUF_ALIAS3 ())
        }
    }
    #[doc = "APB Bridge Group 1 Memory Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APB_PERIPHERAL_GROUP1_MEM_RULE2(pub u32);
    impl APB_PERIPHERAL_GROUP1_MEM_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn COOLFLUX(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_COOLFLUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMARTDMA(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMARTDMA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PLU(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PLU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for APB_PERIPHERAL_GROUP1_MEM_RULE2 {
        #[inline(always)]
        fn default() -> APB_PERIPHERAL_GROUP1_MEM_RULE2 {
            APB_PERIPHERAL_GROUP1_MEM_RULE2(0)
        }
    }
    impl core::fmt::Debug for APB_PERIPHERAL_GROUP1_MEM_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APB_PERIPHERAL_GROUP1_MEM_RULE2")
                .field("COOLFLUX", &self.COOLFLUX())
                .field("SMARTDMA", &self.SMARTDMA())
                .field("PLU", &self.PLU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP1_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP1_MEM_RULE2 {{ COOLFLUX: {=u8:?}, SMARTDMA: {=u8:?}, PLU: {=u8:?} }}" , self . COOLFLUX () , self . SMARTDMA () , self . PLU ())
        }
    }
    #[doc = "Miscellaneous CPU0 Control Signals"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPU0_LOCK_REG(pub u32);
    impl CPU0_LOCK_REG {
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_NS_VTOR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_NS_VTOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_NS_MPU(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_NS_MPU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_S_VTAIRCR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_S_VTAIRCR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_S_MPU(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_S_MPU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_SAU(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_SAU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CM33_LOCK_REG_LOCK(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CM33_LOCK_REG_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CPU0_LOCK_REG {
        #[inline(always)]
        fn default() -> CPU0_LOCK_REG {
            CPU0_LOCK_REG(0)
        }
    }
    impl core::fmt::Debug for CPU0_LOCK_REG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPU0_LOCK_REG")
                .field("LOCK_NS_VTOR", &self.LOCK_NS_VTOR())
                .field("LOCK_NS_MPU", &self.LOCK_NS_MPU())
                .field("LOCK_S_VTAIRCR", &self.LOCK_S_VTAIRCR())
                .field("LOCK_S_MPU", &self.LOCK_S_MPU())
                .field("LOCK_SAU", &self.LOCK_SAU())
                .field("CM33_LOCK_REG_LOCK", &self.CM33_LOCK_REG_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPU0_LOCK_REG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CPU0_LOCK_REG {{ LOCK_NS_VTOR: {=u8:?}, LOCK_NS_MPU: {=u8:?}, LOCK_S_VTAIRCR: {=u8:?}, LOCK_S_MPU: {=u8:?}, LOCK_SAU: {=u8:?}, CM33_LOCK_REG_LOCK: {=u8:?} }}" , self . LOCK_NS_VTOR () , self . LOCK_NS_MPU () , self . LOCK_S_VTAIRCR () , self . LOCK_S_MPU () , self . LOCK_SAU () , self . CM33_LOCK_REG_LOCK ())
        }
    }
    #[doc = "Flash Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLASH00_MEM_RULE(pub u32);
    impl FLASH00_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for FLASH00_MEM_RULE {
        #[inline(always)]
        fn default() -> FLASH00_MEM_RULE {
            FLASH00_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLASH00_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASH00_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASH00_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLASH00_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "Flash Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLASH02_MEM_RULE(pub u32);
    impl FLASH02_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for FLASH02_MEM_RULE {
        #[inline(always)]
        fn default() -> FLASH02_MEM_RULE {
            FLASH02_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLASH02_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASH02_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASH02_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLASH02_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 ())
        }
    }
    #[doc = "Flash Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLASH03_MEM_RULE(pub u32);
    impl FLASH03_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for FLASH03_MEM_RULE {
        #[inline(always)]
        fn default() -> FLASH03_MEM_RULE {
            FLASH03_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLASH03_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASH03_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASH03_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLASH03_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "Master Secure Level"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASTER_SEC_ANTI_POL_REG(pub u32);
    impl MASTER_SEC_ANTI_POL_REG {
        #[must_use]
        #[inline(always)]
        pub const fn SMARTDMA(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMARTDMA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PKC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PKC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_HS(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USB_HS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASTER_SEC_LEVEL_ANTIPOL_LOCK(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASTER_SEC_LEVEL_ANTIPOL_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for MASTER_SEC_ANTI_POL_REG {
        #[inline(always)]
        fn default() -> MASTER_SEC_ANTI_POL_REG {
            MASTER_SEC_ANTI_POL_REG(0)
        }
    }
    impl core::fmt::Debug for MASTER_SEC_ANTI_POL_REG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MASTER_SEC_ANTI_POL_REG")
                .field("SMARTDMA", &self.SMARTDMA())
                .field("eDMA0", &self.eDMA0())
                .field("eDMA1", &self.eDMA1())
                .field("PKC", &self.PKC())
                .field("USB_HS", &self.USB_HS())
                .field(
                    "MASTER_SEC_LEVEL_ANTIPOL_LOCK",
                    &self.MASTER_SEC_LEVEL_ANTIPOL_LOCK(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MASTER_SEC_ANTI_POL_REG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MASTER_SEC_ANTI_POL_REG {{ SMARTDMA: {=u8:?}, eDMA0: {=u8:?}, eDMA1: {=u8:?}, PKC: {=u8:?}, USB_HS: {=u8:?}, MASTER_SEC_LEVEL_ANTIPOL_LOCK: {=u8:?} }}" , self . SMARTDMA () , self . eDMA0 () , self . eDMA1 () , self . PKC () , self . USB_HS () , self . MASTER_SEC_LEVEL_ANTIPOL_LOCK ())
        }
    }
    #[doc = "Master Secure Level"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASTER_SEC_LEVEL(pub u32);
    impl MASTER_SEC_LEVEL {
        #[must_use]
        #[inline(always)]
        pub const fn SMARTDMA(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMARTDMA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PKC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PKC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_HS(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USB_HS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASTER_SEC_LEVEL_LOCK(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MASTER_SEC_LEVEL_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for MASTER_SEC_LEVEL {
        #[inline(always)]
        fn default() -> MASTER_SEC_LEVEL {
            MASTER_SEC_LEVEL(0)
        }
    }
    impl core::fmt::Debug for MASTER_SEC_LEVEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MASTER_SEC_LEVEL")
                .field("SMARTDMA", &self.SMARTDMA())
                .field("eDMA0", &self.eDMA0())
                .field("eDMA1", &self.eDMA1())
                .field("PKC", &self.PKC())
                .field("USB_HS", &self.USB_HS())
                .field("MASTER_SEC_LEVEL_LOCK", &self.MASTER_SEC_LEVEL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MASTER_SEC_LEVEL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MASTER_SEC_LEVEL {{ SMARTDMA: {=u8:?}, eDMA0: {=u8:?}, eDMA1: {=u8:?}, PKC: {=u8:?}, USB_HS: {=u8:?}, MASTER_SEC_LEVEL_LOCK: {=u8:?} }}" , self . SMARTDMA () , self . eDMA0 () , self . eDMA1 () , self . PKC () , self . USB_HS () , self . MASTER_SEC_LEVEL_LOCK ())
        }
    }
    #[doc = "Secure Control Duplicate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISC_CTRL_DP_REG(pub u32);
    impl MISC_CTRL_DP_REG {
        #[must_use]
        #[inline(always)]
        pub const fn WRITE_LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WRITE_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_SECURE_CHECKING(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_SECURE_CHECKING(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_S_PRIV_CHECK(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_S_PRIV_CHECK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_NS_PRIV_CHECK(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_NS_PRIV_CHECK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_VIOLATION_ABORT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DISABLE_VIOLATION_ABORT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_STRICT_MODE(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DISABLE_STRICT_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IDAU_ALL_NS(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IDAU_ALL_NS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for MISC_CTRL_DP_REG {
        #[inline(always)]
        fn default() -> MISC_CTRL_DP_REG {
            MISC_CTRL_DP_REG(0)
        }
    }
    impl core::fmt::Debug for MISC_CTRL_DP_REG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MISC_CTRL_DP_REG")
                .field("WRITE_LOCK", &self.WRITE_LOCK())
                .field("ENABLE_SECURE_CHECKING", &self.ENABLE_SECURE_CHECKING())
                .field("ENABLE_S_PRIV_CHECK", &self.ENABLE_S_PRIV_CHECK())
                .field("ENABLE_NS_PRIV_CHECK", &self.ENABLE_NS_PRIV_CHECK())
                .field("DISABLE_VIOLATION_ABORT", &self.DISABLE_VIOLATION_ABORT())
                .field("DISABLE_STRICT_MODE", &self.DISABLE_STRICT_MODE())
                .field("IDAU_ALL_NS", &self.IDAU_ALL_NS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MISC_CTRL_DP_REG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MISC_CTRL_DP_REG {{ WRITE_LOCK: {=u8:?}, ENABLE_SECURE_CHECKING: {=u8:?}, ENABLE_S_PRIV_CHECK: {=u8:?}, ENABLE_NS_PRIV_CHECK: {=u8:?}, DISABLE_VIOLATION_ABORT: {=u8:?}, DISABLE_STRICT_MODE: {=u8:?}, IDAU_ALL_NS: {=u8:?} }}" , self . WRITE_LOCK () , self . ENABLE_SECURE_CHECKING () , self . ENABLE_S_PRIV_CHECK () , self . ENABLE_NS_PRIV_CHECK () , self . DISABLE_VIOLATION_ABORT () , self . DISABLE_STRICT_MODE () , self . IDAU_ALL_NS ())
        }
    }
    #[doc = "Secure Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISC_CTRL_REG(pub u32);
    impl MISC_CTRL_REG {
        #[must_use]
        #[inline(always)]
        pub const fn WRITE_LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WRITE_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_SECURE_CHECKING(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_SECURE_CHECKING(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_S_PRIV_CHECK(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_S_PRIV_CHECK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENABLE_NS_PRIV_CHECK(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENABLE_NS_PRIV_CHECK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_VIOLATION_ABORT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DISABLE_VIOLATION_ABORT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_STRICT_MODE(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DISABLE_STRICT_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IDAU_ALL_NS(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IDAU_ALL_NS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for MISC_CTRL_REG {
        #[inline(always)]
        fn default() -> MISC_CTRL_REG {
            MISC_CTRL_REG(0)
        }
    }
    impl core::fmt::Debug for MISC_CTRL_REG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MISC_CTRL_REG")
                .field("WRITE_LOCK", &self.WRITE_LOCK())
                .field("ENABLE_SECURE_CHECKING", &self.ENABLE_SECURE_CHECKING())
                .field("ENABLE_S_PRIV_CHECK", &self.ENABLE_S_PRIV_CHECK())
                .field("ENABLE_NS_PRIV_CHECK", &self.ENABLE_NS_PRIV_CHECK())
                .field("DISABLE_VIOLATION_ABORT", &self.DISABLE_VIOLATION_ABORT())
                .field("DISABLE_STRICT_MODE", &self.DISABLE_STRICT_MODE())
                .field("IDAU_ALL_NS", &self.IDAU_ALL_NS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MISC_CTRL_REG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MISC_CTRL_REG {{ WRITE_LOCK: {=u8:?}, ENABLE_SECURE_CHECKING: {=u8:?}, ENABLE_S_PRIV_CHECK: {=u8:?}, ENABLE_NS_PRIV_CHECK: {=u8:?}, DISABLE_VIOLATION_ABORT: {=u8:?}, DISABLE_STRICT_MODE: {=u8:?}, IDAU_ALL_NS: {=u8:?} }}" , self . WRITE_LOCK () , self . ENABLE_SECURE_CHECKING () , self . ENABLE_S_PRIV_CHECK () , self . ENABLE_NS_PRIV_CHECK () , self . DISABLE_VIOLATION_ABORT () , self . DISABLE_STRICT_MODE () , self . IDAU_ALL_NS ())
        }
    }
    #[doc = "RAMA Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMA_MEM_RULE(pub u32);
    impl RAMA_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAMA_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMA_MEM_RULE {
            RAMA_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMA_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMA_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAMA_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMA_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMB Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMB_MEM_RULE(pub u32);
    impl RAMB_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAMB_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMB_MEM_RULE {
            RAMB_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMB_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMB_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAMB_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMB_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMC Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMC_MEM_RULE(pub u32);
    impl RAMC_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAMC_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMC_MEM_RULE {
            RAMC_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMC_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMC_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAMC_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMC_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMD Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMD_MEM_RULE(pub u32);
    impl RAMD_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAMD_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMD_MEM_RULE {
            RAMD_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMD_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMD_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAMD_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMD_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAME Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAME_MEM_RULE(pub u32);
    impl RAME_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAME_MEM_RULE {
        #[inline(always)]
        fn default() -> RAME_MEM_RULE {
            RAME_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAME_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAME_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAME_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAME_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMX Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMX_MEM_RULE(pub u32);
    impl RAMX_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for RAMX_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMX_MEM_RULE {
            RAMX_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMX_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMX_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAMX_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMX_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "ROM Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ROM_MEM_RULE(pub u32);
    impl ROM_MEM_RULE {
        #[must_use]
        #[inline(always)]
        pub const fn RULE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RULE7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RULE7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for ROM_MEM_RULE {
        #[inline(always)]
        fn default() -> ROM_MEM_RULE {
            ROM_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for ROM_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ROM_MEM_RULE")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .field("RULE6", &self.RULE6())
                .field("RULE7", &self.RULE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ROM_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ROM_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "GPIO Mask for Port 0..GPIO Mask for Port 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_GPIO_MASK(pub u32);
    impl SEC_GPIO_MASK {
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN0_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN0_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN0_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN0_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN1_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN1_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN1_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN1_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN2_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN2_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN2_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN2_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN3_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN3_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN3_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN3_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN4_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN4_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN4_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN4_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN5_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN5_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN5_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN5_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN6_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN6_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN6_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN6_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN7_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN7_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN7_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN7_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN8_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN8_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN8_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN8_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN9_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN9_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN9_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN9_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN10_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN10_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN10_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN10_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN11_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN11_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN11_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN11_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN12_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN12_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN12_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN12_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN13_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN13_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN13_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN13_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN14_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN14_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN14_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN14_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN15_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN15_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN15_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN15_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN16_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN16_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN16_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN16_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN17_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN17_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN17_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN17_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN18_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN18_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN18_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN18_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN19_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN19_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN19_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN19_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN20_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN20_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN20_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN20_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN21_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN21_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN21_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN21_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN22_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN22_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN22_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN22_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN23_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN23_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN23_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN23_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN24_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN24_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN24_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN24_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN25_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN25_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN25_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN25_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN26_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN26_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN26_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN26_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN27_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN27_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN27_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN27_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN28_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN28_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN28_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN28_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN29_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN29_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN29_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN29_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN30_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN30_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN30_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN30_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO0_PIN31_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO0_PIN31_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIO1_PIN31_SEC_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIO1_PIN31_SEC_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_GPIO_MASK {
        #[inline(always)]
        fn default() -> SEC_GPIO_MASK {
            SEC_GPIO_MASK(0)
        }
    }
    impl core::fmt::Debug for SEC_GPIO_MASK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_GPIO_MASK")
                .field("PIO0_PIN0_SEC_MASK", &self.PIO0_PIN0_SEC_MASK())
                .field("PIO1_PIN0_SEC_MASK", &self.PIO1_PIN0_SEC_MASK())
                .field("PIO0_PIN1_SEC_MASK", &self.PIO0_PIN1_SEC_MASK())
                .field("PIO1_PIN1_SEC_MASK", &self.PIO1_PIN1_SEC_MASK())
                .field("PIO0_PIN2_SEC_MASK", &self.PIO0_PIN2_SEC_MASK())
                .field("PIO1_PIN2_SEC_MASK", &self.PIO1_PIN2_SEC_MASK())
                .field("PIO0_PIN3_SEC_MASK", &self.PIO0_PIN3_SEC_MASK())
                .field("PIO1_PIN3_SEC_MASK", &self.PIO1_PIN3_SEC_MASK())
                .field("PIO0_PIN4_SEC_MASK", &self.PIO0_PIN4_SEC_MASK())
                .field("PIO1_PIN4_SEC_MASK", &self.PIO1_PIN4_SEC_MASK())
                .field("PIO0_PIN5_SEC_MASK", &self.PIO0_PIN5_SEC_MASK())
                .field("PIO1_PIN5_SEC_MASK", &self.PIO1_PIN5_SEC_MASK())
                .field("PIO0_PIN6_SEC_MASK", &self.PIO0_PIN6_SEC_MASK())
                .field("PIO1_PIN6_SEC_MASK", &self.PIO1_PIN6_SEC_MASK())
                .field("PIO0_PIN7_SEC_MASK", &self.PIO0_PIN7_SEC_MASK())
                .field("PIO1_PIN7_SEC_MASK", &self.PIO1_PIN7_SEC_MASK())
                .field("PIO0_PIN8_SEC_MASK", &self.PIO0_PIN8_SEC_MASK())
                .field("PIO1_PIN8_SEC_MASK", &self.PIO1_PIN8_SEC_MASK())
                .field("PIO0_PIN9_SEC_MASK", &self.PIO0_PIN9_SEC_MASK())
                .field("PIO1_PIN9_SEC_MASK", &self.PIO1_PIN9_SEC_MASK())
                .field("PIO0_PIN10_SEC_MASK", &self.PIO0_PIN10_SEC_MASK())
                .field("PIO1_PIN10_SEC_MASK", &self.PIO1_PIN10_SEC_MASK())
                .field("PIO0_PIN11_SEC_MASK", &self.PIO0_PIN11_SEC_MASK())
                .field("PIO1_PIN11_SEC_MASK", &self.PIO1_PIN11_SEC_MASK())
                .field("PIO0_PIN12_SEC_MASK", &self.PIO0_PIN12_SEC_MASK())
                .field("PIO1_PIN12_SEC_MASK", &self.PIO1_PIN12_SEC_MASK())
                .field("PIO0_PIN13_SEC_MASK", &self.PIO0_PIN13_SEC_MASK())
                .field("PIO1_PIN13_SEC_MASK", &self.PIO1_PIN13_SEC_MASK())
                .field("PIO0_PIN14_SEC_MASK", &self.PIO0_PIN14_SEC_MASK())
                .field("PIO1_PIN14_SEC_MASK", &self.PIO1_PIN14_SEC_MASK())
                .field("PIO0_PIN15_SEC_MASK", &self.PIO0_PIN15_SEC_MASK())
                .field("PIO1_PIN15_SEC_MASK", &self.PIO1_PIN15_SEC_MASK())
                .field("PIO0_PIN16_SEC_MASK", &self.PIO0_PIN16_SEC_MASK())
                .field("PIO1_PIN16_SEC_MASK", &self.PIO1_PIN16_SEC_MASK())
                .field("PIO0_PIN17_SEC_MASK", &self.PIO0_PIN17_SEC_MASK())
                .field("PIO1_PIN17_SEC_MASK", &self.PIO1_PIN17_SEC_MASK())
                .field("PIO0_PIN18_SEC_MASK", &self.PIO0_PIN18_SEC_MASK())
                .field("PIO1_PIN18_SEC_MASK", &self.PIO1_PIN18_SEC_MASK())
                .field("PIO0_PIN19_SEC_MASK", &self.PIO0_PIN19_SEC_MASK())
                .field("PIO1_PIN19_SEC_MASK", &self.PIO1_PIN19_SEC_MASK())
                .field("PIO0_PIN20_SEC_MASK", &self.PIO0_PIN20_SEC_MASK())
                .field("PIO1_PIN20_SEC_MASK", &self.PIO1_PIN20_SEC_MASK())
                .field("PIO0_PIN21_SEC_MASK", &self.PIO0_PIN21_SEC_MASK())
                .field("PIO1_PIN21_SEC_MASK", &self.PIO1_PIN21_SEC_MASK())
                .field("PIO0_PIN22_SEC_MASK", &self.PIO0_PIN22_SEC_MASK())
                .field("PIO1_PIN22_SEC_MASK", &self.PIO1_PIN22_SEC_MASK())
                .field("PIO0_PIN23_SEC_MASK", &self.PIO0_PIN23_SEC_MASK())
                .field("PIO1_PIN23_SEC_MASK", &self.PIO1_PIN23_SEC_MASK())
                .field("PIO0_PIN24_SEC_MASK", &self.PIO0_PIN24_SEC_MASK())
                .field("PIO1_PIN24_SEC_MASK", &self.PIO1_PIN24_SEC_MASK())
                .field("PIO0_PIN25_SEC_MASK", &self.PIO0_PIN25_SEC_MASK())
                .field("PIO1_PIN25_SEC_MASK", &self.PIO1_PIN25_SEC_MASK())
                .field("PIO0_PIN26_SEC_MASK", &self.PIO0_PIN26_SEC_MASK())
                .field("PIO1_PIN26_SEC_MASK", &self.PIO1_PIN26_SEC_MASK())
                .field("PIO0_PIN27_SEC_MASK", &self.PIO0_PIN27_SEC_MASK())
                .field("PIO1_PIN27_SEC_MASK", &self.PIO1_PIN27_SEC_MASK())
                .field("PIO0_PIN28_SEC_MASK", &self.PIO0_PIN28_SEC_MASK())
                .field("PIO1_PIN28_SEC_MASK", &self.PIO1_PIN28_SEC_MASK())
                .field("PIO0_PIN29_SEC_MASK", &self.PIO0_PIN29_SEC_MASK())
                .field("PIO1_PIN29_SEC_MASK", &self.PIO1_PIN29_SEC_MASK())
                .field("PIO0_PIN30_SEC_MASK", &self.PIO0_PIN30_SEC_MASK())
                .field("PIO1_PIN30_SEC_MASK", &self.PIO1_PIN30_SEC_MASK())
                .field("PIO0_PIN31_SEC_MASK", &self.PIO0_PIN31_SEC_MASK())
                .field("PIO1_PIN31_SEC_MASK", &self.PIO1_PIN31_SEC_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_GPIO_MASK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_GPIO_MASK {{ PIO0_PIN0_SEC_MASK: {=bool:?}, PIO1_PIN0_SEC_MASK: {=bool:?}, PIO0_PIN1_SEC_MASK: {=bool:?}, PIO1_PIN1_SEC_MASK: {=bool:?}, PIO0_PIN2_SEC_MASK: {=bool:?}, PIO1_PIN2_SEC_MASK: {=bool:?}, PIO0_PIN3_SEC_MASK: {=bool:?}, PIO1_PIN3_SEC_MASK: {=bool:?}, PIO0_PIN4_SEC_MASK: {=bool:?}, PIO1_PIN4_SEC_MASK: {=bool:?}, PIO0_PIN5_SEC_MASK: {=bool:?}, PIO1_PIN5_SEC_MASK: {=bool:?}, PIO0_PIN6_SEC_MASK: {=bool:?}, PIO1_PIN6_SEC_MASK: {=bool:?}, PIO0_PIN7_SEC_MASK: {=bool:?}, PIO1_PIN7_SEC_MASK: {=bool:?}, PIO0_PIN8_SEC_MASK: {=bool:?}, PIO1_PIN8_SEC_MASK: {=bool:?}, PIO0_PIN9_SEC_MASK: {=bool:?}, PIO1_PIN9_SEC_MASK: {=bool:?}, PIO0_PIN10_SEC_MASK: {=bool:?}, PIO1_PIN10_SEC_MASK: {=bool:?}, PIO0_PIN11_SEC_MASK: {=bool:?}, PIO1_PIN11_SEC_MASK: {=bool:?}, PIO0_PIN12_SEC_MASK: {=bool:?}, PIO1_PIN12_SEC_MASK: {=bool:?}, PIO0_PIN13_SEC_MASK: {=bool:?}, PIO1_PIN13_SEC_MASK: {=bool:?}, PIO0_PIN14_SEC_MASK: {=bool:?}, PIO1_PIN14_SEC_MASK: {=bool:?}, PIO0_PIN15_SEC_MASK: {=bool:?}, PIO1_PIN15_SEC_MASK: {=bool:?}, PIO0_PIN16_SEC_MASK: {=bool:?}, PIO1_PIN16_SEC_MASK: {=bool:?}, PIO0_PIN17_SEC_MASK: {=bool:?}, PIO1_PIN17_SEC_MASK: {=bool:?}, PIO0_PIN18_SEC_MASK: {=bool:?}, PIO1_PIN18_SEC_MASK: {=bool:?}, PIO0_PIN19_SEC_MASK: {=bool:?}, PIO1_PIN19_SEC_MASK: {=bool:?}, PIO0_PIN20_SEC_MASK: {=bool:?}, PIO1_PIN20_SEC_MASK: {=bool:?}, PIO0_PIN21_SEC_MASK: {=bool:?}, PIO1_PIN21_SEC_MASK: {=bool:?}, PIO0_PIN22_SEC_MASK: {=bool:?}, PIO1_PIN22_SEC_MASK: {=bool:?}, PIO0_PIN23_SEC_MASK: {=bool:?}, PIO1_PIN23_SEC_MASK: {=bool:?}, PIO0_PIN24_SEC_MASK: {=bool:?}, PIO1_PIN24_SEC_MASK: {=bool:?}, PIO0_PIN25_SEC_MASK: {=bool:?}, PIO1_PIN25_SEC_MASK: {=bool:?}, PIO0_PIN26_SEC_MASK: {=bool:?}, PIO1_PIN26_SEC_MASK: {=bool:?}, PIO0_PIN27_SEC_MASK: {=bool:?}, PIO1_PIN27_SEC_MASK: {=bool:?}, PIO0_PIN28_SEC_MASK: {=bool:?}, PIO1_PIN28_SEC_MASK: {=bool:?}, PIO0_PIN29_SEC_MASK: {=bool:?}, PIO1_PIN29_SEC_MASK: {=bool:?}, PIO0_PIN30_SEC_MASK: {=bool:?}, PIO1_PIN30_SEC_MASK: {=bool:?}, PIO0_PIN31_SEC_MASK: {=bool:?}, PIO1_PIN31_SEC_MASK: {=bool:?} }}" , self . PIO0_PIN0_SEC_MASK () , self . PIO1_PIN0_SEC_MASK () , self . PIO0_PIN1_SEC_MASK () , self . PIO1_PIN1_SEC_MASK () , self . PIO0_PIN2_SEC_MASK () , self . PIO1_PIN2_SEC_MASK () , self . PIO0_PIN3_SEC_MASK () , self . PIO1_PIN3_SEC_MASK () , self . PIO0_PIN4_SEC_MASK () , self . PIO1_PIN4_SEC_MASK () , self . PIO0_PIN5_SEC_MASK () , self . PIO1_PIN5_SEC_MASK () , self . PIO0_PIN6_SEC_MASK () , self . PIO1_PIN6_SEC_MASK () , self . PIO0_PIN7_SEC_MASK () , self . PIO1_PIN7_SEC_MASK () , self . PIO0_PIN8_SEC_MASK () , self . PIO1_PIN8_SEC_MASK () , self . PIO0_PIN9_SEC_MASK () , self . PIO1_PIN9_SEC_MASK () , self . PIO0_PIN10_SEC_MASK () , self . PIO1_PIN10_SEC_MASK () , self . PIO0_PIN11_SEC_MASK () , self . PIO1_PIN11_SEC_MASK () , self . PIO0_PIN12_SEC_MASK () , self . PIO1_PIN12_SEC_MASK () , self . PIO0_PIN13_SEC_MASK () , self . PIO1_PIN13_SEC_MASK () , self . PIO0_PIN14_SEC_MASK () , self . PIO1_PIN14_SEC_MASK () , self . PIO0_PIN15_SEC_MASK () , self . PIO1_PIN15_SEC_MASK () , self . PIO0_PIN16_SEC_MASK () , self . PIO1_PIN16_SEC_MASK () , self . PIO0_PIN17_SEC_MASK () , self . PIO1_PIN17_SEC_MASK () , self . PIO0_PIN18_SEC_MASK () , self . PIO1_PIN18_SEC_MASK () , self . PIO0_PIN19_SEC_MASK () , self . PIO1_PIN19_SEC_MASK () , self . PIO0_PIN20_SEC_MASK () , self . PIO1_PIN20_SEC_MASK () , self . PIO0_PIN21_SEC_MASK () , self . PIO1_PIN21_SEC_MASK () , self . PIO0_PIN22_SEC_MASK () , self . PIO1_PIN22_SEC_MASK () , self . PIO0_PIN23_SEC_MASK () , self . PIO1_PIN23_SEC_MASK () , self . PIO0_PIN24_SEC_MASK () , self . PIO1_PIN24_SEC_MASK () , self . PIO0_PIN25_SEC_MASK () , self . PIO1_PIN25_SEC_MASK () , self . PIO0_PIN26_SEC_MASK () , self . PIO1_PIN26_SEC_MASK () , self . PIO0_PIN27_SEC_MASK () , self . PIO1_PIN27_SEC_MASK () , self . PIO0_PIN28_SEC_MASK () , self . PIO1_PIN28_SEC_MASK () , self . PIO0_PIN29_SEC_MASK () , self . PIO1_PIN29_SEC_MASK () , self . PIO0_PIN30_SEC_MASK () , self . PIO1_PIN30_SEC_MASK () , self . PIO0_PIN31_SEC_MASK () , self . PIO1_PIN31_SEC_MASK ())
        }
    }
    #[doc = "Security Violation Info Validity for Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_VIO_INFO_VALID(pub u32);
    impl SEC_VIO_INFO_VALID {
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VIO_INFO_VALID18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VIO_INFO_VALID18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for SEC_VIO_INFO_VALID {
        #[inline(always)]
        fn default() -> SEC_VIO_INFO_VALID {
            SEC_VIO_INFO_VALID(0)
        }
    }
    impl core::fmt::Debug for SEC_VIO_INFO_VALID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_VIO_INFO_VALID")
                .field("VIO_INFO_VALID0", &self.VIO_INFO_VALID0())
                .field("VIO_INFO_VALID1", &self.VIO_INFO_VALID1())
                .field("VIO_INFO_VALID2", &self.VIO_INFO_VALID2())
                .field("VIO_INFO_VALID3", &self.VIO_INFO_VALID3())
                .field("VIO_INFO_VALID4", &self.VIO_INFO_VALID4())
                .field("VIO_INFO_VALID5", &self.VIO_INFO_VALID5())
                .field("VIO_INFO_VALID6", &self.VIO_INFO_VALID6())
                .field("VIO_INFO_VALID7", &self.VIO_INFO_VALID7())
                .field("VIO_INFO_VALID8", &self.VIO_INFO_VALID8())
                .field("VIO_INFO_VALID9", &self.VIO_INFO_VALID9())
                .field("VIO_INFO_VALID10", &self.VIO_INFO_VALID10())
                .field("VIO_INFO_VALID11", &self.VIO_INFO_VALID11())
                .field("VIO_INFO_VALID12", &self.VIO_INFO_VALID12())
                .field("VIO_INFO_VALID13", &self.VIO_INFO_VALID13())
                .field("VIO_INFO_VALID14", &self.VIO_INFO_VALID14())
                .field("VIO_INFO_VALID15", &self.VIO_INFO_VALID15())
                .field("VIO_INFO_VALID16", &self.VIO_INFO_VALID16())
                .field("VIO_INFO_VALID17", &self.VIO_INFO_VALID17())
                .field("VIO_INFO_VALID18", &self.VIO_INFO_VALID18())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_VIO_INFO_VALID {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_VIO_INFO_VALID {{ VIO_INFO_VALID0: {=bool:?}, VIO_INFO_VALID1: {=bool:?}, VIO_INFO_VALID2: {=bool:?}, VIO_INFO_VALID3: {=bool:?}, VIO_INFO_VALID4: {=bool:?}, VIO_INFO_VALID5: {=bool:?}, VIO_INFO_VALID6: {=bool:?}, VIO_INFO_VALID7: {=bool:?}, VIO_INFO_VALID8: {=bool:?}, VIO_INFO_VALID9: {=bool:?}, VIO_INFO_VALID10: {=bool:?}, VIO_INFO_VALID11: {=bool:?}, VIO_INFO_VALID12: {=bool:?}, VIO_INFO_VALID13: {=bool:?}, VIO_INFO_VALID14: {=bool:?}, VIO_INFO_VALID15: {=bool:?}, VIO_INFO_VALID16: {=bool:?}, VIO_INFO_VALID17: {=bool:?}, VIO_INFO_VALID18: {=bool:?} }}" , self . VIO_INFO_VALID0 () , self . VIO_INFO_VALID1 () , self . VIO_INFO_VALID2 () , self . VIO_INFO_VALID3 () , self . VIO_INFO_VALID4 () , self . VIO_INFO_VALID5 () , self . VIO_INFO_VALID6 () , self . VIO_INFO_VALID7 () , self . VIO_INFO_VALID8 () , self . VIO_INFO_VALID9 () , self . VIO_INFO_VALID10 () , self . VIO_INFO_VALID11 () , self . VIO_INFO_VALID12 () , self . VIO_INFO_VALID13 () , self . VIO_INFO_VALID14 () , self . VIO_INFO_VALID15 () , self . VIO_INFO_VALID16 () , self . VIO_INFO_VALID17 () , self . VIO_INFO_VALID18 ())
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_VIO_MISC_INFO(pub u32);
    impl SEC_VIO_MISC_INFO {
        #[must_use]
        #[inline(always)]
        pub const fn SEC_VIO_INFO_WRITE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SEC_VIO_INFO_WRITE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_VIO_INFO_DATA_ACCESS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SEC_VIO_INFO_DATA_ACCESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_VIO_INFO_MASTER_SEC_LEVEL(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_VIO_INFO_MASTER_SEC_LEVEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_VIO_INFO_MASTER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_VIO_INFO_MASTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for SEC_VIO_MISC_INFO {
        #[inline(always)]
        fn default() -> SEC_VIO_MISC_INFO {
            SEC_VIO_MISC_INFO(0)
        }
    }
    impl core::fmt::Debug for SEC_VIO_MISC_INFO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_VIO_MISC_INFO")
                .field("SEC_VIO_INFO_WRITE", &self.SEC_VIO_INFO_WRITE())
                .field("SEC_VIO_INFO_DATA_ACCESS", &self.SEC_VIO_INFO_DATA_ACCESS())
                .field(
                    "SEC_VIO_INFO_MASTER_SEC_LEVEL",
                    &self.SEC_VIO_INFO_MASTER_SEC_LEVEL(),
                )
                .field("SEC_VIO_INFO_MASTER", &self.SEC_VIO_INFO_MASTER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_VIO_MISC_INFO {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_VIO_MISC_INFO {{ SEC_VIO_INFO_WRITE: {=bool:?}, SEC_VIO_INFO_DATA_ACCESS: {=bool:?}, SEC_VIO_INFO_MASTER_SEC_LEVEL: {=u8:?}, SEC_VIO_INFO_MASTER: {=u8:?} }}" , self . SEC_VIO_INFO_WRITE () , self . SEC_VIO_INFO_DATA_ACCESS () , self . SEC_VIO_INFO_MASTER_SEC_LEVEL () , self . SEC_VIO_INFO_MASTER ())
        }
    }
}
