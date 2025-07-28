#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMUTEST {
    ptr: *mut u8,
}
unsafe impl Send for FMUTEST {}
unsafe impl Sync for FMUTEST {}
impl FMUTEST {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn FSTAT(self) -> crate::common::Reg<regs::FSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn FCNFG(self) -> crate::common::Reg<regs::FCNFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn FCTRL(self) -> crate::common::Reg<regs::FCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn FTEST(self) -> crate::common::Reg<regs::FTEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB0(self) -> crate::common::Reg<regs::FCCOB0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB1(self) -> crate::common::Reg<regs::FCCOB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCOB7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn RESET_STATUS(self) -> crate::common::Reg<regs::RESET_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn MCTL(self) -> crate::common::Reg<regs::MCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn BSEL_GEN(self) -> crate::common::Reg<regs::BSEL_GEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn PWR_OPT(self) -> crate::common::Reg<regs::PWR_OPT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMD_CHECK(self) -> crate::common::Reg<regs::CMD_CHECK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn BSEL(self) -> crate::common::Reg<regs::BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn MSIZE(self) -> crate::common::Reg<regs::MSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASH_RD_ADD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASH_STOP_ADD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASH_RD_CTRL(self) -> crate::common::Reg<regs::FLASH_RD_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn MM_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn MM_WDATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn MM_CTL(self) -> crate::common::Reg<regs::MM_CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn UINT_CTL(self) -> crate::common::Reg<regs::UINT_CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn RD_DATA0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[inline(always)]
    pub const fn RD_DATA1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn RD_DATA2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn RD_DATA3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn PARITY(self) -> crate::common::Reg<regs::PARITY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[inline(always)]
    pub const fn RD_PATH_CTRL_STATUS(
        self,
    ) -> crate::common::Reg<regs::RD_PATH_CTRL_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_DIN0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_DIN1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_DIN2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_DIN3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_ADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_CMD_WAIT(self) -> crate::common::Reg<regs::SMW_CMD_WAIT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_STATUS(self) -> crate::common::Reg<regs::SMW_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM0_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM0_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM0_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM0_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM1_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM1_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM1_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM1_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM2_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM2_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM2_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM2_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM3_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM3_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM3_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM3_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM4_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM4_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM4_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM4_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM5_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM5_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM5_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM5_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM6_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM6_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM6_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM6_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM7_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM7_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM7_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOCTRIM7_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[inline(always)]
    pub const fn R_IP_CONFIG(self) -> crate::common::Reg<regs::R_IP_CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn R_TESTCODE(self) -> crate::common::Reg<regs::R_TESTCODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DFT_CTRL(self) -> crate::common::Reg<regs::R_DFT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_ADR_CTRL(self) -> crate::common::Reg<regs::R_ADR_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[inline(always)]
    pub const fn R_PIN_CTRL(self) -> crate::common::Reg<regs::R_PIN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn R_CNT_LOOP_CTRL(
        self,
    ) -> crate::common::Reg<regs::R_CNT_LOOP_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_TIMER_CTRL(self) -> crate::common::Reg<regs::R_TIMER_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn R_TEST_CTRL(self) -> crate::common::Reg<regs::R_TEST_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[inline(always)]
    pub const fn R_ABORT_LOOP(self) -> crate::common::Reg<regs::R_ABORT_LOOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[inline(always)]
    pub const fn R_ADR_QUERY(self) -> crate::common::Reg<regs::R_ADR_QUERY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_DOUT_QUERY0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMW_QUERY(self) -> crate::common::Reg<regs::R_SMW_QUERY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMW_SETTING0(
        self,
    ) -> crate::common::Reg<regs::R_SMW_SETTING0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMW_SETTING1(
        self,
    ) -> crate::common::Reg<regs::R_SMW_SETTING1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMP_WHV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMP_WHV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_SME_WHV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SME_WHV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMW_SETTING2(
        self,
    ) -> crate::common::Reg<regs::R_SMW_SETTING2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[inline(always)]
    pub const fn R_D_MISR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_A_MISR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn R_C_MISR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[inline(always)]
    pub const fn R_SMW_SETTING3(
        self,
    ) -> crate::common::Reg<regs::R_SMW_SETTING3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[inline(always)]
    pub const fn R_REPAIR0_0(self) -> crate::common::Reg<regs::R_REPAIR0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn R_REPAIR0_1(self) -> crate::common::Reg<regs::R_REPAIR0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[inline(always)]
    pub const fn R_REPAIR1_0(self) -> crate::common::Reg<regs::R_REPAIR1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[inline(always)]
    pub const fn R_REPAIR1_1(self) -> crate::common::Reg<regs::R_REPAIR1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL0_EX(
        self,
    ) -> crate::common::Reg<regs::R_DATA_CTRL0_EX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[inline(always)]
    pub const fn R_TIMER_CTRL_EX(
        self,
    ) -> crate::common::Reg<regs::R_TIMER_CTRL_EX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DOUT_QUERY1(self) -> crate::common::Reg<regs::R_DOUT_QUERY1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[inline(always)]
    pub const fn R_D_MISR1(self) -> crate::common::Reg<regs::R_D_MISR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_A_MISR1(self) -> crate::common::Reg<regs::R_A_MISR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[inline(always)]
    pub const fn R_C_MISR1(self) -> crate::common::Reg<regs::R_C_MISR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL1_EX(
        self,
    ) -> crate::common::Reg<regs::R_DATA_CTRL1_EX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL2_EX(
        self,
    ) -> crate::common::Reg<regs::R_DATA_CTRL2_EX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[inline(always)]
    pub const fn R_DATA_CTRL3_EX(
        self,
    ) -> crate::common::Reg<regs::R_DATA_CTRL3_EX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_TIMER_OPTION(
        self,
    ) -> crate::common::Reg<regs::SMW_TIMER_OPTION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SETTING_OPTION0(
        self,
    ) -> crate::common::Reg<regs::SMW_SETTING_OPTION0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SETTING_OPTION2(
        self,
    ) -> crate::common::Reg<regs::SMW_SETTING_OPTION2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SETTING_OPTION3(
        self,
    ) -> crate::common::Reg<regs::SMW_SETTING_OPTION3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SMP_WHV_OPTION0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SME_WHV_OPTION0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SETTING_OPTION1(
        self,
    ) -> crate::common::Reg<regs::SMW_SETTING_OPTION1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SMP_WHV_OPTION1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_SME_WHV_OPTION1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn REPAIR0_0(self) -> crate::common::Reg<regs::REPAIR0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn REPAIR0_1(self) -> crate::common::Reg<regs::REPAIR0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[inline(always)]
    pub const fn REPAIR1_0(self) -> crate::common::Reg<regs::REPAIR1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[inline(always)]
    pub const fn REPAIR1_1(self) -> crate::common::Reg<regs::REPAIR1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[inline(always)]
    pub const fn SMW_HB_SIGNALS(
        self,
    ) -> crate::common::Reg<regs::SMW_HB_SIGNALS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn BIST_DUMP_CTRL(
        self,
    ) -> crate::common::Reg<regs::BIST_DUMP_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[inline(always)]
    pub const fn ATX_PIN_CTRL(self) -> crate::common::Reg<regs::ATX_PIN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[inline(always)]
    pub const fn FAILCNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[inline(always)]
    pub const fn PGM_PULSE_CNT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[inline(always)]
    pub const fn PGM_PULSE_CNT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[inline(always)]
    pub const fn ERS_PULSE_CNT(self) -> crate::common::Reg<regs::ERS_PULSE_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[inline(always)]
    pub const fn MAX_PULSE_CNT(self) -> crate::common::Reg<regs::MAX_PULSE_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[inline(always)]
    pub const fn PORT_CTRL(self) -> crate::common::Reg<regs::PORT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
}
pub mod regs {
    #[doc = "ATX Pin Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ATX_PIN_CTRL(pub u32);
    impl ATX_PIN_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn TM_TO_ATX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TM_TO_ATX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ATX_PIN_CTRL {
        #[inline(always)]
        fn default() -> ATX_PIN_CTRL {
            ATX_PIN_CTRL(0)
        }
    }
    impl core::fmt::Debug for ATX_PIN_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ATX_PIN_CTRL")
                .field("TM_TO_ATX", &self.TM_TO_ATX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ATX_PIN_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ATX_PIN_CTRL {{ TM_TO_ATX: {=u8:?} }}", self.TM_TO_ATX())
        }
    }
    #[doc = "BIST Datadump Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BIST_DUMP_CTRL(pub u32);
    impl BIST_DUMP_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn BIST_DONE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_FAIL(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATADUMP(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DATADUMP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATADUMP_TRIG(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DATADUMP_TRIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATADUMP_PATT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATADUMP_PATT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATADUMP_MRGEN(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DATADUMP_MRGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATADUMP_MRGTYPE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DATADUMP_MRGTYPE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for BIST_DUMP_CTRL {
        #[inline(always)]
        fn default() -> BIST_DUMP_CTRL {
            BIST_DUMP_CTRL(0)
        }
    }
    impl core::fmt::Debug for BIST_DUMP_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BIST_DUMP_CTRL")
                .field("BIST_DONE", &self.BIST_DONE())
                .field("BIST_FAIL", &self.BIST_FAIL())
                .field("DATADUMP", &self.DATADUMP())
                .field("DATADUMP_TRIG", &self.DATADUMP_TRIG())
                .field("DATADUMP_PATT", &self.DATADUMP_PATT())
                .field("DATADUMP_MRGEN", &self.DATADUMP_MRGEN())
                .field("DATADUMP_MRGTYPE", &self.DATADUMP_MRGTYPE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BIST_DUMP_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BIST_DUMP_CTRL {{ BIST_DONE: {=bool:?}, BIST_FAIL: {=bool:?}, DATADUMP: {=bool:?}, DATADUMP_TRIG: {=bool:?}, DATADUMP_PATT: {=u8:?}, DATADUMP_MRGEN: {=bool:?}, DATADUMP_MRGTYPE: {=bool:?} }}" , self . BIST_DONE () , self . BIST_FAIL () , self . DATADUMP () , self . DATADUMP_TRIG () , self . DATADUMP_PATT () , self . DATADUMP_MRGEN () , self . DATADUMP_MRGTYPE ())
        }
    }
    #[doc = "FMU Block Select Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BSEL(pub u32);
    impl BSEL {
        #[must_use]
        #[inline(always)]
        pub const fn SBSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SBSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBSEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for BSEL {
        #[inline(always)]
        fn default() -> BSEL {
            BSEL(0)
        }
    }
    impl core::fmt::Debug for BSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BSEL")
                .field("SBSEL", &self.SBSEL())
                .field("MBSEL", &self.MBSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BSEL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BSEL {{ SBSEL: {=u8:?}, MBSEL: {=u8:?} }}",
                self.SBSEL(),
                self.MBSEL()
            )
        }
    }
    #[doc = "FMU Block Select Generation Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BSEL_GEN(pub u32);
    impl BSEL_GEN {
        #[must_use]
        #[inline(always)]
        pub const fn SBSEL_GEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SBSEL_GEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBSEL_GEN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBSEL_GEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for BSEL_GEN {
        #[inline(always)]
        fn default() -> BSEL_GEN {
            BSEL_GEN(0)
        }
    }
    impl core::fmt::Debug for BSEL_GEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BSEL_GEN")
                .field("SBSEL_GEN", &self.SBSEL_GEN())
                .field("MBSEL_GEN", &self.MBSEL_GEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BSEL_GEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BSEL_GEN {{ SBSEL_GEN: {=u8:?}, MBSEL_GEN: {=u8:?} }}",
                self.SBSEL_GEN(),
                self.MBSEL_GEN()
            )
        }
    }
    #[doc = "FMU Command Check Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_CHECK(pub u32);
    impl CMD_CHECK {
        #[must_use]
        #[inline(always)]
        pub const fn ALIGNFAIL_PHR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALIGNFAIL_PHR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALIGNFAIL_PG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALIGNFAIL_PG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALIGNFAIL_SCR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALIGNFAIL_SCR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALIGNFAIL_BLK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALIGNFAIL_BLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR_FAIL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADDR_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IFR_CMD(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IFR_CMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ALL_CMD(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ALL_CMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RANGE_FAIL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RANGE_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCR_ALIGN_CHK(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCR_ALIGN_CHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OPTION_FAIL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OPTION_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ILLEGAL_CMD(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ILLEGAL_CMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for CMD_CHECK {
        #[inline(always)]
        fn default() -> CMD_CHECK {
            CMD_CHECK(0)
        }
    }
    impl core::fmt::Debug for CMD_CHECK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMD_CHECK")
                .field("ALIGNFAIL_PHR", &self.ALIGNFAIL_PHR())
                .field("ALIGNFAIL_PG", &self.ALIGNFAIL_PG())
                .field("ALIGNFAIL_SCR", &self.ALIGNFAIL_SCR())
                .field("ALIGNFAIL_BLK", &self.ALIGNFAIL_BLK())
                .field("ADDR_FAIL", &self.ADDR_FAIL())
                .field("IFR_CMD", &self.IFR_CMD())
                .field("ALL_CMD", &self.ALL_CMD())
                .field("RANGE_FAIL", &self.RANGE_FAIL())
                .field("SCR_ALIGN_CHK", &self.SCR_ALIGN_CHK())
                .field("OPTION_FAIL", &self.OPTION_FAIL())
                .field("ILLEGAL_CMD", &self.ILLEGAL_CMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMD_CHECK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CMD_CHECK {{ ALIGNFAIL_PHR: {=bool:?}, ALIGNFAIL_PG: {=bool:?}, ALIGNFAIL_SCR: {=bool:?}, ALIGNFAIL_BLK: {=bool:?}, ADDR_FAIL: {=bool:?}, IFR_CMD: {=bool:?}, ALL_CMD: {=bool:?}, RANGE_FAIL: {=bool:?}, SCR_ALIGN_CHK: {=bool:?}, OPTION_FAIL: {=bool:?}, ILLEGAL_CMD: {=bool:?} }}" , self . ALIGNFAIL_PHR () , self . ALIGNFAIL_PG () , self . ALIGNFAIL_SCR () , self . ALIGNFAIL_BLK () , self . ADDR_FAIL () , self . IFR_CMD () , self . ALL_CMD () , self . RANGE_FAIL () , self . SCR_ALIGN_CHK () , self . OPTION_FAIL () , self . ILLEGAL_CMD ())
        }
    }
    #[doc = "Erase Pulse Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERS_PULSE_CNT(pub u32);
    impl ERS_PULSE_CNT {
        #[must_use]
        #[inline(always)]
        pub const fn ERS_CNT0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ERS_CNT0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERS_CNT1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ERS_CNT1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ERS_PULSE_CNT {
        #[inline(always)]
        fn default() -> ERS_PULSE_CNT {
            ERS_PULSE_CNT(0)
        }
    }
    impl core::fmt::Debug for ERS_PULSE_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERS_PULSE_CNT")
                .field("ERS_CNT0", &self.ERS_CNT0())
                .field("ERS_CNT1", &self.ERS_CNT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ERS_PULSE_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ERS_PULSE_CNT {{ ERS_CNT0: {=u16:?}, ERS_CNT1: {=u16:?} }}",
                self.ERS_CNT0(),
                self.ERS_CNT1()
            )
        }
    }
    #[doc = "Flash Command Control 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCCOB0(pub u32);
    impl FCCOB0 {
        #[must_use]
        #[inline(always)]
        pub const fn CMDCODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDCODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for FCCOB0 {
        #[inline(always)]
        fn default() -> FCCOB0 {
            FCCOB0(0)
        }
    }
    impl core::fmt::Debug for FCCOB0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCCOB0")
                .field("CMDCODE", &self.CMDCODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCCOB0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FCCOB0 {{ CMDCODE: {=u8:?} }}", self.CMDCODE())
        }
    }
    #[doc = "Flash Command Control 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCCOB1(pub u32);
    impl FCCOB1 {
        #[must_use]
        #[inline(always)]
        pub const fn CMDOPT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDOPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for FCCOB1 {
        #[inline(always)]
        fn default() -> FCCOB1 {
            FCCOB1(0)
        }
    }
    impl core::fmt::Debug for FCCOB1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCCOB1")
                .field("CMDOPT", &self.CMDOPT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCCOB1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FCCOB1 {{ CMDOPT: {=u8:?} }}", self.CMDOPT())
        }
    }
    #[doc = "Flash Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCNFG(pub u32);
    impl FCNFG {
        #[must_use]
        #[inline(always)]
        pub const fn CCIE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSREQ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERSREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFDIE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSIEN0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERSIEN0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSIEN1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERSIEN1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FCNFG {
        #[inline(always)]
        fn default() -> FCNFG {
            FCNFG(0)
        }
    }
    impl core::fmt::Debug for FCNFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCNFG")
                .field("CCIE", &self.CCIE())
                .field("ERSREQ", &self.ERSREQ())
                .field("DFDIE", &self.DFDIE())
                .field("ERSIEN0", &self.ERSIEN0())
                .field("ERSIEN1", &self.ERSIEN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCNFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FCNFG {{ CCIE: {=bool:?}, ERSREQ: {=bool:?}, DFDIE: {=bool:?}, ERSIEN0: {=u8:?}, ERSIEN1: {=u8:?} }}" , self . CCIE () , self . ERSREQ () , self . DFDIE () , self . ERSIEN0 () , self . ERSIEN1 ())
        }
    }
    #[doc = "Flash Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u32);
    impl FCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RWSC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RWSC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LSACTIVE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LSACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FDFD(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FDFD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ABTREQ(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ABTREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for FCTRL {
        #[inline(always)]
        fn default() -> FCTRL {
            FCTRL(0)
        }
    }
    impl core::fmt::Debug for FCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTRL")
                .field("RWSC", &self.RWSC())
                .field("LSACTIVE", &self.LSACTIVE())
                .field("FDFD", &self.FDFD())
                .field("ABTREQ", &self.ABTREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FCTRL {{ RWSC: {=u8:?}, LSACTIVE: {=bool:?}, FDFD: {=bool:?}, ABTREQ: {=bool:?} }}" , self . RWSC () , self . LSACTIVE () , self . FDFD () , self . ABTREQ ())
        }
    }
    #[doc = "Flash Read Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLASH_RD_CTRL(pub u32);
    impl FLASH_RD_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FLASH_RD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLASH_RD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WIDE_LOAD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WIDE_LOAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SINGLE_RD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SINGLE_RD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for FLASH_RD_CTRL {
        #[inline(always)]
        fn default() -> FLASH_RD_CTRL {
            FLASH_RD_CTRL(0)
        }
    }
    impl core::fmt::Debug for FLASH_RD_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASH_RD_CTRL")
                .field("FLASH_RD", &self.FLASH_RD())
                .field("WIDE_LOAD", &self.WIDE_LOAD())
                .field("SINGLE_RD", &self.SINGLE_RD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASH_RD_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLASH_RD_CTRL {{ FLASH_RD: {=bool:?}, WIDE_LOAD: {=bool:?}, SINGLE_RD: {=bool:?} }}" , self . FLASH_RD () , self . WIDE_LOAD () , self . SINGLE_RD ())
        }
    }
    #[doc = "Flash Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FSTAT(pub u32);
    impl FSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn FAIL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDABT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMDABT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PVIOL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PVIOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACCERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWSABT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWSABT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CCIF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CCIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDPRT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDPRT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDP(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMDP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDDID(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDDID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFDIF(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFDIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SALV_USED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SALV_USED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PEWEN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PEWEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PERDY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PERDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FSTAT {
        #[inline(always)]
        fn default() -> FSTAT {
            FSTAT(0)
        }
    }
    impl core::fmt::Debug for FSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FSTAT")
                .field("FAIL", &self.FAIL())
                .field("CMDABT", &self.CMDABT())
                .field("PVIOL", &self.PVIOL())
                .field("ACCERR", &self.ACCERR())
                .field("CWSABT", &self.CWSABT())
                .field("CCIF", &self.CCIF())
                .field("CMDPRT", &self.CMDPRT())
                .field("CMDP", &self.CMDP())
                .field("CMDDID", &self.CMDDID())
                .field("DFDIF", &self.DFDIF())
                .field("SALV_USED", &self.SALV_USED())
                .field("PEWEN", &self.PEWEN())
                .field("PERDY", &self.PERDY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FSTAT {{ FAIL: {=bool:?}, CMDABT: {=bool:?}, PVIOL: {=bool:?}, ACCERR: {=bool:?}, CWSABT: {=bool:?}, CCIF: {=bool:?}, CMDPRT: {=u8:?}, CMDP: {=bool:?}, CMDDID: {=u8:?}, DFDIF: {=bool:?}, SALV_USED: {=bool:?}, PEWEN: {=u8:?}, PERDY: {=bool:?} }}" , self . FAIL () , self . CMDABT () , self . PVIOL () , self . ACCERR () , self . CWSABT () , self . CCIF () , self . CMDPRT () , self . CMDP () , self . CMDDID () , self . DFDIF () , self . SALV_USED () , self . PEWEN () , self . PERDY ())
        }
    }
    #[doc = "Flash Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FTEST(pub u32);
    impl FTEST {
        #[must_use]
        #[inline(always)]
        pub const fn TMECTL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TMECTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TMEWR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TMEWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TME(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TMODE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TMODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TMELOCK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TMELOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for FTEST {
        #[inline(always)]
        fn default() -> FTEST {
            FTEST(0)
        }
    }
    impl core::fmt::Debug for FTEST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FTEST")
                .field("TMECTL", &self.TMECTL())
                .field("TMEWR", &self.TMEWR())
                .field("TME", &self.TME())
                .field("TMODE", &self.TMODE())
                .field("TMELOCK", &self.TMELOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FTEST {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FTEST {{ TMECTL: {=bool:?}, TMEWR: {=bool:?}, TME: {=bool:?}, TMODE: {=bool:?}, TMELOCK: {=bool:?} }}" , self . TMECTL () , self . TMEWR () , self . TME () , self . TMODE () , self . TMELOCK ())
        }
    }
    #[doc = "Maximum Pulse Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAX_PULSE_CNT(pub u32);
    impl MAX_PULSE_CNT {
        #[must_use]
        #[inline(always)]
        pub const fn LAST_PCNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LAST_PCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAX_ERS_CNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MAX_ERS_CNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAX_PGM_CNT(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAX_PGM_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for MAX_PULSE_CNT {
        #[inline(always)]
        fn default() -> MAX_PULSE_CNT {
            MAX_PULSE_CNT(0)
        }
    }
    impl core::fmt::Debug for MAX_PULSE_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAX_PULSE_CNT")
                .field("LAST_PCNT", &self.LAST_PCNT())
                .field("MAX_ERS_CNT", &self.MAX_ERS_CNT())
                .field("MAX_PGM_CNT", &self.MAX_PGM_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAX_PULSE_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAX_PULSE_CNT {{ LAST_PCNT: {=u16:?}, MAX_ERS_CNT: {=u16:?}, MAX_PGM_CNT: {=u8:?} }}" , self . LAST_PCNT () , self . MAX_ERS_CNT () , self . MAX_PGM_CNT ())
        }
    }
    #[doc = "FMU Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTL(pub u32);
    impl MCTL {
        #[must_use]
        #[inline(always)]
        pub const fn COREHLD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COREHLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LSACT_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LSACT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LSACTWREN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LSACTWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASTER_REPAIR_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MASTER_REPAIR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFCMDEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFCMDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CWSABTEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CWSABTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MRGRDDIS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MRGRDDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MRGRD0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MRGRD0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MRGRD1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MRGRD1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSAACK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERSAACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCAN_OBS(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCAN_OBS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_CTL(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_CTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMWR_CTL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMWR_CTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SALV_DIS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SALV_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOC_ECC_CTL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOC_ECC_CTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMU_ECC_CTL(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FMU_ECC_CTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_PWR_DIS(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_PWR_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSC_H(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OSC_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MCTL {
        #[inline(always)]
        fn default() -> MCTL {
            MCTL(0)
        }
    }
    impl core::fmt::Debug for MCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCTL")
                .field("COREHLD", &self.COREHLD())
                .field("LSACT_EN", &self.LSACT_EN())
                .field("LSACTWREN", &self.LSACTWREN())
                .field("MASTER_REPAIR_EN", &self.MASTER_REPAIR_EN())
                .field("RFCMDEN", &self.RFCMDEN())
                .field("CWSABTEN", &self.CWSABTEN())
                .field("MRGRDDIS", &self.MRGRDDIS())
                .field("MRGRD0", &self.MRGRD0())
                .field("MRGRD1", &self.MRGRD1())
                .field("ERSAACK", &self.ERSAACK())
                .field("SCAN_OBS", &self.SCAN_OBS())
                .field("BIST_CTL", &self.BIST_CTL())
                .field("SMWR_CTL", &self.SMWR_CTL())
                .field("SALV_DIS", &self.SALV_DIS())
                .field("SOC_ECC_CTL", &self.SOC_ECC_CTL())
                .field("FMU_ECC_CTL", &self.FMU_ECC_CTL())
                .field("BIST_PWR_DIS", &self.BIST_PWR_DIS())
                .field("OSC_H", &self.OSC_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCTL {{ COREHLD: {=bool:?}, LSACT_EN: {=bool:?}, LSACTWREN: {=bool:?}, MASTER_REPAIR_EN: {=bool:?}, RFCMDEN: {=bool:?}, CWSABTEN: {=bool:?}, MRGRDDIS: {=bool:?}, MRGRD0: {=u8:?}, MRGRD1: {=u8:?}, ERSAACK: {=bool:?}, SCAN_OBS: {=bool:?}, BIST_CTL: {=bool:?}, SMWR_CTL: {=bool:?}, SALV_DIS: {=bool:?}, SOC_ECC_CTL: {=bool:?}, FMU_ECC_CTL: {=bool:?}, BIST_PWR_DIS: {=bool:?}, OSC_H: {=bool:?} }}" , self . COREHLD () , self . LSACT_EN () , self . LSACTWREN () , self . MASTER_REPAIR_EN () , self . RFCMDEN () , self . CWSABTEN () , self . MRGRDDIS () , self . MRGRD0 () , self . MRGRD1 () , self . ERSAACK () , self . SCAN_OBS () , self . BIST_CTL () , self . SMWR_CTL () , self . SALV_DIS () , self . SOC_ECC_CTL () , self . FMU_ECC_CTL () , self . BIST_PWR_DIS () , self . OSC_H ())
        }
    }
    #[doc = "Memory Map Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MM_CTL(pub u32);
    impl MM_CTL {
        #[must_use]
        #[inline(always)]
        pub const fn MM_SEL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MM_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MM_RD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MM_RD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_ON(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FORCE_SW_CLK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FORCE_SW_CLK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for MM_CTL {
        #[inline(always)]
        fn default() -> MM_CTL {
            MM_CTL(0)
        }
    }
    impl core::fmt::Debug for MM_CTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MM_CTL")
                .field("MM_SEL", &self.MM_SEL())
                .field("MM_RD", &self.MM_RD())
                .field("BIST_ON", &self.BIST_ON())
                .field("FORCE_SW_CLK", &self.FORCE_SW_CLK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MM_CTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MM_CTL {{ MM_SEL: {=bool:?}, MM_RD: {=bool:?}, BIST_ON: {=bool:?}, FORCE_SW_CLK: {=bool:?} }}" , self . MM_SEL () , self . MM_RD () , self . BIST_ON () , self . FORCE_SW_CLK ())
        }
    }
    #[doc = "FMU Memory Size Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSIZE(pub u32);
    impl MSIZE {
        #[must_use]
        #[inline(always)]
        pub const fn MAXADDR0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAXADDR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for MSIZE {
        #[inline(always)]
        fn default() -> MSIZE {
            MSIZE(0)
        }
    }
    impl core::fmt::Debug for MSIZE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSIZE")
                .field("MAXADDR0", &self.MAXADDR0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSIZE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MSIZE {{ MAXADDR0: {=u8:?} }}", self.MAXADDR0())
        }
    }
    #[doc = "Parity Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARITY(pub u32);
    impl PARITY {
        #[must_use]
        #[inline(always)]
        pub const fn PARITY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PARITY(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for PARITY {
        #[inline(always)]
        fn default() -> PARITY {
            PARITY(0)
        }
    }
    impl core::fmt::Debug for PARITY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARITY")
                .field("PARITY", &self.PARITY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARITY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PARITY {{ PARITY: {=u16:?} }}", self.PARITY())
        }
    }
    #[doc = "Port Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PORT_CTRL(pub u32);
    impl PORT_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn BDONE_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BDONE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BSDO_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BSDO_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for PORT_CTRL {
        #[inline(always)]
        fn default() -> PORT_CTRL {
            PORT_CTRL(0)
        }
    }
    impl core::fmt::Debug for PORT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PORT_CTRL")
                .field("BDONE_SEL", &self.BDONE_SEL())
                .field("BSDO_SEL", &self.BSDO_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PORT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PORT_CTRL {{ BDONE_SEL: {=u8:?}, BSDO_SEL: {=u8:?} }}",
                self.BDONE_SEL(),
                self.BSDO_SEL()
            )
        }
    }
    #[doc = "Power Mode Options Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWR_OPT(pub u32);
    impl PWR_OPT {
        #[must_use]
        #[inline(always)]
        pub const fn PD_CDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PD_CDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLM_COUNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_SLM_COUNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PD_TIMER_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PD_TIMER_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PWR_OPT {
        #[inline(always)]
        fn default() -> PWR_OPT {
            PWR_OPT(0)
        }
    }
    impl core::fmt::Debug for PWR_OPT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWR_OPT")
                .field("PD_CDIV", &self.PD_CDIV())
                .field("SLM_COUNT", &self.SLM_COUNT())
                .field("PD_TIMER_EN", &self.PD_TIMER_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWR_OPT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PWR_OPT {{ PD_CDIV: {=u8:?}, SLM_COUNT: {=u16:?}, PD_TIMER_EN: {=bool:?} }}",
                self.PD_CDIV(),
                self.SLM_COUNT(),
                self.PD_TIMER_EN()
            )
        }
    }
    #[doc = "Read Path Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RD_PATH_CTRL_STATUS(pub u32);
    impl RD_PATH_CTRL_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn RD_CAPT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RD_CAPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SE_SIZE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SE_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ECC_ENABLEB(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ECC_ENABLEB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MISR_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MISR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPY_PAR_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPY_PAR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_MUX_TO_SMW(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_MUX_TO_SMW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AD_SET(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AD_SET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_PATH_EN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_PATH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_PATH_ECC_EN(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WR_PATH_ECC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBERR_REG(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBERR_REG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SBERR_REG(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SBERR_REG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPY_PHRASE_EN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPY_PHRASE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMW_ARRAY1_SMW0_SEL(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMW_ARRAY1_SMW0_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_ECC_EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_ECC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LAST_READ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LAST_READ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RD_PATH_CTRL_STATUS {
        #[inline(always)]
        fn default() -> RD_PATH_CTRL_STATUS {
            RD_PATH_CTRL_STATUS(0)
        }
    }
    impl core::fmt::Debug for RD_PATH_CTRL_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RD_PATH_CTRL_STATUS")
                .field("RD_CAPT", &self.RD_CAPT())
                .field("SE_SIZE", &self.SE_SIZE())
                .field("ECC_ENABLEB", &self.ECC_ENABLEB())
                .field("MISR_EN", &self.MISR_EN())
                .field("CPY_PAR_EN", &self.CPY_PAR_EN())
                .field("BIST_MUX_TO_SMW", &self.BIST_MUX_TO_SMW())
                .field("AD_SET", &self.AD_SET())
                .field("WR_PATH_EN", &self.WR_PATH_EN())
                .field("WR_PATH_ECC_EN", &self.WR_PATH_ECC_EN())
                .field("DBERR_REG", &self.DBERR_REG())
                .field("SBERR_REG", &self.SBERR_REG())
                .field("CPY_PHRASE_EN", &self.CPY_PHRASE_EN())
                .field("SMW_ARRAY1_SMW0_SEL", &self.SMW_ARRAY1_SMW0_SEL())
                .field("BIST_ECC_EN", &self.BIST_ECC_EN())
                .field("LAST_READ", &self.LAST_READ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RD_PATH_CTRL_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RD_PATH_CTRL_STATUS {{ RD_CAPT: {=u8:?}, SE_SIZE: {=u8:?}, ECC_ENABLEB: {=bool:?}, MISR_EN: {=bool:?}, CPY_PAR_EN: {=bool:?}, BIST_MUX_TO_SMW: {=bool:?}, AD_SET: {=u8:?}, WR_PATH_EN: {=bool:?}, WR_PATH_ECC_EN: {=bool:?}, DBERR_REG: {=bool:?}, SBERR_REG: {=bool:?}, CPY_PHRASE_EN: {=bool:?}, SMW_ARRAY1_SMW0_SEL: {=bool:?}, BIST_ECC_EN: {=bool:?}, LAST_READ: {=bool:?} }}" , self . RD_CAPT () , self . SE_SIZE () , self . ECC_ENABLEB () , self . MISR_EN () , self . CPY_PAR_EN () , self . BIST_MUX_TO_SMW () , self . AD_SET () , self . WR_PATH_EN () , self . WR_PATH_ECC_EN () , self . DBERR_REG () , self . SBERR_REG () , self . CPY_PHRASE_EN () , self . SMW_ARRAY1_SMW0_SEL () , self . BIST_ECC_EN () , self . LAST_READ ())
        }
    }
    #[doc = "FMU Repair 0 Block 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REPAIR0_0(pub u32);
    impl REPAIR0_0 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS0_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS0_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR0_0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR0_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for REPAIR0_0 {
        #[inline(always)]
        fn default() -> REPAIR0_0 {
            REPAIR0_0(0)
        }
    }
    impl core::fmt::Debug for REPAIR0_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REPAIR0_0")
                .field("RDIS0_0", &self.RDIS0_0())
                .field("RADR0_0", &self.RADR0_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REPAIR0_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REPAIR0_0 {{ RDIS0_0: {=bool:?}, RADR0_0: {=u8:?} }}",
                self.RDIS0_0(),
                self.RADR0_0()
            )
        }
    }
    #[doc = "FMU Repair 1 Block 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REPAIR0_1(pub u32);
    impl REPAIR0_1 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS0_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS0_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR0_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR0_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for REPAIR0_1 {
        #[inline(always)]
        fn default() -> REPAIR0_1 {
            REPAIR0_1(0)
        }
    }
    impl core::fmt::Debug for REPAIR0_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REPAIR0_1")
                .field("RDIS0_1", &self.RDIS0_1())
                .field("RADR0_1", &self.RADR0_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REPAIR0_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REPAIR0_1 {{ RDIS0_1: {=bool:?}, RADR0_1: {=u8:?} }}",
                self.RDIS0_1(),
                self.RADR0_1()
            )
        }
    }
    #[doc = "FMU Repair 0 Block 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REPAIR1_0(pub u32);
    impl REPAIR1_0 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS1_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS1_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR1_0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for REPAIR1_0 {
        #[inline(always)]
        fn default() -> REPAIR1_0 {
            REPAIR1_0(0)
        }
    }
    impl core::fmt::Debug for REPAIR1_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REPAIR1_0")
                .field("RDIS1_0", &self.RDIS1_0())
                .field("RADR1_0", &self.RADR1_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REPAIR1_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REPAIR1_0 {{ RDIS1_0: {=bool:?}, RADR1_0: {=u8:?} }}",
                self.RDIS1_0(),
                self.RADR1_0()
            )
        }
    }
    #[doc = "FMU Repair 1 Block 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REPAIR1_1(pub u32);
    impl REPAIR1_1 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS1_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS1_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR1_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR1_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for REPAIR1_1 {
        #[inline(always)]
        fn default() -> REPAIR1_1 {
            REPAIR1_1(0)
        }
    }
    impl core::fmt::Debug for REPAIR1_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REPAIR1_1")
                .field("RDIS1_1", &self.RDIS1_1())
                .field("RADR1_1", &self.RADR1_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REPAIR1_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REPAIR1_1 {{ RDIS1_1: {=bool:?}, RADR1_1: {=u8:?} }}",
                self.RDIS1_1(),
                self.RADR1_1()
            )
        }
    }
    #[doc = "FMU Initialization Tracking Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RESET_STATUS(pub u32);
    impl RESET_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn ARY_TRIM_DONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ARY_TRIM_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMU_PARM_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FMU_PARM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FMU_PARM_DONE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FMU_PARM_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOC_TRIM_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOC_TRIM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOC_TRIM_ECC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOC_TRIM_ECC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOC_TRIM_DONE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOC_TRIM_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPR_DONE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RPR_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INIT_DONE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INIT_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RST_SF_ERR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RST_SF_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RST_DF_ERR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RST_DF_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOC_TRIM_DF_ERR(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SOC_TRIM_DF_ERR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RST_PATCH_LD(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RST_PATCH_LD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RECALL_DATA_MISMATCH(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RECALL_DATA_MISMATCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for RESET_STATUS {
        #[inline(always)]
        fn default() -> RESET_STATUS {
            RESET_STATUS(0)
        }
    }
    impl core::fmt::Debug for RESET_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RESET_STATUS")
                .field("ARY_TRIM_DONE", &self.ARY_TRIM_DONE())
                .field("FMU_PARM_EN", &self.FMU_PARM_EN())
                .field("FMU_PARM_DONE", &self.FMU_PARM_DONE())
                .field("SOC_TRIM_EN", &self.SOC_TRIM_EN())
                .field("SOC_TRIM_ECC", &self.SOC_TRIM_ECC())
                .field("SOC_TRIM_DONE", &self.SOC_TRIM_DONE())
                .field("RPR_DONE", &self.RPR_DONE())
                .field("INIT_DONE", &self.INIT_DONE())
                .field("RST_SF_ERR", &self.RST_SF_ERR())
                .field("RST_DF_ERR", &self.RST_DF_ERR())
                .field("SOC_TRIM_DF_ERR", &self.SOC_TRIM_DF_ERR())
                .field("RST_PATCH_LD", &self.RST_PATCH_LD())
                .field("RECALL_DATA_MISMATCH", &self.RECALL_DATA_MISMATCH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RESET_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RESET_STATUS {{ ARY_TRIM_DONE: {=bool:?}, FMU_PARM_EN: {=bool:?}, FMU_PARM_DONE: {=bool:?}, SOC_TRIM_EN: {=bool:?}, SOC_TRIM_ECC: {=bool:?}, SOC_TRIM_DONE: {=bool:?}, RPR_DONE: {=bool:?}, INIT_DONE: {=bool:?}, RST_SF_ERR: {=bool:?}, RST_DF_ERR: {=bool:?}, SOC_TRIM_DF_ERR: {=u8:?}, RST_PATCH_LD: {=bool:?}, RECALL_DATA_MISMATCH: {=bool:?} }}" , self . ARY_TRIM_DONE () , self . FMU_PARM_EN () , self . FMU_PARM_DONE () , self . SOC_TRIM_EN () , self . SOC_TRIM_ECC () , self . SOC_TRIM_DONE () , self . RPR_DONE () , self . INIT_DONE () , self . RST_SF_ERR () , self . RST_DF_ERR () , self . SOC_TRIM_DF_ERR () , self . RST_PATCH_LD () , self . RECALL_DATA_MISMATCH ())
        }
    }
    #[doc = "BIST Abort Loop Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_ABORT_LOOP(pub u32);
    impl R_ABORT_LOOP {
        #[must_use]
        #[inline(always)]
        pub const fn ABORT_LOOP(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ABORT_LOOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for R_ABORT_LOOP {
        #[inline(always)]
        fn default() -> R_ABORT_LOOP {
            R_ABORT_LOOP(0)
        }
    }
    impl core::fmt::Debug for R_ABORT_LOOP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_ABORT_LOOP")
                .field("ABORT_LOOP", &self.ABORT_LOOP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_ABORT_LOOP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_ABORT_LOOP {{ ABORT_LOOP: {=bool:?} }}",
                self.ABORT_LOOP()
            )
        }
    }
    #[doc = "BIST Address Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_ADR_CTRL(pub u32);
    impl R_ADR_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn GRPSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GRPSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XADR(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_XADR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn YADR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_YADR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROG_ATTR(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PROG_ATTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
    }
    impl Default for R_ADR_CTRL {
        #[inline(always)]
        fn default() -> R_ADR_CTRL {
            R_ADR_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_ADR_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_ADR_CTRL")
                .field("GRPSEL", &self.GRPSEL())
                .field("XADR", &self.XADR())
                .field("YADR", &self.YADR())
                .field("PROG_ATTR", &self.PROG_ATTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_ADR_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_ADR_CTRL {{ GRPSEL: {=u8:?}, XADR: {=u16:?}, YADR: {=u8:?}, PROG_ATTR: {=u8:?} }}" , self . GRPSEL () , self . XADR () , self . YADR () , self . PROG_ATTR ())
        }
    }
    #[doc = "BIST Address Query Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_ADR_QUERY(pub u32);
    impl R_ADR_QUERY {
        #[must_use]
        #[inline(always)]
        pub const fn YADRFAIL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_YADRFAIL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XADRFAIL(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_XADRFAIL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
        }
    }
    impl Default for R_ADR_QUERY {
        #[inline(always)]
        fn default() -> R_ADR_QUERY {
            R_ADR_QUERY(0)
        }
    }
    impl core::fmt::Debug for R_ADR_QUERY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_ADR_QUERY")
                .field("YADRFAIL", &self.YADRFAIL())
                .field("XADRFAIL", &self.XADRFAIL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_ADR_QUERY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_ADR_QUERY {{ YADRFAIL: {=u8:?}, XADRFAIL: {=u16:?} }}",
                self.YADRFAIL(),
                self.XADRFAIL()
            )
        }
    }
    #[doc = "BIST Address MISR 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_A_MISR1(pub u32);
    impl R_A_MISR1 {
        #[must_use]
        #[inline(always)]
        pub const fn ADRSIG1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADRSIG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for R_A_MISR1 {
        #[inline(always)]
        fn default() -> R_A_MISR1 {
            R_A_MISR1(0)
        }
    }
    impl core::fmt::Debug for R_A_MISR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_A_MISR1")
                .field("ADRSIG1", &self.ADRSIG1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_A_MISR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_A_MISR1 {{ ADRSIG1: {=u8:?} }}", self.ADRSIG1())
        }
    }
    #[doc = "BIST Loop Count Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_CNT_LOOP_CTRL(pub u32);
    impl R_CNT_LOOP_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn LOOPCNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LOOPCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOOPOPT(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOOPOPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOOPUNIT(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOOPUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOOPDLY(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOOPDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
        }
    }
    impl Default for R_CNT_LOOP_CTRL {
        #[inline(always)]
        fn default() -> R_CNT_LOOP_CTRL {
            R_CNT_LOOP_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_CNT_LOOP_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_CNT_LOOP_CTRL")
                .field("LOOPCNT", &self.LOOPCNT())
                .field("LOOPOPT", &self.LOOPOPT())
                .field("LOOPUNIT", &self.LOOPUNIT())
                .field("LOOPDLY", &self.LOOPDLY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_CNT_LOOP_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_CNT_LOOP_CTRL {{ LOOPCNT: {=u16:?}, LOOPOPT: {=u8:?}, LOOPUNIT: {=u8:?}, LOOPDLY: {=u8:?} }}" , self . LOOPCNT () , self . LOOPOPT () , self . LOOPUNIT () , self . LOOPDLY ())
        }
    }
    #[doc = "BIST Control MISR 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_C_MISR1(pub u32);
    impl R_C_MISR1 {
        #[must_use]
        #[inline(always)]
        pub const fn CTRLSIG1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTRLSIG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for R_C_MISR1 {
        #[inline(always)]
        fn default() -> R_C_MISR1 {
            R_C_MISR1(0)
        }
    }
    impl core::fmt::Debug for R_C_MISR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_C_MISR1")
                .field("CTRLSIG1", &self.CTRLSIG1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_C_MISR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_C_MISR1 {{ CTRLSIG1: {=u8:?} }}", self.CTRLSIG1())
        }
    }
    #[doc = "BIST Data Control 0 Extension Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DATA_CTRL0_EX(pub u32);
    impl R_DATA_CTRL0_EX {
        #[must_use]
        #[inline(always)]
        pub const fn DATA0X(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA0X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_DATA_CTRL0_EX {
        #[inline(always)]
        fn default() -> R_DATA_CTRL0_EX {
            R_DATA_CTRL0_EX(0)
        }
    }
    impl core::fmt::Debug for R_DATA_CTRL0_EX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DATA_CTRL0_EX")
                .field("DATA0X", &self.DATA0X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DATA_CTRL0_EX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_DATA_CTRL0_EX {{ DATA0X: {=u8:?} }}", self.DATA0X())
        }
    }
    #[doc = "BIST Data Control 1 Extension Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DATA_CTRL1_EX(pub u32);
    impl R_DATA_CTRL1_EX {
        #[must_use]
        #[inline(always)]
        pub const fn DATA1X(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA1X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_DATA_CTRL1_EX {
        #[inline(always)]
        fn default() -> R_DATA_CTRL1_EX {
            R_DATA_CTRL1_EX(0)
        }
    }
    impl core::fmt::Debug for R_DATA_CTRL1_EX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DATA_CTRL1_EX")
                .field("DATA1X", &self.DATA1X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DATA_CTRL1_EX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_DATA_CTRL1_EX {{ DATA1X: {=u8:?} }}", self.DATA1X())
        }
    }
    #[doc = "BIST Data Control 2 Extension Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DATA_CTRL2_EX(pub u32);
    impl R_DATA_CTRL2_EX {
        #[must_use]
        #[inline(always)]
        pub const fn DATA2X(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA2X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_DATA_CTRL2_EX {
        #[inline(always)]
        fn default() -> R_DATA_CTRL2_EX {
            R_DATA_CTRL2_EX(0)
        }
    }
    impl core::fmt::Debug for R_DATA_CTRL2_EX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DATA_CTRL2_EX")
                .field("DATA2X", &self.DATA2X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DATA_CTRL2_EX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_DATA_CTRL2_EX {{ DATA2X: {=u8:?} }}", self.DATA2X())
        }
    }
    #[doc = "BIST Data Control 3 Extension Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DATA_CTRL3_EX(pub u32);
    impl R_DATA_CTRL3_EX {
        #[must_use]
        #[inline(always)]
        pub const fn DATA3X(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATA3X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_DATA_CTRL3_EX {
        #[inline(always)]
        fn default() -> R_DATA_CTRL3_EX {
            R_DATA_CTRL3_EX(0)
        }
    }
    impl core::fmt::Debug for R_DATA_CTRL3_EX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DATA_CTRL3_EX")
                .field("DATA3X", &self.DATA3X())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DATA_CTRL3_EX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_DATA_CTRL3_EX {{ DATA3X: {=u8:?} }}", self.DATA3X())
        }
    }
    #[doc = "BIST DFT Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DFT_CTRL(pub u32);
    impl R_DFT_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DFT_XADR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DFT_XADR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFT_YADR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DFT_YADR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFT_DATA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DFT_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMP_MASK(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMP_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFT_DATA_SRC(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFT_DATA_SRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for R_DFT_CTRL {
        #[inline(always)]
        fn default() -> R_DFT_CTRL {
            R_DFT_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_DFT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DFT_CTRL")
                .field("DFT_XADR", &self.DFT_XADR())
                .field("DFT_YADR", &self.DFT_YADR())
                .field("DFT_DATA", &self.DFT_DATA())
                .field("CMP_MASK", &self.CMP_MASK())
                .field("DFT_DATA_SRC", &self.DFT_DATA_SRC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DFT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_DFT_CTRL {{ DFT_XADR: {=u8:?}, DFT_YADR: {=u8:?}, DFT_DATA: {=u8:?}, CMP_MASK: {=u8:?}, DFT_DATA_SRC: {=bool:?} }}" , self . DFT_XADR () , self . DFT_YADR () , self . DFT_DATA () , self . CMP_MASK () , self . DFT_DATA_SRC ())
        }
    }
    #[doc = "BIST DOUT Query 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_DOUT_QUERY1(pub u32);
    impl R_DOUT_QUERY1 {
        #[must_use]
        #[inline(always)]
        pub const fn DOUT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_DOUT_QUERY1 {
        #[inline(always)]
        fn default() -> R_DOUT_QUERY1 {
            R_DOUT_QUERY1(0)
        }
    }
    impl core::fmt::Debug for R_DOUT_QUERY1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_DOUT_QUERY1")
                .field("DOUT", &self.DOUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_DOUT_QUERY1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_DOUT_QUERY1 {{ DOUT: {=u8:?} }}", self.DOUT())
        }
    }
    #[doc = "BIST DIN MISR 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_D_MISR1(pub u32);
    impl R_D_MISR1 {
        #[must_use]
        #[inline(always)]
        pub const fn DATASIG1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATASIG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for R_D_MISR1 {
        #[inline(always)]
        fn default() -> R_D_MISR1 {
            R_D_MISR1(0)
        }
    }
    impl core::fmt::Debug for R_D_MISR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_D_MISR1")
                .field("DATASIG1", &self.DATASIG1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_D_MISR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_D_MISR1 {{ DATASIG1: {=u8:?} }}", self.DATASIG1())
        }
    }
    #[doc = "BIST Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_IP_CONFIG(pub u32);
    impl R_IP_CONFIG {
        #[must_use]
        #[inline(always)]
        pub const fn IPSEL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPSEL1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_CDIVL(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BIST_CDIVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDIVS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CDIVS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_TVFY(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BIST_TVFY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSTCTL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TSTCTL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBGCTL(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBGCTL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_CLK_SEL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_CLK_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMWTST(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMWTST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ECCEN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ECCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for R_IP_CONFIG {
        #[inline(always)]
        fn default() -> R_IP_CONFIG {
            R_IP_CONFIG(0)
        }
    }
    impl core::fmt::Debug for R_IP_CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_IP_CONFIG")
                .field("IPSEL0", &self.IPSEL0())
                .field("IPSEL1", &self.IPSEL1())
                .field("BIST_CDIVL", &self.BIST_CDIVL())
                .field("CDIVS", &self.CDIVS())
                .field("BIST_TVFY", &self.BIST_TVFY())
                .field("TSTCTL", &self.TSTCTL())
                .field("DBGCTL", &self.DBGCTL())
                .field("BIST_CLK_SEL", &self.BIST_CLK_SEL())
                .field("SMWTST", &self.SMWTST())
                .field("ECCEN", &self.ECCEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_IP_CONFIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_IP_CONFIG {{ IPSEL0: {=u8:?}, IPSEL1: {=u8:?}, BIST_CDIVL: {=u8:?}, CDIVS: {=u8:?}, BIST_TVFY: {=u8:?}, TSTCTL: {=u8:?}, DBGCTL: {=bool:?}, BIST_CLK_SEL: {=bool:?}, SMWTST: {=u8:?}, ECCEN: {=bool:?} }}" , self . IPSEL0 () , self . IPSEL1 () , self . BIST_CDIVL () , self . CDIVS () , self . BIST_TVFY () , self . TSTCTL () , self . DBGCTL () , self . BIST_CLK_SEL () , self . SMWTST () , self . ECCEN ())
        }
    }
    #[doc = "BIST Pin Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_PIN_CTRL(pub u32);
    impl R_PIN_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn MAS1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MAS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IFREN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IFREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IFREN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IFREN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REDEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LVE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PV(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EV(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WIPGM(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WIPGM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WHV(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WHV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WMV(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WMV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_XE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn YE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_YE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERASE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROG(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NVSTR(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NVSTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLM(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RECALL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RECALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HEM(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for R_PIN_CTRL {
        #[inline(always)]
        fn default() -> R_PIN_CTRL {
            R_PIN_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_PIN_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_PIN_CTRL")
                .field("MAS1", &self.MAS1())
                .field("IFREN", &self.IFREN())
                .field("IFREN1", &self.IFREN1())
                .field("REDEN", &self.REDEN())
                .field("LVE", &self.LVE())
                .field("PV", &self.PV())
                .field("EV", &self.EV())
                .field("WIPGM", &self.WIPGM())
                .field("WHV", &self.WHV())
                .field("WMV", &self.WMV())
                .field("XE", &self.XE())
                .field("YE", &self.YE())
                .field("SE", &self.SE())
                .field("ERASE", &self.ERASE())
                .field("PROG", &self.PROG())
                .field("NVSTR", &self.NVSTR())
                .field("SLM", &self.SLM())
                .field("RECALL", &self.RECALL())
                .field("HEM", &self.HEM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_PIN_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_PIN_CTRL {{ MAS1: {=bool:?}, IFREN: {=bool:?}, IFREN1: {=bool:?}, REDEN: {=bool:?}, LVE: {=bool:?}, PV: {=bool:?}, EV: {=bool:?}, WIPGM: {=u8:?}, WHV: {=u8:?}, WMV: {=u8:?}, XE: {=bool:?}, YE: {=bool:?}, SE: {=bool:?}, ERASE: {=bool:?}, PROG: {=bool:?}, NVSTR: {=bool:?}, SLM: {=bool:?}, RECALL: {=bool:?}, HEM: {=bool:?} }}" , self . MAS1 () , self . IFREN () , self . IFREN1 () , self . REDEN () , self . LVE () , self . PV () , self . EV () , self . WIPGM () , self . WHV () , self . WMV () , self . XE () , self . YE () , self . SE () , self . ERASE () , self . PROG () , self . NVSTR () , self . SLM () , self . RECALL () , self . HEM ())
        }
    }
    #[doc = "BIST Repair 0 for Block 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_REPAIR0_0(pub u32);
    impl R_REPAIR0_0 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS0_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS0_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR0_0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR0_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for R_REPAIR0_0 {
        #[inline(always)]
        fn default() -> R_REPAIR0_0 {
            R_REPAIR0_0(0)
        }
    }
    impl core::fmt::Debug for R_REPAIR0_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_REPAIR0_0")
                .field("RDIS0_0", &self.RDIS0_0())
                .field("RADR0_0", &self.RADR0_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_REPAIR0_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_REPAIR0_0 {{ RDIS0_0: {=bool:?}, RADR0_0: {=u8:?} }}",
                self.RDIS0_0(),
                self.RADR0_0()
            )
        }
    }
    #[doc = "BIST Repair 1 Block 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_REPAIR0_1(pub u32);
    impl R_REPAIR0_1 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS0_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS0_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR0_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR0_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for R_REPAIR0_1 {
        #[inline(always)]
        fn default() -> R_REPAIR0_1 {
            R_REPAIR0_1(0)
        }
    }
    impl core::fmt::Debug for R_REPAIR0_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_REPAIR0_1")
                .field("RDIS0_1", &self.RDIS0_1())
                .field("RADR0_1", &self.RADR0_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_REPAIR0_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_REPAIR0_1 {{ RDIS0_1: {=bool:?}, RADR0_1: {=u8:?} }}",
                self.RDIS0_1(),
                self.RADR0_1()
            )
        }
    }
    #[doc = "BIST Repair 0 Block 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_REPAIR1_0(pub u32);
    impl R_REPAIR1_0 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS1_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS1_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR1_0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for R_REPAIR1_0 {
        #[inline(always)]
        fn default() -> R_REPAIR1_0 {
            R_REPAIR1_0(0)
        }
    }
    impl core::fmt::Debug for R_REPAIR1_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_REPAIR1_0")
                .field("RDIS1_0", &self.RDIS1_0())
                .field("RADR1_0", &self.RADR1_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_REPAIR1_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_REPAIR1_0 {{ RDIS1_0: {=bool:?}, RADR1_0: {=u8:?} }}",
                self.RDIS1_0(),
                self.RADR1_0()
            )
        }
    }
    #[doc = "BIST Repair 1 Block 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_REPAIR1_1(pub u32);
    impl R_REPAIR1_1 {
        #[must_use]
        #[inline(always)]
        pub const fn RDIS1_1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDIS1_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RADR1_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RADR1_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
    }
    impl Default for R_REPAIR1_1 {
        #[inline(always)]
        fn default() -> R_REPAIR1_1 {
            R_REPAIR1_1(0)
        }
    }
    impl core::fmt::Debug for R_REPAIR1_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_REPAIR1_1")
                .field("RDIS1_1", &self.RDIS1_1())
                .field("RADR1_1", &self.RADR1_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_REPAIR1_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_REPAIR1_1 {{ RDIS1_1: {=bool:?}, RADR1_1: {=u8:?} }}",
                self.RDIS1_1(),
                self.RADR1_1()
            )
        }
    }
    #[doc = "BIST SMW Query Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_SMW_QUERY(pub u32);
    impl R_SMW_QUERY {
        #[must_use]
        #[inline(always)]
        pub const fn SMWLOOP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_SMWLOOP(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMWLAST(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_SMWLAST(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 10usize)) | (((val as u32) & 0x01ff) << 10usize);
        }
    }
    impl Default for R_SMW_QUERY {
        #[inline(always)]
        fn default() -> R_SMW_QUERY {
            R_SMW_QUERY(0)
        }
    }
    impl core::fmt::Debug for R_SMW_QUERY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_SMW_QUERY")
                .field("SMWLOOP", &self.SMWLOOP())
                .field("SMWLAST", &self.SMWLAST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_SMW_QUERY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_SMW_QUERY {{ SMWLOOP: {=u16:?}, SMWLAST: {=u16:?} }}",
                self.SMWLOOP(),
                self.SMWLAST()
            )
        }
    }
    #[doc = "BIST SMW Setting 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_SMW_SETTING0(pub u32);
    impl R_SMW_SETTING0 {
        #[must_use]
        #[inline(always)]
        pub const fn SMWPARM0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_SMWPARM0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for R_SMW_SETTING0 {
        #[inline(always)]
        fn default() -> R_SMW_SETTING0 {
            R_SMW_SETTING0(0)
        }
    }
    impl core::fmt::Debug for R_SMW_SETTING0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_SMW_SETTING0")
                .field("SMWPARM0", &self.SMWPARM0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_SMW_SETTING0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_SMW_SETTING0 {{ SMWPARM0: {=u32:?} }}",
                self.SMWPARM0()
            )
        }
    }
    #[doc = "BIST SMW Setting 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_SMW_SETTING1(pub u32);
    impl R_SMW_SETTING1 {
        #[must_use]
        #[inline(always)]
        pub const fn SMWPARM1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_SMWPARM1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for R_SMW_SETTING1 {
        #[inline(always)]
        fn default() -> R_SMW_SETTING1 {
            R_SMW_SETTING1(0)
        }
    }
    impl core::fmt::Debug for R_SMW_SETTING1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_SMW_SETTING1")
                .field("SMWPARM1", &self.SMWPARM1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_SMW_SETTING1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_SMW_SETTING1 {{ SMWPARM1: {=u32:?} }}",
                self.SMWPARM1()
            )
        }
    }
    #[doc = "BIST SMW Setting 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_SMW_SETTING2(pub u32);
    impl R_SMW_SETTING2 {
        #[must_use]
        #[inline(always)]
        pub const fn SMWPARM2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_SMWPARM2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for R_SMW_SETTING2 {
        #[inline(always)]
        fn default() -> R_SMW_SETTING2 {
            R_SMW_SETTING2(0)
        }
    }
    impl core::fmt::Debug for R_SMW_SETTING2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_SMW_SETTING2")
                .field("SMWPARM2", &self.SMWPARM2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_SMW_SETTING2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_SMW_SETTING2 {{ SMWPARM2: {=u32:?} }}",
                self.SMWPARM2()
            )
        }
    }
    #[doc = "BIST SMW Setting 3 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_SMW_SETTING3(pub u32);
    impl R_SMW_SETTING3 {
        #[must_use]
        #[inline(always)]
        pub const fn SMWPARM3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_SMWPARM3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for R_SMW_SETTING3 {
        #[inline(always)]
        fn default() -> R_SMW_SETTING3 {
            R_SMW_SETTING3(0)
        }
    }
    impl core::fmt::Debug for R_SMW_SETTING3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_SMW_SETTING3")
                .field("SMWPARM3", &self.SMWPARM3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_SMW_SETTING3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_SMW_SETTING3 {{ SMWPARM3: {=u32:?} }}",
                self.SMWPARM3()
            )
        }
    }
    #[doc = "BIST Test Code Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_TESTCODE(pub u32);
    impl R_TESTCODE {
        #[must_use]
        #[inline(always)]
        pub const fn TESTCODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TESTCODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for R_TESTCODE {
        #[inline(always)]
        fn default() -> R_TESTCODE {
            R_TESTCODE(0)
        }
    }
    impl core::fmt::Debug for R_TESTCODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_TESTCODE")
                .field("TESTCODE", &self.TESTCODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_TESTCODE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "R_TESTCODE {{ TESTCODE: {=u8:?} }}", self.TESTCODE())
        }
    }
    #[doc = "BIST Test Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_TEST_CTRL(pub u32);
    impl R_TEST_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBUG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBUG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STATUS0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STATUS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STATUS1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STATUS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBUGRUN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBUGRUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STARTRUN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STARTRUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDINDEX(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CMDINDEX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DISABLE_IP1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DISABLE_IP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for R_TEST_CTRL {
        #[inline(always)]
        fn default() -> R_TEST_CTRL {
            R_TEST_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_TEST_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_TEST_CTRL")
                .field("BUSY", &self.BUSY())
                .field("DEBUG", &self.DEBUG())
                .field("STATUS0", &self.STATUS0())
                .field("STATUS1", &self.STATUS1())
                .field("DEBUGRUN", &self.DEBUGRUN())
                .field("STARTRUN", &self.STARTRUN())
                .field("CMDINDEX", &self.CMDINDEX())
                .field("DISABLE_IP1", &self.DISABLE_IP1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_TEST_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_TEST_CTRL {{ BUSY: {=bool:?}, DEBUG: {=bool:?}, STATUS0: {=bool:?}, STATUS1: {=bool:?}, DEBUGRUN: {=bool:?}, STARTRUN: {=bool:?}, CMDINDEX: {=u16:?}, DISABLE_IP1: {=bool:?} }}" , self . BUSY () , self . DEBUG () , self . STATUS0 () , self . STATUS1 () , self . DEBUGRUN () , self . STARTRUN () , self . CMDINDEX () , self . DISABLE_IP1 ())
        }
    }
    #[doc = "BIST Timer Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_TIMER_CTRL(pub u32);
    impl R_TIMER_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn TNVSUNIT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVSUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNVSDLY(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVSDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNVHUNIT(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVHUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNVHDLY(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVHDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPGSUNIT(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPGSUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPGSDLY(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPGSDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRCVUNIT(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRCVUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRCVDLY(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRCVDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TLVSUNIT(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TLVSUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TLVSDLY_L(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TLVSDLY_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for R_TIMER_CTRL {
        #[inline(always)]
        fn default() -> R_TIMER_CTRL {
            R_TIMER_CTRL(0)
        }
    }
    impl core::fmt::Debug for R_TIMER_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_TIMER_CTRL")
                .field("TNVSUNIT", &self.TNVSUNIT())
                .field("TNVSDLY", &self.TNVSDLY())
                .field("TNVHUNIT", &self.TNVHUNIT())
                .field("TNVHDLY", &self.TNVHDLY())
                .field("TPGSUNIT", &self.TPGSUNIT())
                .field("TPGSDLY", &self.TPGSDLY())
                .field("TRCVUNIT", &self.TRCVUNIT())
                .field("TRCVDLY", &self.TRCVDLY())
                .field("TLVSUNIT", &self.TLVSUNIT())
                .field("TLVSDLY_L", &self.TLVSDLY_L())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_TIMER_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R_TIMER_CTRL {{ TNVSUNIT: {=u8:?}, TNVSDLY: {=u8:?}, TNVHUNIT: {=u8:?}, TNVHDLY: {=u8:?}, TPGSUNIT: {=u8:?}, TPGSDLY: {=u8:?}, TRCVUNIT: {=u8:?}, TRCVDLY: {=u8:?}, TLVSUNIT: {=u8:?}, TLVSDLY_L: {=bool:?} }}" , self . TNVSUNIT () , self . TNVSDLY () , self . TNVHUNIT () , self . TNVHDLY () , self . TPGSUNIT () , self . TPGSDLY () , self . TRCVUNIT () , self . TRCVDLY () , self . TLVSUNIT () , self . TLVSDLY_L ())
        }
    }
    #[doc = "BIST Timer Control Extension Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R_TIMER_CTRL_EX(pub u32);
    impl R_TIMER_CTRL_EX {
        #[must_use]
        #[inline(always)]
        pub const fn TLVSDLY_H(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TLVSDLY_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for R_TIMER_CTRL_EX {
        #[inline(always)]
        fn default() -> R_TIMER_CTRL_EX {
            R_TIMER_CTRL_EX(0)
        }
    }
    impl core::fmt::Debug for R_TIMER_CTRL_EX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R_TIMER_CTRL_EX")
                .field("TLVSDLY_H", &self.TLVSDLY_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R_TIMER_CTRL_EX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R_TIMER_CTRL_EX {{ TLVSDLY_H: {=u8:?} }}",
                self.TLVSDLY_H()
            )
        }
    }
    #[doc = "SMW Command and Wait Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_CMD_WAIT(pub u32);
    impl SMW_CMD_WAIT {
        #[must_use]
        #[inline(always)]
        pub const fn CMD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAIT_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAIT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAIT_AUTO_SET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAIT_AUTO_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for SMW_CMD_WAIT {
        #[inline(always)]
        fn default() -> SMW_CMD_WAIT {
            SMW_CMD_WAIT(0)
        }
    }
    impl core::fmt::Debug for SMW_CMD_WAIT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_CMD_WAIT")
                .field("CMD", &self.CMD())
                .field("WAIT_EN", &self.WAIT_EN())
                .field("WAIT_AUTO_SET", &self.WAIT_AUTO_SET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_CMD_WAIT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMW_CMD_WAIT {{ CMD: {=u8:?}, WAIT_EN: {=bool:?}, WAIT_AUTO_SET: {=bool:?} }}",
                self.CMD(),
                self.WAIT_EN(),
                self.WAIT_AUTO_SET()
            )
        }
    }
    #[doc = "SMW HB Signals Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_HB_SIGNALS(pub u32);
    impl SMW_HB_SIGNALS {
        #[must_use]
        #[inline(always)]
        pub const fn SMW_ARRAY(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMW_ARRAY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_IFREN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_IFREN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_PV(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_PV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_EV(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_EV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_IFREN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_IFREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_REDEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_REDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USER_HEM(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USER_HEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for SMW_HB_SIGNALS {
        #[inline(always)]
        fn default() -> SMW_HB_SIGNALS {
            SMW_HB_SIGNALS(0)
        }
    }
    impl core::fmt::Debug for SMW_HB_SIGNALS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_HB_SIGNALS")
                .field("SMW_ARRAY", &self.SMW_ARRAY())
                .field("USER_IFREN1", &self.USER_IFREN1())
                .field("USER_PV", &self.USER_PV())
                .field("USER_EV", &self.USER_EV())
                .field("USER_IFREN", &self.USER_IFREN())
                .field("USER_REDEN", &self.USER_REDEN())
                .field("USER_HEM", &self.USER_HEM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_HB_SIGNALS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SMW_HB_SIGNALS {{ SMW_ARRAY: {=u8:?}, USER_IFREN1: {=bool:?}, USER_PV: {=bool:?}, USER_EV: {=bool:?}, USER_IFREN: {=bool:?}, USER_REDEN: {=bool:?}, USER_HEM: {=bool:?} }}" , self . SMW_ARRAY () , self . USER_IFREN1 () , self . USER_PV () , self . USER_EV () , self . USER_IFREN () , self . USER_REDEN () , self . USER_HEM ())
        }
    }
    #[doc = "SMW Setting Option 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_SETTING_OPTION0(pub u32);
    impl SMW_SETTING_OPTION0 {
        #[must_use]
        #[inline(always)]
        pub const fn MV_INIT(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MV_INIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MV_END(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MV_END(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MV_MISC(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MV_MISC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPGM_INIT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPGM_INIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPGM_END(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPGM_END(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPGM_MISC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPGM_MISC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for SMW_SETTING_OPTION0 {
        #[inline(always)]
        fn default() -> SMW_SETTING_OPTION0 {
            SMW_SETTING_OPTION0(0)
        }
    }
    impl core::fmt::Debug for SMW_SETTING_OPTION0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_SETTING_OPTION0")
                .field("MV_INIT", &self.MV_INIT())
                .field("MV_END", &self.MV_END())
                .field("MV_MISC", &self.MV_MISC())
                .field("IPGM_INIT", &self.IPGM_INIT())
                .field("IPGM_END", &self.IPGM_END())
                .field("IPGM_MISC", &self.IPGM_MISC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_SETTING_OPTION0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SMW_SETTING_OPTION0 {{ MV_INIT: {=u8:?}, MV_END: {=u8:?}, MV_MISC: {=u8:?}, IPGM_INIT: {=u8:?}, IPGM_END: {=u8:?}, IPGM_MISC: {=u8:?} }}" , self . MV_INIT () , self . MV_END () , self . MV_MISC () , self . IPGM_INIT () , self . IPGM_END () , self . IPGM_MISC ())
        }
    }
    #[doc = "SMW Setting Option 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_SETTING_OPTION1(pub u32);
    impl SMW_SETTING_OPTION1 {
        #[must_use]
        #[inline(always)]
        pub const fn TERS_CTRL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TERS_CTRL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPGM_CTRL(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPGM_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNVS_CTRL(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVS_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TNVH_CTRL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TNVH_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPGS_CTRL(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPGS_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAX_ERASE(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MAX_ERASE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 14usize)) | (((val as u32) & 0x01ff) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAX_PROG(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAX_PROG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
        }
    }
    impl Default for SMW_SETTING_OPTION1 {
        #[inline(always)]
        fn default() -> SMW_SETTING_OPTION1 {
            SMW_SETTING_OPTION1(0)
        }
    }
    impl core::fmt::Debug for SMW_SETTING_OPTION1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_SETTING_OPTION1")
                .field("TERS_CTRL0", &self.TERS_CTRL0())
                .field("TPGM_CTRL", &self.TPGM_CTRL())
                .field("TNVS_CTRL", &self.TNVS_CTRL())
                .field("TNVH_CTRL", &self.TNVH_CTRL())
                .field("TPGS_CTRL", &self.TPGS_CTRL())
                .field("MAX_ERASE", &self.MAX_ERASE())
                .field("MAX_PROG", &self.MAX_PROG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_SETTING_OPTION1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SMW_SETTING_OPTION1 {{ TERS_CTRL0: {=u8:?}, TPGM_CTRL: {=u8:?}, TNVS_CTRL: {=u8:?}, TNVH_CTRL: {=u8:?}, TPGS_CTRL: {=u8:?}, MAX_ERASE: {=u16:?}, MAX_PROG: {=u8:?} }}" , self . TERS_CTRL0 () , self . TPGM_CTRL () , self . TNVS_CTRL () , self . TNVH_CTRL () , self . TPGS_CTRL () , self . MAX_ERASE () , self . MAX_PROG ())
        }
    }
    #[doc = "SMW Setting Option 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_SETTING_OPTION2(pub u32);
    impl SMW_SETTING_OPTION2 {
        #[must_use]
        #[inline(always)]
        pub const fn THVS_CTRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_THVS_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRCV_CTRL(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRCV_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XTRA_ERS(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_XTRA_ERS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XTRA_PGM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_XTRA_PGM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WHV_CNTR(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WHV_CNTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POST_TERS(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_POST_TERS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POST_TPGM(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_POST_TPGM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VFY_OPT(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VFY_OPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPGM_OPT(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPGM_OPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MASK0_OPT(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MASK0_OPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS_PRER(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS_PRER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for SMW_SETTING_OPTION2 {
        #[inline(always)]
        fn default() -> SMW_SETTING_OPTION2 {
            SMW_SETTING_OPTION2(0)
        }
    }
    impl core::fmt::Debug for SMW_SETTING_OPTION2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_SETTING_OPTION2")
                .field("THVS_CTRL", &self.THVS_CTRL())
                .field("TRCV_CTRL", &self.TRCV_CTRL())
                .field("XTRA_ERS", &self.XTRA_ERS())
                .field("XTRA_PGM", &self.XTRA_PGM())
                .field("WHV_CNTR", &self.WHV_CNTR())
                .field("POST_TERS", &self.POST_TERS())
                .field("POST_TPGM", &self.POST_TPGM())
                .field("VFY_OPT", &self.VFY_OPT())
                .field("TPGM_OPT", &self.TPGM_OPT())
                .field("MASK0_OPT", &self.MASK0_OPT())
                .field("DIS_PRER", &self.DIS_PRER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_SETTING_OPTION2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SMW_SETTING_OPTION2 {{ THVS_CTRL: {=u8:?}, TRCV_CTRL: {=u8:?}, XTRA_ERS: {=u8:?}, XTRA_PGM: {=u8:?}, WHV_CNTR: {=u8:?}, POST_TERS: {=u8:?}, POST_TPGM: {=u8:?}, VFY_OPT: {=u8:?}, TPGM_OPT: {=u8:?}, MASK0_OPT: {=bool:?}, DIS_PRER: {=bool:?} }}" , self . THVS_CTRL () , self . TRCV_CTRL () , self . XTRA_ERS () , self . XTRA_PGM () , self . WHV_CNTR () , self . POST_TERS () , self . POST_TPGM () , self . VFY_OPT () , self . TPGM_OPT () , self . MASK0_OPT () , self . DIS_PRER ())
        }
    }
    #[doc = "SMW Setting Option 3 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_SETTING_OPTION3(pub u32);
    impl SMW_SETTING_OPTION3 {
        #[must_use]
        #[inline(always)]
        pub const fn HEM_WHV_CNTR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HEM_WHV_CNTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HEM_MAX_ERS(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_HEM_MAX_ERS(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
    }
    impl Default for SMW_SETTING_OPTION3 {
        #[inline(always)]
        fn default() -> SMW_SETTING_OPTION3 {
            SMW_SETTING_OPTION3(0)
        }
    }
    impl core::fmt::Debug for SMW_SETTING_OPTION3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_SETTING_OPTION3")
                .field("HEM_WHV_CNTR", &self.HEM_WHV_CNTR())
                .field("HEM_MAX_ERS", &self.HEM_MAX_ERS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_SETTING_OPTION3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMW_SETTING_OPTION3 {{ HEM_WHV_CNTR: {=u8:?}, HEM_MAX_ERS: {=u16:?} }}",
                self.HEM_WHV_CNTR(),
                self.HEM_MAX_ERS()
            )
        }
    }
    #[doc = "SMW Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_STATUS(pub u32);
    impl SMW_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn SMW_ERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMW_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMW_BUSY(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMW_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIST_BUSY(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIST_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SMW_STATUS {
        #[inline(always)]
        fn default() -> SMW_STATUS {
            SMW_STATUS(0)
        }
    }
    impl core::fmt::Debug for SMW_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_STATUS")
                .field("SMW_ERR", &self.SMW_ERR())
                .field("SMW_BUSY", &self.SMW_BUSY())
                .field("BIST_BUSY", &self.BIST_BUSY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMW_STATUS {{ SMW_ERR: {=bool:?}, SMW_BUSY: {=bool:?}, BIST_BUSY: {=bool:?} }}",
                self.SMW_ERR(),
                self.SMW_BUSY(),
                self.BIST_BUSY()
            )
        }
    }
    #[doc = "SMW Timer Option Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMW_TIMER_OPTION(pub u32);
    impl SMW_TIMER_OPTION {
        #[must_use]
        #[inline(always)]
        pub const fn SMW_CDIVL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMW_CDIVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMW_TVFY(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SMW_TVFY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for SMW_TIMER_OPTION {
        #[inline(always)]
        fn default() -> SMW_TIMER_OPTION {
            SMW_TIMER_OPTION(0)
        }
    }
    impl core::fmt::Debug for SMW_TIMER_OPTION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMW_TIMER_OPTION")
                .field("SMW_CDIVL", &self.SMW_CDIVL())
                .field("SMW_TVFY", &self.SMW_TVFY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMW_TIMER_OPTION {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SMW_TIMER_OPTION {{ SMW_CDIVL: {=u8:?}, SMW_TVFY: {=u8:?} }}",
                self.SMW_CDIVL(),
                self.SMW_TVFY()
            )
        }
    }
    #[doc = "User Interface Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UINT_CTL(pub u32);
    impl UINT_CTL {
        #[must_use]
        #[inline(always)]
        pub const fn SET_FAIL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SET_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for UINT_CTL {
        #[inline(always)]
        fn default() -> UINT_CTL {
            UINT_CTL(0)
        }
    }
    impl core::fmt::Debug for UINT_CTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UINT_CTL")
                .field("SET_FAIL", &self.SET_FAIL())
                .field("DBERR", &self.DBERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UINT_CTL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UINT_CTL {{ SET_FAIL: {=bool:?}, DBERR: {=bool:?} }}",
                self.SET_FAIL(),
                self.DBERR()
            )
        }
    }
}
