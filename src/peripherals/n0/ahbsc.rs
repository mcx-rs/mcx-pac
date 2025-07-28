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
    pub const fn FLASH01_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLASH01_MEM_RULE, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
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
    pub const fn RAMF_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAMF_MEM_RULE, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMG_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RAMG_MEM_RULE, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RAMH_MEM_RULE(self) -> crate::common::Reg<regs::RAMH_MEM_RULE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
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
    ) -> crate::common::Reg<regs::AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2, crate::common::RW>
    {
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
    pub const fn FLEXSPI0_REGION0_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLEXSPI0_REGION0_MEM_RULE, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXSPI0_REGION1_6_MEM_RULE(self, n: usize) -> FLEXSPI0_REGION1_6_MEM_RULE {
        assert!(n < 6usize);
        unsafe {
            FLEXSPI0_REGION1_6_MEM_RULE::from_ptr(self.ptr.add(0x0280usize + n * 16usize) as _)
        }
    }
    #[inline(always)]
    pub const fn FLEXSPI0_REGION7_MEM_RULE(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLEXSPI0_REGION7_MEM_RULE, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXSPI0_REGION8_13_MEM_RULE(self, n: usize) -> FLEXSPI0_REGION8_13_MEM_RULE {
        assert!(n < 6usize);
        unsafe {
            FLEXSPI0_REGION8_13_MEM_RULE::from_ptr(self.ptr.add(0x02f0usize + n * 16usize) as _)
        }
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
    pub const fn SEC_CPU1_INT_MASK0(
        self,
    ) -> crate::common::Reg<regs::SEC_CPU1_INT_MASK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f98usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK1(
        self,
    ) -> crate::common::Reg<regs::SEC_CPU1_INT_MASK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f9cusize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK2(
        self,
    ) -> crate::common::Reg<regs::SEC_CPU1_INT_MASK2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa0usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK3(
        self,
    ) -> crate::common::Reg<regs::SEC_CPU1_INT_MASK3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa4usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_CPU1_INT_MASK4(
        self,
    ) -> crate::common::Reg<regs::SEC_CPU1_INT_MASK4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa8usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_GP_REG_LOCK(
        self,
    ) -> crate::common::Reg<regs::SEC_GP_REG_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fbcusize) as _) }
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
    pub const fn CPU1_LOCK_REG(self) -> crate::common::Reg<regs::CPU1_LOCK_REG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI0_REGION1_6_MEM_RULE {
    ptr: *mut u8,
}
unsafe impl Send for FLEXSPI0_REGION1_6_MEM_RULE {}
unsafe impl Sync for FLEXSPI0_REGION1_6_MEM_RULE {}
impl FLEXSPI0_REGION1_6_MEM_RULE {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn FLEXSPI0_REGION_MEM_RULE0(
        self,
    ) -> crate::common::Reg<
        regs::FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0,
        crate::common::RW,
    > {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI0_REGION8_13_MEM_RULE {
    ptr: *mut u8,
}
unsafe impl Send for FLEXSPI0_REGION8_13_MEM_RULE {}
unsafe impl Sync for FLEXSPI0_REGION8_13_MEM_RULE {}
impl FLEXSPI0_REGION8_13_MEM_RULE {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn FLEXSPI0_REGION_MEM_RULE0(
        self,
    ) -> crate::common::Reg<
        regs::FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0,
        crate::common::RW,
    > {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
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
    #[doc = "AHB Peripheral 1 Slave Port 13 Slave Rule 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2(pub u32);
    impl AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {
        #[must_use]
        #[inline(always)]
        pub const fn POWERQUAD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_POWERQUAD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {
        #[inline(always)]
        fn default() -> AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {
            AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2(0)
        }
    }
    impl core::fmt::Debug for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2")
                .field("POWERQUAD", &self.POWERQUAD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHB_PERIPHERAL1_SLAVE_PORT_P13_SLAVE_RULE2 {{ POWERQUAD: {=u8:?} }}",
                self.POWERQUAD()
            )
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
        #[must_use]
        #[inline(always)]
        pub const fn TRO0(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRO0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
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
                .field("TRO0", &self.TRO0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP0_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP0_MEM_RULE0 {{ GPIO5_ALIAS0: {=u8:?}, GPIO5_ALIAS1: {=u8:?}, PORT5: {=u8:?}, FMU0: {=u8:?}, SCG0: {=u8:?}, SPC0: {=u8:?}, WUU0: {=u8:?}, TRO0: {=u8:?} }}" , self . GPIO5_ALIAS0 () , self . GPIO5_ALIAS1 () , self . PORT5 () , self . FMU0 () , self . SCG0 () , self . SPC0 () , self . WUU0 () , self . TRO0 ())
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
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH8(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH9(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH10(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH11(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH12(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH13(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn eDMA1_CH14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_eDMA1_CH14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
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
                .field("eDMA1_CH8", &self.eDMA1_CH8())
                .field("eDMA1_CH9", &self.eDMA1_CH9())
                .field("eDMA1_CH10", &self.eDMA1_CH10())
                .field("eDMA1_CH11", &self.eDMA1_CH11())
                .field("eDMA1_CH12", &self.eDMA1_CH12())
                .field("eDMA1_CH13", &self.eDMA1_CH13())
                .field("eDMA1_CH14", &self.eDMA1_CH14())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP2_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP2_MEM_RULE1 {{ eDMA1_CH7: {=u8:?}, eDMA1_CH8: {=u8:?}, eDMA1_CH9: {=u8:?}, eDMA1_CH10: {=u8:?}, eDMA1_CH11: {=u8:?}, eDMA1_CH12: {=u8:?}, eDMA1_CH13: {=u8:?}, eDMA1_CH14: {=u8:?} }}" , self . eDMA1_CH7 () , self . eDMA1_CH8 () , self . eDMA1_CH9 () , self . eDMA1_CH10 () , self . eDMA1_CH11 () , self . eDMA1_CH12 () , self . eDMA1_CH13 () , self . eDMA1_CH14 ())
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
        pub const fn ENC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENC(&mut self, val: u8) {
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
                .field("ENC", &self.ENC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AIPS_BRIDGE_GROUP3_MEM_RULE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE1 {{ FLEXSPI: {=u8:?}, OTPC: {=u8:?}, CRC: {=u8:?}, NPX: {=u8:?}, PWM: {=u8:?}, ENC: {=u8:?} }}" , self . FLEXSPI () , self . OTPC () , self . CRC () , self . NPX () , self . PWM () , self . ENC ())
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
        pub const fn ENC1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENC1(&mut self, val: u8) {
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
                .field("ENC1", &self.ENC1())
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
            defmt :: write ! (f , "AIPS_BRIDGE_GROUP3_MEM_RULE2 {{ PWM1: {=u8:?}, ENC1: {=u8:?}, EVTG: {=u8:?}, CAN0_RULE0: {=u8:?}, CAN0_RULE1: {=u8:?}, CAN0_RULE2: {=u8:?}, CAN0_RULE3: {=u8:?} }}" , self . PWM1 () , self . ENC1 () , self . EVTG () , self . CAN0_RULE0 () , self . CAN0_RULE1 () , self . CAN0_RULE2 () , self . CAN0_RULE3 ())
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
        pub const fn SM3(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SM3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
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
                .field("SM3", &self.SM3())
                .field("COOLFLUX", &self.COOLFLUX())
                .field("SMARTDMA", &self.SMARTDMA())
                .field("PLU", &self.PLU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APB_PERIPHERAL_GROUP1_MEM_RULE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APB_PERIPHERAL_GROUP1_MEM_RULE2 {{ SM3: {=u8:?}, COOLFLUX: {=u8:?}, SMARTDMA: {=u8:?}, PLU: {=u8:?} }}" , self . SM3 () , self . COOLFLUX () , self . SMARTDMA () , self . PLU ())
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
    #[doc = "Miscellaneous CPU1 Control Signals"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPU1_LOCK_REG(pub u32);
    impl CPU1_LOCK_REG {
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
    }
    impl Default for CPU1_LOCK_REG {
        #[inline(always)]
        fn default() -> CPU1_LOCK_REG {
            CPU1_LOCK_REG(0)
        }
    }
    impl core::fmt::Debug for CPU1_LOCK_REG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPU1_LOCK_REG")
                .field("LOCK_NS_VTOR", &self.LOCK_NS_VTOR())
                .field("LOCK_NS_MPU", &self.LOCK_NS_MPU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPU1_LOCK_REG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CPU1_LOCK_REG {{ LOCK_NS_VTOR: {=u8:?}, LOCK_NS_MPU: {=u8:?} }}",
                self.LOCK_NS_VTOR(),
                self.LOCK_NS_MPU()
            )
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
    pub struct FLASH01_MEM_RULE(pub u32);
    impl FLASH01_MEM_RULE {
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
    impl Default for FLASH01_MEM_RULE {
        #[inline(always)]
        fn default() -> FLASH01_MEM_RULE {
            FLASH01_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLASH01_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASH01_MEM_RULE")
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
    impl defmt::Format for FLASH01_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLASH01_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
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
    #[doc = "FLEXSPI0 Region 0 Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXSPI0_REGION0_MEM_RULE(pub u32);
    impl FLEXSPI0_REGION0_MEM_RULE {
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
    impl Default for FLEXSPI0_REGION0_MEM_RULE {
        #[inline(always)]
        fn default() -> FLEXSPI0_REGION0_MEM_RULE {
            FLEXSPI0_REGION0_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLEXSPI0_REGION0_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXSPI0_REGION0_MEM_RULE")
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
    impl defmt::Format for FLEXSPI0_REGION0_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLEXSPI0_REGION0_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "FLEXSPI0 Region 1 Memory Rule 0..FLEXSPI0 Region 6 Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0(pub u32);
    impl FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
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
    }
    impl Default for FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        #[inline(always)]
        fn default() -> FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
            FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLEXSPI0_REGION1_6_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 ())
        }
    }
    #[doc = "FLEXSPI0 Region 7 Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXSPI0_REGION7_MEM_RULE(pub u32);
    impl FLEXSPI0_REGION7_MEM_RULE {
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
    impl Default for FLEXSPI0_REGION7_MEM_RULE {
        #[inline(always)]
        fn default() -> FLEXSPI0_REGION7_MEM_RULE {
            FLEXSPI0_REGION7_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for FLEXSPI0_REGION7_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXSPI0_REGION7_MEM_RULE")
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
    impl defmt::Format for FLEXSPI0_REGION7_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLEXSPI0_REGION7_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "FLEXSPI0 Region 8 Memory Rule 0..FLEXSPI0 Region 13 Memory Rule 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0(pub u32);
    impl FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
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
    }
    impl Default for FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        #[inline(always)]
        fn default() -> FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
            FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0(0)
        }
    }
    impl core::fmt::Debug for FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0")
                .field("RULE0", &self.RULE0())
                .field("RULE1", &self.RULE1())
                .field("RULE2", &self.RULE2())
                .field("RULE3", &self.RULE3())
                .field("RULE4", &self.RULE4())
                .field("RULE5", &self.RULE5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLEXSPI0_REGION8_13_MEM_RULE_FLEXSPI0_REGION_MEM_RULE0 {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 ())
        }
    }
    #[doc = "Master Secure Level"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASTER_SEC_ANTI_POL_REG(pub u32);
    impl MASTER_SEC_ANTI_POL_REG {
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
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
        pub const fn PQ(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPUO(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NPUO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COOLFLUXI(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_COOLFLUXI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_FS(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USB_FS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETHERNET(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ETHERNET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
        pub const fn USDHC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USDHC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
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
                .field("CPU1", &self.CPU1())
                .field("SMARTDMA", &self.SMARTDMA())
                .field("eDMA0", &self.eDMA0())
                .field("eDMA1", &self.eDMA1())
                .field("PKC", &self.PKC())
                .field("PQ", &self.PQ())
                .field("NPUO", &self.NPUO())
                .field("COOLFLUXI", &self.COOLFLUXI())
                .field("USB_FS", &self.USB_FS())
                .field("ETHERNET", &self.ETHERNET())
                .field("USB_HS", &self.USB_HS())
                .field("USDHC", &self.USDHC())
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
            defmt :: write ! (f , "MASTER_SEC_ANTI_POL_REG {{ CPU1: {=u8:?}, SMARTDMA: {=u8:?}, eDMA0: {=u8:?}, eDMA1: {=u8:?}, PKC: {=u8:?}, PQ: {=u8:?}, NPUO: {=u8:?}, COOLFLUXI: {=u8:?}, USB_FS: {=u8:?}, ETHERNET: {=u8:?}, USB_HS: {=u8:?}, USDHC: {=u8:?}, MASTER_SEC_LEVEL_ANTIPOL_LOCK: {=u8:?} }}" , self . CPU1 () , self . SMARTDMA () , self . eDMA0 () , self . eDMA1 () , self . PKC () , self . PQ () , self . NPUO () , self . COOLFLUXI () , self . USB_FS () , self . ETHERNET () , self . USB_HS () , self . USDHC () , self . MASTER_SEC_LEVEL_ANTIPOL_LOCK ())
        }
    }
    #[doc = "Master Secure Level"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MASTER_SEC_LEVEL(pub u32);
    impl MASTER_SEC_LEVEL {
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
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
        pub const fn PQ(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPUO(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NPUO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COOLFLUXI(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_COOLFLUXI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_FS(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USB_FS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETHERNET(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ETHERNET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
        pub const fn USDHC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USDHC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
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
                .field("CPU1", &self.CPU1())
                .field("SMARTDMA", &self.SMARTDMA())
                .field("eDMA0", &self.eDMA0())
                .field("eDMA1", &self.eDMA1())
                .field("PKC", &self.PKC())
                .field("PQ", &self.PQ())
                .field("NPUO", &self.NPUO())
                .field("COOLFLUXI", &self.COOLFLUXI())
                .field("USB_FS", &self.USB_FS())
                .field("ETHERNET", &self.ETHERNET())
                .field("USB_HS", &self.USB_HS())
                .field("USDHC", &self.USDHC())
                .field("MASTER_SEC_LEVEL_LOCK", &self.MASTER_SEC_LEVEL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MASTER_SEC_LEVEL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MASTER_SEC_LEVEL {{ CPU1: {=u8:?}, SMARTDMA: {=u8:?}, eDMA0: {=u8:?}, eDMA1: {=u8:?}, PKC: {=u8:?}, PQ: {=u8:?}, NPUO: {=u8:?}, COOLFLUXI: {=u8:?}, USB_FS: {=u8:?}, ETHERNET: {=u8:?}, USB_HS: {=u8:?}, USDHC: {=u8:?}, MASTER_SEC_LEVEL_LOCK: {=u8:?} }}" , self . CPU1 () , self . SMARTDMA () , self . eDMA0 () , self . eDMA1 () , self . PKC () , self . PQ () , self . NPUO () , self . COOLFLUXI () , self . USB_FS () , self . ETHERNET () , self . USB_HS () , self . USDHC () , self . MASTER_SEC_LEVEL_LOCK ())
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
    #[doc = "RAMF Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMF_MEM_RULE(pub u32);
    impl RAMF_MEM_RULE {
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
    impl Default for RAMF_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMF_MEM_RULE {
            RAMF_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMF_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMF_MEM_RULE")
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
    impl defmt::Format for RAMF_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMF_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMG Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMG_MEM_RULE(pub u32);
    impl RAMG_MEM_RULE {
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
    impl Default for RAMG_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMG_MEM_RULE {
            RAMG_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMG_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMG_MEM_RULE")
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
    impl defmt::Format for RAMG_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMG_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
        }
    }
    #[doc = "RAMH Memory Rule"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAMH_MEM_RULE(pub u32);
    impl RAMH_MEM_RULE {
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
    impl Default for RAMH_MEM_RULE {
        #[inline(always)]
        fn default() -> RAMH_MEM_RULE {
            RAMH_MEM_RULE(0)
        }
    }
    impl core::fmt::Debug for RAMH_MEM_RULE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAMH_MEM_RULE")
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
    impl defmt::Format for RAMH_MEM_RULE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAMH_MEM_RULE {{ RULE0: {=u8:?}, RULE1: {=u8:?}, RULE2: {=u8:?}, RULE3: {=u8:?}, RULE4: {=u8:?}, RULE5: {=u8:?}, RULE6: {=u8:?}, RULE7: {=u8:?} }}" , self . RULE0 () , self . RULE1 () , self . RULE2 () , self . RULE3 () , self . RULE4 () , self . RULE5 () , self . RULE6 () , self . RULE7 ())
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
    #[doc = "Secure Interrupt Mask 0 for CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CPU1_INT_MASK0(pub u32);
    impl SEC_CPU1_INT_MASK0 {
        #[must_use]
        #[inline(always)]
        pub const fn INT0_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT0_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT1_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT1_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT2_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT2_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT3_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT3_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT4_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT4_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT5_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT5_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT6_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT6_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT7_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT7_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT8_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT8_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT9_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT9_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT10_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT10_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT11_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT11_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT12_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT12_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT13_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT13_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT14_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT14_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT15_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT15_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT16_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT16_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT17_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT17_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT18_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT18_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT19_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT19_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT20_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT20_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT21_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT21_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT22_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT22_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT23_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT23_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT24_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT24_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT25_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT25_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT26_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT26_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT27_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT27_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT28_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT28_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT29_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT29_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT30_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT30_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT31_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT31_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_CPU1_INT_MASK0 {
        #[inline(always)]
        fn default() -> SEC_CPU1_INT_MASK0 {
            SEC_CPU1_INT_MASK0(0)
        }
    }
    impl core::fmt::Debug for SEC_CPU1_INT_MASK0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CPU1_INT_MASK0")
                .field("INT0_MASK", &self.INT0_MASK())
                .field("INT1_MASK", &self.INT1_MASK())
                .field("INT2_MASK", &self.INT2_MASK())
                .field("INT3_MASK", &self.INT3_MASK())
                .field("INT4_MASK", &self.INT4_MASK())
                .field("INT5_MASK", &self.INT5_MASK())
                .field("INT6_MASK", &self.INT6_MASK())
                .field("INT7_MASK", &self.INT7_MASK())
                .field("INT8_MASK", &self.INT8_MASK())
                .field("INT9_MASK", &self.INT9_MASK())
                .field("INT10_MASK", &self.INT10_MASK())
                .field("INT11_MASK", &self.INT11_MASK())
                .field("INT12_MASK", &self.INT12_MASK())
                .field("INT13_MASK", &self.INT13_MASK())
                .field("INT14_MASK", &self.INT14_MASK())
                .field("INT15_MASK", &self.INT15_MASK())
                .field("INT16_MASK", &self.INT16_MASK())
                .field("INT17_MASK", &self.INT17_MASK())
                .field("INT18_MASK", &self.INT18_MASK())
                .field("INT19_MASK", &self.INT19_MASK())
                .field("INT20_MASK", &self.INT20_MASK())
                .field("INT21_MASK", &self.INT21_MASK())
                .field("INT22_MASK", &self.INT22_MASK())
                .field("INT23_MASK", &self.INT23_MASK())
                .field("INT24_MASK", &self.INT24_MASK())
                .field("INT25_MASK", &self.INT25_MASK())
                .field("INT26_MASK", &self.INT26_MASK())
                .field("INT27_MASK", &self.INT27_MASK())
                .field("INT28_MASK", &self.INT28_MASK())
                .field("INT29_MASK", &self.INT29_MASK())
                .field("INT30_MASK", &self.INT30_MASK())
                .field("INT31_MASK", &self.INT31_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CPU1_INT_MASK0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_CPU1_INT_MASK0 {{ INT0_MASK: {=bool:?}, INT1_MASK: {=bool:?}, INT2_MASK: {=bool:?}, INT3_MASK: {=bool:?}, INT4_MASK: {=bool:?}, INT5_MASK: {=bool:?}, INT6_MASK: {=bool:?}, INT7_MASK: {=bool:?}, INT8_MASK: {=bool:?}, INT9_MASK: {=bool:?}, INT10_MASK: {=bool:?}, INT11_MASK: {=bool:?}, INT12_MASK: {=bool:?}, INT13_MASK: {=bool:?}, INT14_MASK: {=bool:?}, INT15_MASK: {=bool:?}, INT16_MASK: {=bool:?}, INT17_MASK: {=bool:?}, INT18_MASK: {=bool:?}, INT19_MASK: {=bool:?}, INT20_MASK: {=bool:?}, INT21_MASK: {=bool:?}, INT22_MASK: {=bool:?}, INT23_MASK: {=bool:?}, INT24_MASK: {=bool:?}, INT25_MASK: {=bool:?}, INT26_MASK: {=bool:?}, INT27_MASK: {=bool:?}, INT28_MASK: {=bool:?}, INT29_MASK: {=bool:?}, INT30_MASK: {=bool:?}, INT31_MASK: {=bool:?} }}" , self . INT0_MASK () , self . INT1_MASK () , self . INT2_MASK () , self . INT3_MASK () , self . INT4_MASK () , self . INT5_MASK () , self . INT6_MASK () , self . INT7_MASK () , self . INT8_MASK () , self . INT9_MASK () , self . INT10_MASK () , self . INT11_MASK () , self . INT12_MASK () , self . INT13_MASK () , self . INT14_MASK () , self . INT15_MASK () , self . INT16_MASK () , self . INT17_MASK () , self . INT18_MASK () , self . INT19_MASK () , self . INT20_MASK () , self . INT21_MASK () , self . INT22_MASK () , self . INT23_MASK () , self . INT24_MASK () , self . INT25_MASK () , self . INT26_MASK () , self . INT27_MASK () , self . INT28_MASK () , self . INT29_MASK () , self . INT30_MASK () , self . INT31_MASK ())
        }
    }
    #[doc = "Secure Interrupt Mask 1 for CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CPU1_INT_MASK1(pub u32);
    impl SEC_CPU1_INT_MASK1 {
        #[must_use]
        #[inline(always)]
        pub const fn INT32_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT32_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT33_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT33_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT34_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT34_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT35_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT35_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT36_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT36_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT37_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT37_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT38_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT38_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT39_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT39_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT40_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT40_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT41_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT41_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT42_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT42_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT43_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT43_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT44_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT44_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT45_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT45_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT46_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT46_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT47_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT47_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT48_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT48_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT49_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT49_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT50_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT50_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT51_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT51_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT52_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT52_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT53_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT53_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT54_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT54_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT55_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT55_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT56_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT56_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT57_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT57_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT58_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT58_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT59_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT59_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT60_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT60_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT61_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT61_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT62_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT62_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT63_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT63_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_CPU1_INT_MASK1 {
        #[inline(always)]
        fn default() -> SEC_CPU1_INT_MASK1 {
            SEC_CPU1_INT_MASK1(0)
        }
    }
    impl core::fmt::Debug for SEC_CPU1_INT_MASK1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CPU1_INT_MASK1")
                .field("INT32_MASK", &self.INT32_MASK())
                .field("INT33_MASK", &self.INT33_MASK())
                .field("INT34_MASK", &self.INT34_MASK())
                .field("INT35_MASK", &self.INT35_MASK())
                .field("INT36_MASK", &self.INT36_MASK())
                .field("INT37_MASK", &self.INT37_MASK())
                .field("INT38_MASK", &self.INT38_MASK())
                .field("INT39_MASK", &self.INT39_MASK())
                .field("INT40_MASK", &self.INT40_MASK())
                .field("INT41_MASK", &self.INT41_MASK())
                .field("INT42_MASK", &self.INT42_MASK())
                .field("INT43_MASK", &self.INT43_MASK())
                .field("INT44_MASK", &self.INT44_MASK())
                .field("INT45_MASK", &self.INT45_MASK())
                .field("INT46_MASK", &self.INT46_MASK())
                .field("INT47_MASK", &self.INT47_MASK())
                .field("INT48_MASK", &self.INT48_MASK())
                .field("INT49_MASK", &self.INT49_MASK())
                .field("INT50_MASK", &self.INT50_MASK())
                .field("INT51_MASK", &self.INT51_MASK())
                .field("INT52_MASK", &self.INT52_MASK())
                .field("INT53_MASK", &self.INT53_MASK())
                .field("INT54_MASK", &self.INT54_MASK())
                .field("INT55_MASK", &self.INT55_MASK())
                .field("INT56_MASK", &self.INT56_MASK())
                .field("INT57_MASK", &self.INT57_MASK())
                .field("INT58_MASK", &self.INT58_MASK())
                .field("INT59_MASK", &self.INT59_MASK())
                .field("INT60_MASK", &self.INT60_MASK())
                .field("INT61_MASK", &self.INT61_MASK())
                .field("INT62_MASK", &self.INT62_MASK())
                .field("INT63_MASK", &self.INT63_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CPU1_INT_MASK1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_CPU1_INT_MASK1 {{ INT32_MASK: {=bool:?}, INT33_MASK: {=bool:?}, INT34_MASK: {=bool:?}, INT35_MASK: {=bool:?}, INT36_MASK: {=bool:?}, INT37_MASK: {=bool:?}, INT38_MASK: {=bool:?}, INT39_MASK: {=bool:?}, INT40_MASK: {=bool:?}, INT41_MASK: {=bool:?}, INT42_MASK: {=bool:?}, INT43_MASK: {=bool:?}, INT44_MASK: {=bool:?}, INT45_MASK: {=bool:?}, INT46_MASK: {=bool:?}, INT47_MASK: {=bool:?}, INT48_MASK: {=bool:?}, INT49_MASK: {=bool:?}, INT50_MASK: {=bool:?}, INT51_MASK: {=bool:?}, INT52_MASK: {=bool:?}, INT53_MASK: {=bool:?}, INT54_MASK: {=bool:?}, INT55_MASK: {=bool:?}, INT56_MASK: {=bool:?}, INT57_MASK: {=bool:?}, INT58_MASK: {=bool:?}, INT59_MASK: {=bool:?}, INT60_MASK: {=bool:?}, INT61_MASK: {=bool:?}, INT62_MASK: {=bool:?}, INT63_MASK: {=bool:?} }}" , self . INT32_MASK () , self . INT33_MASK () , self . INT34_MASK () , self . INT35_MASK () , self . INT36_MASK () , self . INT37_MASK () , self . INT38_MASK () , self . INT39_MASK () , self . INT40_MASK () , self . INT41_MASK () , self . INT42_MASK () , self . INT43_MASK () , self . INT44_MASK () , self . INT45_MASK () , self . INT46_MASK () , self . INT47_MASK () , self . INT48_MASK () , self . INT49_MASK () , self . INT50_MASK () , self . INT51_MASK () , self . INT52_MASK () , self . INT53_MASK () , self . INT54_MASK () , self . INT55_MASK () , self . INT56_MASK () , self . INT57_MASK () , self . INT58_MASK () , self . INT59_MASK () , self . INT60_MASK () , self . INT61_MASK () , self . INT62_MASK () , self . INT63_MASK ())
        }
    }
    #[doc = "Secure Interrupt Mask 2 for CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CPU1_INT_MASK2(pub u32);
    impl SEC_CPU1_INT_MASK2 {
        #[must_use]
        #[inline(always)]
        pub const fn INT64_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT64_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT65_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT65_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT66_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT66_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT67_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT67_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT68_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT68_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT69_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT69_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT70_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT70_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT71_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT71_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT72_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT72_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT73_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT73_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT74_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT74_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT75_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT75_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT76_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT76_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT77_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT77_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT78_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT78_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT79_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT79_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT80_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT80_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT81_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT81_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT82_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT82_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT83_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT83_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT84_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT84_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT85_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT85_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT86_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT86_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT87_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT87_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT88_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT88_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT89_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT89_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT90_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT90_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT91_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT91_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT92_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT92_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT93_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT93_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT94_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT94_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT95_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT95_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_CPU1_INT_MASK2 {
        #[inline(always)]
        fn default() -> SEC_CPU1_INT_MASK2 {
            SEC_CPU1_INT_MASK2(0)
        }
    }
    impl core::fmt::Debug for SEC_CPU1_INT_MASK2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CPU1_INT_MASK2")
                .field("INT64_MASK", &self.INT64_MASK())
                .field("INT65_MASK", &self.INT65_MASK())
                .field("INT66_MASK", &self.INT66_MASK())
                .field("INT67_MASK", &self.INT67_MASK())
                .field("INT68_MASK", &self.INT68_MASK())
                .field("INT69_MASK", &self.INT69_MASK())
                .field("INT70_MASK", &self.INT70_MASK())
                .field("INT71_MASK", &self.INT71_MASK())
                .field("INT72_MASK", &self.INT72_MASK())
                .field("INT73_MASK", &self.INT73_MASK())
                .field("INT74_MASK", &self.INT74_MASK())
                .field("INT75_MASK", &self.INT75_MASK())
                .field("INT76_MASK", &self.INT76_MASK())
                .field("INT77_MASK", &self.INT77_MASK())
                .field("INT78_MASK", &self.INT78_MASK())
                .field("INT79_MASK", &self.INT79_MASK())
                .field("INT80_MASK", &self.INT80_MASK())
                .field("INT81_MASK", &self.INT81_MASK())
                .field("INT82_MASK", &self.INT82_MASK())
                .field("INT83_MASK", &self.INT83_MASK())
                .field("INT84_MASK", &self.INT84_MASK())
                .field("INT85_MASK", &self.INT85_MASK())
                .field("INT86_MASK", &self.INT86_MASK())
                .field("INT87_MASK", &self.INT87_MASK())
                .field("INT88_MASK", &self.INT88_MASK())
                .field("INT89_MASK", &self.INT89_MASK())
                .field("INT90_MASK", &self.INT90_MASK())
                .field("INT91_MASK", &self.INT91_MASK())
                .field("INT92_MASK", &self.INT92_MASK())
                .field("INT93_MASK", &self.INT93_MASK())
                .field("INT94_MASK", &self.INT94_MASK())
                .field("INT95_MASK", &self.INT95_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CPU1_INT_MASK2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_CPU1_INT_MASK2 {{ INT64_MASK: {=bool:?}, INT65_MASK: {=bool:?}, INT66_MASK: {=bool:?}, INT67_MASK: {=bool:?}, INT68_MASK: {=bool:?}, INT69_MASK: {=bool:?}, INT70_MASK: {=bool:?}, INT71_MASK: {=bool:?}, INT72_MASK: {=bool:?}, INT73_MASK: {=bool:?}, INT74_MASK: {=bool:?}, INT75_MASK: {=bool:?}, INT76_MASK: {=bool:?}, INT77_MASK: {=bool:?}, INT78_MASK: {=bool:?}, INT79_MASK: {=bool:?}, INT80_MASK: {=bool:?}, INT81_MASK: {=bool:?}, INT82_MASK: {=bool:?}, INT83_MASK: {=bool:?}, INT84_MASK: {=bool:?}, INT85_MASK: {=bool:?}, INT86_MASK: {=bool:?}, INT87_MASK: {=bool:?}, INT88_MASK: {=bool:?}, INT89_MASK: {=bool:?}, INT90_MASK: {=bool:?}, INT91_MASK: {=bool:?}, INT92_MASK: {=bool:?}, INT93_MASK: {=bool:?}, INT94_MASK: {=bool:?}, INT95_MASK: {=bool:?} }}" , self . INT64_MASK () , self . INT65_MASK () , self . INT66_MASK () , self . INT67_MASK () , self . INT68_MASK () , self . INT69_MASK () , self . INT70_MASK () , self . INT71_MASK () , self . INT72_MASK () , self . INT73_MASK () , self . INT74_MASK () , self . INT75_MASK () , self . INT76_MASK () , self . INT77_MASK () , self . INT78_MASK () , self . INT79_MASK () , self . INT80_MASK () , self . INT81_MASK () , self . INT82_MASK () , self . INT83_MASK () , self . INT84_MASK () , self . INT85_MASK () , self . INT86_MASK () , self . INT87_MASK () , self . INT88_MASK () , self . INT89_MASK () , self . INT90_MASK () , self . INT91_MASK () , self . INT92_MASK () , self . INT93_MASK () , self . INT94_MASK () , self . INT95_MASK ())
        }
    }
    #[doc = "Secure Interrupt Mask 3 for CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CPU1_INT_MASK3(pub u32);
    impl SEC_CPU1_INT_MASK3 {
        #[must_use]
        #[inline(always)]
        pub const fn INT96_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT96_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT97_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT97_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT98_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT98_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT99_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT99_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT100_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT100_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT101_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT101_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT102_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT102_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT103_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT103_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT104_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT104_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT105_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT105_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT106_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT106_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT107_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT107_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT108_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT108_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT109_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT109_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT110_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT110_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT111_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT111_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT112_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT112_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT113_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT113_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT114_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT114_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT115_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT115_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT116_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT116_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT117_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT117_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT118_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT118_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT119_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT119_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT120_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT120_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT121_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT121_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT122_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT122_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT123_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT123_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT124_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT124_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT125_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT125_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT126_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT126_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT127_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT127_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_CPU1_INT_MASK3 {
        #[inline(always)]
        fn default() -> SEC_CPU1_INT_MASK3 {
            SEC_CPU1_INT_MASK3(0)
        }
    }
    impl core::fmt::Debug for SEC_CPU1_INT_MASK3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CPU1_INT_MASK3")
                .field("INT96_MASK", &self.INT96_MASK())
                .field("INT97_MASK", &self.INT97_MASK())
                .field("INT98_MASK", &self.INT98_MASK())
                .field("INT99_MASK", &self.INT99_MASK())
                .field("INT100_MASK", &self.INT100_MASK())
                .field("INT101_MASK", &self.INT101_MASK())
                .field("INT102_MASK", &self.INT102_MASK())
                .field("INT103_MASK", &self.INT103_MASK())
                .field("INT104_MASK", &self.INT104_MASK())
                .field("INT105_MASK", &self.INT105_MASK())
                .field("INT106_MASK", &self.INT106_MASK())
                .field("INT107_MASK", &self.INT107_MASK())
                .field("INT108_MASK", &self.INT108_MASK())
                .field("INT109_MASK", &self.INT109_MASK())
                .field("INT110_MASK", &self.INT110_MASK())
                .field("INT111_MASK", &self.INT111_MASK())
                .field("INT112_MASK", &self.INT112_MASK())
                .field("INT113_MASK", &self.INT113_MASK())
                .field("INT114_MASK", &self.INT114_MASK())
                .field("INT115_MASK", &self.INT115_MASK())
                .field("INT116_MASK", &self.INT116_MASK())
                .field("INT117_MASK", &self.INT117_MASK())
                .field("INT118_MASK", &self.INT118_MASK())
                .field("INT119_MASK", &self.INT119_MASK())
                .field("INT120_MASK", &self.INT120_MASK())
                .field("INT121_MASK", &self.INT121_MASK())
                .field("INT122_MASK", &self.INT122_MASK())
                .field("INT123_MASK", &self.INT123_MASK())
                .field("INT124_MASK", &self.INT124_MASK())
                .field("INT125_MASK", &self.INT125_MASK())
                .field("INT126_MASK", &self.INT126_MASK())
                .field("INT127_MASK", &self.INT127_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CPU1_INT_MASK3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_CPU1_INT_MASK3 {{ INT96_MASK: {=bool:?}, INT97_MASK: {=bool:?}, INT98_MASK: {=bool:?}, INT99_MASK: {=bool:?}, INT100_MASK: {=bool:?}, INT101_MASK: {=bool:?}, INT102_MASK: {=bool:?}, INT103_MASK: {=bool:?}, INT104_MASK: {=bool:?}, INT105_MASK: {=bool:?}, INT106_MASK: {=bool:?}, INT107_MASK: {=bool:?}, INT108_MASK: {=bool:?}, INT109_MASK: {=bool:?}, INT110_MASK: {=bool:?}, INT111_MASK: {=bool:?}, INT112_MASK: {=bool:?}, INT113_MASK: {=bool:?}, INT114_MASK: {=bool:?}, INT115_MASK: {=bool:?}, INT116_MASK: {=bool:?}, INT117_MASK: {=bool:?}, INT118_MASK: {=bool:?}, INT119_MASK: {=bool:?}, INT120_MASK: {=bool:?}, INT121_MASK: {=bool:?}, INT122_MASK: {=bool:?}, INT123_MASK: {=bool:?}, INT124_MASK: {=bool:?}, INT125_MASK: {=bool:?}, INT126_MASK: {=bool:?}, INT127_MASK: {=bool:?} }}" , self . INT96_MASK () , self . INT97_MASK () , self . INT98_MASK () , self . INT99_MASK () , self . INT100_MASK () , self . INT101_MASK () , self . INT102_MASK () , self . INT103_MASK () , self . INT104_MASK () , self . INT105_MASK () , self . INT106_MASK () , self . INT107_MASK () , self . INT108_MASK () , self . INT109_MASK () , self . INT110_MASK () , self . INT111_MASK () , self . INT112_MASK () , self . INT113_MASK () , self . INT114_MASK () , self . INT115_MASK () , self . INT116_MASK () , self . INT117_MASK () , self . INT118_MASK () , self . INT119_MASK () , self . INT120_MASK () , self . INT121_MASK () , self . INT122_MASK () , self . INT123_MASK () , self . INT124_MASK () , self . INT125_MASK () , self . INT126_MASK () , self . INT127_MASK ())
        }
    }
    #[doc = "Secure Interrupt Mask 4 for CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CPU1_INT_MASK4(pub u32);
    impl SEC_CPU1_INT_MASK4 {
        #[must_use]
        #[inline(always)]
        pub const fn INT128_MASK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT128_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT129_MASK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT129_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT130_MASK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT130_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT131_MASK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT131_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT132_MASK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT132_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT133_MASK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT133_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT134_MASK(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT134_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT135_MASK(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT135_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT136_MASK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT136_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT137_MASK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT137_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT138_MASK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT138_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT139_MASK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT139_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT140_MASK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT140_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT141_MASK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT141_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT142_MASK(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT142_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT143_MASK(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT143_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT144_MASK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT144_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT145_MASK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT145_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT146_MASK(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT146_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT147_MASK(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT147_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT148_MASK(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT148_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT149_MASK(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT149_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT150_MASK(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT150_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT151_MASK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT151_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT152_MASK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT152_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT153_MASK(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT153_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT154_MASK(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT154_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT155_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT155_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT156_MASK(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT156_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT157_MASK(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT157_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT158_MASK(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT158_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT159_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT159_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SEC_CPU1_INT_MASK4 {
        #[inline(always)]
        fn default() -> SEC_CPU1_INT_MASK4 {
            SEC_CPU1_INT_MASK4(0)
        }
    }
    impl core::fmt::Debug for SEC_CPU1_INT_MASK4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CPU1_INT_MASK4")
                .field("INT128_MASK", &self.INT128_MASK())
                .field("INT129_MASK", &self.INT129_MASK())
                .field("INT130_MASK", &self.INT130_MASK())
                .field("INT131_MASK", &self.INT131_MASK())
                .field("INT132_MASK", &self.INT132_MASK())
                .field("INT133_MASK", &self.INT133_MASK())
                .field("INT134_MASK", &self.INT134_MASK())
                .field("INT135_MASK", &self.INT135_MASK())
                .field("INT136_MASK", &self.INT136_MASK())
                .field("INT137_MASK", &self.INT137_MASK())
                .field("INT138_MASK", &self.INT138_MASK())
                .field("INT139_MASK", &self.INT139_MASK())
                .field("INT140_MASK", &self.INT140_MASK())
                .field("INT141_MASK", &self.INT141_MASK())
                .field("INT142_MASK", &self.INT142_MASK())
                .field("INT143_MASK", &self.INT143_MASK())
                .field("INT144_MASK", &self.INT144_MASK())
                .field("INT145_MASK", &self.INT145_MASK())
                .field("INT146_MASK", &self.INT146_MASK())
                .field("INT147_MASK", &self.INT147_MASK())
                .field("INT148_MASK", &self.INT148_MASK())
                .field("INT149_MASK", &self.INT149_MASK())
                .field("INT150_MASK", &self.INT150_MASK())
                .field("INT151_MASK", &self.INT151_MASK())
                .field("INT152_MASK", &self.INT152_MASK())
                .field("INT153_MASK", &self.INT153_MASK())
                .field("INT154_MASK", &self.INT154_MASK())
                .field("INT155_MASK", &self.INT155_MASK())
                .field("INT156_MASK", &self.INT156_MASK())
                .field("INT157_MASK", &self.INT157_MASK())
                .field("INT158_MASK", &self.INT158_MASK())
                .field("INT159_MASK", &self.INT159_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CPU1_INT_MASK4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_CPU1_INT_MASK4 {{ INT128_MASK: {=bool:?}, INT129_MASK: {=bool:?}, INT130_MASK: {=bool:?}, INT131_MASK: {=bool:?}, INT132_MASK: {=bool:?}, INT133_MASK: {=bool:?}, INT134_MASK: {=bool:?}, INT135_MASK: {=bool:?}, INT136_MASK: {=bool:?}, INT137_MASK: {=bool:?}, INT138_MASK: {=bool:?}, INT139_MASK: {=bool:?}, INT140_MASK: {=bool:?}, INT141_MASK: {=bool:?}, INT142_MASK: {=bool:?}, INT143_MASK: {=bool:?}, INT144_MASK: {=bool:?}, INT145_MASK: {=bool:?}, INT146_MASK: {=bool:?}, INT147_MASK: {=bool:?}, INT148_MASK: {=bool:?}, INT149_MASK: {=bool:?}, INT150_MASK: {=bool:?}, INT151_MASK: {=bool:?}, INT152_MASK: {=bool:?}, INT153_MASK: {=bool:?}, INT154_MASK: {=bool:?}, INT155_MASK: {=bool:?}, INT156_MASK: {=bool:?}, INT157_MASK: {=bool:?}, INT158_MASK: {=bool:?}, INT159_MASK: {=bool:?} }}" , self . INT128_MASK () , self . INT129_MASK () , self . INT130_MASK () , self . INT131_MASK () , self . INT132_MASK () , self . INT133_MASK () , self . INT134_MASK () , self . INT135_MASK () , self . INT136_MASK () , self . INT137_MASK () , self . INT138_MASK () , self . INT139_MASK () , self . INT140_MASK () , self . INT141_MASK () , self . INT142_MASK () , self . INT143_MASK () , self . INT144_MASK () , self . INT145_MASK () , self . INT146_MASK () , self . INT147_MASK () , self . INT148_MASK () , self . INT149_MASK () , self . INT150_MASK () , self . INT151_MASK () , self . INT152_MASK () , self . INT153_MASK () , self . INT154_MASK () , self . INT155_MASK () , self . INT156_MASK () , self . INT157_MASK () , self . INT158_MASK () , self . INT159_MASK ())
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
    #[doc = "Secure Mask Lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_GP_REG_LOCK(pub u32);
    impl SEC_GP_REG_LOCK {
        #[must_use]
        #[inline(always)]
        pub const fn SEC_GPIO_MASK0_LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_GPIO_MASK0_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_GPIO_MASK1_LOCK(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_GPIO_MASK1_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_CPU1_INT_MASK0_LOCK(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_CPU1_INT_MASK0_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_CPU1_INT_MASK1_LOCK(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_CPU1_INT_MASK1_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_CPU1_INT_MASK2_LOCK(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_CPU1_INT_MASK2_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_CPU1_INT_MASK3_LOCK(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_CPU1_INT_MASK3_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEC_CPU1_INT_MASK4_LOCK(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEC_CPU1_INT_MASK4_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for SEC_GP_REG_LOCK {
        #[inline(always)]
        fn default() -> SEC_GP_REG_LOCK {
            SEC_GP_REG_LOCK(0)
        }
    }
    impl core::fmt::Debug for SEC_GP_REG_LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_GP_REG_LOCK")
                .field("SEC_GPIO_MASK0_LOCK", &self.SEC_GPIO_MASK0_LOCK())
                .field("SEC_GPIO_MASK1_LOCK", &self.SEC_GPIO_MASK1_LOCK())
                .field("SEC_CPU1_INT_MASK0_LOCK", &self.SEC_CPU1_INT_MASK0_LOCK())
                .field("SEC_CPU1_INT_MASK1_LOCK", &self.SEC_CPU1_INT_MASK1_LOCK())
                .field("SEC_CPU1_INT_MASK2_LOCK", &self.SEC_CPU1_INT_MASK2_LOCK())
                .field("SEC_CPU1_INT_MASK3_LOCK", &self.SEC_CPU1_INT_MASK3_LOCK())
                .field("SEC_CPU1_INT_MASK4_LOCK", &self.SEC_CPU1_INT_MASK4_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_GP_REG_LOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SEC_GP_REG_LOCK {{ SEC_GPIO_MASK0_LOCK: {=u8:?}, SEC_GPIO_MASK1_LOCK: {=u8:?}, SEC_CPU1_INT_MASK0_LOCK: {=u8:?}, SEC_CPU1_INT_MASK1_LOCK: {=u8:?}, SEC_CPU1_INT_MASK2_LOCK: {=u8:?}, SEC_CPU1_INT_MASK3_LOCK: {=u8:?}, SEC_CPU1_INT_MASK4_LOCK: {=u8:?} }}" , self . SEC_GPIO_MASK0_LOCK () , self . SEC_GPIO_MASK1_LOCK () , self . SEC_CPU1_INT_MASK0_LOCK () , self . SEC_CPU1_INT_MASK1_LOCK () , self . SEC_CPU1_INT_MASK2_LOCK () , self . SEC_CPU1_INT_MASK3_LOCK () , self . SEC_CPU1_INT_MASK4_LOCK ())
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
