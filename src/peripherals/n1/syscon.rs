#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSCON {
    ptr: *mut u8,
}
unsafe impl Send for SYSCON {}
unsafe impl Sync for SYSCON {}
impl SYSCON {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn AHBMATPRIO(self) -> crate::common::Reg<regs::AHBMATPRIO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CPU0STCKCAL(self) -> crate::common::Reg<regs::CPU0STCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn CPU0NSTCKCAL(self) -> crate::common::Reg<regs::CPU0NSTCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn NMISRC(self) -> crate::common::Reg<regs::NMISRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRL0(self) -> crate::common::Reg<regs::PRESETCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRL1(self) -> crate::common::Reg<regs::PRESETCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRL2(self) -> crate::common::Reg<regs::PRESETCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRL3(self) -> crate::common::Reg<regs::PRESETCTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRLSET(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn PRESETCTRLCLR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRL0(self) -> crate::common::Reg<regs::AHBCLKCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRL1(self) -> crate::common::Reg<regs::AHBCLKCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRL2(self) -> crate::common::Reg<regs::AHBCLKCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRL3(self) -> crate::common::Reg<regs::AHBCLKCTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRLSET(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKCTRLCLR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SYSTICKCLKSEL0(
        self,
    ) -> crate::common::Reg<regs::SYSTICKCLKSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn TRACECLKSEL(self) -> crate::common::Reg<regs::TRACECLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMERCLKSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CTIMERCLKSEL, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKOUTSEL(self) -> crate::common::Reg<regs::CLKOUTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC0CLKSEL(self) -> crate::common::Reg<regs::ADC0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[inline(always)]
    pub const fn FCCLKSEL(self, n: usize) -> crate::common::Reg<regs::FCCLKSEL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SYSTICKCLKDIV(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SYSTICKCLKDIV, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TRACECLKDIV(self) -> crate::common::Reg<regs::TRACECLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[inline(always)]
    pub const fn SLOWCLKDIV(self) -> crate::common::Reg<regs::SLOWCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKDIV(self) -> crate::common::Reg<regs::AHBCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKOUTDIV(self) -> crate::common::Reg<regs::CLKOUTDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[inline(always)]
    pub const fn FROHFDIV(self) -> crate::common::Reg<regs::FROHFDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[inline(always)]
    pub const fn WDT0CLKDIV(self) -> crate::common::Reg<regs::WDT0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[inline(always)]
    pub const fn ADC0CLKDIV(self) -> crate::common::Reg<regs::ADC0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[inline(always)]
    pub const fn PLLCLKDIV(self) -> crate::common::Reg<regs::PLLCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMERCLKDIV(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn PLL1CLK0DIV(self) -> crate::common::Reg<regs::PLL1CLK0DIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[inline(always)]
    pub const fn PLL1CLK1DIV(self) -> crate::common::Reg<regs::PLL1CLK1DIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[inline(always)]
    pub const fn UTICKCLKDIV(self) -> crate::common::Reg<regs::UTICKCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKOUT_FRGCTRL(
        self,
    ) -> crate::common::Reg<regs::CLKOUT_FRGCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKUNLOCK(self) -> crate::common::Reg<regs::CLKUNLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
    #[inline(always)]
    pub const fn NVM_CTRL(self) -> crate::common::Reg<regs::NVM_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn ROMCR(self) -> crate::common::Reg<regs::ROMCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn SMARTDMAINT(self) -> crate::common::Reg<regs::SMARTDMAINT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC1CLKSEL(self) -> crate::common::Reg<regs::ADC1CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC1CLKDIV(self) -> crate::common::Reg<regs::ADC1CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[inline(always)]
    pub const fn RAM_INTERLEAVE(
        self,
    ) -> crate::common::Reg<regs::RAM_INTERLEAVE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[inline(always)]
    pub const fn PLLCLKDIVSEL(self) -> crate::common::Reg<regs::PLLCLKDIVSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[inline(always)]
    pub const fn I3C0FCLKSEL(self) -> crate::common::Reg<regs::I3C0FCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[inline(always)]
    pub const fn I3C0FCLKDIV(self) -> crate::common::Reg<regs::I3C0FCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[inline(always)]
    pub const fn MICFILFCLKSEL(self) -> crate::common::Reg<regs::MICFILFCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[inline(always)]
    pub const fn MICFILFCLKDIV(self) -> crate::common::Reg<regs::MICFILFCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXIOCLKSEL(self) -> crate::common::Reg<regs::FLEXIOCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXIOCLKDIV(self) -> crate::common::Reg<regs::FLEXIOCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCAN0CLKSEL(
        self,
    ) -> crate::common::Reg<regs::FLEXCAN0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCAN0CLKDIV(
        self,
    ) -> crate::common::Reg<regs::FLEXCAN0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCAN1CLKSEL(
        self,
    ) -> crate::common::Reg<regs::FLEXCAN1CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCAN1CLKDIV(
        self,
    ) -> crate::common::Reg<regs::FLEXCAN1CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05acusize) as _) }
    }
    #[inline(always)]
    pub const fn EWM0CLKSEL(self) -> crate::common::Reg<regs::EWM0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[inline(always)]
    pub const fn WDT1CLKSEL(self) -> crate::common::Reg<regs::WDT1CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d8usize) as _) }
    }
    #[inline(always)]
    pub const fn WDT1CLKDIV(self) -> crate::common::Reg<regs::WDT1CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05dcusize) as _) }
    }
    #[inline(always)]
    pub const fn OSTIMERCLKSEL(self) -> crate::common::Reg<regs::OSTIMERCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP0FCLKSEL(self) -> crate::common::Reg<regs::CMP0FCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP0FCLKDIV(self) -> crate::common::Reg<regs::CMP0FCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP0RRCLKSEL(self) -> crate::common::Reg<regs::CMP0RRCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP0RRCLKDIV(self) -> crate::common::Reg<regs::CMP0RRCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05fcusize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1FCLKSEL(self) -> crate::common::Reg<regs::CMP1FCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1FCLKDIV(self) -> crate::common::Reg<regs::CMP1FCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1RRCLKSEL(self) -> crate::common::Reg<regs::CMP1RRCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1RRCLKDIV(self) -> crate::common::Reg<regs::CMP1RRCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[inline(always)]
    pub const fn CPUSTAT(self) -> crate::common::Reg<regs::CPUSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
    #[inline(always)]
    pub const fn LPCAC_CTRL(self) -> crate::common::Reg<regs::LPCAC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0824usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMMCLKDIV(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0850usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn UTICKCLKSEL(self) -> crate::common::Reg<regs::UTICKCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0878usize) as _) }
    }
    #[inline(always)]
    pub const fn SAI0CLKSEL(self) -> crate::common::Reg<regs::SAI0CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize) as _) }
    }
    #[inline(always)]
    pub const fn SAI1CLKSEL(self) -> crate::common::Reg<regs::SAI1CLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0884usize) as _) }
    }
    #[inline(always)]
    pub const fn SAI0CLKDIV(self) -> crate::common::Reg<regs::SAI0CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0888usize) as _) }
    }
    #[inline(always)]
    pub const fn SAI1CLKDIV(self) -> crate::common::Reg<regs::SAI1CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x088cusize) as _) }
    }
    #[inline(always)]
    pub const fn KEY_RETAIN_CTRL(
        self,
    ) -> crate::common::Reg<regs::KEY_RETAIN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0950usize) as _) }
    }
    #[inline(always)]
    pub const fn REF_CLK_CTRL(self) -> crate::common::Reg<regs::REF_CLK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0960usize) as _) }
    }
    #[inline(always)]
    pub const fn REF_CLK_CTRL_SET(
        self,
    ) -> crate::common::Reg<regs::REF_CLK_CTRL_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0964usize) as _) }
    }
    #[inline(always)]
    pub const fn REF_CLK_CTRL_CLR(
        self,
    ) -> crate::common::Reg<regs::REF_CLK_CTRL_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0968usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CTRL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x096cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_ASSET_PROT(
        self,
    ) -> crate::common::Reg<regs::ELS_ASSET_PROT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0974usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_LOCK_CTRL(self) -> crate::common::Reg<regs::ELS_LOCK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0978usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_LOCK_CTRL_DP(
        self,
    ) -> crate::common::Reg<regs::ELS_LOCK_CTRL_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x097cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_OTP_LC_STATE(
        self,
    ) -> crate::common::Reg<regs::ELS_OTP_LC_STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0980usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_OTP_LC_STATE_DP(
        self,
    ) -> crate::common::Reg<regs::ELS_OTP_LC_STATE_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0984usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_TEMPORAL_STATE(
        self,
    ) -> crate::common::Reg<regs::ELS_TEMPORAL_STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0988usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_KDF_MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x098cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_CFG0(self) -> crate::common::Reg<regs::ELS_AS_CFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09d0usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_CFG1(self) -> crate::common::Reg<regs::ELS_AS_CFG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09d4usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_CFG2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09d8usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_CFG3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09dcusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_ST0(self) -> crate::common::Reg<regs::ELS_AS_ST0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09e0usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_ST1(self) -> crate::common::Reg<regs::ELS_AS_ST1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09e4usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_BOOT_LOG0(
        self,
    ) -> crate::common::Reg<regs::ELS_AS_BOOT_LOG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09e8usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_BOOT_LOG1(
        self,
    ) -> crate::common::Reg<regs::ELS_AS_BOOT_LOG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09ecusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_BOOT_LOG2(
        self,
    ) -> crate::common::Reg<regs::ELS_AS_BOOT_LOG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09f0usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_BOOT_LOG3(
        self,
    ) -> crate::common::Reg<regs::ELS_AS_BOOT_LOG3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09f4usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_FLAG0(self) -> crate::common::Reg<regs::ELS_AS_FLAG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09f8usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_AS_FLAG1(self) -> crate::common::Reg<regs::ELS_AS_FLAG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09fcusize) as _) }
    }
    #[inline(always)]
    pub const fn CLOCK_CTRL(self) -> crate::common::Reg<regs::CLOCK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a18usize) as _) }
    }
    #[inline(always)]
    pub const fn I3C1FCLKSEL(self) -> crate::common::Reg<regs::I3C1FCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b30usize) as _) }
    }
    #[inline(always)]
    pub const fn I3C1FCLKDIV(self) -> crate::common::Reg<regs::I3C1FCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b40usize) as _) }
    }
    #[inline(always)]
    pub const fn GRAY_CODE_LSB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b60usize) as _) }
    }
    #[inline(always)]
    pub const fn GRAY_CODE_MSB(self) -> crate::common::Reg<regs::GRAY_CODE_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b64usize) as _) }
    }
    #[inline(always)]
    pub const fn BINARY_CODE_LSB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b68usize) as _) }
    }
    #[inline(always)]
    pub const fn BINARY_CODE_MSB(
        self,
    ) -> crate::common::Reg<regs::BINARY_CODE_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b6cusize) as _) }
    }
    #[inline(always)]
    pub const fn AUTOCLKGATEOVERRIDE(
        self,
    ) -> crate::common::Reg<regs::AUTOCLKGATEOVERRIDE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e04usize) as _) }
    }
    #[inline(always)]
    pub const fn AUTOCLKGATEOVERRIDEC(
        self,
    ) -> crate::common::Reg<regs::AUTOCLKGATEOVERRIDEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e2cusize) as _) }
    }
    #[inline(always)]
    pub const fn PWM0SUBCTL(self) -> crate::common::Reg<regs::PWM0SUBCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e38usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM1SUBCTL(self) -> crate::common::Reg<regs::PWM1SUBCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e3cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMERGLOBALSTARTEN(
        self,
    ) -> crate::common::Reg<regs::CTIMERGLOBALSTARTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e40usize) as _) }
    }
    #[inline(always)]
    pub const fn ECC_ENABLE_CTRL(
        self,
    ) -> crate::common::Reg<regs::ECC_ENABLE_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e44usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_LOCK_EN(self) -> crate::common::Reg<regs::DEBUG_LOCK_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa0usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_FEATURES(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_FEATURES_DP(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa8usize) as _) }
    }
    #[inline(always)]
    pub const fn SWD_ACCESS_CPU(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fb4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_AUTH_BEACON(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn JTAG_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[inline(always)]
    pub const fn DEVICE_TYPE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEVICE_ID0(self) -> crate::common::Reg<regs::DEVICE_ID0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[inline(always)]
    pub const fn DIEID(self) -> crate::common::Reg<regs::DIEID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC0 Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC0CLKDIV(pub u32);
    impl ADC0CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for ADC0CLKDIV {
        #[inline(always)]
        fn default() -> ADC0CLKDIV {
            ADC0CLKDIV(0)
        }
    }
    impl core::fmt::Debug for ADC0CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC0CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC0CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC0CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = ADC0CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ADC0 Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC0CLKSEL(pub u32);
    impl ADC0CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for ADC0CLKSEL {
        #[inline(always)]
        fn default() -> ADC0CLKSEL {
            ADC0CLKSEL(0)
        }
    }
    impl core::fmt::Debug for ADC0CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC0CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC0CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC0CLKSEL {
                SEL: u8,
            }
            let proxy = ADC0CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ADC1 Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC1CLKDIV(pub u32);
    impl ADC1CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for ADC1CLKDIV {
        #[inline(always)]
        fn default() -> ADC1CLKDIV {
            ADC1CLKDIV(0)
        }
    }
    impl core::fmt::Debug for ADC1CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC1CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC1CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC1CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = ADC1CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ADC1 Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC1CLKSEL(pub u32);
    impl ADC1CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for ADC1CLKSEL {
        #[inline(always)]
        fn default() -> ADC1CLKSEL {
            ADC1CLKSEL(0)
        }
    }
    impl core::fmt::Debug for ADC1CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC1CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC1CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC1CLKSEL {
                SEL: u8,
            }
            let proxy = ADC1CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKCTRL0(pub u32);
    impl AHBCLKCTRL0 {
        #[inline(always)]
        pub const fn ROM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMB_CTRL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMC_CTRL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RAMD_CTRL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMD_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RAME_CTRL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAME_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FMU(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FMU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FMC(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FMC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn MUX(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MUX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PORT0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PORT1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PORT2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PORT3(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn PORT4(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn GPIO0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn GPIO1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn GPIO2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn GPIO3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn GPIO4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PINT(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PINT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn CRC(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn WWDT1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WWDT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for AHBCLKCTRL0 {
        #[inline(always)]
        fn default() -> AHBCLKCTRL0 {
            AHBCLKCTRL0(0)
        }
    }
    impl core::fmt::Debug for AHBCLKCTRL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKCTRL0")
                .field("ROM", &self.ROM())
                .field("RAMB_CTRL", &self.RAMB_CTRL())
                .field("RAMC_CTRL", &self.RAMC_CTRL())
                .field("RAMD_CTRL", &self.RAMD_CTRL())
                .field("RAME_CTRL", &self.RAME_CTRL())
                .field("FMU", &self.FMU())
                .field("FMC", &self.FMC())
                .field("MUX", &self.MUX())
                .field("PORT0", &self.PORT0())
                .field("PORT1", &self.PORT1())
                .field("PORT2", &self.PORT2())
                .field("PORT3", &self.PORT3())
                .field("PORT4", &self.PORT4())
                .field("GPIO0", &self.GPIO0())
                .field("GPIO1", &self.GPIO1())
                .field("GPIO2", &self.GPIO2())
                .field("GPIO3", &self.GPIO3())
                .field("GPIO4", &self.GPIO4())
                .field("PINT", &self.PINT())
                .field("DMA0", &self.DMA0())
                .field("CRC", &self.CRC())
                .field("WWDT0", &self.WWDT0())
                .field("WWDT1", &self.WWDT1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKCTRL0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBCLKCTRL0 {
                ROM: bool,
                RAMB_CTRL: bool,
                RAMC_CTRL: bool,
                RAMD_CTRL: bool,
                RAME_CTRL: bool,
                FMU: bool,
                FMC: bool,
                MUX: bool,
                PORT0: bool,
                PORT1: bool,
                PORT2: bool,
                PORT3: bool,
                PORT4: bool,
                GPIO0: bool,
                GPIO1: bool,
                GPIO2: bool,
                GPIO3: bool,
                GPIO4: bool,
                PINT: bool,
                DMA0: bool,
                CRC: bool,
                WWDT0: bool,
                WWDT1: bool,
            }
            let proxy = AHBCLKCTRL0 {
                ROM: self.ROM(),
                RAMB_CTRL: self.RAMB_CTRL(),
                RAMC_CTRL: self.RAMC_CTRL(),
                RAMD_CTRL: self.RAMD_CTRL(),
                RAME_CTRL: self.RAME_CTRL(),
                FMU: self.FMU(),
                FMC: self.FMC(),
                MUX: self.MUX(),
                PORT0: self.PORT0(),
                PORT1: self.PORT1(),
                PORT2: self.PORT2(),
                PORT3: self.PORT3(),
                PORT4: self.PORT4(),
                GPIO0: self.GPIO0(),
                GPIO1: self.GPIO1(),
                GPIO2: self.GPIO2(),
                GPIO3: self.GPIO3(),
                GPIO4: self.GPIO4(),
                PINT: self.PINT(),
                DMA0: self.DMA0(),
                CRC: self.CRC(),
                WWDT0: self.WWDT0(),
                WWDT1: self.WWDT1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKCTRL1(pub u32);
    impl AHBCLKCTRL1 {
        #[inline(always)]
        pub const fn MRT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MRT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ADC0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ADC1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RTC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn UTICK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTICK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FC0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FC1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FC2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn FC3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FC4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn FC5(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FC6(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FC7(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn MICFIL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MICFIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TIMER2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TIMER0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn TIMER1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn PKC_RAM(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC_RAM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn SmartDMA(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SmartDMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AHBCLKCTRL1 {
        #[inline(always)]
        fn default() -> AHBCLKCTRL1 {
            AHBCLKCTRL1(0)
        }
    }
    impl core::fmt::Debug for AHBCLKCTRL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKCTRL1")
                .field("MRT", &self.MRT())
                .field("OSTIMER", &self.OSTIMER())
                .field("ADC0", &self.ADC0())
                .field("ADC1", &self.ADC1())
                .field("RTC", &self.RTC())
                .field("UTICK", &self.UTICK())
                .field("FC0", &self.FC0())
                .field("FC1", &self.FC1())
                .field("FC2", &self.FC2())
                .field("FC3", &self.FC3())
                .field("FC4", &self.FC4())
                .field("FC5", &self.FC5())
                .field("FC6", &self.FC6())
                .field("FC7", &self.FC7())
                .field("MICFIL", &self.MICFIL())
                .field("TIMER2", &self.TIMER2())
                .field("TIMER0", &self.TIMER0())
                .field("TIMER1", &self.TIMER1())
                .field("PKC_RAM", &self.PKC_RAM())
                .field("SmartDMA", &self.SmartDMA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKCTRL1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBCLKCTRL1 {
                MRT: bool,
                OSTIMER: bool,
                ADC0: bool,
                ADC1: bool,
                RTC: bool,
                UTICK: bool,
                FC0: bool,
                FC1: bool,
                FC2: bool,
                FC3: bool,
                FC4: bool,
                FC5: bool,
                FC6: bool,
                FC7: bool,
                MICFIL: bool,
                TIMER2: bool,
                TIMER0: bool,
                TIMER1: bool,
                PKC_RAM: bool,
                SmartDMA: bool,
            }
            let proxy = AHBCLKCTRL1 {
                MRT: self.MRT(),
                OSTIMER: self.OSTIMER(),
                ADC0: self.ADC0(),
                ADC1: self.ADC1(),
                RTC: self.RTC(),
                UTICK: self.UTICK(),
                FC0: self.FC0(),
                FC1: self.FC1(),
                FC2: self.FC2(),
                FC3: self.FC3(),
                FC4: self.FC4(),
                FC5: self.FC5(),
                FC6: self.FC6(),
                FC7: self.FC7(),
                MICFIL: self.MICFIL(),
                TIMER2: self.TIMER2(),
                TIMER0: self.TIMER0(),
                TIMER1: self.TIMER1(),
                PKC_RAM: self.PKC_RAM(),
                SmartDMA: self.SmartDMA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKCTRL2(pub u32);
    impl AHBCLKCTRL2 {
        #[inline(always)]
        pub const fn DMA1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FLEXIO(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXIO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn SAI0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAI0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SAI1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAI1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn TRO(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FREQME(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FREQME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn USB_HS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_HS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn USB_HS_PHY(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_HS_PHY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ELS(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ELS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn TIMER3(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TIMER4(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn PUF(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PKC(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SCG(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn GDET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for AHBCLKCTRL2 {
        #[inline(always)]
        fn default() -> AHBCLKCTRL2 {
            AHBCLKCTRL2(0)
        }
    }
    impl core::fmt::Debug for AHBCLKCTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKCTRL2")
                .field("DMA1", &self.DMA1())
                .field("FLEXIO", &self.FLEXIO())
                .field("SAI0", &self.SAI0())
                .field("SAI1", &self.SAI1())
                .field("TRO", &self.TRO())
                .field("FREQME", &self.FREQME())
                .field("FLEXCAN0", &self.FLEXCAN0())
                .field("FLEXCAN1", &self.FLEXCAN1())
                .field("USB_HS", &self.USB_HS())
                .field("USB_HS_PHY", &self.USB_HS_PHY())
                .field("ELS", &self.ELS())
                .field("TIMER3", &self.TIMER3())
                .field("TIMER4", &self.TIMER4())
                .field("PUF", &self.PUF())
                .field("PKC", &self.PKC())
                .field("SCG", &self.SCG())
                .field("GDET", &self.GDET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKCTRL2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBCLKCTRL2 {
                DMA1: bool,
                FLEXIO: bool,
                SAI0: bool,
                SAI1: bool,
                TRO: bool,
                FREQME: bool,
                FLEXCAN0: bool,
                FLEXCAN1: bool,
                USB_HS: bool,
                USB_HS_PHY: bool,
                ELS: bool,
                TIMER3: bool,
                TIMER4: bool,
                PUF: bool,
                PKC: bool,
                SCG: bool,
                GDET: bool,
            }
            let proxy = AHBCLKCTRL2 {
                DMA1: self.DMA1(),
                FLEXIO: self.FLEXIO(),
                SAI0: self.SAI0(),
                SAI1: self.SAI1(),
                TRO: self.TRO(),
                FREQME: self.FREQME(),
                FLEXCAN0: self.FLEXCAN0(),
                FLEXCAN1: self.FLEXCAN1(),
                USB_HS: self.USB_HS(),
                USB_HS_PHY: self.USB_HS_PHY(),
                ELS: self.ELS(),
                TIMER3: self.TIMER3(),
                TIMER4: self.TIMER4(),
                PUF: self.PUF(),
                PKC: self.PKC(),
                SCG: self.SCG(),
                GDET: self.GDET(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Clock Control 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKCTRL3(pub u32);
    impl AHBCLKCTRL3 {
        #[inline(always)]
        pub const fn I3C0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn I3C1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn QDC0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn QDC1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PWM0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PWM1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn EVTG(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVTG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VREF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VREF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EWM(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EWM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn EIM(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn ERM(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn INTM(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for AHBCLKCTRL3 {
        #[inline(always)]
        fn default() -> AHBCLKCTRL3 {
            AHBCLKCTRL3(0)
        }
    }
    impl core::fmt::Debug for AHBCLKCTRL3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKCTRL3")
                .field("I3C0", &self.I3C0())
                .field("I3C1", &self.I3C1())
                .field("QDC0", &self.QDC0())
                .field("QDC1", &self.QDC1())
                .field("PWM0", &self.PWM0())
                .field("PWM1", &self.PWM1())
                .field("EVTG", &self.EVTG())
                .field("VREF", &self.VREF())
                .field("EWM", &self.EWM())
                .field("EIM", &self.EIM())
                .field("ERM", &self.ERM())
                .field("INTM", &self.INTM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKCTRL3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBCLKCTRL3 {
                I3C0: bool,
                I3C1: bool,
                QDC0: bool,
                QDC1: bool,
                PWM0: bool,
                PWM1: bool,
                EVTG: bool,
                VREF: bool,
                EWM: bool,
                EIM: bool,
                ERM: bool,
                INTM: bool,
            }
            let proxy = AHBCLKCTRL3 {
                I3C0: self.I3C0(),
                I3C1: self.I3C1(),
                QDC0: self.QDC0(),
                QDC1: self.QDC1(),
                PWM0: self.PWM0(),
                PWM1: self.PWM1(),
                EVTG: self.EVTG(),
                VREF: self.VREF(),
                EWM: self.EWM(),
                EIM: self.EIM(),
                ERM: self.ERM(),
                INTM: self.INTM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "System Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKDIV(pub u32);
    impl AHBCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for AHBCLKDIV {
        #[inline(always)]
        fn default() -> AHBCLKDIV {
            AHBCLKDIV(0)
        }
    }
    impl core::fmt::Debug for AHBCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKDIV")
                .field("DIV", &self.DIV())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBCLKDIV {
                DIV: u8,
                UNSTAB: bool,
            }
            let proxy = AHBCLKDIV {
                DIV: self.DIV(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Matrix Priority Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBMATPRIO(pub u32);
    impl AHBMATPRIO {
        #[inline(always)]
        pub const fn PRI_CPU0_CBUS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRI_CPU0_CBUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PRI_CPU0_SBUS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRI_CPU0_SBUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn DMA1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PRI_PKC_ELS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRI_PKC_ELS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn PRI_USB_HS(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRI_USB_HS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
    }
    impl Default for AHBMATPRIO {
        #[inline(always)]
        fn default() -> AHBMATPRIO {
            AHBMATPRIO(0)
        }
    }
    impl core::fmt::Debug for AHBMATPRIO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBMATPRIO")
                .field("PRI_CPU0_CBUS", &self.PRI_CPU0_CBUS())
                .field("PRI_CPU0_SBUS", &self.PRI_CPU0_SBUS())
                .field("DMA0", &self.DMA0())
                .field("DMA1", &self.DMA1())
                .field("PRI_PKC_ELS", &self.PRI_PKC_ELS())
                .field("PRI_USB_HS", &self.PRI_USB_HS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBMATPRIO {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBMATPRIO {
                PRI_CPU0_CBUS: u8,
                PRI_CPU0_SBUS: u8,
                DMA0: u8,
                DMA1: u8,
                PRI_PKC_ELS: u8,
                PRI_USB_HS: u8,
            }
            let proxy = AHBMATPRIO {
                PRI_CPU0_CBUS: self.PRI_CPU0_CBUS(),
                PRI_CPU0_SBUS: self.PRI_CPU0_SBUS(),
                DMA0: self.DMA0(),
                DMA1: self.DMA1(),
                PRI_PKC_ELS: self.PRI_PKC_ELS(),
                PRI_USB_HS: self.PRI_USB_HS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Automatic Clock Gating"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AUTOCLKGATEOVERRIDE(pub u32);
    impl AUTOCLKGATEOVERRIDE {
        #[inline(always)]
        pub const fn RAMB_CTRL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMC_CTRL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RAMD_CTRL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMD_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RAME_CTRL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAME_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for AUTOCLKGATEOVERRIDE {
        #[inline(always)]
        fn default() -> AUTOCLKGATEOVERRIDE {
            AUTOCLKGATEOVERRIDE(0)
        }
    }
    impl core::fmt::Debug for AUTOCLKGATEOVERRIDE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AUTOCLKGATEOVERRIDE")
                .field("RAMB_CTRL", &self.RAMB_CTRL())
                .field("RAMC_CTRL", &self.RAMC_CTRL())
                .field("RAMD_CTRL", &self.RAMD_CTRL())
                .field("RAME_CTRL", &self.RAME_CTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AUTOCLKGATEOVERRIDE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AUTOCLKGATEOVERRIDE {
                RAMB_CTRL: bool,
                RAMC_CTRL: bool,
                RAMD_CTRL: bool,
                RAME_CTRL: bool,
            }
            let proxy = AUTOCLKGATEOVERRIDE {
                RAMB_CTRL: self.RAMB_CTRL(),
                RAMC_CTRL: self.RAMC_CTRL(),
                RAMD_CTRL: self.RAMD_CTRL(),
                RAME_CTRL: self.RAME_CTRL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Automatic Clock Gating C"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AUTOCLKGATEOVERRIDEC(pub u32);
    impl AUTOCLKGATEOVERRIDEC {
        #[inline(always)]
        pub const fn RAMX(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn RAMA(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AUTOCLKGATEOVERRIDEC {
        #[inline(always)]
        fn default() -> AUTOCLKGATEOVERRIDEC {
            AUTOCLKGATEOVERRIDEC(0)
        }
    }
    impl core::fmt::Debug for AUTOCLKGATEOVERRIDEC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AUTOCLKGATEOVERRIDEC")
                .field("RAMX", &self.RAMX())
                .field("RAMA", &self.RAMA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AUTOCLKGATEOVERRIDEC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AUTOCLKGATEOVERRIDEC {
                RAMX: bool,
                RAMA: bool,
            }
            let proxy = AUTOCLKGATEOVERRIDEC {
                RAMX: self.RAMX(),
                RAMA: self.RAMA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BINARY_CODE_MSB(pub u32);
    impl BINARY_CODE_MSB {
        #[inline(always)]
        pub const fn code_bin_41_32(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_code_bin_41_32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for BINARY_CODE_MSB {
        #[inline(always)]
        fn default() -> BINARY_CODE_MSB {
            BINARY_CODE_MSB(0)
        }
    }
    impl core::fmt::Debug for BINARY_CODE_MSB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BINARY_CODE_MSB")
                .field("code_bin_41_32", &self.code_bin_41_32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BINARY_CODE_MSB {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BINARY_CODE_MSB {
                code_bin_41_32: u16,
            }
            let proxy = BINARY_CODE_MSB {
                code_bin_41_32: self.code_bin_41_32(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CLKOUT Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKOUTDIV(pub u32);
    impl CLKOUTDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for CLKOUTDIV {
        #[inline(always)]
        fn default() -> CLKOUTDIV {
            CLKOUTDIV(0)
        }
    }
    impl core::fmt::Debug for CLKOUTDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKOUTDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKOUTDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKOUTDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CLKOUTDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CLKOUT Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKOUTSEL(pub u32);
    impl CLKOUTSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for CLKOUTSEL {
        #[inline(always)]
        fn default() -> CLKOUTSEL {
            CLKOUTSEL(0)
        }
    }
    impl core::fmt::Debug for CLKOUTSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKOUTSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKOUTSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKOUTSEL {
                SEL: u8,
            }
            let proxy = CLKOUTSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CLKOUT FRG Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKOUT_FRGCTRL(pub u32);
    impl CLKOUT_FRGCTRL {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn MULT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MULT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CLKOUT_FRGCTRL {
        #[inline(always)]
        fn default() -> CLKOUT_FRGCTRL {
            CLKOUT_FRGCTRL(0)
        }
    }
    impl core::fmt::Debug for CLKOUT_FRGCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKOUT_FRGCTRL")
                .field("DIV", &self.DIV())
                .field("MULT", &self.MULT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKOUT_FRGCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKOUT_FRGCTRL {
                DIV: u8,
                MULT: u8,
            }
            let proxy = CLKOUT_FRGCTRL {
                DIV: self.DIV(),
                MULT: self.MULT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock Configuration Unlock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKUNLOCK(pub u32);
    impl CLKUNLOCK {
        #[inline(always)]
        pub const fn UNLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CLKUNLOCK {
        #[inline(always)]
        fn default() -> CLKUNLOCK {
            CLKUNLOCK(0)
        }
    }
    impl core::fmt::Debug for CLKUNLOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKUNLOCK")
                .field("UNLOCK", &self.UNLOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKUNLOCK {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKUNLOCK {
                UNLOCK: bool,
            }
            let proxy = CLKUNLOCK {
                UNLOCK: self.UNLOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLOCK_CTRL(pub u32);
    impl CLOCK_CTRL {
        #[inline(always)]
        pub const fn CLKIN_ENA_FM_USBH_LPT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKIN_ENA_FM_USBH_LPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRO1MHZ_ENA(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO1MHZ_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FRO12MHZ_ENA(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO12MHZ_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FRO_HF_ENA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO_HF_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CLKIN_ENA(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKIN_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FRO1MHZ_CLK_ENA(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO1MHZ_CLK_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for CLOCK_CTRL {
        #[inline(always)]
        fn default() -> CLOCK_CTRL {
            CLOCK_CTRL(0)
        }
    }
    impl core::fmt::Debug for CLOCK_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLOCK_CTRL")
                .field("CLKIN_ENA_FM_USBH_LPT", &self.CLKIN_ENA_FM_USBH_LPT())
                .field("FRO1MHZ_ENA", &self.FRO1MHZ_ENA())
                .field("FRO12MHZ_ENA", &self.FRO12MHZ_ENA())
                .field("FRO_HF_ENA", &self.FRO_HF_ENA())
                .field("CLKIN_ENA", &self.CLKIN_ENA())
                .field("FRO1MHZ_CLK_ENA", &self.FRO1MHZ_CLK_ENA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLOCK_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLOCK_CTRL {
                CLKIN_ENA_FM_USBH_LPT: bool,
                FRO1MHZ_ENA: bool,
                FRO12MHZ_ENA: bool,
                FRO_HF_ENA: bool,
                CLKIN_ENA: bool,
                FRO1MHZ_CLK_ENA: bool,
            }
            let proxy = CLOCK_CTRL {
                CLKIN_ENA_FM_USBH_LPT: self.CLKIN_ENA_FM_USBH_LPT(),
                FRO1MHZ_ENA: self.FRO1MHZ_ENA(),
                FRO12MHZ_ENA: self.FRO12MHZ_ENA(),
                FRO_HF_ENA: self.FRO_HF_ENA(),
                CLKIN_ENA: self.CLKIN_ENA(),
                FRO1MHZ_CLK_ENA: self.FRO1MHZ_CLK_ENA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0 Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP0FCLKDIV(pub u32);
    impl CMP0FCLKDIV {
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
    impl Default for CMP0FCLKDIV {
        #[inline(always)]
        fn default() -> CMP0FCLKDIV {
            CMP0FCLKDIV(0)
        }
    }
    impl core::fmt::Debug for CMP0FCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP0FCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP0FCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP0FCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CMP0FCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0 Function Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP0FCLKSEL(pub u32);
    impl CMP0FCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for CMP0FCLKSEL {
        #[inline(always)]
        fn default() -> CMP0FCLKSEL {
            CMP0FCLKSEL(0)
        }
    }
    impl core::fmt::Debug for CMP0FCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP0FCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP0FCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP0FCLKSEL {
                SEL: u8,
            }
            let proxy = CMP0FCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0 Round Robin Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP0RRCLKDIV(pub u32);
    impl CMP0RRCLKDIV {
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
    impl Default for CMP0RRCLKDIV {
        #[inline(always)]
        fn default() -> CMP0RRCLKDIV {
            CMP0RRCLKDIV(0)
        }
    }
    impl core::fmt::Debug for CMP0RRCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP0RRCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP0RRCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP0RRCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CMP0RRCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0 Round Robin Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP0RRCLKSEL(pub u32);
    impl CMP0RRCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for CMP0RRCLKSEL {
        #[inline(always)]
        fn default() -> CMP0RRCLKSEL {
            CMP0RRCLKSEL(0)
        }
    }
    impl core::fmt::Debug for CMP0RRCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP0RRCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP0RRCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP0RRCLKSEL {
                SEL: u8,
            }
            let proxy = CMP0RRCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1 Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP1FCLKDIV(pub u32);
    impl CMP1FCLKDIV {
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
    impl Default for CMP1FCLKDIV {
        #[inline(always)]
        fn default() -> CMP1FCLKDIV {
            CMP1FCLKDIV(0)
        }
    }
    impl core::fmt::Debug for CMP1FCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP1FCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP1FCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP1FCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CMP1FCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1 Function Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP1FCLKSEL(pub u32);
    impl CMP1FCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for CMP1FCLKSEL {
        #[inline(always)]
        fn default() -> CMP1FCLKSEL {
            CMP1FCLKSEL(0)
        }
    }
    impl core::fmt::Debug for CMP1FCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP1FCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP1FCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP1FCLKSEL {
                SEL: u8,
            }
            let proxy = CMP1FCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1 Round Robin Clock Division"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP1RRCLKDIV(pub u32);
    impl CMP1RRCLKDIV {
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
    impl Default for CMP1RRCLKDIV {
        #[inline(always)]
        fn default() -> CMP1RRCLKDIV {
            CMP1RRCLKDIV(0)
        }
    }
    impl core::fmt::Debug for CMP1RRCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP1RRCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP1RRCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP1RRCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CMP1RRCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1 Round Robin Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP1RRCLKSEL(pub u32);
    impl CMP1RRCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for CMP1RRCLKSEL {
        #[inline(always)]
        fn default() -> CMP1RRCLKSEL {
            CMP1RRCLKSEL(0)
        }
    }
    impl core::fmt::Debug for CMP1RRCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP1RRCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP1RRCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP1RRCLKSEL {
                SEL: u8,
            }
            let proxy = CMP1RRCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPU0NSTCKCAL(pub u32);
    impl CPU0NSTCKCAL {
        #[inline(always)]
        pub const fn TENMS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TENMS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn SKEW(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SKEW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn NOREF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NOREF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for CPU0NSTCKCAL {
        #[inline(always)]
        fn default() -> CPU0NSTCKCAL {
            CPU0NSTCKCAL(0)
        }
    }
    impl core::fmt::Debug for CPU0NSTCKCAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPU0NSTCKCAL")
                .field("TENMS", &self.TENMS())
                .field("SKEW", &self.SKEW())
                .field("NOREF", &self.NOREF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPU0NSTCKCAL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CPU0NSTCKCAL {
                TENMS: u32,
                SKEW: bool,
                NOREF: bool,
            }
            let proxy = CPU0NSTCKCAL {
                TENMS: self.TENMS(),
                SKEW: self.SKEW(),
                NOREF: self.NOREF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Secure CPU0 System Tick Calibration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPU0STCKCAL(pub u32);
    impl CPU0STCKCAL {
        #[inline(always)]
        pub const fn TENMS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TENMS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn SKEW(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SKEW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn NOREF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NOREF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for CPU0STCKCAL {
        #[inline(always)]
        fn default() -> CPU0STCKCAL {
            CPU0STCKCAL(0)
        }
    }
    impl core::fmt::Debug for CPU0STCKCAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPU0STCKCAL")
                .field("TENMS", &self.TENMS())
                .field("SKEW", &self.SKEW())
                .field("NOREF", &self.NOREF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPU0STCKCAL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CPU0STCKCAL {
                TENMS: u32,
                SKEW: bool,
                NOREF: bool,
            }
            let proxy = CPU0STCKCAL {
                TENMS: self.TENMS(),
                SKEW: self.SKEW(),
                NOREF: self.NOREF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CPU Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPUSTAT(pub u32);
    impl CPUSTAT {
        #[inline(always)]
        pub const fn CPU0SLEEPING(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPU0SLEEPING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0LOCKUP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPU0LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CPUSTAT {
        #[inline(always)]
        fn default() -> CPUSTAT {
            CPUSTAT(0)
        }
    }
    impl core::fmt::Debug for CPUSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPUSTAT")
                .field("CPU0SLEEPING", &self.CPU0SLEEPING())
                .field("CPU0LOCKUP", &self.CPU0LOCKUP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPUSTAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CPUSTAT {
                CPU0SLEEPING: bool,
                CPU0LOCKUP: bool,
            }
            let proxy = CPUSTAT {
                CPU0SLEEPING: self.CPU0SLEEPING(),
                CPU0LOCKUP: self.CPU0LOCKUP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTimer Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMERCLKDIV(pub u32);
    impl CTIMERCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for CTIMERCLKDIV {
        #[inline(always)]
        fn default() -> CTIMERCLKDIV {
            CTIMERCLKDIV(0)
        }
    }
    impl core::fmt::Debug for CTIMERCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMERCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMERCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMERCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = CTIMERCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMERCLKSEL(pub u32);
    impl CTIMERCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for CTIMERCLKSEL {
        #[inline(always)]
        fn default() -> CTIMERCLKSEL {
            CTIMERCLKSEL(0)
        }
    }
    impl core::fmt::Debug for CTIMERCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMERCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMERCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMERCLKSEL {
                SEL: u8,
            }
            let proxy = CTIMERCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CTIMER Global Start Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMERGLOBALSTARTEN(pub u32);
    impl CTIMERGLOBALSTARTEN {
        #[inline(always)]
        pub const fn CTIMER0_CLK_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER0_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CTIMER1_CLK_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER1_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CTIMER2_CLK_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER2_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CTIMER3_CLK_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER3_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTIMER4_CLK_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER4_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CTIMERGLOBALSTARTEN {
        #[inline(always)]
        fn default() -> CTIMERGLOBALSTARTEN {
            CTIMERGLOBALSTARTEN(0)
        }
    }
    impl core::fmt::Debug for CTIMERGLOBALSTARTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMERGLOBALSTARTEN")
                .field("CTIMER0_CLK_EN", &self.CTIMER0_CLK_EN())
                .field("CTIMER1_CLK_EN", &self.CTIMER1_CLK_EN())
                .field("CTIMER2_CLK_EN", &self.CTIMER2_CLK_EN())
                .field("CTIMER3_CLK_EN", &self.CTIMER3_CLK_EN())
                .field("CTIMER4_CLK_EN", &self.CTIMER4_CLK_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMERGLOBALSTARTEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMERGLOBALSTARTEN {
                CTIMER0_CLK_EN: bool,
                CTIMER1_CLK_EN: bool,
                CTIMER2_CLK_EN: bool,
                CTIMER3_CLK_EN: bool,
                CTIMER4_CLK_EN: bool,
            }
            let proxy = CTIMERGLOBALSTARTEN {
                CTIMER0_CLK_EN: self.CTIMER0_CLK_EN(),
                CTIMER1_CLK_EN: self.CTIMER1_CLK_EN(),
                CTIMER2_CLK_EN: self.CTIMER2_CLK_EN(),
                CTIMER3_CLK_EN: self.CTIMER3_CLK_EN(),
                CTIMER4_CLK_EN: self.CTIMER4_CLK_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Cortex Debug Features Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_FEATURES(pub u32);
    impl DEBUG_FEATURES {
        #[inline(always)]
        pub const fn CPU0_DBGEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_DBGEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0_NIDEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_NIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn CPU0_SPIDEN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SPIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn CPU0_SPNIDEN(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SPNIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for DEBUG_FEATURES {
        #[inline(always)]
        fn default() -> DEBUG_FEATURES {
            DEBUG_FEATURES(0)
        }
    }
    impl core::fmt::Debug for DEBUG_FEATURES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_FEATURES")
                .field("CPU0_DBGEN", &self.CPU0_DBGEN())
                .field("CPU0_NIDEN", &self.CPU0_NIDEN())
                .field("CPU0_SPIDEN", &self.CPU0_SPIDEN())
                .field("CPU0_SPNIDEN", &self.CPU0_SPNIDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_FEATURES {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG_FEATURES {
                CPU0_DBGEN: u8,
                CPU0_NIDEN: u8,
                CPU0_SPIDEN: u8,
                CPU0_SPNIDEN: u8,
            }
            let proxy = DEBUG_FEATURES {
                CPU0_DBGEN: self.CPU0_DBGEN(),
                CPU0_NIDEN: self.CPU0_NIDEN(),
                CPU0_SPIDEN: self.CPU0_SPIDEN(),
                CPU0_SPNIDEN: self.CPU0_SPNIDEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_FEATURES_DP(pub u32);
    impl DEBUG_FEATURES_DP {
        #[inline(always)]
        pub const fn CPU0_DBGEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_DBGEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0_NIDEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_NIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn CPU0_SPIDEN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SPIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn CPU0_SPNIDEN(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SPNIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for DEBUG_FEATURES_DP {
        #[inline(always)]
        fn default() -> DEBUG_FEATURES_DP {
            DEBUG_FEATURES_DP(0)
        }
    }
    impl core::fmt::Debug for DEBUG_FEATURES_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_FEATURES_DP")
                .field("CPU0_DBGEN", &self.CPU0_DBGEN())
                .field("CPU0_NIDEN", &self.CPU0_NIDEN())
                .field("CPU0_SPIDEN", &self.CPU0_SPIDEN())
                .field("CPU0_SPNIDEN", &self.CPU0_SPNIDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_FEATURES_DP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG_FEATURES_DP {
                CPU0_DBGEN: u8,
                CPU0_NIDEN: u8,
                CPU0_SPIDEN: u8,
                CPU0_SPNIDEN: u8,
            }
            let proxy = DEBUG_FEATURES_DP {
                CPU0_DBGEN: self.CPU0_DBGEN(),
                CPU0_NIDEN: self.CPU0_NIDEN(),
                CPU0_SPIDEN: self.CPU0_SPIDEN(),
                CPU0_SPNIDEN: self.CPU0_SPNIDEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control Write Access to Security"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_LOCK_EN(pub u32);
    impl DEBUG_LOCK_EN {
        #[inline(always)]
        pub const fn LOCK_ALL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOCK_ALL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for DEBUG_LOCK_EN {
        #[inline(always)]
        fn default() -> DEBUG_LOCK_EN {
            DEBUG_LOCK_EN(0)
        }
    }
    impl core::fmt::Debug for DEBUG_LOCK_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_LOCK_EN")
                .field("LOCK_ALL", &self.LOCK_ALL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_LOCK_EN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG_LOCK_EN {
                LOCK_ALL: u8,
            }
            let proxy = DEBUG_LOCK_EN {
                LOCK_ALL: self.LOCK_ALL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Device ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEVICE_ID0(pub u32);
    impl DEVICE_ID0 {
        #[inline(always)]
        pub const fn ROM_REV_MINOR(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ROM_REV_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for DEVICE_ID0 {
        #[inline(always)]
        fn default() -> DEVICE_ID0 {
            DEVICE_ID0(0)
        }
    }
    impl core::fmt::Debug for DEVICE_ID0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEVICE_ID0")
                .field("ROM_REV_MINOR", &self.ROM_REV_MINOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEVICE_ID0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEVICE_ID0 {
                ROM_REV_MINOR: u8,
            }
            let proxy = DEVICE_ID0 {
                ROM_REV_MINOR: self.ROM_REV_MINOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Chip Revision ID and Number"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DIEID(pub u32);
    impl DIEID {
        #[inline(always)]
        pub const fn MINOR_REVISION(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR_REVISION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn MAJOR_REVISION(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR_REVISION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn MCO_NUM_IN_DIE_ID(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MCO_NUM_IN_DIE_ID(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
        }
    }
    impl Default for DIEID {
        #[inline(always)]
        fn default() -> DIEID {
            DIEID(0)
        }
    }
    impl core::fmt::Debug for DIEID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DIEID")
                .field("MINOR_REVISION", &self.MINOR_REVISION())
                .field("MAJOR_REVISION", &self.MAJOR_REVISION())
                .field("MCO_NUM_IN_DIE_ID", &self.MCO_NUM_IN_DIE_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DIEID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DIEID {
                MINOR_REVISION: u8,
                MAJOR_REVISION: u8,
                MCO_NUM_IN_DIE_ID: u32,
            }
            let proxy = DIEID {
                MINOR_REVISION: self.MINOR_REVISION(),
                MAJOR_REVISION: self.MAJOR_REVISION(),
                MCO_NUM_IN_DIE_ID: self.MCO_NUM_IN_DIE_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RAM ECC Enable Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ECC_ENABLE_CTRL(pub u32);
    impl ECC_ENABLE_CTRL {
        #[inline(always)]
        pub const fn RAMA_ECC_ENABLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA_ECC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RAMB_RAMX_ECC_ENABLE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_RAMX_ECC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMD_RAMC_ECC_ENABLE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMD_RAMC_ECC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for ECC_ENABLE_CTRL {
        #[inline(always)]
        fn default() -> ECC_ENABLE_CTRL {
            ECC_ENABLE_CTRL(0)
        }
    }
    impl core::fmt::Debug for ECC_ENABLE_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ECC_ENABLE_CTRL")
                .field("RAMA_ECC_ENABLE", &self.RAMA_ECC_ENABLE())
                .field("RAMB_RAMX_ECC_ENABLE", &self.RAMB_RAMX_ECC_ENABLE())
                .field("RAMD_RAMC_ECC_ENABLE", &self.RAMD_RAMC_ECC_ENABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ECC_ENABLE_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ECC_ENABLE_CTRL {
                RAMA_ECC_ENABLE: bool,
                RAMB_RAMX_ECC_ENABLE: bool,
                RAMD_RAMC_ECC_ENABLE: bool,
            }
            let proxy = ECC_ENABLE_CTRL {
                RAMA_ECC_ENABLE: self.RAMA_ECC_ENABLE(),
                RAMB_RAMX_ECC_ENABLE: self.RAMB_RAMX_ECC_ENABLE(),
                RAMD_RAMC_ECC_ENABLE: self.RAMD_RAMC_ECC_ENABLE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS Asset Protection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_ASSET_PROT(pub u32);
    impl ELS_ASSET_PROT {
        #[inline(always)]
        pub const fn ASSET_PROTECTION(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ASSET_PROTECTION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for ELS_ASSET_PROT {
        #[inline(always)]
        fn default() -> ELS_ASSET_PROT {
            ELS_ASSET_PROT(0)
        }
    }
    impl core::fmt::Debug for ELS_ASSET_PROT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_ASSET_PROT")
                .field("ASSET_PROTECTION", &self.ASSET_PROTECTION())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_ASSET_PROT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_ASSET_PROT {
                ASSET_PROTECTION: u8,
            }
            let proxy = ELS_ASSET_PROT {
                ASSET_PROTECTION: self.ASSET_PROTECTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Boot state captured during boot: Main ROM log"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_BOOT_LOG0(pub u32);
    impl ELS_AS_BOOT_LOG0 {
        #[inline(always)]
        pub const fn BOOT_IMAGE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BOOT_IMAGE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn CMAC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ECDSA(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ECDSA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn OFF_CHIP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OFF_CHIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ON_CHIP(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ON_CHIP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CDI_CSR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CDI_CSR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CDI_DICE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CDI_DICE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TRUSTZONE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRUSTZONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEBUG_AUTH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEBUG_AUTH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ITRC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ITRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DIG_GDET(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIG_GDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ANA_GDET(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ANA_GDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn DEEP_PD(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEEP_PD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn LOW_POWER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOW_POWER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn ISP(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ELS_AS_BOOT_LOG0 {
        #[inline(always)]
        fn default() -> ELS_AS_BOOT_LOG0 {
            ELS_AS_BOOT_LOG0(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_BOOT_LOG0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_BOOT_LOG0")
                .field("BOOT_IMAGE", &self.BOOT_IMAGE())
                .field("CMAC", &self.CMAC())
                .field("ECDSA", &self.ECDSA())
                .field("OFF_CHIP", &self.OFF_CHIP())
                .field("ON_CHIP", &self.ON_CHIP())
                .field("CDI_CSR", &self.CDI_CSR())
                .field("CDI_DICE", &self.CDI_DICE())
                .field("TRUSTZONE", &self.TRUSTZONE())
                .field("DEBUG_AUTH", &self.DEBUG_AUTH())
                .field("ITRC", &self.ITRC())
                .field("DIG_GDET", &self.DIG_GDET())
                .field("ANA_GDET", &self.ANA_GDET())
                .field("DEEP_PD", &self.DEEP_PD())
                .field("LOW_POWER", &self.LOW_POWER())
                .field("ISP", &self.ISP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_BOOT_LOG0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_BOOT_LOG0 {
                BOOT_IMAGE: u8,
                CMAC: bool,
                ECDSA: bool,
                OFF_CHIP: bool,
                ON_CHIP: bool,
                CDI_CSR: bool,
                CDI_DICE: bool,
                TRUSTZONE: bool,
                DEBUG_AUTH: bool,
                ITRC: bool,
                DIG_GDET: bool,
                ANA_GDET: bool,
                DEEP_PD: bool,
                LOW_POWER: u8,
                ISP: bool,
            }
            let proxy = ELS_AS_BOOT_LOG0 {
                BOOT_IMAGE: self.BOOT_IMAGE(),
                CMAC: self.CMAC(),
                ECDSA: self.ECDSA(),
                OFF_CHIP: self.OFF_CHIP(),
                ON_CHIP: self.ON_CHIP(),
                CDI_CSR: self.CDI_CSR(),
                CDI_DICE: self.CDI_DICE(),
                TRUSTZONE: self.TRUSTZONE(),
                DEBUG_AUTH: self.DEBUG_AUTH(),
                ITRC: self.ITRC(),
                DIG_GDET: self.DIG_GDET(),
                ANA_GDET: self.ANA_GDET(),
                DEEP_PD: self.DEEP_PD(),
                LOW_POWER: self.LOW_POWER(),
                ISP: self.ISP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Boot state captured during boot: Library log"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_BOOT_LOG1(pub u32);
    impl ELS_AS_BOOT_LOG1 {
        #[inline(always)]
        pub const fn RoTK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RoTK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn FIPS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 2usize)) | (((val as u32) & 0xff) << 2usize);
        }
        #[inline(always)]
        pub const fn SB3(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SB3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for ELS_AS_BOOT_LOG1 {
        #[inline(always)]
        fn default() -> ELS_AS_BOOT_LOG1 {
            ELS_AS_BOOT_LOG1(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_BOOT_LOG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_BOOT_LOG1")
                .field("RoTK", &self.RoTK())
                .field("FIPS", &self.FIPS())
                .field("SB3", &self.SB3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_BOOT_LOG1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_BOOT_LOG1 {
                RoTK: u8,
                FIPS: u8,
                SB3: u8,
            }
            let proxy = ELS_AS_BOOT_LOG1 {
                RoTK: self.RoTK(),
                FIPS: self.FIPS(),
                SB3: self.SB3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Boot state captured during boot: Hardware status signals log"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_BOOT_LOG2(pub u32);
    impl ELS_AS_BOOT_LOG2 {
        #[inline(always)]
        pub const fn CMC_SRS0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMC_SRS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn VBAT_STATUS0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBAT_STATUS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn CMC_SRS1(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CMC_SRS1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
        #[inline(always)]
        pub const fn VBAT_STATUS1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBAT_STATUS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[inline(always)]
        pub const fn CMC_SRS2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMC_SRS2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for ELS_AS_BOOT_LOG2 {
        #[inline(always)]
        fn default() -> ELS_AS_BOOT_LOG2 {
            ELS_AS_BOOT_LOG2(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_BOOT_LOG2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_BOOT_LOG2")
                .field("CMC_SRS0", &self.CMC_SRS0())
                .field("VBAT_STATUS0", &self.VBAT_STATUS0())
                .field("CMC_SRS1", &self.CMC_SRS1())
                .field("VBAT_STATUS1", &self.VBAT_STATUS1())
                .field("CMC_SRS2", &self.CMC_SRS2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_BOOT_LOG2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_BOOT_LOG2 {
                CMC_SRS0: u8,
                VBAT_STATUS0: u8,
                CMC_SRS1: u16,
                VBAT_STATUS1: u8,
                CMC_SRS2: u8,
            }
            let proxy = ELS_AS_BOOT_LOG2 {
                CMC_SRS0: self.CMC_SRS0(),
                VBAT_STATUS0: self.VBAT_STATUS0(),
                CMC_SRS1: self.CMC_SRS1(),
                VBAT_STATUS1: self.VBAT_STATUS1(),
                CMC_SRS2: self.CMC_SRS2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Boot state captured during boot: Security log"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_BOOT_LOG3(pub u32);
    impl ELS_AS_BOOT_LOG3 {
        #[inline(always)]
        pub const fn ERR_AUTH_FAIL_COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERR_AUTH_FAIL_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn ERR_ITRC_COUNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERR_ITRC_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for ELS_AS_BOOT_LOG3 {
        #[inline(always)]
        fn default() -> ELS_AS_BOOT_LOG3 {
            ELS_AS_BOOT_LOG3(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_BOOT_LOG3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_BOOT_LOG3")
                .field("ERR_AUTH_FAIL_COUNT", &self.ERR_AUTH_FAIL_COUNT())
                .field("ERR_ITRC_COUNT", &self.ERR_ITRC_COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_BOOT_LOG3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_BOOT_LOG3 {
                ERR_AUTH_FAIL_COUNT: u8,
                ERR_ITRC_COUNT: u8,
            }
            let proxy = ELS_AS_BOOT_LOG3 {
                ERR_AUTH_FAIL_COUNT: self.ERR_AUTH_FAIL_COUNT(),
                ERR_ITRC_COUNT: self.ERR_ITRC_COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_CFG0(pub u32);
    impl ELS_AS_CFG0 {
        #[inline(always)]
        pub const fn CFG_LC_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CFG_LC_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_CORE_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_CORE_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_CORE_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_CORE_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CFG_WDT0_ENABLED(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_WDT0_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CFG_CWDT0_ENABLED(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_CWDT0_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn CFG_ELS_GDET_ENABLED(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_ELS_GDET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn CFG_ANA_GDET_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_ANA_GDET_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn CFG_ANA_GDET_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_ANA_GDET_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn CFG_TAMPER_DET_ENABLED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_TAMPER_DET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_VSYS_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_VSYS_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_VDDIO_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_VDDIO_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_VSYS_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_VSYS_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn CFG_LVD_VDDIO_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LVD_VDDIO_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn CFG_WDT1_ENABLED(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_WDT1_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn CFG_CWDT1_ENABLED(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_CWDT1_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn CFG_TEMPTAMPER_DET_ENABLED(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_TEMPTAMPER_DET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn CFG_VOLTAMPER_DET_ENABLED(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_VOLTAMPER_DET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn CFG_LHTTAMPER_DET_ENABLED(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_LHTTAMPER_DET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn CFG_CLKTAMPER_DET_ENABLED(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_CLKTAMPER_DET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn CFG_QK_DISABLE_ENROLL(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_QK_DISABLE_ENROLL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn CFG_QK_DISABLE_WRAP(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_QK_DISABLE_WRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for ELS_AS_CFG0 {
        #[inline(always)]
        fn default() -> ELS_AS_CFG0 {
            ELS_AS_CFG0(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_CFG0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_CFG0")
                .field("CFG_LC_STATE", &self.CFG_LC_STATE())
                .field(
                    "CFG_LVD_CORE_RESET_ENABLED",
                    &self.CFG_LVD_CORE_RESET_ENABLED(),
                )
                .field("CFG_LVD_CORE_IRQ_ENABLED", &self.CFG_LVD_CORE_IRQ_ENABLED())
                .field("CFG_WDT0_ENABLED", &self.CFG_WDT0_ENABLED())
                .field("CFG_CWDT0_ENABLED", &self.CFG_CWDT0_ENABLED())
                .field("CFG_ELS_GDET_ENABLED", &self.CFG_ELS_GDET_ENABLED())
                .field(
                    "CFG_ANA_GDET_RESET_ENABLED",
                    &self.CFG_ANA_GDET_RESET_ENABLED(),
                )
                .field("CFG_ANA_GDET_IRQ_ENABLED", &self.CFG_ANA_GDET_IRQ_ENABLED())
                .field("CFG_TAMPER_DET_ENABLED", &self.CFG_TAMPER_DET_ENABLED())
                .field(
                    "CFG_LVD_VSYS_RESET_ENABLED",
                    &self.CFG_LVD_VSYS_RESET_ENABLED(),
                )
                .field(
                    "CFG_LVD_VDDIO_RESET_ENABLED",
                    &self.CFG_LVD_VDDIO_RESET_ENABLED(),
                )
                .field("CFG_LVD_VSYS_IRQ_ENABLED", &self.CFG_LVD_VSYS_IRQ_ENABLED())
                .field(
                    "CFG_LVD_VDDIO_IRQ_ENABLED",
                    &self.CFG_LVD_VDDIO_IRQ_ENABLED(),
                )
                .field("CFG_WDT1_ENABLED", &self.CFG_WDT1_ENABLED())
                .field("CFG_CWDT1_ENABLED", &self.CFG_CWDT1_ENABLED())
                .field(
                    "CFG_TEMPTAMPER_DET_ENABLED",
                    &self.CFG_TEMPTAMPER_DET_ENABLED(),
                )
                .field(
                    "CFG_VOLTAMPER_DET_ENABLED",
                    &self.CFG_VOLTAMPER_DET_ENABLED(),
                )
                .field(
                    "CFG_LHTTAMPER_DET_ENABLED",
                    &self.CFG_LHTTAMPER_DET_ENABLED(),
                )
                .field(
                    "CFG_CLKTAMPER_DET_ENABLED",
                    &self.CFG_CLKTAMPER_DET_ENABLED(),
                )
                .field("CFG_QK_DISABLE_ENROLL", &self.CFG_QK_DISABLE_ENROLL())
                .field("CFG_QK_DISABLE_WRAP", &self.CFG_QK_DISABLE_WRAP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_CFG0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_CFG0 {
                CFG_LC_STATE: u8,
                CFG_LVD_CORE_RESET_ENABLED: bool,
                CFG_LVD_CORE_IRQ_ENABLED: bool,
                CFG_WDT0_ENABLED: bool,
                CFG_CWDT0_ENABLED: bool,
                CFG_ELS_GDET_ENABLED: bool,
                CFG_ANA_GDET_RESET_ENABLED: bool,
                CFG_ANA_GDET_IRQ_ENABLED: bool,
                CFG_TAMPER_DET_ENABLED: bool,
                CFG_LVD_VSYS_RESET_ENABLED: bool,
                CFG_LVD_VDDIO_RESET_ENABLED: bool,
                CFG_LVD_VSYS_IRQ_ENABLED: bool,
                CFG_LVD_VDDIO_IRQ_ENABLED: bool,
                CFG_WDT1_ENABLED: bool,
                CFG_CWDT1_ENABLED: bool,
                CFG_TEMPTAMPER_DET_ENABLED: bool,
                CFG_VOLTAMPER_DET_ENABLED: bool,
                CFG_LHTTAMPER_DET_ENABLED: bool,
                CFG_CLKTAMPER_DET_ENABLED: bool,
                CFG_QK_DISABLE_ENROLL: bool,
                CFG_QK_DISABLE_WRAP: bool,
            }
            let proxy = ELS_AS_CFG0 {
                CFG_LC_STATE: self.CFG_LC_STATE(),
                CFG_LVD_CORE_RESET_ENABLED: self.CFG_LVD_CORE_RESET_ENABLED(),
                CFG_LVD_CORE_IRQ_ENABLED: self.CFG_LVD_CORE_IRQ_ENABLED(),
                CFG_WDT0_ENABLED: self.CFG_WDT0_ENABLED(),
                CFG_CWDT0_ENABLED: self.CFG_CWDT0_ENABLED(),
                CFG_ELS_GDET_ENABLED: self.CFG_ELS_GDET_ENABLED(),
                CFG_ANA_GDET_RESET_ENABLED: self.CFG_ANA_GDET_RESET_ENABLED(),
                CFG_ANA_GDET_IRQ_ENABLED: self.CFG_ANA_GDET_IRQ_ENABLED(),
                CFG_TAMPER_DET_ENABLED: self.CFG_TAMPER_DET_ENABLED(),
                CFG_LVD_VSYS_RESET_ENABLED: self.CFG_LVD_VSYS_RESET_ENABLED(),
                CFG_LVD_VDDIO_RESET_ENABLED: self.CFG_LVD_VDDIO_RESET_ENABLED(),
                CFG_LVD_VSYS_IRQ_ENABLED: self.CFG_LVD_VSYS_IRQ_ENABLED(),
                CFG_LVD_VDDIO_IRQ_ENABLED: self.CFG_LVD_VDDIO_IRQ_ENABLED(),
                CFG_WDT1_ENABLED: self.CFG_WDT1_ENABLED(),
                CFG_CWDT1_ENABLED: self.CFG_CWDT1_ENABLED(),
                CFG_TEMPTAMPER_DET_ENABLED: self.CFG_TEMPTAMPER_DET_ENABLED(),
                CFG_VOLTAMPER_DET_ENABLED: self.CFG_VOLTAMPER_DET_ENABLED(),
                CFG_LHTTAMPER_DET_ENABLED: self.CFG_LHTTAMPER_DET_ENABLED(),
                CFG_CLKTAMPER_DET_ENABLED: self.CFG_CLKTAMPER_DET_ENABLED(),
                CFG_QK_DISABLE_ENROLL: self.CFG_QK_DISABLE_ENROLL(),
                CFG_QK_DISABLE_WRAP: self.CFG_QK_DISABLE_WRAP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS Configuration1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_CFG1(pub u32);
    impl ELS_AS_CFG1 {
        #[inline(always)]
        pub const fn CFG_SEC_DIS_STRICT_MODE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_DIS_STRICT_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_DIS_VIOL_ABORT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_DIS_VIOL_ABORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_ENA_NS_PRIV_CHK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_ENA_NS_PRIV_CHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_ENA_S_PRIV_CHK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_ENA_S_PRIV_CHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_ENA_SEC_CHK(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_ENA_SEC_CHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_IDAU_ALLNS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_IDAU_ALLNS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_LOCK_NS_MPU(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_LOCK_NS_MPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_LOCK_NS_VTOR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_LOCK_NS_VTOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_LOCK_S_MPU(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_LOCK_S_MPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_LOCK_S_VTAIRCR(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_LOCK_S_VTAIRCR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CFG_SEC_LOCK_SAU(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_SEC_LOCK_SAU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn METAL_VERSION(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_METAL_VERSION(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 13usize)) | (((val as u32) & 0xff) << 13usize);
        }
        #[inline(always)]
        pub const fn ROM_PATCH_VERSION(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ROM_PATCH_VERSION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_CORE_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_CORE_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_CORE_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_CORE_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_VSYS_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_VSYS_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_VDDIO_RESET_ENABLED(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_VDDIO_RESET_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_VSYS_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_VSYS_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn CFG_HVD_VDDIO_IRQ_ENABLED(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFG_HVD_VDDIO_IRQ_ENABLED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ELS_AS_CFG1 {
        #[inline(always)]
        fn default() -> ELS_AS_CFG1 {
            ELS_AS_CFG1(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_CFG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_CFG1")
                .field("CFG_SEC_DIS_STRICT_MODE", &self.CFG_SEC_DIS_STRICT_MODE())
                .field("CFG_SEC_DIS_VIOL_ABORT", &self.CFG_SEC_DIS_VIOL_ABORT())
                .field("CFG_SEC_ENA_NS_PRIV_CHK", &self.CFG_SEC_ENA_NS_PRIV_CHK())
                .field("CFG_SEC_ENA_S_PRIV_CHK", &self.CFG_SEC_ENA_S_PRIV_CHK())
                .field("CFG_SEC_ENA_SEC_CHK", &self.CFG_SEC_ENA_SEC_CHK())
                .field("CFG_SEC_IDAU_ALLNS", &self.CFG_SEC_IDAU_ALLNS())
                .field("CFG_SEC_LOCK_NS_MPU", &self.CFG_SEC_LOCK_NS_MPU())
                .field("CFG_SEC_LOCK_NS_VTOR", &self.CFG_SEC_LOCK_NS_VTOR())
                .field("CFG_SEC_LOCK_S_MPU", &self.CFG_SEC_LOCK_S_MPU())
                .field("CFG_SEC_LOCK_S_VTAIRCR", &self.CFG_SEC_LOCK_S_VTAIRCR())
                .field("CFG_SEC_LOCK_SAU", &self.CFG_SEC_LOCK_SAU())
                .field("METAL_VERSION", &self.METAL_VERSION())
                .field("ROM_PATCH_VERSION", &self.ROM_PATCH_VERSION())
                .field(
                    "CFG_HVD_CORE_RESET_ENABLED",
                    &self.CFG_HVD_CORE_RESET_ENABLED(),
                )
                .field("CFG_HVD_CORE_IRQ_ENABLED", &self.CFG_HVD_CORE_IRQ_ENABLED())
                .field(
                    "CFG_HVD_VSYS_RESET_ENABLED",
                    &self.CFG_HVD_VSYS_RESET_ENABLED(),
                )
                .field(
                    "CFG_HVD_VDDIO_RESET_ENABLED",
                    &self.CFG_HVD_VDDIO_RESET_ENABLED(),
                )
                .field("CFG_HVD_VSYS_IRQ_ENABLED", &self.CFG_HVD_VSYS_IRQ_ENABLED())
                .field(
                    "CFG_HVD_VDDIO_IRQ_ENABLED",
                    &self.CFG_HVD_VDDIO_IRQ_ENABLED(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_CFG1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_CFG1 {
                CFG_SEC_DIS_STRICT_MODE: bool,
                CFG_SEC_DIS_VIOL_ABORT: bool,
                CFG_SEC_ENA_NS_PRIV_CHK: bool,
                CFG_SEC_ENA_S_PRIV_CHK: bool,
                CFG_SEC_ENA_SEC_CHK: bool,
                CFG_SEC_IDAU_ALLNS: bool,
                CFG_SEC_LOCK_NS_MPU: bool,
                CFG_SEC_LOCK_NS_VTOR: bool,
                CFG_SEC_LOCK_S_MPU: bool,
                CFG_SEC_LOCK_S_VTAIRCR: bool,
                CFG_SEC_LOCK_SAU: bool,
                METAL_VERSION: u8,
                ROM_PATCH_VERSION: u8,
                CFG_HVD_CORE_RESET_ENABLED: bool,
                CFG_HVD_CORE_IRQ_ENABLED: bool,
                CFG_HVD_VSYS_RESET_ENABLED: bool,
                CFG_HVD_VDDIO_RESET_ENABLED: bool,
                CFG_HVD_VSYS_IRQ_ENABLED: bool,
                CFG_HVD_VDDIO_IRQ_ENABLED: bool,
            }
            let proxy = ELS_AS_CFG1 {
                CFG_SEC_DIS_STRICT_MODE: self.CFG_SEC_DIS_STRICT_MODE(),
                CFG_SEC_DIS_VIOL_ABORT: self.CFG_SEC_DIS_VIOL_ABORT(),
                CFG_SEC_ENA_NS_PRIV_CHK: self.CFG_SEC_ENA_NS_PRIV_CHK(),
                CFG_SEC_ENA_S_PRIV_CHK: self.CFG_SEC_ENA_S_PRIV_CHK(),
                CFG_SEC_ENA_SEC_CHK: self.CFG_SEC_ENA_SEC_CHK(),
                CFG_SEC_IDAU_ALLNS: self.CFG_SEC_IDAU_ALLNS(),
                CFG_SEC_LOCK_NS_MPU: self.CFG_SEC_LOCK_NS_MPU(),
                CFG_SEC_LOCK_NS_VTOR: self.CFG_SEC_LOCK_NS_VTOR(),
                CFG_SEC_LOCK_S_MPU: self.CFG_SEC_LOCK_S_MPU(),
                CFG_SEC_LOCK_S_VTAIRCR: self.CFG_SEC_LOCK_S_VTAIRCR(),
                CFG_SEC_LOCK_SAU: self.CFG_SEC_LOCK_SAU(),
                METAL_VERSION: self.METAL_VERSION(),
                ROM_PATCH_VERSION: self.ROM_PATCH_VERSION(),
                CFG_HVD_CORE_RESET_ENABLED: self.CFG_HVD_CORE_RESET_ENABLED(),
                CFG_HVD_CORE_IRQ_ENABLED: self.CFG_HVD_CORE_IRQ_ENABLED(),
                CFG_HVD_VSYS_RESET_ENABLED: self.CFG_HVD_VSYS_RESET_ENABLED(),
                CFG_HVD_VDDIO_RESET_ENABLED: self.CFG_HVD_VDDIO_RESET_ENABLED(),
                CFG_HVD_VSYS_IRQ_ENABLED: self.CFG_HVD_VSYS_IRQ_ENABLED(),
                CFG_HVD_VDDIO_IRQ_ENABLED: self.CFG_HVD_VDDIO_IRQ_ENABLED(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS Flag0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_FLAG0(pub u32);
    impl ELS_AS_FLAG0 {
        #[inline(always)]
        pub const fn FLAG_AP_ENABLE_CPU0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_AP_ENABLE_CPU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EFUSE_ATTACK_DETECT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EFUSE_ATTACK_DETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FLAG_LVD_CORE_OCCURED(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_LVD_CORE_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FLAG_WDT0_RESET_OCCURED(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_WDT0_RESET_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FLAG_CWDT0_RESET_OCCURED(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CWDT0_RESET_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FLAG_WDT0_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_WDT0_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FLAG_CWDT0_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CWDT0_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FLAG_QK_ERROR(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_QK_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FLAG_ELS_GLITCH_DETECTED(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_ELS_GLITCH_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn FLAG_ANA_GLITCH_DETECTED(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_ANA_GLITCH_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FLAG_TAMPER_EVENT_DETECTED(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_TAMPER_EVENT_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn FLAG_FLASH_ECC_INVALID(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_FLASH_ECC_INVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FLAG_SEC_VIOL_IRQ_OCURRED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_SEC_VIOL_IRQ_OCURRED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FLAG_CPU0_NS_C_ACC_OCCURED(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CPU0_NS_C_ACC_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn FLAG_CPU0_NS_D_ACC_OCCURED(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CPU0_NS_D_ACC_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn FLAG_LVD_VSYS_OCCURED(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_LVD_VSYS_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn FLAG_LVD_VDDIO_OCCURED(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_LVD_VDDIO_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn FLAG_WDT1_RESET_OCCURED(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_WDT1_RESET_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn FLAG_CWDT1_RESET_OCCURED(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CWDT1_RESET_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn FLAG_WDT1_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_WDT1_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn FLAG_CWDT1_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CWDT1_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn FLAG_TEMPTAMPER_DET_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_TEMPTAMPER_DET_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn FLAG_VOLTAMPER_DET_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_VOLTAMPER_DET_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn FLAG_LHTTAMPER_DET_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_LHTTAMPER_DET_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn FLAG_CLKTAMPER_DET_IRQ_OCCURED(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_CLKTAMPER_DET_IRQ_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for ELS_AS_FLAG0 {
        #[inline(always)]
        fn default() -> ELS_AS_FLAG0 {
            ELS_AS_FLAG0(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_FLAG0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_FLAG0")
                .field("FLAG_AP_ENABLE_CPU0", &self.FLAG_AP_ENABLE_CPU0())
                .field("EFUSE_ATTACK_DETECT", &self.EFUSE_ATTACK_DETECT())
                .field("FLAG_LVD_CORE_OCCURED", &self.FLAG_LVD_CORE_OCCURED())
                .field("FLAG_WDT0_RESET_OCCURED", &self.FLAG_WDT0_RESET_OCCURED())
                .field("FLAG_CWDT0_RESET_OCCURED", &self.FLAG_CWDT0_RESET_OCCURED())
                .field("FLAG_WDT0_IRQ_OCCURED", &self.FLAG_WDT0_IRQ_OCCURED())
                .field("FLAG_CWDT0_IRQ_OCCURED", &self.FLAG_CWDT0_IRQ_OCCURED())
                .field("FLAG_QK_ERROR", &self.FLAG_QK_ERROR())
                .field("FLAG_ELS_GLITCH_DETECTED", &self.FLAG_ELS_GLITCH_DETECTED())
                .field("FLAG_ANA_GLITCH_DETECTED", &self.FLAG_ANA_GLITCH_DETECTED())
                .field(
                    "FLAG_TAMPER_EVENT_DETECTED",
                    &self.FLAG_TAMPER_EVENT_DETECTED(),
                )
                .field("FLAG_FLASH_ECC_INVALID", &self.FLAG_FLASH_ECC_INVALID())
                .field(
                    "FLAG_SEC_VIOL_IRQ_OCURRED",
                    &self.FLAG_SEC_VIOL_IRQ_OCURRED(),
                )
                .field(
                    "FLAG_CPU0_NS_C_ACC_OCCURED",
                    &self.FLAG_CPU0_NS_C_ACC_OCCURED(),
                )
                .field(
                    "FLAG_CPU0_NS_D_ACC_OCCURED",
                    &self.FLAG_CPU0_NS_D_ACC_OCCURED(),
                )
                .field("FLAG_LVD_VSYS_OCCURED", &self.FLAG_LVD_VSYS_OCCURED())
                .field("FLAG_LVD_VDDIO_OCCURED", &self.FLAG_LVD_VDDIO_OCCURED())
                .field("FLAG_WDT1_RESET_OCCURED", &self.FLAG_WDT1_RESET_OCCURED())
                .field("FLAG_CWDT1_RESET_OCCURED", &self.FLAG_CWDT1_RESET_OCCURED())
                .field("FLAG_WDT1_IRQ_OCCURED", &self.FLAG_WDT1_IRQ_OCCURED())
                .field("FLAG_CWDT1_IRQ_OCCURED", &self.FLAG_CWDT1_IRQ_OCCURED())
                .field(
                    "FLAG_TEMPTAMPER_DET_IRQ_OCCURED",
                    &self.FLAG_TEMPTAMPER_DET_IRQ_OCCURED(),
                )
                .field(
                    "FLAG_VOLTAMPER_DET_IRQ_OCCURED",
                    &self.FLAG_VOLTAMPER_DET_IRQ_OCCURED(),
                )
                .field(
                    "FLAG_LHTTAMPER_DET_IRQ_OCCURED",
                    &self.FLAG_LHTTAMPER_DET_IRQ_OCCURED(),
                )
                .field(
                    "FLAG_CLKTAMPER_DET_IRQ_OCCURED",
                    &self.FLAG_CLKTAMPER_DET_IRQ_OCCURED(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_FLAG0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_FLAG0 {
                FLAG_AP_ENABLE_CPU0: bool,
                EFUSE_ATTACK_DETECT: bool,
                FLAG_LVD_CORE_OCCURED: bool,
                FLAG_WDT0_RESET_OCCURED: bool,
                FLAG_CWDT0_RESET_OCCURED: bool,
                FLAG_WDT0_IRQ_OCCURED: bool,
                FLAG_CWDT0_IRQ_OCCURED: bool,
                FLAG_QK_ERROR: bool,
                FLAG_ELS_GLITCH_DETECTED: bool,
                FLAG_ANA_GLITCH_DETECTED: bool,
                FLAG_TAMPER_EVENT_DETECTED: bool,
                FLAG_FLASH_ECC_INVALID: bool,
                FLAG_SEC_VIOL_IRQ_OCURRED: bool,
                FLAG_CPU0_NS_C_ACC_OCCURED: bool,
                FLAG_CPU0_NS_D_ACC_OCCURED: bool,
                FLAG_LVD_VSYS_OCCURED: bool,
                FLAG_LVD_VDDIO_OCCURED: bool,
                FLAG_WDT1_RESET_OCCURED: bool,
                FLAG_CWDT1_RESET_OCCURED: bool,
                FLAG_WDT1_IRQ_OCCURED: bool,
                FLAG_CWDT1_IRQ_OCCURED: bool,
                FLAG_TEMPTAMPER_DET_IRQ_OCCURED: bool,
                FLAG_VOLTAMPER_DET_IRQ_OCCURED: bool,
                FLAG_LHTTAMPER_DET_IRQ_OCCURED: bool,
                FLAG_CLKTAMPER_DET_IRQ_OCCURED: bool,
            }
            let proxy = ELS_AS_FLAG0 {
                FLAG_AP_ENABLE_CPU0: self.FLAG_AP_ENABLE_CPU0(),
                EFUSE_ATTACK_DETECT: self.EFUSE_ATTACK_DETECT(),
                FLAG_LVD_CORE_OCCURED: self.FLAG_LVD_CORE_OCCURED(),
                FLAG_WDT0_RESET_OCCURED: self.FLAG_WDT0_RESET_OCCURED(),
                FLAG_CWDT0_RESET_OCCURED: self.FLAG_CWDT0_RESET_OCCURED(),
                FLAG_WDT0_IRQ_OCCURED: self.FLAG_WDT0_IRQ_OCCURED(),
                FLAG_CWDT0_IRQ_OCCURED: self.FLAG_CWDT0_IRQ_OCCURED(),
                FLAG_QK_ERROR: self.FLAG_QK_ERROR(),
                FLAG_ELS_GLITCH_DETECTED: self.FLAG_ELS_GLITCH_DETECTED(),
                FLAG_ANA_GLITCH_DETECTED: self.FLAG_ANA_GLITCH_DETECTED(),
                FLAG_TAMPER_EVENT_DETECTED: self.FLAG_TAMPER_EVENT_DETECTED(),
                FLAG_FLASH_ECC_INVALID: self.FLAG_FLASH_ECC_INVALID(),
                FLAG_SEC_VIOL_IRQ_OCURRED: self.FLAG_SEC_VIOL_IRQ_OCURRED(),
                FLAG_CPU0_NS_C_ACC_OCCURED: self.FLAG_CPU0_NS_C_ACC_OCCURED(),
                FLAG_CPU0_NS_D_ACC_OCCURED: self.FLAG_CPU0_NS_D_ACC_OCCURED(),
                FLAG_LVD_VSYS_OCCURED: self.FLAG_LVD_VSYS_OCCURED(),
                FLAG_LVD_VDDIO_OCCURED: self.FLAG_LVD_VDDIO_OCCURED(),
                FLAG_WDT1_RESET_OCCURED: self.FLAG_WDT1_RESET_OCCURED(),
                FLAG_CWDT1_RESET_OCCURED: self.FLAG_CWDT1_RESET_OCCURED(),
                FLAG_WDT1_IRQ_OCCURED: self.FLAG_WDT1_IRQ_OCCURED(),
                FLAG_CWDT1_IRQ_OCCURED: self.FLAG_CWDT1_IRQ_OCCURED(),
                FLAG_TEMPTAMPER_DET_IRQ_OCCURED: self.FLAG_TEMPTAMPER_DET_IRQ_OCCURED(),
                FLAG_VOLTAMPER_DET_IRQ_OCCURED: self.FLAG_VOLTAMPER_DET_IRQ_OCCURED(),
                FLAG_LHTTAMPER_DET_IRQ_OCCURED: self.FLAG_LHTTAMPER_DET_IRQ_OCCURED(),
                FLAG_CLKTAMPER_DET_IRQ_OCCURED: self.FLAG_CLKTAMPER_DET_IRQ_OCCURED(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS Flag1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_FLAG1(pub u32);
    impl ELS_AS_FLAG1 {
        #[inline(always)]
        pub const fn FLAG_HVD_CORE_OCCURED(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_HVD_CORE_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn FLAG_HVD_VSYS_OCCURED(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_HVD_VSYS_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn FLAG_HVD_VDDIO_OCCURED(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG_HVD_VDDIO_OCCURED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ELS_AS_FLAG1 {
        #[inline(always)]
        fn default() -> ELS_AS_FLAG1 {
            ELS_AS_FLAG1(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_FLAG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_FLAG1")
                .field("FLAG_HVD_CORE_OCCURED", &self.FLAG_HVD_CORE_OCCURED())
                .field("FLAG_HVD_VSYS_OCCURED", &self.FLAG_HVD_VSYS_OCCURED())
                .field("FLAG_HVD_VDDIO_OCCURED", &self.FLAG_HVD_VDDIO_OCCURED())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_FLAG1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_FLAG1 {
                FLAG_HVD_CORE_OCCURED: bool,
                FLAG_HVD_VSYS_OCCURED: bool,
                FLAG_HVD_VDDIO_OCCURED: bool,
            }
            let proxy = ELS_AS_FLAG1 {
                FLAG_HVD_CORE_OCCURED: self.FLAG_HVD_CORE_OCCURED(),
                FLAG_HVD_VSYS_OCCURED: self.FLAG_HVD_VSYS_OCCURED(),
                FLAG_HVD_VDDIO_OCCURED: self.FLAG_HVD_VDDIO_OCCURED(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS State Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_ST0(pub u32);
    impl ELS_AS_ST0 {
        #[inline(always)]
        pub const fn ST_TEMPORAL_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_TEMPORAL_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn ST_CPU0_DBGEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_CPU0_DBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ST_CPU0_NIDEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_CPU0_NIDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ST_CPU0_SPIDEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_CPU0_SPIDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ST_CPU0_SPNIDEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_CPU0_SPNIDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ST_DAP_ENABLE_CPU0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_DAP_ENABLE_CPU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ST_ALLOW_TEST_ACCESS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_ALLOW_TEST_ACCESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ST_XO32K_FAILED(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_XO32K_FAILED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ST_XO40M_FAILED(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_XO40M_FAILED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ST_IFR_LOAD_FAILED(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_IFR_LOAD_FAILED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ST_GLITCH_DETECT_FLAG(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_GLITCH_DETECT_FLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
    }
    impl Default for ELS_AS_ST0 {
        #[inline(always)]
        fn default() -> ELS_AS_ST0 {
            ELS_AS_ST0(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_ST0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_ST0")
                .field("ST_TEMPORAL_STATE", &self.ST_TEMPORAL_STATE())
                .field("ST_CPU0_DBGEN", &self.ST_CPU0_DBGEN())
                .field("ST_CPU0_NIDEN", &self.ST_CPU0_NIDEN())
                .field("ST_CPU0_SPIDEN", &self.ST_CPU0_SPIDEN())
                .field("ST_CPU0_SPNIDEN", &self.ST_CPU0_SPNIDEN())
                .field("ST_DAP_ENABLE_CPU0", &self.ST_DAP_ENABLE_CPU0())
                .field("ST_ALLOW_TEST_ACCESS", &self.ST_ALLOW_TEST_ACCESS())
                .field("ST_XO32K_FAILED", &self.ST_XO32K_FAILED())
                .field("ST_XO40M_FAILED", &self.ST_XO40M_FAILED())
                .field("ST_IFR_LOAD_FAILED", &self.ST_IFR_LOAD_FAILED())
                .field("ST_GLITCH_DETECT_FLAG", &self.ST_GLITCH_DETECT_FLAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_ST0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_ST0 {
                ST_TEMPORAL_STATE: u8,
                ST_CPU0_DBGEN: bool,
                ST_CPU0_NIDEN: bool,
                ST_CPU0_SPIDEN: bool,
                ST_CPU0_SPNIDEN: bool,
                ST_DAP_ENABLE_CPU0: bool,
                ST_ALLOW_TEST_ACCESS: bool,
                ST_XO32K_FAILED: bool,
                ST_XO40M_FAILED: bool,
                ST_IFR_LOAD_FAILED: bool,
                ST_GLITCH_DETECT_FLAG: u8,
            }
            let proxy = ELS_AS_ST0 {
                ST_TEMPORAL_STATE: self.ST_TEMPORAL_STATE(),
                ST_CPU0_DBGEN: self.ST_CPU0_DBGEN(),
                ST_CPU0_NIDEN: self.ST_CPU0_NIDEN(),
                ST_CPU0_SPIDEN: self.ST_CPU0_SPIDEN(),
                ST_CPU0_SPNIDEN: self.ST_CPU0_SPNIDEN(),
                ST_DAP_ENABLE_CPU0: self.ST_DAP_ENABLE_CPU0(),
                ST_ALLOW_TEST_ACCESS: self.ST_ALLOW_TEST_ACCESS(),
                ST_XO32K_FAILED: self.ST_XO32K_FAILED(),
                ST_XO40M_FAILED: self.ST_XO40M_FAILED(),
                ST_IFR_LOAD_FAILED: self.ST_IFR_LOAD_FAILED(),
                ST_GLITCH_DETECT_FLAG: self.ST_GLITCH_DETECT_FLAG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS AS State1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_AS_ST1(pub u32);
    impl ELS_AS_ST1 {
        #[inline(always)]
        pub const fn ST_QK_PUF_SCORE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_QK_PUF_SCORE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn ST_QK_ZEROIZED(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_QK_ZEROIZED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ST_MAIN_CLK_IS_EXT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ST_MAIN_CLK_IS_EXT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ST_DCDC_VOUT(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_DCDC_VOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn ST_DCDC_DS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_DCDC_DS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn ST_BOOT_MODE(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_BOOT_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn ST_BOOT_RETRY_CNT(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_BOOT_RETRY_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn ST_LDO_CORE_VOUT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_LDO_CORE_VOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn ST_LDO_CORE_DS(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ST_LDO_CORE_DS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for ELS_AS_ST1 {
        #[inline(always)]
        fn default() -> ELS_AS_ST1 {
            ELS_AS_ST1(0)
        }
    }
    impl core::fmt::Debug for ELS_AS_ST1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_AS_ST1")
                .field("ST_QK_PUF_SCORE", &self.ST_QK_PUF_SCORE())
                .field("ST_QK_ZEROIZED", &self.ST_QK_ZEROIZED())
                .field("ST_MAIN_CLK_IS_EXT", &self.ST_MAIN_CLK_IS_EXT())
                .field("ST_DCDC_VOUT", &self.ST_DCDC_VOUT())
                .field("ST_DCDC_DS", &self.ST_DCDC_DS())
                .field("ST_BOOT_MODE", &self.ST_BOOT_MODE())
                .field("ST_BOOT_RETRY_CNT", &self.ST_BOOT_RETRY_CNT())
                .field("ST_LDO_CORE_VOUT", &self.ST_LDO_CORE_VOUT())
                .field("ST_LDO_CORE_DS", &self.ST_LDO_CORE_DS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_AS_ST1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_AS_ST1 {
                ST_QK_PUF_SCORE: u8,
                ST_QK_ZEROIZED: bool,
                ST_MAIN_CLK_IS_EXT: bool,
                ST_DCDC_VOUT: u8,
                ST_DCDC_DS: u8,
                ST_BOOT_MODE: u8,
                ST_BOOT_RETRY_CNT: u8,
                ST_LDO_CORE_VOUT: u8,
                ST_LDO_CORE_DS: u8,
            }
            let proxy = ELS_AS_ST1 {
                ST_QK_PUF_SCORE: self.ST_QK_PUF_SCORE(),
                ST_QK_ZEROIZED: self.ST_QK_ZEROIZED(),
                ST_MAIN_CLK_IS_EXT: self.ST_MAIN_CLK_IS_EXT(),
                ST_DCDC_VOUT: self.ST_DCDC_VOUT(),
                ST_DCDC_DS: self.ST_DCDC_DS(),
                ST_BOOT_MODE: self.ST_BOOT_MODE(),
                ST_BOOT_RETRY_CNT: self.ST_BOOT_RETRY_CNT(),
                ST_LDO_CORE_VOUT: self.ST_LDO_CORE_VOUT(),
                ST_LDO_CORE_DS: self.ST_LDO_CORE_DS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS Lock Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_LOCK_CTRL(pub u32);
    impl ELS_LOCK_CTRL {
        #[inline(always)]
        pub const fn LOCK_CTRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOCK_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for ELS_LOCK_CTRL {
        #[inline(always)]
        fn default() -> ELS_LOCK_CTRL {
            ELS_LOCK_CTRL(0)
        }
    }
    impl core::fmt::Debug for ELS_LOCK_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_LOCK_CTRL")
                .field("LOCK_CTRL", &self.LOCK_CTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_LOCK_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_LOCK_CTRL {
                LOCK_CTRL: u8,
            }
            let proxy = ELS_LOCK_CTRL {
                LOCK_CTRL: self.LOCK_CTRL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS Lock Control DP"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_LOCK_CTRL_DP(pub u32);
    impl ELS_LOCK_CTRL_DP {
        #[inline(always)]
        pub const fn LOCK_CTRL_DP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOCK_CTRL_DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for ELS_LOCK_CTRL_DP {
        #[inline(always)]
        fn default() -> ELS_LOCK_CTRL_DP {
            ELS_LOCK_CTRL_DP(0)
        }
    }
    impl core::fmt::Debug for ELS_LOCK_CTRL_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_LOCK_CTRL_DP")
                .field("LOCK_CTRL_DP", &self.LOCK_CTRL_DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_LOCK_CTRL_DP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_LOCK_CTRL_DP {
                LOCK_CTRL_DP: u8,
            }
            let proxy = ELS_LOCK_CTRL_DP {
                LOCK_CTRL_DP: self.LOCK_CTRL_DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Life Cycle State Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_OTP_LC_STATE(pub u32);
    impl ELS_OTP_LC_STATE {
        #[inline(always)]
        pub const fn OTP_LC_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OTP_LC_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ELS_OTP_LC_STATE {
        #[inline(always)]
        fn default() -> ELS_OTP_LC_STATE {
            ELS_OTP_LC_STATE(0)
        }
    }
    impl core::fmt::Debug for ELS_OTP_LC_STATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_OTP_LC_STATE")
                .field("OTP_LC_STATE", &self.OTP_LC_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_OTP_LC_STATE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_OTP_LC_STATE {
                OTP_LC_STATE: u8,
            }
            let proxy = ELS_OTP_LC_STATE {
                OTP_LC_STATE: self.OTP_LC_STATE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Life Cycle State Register (Duplicate)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_OTP_LC_STATE_DP(pub u32);
    impl ELS_OTP_LC_STATE_DP {
        #[inline(always)]
        pub const fn OTP_LC_STATE_DP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OTP_LC_STATE_DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ELS_OTP_LC_STATE_DP {
        #[inline(always)]
        fn default() -> ELS_OTP_LC_STATE_DP {
            ELS_OTP_LC_STATE_DP(0)
        }
    }
    impl core::fmt::Debug for ELS_OTP_LC_STATE_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_OTP_LC_STATE_DP")
                .field("OTP_LC_STATE_DP", &self.OTP_LC_STATE_DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_OTP_LC_STATE_DP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_OTP_LC_STATE_DP {
                OTP_LC_STATE_DP: u8,
            }
            let proxy = ELS_OTP_LC_STATE_DP {
                OTP_LC_STATE_DP: self.OTP_LC_STATE_DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ELS Temporal State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_TEMPORAL_STATE(pub u32);
    impl ELS_TEMPORAL_STATE {
        #[inline(always)]
        pub const fn TEMPORAL_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEMPORAL_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ELS_TEMPORAL_STATE {
        #[inline(always)]
        fn default() -> ELS_TEMPORAL_STATE {
            ELS_TEMPORAL_STATE(0)
        }
    }
    impl core::fmt::Debug for ELS_TEMPORAL_STATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_TEMPORAL_STATE")
                .field("TEMPORAL_STATE", &self.TEMPORAL_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_TEMPORAL_STATE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_TEMPORAL_STATE {
                TEMPORAL_STATE: u8,
            }
            let proxy = ELS_TEMPORAL_STATE {
                TEMPORAL_STATE: self.TEMPORAL_STATE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EWM0 Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EWM0CLKSEL(pub u32);
    impl EWM0CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EWM0CLKSEL {
        #[inline(always)]
        fn default() -> EWM0CLKSEL {
            EWM0CLKSEL(0)
        }
    }
    impl core::fmt::Debug for EWM0CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EWM0CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EWM0CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EWM0CLKSEL {
                SEL: bool,
            }
            let proxy = EWM0CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCCLKSEL(pub u32);
    impl FCCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FCCLKSEL {
        #[inline(always)]
        fn default() -> FCCLKSEL {
            FCCLKSEL(0)
        }
    }
    impl core::fmt::Debug for FCCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FCCLKSEL {
                SEL: u8,
            }
            let proxy = FCCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN0 Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCAN0CLKDIV(pub u32);
    impl FLEXCAN0CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for FLEXCAN0CLKDIV {
        #[inline(always)]
        fn default() -> FLEXCAN0CLKDIV {
            FLEXCAN0CLKDIV(0)
        }
    }
    impl core::fmt::Debug for FLEXCAN0CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCAN0CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCAN0CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCAN0CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FLEXCAN0CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN0 Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCAN0CLKSEL(pub u32);
    impl FLEXCAN0CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FLEXCAN0CLKSEL {
        #[inline(always)]
        fn default() -> FLEXCAN0CLKSEL {
            FLEXCAN0CLKSEL(0)
        }
    }
    impl core::fmt::Debug for FLEXCAN0CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCAN0CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCAN0CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCAN0CLKSEL {
                SEL: u8,
            }
            let proxy = FLEXCAN0CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN1 Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCAN1CLKDIV(pub u32);
    impl FLEXCAN1CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for FLEXCAN1CLKDIV {
        #[inline(always)]
        fn default() -> FLEXCAN1CLKDIV {
            FLEXCAN1CLKDIV(0)
        }
    }
    impl core::fmt::Debug for FLEXCAN1CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCAN1CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCAN1CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCAN1CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FLEXCAN1CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXCAN1 Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCAN1CLKSEL(pub u32);
    impl FLEXCAN1CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FLEXCAN1CLKSEL {
        #[inline(always)]
        fn default() -> FLEXCAN1CLKSEL {
            FLEXCAN1CLKSEL(0)
        }
    }
    impl core::fmt::Debug for FLEXCAN1CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCAN1CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCAN1CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCAN1CLKSEL {
                SEL: u8,
            }
            let proxy = FLEXCAN1CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMMCLKDIV(pub u32);
    impl FLEXCOMMCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for FLEXCOMMCLKDIV {
        #[inline(always)]
        fn default() -> FLEXCOMMCLKDIV {
            FLEXCOMMCLKDIV(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMMCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMMCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMMCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMMCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FLEXCOMMCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXIO Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXIOCLKDIV(pub u32);
    impl FLEXIOCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for FLEXIOCLKDIV {
        #[inline(always)]
        fn default() -> FLEXIOCLKDIV {
            FLEXIOCLKDIV(0)
        }
    }
    impl core::fmt::Debug for FLEXIOCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXIOCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXIOCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXIOCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FLEXIOCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FLEXIO Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXIOCLKSEL(pub u32);
    impl FLEXIOCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FLEXIOCLKSEL {
        #[inline(always)]
        fn default() -> FLEXIOCLKSEL {
            FLEXIOCLKSEL(0)
        }
    }
    impl core::fmt::Debug for FLEXIOCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXIOCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXIOCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXIOCLKSEL {
                SEL: u8,
            }
            let proxy = FLEXIOCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO_HF_DIV Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROHFDIV(pub u32);
    impl FROHFDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for FROHFDIV {
        #[inline(always)]
        fn default() -> FROHFDIV {
            FROHFDIV(0)
        }
    }
    impl core::fmt::Debug for FROHFDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROHFDIV")
                .field("DIV", &self.DIV())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROHFDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FROHFDIV {
                DIV: u8,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FROHFDIV {
                DIV: self.DIV(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "GDET Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CTRL(pub u32);
    impl GDET_CTRL {
        #[inline(always)]
        pub const fn GDET_EVTCNT_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET_EVTCNT_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn GDET_ERR_CLR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET_ERR_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn GDET_ISO_SW(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_GDET_ISO_SW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn EVENT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EVENT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn POS_SYNC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POS_SYNC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn NEG_SYNC(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NEG_SYNC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn EVENT_CLR_FLAG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EVENT_CLR_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for GDET_CTRL {
        #[inline(always)]
        fn default() -> GDET_CTRL {
            GDET_CTRL(0)
        }
    }
    impl core::fmt::Debug for GDET_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CTRL")
                .field("GDET_EVTCNT_CLR", &self.GDET_EVTCNT_CLR())
                .field("GDET_ERR_CLR", &self.GDET_ERR_CLR())
                .field("GDET_ISO_SW", &self.GDET_ISO_SW())
                .field("EVENT_CNT", &self.EVENT_CNT())
                .field("POS_SYNC", &self.POS_SYNC())
                .field("NEG_SYNC", &self.NEG_SYNC())
                .field("EVENT_CLR_FLAG", &self.EVENT_CLR_FLAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GDET_CTRL {
                GDET_EVTCNT_CLR: bool,
                GDET_ERR_CLR: bool,
                GDET_ISO_SW: u8,
                EVENT_CNT: u8,
                POS_SYNC: bool,
                NEG_SYNC: bool,
                EVENT_CLR_FLAG: bool,
            }
            let proxy = GDET_CTRL {
                GDET_EVTCNT_CLR: self.GDET_EVTCNT_CLR(),
                GDET_ERR_CLR: self.GDET_ERR_CLR(),
                GDET_ISO_SW: self.GDET_ISO_SW(),
                EVENT_CNT: self.EVENT_CNT(),
                POS_SYNC: self.POS_SYNC(),
                NEG_SYNC: self.NEG_SYNC(),
                EVENT_CLR_FLAG: self.EVENT_CLR_FLAG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Gray to Binary Converter Gray code_gray\\[41:32\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GRAY_CODE_MSB(pub u32);
    impl GRAY_CODE_MSB {
        #[inline(always)]
        pub const fn code_gray_41_32(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_code_gray_41_32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for GRAY_CODE_MSB {
        #[inline(always)]
        fn default() -> GRAY_CODE_MSB {
            GRAY_CODE_MSB(0)
        }
    }
    impl core::fmt::Debug for GRAY_CODE_MSB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GRAY_CODE_MSB")
                .field("code_gray_41_32", &self.code_gray_41_32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GRAY_CODE_MSB {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GRAY_CODE_MSB {
                code_gray_41_32: u16,
            }
            let proxy = GRAY_CODE_MSB {
                code_gray_41_32: self.code_gray_41_32(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C0 Functional Clock FCLK Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3C0FCLKDIV(pub u32);
    impl I3C0FCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for I3C0FCLKDIV {
        #[inline(always)]
        fn default() -> I3C0FCLKDIV {
            I3C0FCLKDIV(0)
        }
    }
    impl core::fmt::Debug for I3C0FCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3C0FCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3C0FCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct I3C0FCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = I3C0FCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C0 Functional Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3C0FCLKSEL(pub u32);
    impl I3C0FCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for I3C0FCLKSEL {
        #[inline(always)]
        fn default() -> I3C0FCLKSEL {
            I3C0FCLKSEL(0)
        }
    }
    impl core::fmt::Debug for I3C0FCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3C0FCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3C0FCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct I3C0FCLKSEL {
                SEL: u8,
            }
            let proxy = I3C0FCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C1 Functional Clock FCLK Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3C1FCLKDIV(pub u32);
    impl I3C1FCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for I3C1FCLKDIV {
        #[inline(always)]
        fn default() -> I3C1FCLKDIV {
            I3C1FCLKDIV(0)
        }
    }
    impl core::fmt::Debug for I3C1FCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3C1FCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3C1FCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct I3C1FCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = I3C1FCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "I3C1 Functional Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3C1FCLKSEL(pub u32);
    impl I3C1FCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for I3C1FCLKSEL {
        #[inline(always)]
        fn default() -> I3C1FCLKSEL {
            I3C1FCLKSEL(0)
        }
    }
    impl core::fmt::Debug for I3C1FCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3C1FCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3C1FCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct I3C1FCLKSEL {
                SEL: u8,
            }
            let proxy = I3C1FCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Key Retain Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct KEY_RETAIN_CTRL(pub u32);
    impl KEY_RETAIN_CTRL {
        #[inline(always)]
        pub const fn KEY_RETAIN_VALID(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KEY_RETAIN_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn KEY_RETAIN_DONE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KEY_RETAIN_DONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn KEY_SAVE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KEY_SAVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn KEY_LOAD(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_KEY_LOAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for KEY_RETAIN_CTRL {
        #[inline(always)]
        fn default() -> KEY_RETAIN_CTRL {
            KEY_RETAIN_CTRL(0)
        }
    }
    impl core::fmt::Debug for KEY_RETAIN_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("KEY_RETAIN_CTRL")
                .field("KEY_RETAIN_VALID", &self.KEY_RETAIN_VALID())
                .field("KEY_RETAIN_DONE", &self.KEY_RETAIN_DONE())
                .field("KEY_SAVE", &self.KEY_SAVE())
                .field("KEY_LOAD", &self.KEY_LOAD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for KEY_RETAIN_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct KEY_RETAIN_CTRL {
                KEY_RETAIN_VALID: bool,
                KEY_RETAIN_DONE: bool,
                KEY_SAVE: bool,
                KEY_LOAD: bool,
            }
            let proxy = KEY_RETAIN_CTRL {
                KEY_RETAIN_VALID: self.KEY_RETAIN_VALID(),
                KEY_RETAIN_DONE: self.KEY_RETAIN_DONE(),
                KEY_SAVE: self.KEY_SAVE(),
                KEY_LOAD: self.KEY_LOAD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LPCAC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPCAC_CTRL(pub u32);
    impl LPCAC_CTRL {
        #[inline(always)]
        pub const fn DIS_LPCAC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_LPCAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLR_LPCAC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLR_LPCAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRC_NO_ALLOC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRC_NO_ALLOC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PARITY_MISS_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PARITY_MISS_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DIS_LPCAC_WTBF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_LPCAC_WTBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn LIM_LPCAC_WTBF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIM_LPCAC_WTBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PARITY_FAULT_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PARITY_FAULT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn LPCAC_XOM(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPCAC_XOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for LPCAC_CTRL {
        #[inline(always)]
        fn default() -> LPCAC_CTRL {
            LPCAC_CTRL(0)
        }
    }
    impl core::fmt::Debug for LPCAC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPCAC_CTRL")
                .field("DIS_LPCAC", &self.DIS_LPCAC())
                .field("CLR_LPCAC", &self.CLR_LPCAC())
                .field("FRC_NO_ALLOC", &self.FRC_NO_ALLOC())
                .field("PARITY_MISS_EN", &self.PARITY_MISS_EN())
                .field("DIS_LPCAC_WTBF", &self.DIS_LPCAC_WTBF())
                .field("LIM_LPCAC_WTBF", &self.LIM_LPCAC_WTBF())
                .field("PARITY_FAULT_EN", &self.PARITY_FAULT_EN())
                .field("LPCAC_XOM", &self.LPCAC_XOM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPCAC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPCAC_CTRL {
                DIS_LPCAC: bool,
                CLR_LPCAC: bool,
                FRC_NO_ALLOC: bool,
                PARITY_MISS_EN: bool,
                DIS_LPCAC_WTBF: bool,
                LIM_LPCAC_WTBF: bool,
                PARITY_FAULT_EN: bool,
                LPCAC_XOM: bool,
            }
            let proxy = LPCAC_CTRL {
                DIS_LPCAC: self.DIS_LPCAC(),
                CLR_LPCAC: self.CLR_LPCAC(),
                FRC_NO_ALLOC: self.FRC_NO_ALLOC(),
                PARITY_MISS_EN: self.PARITY_MISS_EN(),
                DIS_LPCAC_WTBF: self.DIS_LPCAC_WTBF(),
                LIM_LPCAC_WTBF: self.LIM_LPCAC_WTBF(),
                PARITY_FAULT_EN: self.PARITY_FAULT_EN(),
                LPCAC_XOM: self.LPCAC_XOM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Clock Division"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MICFILFCLKDIV(pub u32);
    impl MICFILFCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for MICFILFCLKDIV {
        #[inline(always)]
        fn default() -> MICFILFCLKDIV {
            MICFILFCLKDIV(0)
        }
    }
    impl core::fmt::Debug for MICFILFCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MICFILFCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MICFILFCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MICFILFCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = MICFILFCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MICFIL Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MICFILFCLKSEL(pub u32);
    impl MICFILFCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for MICFILFCLKSEL {
        #[inline(always)]
        fn default() -> MICFILFCLKSEL {
            MICFILFCLKSEL(0)
        }
    }
    impl core::fmt::Debug for MICFILFCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MICFILFCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MICFILFCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MICFILFCLKSEL {
                SEL: u8,
            }
            let proxy = MICFILFCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "NMI Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NMISRC(pub u32);
    impl NMISRC {
        #[inline(always)]
        pub const fn IRQCPU0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_IRQCPU0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NMIENCPU0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NMIENCPU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for NMISRC {
        #[inline(always)]
        fn default() -> NMISRC {
            NMISRC(0)
        }
    }
    impl core::fmt::Debug for NMISRC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NMISRC")
                .field("IRQCPU0", &self.IRQCPU0())
                .field("NMIENCPU0", &self.NMIENCPU0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NMISRC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct NMISRC {
                IRQCPU0: u8,
                NMIENCPU0: bool,
            }
            let proxy = NMISRC {
                IRQCPU0: self.IRQCPU0(),
                NMIENCPU0: self.NMIENCPU0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "NVM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NVM_CTRL(pub u32);
    impl NVM_CTRL {
        #[inline(always)]
        pub const fn DIS_FLASH_SPEC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_FLASH_SPEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DIS_DATA_SPEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_DATA_SPEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DIS_FLASH_CACHE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_FLASH_CACHE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIS_FLASH_INST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_FLASH_INST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DIS_FLASH_DATA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_FLASH_DATA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CLR_FLASH_CACHE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLR_FLASH_CACHE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FLASH_STALL_EN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLASH_STALL_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DIS_MBECC_ERR_INST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_MBECC_ERR_INST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn DIS_MBECC_ERR_DATA(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_MBECC_ERR_DATA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for NVM_CTRL {
        #[inline(always)]
        fn default() -> NVM_CTRL {
            NVM_CTRL(0)
        }
    }
    impl core::fmt::Debug for NVM_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NVM_CTRL")
                .field("DIS_FLASH_SPEC", &self.DIS_FLASH_SPEC())
                .field("DIS_DATA_SPEC", &self.DIS_DATA_SPEC())
                .field("DIS_FLASH_CACHE", &self.DIS_FLASH_CACHE())
                .field("DIS_FLASH_INST", &self.DIS_FLASH_INST())
                .field("DIS_FLASH_DATA", &self.DIS_FLASH_DATA())
                .field("CLR_FLASH_CACHE", &self.CLR_FLASH_CACHE())
                .field("FLASH_STALL_EN", &self.FLASH_STALL_EN())
                .field("DIS_MBECC_ERR_INST", &self.DIS_MBECC_ERR_INST())
                .field("DIS_MBECC_ERR_DATA", &self.DIS_MBECC_ERR_DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NVM_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct NVM_CTRL {
                DIS_FLASH_SPEC: bool,
                DIS_DATA_SPEC: bool,
                DIS_FLASH_CACHE: bool,
                DIS_FLASH_INST: bool,
                DIS_FLASH_DATA: bool,
                CLR_FLASH_CACHE: bool,
                FLASH_STALL_EN: bool,
                DIS_MBECC_ERR_INST: bool,
                DIS_MBECC_ERR_DATA: bool,
            }
            let proxy = NVM_CTRL {
                DIS_FLASH_SPEC: self.DIS_FLASH_SPEC(),
                DIS_DATA_SPEC: self.DIS_DATA_SPEC(),
                DIS_FLASH_CACHE: self.DIS_FLASH_CACHE(),
                DIS_FLASH_INST: self.DIS_FLASH_INST(),
                DIS_FLASH_DATA: self.DIS_FLASH_DATA(),
                CLR_FLASH_CACHE: self.CLR_FLASH_CACHE(),
                FLASH_STALL_EN: self.FLASH_STALL_EN(),
                DIS_MBECC_ERR_INST: self.DIS_MBECC_ERR_INST(),
                DIS_MBECC_ERR_DATA: self.DIS_MBECC_ERR_DATA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "OSTIMER Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSTIMERCLKSEL(pub u32);
    impl OSTIMERCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for OSTIMERCLKSEL {
        #[inline(always)]
        fn default() -> OSTIMERCLKSEL {
            OSTIMERCLKSEL(0)
        }
    }
    impl core::fmt::Debug for OSTIMERCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSTIMERCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSTIMERCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OSTIMERCLKSEL {
                SEL: u8,
            }
            let proxy = OSTIMERCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL1 Clock 0 Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL1CLK0DIV(pub u32);
    impl PLL1CLK0DIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for PLL1CLK0DIV {
        #[inline(always)]
        fn default() -> PLL1CLK0DIV {
            PLL1CLK0DIV(0)
        }
    }
    impl core::fmt::Debug for PLL1CLK0DIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL1CLK0DIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL1CLK0DIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL1CLK0DIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = PLL1CLK0DIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL1 Clock 1 Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL1CLK1DIV(pub u32);
    impl PLL1CLK1DIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for PLL1CLK1DIV {
        #[inline(always)]
        fn default() -> PLL1CLK1DIV {
            PLL1CLK1DIV(0)
        }
    }
    impl core::fmt::Debug for PLL1CLK1DIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL1CLK1DIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL1CLK1DIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL1CLK1DIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = PLL1CLK1DIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLLCLKDIV(pub u32);
    impl PLLCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for PLLCLKDIV {
        #[inline(always)]
        fn default() -> PLLCLKDIV {
            PLLCLKDIV(0)
        }
    }
    impl core::fmt::Debug for PLLCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLLCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLLCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLLCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = PLLCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL Clock Divider Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLLCLKDIVSEL(pub u32);
    impl PLLCLKDIVSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for PLLCLKDIVSEL {
        #[inline(always)]
        fn default() -> PLLCLKDIVSEL {
            PLLCLKDIVSEL(0)
        }
    }
    impl core::fmt::Debug for PLLCLKDIVSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLLCLKDIVSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLLCLKDIVSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLLCLKDIVSEL {
                SEL: u8,
            }
            let proxy = PLLCLKDIVSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PRESETCTRL0(pub u32);
    impl PRESETCTRL0 {
        #[inline(always)]
        pub const fn FMU_RST(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FMU_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn MUX_RST(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MUX_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PORT0_RST(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PORT1_RST(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn PORT2_RST(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT2_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PORT3_RST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT3_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn PORT4_RST(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PORT4_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn GPIO0_RST(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn GPIO1_RST(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn GPIO2_RST(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO2_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn GPIO3_RST(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO3_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn GPIO4_RST(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GPIO4_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PINT_RST(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PINT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn DMA0_RST(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn CRC_RST(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for PRESETCTRL0 {
        #[inline(always)]
        fn default() -> PRESETCTRL0 {
            PRESETCTRL0(0)
        }
    }
    impl core::fmt::Debug for PRESETCTRL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PRESETCTRL0")
                .field("FMU_RST", &self.FMU_RST())
                .field("MUX_RST", &self.MUX_RST())
                .field("PORT0_RST", &self.PORT0_RST())
                .field("PORT1_RST", &self.PORT1_RST())
                .field("PORT2_RST", &self.PORT2_RST())
                .field("PORT3_RST", &self.PORT3_RST())
                .field("PORT4_RST", &self.PORT4_RST())
                .field("GPIO0_RST", &self.GPIO0_RST())
                .field("GPIO1_RST", &self.GPIO1_RST())
                .field("GPIO2_RST", &self.GPIO2_RST())
                .field("GPIO3_RST", &self.GPIO3_RST())
                .field("GPIO4_RST", &self.GPIO4_RST())
                .field("PINT_RST", &self.PINT_RST())
                .field("DMA0_RST", &self.DMA0_RST())
                .field("CRC_RST", &self.CRC_RST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PRESETCTRL0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PRESETCTRL0 {
                FMU_RST: bool,
                MUX_RST: bool,
                PORT0_RST: bool,
                PORT1_RST: bool,
                PORT2_RST: bool,
                PORT3_RST: bool,
                PORT4_RST: bool,
                GPIO0_RST: bool,
                GPIO1_RST: bool,
                GPIO2_RST: bool,
                GPIO3_RST: bool,
                GPIO4_RST: bool,
                PINT_RST: bool,
                DMA0_RST: bool,
                CRC_RST: bool,
            }
            let proxy = PRESETCTRL0 {
                FMU_RST: self.FMU_RST(),
                MUX_RST: self.MUX_RST(),
                PORT0_RST: self.PORT0_RST(),
                PORT1_RST: self.PORT1_RST(),
                PORT2_RST: self.PORT2_RST(),
                PORT3_RST: self.PORT3_RST(),
                PORT4_RST: self.PORT4_RST(),
                GPIO0_RST: self.GPIO0_RST(),
                GPIO1_RST: self.GPIO1_RST(),
                GPIO2_RST: self.GPIO2_RST(),
                GPIO3_RST: self.GPIO3_RST(),
                GPIO4_RST: self.GPIO4_RST(),
                PINT_RST: self.PINT_RST(),
                DMA0_RST: self.DMA0_RST(),
                CRC_RST: self.CRC_RST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PRESETCTRL1(pub u32);
    impl PRESETCTRL1 {
        #[inline(always)]
        pub const fn MRT_RST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MRT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER_RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ADC0_RST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ADC1_RST(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RTC_RST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTC_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn UTICK_RST(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTICK_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FC0_RST(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FC1_RST(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FC2_RST(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC2_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn FC3_RST(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC3_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FC4_RST(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC4_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn FC5_RST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC5_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FC6_RST(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC6_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn FC7_RST(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FC7_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn MICFIL_RST(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MICFIL_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TIMER2_RST(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER2_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TIMER0_RST(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn TIMER1_RST(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn SmartDMA_RST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SmartDMA_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PRESETCTRL1 {
        #[inline(always)]
        fn default() -> PRESETCTRL1 {
            PRESETCTRL1(0)
        }
    }
    impl core::fmt::Debug for PRESETCTRL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PRESETCTRL1")
                .field("MRT_RST", &self.MRT_RST())
                .field("OSTIMER_RST", &self.OSTIMER_RST())
                .field("ADC0_RST", &self.ADC0_RST())
                .field("ADC1_RST", &self.ADC1_RST())
                .field("RTC_RST", &self.RTC_RST())
                .field("UTICK_RST", &self.UTICK_RST())
                .field("FC0_RST", &self.FC0_RST())
                .field("FC1_RST", &self.FC1_RST())
                .field("FC2_RST", &self.FC2_RST())
                .field("FC3_RST", &self.FC3_RST())
                .field("FC4_RST", &self.FC4_RST())
                .field("FC5_RST", &self.FC5_RST())
                .field("FC6_RST", &self.FC6_RST())
                .field("FC7_RST", &self.FC7_RST())
                .field("MICFIL_RST", &self.MICFIL_RST())
                .field("TIMER2_RST", &self.TIMER2_RST())
                .field("TIMER0_RST", &self.TIMER0_RST())
                .field("TIMER1_RST", &self.TIMER1_RST())
                .field("SmartDMA_RST", &self.SmartDMA_RST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PRESETCTRL1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PRESETCTRL1 {
                MRT_RST: bool,
                OSTIMER_RST: bool,
                ADC0_RST: bool,
                ADC1_RST: bool,
                RTC_RST: bool,
                UTICK_RST: bool,
                FC0_RST: bool,
                FC1_RST: bool,
                FC2_RST: bool,
                FC3_RST: bool,
                FC4_RST: bool,
                FC5_RST: bool,
                FC6_RST: bool,
                FC7_RST: bool,
                MICFIL_RST: bool,
                TIMER2_RST: bool,
                TIMER0_RST: bool,
                TIMER1_RST: bool,
                SmartDMA_RST: bool,
            }
            let proxy = PRESETCTRL1 {
                MRT_RST: self.MRT_RST(),
                OSTIMER_RST: self.OSTIMER_RST(),
                ADC0_RST: self.ADC0_RST(),
                ADC1_RST: self.ADC1_RST(),
                RTC_RST: self.RTC_RST(),
                UTICK_RST: self.UTICK_RST(),
                FC0_RST: self.FC0_RST(),
                FC1_RST: self.FC1_RST(),
                FC2_RST: self.FC2_RST(),
                FC3_RST: self.FC3_RST(),
                FC4_RST: self.FC4_RST(),
                FC5_RST: self.FC5_RST(),
                FC6_RST: self.FC6_RST(),
                FC7_RST: self.FC7_RST(),
                MICFIL_RST: self.MICFIL_RST(),
                TIMER2_RST: self.TIMER2_RST(),
                TIMER0_RST: self.TIMER0_RST(),
                TIMER1_RST: self.TIMER1_RST(),
                SmartDMA_RST: self.SmartDMA_RST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PRESETCTRL2(pub u32);
    impl PRESETCTRL2 {
        #[inline(always)]
        pub const fn DMA1_RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FLEXIO_RST(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXIO_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn SAI0_RST(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAI0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SAI1_RST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAI1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn TRO_RST(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRO_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FREQME_RST(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FREQME_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN0_RST(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FLEXCAN1_RST(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXCAN1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn USB_HS_RST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_HS_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn USB_HS_PHY_RST(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_HS_PHY_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn TIMER3_RST(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER3_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn TIMER4_RST(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER4_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn PUF_RST(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PUF_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn PKC_RST(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PKC_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PRESETCTRL2 {
        #[inline(always)]
        fn default() -> PRESETCTRL2 {
            PRESETCTRL2(0)
        }
    }
    impl core::fmt::Debug for PRESETCTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PRESETCTRL2")
                .field("DMA1_RST", &self.DMA1_RST())
                .field("FLEXIO_RST", &self.FLEXIO_RST())
                .field("SAI0_RST", &self.SAI0_RST())
                .field("SAI1_RST", &self.SAI1_RST())
                .field("TRO_RST", &self.TRO_RST())
                .field("FREQME_RST", &self.FREQME_RST())
                .field("FLEXCAN0_RST", &self.FLEXCAN0_RST())
                .field("FLEXCAN1_RST", &self.FLEXCAN1_RST())
                .field("USB_HS_RST", &self.USB_HS_RST())
                .field("USB_HS_PHY_RST", &self.USB_HS_PHY_RST())
                .field("TIMER3_RST", &self.TIMER3_RST())
                .field("TIMER4_RST", &self.TIMER4_RST())
                .field("PUF_RST", &self.PUF_RST())
                .field("PKC_RST", &self.PKC_RST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PRESETCTRL2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PRESETCTRL2 {
                DMA1_RST: bool,
                FLEXIO_RST: bool,
                SAI0_RST: bool,
                SAI1_RST: bool,
                TRO_RST: bool,
                FREQME_RST: bool,
                FLEXCAN0_RST: bool,
                FLEXCAN1_RST: bool,
                USB_HS_RST: bool,
                USB_HS_PHY_RST: bool,
                TIMER3_RST: bool,
                TIMER4_RST: bool,
                PUF_RST: bool,
                PKC_RST: bool,
            }
            let proxy = PRESETCTRL2 {
                DMA1_RST: self.DMA1_RST(),
                FLEXIO_RST: self.FLEXIO_RST(),
                SAI0_RST: self.SAI0_RST(),
                SAI1_RST: self.SAI1_RST(),
                TRO_RST: self.TRO_RST(),
                FREQME_RST: self.FREQME_RST(),
                FLEXCAN0_RST: self.FLEXCAN0_RST(),
                FLEXCAN1_RST: self.FLEXCAN1_RST(),
                USB_HS_RST: self.USB_HS_RST(),
                USB_HS_PHY_RST: self.USB_HS_PHY_RST(),
                TIMER3_RST: self.TIMER3_RST(),
                TIMER4_RST: self.TIMER4_RST(),
                PUF_RST: self.PUF_RST(),
                PKC_RST: self.PKC_RST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Reset Control 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PRESETCTRL3(pub u32);
    impl PRESETCTRL3 {
        #[inline(always)]
        pub const fn I3C0_RST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn I3C1_RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_I3C1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn QDC0_RST(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn QDC1_RST(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_QDC1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PWM0_RST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PWM1_RST(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWM1_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn AOI0_RST(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AOI0_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VREF_RST(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VREF_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EWM_RST(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EWM_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn EIM_RST(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EIM_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PRESETCTRL3 {
        #[inline(always)]
        fn default() -> PRESETCTRL3 {
            PRESETCTRL3(0)
        }
    }
    impl core::fmt::Debug for PRESETCTRL3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PRESETCTRL3")
                .field("I3C0_RST", &self.I3C0_RST())
                .field("I3C1_RST", &self.I3C1_RST())
                .field("QDC0_RST", &self.QDC0_RST())
                .field("QDC1_RST", &self.QDC1_RST())
                .field("PWM0_RST", &self.PWM0_RST())
                .field("PWM1_RST", &self.PWM1_RST())
                .field("AOI0_RST", &self.AOI0_RST())
                .field("VREF_RST", &self.VREF_RST())
                .field("EWM_RST", &self.EWM_RST())
                .field("EIM_RST", &self.EIM_RST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PRESETCTRL3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PRESETCTRL3 {
                I3C0_RST: bool,
                I3C1_RST: bool,
                QDC0_RST: bool,
                QDC1_RST: bool,
                PWM0_RST: bool,
                PWM1_RST: bool,
                AOI0_RST: bool,
                VREF_RST: bool,
                EWM_RST: bool,
                EIM_RST: bool,
            }
            let proxy = PRESETCTRL3 {
                I3C0_RST: self.I3C0_RST(),
                I3C1_RST: self.I3C1_RST(),
                QDC0_RST: self.QDC0_RST(),
                QDC1_RST: self.QDC1_RST(),
                PWM0_RST: self.PWM0_RST(),
                PWM1_RST: self.PWM1_RST(),
                AOI0_RST: self.AOI0_RST(),
                VREF_RST: self.VREF_RST(),
                EWM_RST: self.EWM_RST(),
                EIM_RST: self.EIM_RST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 Submodule Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM0SUBCTL(pub u32);
    impl PWM0SUBCTL {
        #[inline(always)]
        pub const fn CLK0_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK0_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLK1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLK2_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLK3_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK3_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DMAVALM0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DMAVALM1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DMAVALM2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DMAVALM3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PWM0SUBCTL {
        #[inline(always)]
        fn default() -> PWM0SUBCTL {
            PWM0SUBCTL(0)
        }
    }
    impl core::fmt::Debug for PWM0SUBCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM0SUBCTL")
                .field("CLK0_EN", &self.CLK0_EN())
                .field("CLK1_EN", &self.CLK1_EN())
                .field("CLK2_EN", &self.CLK2_EN())
                .field("CLK3_EN", &self.CLK3_EN())
                .field("DMAVALM0", &self.DMAVALM0())
                .field("DMAVALM1", &self.DMAVALM1())
                .field("DMAVALM2", &self.DMAVALM2())
                .field("DMAVALM3", &self.DMAVALM3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM0SUBCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWM0SUBCTL {
                CLK0_EN: bool,
                CLK1_EN: bool,
                CLK2_EN: bool,
                CLK3_EN: bool,
                DMAVALM0: bool,
                DMAVALM1: bool,
                DMAVALM2: bool,
                DMAVALM3: bool,
            }
            let proxy = PWM0SUBCTL {
                CLK0_EN: self.CLK0_EN(),
                CLK1_EN: self.CLK1_EN(),
                CLK2_EN: self.CLK2_EN(),
                CLK3_EN: self.CLK3_EN(),
                DMAVALM0: self.DMAVALM0(),
                DMAVALM1: self.DMAVALM1(),
                DMAVALM2: self.DMAVALM2(),
                DMAVALM3: self.DMAVALM3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 Submodule Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM1SUBCTL(pub u32);
    impl PWM1SUBCTL {
        #[inline(always)]
        pub const fn CLK0_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK0_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLK1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLK2_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLK3_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK3_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DMAVALM0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DMAVALM1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DMAVALM2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DMAVALM3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAVALM3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PWM1SUBCTL {
        #[inline(always)]
        fn default() -> PWM1SUBCTL {
            PWM1SUBCTL(0)
        }
    }
    impl core::fmt::Debug for PWM1SUBCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM1SUBCTL")
                .field("CLK0_EN", &self.CLK0_EN())
                .field("CLK1_EN", &self.CLK1_EN())
                .field("CLK2_EN", &self.CLK2_EN())
                .field("CLK3_EN", &self.CLK3_EN())
                .field("DMAVALM0", &self.DMAVALM0())
                .field("DMAVALM1", &self.DMAVALM1())
                .field("DMAVALM2", &self.DMAVALM2())
                .field("DMAVALM3", &self.DMAVALM3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM1SUBCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWM1SUBCTL {
                CLK0_EN: bool,
                CLK1_EN: bool,
                CLK2_EN: bool,
                CLK3_EN: bool,
                DMAVALM0: bool,
                DMAVALM1: bool,
                DMAVALM2: bool,
                DMAVALM3: bool,
            }
            let proxy = PWM1SUBCTL {
                CLK0_EN: self.CLK0_EN(),
                CLK1_EN: self.CLK1_EN(),
                CLK2_EN: self.CLK2_EN(),
                CLK3_EN: self.CLK3_EN(),
                DMAVALM0: self.DMAVALM0(),
                DMAVALM1: self.DMAVALM1(),
                DMAVALM2: self.DMAVALM2(),
                DMAVALM3: self.DMAVALM3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control PKC RAM Interleave Access"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAM_INTERLEAVE(pub u32);
    impl RAM_INTERLEAVE {
        #[inline(always)]
        pub const fn INTERLEAVE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTERLEAVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RAM_INTERLEAVE {
        #[inline(always)]
        fn default() -> RAM_INTERLEAVE {
            RAM_INTERLEAVE(0)
        }
    }
    impl core::fmt::Debug for RAM_INTERLEAVE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAM_INTERLEAVE")
                .field("INTERLEAVE", &self.INTERLEAVE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAM_INTERLEAVE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RAM_INTERLEAVE {
                INTERLEAVE: bool,
            }
            let proxy = RAM_INTERLEAVE {
                INTERLEAVE: self.INTERLEAVE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO 48MHz Reference Clock Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REF_CLK_CTRL(pub u32);
    impl REF_CLK_CTRL {
        #[inline(always)]
        pub const fn GDET_REFCLK_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET_REFCLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TRNG_REFCLK_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG_REFCLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for REF_CLK_CTRL {
        #[inline(always)]
        fn default() -> REF_CLK_CTRL {
            REF_CLK_CTRL(0)
        }
    }
    impl core::fmt::Debug for REF_CLK_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REF_CLK_CTRL")
                .field("GDET_REFCLK_EN", &self.GDET_REFCLK_EN())
                .field("TRNG_REFCLK_EN", &self.TRNG_REFCLK_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REF_CLK_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REF_CLK_CTRL {
                GDET_REFCLK_EN: bool,
                TRNG_REFCLK_EN: bool,
            }
            let proxy = REF_CLK_CTRL {
                GDET_REFCLK_EN: self.GDET_REFCLK_EN(),
                TRNG_REFCLK_EN: self.TRNG_REFCLK_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO 48MHz Reference Clock Control Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REF_CLK_CTRL_CLR(pub u32);
    impl REF_CLK_CTRL_CLR {
        #[inline(always)]
        pub const fn GDET_REFCLK_EN_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET_REFCLK_EN_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TRNG_REFCLK_EN_CLR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG_REFCLK_EN_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for REF_CLK_CTRL_CLR {
        #[inline(always)]
        fn default() -> REF_CLK_CTRL_CLR {
            REF_CLK_CTRL_CLR(0)
        }
    }
    impl core::fmt::Debug for REF_CLK_CTRL_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REF_CLK_CTRL_CLR")
                .field("GDET_REFCLK_EN_CLR", &self.GDET_REFCLK_EN_CLR())
                .field("TRNG_REFCLK_EN_CLR", &self.TRNG_REFCLK_EN_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REF_CLK_CTRL_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REF_CLK_CTRL_CLR {
                GDET_REFCLK_EN_CLR: bool,
                TRNG_REFCLK_EN_CLR: bool,
            }
            let proxy = REF_CLK_CTRL_CLR {
                GDET_REFCLK_EN_CLR: self.GDET_REFCLK_EN_CLR(),
                TRNG_REFCLK_EN_CLR: self.TRNG_REFCLK_EN_CLR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO 48MHz Reference Clock Control Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REF_CLK_CTRL_SET(pub u32);
    impl REF_CLK_CTRL_SET {
        #[inline(always)]
        pub const fn GDET_REFCLK_EN_SET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GDET_REFCLK_EN_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TRNG_REFCLK_EN_SET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG_REFCLK_EN_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for REF_CLK_CTRL_SET {
        #[inline(always)]
        fn default() -> REF_CLK_CTRL_SET {
            REF_CLK_CTRL_SET(0)
        }
    }
    impl core::fmt::Debug for REF_CLK_CTRL_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REF_CLK_CTRL_SET")
                .field("GDET_REFCLK_EN_SET", &self.GDET_REFCLK_EN_SET())
                .field("TRNG_REFCLK_EN_SET", &self.TRNG_REFCLK_EN_SET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REF_CLK_CTRL_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REF_CLK_CTRL_SET {
                GDET_REFCLK_EN_SET: bool,
                TRNG_REFCLK_EN_SET: bool,
            }
            let proxy = REF_CLK_CTRL_SET {
                GDET_REFCLK_EN_SET: self.GDET_REFCLK_EN_SET(),
                TRNG_REFCLK_EN_SET: self.TRNG_REFCLK_EN_SET(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ROM Wait State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ROMCR(pub u32);
    impl ROMCR {
        #[inline(always)]
        pub const fn ROM_WAIT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROM_WAIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ROMCR {
        #[inline(always)]
        fn default() -> ROMCR {
            ROMCR(0)
        }
    }
    impl core::fmt::Debug for ROMCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ROMCR")
                .field("ROM_WAIT", &self.ROM_WAIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ROMCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ROMCR {
                ROM_WAIT: bool,
            }
            let proxy = ROMCR {
                ROM_WAIT: self.ROM_WAIT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SAI0 Function Clock Division"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SAI0CLKDIV(pub u32);
    impl SAI0CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for SAI0CLKDIV {
        #[inline(always)]
        fn default() -> SAI0CLKDIV {
            SAI0CLKDIV(0)
        }
    }
    impl core::fmt::Debug for SAI0CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SAI0CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SAI0CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SAI0CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = SAI0CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SAI0 Function Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SAI0CLKSEL(pub u32);
    impl SAI0CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SAI0CLKSEL {
        #[inline(always)]
        fn default() -> SAI0CLKSEL {
            SAI0CLKSEL(0)
        }
    }
    impl core::fmt::Debug for SAI0CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SAI0CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SAI0CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SAI0CLKSEL {
                SEL: u8,
            }
            let proxy = SAI0CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SAI1 Function Clock Division"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SAI1CLKDIV(pub u32);
    impl SAI1CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
    impl Default for SAI1CLKDIV {
        #[inline(always)]
        fn default() -> SAI1CLKDIV {
            SAI1CLKDIV(0)
        }
    }
    impl core::fmt::Debug for SAI1CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SAI1CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SAI1CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SAI1CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = SAI1CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SAI1 Function Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SAI1CLKSEL(pub u32);
    impl SAI1CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SAI1CLKSEL {
        #[inline(always)]
        fn default() -> SAI1CLKSEL {
            SAI1CLKSEL(0)
        }
    }
    impl core::fmt::Debug for SAI1CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SAI1CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SAI1CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SAI1CLKSEL {
                SEL: u8,
            }
            let proxy = SAI1CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SLOWCLKDIV(pub u32);
    impl SLOWCLKDIV {
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
    impl Default for SLOWCLKDIV {
        #[inline(always)]
        fn default() -> SLOWCLKDIV {
            SLOWCLKDIV(0)
        }
    }
    impl core::fmt::Debug for SLOWCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SLOWCLKDIV")
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SLOWCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SLOWCLKDIV {
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = SLOWCLKDIV {
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SmartDMA Interrupt Hijack"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMARTDMAINT(pub u32);
    impl SMARTDMAINT {
        #[inline(always)]
        pub const fn INT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn INT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn INT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn INT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn INT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn INT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn INT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn INT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn INT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn INT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn INT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn INT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn INT16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn INT17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn INT18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn INT19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn INT20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn INT21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn INT22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn INT23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SMARTDMAINT {
        #[inline(always)]
        fn default() -> SMARTDMAINT {
            SMARTDMAINT(0)
        }
    }
    impl core::fmt::Debug for SMARTDMAINT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMARTDMAINT")
                .field("INT0", &self.INT0())
                .field("INT1", &self.INT1())
                .field("INT2", &self.INT2())
                .field("INT3", &self.INT3())
                .field("INT4", &self.INT4())
                .field("INT5", &self.INT5())
                .field("INT6", &self.INT6())
                .field("INT7", &self.INT7())
                .field("INT8", &self.INT8())
                .field("INT9", &self.INT9())
                .field("INT10", &self.INT10())
                .field("INT11", &self.INT11())
                .field("INT12", &self.INT12())
                .field("INT13", &self.INT13())
                .field("INT14", &self.INT14())
                .field("INT15", &self.INT15())
                .field("INT16", &self.INT16())
                .field("INT17", &self.INT17())
                .field("INT18", &self.INT18())
                .field("INT19", &self.INT19())
                .field("INT20", &self.INT20())
                .field("INT21", &self.INT21())
                .field("INT22", &self.INT22())
                .field("INT23", &self.INT23())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMARTDMAINT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SMARTDMAINT {
                INT0: bool,
                INT1: bool,
                INT2: bool,
                INT3: bool,
                INT4: bool,
                INT5: bool,
                INT6: bool,
                INT7: bool,
                INT8: bool,
                INT9: bool,
                INT10: bool,
                INT11: bool,
                INT12: bool,
                INT13: bool,
                INT14: bool,
                INT15: bool,
                INT16: bool,
                INT17: bool,
                INT18: bool,
                INT19: bool,
                INT20: bool,
                INT21: bool,
                INT22: bool,
                INT23: bool,
            }
            let proxy = SMARTDMAINT {
                INT0: self.INT0(),
                INT1: self.INT1(),
                INT2: self.INT2(),
                INT3: self.INT3(),
                INT4: self.INT4(),
                INT5: self.INT5(),
                INT6: self.INT6(),
                INT7: self.INT7(),
                INT8: self.INT8(),
                INT9: self.INT9(),
                INT10: self.INT10(),
                INT11: self.INT11(),
                INT12: self.INT12(),
                INT13: self.INT13(),
                INT14: self.INT14(),
                INT15: self.INT15(),
                INT16: self.INT16(),
                INT17: self.INT17(),
                INT18: self.INT18(),
                INT19: self.INT19(),
                INT20: self.INT20(),
                INT21: self.INT21(),
                INT22: self.INT22(),
                INT23: self.INT23(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CPU0 System Tick Timer Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYSTICKCLKDIV(pub u32);
    impl SYSTICKCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for SYSTICKCLKDIV {
        #[inline(always)]
        fn default() -> SYSTICKCLKDIV {
            SYSTICKCLKDIV(0)
        }
    }
    impl core::fmt::Debug for SYSTICKCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYSTICKCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYSTICKCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYSTICKCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = SYSTICKCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CPU0 System Tick Timer Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYSTICKCLKSEL0(pub u32);
    impl SYSTICKCLKSEL0 {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SYSTICKCLKSEL0 {
        #[inline(always)]
        fn default() -> SYSTICKCLKSEL0 {
            SYSTICKCLKSEL0(0)
        }
    }
    impl core::fmt::Debug for SYSTICKCLKSEL0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYSTICKCLKSEL0")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYSTICKCLKSEL0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYSTICKCLKSEL0 {
                SEL: u8,
            }
            let proxy = SYSTICKCLKSEL0 { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TRACE Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRACECLKDIV(pub u32);
    impl TRACECLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    impl Default for TRACECLKDIV {
        #[inline(always)]
        fn default() -> TRACECLKDIV {
            TRACECLKDIV(0)
        }
    }
    impl core::fmt::Debug for TRACECLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRACECLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRACECLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRACECLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = TRACECLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trace Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRACECLKSEL(pub u32);
    impl TRACECLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for TRACECLKSEL {
        #[inline(always)]
        fn default() -> TRACECLKSEL {
            TRACECLKSEL(0)
        }
    }
    impl core::fmt::Debug for TRACECLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRACECLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRACECLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRACECLKSEL {
                SEL: u8,
            }
            let proxy = TRACECLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "UTICK Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UTICKCLKDIV(pub u32);
    impl UTICKCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
    impl Default for UTICKCLKDIV {
        #[inline(always)]
        fn default() -> UTICKCLKDIV {
            UTICKCLKDIV(0)
        }
    }
    impl core::fmt::Debug for UTICKCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UTICKCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UTICKCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UTICKCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = UTICKCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "UTICK Function Clock Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UTICKCLKSEL(pub u32);
    impl UTICKCLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for UTICKCLKSEL {
        #[inline(always)]
        fn default() -> UTICKCLKSEL {
            UTICKCLKSEL(0)
        }
    }
    impl core::fmt::Debug for UTICKCLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UTICKCLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UTICKCLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct UTICKCLKSEL {
                SEL: u8,
            }
            let proxy = UTICKCLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "WDT0 Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WDT0CLKDIV(pub u32);
    impl WDT0CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
    impl Default for WDT0CLKDIV {
        #[inline(always)]
        fn default() -> WDT0CLKDIV {
            WDT0CLKDIV(0)
        }
    }
    impl core::fmt::Debug for WDT0CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WDT0CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WDT0CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WDT0CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = WDT0CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "WDT1 Function Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WDT1CLKDIV(pub u32);
    impl WDT1CLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
    impl Default for WDT1CLKDIV {
        #[inline(always)]
        fn default() -> WDT1CLKDIV {
            WDT1CLKDIV(0)
        }
    }
    impl core::fmt::Debug for WDT1CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WDT1CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WDT1CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WDT1CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = WDT1CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "WDT1 Clock Selection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WDT1CLKSEL(pub u32);
    impl WDT1CLKSEL {
        #[inline(always)]
        pub const fn SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for WDT1CLKSEL {
        #[inline(always)]
        fn default() -> WDT1CLKSEL {
            WDT1CLKSEL(0)
        }
    }
    impl core::fmt::Debug for WDT1CLKSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WDT1CLKSEL")
                .field("SEL", &self.SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WDT1CLKSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WDT1CLKSEL {
                SEL: u8,
            }
            let proxy = WDT1CLKSEL { SEL: self.SEL() };
            defmt::write!(f, "{}", proxy)
        }
    }
}
