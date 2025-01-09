#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MRCC {
    ptr: *mut u8,
}
unsafe impl Send for MRCC {}
unsafe impl Sync for MRCC {}
impl MRCC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST0(self) -> crate::common::Reg<regs::MRCC_GLB_RST0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST0_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST0_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST1(self) -> crate::common::Reg<regs::MRCC_GLB_RST1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST1_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST1_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST2(self) -> crate::common::Reg<regs::MRCC_GLB_RST2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST2_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_RST2_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC0(self) -> crate::common::Reg<regs::MRCC_GLB_CC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC0_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC0_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC1(self) -> crate::common::Reg<regs::MRCC_GLB_CC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC1_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC1_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC2(self) -> crate::common::Reg<regs::MRCC_GLB_CC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC2_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_CC2_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_ACC0(self) -> crate::common::Reg<regs::MRCC_GLB_ACC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_ACC1(self) -> crate::common::Reg<regs::MRCC_GLB_ACC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_GLB_ACC2(self) -> crate::common::Reg<regs::MRCC_GLB_ACC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_I3C0_FCLK_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_I3C0_FCLK_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_I3C0_FCLK_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_I3C0_FCLK_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER1_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER1_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER1_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER1_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER2_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER2_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER2_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER2_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER3_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER3_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER3_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER3_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER4_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER4_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CTIMER4_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CTIMER4_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_WWDT0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_WWDT0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXIO0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXIO0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXIO0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXIO0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C1_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C1_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C1_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C1_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPSPI0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPSPI0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPSPI0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPSPI0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPSPI1_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPSPI1_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPSPI1_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPSPI1_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART1_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART1_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART1_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART1_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART2_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART2_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART2_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART2_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART3_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART3_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART3_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART3_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART4_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART4_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART4_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART4_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_USB0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_USB0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_USB0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_USB0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPTMR0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPTMR0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPTMR0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPTMR0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_OSTIMER0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_OSTIMER0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_ADC_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_ADC_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_ADC_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_ADC_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP0_FUNC_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP0_FUNC_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP0_RR_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP0_RR_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP0_RR_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP0_RR_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP1_FUNC_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP1_FUNC_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP1_RR_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP1_RR_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP1_RR_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP1_RR_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP2_FUNC_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP2_FUNC_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP2_RR_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP2_RR_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CMP2_RR_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CMP2_RR_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_DAC0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_DAC0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_DAC0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_DAC0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXCAN0_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXCAN0_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXCAN0_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXCAN0_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXCAN1_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXCAN1_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_FLEXCAN1_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_FLEXCAN1_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C2_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C2_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C2_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C2_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C3_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C3_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPI2C3_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPI2C3_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART5_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART5_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_LPUART5_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_LPUART5_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_DBG_TRACE_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_DBG_TRACE_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_DBG_TRACE_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_DBG_TRACE_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CLKOUT_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_CLKOUT_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_CLKOUT_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_CLKOUT_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_SYSTICK_CLKSEL(
        self,
    ) -> crate::common::Reg<regs::MRCC_SYSTICK_CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRCC_SYSTICK_CLKDIV(
        self,
    ) -> crate::common::Reg<regs::MRCC_SYSTICK_CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADCx clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_ADC_CLKDIV(pub u32);
    impl MRCC_ADC_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_ADC_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_ADC_CLKDIV {
            MRCC_ADC_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_ADC_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_ADC_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_ADC_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_ADC_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_ADC_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ADCx clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_ADC_CLKSEL(pub u32);
    impl MRCC_ADC_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_ADC_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_ADC_CLKSEL {
            MRCC_ADC_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_ADC_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_ADC_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_ADC_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_ADC_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_ADC_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CLKOUT clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CLKOUT_CLKDIV(pub u32);
    impl MRCC_CLKOUT_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CLKOUT_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CLKOUT_CLKDIV {
            MRCC_CLKOUT_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CLKOUT_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CLKOUT_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CLKOUT_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CLKOUT_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CLKOUT_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CLKOUT clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CLKOUT_CLKSEL(pub u32);
    impl MRCC_CLKOUT_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CLKOUT_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CLKOUT_CLKSEL {
            MRCC_CLKOUT_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CLKOUT_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CLKOUT_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CLKOUT_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CLKOUT_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CLKOUT_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0_FUNC clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP0_FUNC_CLKDIV(pub u32);
    impl MRCC_CMP0_FUNC_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP0_FUNC_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP0_FUNC_CLKDIV {
            MRCC_CMP0_FUNC_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP0_FUNC_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP0_FUNC_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP0_FUNC_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP0_FUNC_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP0_FUNC_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0_RR clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP0_RR_CLKDIV(pub u32);
    impl MRCC_CMP0_RR_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP0_RR_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP0_RR_CLKDIV {
            MRCC_CMP0_RR_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP0_RR_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP0_RR_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP0_RR_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP0_RR_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP0_RR_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0_RR clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP0_RR_CLKSEL(pub u32);
    impl MRCC_CMP0_RR_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CMP0_RR_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CMP0_RR_CLKSEL {
            MRCC_CMP0_RR_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP0_RR_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP0_RR_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP0_RR_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP0_RR_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CMP0_RR_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1_FUNC clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP1_FUNC_CLKDIV(pub u32);
    impl MRCC_CMP1_FUNC_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP1_FUNC_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP1_FUNC_CLKDIV {
            MRCC_CMP1_FUNC_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP1_FUNC_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP1_FUNC_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP1_FUNC_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP1_FUNC_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP1_FUNC_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1_RR clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP1_RR_CLKDIV(pub u32);
    impl MRCC_CMP1_RR_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP1_RR_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP1_RR_CLKDIV {
            MRCC_CMP1_RR_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP1_RR_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP1_RR_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP1_RR_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP1_RR_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP1_RR_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1_RR clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP1_RR_CLKSEL(pub u32);
    impl MRCC_CMP1_RR_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CMP1_RR_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CMP1_RR_CLKSEL {
            MRCC_CMP1_RR_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP1_RR_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP1_RR_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP1_RR_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP1_RR_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CMP1_RR_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP2_FUNC clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP2_FUNC_CLKDIV(pub u32);
    impl MRCC_CMP2_FUNC_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP2_FUNC_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP2_FUNC_CLKDIV {
            MRCC_CMP2_FUNC_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP2_FUNC_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP2_FUNC_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP2_FUNC_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP2_FUNC_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP2_FUNC_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP2_RR clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP2_RR_CLKDIV(pub u32);
    impl MRCC_CMP2_RR_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CMP2_RR_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CMP2_RR_CLKDIV {
            MRCC_CMP2_RR_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP2_RR_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP2_RR_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP2_RR_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP2_RR_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CMP2_RR_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP2_RR clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CMP2_RR_CLKSEL(pub u32);
    impl MRCC_CMP2_RR_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CMP2_RR_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CMP2_RR_CLKSEL {
            MRCC_CMP2_RR_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CMP2_RR_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CMP2_RR_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CMP2_RR_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CMP2_RR_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CMP2_RR_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER0_CLKDIV(pub u32);
    impl MRCC_CTIMER0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CTIMER0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CTIMER0_CLKDIV {
            MRCC_CTIMER0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CTIMER0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER0_CLKSEL(pub u32);
    impl MRCC_CTIMER0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CTIMER0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CTIMER0_CLKSEL {
            MRCC_CTIMER0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CTIMER0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER1 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER1_CLKDIV(pub u32);
    impl MRCC_CTIMER1_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CTIMER1_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CTIMER1_CLKDIV {
            MRCC_CTIMER1_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER1_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER1_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER1_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER1_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CTIMER1_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER1 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER1_CLKSEL(pub u32);
    impl MRCC_CTIMER1_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CTIMER1_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CTIMER1_CLKSEL {
            MRCC_CTIMER1_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER1_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER1_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER1_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER1_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CTIMER1_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER2 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER2_CLKDIV(pub u32);
    impl MRCC_CTIMER2_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CTIMER2_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CTIMER2_CLKDIV {
            MRCC_CTIMER2_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER2_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER2_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER2_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER2_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CTIMER2_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER2 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER2_CLKSEL(pub u32);
    impl MRCC_CTIMER2_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CTIMER2_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CTIMER2_CLKSEL {
            MRCC_CTIMER2_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER2_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER2_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER2_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER2_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CTIMER2_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER3 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER3_CLKDIV(pub u32);
    impl MRCC_CTIMER3_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CTIMER3_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CTIMER3_CLKDIV {
            MRCC_CTIMER3_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER3_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER3_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER3_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER3_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CTIMER3_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER3 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER3_CLKSEL(pub u32);
    impl MRCC_CTIMER3_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CTIMER3_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CTIMER3_CLKSEL {
            MRCC_CTIMER3_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER3_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER3_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER3_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER3_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CTIMER3_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER4 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER4_CLKDIV(pub u32);
    impl MRCC_CTIMER4_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_CTIMER4_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_CTIMER4_CLKDIV {
            MRCC_CTIMER4_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER4_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER4_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER4_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER4_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_CTIMER4_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER4 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_CTIMER4_CLKSEL(pub u32);
    impl MRCC_CTIMER4_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_CTIMER4_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_CTIMER4_CLKSEL {
            MRCC_CTIMER4_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_CTIMER4_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_CTIMER4_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_CTIMER4_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_CTIMER4_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_CTIMER4_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_DAC0_CLKDIV(pub u32);
    impl MRCC_DAC0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_DAC0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_DAC0_CLKDIV {
            MRCC_DAC0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_DAC0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_DAC0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_DAC0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_DAC0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_DAC0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_DAC0_CLKSEL(pub u32);
    impl MRCC_DAC0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_DAC0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_DAC0_CLKSEL {
            MRCC_DAC0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_DAC0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_DAC0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_DAC0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_DAC0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_DAC0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DBG_TRACE clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_DBG_TRACE_CLKDIV(pub u32);
    impl MRCC_DBG_TRACE_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_DBG_TRACE_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_DBG_TRACE_CLKDIV {
            MRCC_DBG_TRACE_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_DBG_TRACE_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_DBG_TRACE_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_DBG_TRACE_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_DBG_TRACE_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_DBG_TRACE_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DBG_TRACE clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_DBG_TRACE_CLKSEL(pub u32);
    impl MRCC_DBG_TRACE_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for MRCC_DBG_TRACE_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_DBG_TRACE_CLKSEL {
            MRCC_DBG_TRACE_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_DBG_TRACE_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_DBG_TRACE_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_DBG_TRACE_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_DBG_TRACE_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_DBG_TRACE_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXCAN0_CLKDIV(pub u32);
    impl MRCC_FLEXCAN0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_FLEXCAN0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_FLEXCAN0_CLKDIV {
            MRCC_FLEXCAN0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXCAN0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXCAN0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXCAN0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXCAN0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_FLEXCAN0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXCAN0_CLKSEL(pub u32);
    impl MRCC_FLEXCAN0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_FLEXCAN0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_FLEXCAN0_CLKSEL {
            MRCC_FLEXCAN0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXCAN0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXCAN0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXCAN0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXCAN0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_FLEXCAN0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN1 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXCAN1_CLKDIV(pub u32);
    impl MRCC_FLEXCAN1_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_FLEXCAN1_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_FLEXCAN1_CLKDIV {
            MRCC_FLEXCAN1_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXCAN1_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXCAN1_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXCAN1_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXCAN1_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_FLEXCAN1_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN1 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXCAN1_CLKSEL(pub u32);
    impl MRCC_FLEXCAN1_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_FLEXCAN1_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_FLEXCAN1_CLKSEL {
            MRCC_FLEXCAN1_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXCAN1_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXCAN1_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXCAN1_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXCAN1_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_FLEXCAN1_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXIO0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXIO0_CLKDIV(pub u32);
    impl MRCC_FLEXIO0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_FLEXIO0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_FLEXIO0_CLKDIV {
            MRCC_FLEXIO0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXIO0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXIO0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXIO0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXIO0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_FLEXIO0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXIO0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_FLEXIO0_CLKSEL(pub u32);
    impl MRCC_FLEXIO0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_FLEXIO0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_FLEXIO0_CLKSEL {
            MRCC_FLEXIO0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_FLEXIO0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_FLEXIO0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_FLEXIO0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_FLEXIO0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_FLEXIO0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Automatic Clock Gating 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_ACC0(pub u32);
    impl MRCC_GLB_ACC0 {
        #[inline(always)]
        pub const fn INPUTMUX0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INPUTMUX0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn I3C0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CTIMER0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CTIMER1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTIMER2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CTIMER3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CTIMER4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FREQME(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FREQME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn UTICK0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTICK0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SMARTDMA0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMARTDMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AOI0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CRC0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EIM0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EIM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ERM0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn FMC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn AOI1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXIO0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn LPI2C0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPSPI0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPSPI1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn LPUART0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn LPUART1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn LPUART2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn LPUART3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn LPUART4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn USB0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn QDC0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn QDC1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn FLEXPWM0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_GLB_ACC0 {
        #[inline(always)]
        fn default() -> MRCC_GLB_ACC0 {
            MRCC_GLB_ACC0(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_ACC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_ACC0")
                .field("INPUTMUX0", &self.INPUTMUX0())
                .field("I3C0", &self.I3C0())
                .field("CTIMER0", &self.CTIMER0())
                .field("CTIMER1", &self.CTIMER1())
                .field("CTIMER2", &self.CTIMER2())
                .field("CTIMER3", &self.CTIMER3())
                .field("CTIMER4", &self.CTIMER4())
                .field("FREQME", &self.FREQME())
                .field("UTICK0", &self.UTICK0())
                .field("WWDT0", &self.WWDT0())
                .field("SMARTDMA0", &self.SMARTDMA0())
                .field("DMA0", &self.DMA0())
                .field("AOI0", &self.AOI0())
                .field("CRC0", &self.CRC0())
                .field("EIM0", &self.EIM0())
                .field("ERM0", &self.ERM0())
                .field("FMC", &self.FMC())
                .field("AOI1", &self.AOI1())
                .field("FLEXIO0", &self.FLEXIO0())
                .field("LPI2C0", &self.LPI2C0())
                .field("LPI2C1", &self.LPI2C1())
                .field("LPSPI0", &self.LPSPI0())
                .field("LPSPI1", &self.LPSPI1())
                .field("LPUART0", &self.LPUART0())
                .field("LPUART1", &self.LPUART1())
                .field("LPUART2", &self.LPUART2())
                .field("LPUART3", &self.LPUART3())
                .field("LPUART4", &self.LPUART4())
                .field("USB0", &self.USB0())
                .field("QDC0", &self.QDC0())
                .field("QDC1", &self.QDC1())
                .field("FLEXPWM0", &self.FLEXPWM0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_ACC0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_ACC0 {
                INPUTMUX0: bool,
                I3C0: bool,
                CTIMER0: bool,
                CTIMER1: bool,
                CTIMER2: bool,
                CTIMER3: bool,
                CTIMER4: bool,
                FREQME: bool,
                UTICK0: bool,
                WWDT0: bool,
                SMARTDMA0: bool,
                DMA0: bool,
                AOI0: bool,
                CRC0: bool,
                EIM0: bool,
                ERM0: bool,
                FMC: bool,
                AOI1: bool,
                FLEXIO0: bool,
                LPI2C0: bool,
                LPI2C1: bool,
                LPSPI0: bool,
                LPSPI1: bool,
                LPUART0: bool,
                LPUART1: bool,
                LPUART2: bool,
                LPUART3: bool,
                LPUART4: bool,
                USB0: bool,
                QDC0: bool,
                QDC1: bool,
                FLEXPWM0: bool,
            }
            let proxy = MRCC_GLB_ACC0 {
                INPUTMUX0: self.INPUTMUX0(),
                I3C0: self.I3C0(),
                CTIMER0: self.CTIMER0(),
                CTIMER1: self.CTIMER1(),
                CTIMER2: self.CTIMER2(),
                CTIMER3: self.CTIMER3(),
                CTIMER4: self.CTIMER4(),
                FREQME: self.FREQME(),
                UTICK0: self.UTICK0(),
                WWDT0: self.WWDT0(),
                SMARTDMA0: self.SMARTDMA0(),
                DMA0: self.DMA0(),
                AOI0: self.AOI0(),
                CRC0: self.CRC0(),
                EIM0: self.EIM0(),
                ERM0: self.ERM0(),
                FMC: self.FMC(),
                AOI1: self.AOI1(),
                FLEXIO0: self.FLEXIO0(),
                LPI2C0: self.LPI2C0(),
                LPI2C1: self.LPI2C1(),
                LPSPI0: self.LPSPI0(),
                LPSPI1: self.LPSPI1(),
                LPUART0: self.LPUART0(),
                LPUART1: self.LPUART1(),
                LPUART2: self.LPUART2(),
                LPUART3: self.LPUART3(),
                LPUART4: self.LPUART4(),
                USB0: self.USB0(),
                QDC0: self.QDC0(),
                QDC1: self.QDC1(),
                FLEXPWM0: self.FLEXPWM0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Automatic Clock Gating 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_ACC1(pub u32);
    impl MRCC_GLB_ACC1 {
        #[inline(always)]
        pub const fn FLEXPWM1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ADC0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ADC1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CMP0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CMP1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CMP2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DAC0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn OPAMP0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OPAMP1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn OPAMP2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn OPAMP3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn PORT0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PORT1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PORT2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PORT3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PORT4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SLCD0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLCD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C2(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPI2C3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPUART5(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn PKC0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SGI0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SGI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn TRNG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn UDF0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UDF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ADC2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ADC3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MRCC_GLB_ACC1 {
        #[inline(always)]
        fn default() -> MRCC_GLB_ACC1 {
            MRCC_GLB_ACC1(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_ACC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_ACC1")
                .field("FLEXPWM1", &self.FLEXPWM1())
                .field("OSTIMER0", &self.OSTIMER0())
                .field("ADC0", &self.ADC0())
                .field("ADC1", &self.ADC1())
                .field("CMP0", &self.CMP0())
                .field("CMP1", &self.CMP1())
                .field("CMP2", &self.CMP2())
                .field("DAC0", &self.DAC0())
                .field("OPAMP0", &self.OPAMP0())
                .field("OPAMP1", &self.OPAMP1())
                .field("OPAMP2", &self.OPAMP2())
                .field("OPAMP3", &self.OPAMP3())
                .field("PORT0", &self.PORT0())
                .field("PORT1", &self.PORT1())
                .field("PORT2", &self.PORT2())
                .field("PORT3", &self.PORT3())
                .field("PORT4", &self.PORT4())
                .field("SLCD0", &self.SLCD0())
                .field("FLEXCAN0", &self.FLEXCAN0())
                .field("FLEXCAN1", &self.FLEXCAN1())
                .field("LPI2C2", &self.LPI2C2())
                .field("LPI2C3", &self.LPI2C3())
                .field("LPUART5", &self.LPUART5())
                .field("PKC0", &self.PKC0())
                .field("SGI0", &self.SGI0())
                .field("TRNG0", &self.TRNG0())
                .field("UDF0", &self.UDF0())
                .field("ADC2", &self.ADC2())
                .field("ADC3", &self.ADC3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_ACC1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_ACC1 {
                FLEXPWM1: bool,
                OSTIMER0: bool,
                ADC0: bool,
                ADC1: bool,
                CMP0: bool,
                CMP1: bool,
                CMP2: bool,
                DAC0: bool,
                OPAMP0: bool,
                OPAMP1: bool,
                OPAMP2: bool,
                OPAMP3: bool,
                PORT0: bool,
                PORT1: bool,
                PORT2: bool,
                PORT3: bool,
                PORT4: bool,
                SLCD0: bool,
                FLEXCAN0: bool,
                FLEXCAN1: bool,
                LPI2C2: bool,
                LPI2C3: bool,
                LPUART5: bool,
                PKC0: bool,
                SGI0: bool,
                TRNG0: bool,
                UDF0: bool,
                ADC2: bool,
                ADC3: bool,
            }
            let proxy = MRCC_GLB_ACC1 {
                FLEXPWM1: self.FLEXPWM1(),
                OSTIMER0: self.OSTIMER0(),
                ADC0: self.ADC0(),
                ADC1: self.ADC1(),
                CMP0: self.CMP0(),
                CMP1: self.CMP1(),
                CMP2: self.CMP2(),
                DAC0: self.DAC0(),
                OPAMP0: self.OPAMP0(),
                OPAMP1: self.OPAMP1(),
                OPAMP2: self.OPAMP2(),
                OPAMP3: self.OPAMP3(),
                PORT0: self.PORT0(),
                PORT1: self.PORT1(),
                PORT2: self.PORT2(),
                PORT3: self.PORT3(),
                PORT4: self.PORT4(),
                SLCD0: self.SLCD0(),
                FLEXCAN0: self.FLEXCAN0(),
                FLEXCAN1: self.FLEXCAN1(),
                LPI2C2: self.LPI2C2(),
                LPI2C3: self.LPI2C3(),
                LPUART5: self.LPUART5(),
                PKC0: self.PKC0(),
                SGI0: self.SGI0(),
                TRNG0: self.TRNG0(),
                UDF0: self.UDF0(),
                ADC2: self.ADC2(),
                ADC3: self.ADC3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Automatic Clock Gating 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_ACC2(pub u32);
    impl MRCC_GLB_ACC2 {
        #[inline(always)]
        pub const fn RAMA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMB(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn GPIO0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GPIO1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GPIO2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GPIO3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn GPIO4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn MAU0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ROMC(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for MRCC_GLB_ACC2 {
        #[inline(always)]
        fn default() -> MRCC_GLB_ACC2 {
            MRCC_GLB_ACC2(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_ACC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_ACC2")
                .field("RAMA", &self.RAMA())
                .field("RAMB", &self.RAMB())
                .field("RAMC", &self.RAMC())
                .field("GPIO0", &self.GPIO0())
                .field("GPIO1", &self.GPIO1())
                .field("GPIO2", &self.GPIO2())
                .field("GPIO3", &self.GPIO3())
                .field("GPIO4", &self.GPIO4())
                .field("MAU0", &self.MAU0())
                .field("ROMC", &self.ROMC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_ACC2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_ACC2 {
                RAMA: bool,
                RAMB: bool,
                RAMC: bool,
                GPIO0: bool,
                GPIO1: bool,
                GPIO2: bool,
                GPIO3: bool,
                GPIO4: bool,
                MAU0: bool,
                ROMC: bool,
            }
            let proxy = MRCC_GLB_ACC2 {
                RAMA: self.RAMA(),
                RAMB: self.RAMB(),
                RAMC: self.RAMC(),
                GPIO0: self.GPIO0(),
                GPIO1: self.GPIO1(),
                GPIO2: self.GPIO2(),
                GPIO3: self.GPIO3(),
                GPIO4: self.GPIO4(),
                MAU0: self.MAU0(),
                ROMC: self.ROMC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_CC0(pub u32);
    impl MRCC_GLB_CC0 {
        #[inline(always)]
        pub const fn INPUTMUX0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INPUTMUX0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn I3C0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CTIMER0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CTIMER1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTIMER2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CTIMER3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CTIMER4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FREQME(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FREQME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn UTICK0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTICK0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SMARTDMA0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMARTDMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AOI0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CRC0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EIM0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EIM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ERM0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn FMC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn AOI1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXIO0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn LPI2C0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPSPI0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPSPI1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn LPUART0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn LPUART1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn LPUART2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn LPUART3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn LPUART4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn USB0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn QDC0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn QDC1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn FLEXPWM0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_GLB_CC0 {
        #[inline(always)]
        fn default() -> MRCC_GLB_CC0 {
            MRCC_GLB_CC0(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_CC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_CC0")
                .field("INPUTMUX0", &self.INPUTMUX0())
                .field("I3C0", &self.I3C0())
                .field("CTIMER0", &self.CTIMER0())
                .field("CTIMER1", &self.CTIMER1())
                .field("CTIMER2", &self.CTIMER2())
                .field("CTIMER3", &self.CTIMER3())
                .field("CTIMER4", &self.CTIMER4())
                .field("FREQME", &self.FREQME())
                .field("UTICK0", &self.UTICK0())
                .field("WWDT0", &self.WWDT0())
                .field("SMARTDMA0", &self.SMARTDMA0())
                .field("DMA0", &self.DMA0())
                .field("AOI0", &self.AOI0())
                .field("CRC0", &self.CRC0())
                .field("EIM0", &self.EIM0())
                .field("ERM0", &self.ERM0())
                .field("FMC", &self.FMC())
                .field("AOI1", &self.AOI1())
                .field("FLEXIO0", &self.FLEXIO0())
                .field("LPI2C0", &self.LPI2C0())
                .field("LPI2C1", &self.LPI2C1())
                .field("LPSPI0", &self.LPSPI0())
                .field("LPSPI1", &self.LPSPI1())
                .field("LPUART0", &self.LPUART0())
                .field("LPUART1", &self.LPUART1())
                .field("LPUART2", &self.LPUART2())
                .field("LPUART3", &self.LPUART3())
                .field("LPUART4", &self.LPUART4())
                .field("USB0", &self.USB0())
                .field("QDC0", &self.QDC0())
                .field("QDC1", &self.QDC1())
                .field("FLEXPWM0", &self.FLEXPWM0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_CC0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_CC0 {
                INPUTMUX0: bool,
                I3C0: bool,
                CTIMER0: bool,
                CTIMER1: bool,
                CTIMER2: bool,
                CTIMER3: bool,
                CTIMER4: bool,
                FREQME: bool,
                UTICK0: bool,
                WWDT0: bool,
                SMARTDMA0: bool,
                DMA0: bool,
                AOI0: bool,
                CRC0: bool,
                EIM0: bool,
                ERM0: bool,
                FMC: bool,
                AOI1: bool,
                FLEXIO0: bool,
                LPI2C0: bool,
                LPI2C1: bool,
                LPSPI0: bool,
                LPSPI1: bool,
                LPUART0: bool,
                LPUART1: bool,
                LPUART2: bool,
                LPUART3: bool,
                LPUART4: bool,
                USB0: bool,
                QDC0: bool,
                QDC1: bool,
                FLEXPWM0: bool,
            }
            let proxy = MRCC_GLB_CC0 {
                INPUTMUX0: self.INPUTMUX0(),
                I3C0: self.I3C0(),
                CTIMER0: self.CTIMER0(),
                CTIMER1: self.CTIMER1(),
                CTIMER2: self.CTIMER2(),
                CTIMER3: self.CTIMER3(),
                CTIMER4: self.CTIMER4(),
                FREQME: self.FREQME(),
                UTICK0: self.UTICK0(),
                WWDT0: self.WWDT0(),
                SMARTDMA0: self.SMARTDMA0(),
                DMA0: self.DMA0(),
                AOI0: self.AOI0(),
                CRC0: self.CRC0(),
                EIM0: self.EIM0(),
                ERM0: self.ERM0(),
                FMC: self.FMC(),
                AOI1: self.AOI1(),
                FLEXIO0: self.FLEXIO0(),
                LPI2C0: self.LPI2C0(),
                LPI2C1: self.LPI2C1(),
                LPSPI0: self.LPSPI0(),
                LPSPI1: self.LPSPI1(),
                LPUART0: self.LPUART0(),
                LPUART1: self.LPUART1(),
                LPUART2: self.LPUART2(),
                LPUART3: self.LPUART3(),
                LPUART4: self.LPUART4(),
                USB0: self.USB0(),
                QDC0: self.QDC0(),
                QDC1: self.QDC1(),
                FLEXPWM0: self.FLEXPWM0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_CC1(pub u32);
    impl MRCC_GLB_CC1 {
        #[inline(always)]
        pub const fn FLEXPWM1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ADC0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ADC1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CMP0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CMP1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CMP2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DAC0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn OPAMP0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OPAMP1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn OPAMP2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn OPAMP3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn PORT0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PORT1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PORT2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PORT3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PORT4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SLCD0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLCD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C2(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPI2C3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPUART5(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TDET0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDET0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PKC0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SGI0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SGI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn TRNG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn UDF0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UDF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ADC2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ADC3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MRCC_GLB_CC1 {
        #[inline(always)]
        fn default() -> MRCC_GLB_CC1 {
            MRCC_GLB_CC1(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_CC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_CC1")
                .field("FLEXPWM1", &self.FLEXPWM1())
                .field("OSTIMER0", &self.OSTIMER0())
                .field("ADC0", &self.ADC0())
                .field("ADC1", &self.ADC1())
                .field("CMP0", &self.CMP0())
                .field("CMP1", &self.CMP1())
                .field("CMP2", &self.CMP2())
                .field("DAC0", &self.DAC0())
                .field("OPAMP0", &self.OPAMP0())
                .field("OPAMP1", &self.OPAMP1())
                .field("OPAMP2", &self.OPAMP2())
                .field("OPAMP3", &self.OPAMP3())
                .field("PORT0", &self.PORT0())
                .field("PORT1", &self.PORT1())
                .field("PORT2", &self.PORT2())
                .field("PORT3", &self.PORT3())
                .field("PORT4", &self.PORT4())
                .field("SLCD0", &self.SLCD0())
                .field("FLEXCAN0", &self.FLEXCAN0())
                .field("FLEXCAN1", &self.FLEXCAN1())
                .field("LPI2C2", &self.LPI2C2())
                .field("LPI2C3", &self.LPI2C3())
                .field("LPUART5", &self.LPUART5())
                .field("TDET0", &self.TDET0())
                .field("PKC0", &self.PKC0())
                .field("SGI0", &self.SGI0())
                .field("TRNG0", &self.TRNG0())
                .field("UDF0", &self.UDF0())
                .field("ADC2", &self.ADC2())
                .field("ADC3", &self.ADC3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_CC1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_CC1 {
                FLEXPWM1: bool,
                OSTIMER0: bool,
                ADC0: bool,
                ADC1: bool,
                CMP0: bool,
                CMP1: bool,
                CMP2: bool,
                DAC0: bool,
                OPAMP0: bool,
                OPAMP1: bool,
                OPAMP2: bool,
                OPAMP3: bool,
                PORT0: bool,
                PORT1: bool,
                PORT2: bool,
                PORT3: bool,
                PORT4: bool,
                SLCD0: bool,
                FLEXCAN0: bool,
                FLEXCAN1: bool,
                LPI2C2: bool,
                LPI2C3: bool,
                LPUART5: bool,
                TDET0: bool,
                PKC0: bool,
                SGI0: bool,
                TRNG0: bool,
                UDF0: bool,
                ADC2: bool,
                ADC3: bool,
            }
            let proxy = MRCC_GLB_CC1 {
                FLEXPWM1: self.FLEXPWM1(),
                OSTIMER0: self.OSTIMER0(),
                ADC0: self.ADC0(),
                ADC1: self.ADC1(),
                CMP0: self.CMP0(),
                CMP1: self.CMP1(),
                CMP2: self.CMP2(),
                DAC0: self.DAC0(),
                OPAMP0: self.OPAMP0(),
                OPAMP1: self.OPAMP1(),
                OPAMP2: self.OPAMP2(),
                OPAMP3: self.OPAMP3(),
                PORT0: self.PORT0(),
                PORT1: self.PORT1(),
                PORT2: self.PORT2(),
                PORT3: self.PORT3(),
                PORT4: self.PORT4(),
                SLCD0: self.SLCD0(),
                FLEXCAN0: self.FLEXCAN0(),
                FLEXCAN1: self.FLEXCAN1(),
                LPI2C2: self.LPI2C2(),
                LPI2C3: self.LPI2C3(),
                LPUART5: self.LPUART5(),
                TDET0: self.TDET0(),
                PKC0: self.PKC0(),
                SGI0: self.SGI0(),
                TRNG0: self.TRNG0(),
                UDF0: self.UDF0(),
                ADC2: self.ADC2(),
                ADC3: self.ADC3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_CC2(pub u32);
    impl MRCC_GLB_CC2 {
        #[inline(always)]
        pub const fn RAMA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMB(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn GPIO0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GPIO1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GPIO2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GPIO3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn GPIO4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn MAU0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ROMC(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for MRCC_GLB_CC2 {
        #[inline(always)]
        fn default() -> MRCC_GLB_CC2 {
            MRCC_GLB_CC2(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_CC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_CC2")
                .field("RAMA", &self.RAMA())
                .field("RAMB", &self.RAMB())
                .field("RAMC", &self.RAMC())
                .field("GPIO0", &self.GPIO0())
                .field("GPIO1", &self.GPIO1())
                .field("GPIO2", &self.GPIO2())
                .field("GPIO3", &self.GPIO3())
                .field("GPIO4", &self.GPIO4())
                .field("MAU0", &self.MAU0())
                .field("ROMC", &self.ROMC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_CC2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_CC2 {
                RAMA: bool,
                RAMB: bool,
                RAMC: bool,
                GPIO0: bool,
                GPIO1: bool,
                GPIO2: bool,
                GPIO3: bool,
                GPIO4: bool,
                MAU0: bool,
                ROMC: bool,
            }
            let proxy = MRCC_GLB_CC2 {
                RAMA: self.RAMA(),
                RAMB: self.RAMB(),
                RAMC: self.RAMC(),
                GPIO0: self.GPIO0(),
                GPIO1: self.GPIO1(),
                GPIO2: self.GPIO2(),
                GPIO3: self.GPIO3(),
                GPIO4: self.GPIO4(),
                MAU0: self.MAU0(),
                ROMC: self.ROMC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_RST0(pub u32);
    impl MRCC_GLB_RST0 {
        #[inline(always)]
        pub const fn INPUTMUX0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INPUTMUX0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn I3C0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CTIMER0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CTIMER1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTIMER2(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CTIMER3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CTIMER4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FREQME(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FREQME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn UTICK0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTICK0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SMARTDMA0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SMARTDMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AOI0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CRC0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EIM0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EIM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ERM0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn AOI1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXIO0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn LPI2C0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPSPI0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPSPI1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPSPI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn LPUART0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn LPUART1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn LPUART2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn LPUART3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn LPUART4(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn USB0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn QDC0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn QDC1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn FLEXPWM0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_GLB_RST0 {
        #[inline(always)]
        fn default() -> MRCC_GLB_RST0 {
            MRCC_GLB_RST0(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_RST0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_RST0")
                .field("INPUTMUX0", &self.INPUTMUX0())
                .field("I3C0", &self.I3C0())
                .field("CTIMER0", &self.CTIMER0())
                .field("CTIMER1", &self.CTIMER1())
                .field("CTIMER2", &self.CTIMER2())
                .field("CTIMER3", &self.CTIMER3())
                .field("CTIMER4", &self.CTIMER4())
                .field("FREQME", &self.FREQME())
                .field("UTICK0", &self.UTICK0())
                .field("SMARTDMA0", &self.SMARTDMA0())
                .field("DMA0", &self.DMA0())
                .field("AOI0", &self.AOI0())
                .field("CRC0", &self.CRC0())
                .field("EIM0", &self.EIM0())
                .field("ERM0", &self.ERM0())
                .field("AOI1", &self.AOI1())
                .field("FLEXIO0", &self.FLEXIO0())
                .field("LPI2C0", &self.LPI2C0())
                .field("LPI2C1", &self.LPI2C1())
                .field("LPSPI0", &self.LPSPI0())
                .field("LPSPI1", &self.LPSPI1())
                .field("LPUART0", &self.LPUART0())
                .field("LPUART1", &self.LPUART1())
                .field("LPUART2", &self.LPUART2())
                .field("LPUART3", &self.LPUART3())
                .field("LPUART4", &self.LPUART4())
                .field("USB0", &self.USB0())
                .field("QDC0", &self.QDC0())
                .field("QDC1", &self.QDC1())
                .field("FLEXPWM0", &self.FLEXPWM0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_RST0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_RST0 {
                INPUTMUX0: bool,
                I3C0: bool,
                CTIMER0: bool,
                CTIMER1: bool,
                CTIMER2: bool,
                CTIMER3: bool,
                CTIMER4: bool,
                FREQME: bool,
                UTICK0: bool,
                SMARTDMA0: bool,
                DMA0: bool,
                AOI0: bool,
                CRC0: bool,
                EIM0: bool,
                ERM0: bool,
                AOI1: bool,
                FLEXIO0: bool,
                LPI2C0: bool,
                LPI2C1: bool,
                LPSPI0: bool,
                LPSPI1: bool,
                LPUART0: bool,
                LPUART1: bool,
                LPUART2: bool,
                LPUART3: bool,
                LPUART4: bool,
                USB0: bool,
                QDC0: bool,
                QDC1: bool,
                FLEXPWM0: bool,
            }
            let proxy = MRCC_GLB_RST0 {
                INPUTMUX0: self.INPUTMUX0(),
                I3C0: self.I3C0(),
                CTIMER0: self.CTIMER0(),
                CTIMER1: self.CTIMER1(),
                CTIMER2: self.CTIMER2(),
                CTIMER3: self.CTIMER3(),
                CTIMER4: self.CTIMER4(),
                FREQME: self.FREQME(),
                UTICK0: self.UTICK0(),
                SMARTDMA0: self.SMARTDMA0(),
                DMA0: self.DMA0(),
                AOI0: self.AOI0(),
                CRC0: self.CRC0(),
                EIM0: self.EIM0(),
                ERM0: self.ERM0(),
                AOI1: self.AOI1(),
                FLEXIO0: self.FLEXIO0(),
                LPI2C0: self.LPI2C0(),
                LPI2C1: self.LPI2C1(),
                LPSPI0: self.LPSPI0(),
                LPSPI1: self.LPSPI1(),
                LPUART0: self.LPUART0(),
                LPUART1: self.LPUART1(),
                LPUART2: self.LPUART2(),
                LPUART3: self.LPUART3(),
                LPUART4: self.LPUART4(),
                USB0: self.USB0(),
                QDC0: self.QDC0(),
                QDC1: self.QDC1(),
                FLEXPWM0: self.FLEXPWM0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_RST1(pub u32);
    impl MRCC_GLB_RST1 {
        #[inline(always)]
        pub const fn FLEXPWM1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXPWM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ADC0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ADC1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CMP1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CMP2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DAC0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn OPAMP0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OPAMP1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn OPAMP2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn OPAMP3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OPAMP3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn PORT0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PORT1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PORT2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PORT3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PORT4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SLCD0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLCD0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn LPI2C2(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LPI2C3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPI2C3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn LPUART5(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPUART5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn PKC0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn TRNG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ADC2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ADC3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MRCC_GLB_RST1 {
        #[inline(always)]
        fn default() -> MRCC_GLB_RST1 {
            MRCC_GLB_RST1(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_RST1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_RST1")
                .field("FLEXPWM1", &self.FLEXPWM1())
                .field("OSTIMER0", &self.OSTIMER0())
                .field("ADC0", &self.ADC0())
                .field("ADC1", &self.ADC1())
                .field("CMP1", &self.CMP1())
                .field("CMP2", &self.CMP2())
                .field("DAC0", &self.DAC0())
                .field("OPAMP0", &self.OPAMP0())
                .field("OPAMP1", &self.OPAMP1())
                .field("OPAMP2", &self.OPAMP2())
                .field("OPAMP3", &self.OPAMP3())
                .field("PORT0", &self.PORT0())
                .field("PORT1", &self.PORT1())
                .field("PORT2", &self.PORT2())
                .field("PORT3", &self.PORT3())
                .field("PORT4", &self.PORT4())
                .field("SLCD0", &self.SLCD0())
                .field("FLEXCAN0", &self.FLEXCAN0())
                .field("FLEXCAN1", &self.FLEXCAN1())
                .field("LPI2C2", &self.LPI2C2())
                .field("LPI2C3", &self.LPI2C3())
                .field("LPUART5", &self.LPUART5())
                .field("PKC0", &self.PKC0())
                .field("TRNG0", &self.TRNG0())
                .field("ADC2", &self.ADC2())
                .field("ADC3", &self.ADC3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_RST1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_RST1 {
                FLEXPWM1: bool,
                OSTIMER0: bool,
                ADC0: bool,
                ADC1: bool,
                CMP1: bool,
                CMP2: bool,
                DAC0: bool,
                OPAMP0: bool,
                OPAMP1: bool,
                OPAMP2: bool,
                OPAMP3: bool,
                PORT0: bool,
                PORT1: bool,
                PORT2: bool,
                PORT3: bool,
                PORT4: bool,
                SLCD0: bool,
                FLEXCAN0: bool,
                FLEXCAN1: bool,
                LPI2C2: bool,
                LPI2C3: bool,
                LPUART5: bool,
                PKC0: bool,
                TRNG0: bool,
                ADC2: bool,
                ADC3: bool,
            }
            let proxy = MRCC_GLB_RST1 {
                FLEXPWM1: self.FLEXPWM1(),
                OSTIMER0: self.OSTIMER0(),
                ADC0: self.ADC0(),
                ADC1: self.ADC1(),
                CMP1: self.CMP1(),
                CMP2: self.CMP2(),
                DAC0: self.DAC0(),
                OPAMP0: self.OPAMP0(),
                OPAMP1: self.OPAMP1(),
                OPAMP2: self.OPAMP2(),
                OPAMP3: self.OPAMP3(),
                PORT0: self.PORT0(),
                PORT1: self.PORT1(),
                PORT2: self.PORT2(),
                PORT3: self.PORT3(),
                PORT4: self.PORT4(),
                SLCD0: self.SLCD0(),
                FLEXCAN0: self.FLEXCAN0(),
                FLEXCAN1: self.FLEXCAN1(),
                LPI2C2: self.LPI2C2(),
                LPI2C3: self.LPI2C3(),
                LPUART5: self.LPUART5(),
                PKC0: self.PKC0(),
                TRNG0: self.TRNG0(),
                ADC2: self.ADC2(),
                ADC3: self.ADC3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_GLB_RST2(pub u32);
    impl MRCC_GLB_RST2 {
        #[inline(always)]
        pub const fn GPIO0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GPIO1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GPIO2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GPIO3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn GPIO4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn MAU0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MAU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for MRCC_GLB_RST2 {
        #[inline(always)]
        fn default() -> MRCC_GLB_RST2 {
            MRCC_GLB_RST2(0)
        }
    }
    impl core::fmt::Debug for MRCC_GLB_RST2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_GLB_RST2")
                .field("GPIO0", &self.GPIO0())
                .field("GPIO1", &self.GPIO1())
                .field("GPIO2", &self.GPIO2())
                .field("GPIO3", &self.GPIO3())
                .field("GPIO4", &self.GPIO4())
                .field("MAU0", &self.MAU0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_GLB_RST2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_GLB_RST2 {
                GPIO0: bool,
                GPIO1: bool,
                GPIO2: bool,
                GPIO3: bool,
                GPIO4: bool,
                MAU0: bool,
            }
            let proxy = MRCC_GLB_RST2 {
                GPIO0: self.GPIO0(),
                GPIO1: self.GPIO1(),
                GPIO2: self.GPIO2(),
                GPIO3: self.GPIO3(),
                GPIO4: self.GPIO4(),
                MAU0: self.MAU0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C0_FCLK clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_I3C0_FCLK_CLKDIV(pub u32);
    impl MRCC_I3C0_FCLK_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_I3C0_FCLK_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_I3C0_FCLK_CLKDIV {
            MRCC_I3C0_FCLK_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_I3C0_FCLK_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_I3C0_FCLK_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_I3C0_FCLK_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_I3C0_FCLK_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_I3C0_FCLK_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C0_FCLK clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_I3C0_FCLK_CLKSEL(pub u32);
    impl MRCC_I3C0_FCLK_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_I3C0_FCLK_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_I3C0_FCLK_CLKSEL {
            MRCC_I3C0_FCLK_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_I3C0_FCLK_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_I3C0_FCLK_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_I3C0_FCLK_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_I3C0_FCLK_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_I3C0_FCLK_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C0_CLKDIV(pub u32);
    impl MRCC_LPI2C0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPI2C0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPI2C0_CLKDIV {
            MRCC_LPI2C0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPI2C0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C0_CLKSEL(pub u32);
    impl MRCC_LPI2C0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPI2C0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPI2C0_CLKSEL {
            MRCC_LPI2C0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPI2C0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C1 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C1_CLKDIV(pub u32);
    impl MRCC_LPI2C1_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPI2C1_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPI2C1_CLKDIV {
            MRCC_LPI2C1_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C1_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C1_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C1_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C1_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPI2C1_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C1 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C1_CLKSEL(pub u32);
    impl MRCC_LPI2C1_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPI2C1_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPI2C1_CLKSEL {
            MRCC_LPI2C1_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C1_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C1_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C1_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C1_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPI2C1_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C2 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C2_CLKDIV(pub u32);
    impl MRCC_LPI2C2_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPI2C2_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPI2C2_CLKDIV {
            MRCC_LPI2C2_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C2_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C2_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C2_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C2_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPI2C2_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C2 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C2_CLKSEL(pub u32);
    impl MRCC_LPI2C2_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPI2C2_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPI2C2_CLKSEL {
            MRCC_LPI2C2_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C2_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C2_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C2_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C2_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPI2C2_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C3 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C3_CLKDIV(pub u32);
    impl MRCC_LPI2C3_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPI2C3_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPI2C3_CLKDIV {
            MRCC_LPI2C3_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C3_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C3_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C3_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C3_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPI2C3_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPI2C3 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPI2C3_CLKSEL(pub u32);
    impl MRCC_LPI2C3_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPI2C3_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPI2C3_CLKSEL {
            MRCC_LPI2C3_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPI2C3_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPI2C3_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPI2C3_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPI2C3_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPI2C3_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPSPI0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPSPI0_CLKDIV(pub u32);
    impl MRCC_LPSPI0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPSPI0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPSPI0_CLKDIV {
            MRCC_LPSPI0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPSPI0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPSPI0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPSPI0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPSPI0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPSPI0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPSPI0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPSPI0_CLKSEL(pub u32);
    impl MRCC_LPSPI0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPSPI0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPSPI0_CLKSEL {
            MRCC_LPSPI0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPSPI0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPSPI0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPSPI0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPSPI0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPSPI0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPSPI1 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPSPI1_CLKDIV(pub u32);
    impl MRCC_LPSPI1_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPSPI1_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPSPI1_CLKDIV {
            MRCC_LPSPI1_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPSPI1_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPSPI1_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPSPI1_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPSPI1_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPSPI1_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPSPI1 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPSPI1_CLKSEL(pub u32);
    impl MRCC_LPSPI1_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPSPI1_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPSPI1_CLKSEL {
            MRCC_LPSPI1_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPSPI1_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPSPI1_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPSPI1_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPSPI1_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPSPI1_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPTMR0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPTMR0_CLKDIV(pub u32);
    impl MRCC_LPTMR0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPTMR0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPTMR0_CLKDIV {
            MRCC_LPTMR0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPTMR0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPTMR0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPTMR0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPTMR0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPTMR0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPTMR0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPTMR0_CLKSEL(pub u32);
    impl MRCC_LPTMR0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPTMR0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPTMR0_CLKSEL {
            MRCC_LPTMR0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPTMR0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPTMR0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPTMR0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPTMR0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPTMR0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART0_CLKDIV(pub u32);
    impl MRCC_LPUART0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART0_CLKDIV {
            MRCC_LPUART0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART0_CLKSEL(pub u32);
    impl MRCC_LPUART0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART0_CLKSEL {
            MRCC_LPUART0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART1 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART1_CLKDIV(pub u32);
    impl MRCC_LPUART1_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART1_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART1_CLKDIV {
            MRCC_LPUART1_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART1_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART1_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART1_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART1_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART1_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART1 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART1_CLKSEL(pub u32);
    impl MRCC_LPUART1_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART1_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART1_CLKSEL {
            MRCC_LPUART1_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART1_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART1_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART1_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART1_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART1_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART2 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART2_CLKDIV(pub u32);
    impl MRCC_LPUART2_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART2_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART2_CLKDIV {
            MRCC_LPUART2_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART2_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART2_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART2_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART2_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART2_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART2 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART2_CLKSEL(pub u32);
    impl MRCC_LPUART2_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART2_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART2_CLKSEL {
            MRCC_LPUART2_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART2_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART2_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART2_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART2_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART2_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART3 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART3_CLKDIV(pub u32);
    impl MRCC_LPUART3_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART3_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART3_CLKDIV {
            MRCC_LPUART3_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART3_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART3_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART3_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART3_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART3_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART3 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART3_CLKSEL(pub u32);
    impl MRCC_LPUART3_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART3_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART3_CLKSEL {
            MRCC_LPUART3_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART3_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART3_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART3_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART3_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART3_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART4 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART4_CLKDIV(pub u32);
    impl MRCC_LPUART4_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART4_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART4_CLKDIV {
            MRCC_LPUART4_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART4_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART4_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART4_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART4_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART4_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART4 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART4_CLKSEL(pub u32);
    impl MRCC_LPUART4_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART4_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART4_CLKSEL {
            MRCC_LPUART4_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART4_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART4_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART4_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART4_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART4_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART5 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART5_CLKDIV(pub u32);
    impl MRCC_LPUART5_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_LPUART5_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_LPUART5_CLKDIV {
            MRCC_LPUART5_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART5_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART5_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART5_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART5_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_LPUART5_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPUART5 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_LPUART5_CLKSEL(pub u32);
    impl MRCC_LPUART5_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for MRCC_LPUART5_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_LPUART5_CLKSEL {
            MRCC_LPUART5_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_LPUART5_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_LPUART5_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_LPUART5_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_LPUART5_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_LPUART5_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "OSTIMER0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_OSTIMER0_CLKSEL(pub u32);
    impl MRCC_OSTIMER0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for MRCC_OSTIMER0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_OSTIMER0_CLKSEL {
            MRCC_OSTIMER0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_OSTIMER0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_OSTIMER0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_OSTIMER0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_OSTIMER0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_OSTIMER0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SYSTICK clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_SYSTICK_CLKDIV(pub u32);
    impl MRCC_SYSTICK_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_SYSTICK_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_SYSTICK_CLKDIV {
            MRCC_SYSTICK_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_SYSTICK_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_SYSTICK_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_SYSTICK_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_SYSTICK_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_SYSTICK_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SYSTICK clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_SYSTICK_CLKSEL(pub u32);
    impl MRCC_SYSTICK_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for MRCC_SYSTICK_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_SYSTICK_CLKSEL {
            MRCC_SYSTICK_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_SYSTICK_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_SYSTICK_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_SYSTICK_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_SYSTICK_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_SYSTICK_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_USB0_CLKDIV(pub u32);
    impl MRCC_USB0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_USB0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_USB0_CLKDIV {
            MRCC_USB0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_USB0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_USB0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_USB0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_USB0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_USB0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB0 clock selection control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_USB0_CLKSEL(pub u32);
    impl MRCC_USB0_CLKSEL {
        #[inline(always)]
        pub const fn MUX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for MRCC_USB0_CLKSEL {
        #[inline(always)]
        fn default() -> MRCC_USB0_CLKSEL {
            MRCC_USB0_CLKSEL(0)
        }
    }
    impl core::fmt::Debug for MRCC_USB0_CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_USB0_CLKSEL")
                .field("MUX", &self.MUX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_USB0_CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_USB0_CLKSEL {
                MUX: u8,
            }
            let proxy = MRCC_USB0_CLKSEL { MUX: self.MUX() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "WWDT0 clock divider control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRCC_WWDT0_CLKDIV(pub u32);
    impl MRCC_WWDT0_CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MRCC_WWDT0_CLKDIV {
        #[inline(always)]
        fn default() -> MRCC_WWDT0_CLKDIV {
            MRCC_WWDT0_CLKDIV(0)
        }
    }
    impl core::fmt::Debug for MRCC_WWDT0_CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRCC_WWDT0_CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRCC_WWDT0_CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRCC_WWDT0_CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MRCC_WWDT0_CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
