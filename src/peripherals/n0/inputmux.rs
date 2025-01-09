#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INPUTMUX {
    ptr: *mut u8,
}
unsafe impl Send for INPUTMUX {}
unsafe impl Sync for INPUTMUX {}
impl INPUTMUX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SCT0_INMUX(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER0CAP0(self) -> crate::common::Reg<regs::CTIMER0CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER0CAP1(self) -> crate::common::Reg<regs::CTIMER0CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER0CAP2(self) -> crate::common::Reg<regs::CTIMER0CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER0CAP3(self) -> crate::common::Reg<regs::CTIMER0CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER0TRIG(self) -> crate::common::Reg<regs::TIMER0TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP0(self) -> crate::common::Reg<regs::CTIMER1CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP1(self) -> crate::common::Reg<regs::CTIMER1CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP2(self) -> crate::common::Reg<regs::CTIMER1CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP3(self) -> crate::common::Reg<regs::CTIMER1CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER1TRIG(self) -> crate::common::Reg<regs::TIMER1TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP0(self) -> crate::common::Reg<regs::CTIMER2CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP1(self) -> crate::common::Reg<regs::CTIMER2CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP2(self) -> crate::common::Reg<regs::CTIMER2CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP3(self) -> crate::common::Reg<regs::CTIMER2CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER2TRIG(self) -> crate::common::Reg<regs::TIMER2TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn SMARTDMAARCHB_INMUX(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn PINTSEL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FREQMEAS_REF(self) -> crate::common::Reg<regs::FREQMEAS_REF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn FREQMEAS_TAR(self) -> crate::common::Reg<regs::FREQMEAS_TAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER3CAP0(self) -> crate::common::Reg<regs::CTIMER3CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER3CAP1(self) -> crate::common::Reg<regs::CTIMER3CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER3CAP2(self) -> crate::common::Reg<regs::CTIMER3CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER3CAP3(self) -> crate::common::Reg<regs::CTIMER3CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER3TRIG(self) -> crate::common::Reg<regs::TIMER3TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER4CAP0(self) -> crate::common::Reg<regs::CTIMER4CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER4CAP1(self) -> crate::common::Reg<regs::CTIMER4CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER4CAP2(self) -> crate::common::Reg<regs::CTIMER4CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER4CAP3(self) -> crate::common::Reg<regs::CTIMER4CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER4TRIG(self) -> crate::common::Reg<regs::TIMER4TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP0_TRIG(self) -> crate::common::Reg<regs::CMP0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC0_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC1_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn DAC0_TRIG(self) -> crate::common::Reg<regs::DAC0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn DAC1_TRIG(self) -> crate::common::Reg<regs::DAC1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[inline(always)]
    pub const fn DAC2_TRIG(self) -> crate::common::Reg<regs::DAC2_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[inline(always)]
    pub const fn QDCN(self, n: usize) -> QDCN {
        assert!(n < 2usize);
        unsafe { QDCN::from_ptr(self.ptr.add(0x0360usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM_EXTSYNC(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM_EXTA(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_EXTFORCE(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_EXTFORCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM_EXTSYNC(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM_EXTA(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_EXTFORCE(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_EXTFORCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_FAULT(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLEXPWM1_FAULT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM0_EXT_CLK(self) -> crate::common::Reg<regs::PWM0_EXT_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM1_EXT_CLK(self) -> crate::common::Reg<regs::PWM1_EXT_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn USBFS_TRIG(self) -> crate::common::Reg<regs::USBFS_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[inline(always)]
    pub const fn TSI_TRIG(self) -> crate::common::Reg<regs::TSI_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[inline(always)]
    pub const fn EXT_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1_TRIG(self) -> crate::common::Reg<regs::CMP1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP2_TRIG(self) -> crate::common::Reg<regs::CMP2_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn SINC_FILTER_CH(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn OPAMP_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM0_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM1_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM2_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM2_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM3_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM3_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM4_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM4_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM5_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM5_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM6_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM6_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM7_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM7_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM8_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM8_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXCOMM9_TRIG(
        self,
    ) -> crate::common::Reg<regs::FLEXCOMM9_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXIO_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE0(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE0_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE0_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE0_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE0_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE0_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE0_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE1(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE1_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE1_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0714usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE1_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE1_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0718usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE1_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE1_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x071cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE2(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE2_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE2_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0724usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE2_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE2_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0728usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE2_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE2_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x072cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE3(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0730usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE3_SET(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE3_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0734usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA0_REQ_ENABLE3_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA0_REQ_ENABLE3_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0738usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE0(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE0_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE0_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0784usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE0_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE0_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0788usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE0_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE0_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x078cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE1(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0790usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE1_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE1_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0794usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE1_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE1_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0798usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE1_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE1_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x079cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE2(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a0usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE2_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE2_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a4usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE2_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE2_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a8usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE2_TOG(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE2_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07acusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE3(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b0usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE3_SET(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE3_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b4usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA1_REQ_ENABLE3_CLR(
        self,
    ) -> crate::common::Reg<regs::DMA1_REQ_ENABLE3_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QDCN {
    ptr: *mut u8,
}
unsafe impl Send for QDCN {}
unsafe impl Sync for QDCN {}
impl QDCN {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn QDC_TRIG(self) -> crate::common::Reg<regs::QDCN_QDC_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC_HOME(self) -> crate::common::Reg<regs::QDCN_QDC_HOME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC_INDEX(self) -> crate::common::Reg<regs::QDCN_QDC_INDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC_PHASEB(self) -> crate::common::Reg<regs::QDCN_QDC_PHASEB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn QDC_PHASEA(self) -> crate::common::Reg<regs::QDCN_QDC_PHASEA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC0_TRIG(pub u32);
    impl ADC0_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ADC0_TRIG {
        #[inline(always)]
        fn default() -> ADC0_TRIG {
            ADC0_TRIG(0)
        }
    }
    impl core::fmt::Debug for ADC0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC0_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC0_TRIG {
                TRIGIN: u8,
            }
            let proxy = ADC0_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ADC Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC1_TRIG(pub u32);
    impl ADC1_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ADC1_TRIG {
        #[inline(always)]
        fn default() -> ADC1_TRIG {
            ADC1_TRIG(0)
        }
    }
    impl core::fmt::Debug for ADC1_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADC1_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADC1_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADC1_TRIG {
                TRIGIN: u8,
            }
            let proxy = ADC1_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP0 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP0_TRIG(pub u32);
    impl CMP0_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for CMP0_TRIG {
        #[inline(always)]
        fn default() -> CMP0_TRIG {
            CMP0_TRIG(0)
        }
    }
    impl core::fmt::Debug for CMP0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP0_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP0_TRIG {
                TRIGIN: u8,
            }
            let proxy = CMP0_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP1 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP1_TRIG(pub u32);
    impl CMP1_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for CMP1_TRIG {
        #[inline(always)]
        fn default() -> CMP1_TRIG {
            CMP1_TRIG(0)
        }
    }
    impl core::fmt::Debug for CMP1_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP1_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP1_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP1_TRIG {
                TRIGIN: u8,
            }
            let proxy = CMP1_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CMP2 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMP2_TRIG(pub u32);
    impl CMP2_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for CMP2_TRIG {
        #[inline(always)]
        fn default() -> CMP2_TRIG {
            CMP2_TRIG(0)
        }
    }
    impl core::fmt::Debug for CMP2_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMP2_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMP2_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMP2_TRIG {
                TRIGIN: u8,
            }
            let proxy = CMP2_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER0CAP0(pub u32);
    impl CTIMER0CAP0 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER0CAP0 {
        #[inline(always)]
        fn default() -> CTIMER0CAP0 {
            CTIMER0CAP0(0)
        }
    }
    impl core::fmt::Debug for CTIMER0CAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER0CAP0")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER0CAP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER0CAP0 {
                INP: u8,
            }
            let proxy = CTIMER0CAP0 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER0CAP1(pub u32);
    impl CTIMER0CAP1 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER0CAP1 {
        #[inline(always)]
        fn default() -> CTIMER0CAP1 {
            CTIMER0CAP1(0)
        }
    }
    impl core::fmt::Debug for CTIMER0CAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER0CAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER0CAP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER0CAP1 {
                INP: u8,
            }
            let proxy = CTIMER0CAP1 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER0CAP2(pub u32);
    impl CTIMER0CAP2 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER0CAP2 {
        #[inline(always)]
        fn default() -> CTIMER0CAP2 {
            CTIMER0CAP2(0)
        }
    }
    impl core::fmt::Debug for CTIMER0CAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER0CAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER0CAP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER0CAP2 {
                INP: u8,
            }
            let proxy = CTIMER0CAP2 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER0CAP3(pub u32);
    impl CTIMER0CAP3 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER0CAP3 {
        #[inline(always)]
        fn default() -> CTIMER0CAP3 {
            CTIMER0CAP3(0)
        }
    }
    impl core::fmt::Debug for CTIMER0CAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER0CAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER0CAP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER0CAP3 {
                INP: u8,
            }
            let proxy = CTIMER0CAP3 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER1CAP0(pub u32);
    impl CTIMER1CAP0 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER1CAP0 {
        #[inline(always)]
        fn default() -> CTIMER1CAP0 {
            CTIMER1CAP0(0)
        }
    }
    impl core::fmt::Debug for CTIMER1CAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER1CAP0")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER1CAP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER1CAP0 {
                INP: u8,
            }
            let proxy = CTIMER1CAP0 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER1CAP1(pub u32);
    impl CTIMER1CAP1 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER1CAP1 {
        #[inline(always)]
        fn default() -> CTIMER1CAP1 {
            CTIMER1CAP1(0)
        }
    }
    impl core::fmt::Debug for CTIMER1CAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER1CAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER1CAP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER1CAP1 {
                INP: u8,
            }
            let proxy = CTIMER1CAP1 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER1CAP2(pub u32);
    impl CTIMER1CAP2 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER1CAP2 {
        #[inline(always)]
        fn default() -> CTIMER1CAP2 {
            CTIMER1CAP2(0)
        }
    }
    impl core::fmt::Debug for CTIMER1CAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER1CAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER1CAP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER1CAP2 {
                INP: u8,
            }
            let proxy = CTIMER1CAP2 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER1CAP3(pub u32);
    impl CTIMER1CAP3 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER1CAP3 {
        #[inline(always)]
        fn default() -> CTIMER1CAP3 {
            CTIMER1CAP3(0)
        }
    }
    impl core::fmt::Debug for CTIMER1CAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER1CAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER1CAP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER1CAP3 {
                INP: u8,
            }
            let proxy = CTIMER1CAP3 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER2CAP0(pub u32);
    impl CTIMER2CAP0 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER2CAP0 {
        #[inline(always)]
        fn default() -> CTIMER2CAP0 {
            CTIMER2CAP0(0)
        }
    }
    impl core::fmt::Debug for CTIMER2CAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER2CAP0")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER2CAP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER2CAP0 {
                INP: u8,
            }
            let proxy = CTIMER2CAP0 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER2CAP1(pub u32);
    impl CTIMER2CAP1 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER2CAP1 {
        #[inline(always)]
        fn default() -> CTIMER2CAP1 {
            CTIMER2CAP1(0)
        }
    }
    impl core::fmt::Debug for CTIMER2CAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER2CAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER2CAP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER2CAP1 {
                INP: u8,
            }
            let proxy = CTIMER2CAP1 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER2CAP2(pub u32);
    impl CTIMER2CAP2 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER2CAP2 {
        #[inline(always)]
        fn default() -> CTIMER2CAP2 {
            CTIMER2CAP2(0)
        }
    }
    impl core::fmt::Debug for CTIMER2CAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER2CAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER2CAP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER2CAP2 {
                INP: u8,
            }
            let proxy = CTIMER2CAP2 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER2CAP3(pub u32);
    impl CTIMER2CAP3 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER2CAP3 {
        #[inline(always)]
        fn default() -> CTIMER2CAP3 {
            CTIMER2CAP3(0)
        }
    }
    impl core::fmt::Debug for CTIMER2CAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER2CAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER2CAP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER2CAP3 {
                INP: u8,
            }
            let proxy = CTIMER2CAP3 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER3CAP0(pub u32);
    impl CTIMER3CAP0 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER3CAP0 {
        #[inline(always)]
        fn default() -> CTIMER3CAP0 {
            CTIMER3CAP0(0)
        }
    }
    impl core::fmt::Debug for CTIMER3CAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER3CAP0")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER3CAP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER3CAP0 {
                INP: u8,
            }
            let proxy = CTIMER3CAP0 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER3CAP1(pub u32);
    impl CTIMER3CAP1 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER3CAP1 {
        #[inline(always)]
        fn default() -> CTIMER3CAP1 {
            CTIMER3CAP1(0)
        }
    }
    impl core::fmt::Debug for CTIMER3CAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER3CAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER3CAP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER3CAP1 {
                INP: u8,
            }
            let proxy = CTIMER3CAP1 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER3CAP2(pub u32);
    impl CTIMER3CAP2 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER3CAP2 {
        #[inline(always)]
        fn default() -> CTIMER3CAP2 {
            CTIMER3CAP2(0)
        }
    }
    impl core::fmt::Debug for CTIMER3CAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER3CAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER3CAP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER3CAP2 {
                INP: u8,
            }
            let proxy = CTIMER3CAP2 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER3CAP3(pub u32);
    impl CTIMER3CAP3 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER3CAP3 {
        #[inline(always)]
        fn default() -> CTIMER3CAP3 {
            CTIMER3CAP3(0)
        }
    }
    impl core::fmt::Debug for CTIMER3CAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER3CAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER3CAP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER3CAP3 {
                INP: u8,
            }
            let proxy = CTIMER3CAP3 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER4CAP0(pub u32);
    impl CTIMER4CAP0 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER4CAP0 {
        #[inline(always)]
        fn default() -> CTIMER4CAP0 {
            CTIMER4CAP0(0)
        }
    }
    impl core::fmt::Debug for CTIMER4CAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER4CAP0")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER4CAP0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER4CAP0 {
                INP: u8,
            }
            let proxy = CTIMER4CAP0 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER4CAP1(pub u32);
    impl CTIMER4CAP1 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER4CAP1 {
        #[inline(always)]
        fn default() -> CTIMER4CAP1 {
            CTIMER4CAP1(0)
        }
    }
    impl core::fmt::Debug for CTIMER4CAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER4CAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER4CAP1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER4CAP1 {
                INP: u8,
            }
            let proxy = CTIMER4CAP1 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER4CAP2(pub u32);
    impl CTIMER4CAP2 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER4CAP2 {
        #[inline(always)]
        fn default() -> CTIMER4CAP2 {
            CTIMER4CAP2(0)
        }
    }
    impl core::fmt::Debug for CTIMER4CAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER4CAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER4CAP2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER4CAP2 {
                INP: u8,
            }
            let proxy = CTIMER4CAP2 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER4CAP3(pub u32);
    impl CTIMER4CAP3 {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CTIMER4CAP3 {
        #[inline(always)]
        fn default() -> CTIMER4CAP3 {
            CTIMER4CAP3(0)
        }
    }
    impl core::fmt::Debug for CTIMER4CAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER4CAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER4CAP3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTIMER4CAP3 {
                INP: u8,
            }
            let proxy = CTIMER4CAP3 { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC0 Trigger Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DAC0_TRIG(pub u32);
    impl DAC0_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for DAC0_TRIG {
        #[inline(always)]
        fn default() -> DAC0_TRIG {
            DAC0_TRIG(0)
        }
    }
    impl core::fmt::Debug for DAC0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DAC0_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DAC0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DAC0_TRIG {
                TRIGIN: u8,
            }
            let proxy = DAC0_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC1 Trigger Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DAC1_TRIG(pub u32);
    impl DAC1_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for DAC1_TRIG {
        #[inline(always)]
        fn default() -> DAC1_TRIG {
            DAC1_TRIG(0)
        }
    }
    impl core::fmt::Debug for DAC1_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DAC1_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DAC1_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DAC1_TRIG {
                TRIGIN: u8,
            }
            let proxy = DAC1_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC2 Trigger Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DAC2_TRIG(pub u32);
    impl DAC2_TRIG {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for DAC2_TRIG {
        #[inline(always)]
        fn default() -> DAC2_TRIG {
            DAC2_TRIG(0)
        }
    }
    impl core::fmt::Debug for DAC2_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DAC2_TRIG")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DAC2_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DAC2_TRIG {
                TRIGIN: u8,
            }
            let proxy = DAC2_TRIG {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE0(pub u32);
    impl DMA0_REQ_ENABLE0 {
        #[inline(always)]
        pub const fn REQ1_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE0 {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE0 {
            DMA0_REQ_ENABLE0(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE0")
                .field("REQ1_EN0", &self.REQ1_EN0())
                .field("REQ2_EN0", &self.REQ2_EN0())
                .field("REQ3_EN0", &self.REQ3_EN0())
                .field("REQ4_EN0", &self.REQ4_EN0())
                .field("REQ5_EN0", &self.REQ5_EN0())
                .field("REQ6_EN0", &self.REQ6_EN0())
                .field("REQ7_EN0", &self.REQ7_EN0())
                .field("REQ8_EN0", &self.REQ8_EN0())
                .field("REQ9_EN0", &self.REQ9_EN0())
                .field("REQ10_EN0", &self.REQ10_EN0())
                .field("REQ11_EN0", &self.REQ11_EN0())
                .field("REQ12_EN0", &self.REQ12_EN0())
                .field("REQ13_EN0", &self.REQ13_EN0())
                .field("REQ14_EN0", &self.REQ14_EN0())
                .field("REQ15_EN0", &self.REQ15_EN0())
                .field("REQ16_EN0", &self.REQ16_EN0())
                .field("REQ17_EN0", &self.REQ17_EN0())
                .field("REQ18_EN0", &self.REQ18_EN0())
                .field("REQ19_EN0", &self.REQ19_EN0())
                .field("REQ20_EN0", &self.REQ20_EN0())
                .field("REQ21_EN0", &self.REQ21_EN0())
                .field("REQ22_EN0", &self.REQ22_EN0())
                .field("REQ23_EN0", &self.REQ23_EN0())
                .field("REQ24_EN0", &self.REQ24_EN0())
                .field("REQ25_EN0", &self.REQ25_EN0())
                .field("REQ26_EN0", &self.REQ26_EN0())
                .field("REQ27_EN0", &self.REQ27_EN0())
                .field("REQ28_EN0", &self.REQ28_EN0())
                .field("REQ29_EN0", &self.REQ29_EN0())
                .field("REQ30_EN0", &self.REQ30_EN0())
                .field("REQ31_EN0", &self.REQ31_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE0 {
                REQ1_EN0: bool,
                REQ2_EN0: bool,
                REQ3_EN0: bool,
                REQ4_EN0: bool,
                REQ5_EN0: bool,
                REQ6_EN0: bool,
                REQ7_EN0: bool,
                REQ8_EN0: bool,
                REQ9_EN0: bool,
                REQ10_EN0: bool,
                REQ11_EN0: bool,
                REQ12_EN0: bool,
                REQ13_EN0: bool,
                REQ14_EN0: bool,
                REQ15_EN0: bool,
                REQ16_EN0: bool,
                REQ17_EN0: bool,
                REQ18_EN0: bool,
                REQ19_EN0: bool,
                REQ20_EN0: bool,
                REQ21_EN0: bool,
                REQ22_EN0: bool,
                REQ23_EN0: bool,
                REQ24_EN0: bool,
                REQ25_EN0: bool,
                REQ26_EN0: bool,
                REQ27_EN0: bool,
                REQ28_EN0: bool,
                REQ29_EN0: bool,
                REQ30_EN0: bool,
                REQ31_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE0 {
                REQ1_EN0: self.REQ1_EN0(),
                REQ2_EN0: self.REQ2_EN0(),
                REQ3_EN0: self.REQ3_EN0(),
                REQ4_EN0: self.REQ4_EN0(),
                REQ5_EN0: self.REQ5_EN0(),
                REQ6_EN0: self.REQ6_EN0(),
                REQ7_EN0: self.REQ7_EN0(),
                REQ8_EN0: self.REQ8_EN0(),
                REQ9_EN0: self.REQ9_EN0(),
                REQ10_EN0: self.REQ10_EN0(),
                REQ11_EN0: self.REQ11_EN0(),
                REQ12_EN0: self.REQ12_EN0(),
                REQ13_EN0: self.REQ13_EN0(),
                REQ14_EN0: self.REQ14_EN0(),
                REQ15_EN0: self.REQ15_EN0(),
                REQ16_EN0: self.REQ16_EN0(),
                REQ17_EN0: self.REQ17_EN0(),
                REQ18_EN0: self.REQ18_EN0(),
                REQ19_EN0: self.REQ19_EN0(),
                REQ20_EN0: self.REQ20_EN0(),
                REQ21_EN0: self.REQ21_EN0(),
                REQ22_EN0: self.REQ22_EN0(),
                REQ23_EN0: self.REQ23_EN0(),
                REQ24_EN0: self.REQ24_EN0(),
                REQ25_EN0: self.REQ25_EN0(),
                REQ26_EN0: self.REQ26_EN0(),
                REQ27_EN0: self.REQ27_EN0(),
                REQ28_EN0: self.REQ28_EN0(),
                REQ29_EN0: self.REQ29_EN0(),
                REQ30_EN0: self.REQ30_EN0(),
                REQ31_EN0: self.REQ31_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE0_CLR(pub u32);
    impl DMA0_REQ_ENABLE0_CLR {
        #[inline(always)]
        pub const fn REQ1_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE0_CLR {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE0_CLR {
            DMA0_REQ_ENABLE0_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE0_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE0_CLR")
                .field("REQ1_EN0", &self.REQ1_EN0())
                .field("REQ2_EN0", &self.REQ2_EN0())
                .field("REQ3_EN0", &self.REQ3_EN0())
                .field("REQ4_EN0", &self.REQ4_EN0())
                .field("REQ5_EN0", &self.REQ5_EN0())
                .field("REQ6_EN0", &self.REQ6_EN0())
                .field("REQ7_EN0", &self.REQ7_EN0())
                .field("REQ8_EN0", &self.REQ8_EN0())
                .field("REQ9_EN0", &self.REQ9_EN0())
                .field("REQ10_EN0", &self.REQ10_EN0())
                .field("REQ11_EN0", &self.REQ11_EN0())
                .field("REQ12_EN0", &self.REQ12_EN0())
                .field("REQ13_EN0", &self.REQ13_EN0())
                .field("REQ14_EN0", &self.REQ14_EN0())
                .field("REQ15_EN0", &self.REQ15_EN0())
                .field("REQ16_EN0", &self.REQ16_EN0())
                .field("REQ17_EN0", &self.REQ17_EN0())
                .field("REQ18_EN0", &self.REQ18_EN0())
                .field("REQ19_EN0", &self.REQ19_EN0())
                .field("REQ20_EN0", &self.REQ20_EN0())
                .field("REQ21_EN0", &self.REQ21_EN0())
                .field("REQ22_EN0", &self.REQ22_EN0())
                .field("REQ23_EN0", &self.REQ23_EN0())
                .field("REQ24_EN0", &self.REQ24_EN0())
                .field("REQ25_EN0", &self.REQ25_EN0())
                .field("REQ26_EN0", &self.REQ26_EN0())
                .field("REQ27_EN0", &self.REQ27_EN0())
                .field("REQ28_EN0", &self.REQ28_EN0())
                .field("REQ29_EN0", &self.REQ29_EN0())
                .field("REQ30_EN0", &self.REQ30_EN0())
                .field("REQ31_EN0", &self.REQ31_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE0_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE0_CLR {
                REQ1_EN0: bool,
                REQ2_EN0: bool,
                REQ3_EN0: bool,
                REQ4_EN0: bool,
                REQ5_EN0: bool,
                REQ6_EN0: bool,
                REQ7_EN0: bool,
                REQ8_EN0: bool,
                REQ9_EN0: bool,
                REQ10_EN0: bool,
                REQ11_EN0: bool,
                REQ12_EN0: bool,
                REQ13_EN0: bool,
                REQ14_EN0: bool,
                REQ15_EN0: bool,
                REQ16_EN0: bool,
                REQ17_EN0: bool,
                REQ18_EN0: bool,
                REQ19_EN0: bool,
                REQ20_EN0: bool,
                REQ21_EN0: bool,
                REQ22_EN0: bool,
                REQ23_EN0: bool,
                REQ24_EN0: bool,
                REQ25_EN0: bool,
                REQ26_EN0: bool,
                REQ27_EN0: bool,
                REQ28_EN0: bool,
                REQ29_EN0: bool,
                REQ30_EN0: bool,
                REQ31_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE0_CLR {
                REQ1_EN0: self.REQ1_EN0(),
                REQ2_EN0: self.REQ2_EN0(),
                REQ3_EN0: self.REQ3_EN0(),
                REQ4_EN0: self.REQ4_EN0(),
                REQ5_EN0: self.REQ5_EN0(),
                REQ6_EN0: self.REQ6_EN0(),
                REQ7_EN0: self.REQ7_EN0(),
                REQ8_EN0: self.REQ8_EN0(),
                REQ9_EN0: self.REQ9_EN0(),
                REQ10_EN0: self.REQ10_EN0(),
                REQ11_EN0: self.REQ11_EN0(),
                REQ12_EN0: self.REQ12_EN0(),
                REQ13_EN0: self.REQ13_EN0(),
                REQ14_EN0: self.REQ14_EN0(),
                REQ15_EN0: self.REQ15_EN0(),
                REQ16_EN0: self.REQ16_EN0(),
                REQ17_EN0: self.REQ17_EN0(),
                REQ18_EN0: self.REQ18_EN0(),
                REQ19_EN0: self.REQ19_EN0(),
                REQ20_EN0: self.REQ20_EN0(),
                REQ21_EN0: self.REQ21_EN0(),
                REQ22_EN0: self.REQ22_EN0(),
                REQ23_EN0: self.REQ23_EN0(),
                REQ24_EN0: self.REQ24_EN0(),
                REQ25_EN0: self.REQ25_EN0(),
                REQ26_EN0: self.REQ26_EN0(),
                REQ27_EN0: self.REQ27_EN0(),
                REQ28_EN0: self.REQ28_EN0(),
                REQ29_EN0: self.REQ29_EN0(),
                REQ30_EN0: self.REQ30_EN0(),
                REQ31_EN0: self.REQ31_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE0_SET(pub u32);
    impl DMA0_REQ_ENABLE0_SET {
        #[inline(always)]
        pub const fn REQ1_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE0_SET {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE0_SET {
            DMA0_REQ_ENABLE0_SET(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE0_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE0_SET")
                .field("REQ1_EN0", &self.REQ1_EN0())
                .field("REQ2_EN0", &self.REQ2_EN0())
                .field("REQ3_EN0", &self.REQ3_EN0())
                .field("REQ4_EN0", &self.REQ4_EN0())
                .field("REQ5_EN0", &self.REQ5_EN0())
                .field("REQ6_EN0", &self.REQ6_EN0())
                .field("REQ7_EN0", &self.REQ7_EN0())
                .field("REQ8_EN0", &self.REQ8_EN0())
                .field("REQ9_EN0", &self.REQ9_EN0())
                .field("REQ10_EN0", &self.REQ10_EN0())
                .field("REQ11_EN0", &self.REQ11_EN0())
                .field("REQ12_EN0", &self.REQ12_EN0())
                .field("REQ13_EN0", &self.REQ13_EN0())
                .field("REQ14_EN0", &self.REQ14_EN0())
                .field("REQ15_EN0", &self.REQ15_EN0())
                .field("REQ16_EN0", &self.REQ16_EN0())
                .field("REQ17_EN0", &self.REQ17_EN0())
                .field("REQ18_EN0", &self.REQ18_EN0())
                .field("REQ19_EN0", &self.REQ19_EN0())
                .field("REQ20_EN0", &self.REQ20_EN0())
                .field("REQ21_EN0", &self.REQ21_EN0())
                .field("REQ22_EN0", &self.REQ22_EN0())
                .field("REQ23_EN0", &self.REQ23_EN0())
                .field("REQ24_EN0", &self.REQ24_EN0())
                .field("REQ25_EN0", &self.REQ25_EN0())
                .field("REQ26_EN0", &self.REQ26_EN0())
                .field("REQ27_EN0", &self.REQ27_EN0())
                .field("REQ28_EN0", &self.REQ28_EN0())
                .field("REQ29_EN0", &self.REQ29_EN0())
                .field("REQ30_EN0", &self.REQ30_EN0())
                .field("REQ31_EN0", &self.REQ31_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE0_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE0_SET {
                REQ1_EN0: bool,
                REQ2_EN0: bool,
                REQ3_EN0: bool,
                REQ4_EN0: bool,
                REQ5_EN0: bool,
                REQ6_EN0: bool,
                REQ7_EN0: bool,
                REQ8_EN0: bool,
                REQ9_EN0: bool,
                REQ10_EN0: bool,
                REQ11_EN0: bool,
                REQ12_EN0: bool,
                REQ13_EN0: bool,
                REQ14_EN0: bool,
                REQ15_EN0: bool,
                REQ16_EN0: bool,
                REQ17_EN0: bool,
                REQ18_EN0: bool,
                REQ19_EN0: bool,
                REQ20_EN0: bool,
                REQ21_EN0: bool,
                REQ22_EN0: bool,
                REQ23_EN0: bool,
                REQ24_EN0: bool,
                REQ25_EN0: bool,
                REQ26_EN0: bool,
                REQ27_EN0: bool,
                REQ28_EN0: bool,
                REQ29_EN0: bool,
                REQ30_EN0: bool,
                REQ31_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE0_SET {
                REQ1_EN0: self.REQ1_EN0(),
                REQ2_EN0: self.REQ2_EN0(),
                REQ3_EN0: self.REQ3_EN0(),
                REQ4_EN0: self.REQ4_EN0(),
                REQ5_EN0: self.REQ5_EN0(),
                REQ6_EN0: self.REQ6_EN0(),
                REQ7_EN0: self.REQ7_EN0(),
                REQ8_EN0: self.REQ8_EN0(),
                REQ9_EN0: self.REQ9_EN0(),
                REQ10_EN0: self.REQ10_EN0(),
                REQ11_EN0: self.REQ11_EN0(),
                REQ12_EN0: self.REQ12_EN0(),
                REQ13_EN0: self.REQ13_EN0(),
                REQ14_EN0: self.REQ14_EN0(),
                REQ15_EN0: self.REQ15_EN0(),
                REQ16_EN0: self.REQ16_EN0(),
                REQ17_EN0: self.REQ17_EN0(),
                REQ18_EN0: self.REQ18_EN0(),
                REQ19_EN0: self.REQ19_EN0(),
                REQ20_EN0: self.REQ20_EN0(),
                REQ21_EN0: self.REQ21_EN0(),
                REQ22_EN0: self.REQ22_EN0(),
                REQ23_EN0: self.REQ23_EN0(),
                REQ24_EN0: self.REQ24_EN0(),
                REQ25_EN0: self.REQ25_EN0(),
                REQ26_EN0: self.REQ26_EN0(),
                REQ27_EN0: self.REQ27_EN0(),
                REQ28_EN0: self.REQ28_EN0(),
                REQ29_EN0: self.REQ29_EN0(),
                REQ30_EN0: self.REQ30_EN0(),
                REQ31_EN0: self.REQ31_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE0_TOG(pub u32);
    impl DMA0_REQ_ENABLE0_TOG {
        #[inline(always)]
        pub const fn REQ1_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE0_TOG {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE0_TOG {
            DMA0_REQ_ENABLE0_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE0_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE0_TOG")
                .field("REQ1_EN0", &self.REQ1_EN0())
                .field("REQ2_EN0", &self.REQ2_EN0())
                .field("REQ3_EN0", &self.REQ3_EN0())
                .field("REQ4_EN0", &self.REQ4_EN0())
                .field("REQ5_EN0", &self.REQ5_EN0())
                .field("REQ6_EN0", &self.REQ6_EN0())
                .field("REQ7_EN0", &self.REQ7_EN0())
                .field("REQ8_EN0", &self.REQ8_EN0())
                .field("REQ9_EN0", &self.REQ9_EN0())
                .field("REQ10_EN0", &self.REQ10_EN0())
                .field("REQ11_EN0", &self.REQ11_EN0())
                .field("REQ12_EN0", &self.REQ12_EN0())
                .field("REQ13_EN0", &self.REQ13_EN0())
                .field("REQ14_EN0", &self.REQ14_EN0())
                .field("REQ15_EN0", &self.REQ15_EN0())
                .field("REQ16_EN0", &self.REQ16_EN0())
                .field("REQ17_EN0", &self.REQ17_EN0())
                .field("REQ18_EN0", &self.REQ18_EN0())
                .field("REQ19_EN0", &self.REQ19_EN0())
                .field("REQ20_EN0", &self.REQ20_EN0())
                .field("REQ21_EN0", &self.REQ21_EN0())
                .field("REQ22_EN0", &self.REQ22_EN0())
                .field("REQ23_EN0", &self.REQ23_EN0())
                .field("REQ24_EN0", &self.REQ24_EN0())
                .field("REQ25_EN0", &self.REQ25_EN0())
                .field("REQ26_EN0", &self.REQ26_EN0())
                .field("REQ27_EN0", &self.REQ27_EN0())
                .field("REQ28_EN0", &self.REQ28_EN0())
                .field("REQ29_EN0", &self.REQ29_EN0())
                .field("REQ30_EN0", &self.REQ30_EN0())
                .field("REQ31_EN0", &self.REQ31_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE0_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE0_TOG {
                REQ1_EN0: bool,
                REQ2_EN0: bool,
                REQ3_EN0: bool,
                REQ4_EN0: bool,
                REQ5_EN0: bool,
                REQ6_EN0: bool,
                REQ7_EN0: bool,
                REQ8_EN0: bool,
                REQ9_EN0: bool,
                REQ10_EN0: bool,
                REQ11_EN0: bool,
                REQ12_EN0: bool,
                REQ13_EN0: bool,
                REQ14_EN0: bool,
                REQ15_EN0: bool,
                REQ16_EN0: bool,
                REQ17_EN0: bool,
                REQ18_EN0: bool,
                REQ19_EN0: bool,
                REQ20_EN0: bool,
                REQ21_EN0: bool,
                REQ22_EN0: bool,
                REQ23_EN0: bool,
                REQ24_EN0: bool,
                REQ25_EN0: bool,
                REQ26_EN0: bool,
                REQ27_EN0: bool,
                REQ28_EN0: bool,
                REQ29_EN0: bool,
                REQ30_EN0: bool,
                REQ31_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE0_TOG {
                REQ1_EN0: self.REQ1_EN0(),
                REQ2_EN0: self.REQ2_EN0(),
                REQ3_EN0: self.REQ3_EN0(),
                REQ4_EN0: self.REQ4_EN0(),
                REQ5_EN0: self.REQ5_EN0(),
                REQ6_EN0: self.REQ6_EN0(),
                REQ7_EN0: self.REQ7_EN0(),
                REQ8_EN0: self.REQ8_EN0(),
                REQ9_EN0: self.REQ9_EN0(),
                REQ10_EN0: self.REQ10_EN0(),
                REQ11_EN0: self.REQ11_EN0(),
                REQ12_EN0: self.REQ12_EN0(),
                REQ13_EN0: self.REQ13_EN0(),
                REQ14_EN0: self.REQ14_EN0(),
                REQ15_EN0: self.REQ15_EN0(),
                REQ16_EN0: self.REQ16_EN0(),
                REQ17_EN0: self.REQ17_EN0(),
                REQ18_EN0: self.REQ18_EN0(),
                REQ19_EN0: self.REQ19_EN0(),
                REQ20_EN0: self.REQ20_EN0(),
                REQ21_EN0: self.REQ21_EN0(),
                REQ22_EN0: self.REQ22_EN0(),
                REQ23_EN0: self.REQ23_EN0(),
                REQ24_EN0: self.REQ24_EN0(),
                REQ25_EN0: self.REQ25_EN0(),
                REQ26_EN0: self.REQ26_EN0(),
                REQ27_EN0: self.REQ27_EN0(),
                REQ28_EN0: self.REQ28_EN0(),
                REQ29_EN0: self.REQ29_EN0(),
                REQ30_EN0: self.REQ30_EN0(),
                REQ31_EN0: self.REQ31_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE1(pub u32);
    impl DMA0_REQ_ENABLE1 {
        #[inline(always)]
        pub const fn REQ32_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE1 {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE1 {
            DMA0_REQ_ENABLE1(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE1")
                .field("REQ32_EN0", &self.REQ32_EN0())
                .field("REQ33_EN0", &self.REQ33_EN0())
                .field("REQ34_EN0", &self.REQ34_EN0())
                .field("REQ35_EN0", &self.REQ35_EN0())
                .field("REQ36_EN0", &self.REQ36_EN0())
                .field("REQ37_EN0", &self.REQ37_EN0())
                .field("REQ38_EN0", &self.REQ38_EN0())
                .field("REQ39_EN0", &self.REQ39_EN0())
                .field("REQ40_EN0", &self.REQ40_EN0())
                .field("REQ41_EN0", &self.REQ41_EN0())
                .field("REQ42_EN0", &self.REQ42_EN0())
                .field("REQ43_EN0", &self.REQ43_EN0())
                .field("REQ44_EN0", &self.REQ44_EN0())
                .field("REQ45_EN0", &self.REQ45_EN0())
                .field("REQ46_EN0", &self.REQ46_EN0())
                .field("REQ47_EN0", &self.REQ47_EN0())
                .field("REQ48_EN0", &self.REQ48_EN0())
                .field("REQ49_EN0", &self.REQ49_EN0())
                .field("REQ50_EN0", &self.REQ50_EN0())
                .field("REQ51_EN0", &self.REQ51_EN0())
                .field("REQ52_EN0", &self.REQ52_EN0())
                .field("REQ53_EN0", &self.REQ53_EN0())
                .field("REQ54_EN0", &self.REQ54_EN0())
                .field("REQ57_EN0", &self.REQ57_EN0())
                .field("REQ58_EN0", &self.REQ58_EN0())
                .field("REQ59_EN0", &self.REQ59_EN0())
                .field("REQ60_EN0", &self.REQ60_EN0())
                .field("REQ61_EN0", &self.REQ61_EN0())
                .field("REQ62_EN0", &self.REQ62_EN0())
                .field("REQ63_EN0", &self.REQ63_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE1 {
                REQ32_EN0: bool,
                REQ33_EN0: bool,
                REQ34_EN0: bool,
                REQ35_EN0: bool,
                REQ36_EN0: bool,
                REQ37_EN0: bool,
                REQ38_EN0: bool,
                REQ39_EN0: bool,
                REQ40_EN0: bool,
                REQ41_EN0: bool,
                REQ42_EN0: bool,
                REQ43_EN0: bool,
                REQ44_EN0: bool,
                REQ45_EN0: bool,
                REQ46_EN0: bool,
                REQ47_EN0: bool,
                REQ48_EN0: bool,
                REQ49_EN0: bool,
                REQ50_EN0: bool,
                REQ51_EN0: bool,
                REQ52_EN0: bool,
                REQ53_EN0: bool,
                REQ54_EN0: bool,
                REQ57_EN0: bool,
                REQ58_EN0: bool,
                REQ59_EN0: bool,
                REQ60_EN0: bool,
                REQ61_EN0: bool,
                REQ62_EN0: bool,
                REQ63_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE1 {
                REQ32_EN0: self.REQ32_EN0(),
                REQ33_EN0: self.REQ33_EN0(),
                REQ34_EN0: self.REQ34_EN0(),
                REQ35_EN0: self.REQ35_EN0(),
                REQ36_EN0: self.REQ36_EN0(),
                REQ37_EN0: self.REQ37_EN0(),
                REQ38_EN0: self.REQ38_EN0(),
                REQ39_EN0: self.REQ39_EN0(),
                REQ40_EN0: self.REQ40_EN0(),
                REQ41_EN0: self.REQ41_EN0(),
                REQ42_EN0: self.REQ42_EN0(),
                REQ43_EN0: self.REQ43_EN0(),
                REQ44_EN0: self.REQ44_EN0(),
                REQ45_EN0: self.REQ45_EN0(),
                REQ46_EN0: self.REQ46_EN0(),
                REQ47_EN0: self.REQ47_EN0(),
                REQ48_EN0: self.REQ48_EN0(),
                REQ49_EN0: self.REQ49_EN0(),
                REQ50_EN0: self.REQ50_EN0(),
                REQ51_EN0: self.REQ51_EN0(),
                REQ52_EN0: self.REQ52_EN0(),
                REQ53_EN0: self.REQ53_EN0(),
                REQ54_EN0: self.REQ54_EN0(),
                REQ57_EN0: self.REQ57_EN0(),
                REQ58_EN0: self.REQ58_EN0(),
                REQ59_EN0: self.REQ59_EN0(),
                REQ60_EN0: self.REQ60_EN0(),
                REQ61_EN0: self.REQ61_EN0(),
                REQ62_EN0: self.REQ62_EN0(),
                REQ63_EN0: self.REQ63_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE1_CLR(pub u32);
    impl DMA0_REQ_ENABLE1_CLR {
        #[inline(always)]
        pub const fn REQ32_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE1_CLR {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE1_CLR {
            DMA0_REQ_ENABLE1_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE1_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE1_CLR")
                .field("REQ32_EN0", &self.REQ32_EN0())
                .field("REQ33_EN0", &self.REQ33_EN0())
                .field("REQ34_EN0", &self.REQ34_EN0())
                .field("REQ35_EN0", &self.REQ35_EN0())
                .field("REQ36_EN0", &self.REQ36_EN0())
                .field("REQ37_EN0", &self.REQ37_EN0())
                .field("REQ38_EN0", &self.REQ38_EN0())
                .field("REQ39_EN0", &self.REQ39_EN0())
                .field("REQ40_EN0", &self.REQ40_EN0())
                .field("REQ41_EN0", &self.REQ41_EN0())
                .field("REQ42_EN0", &self.REQ42_EN0())
                .field("REQ43_EN0", &self.REQ43_EN0())
                .field("REQ44_EN0", &self.REQ44_EN0())
                .field("REQ45_EN0", &self.REQ45_EN0())
                .field("REQ46_EN0", &self.REQ46_EN0())
                .field("REQ47_EN0", &self.REQ47_EN0())
                .field("REQ48_EN0", &self.REQ48_EN0())
                .field("REQ49_EN0", &self.REQ49_EN0())
                .field("REQ50_EN0", &self.REQ50_EN0())
                .field("REQ51_EN0", &self.REQ51_EN0())
                .field("REQ52_EN0", &self.REQ52_EN0())
                .field("REQ53_EN0", &self.REQ53_EN0())
                .field("REQ54_EN0", &self.REQ54_EN0())
                .field("REQ57_EN0", &self.REQ57_EN0())
                .field("REQ58_EN0", &self.REQ58_EN0())
                .field("REQ59_EN0", &self.REQ59_EN0())
                .field("REQ60_EN0", &self.REQ60_EN0())
                .field("REQ61_EN0", &self.REQ61_EN0())
                .field("REQ62_EN0", &self.REQ62_EN0())
                .field("REQ63_EN0", &self.REQ63_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE1_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE1_CLR {
                REQ32_EN0: bool,
                REQ33_EN0: bool,
                REQ34_EN0: bool,
                REQ35_EN0: bool,
                REQ36_EN0: bool,
                REQ37_EN0: bool,
                REQ38_EN0: bool,
                REQ39_EN0: bool,
                REQ40_EN0: bool,
                REQ41_EN0: bool,
                REQ42_EN0: bool,
                REQ43_EN0: bool,
                REQ44_EN0: bool,
                REQ45_EN0: bool,
                REQ46_EN0: bool,
                REQ47_EN0: bool,
                REQ48_EN0: bool,
                REQ49_EN0: bool,
                REQ50_EN0: bool,
                REQ51_EN0: bool,
                REQ52_EN0: bool,
                REQ53_EN0: bool,
                REQ54_EN0: bool,
                REQ57_EN0: bool,
                REQ58_EN0: bool,
                REQ59_EN0: bool,
                REQ60_EN0: bool,
                REQ61_EN0: bool,
                REQ62_EN0: bool,
                REQ63_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE1_CLR {
                REQ32_EN0: self.REQ32_EN0(),
                REQ33_EN0: self.REQ33_EN0(),
                REQ34_EN0: self.REQ34_EN0(),
                REQ35_EN0: self.REQ35_EN0(),
                REQ36_EN0: self.REQ36_EN0(),
                REQ37_EN0: self.REQ37_EN0(),
                REQ38_EN0: self.REQ38_EN0(),
                REQ39_EN0: self.REQ39_EN0(),
                REQ40_EN0: self.REQ40_EN0(),
                REQ41_EN0: self.REQ41_EN0(),
                REQ42_EN0: self.REQ42_EN0(),
                REQ43_EN0: self.REQ43_EN0(),
                REQ44_EN0: self.REQ44_EN0(),
                REQ45_EN0: self.REQ45_EN0(),
                REQ46_EN0: self.REQ46_EN0(),
                REQ47_EN0: self.REQ47_EN0(),
                REQ48_EN0: self.REQ48_EN0(),
                REQ49_EN0: self.REQ49_EN0(),
                REQ50_EN0: self.REQ50_EN0(),
                REQ51_EN0: self.REQ51_EN0(),
                REQ52_EN0: self.REQ52_EN0(),
                REQ53_EN0: self.REQ53_EN0(),
                REQ54_EN0: self.REQ54_EN0(),
                REQ57_EN0: self.REQ57_EN0(),
                REQ58_EN0: self.REQ58_EN0(),
                REQ59_EN0: self.REQ59_EN0(),
                REQ60_EN0: self.REQ60_EN0(),
                REQ61_EN0: self.REQ61_EN0(),
                REQ62_EN0: self.REQ62_EN0(),
                REQ63_EN0: self.REQ63_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE1_SET(pub u32);
    impl DMA0_REQ_ENABLE1_SET {
        #[inline(always)]
        pub const fn REQ32_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE1_SET {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE1_SET {
            DMA0_REQ_ENABLE1_SET(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE1_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE1_SET")
                .field("REQ32_EN0", &self.REQ32_EN0())
                .field("REQ33_EN0", &self.REQ33_EN0())
                .field("REQ34_EN0", &self.REQ34_EN0())
                .field("REQ35_EN0", &self.REQ35_EN0())
                .field("REQ36_EN0", &self.REQ36_EN0())
                .field("REQ37_EN0", &self.REQ37_EN0())
                .field("REQ38_EN0", &self.REQ38_EN0())
                .field("REQ39_EN0", &self.REQ39_EN0())
                .field("REQ40_EN0", &self.REQ40_EN0())
                .field("REQ41_EN0", &self.REQ41_EN0())
                .field("REQ42_EN0", &self.REQ42_EN0())
                .field("REQ43_EN0", &self.REQ43_EN0())
                .field("REQ44_EN0", &self.REQ44_EN0())
                .field("REQ45_EN0", &self.REQ45_EN0())
                .field("REQ46_EN0", &self.REQ46_EN0())
                .field("REQ47_EN0", &self.REQ47_EN0())
                .field("REQ48_EN0", &self.REQ48_EN0())
                .field("REQ49_EN0", &self.REQ49_EN0())
                .field("REQ50_EN0", &self.REQ50_EN0())
                .field("REQ51_EN0", &self.REQ51_EN0())
                .field("REQ52_EN0", &self.REQ52_EN0())
                .field("REQ53_EN0", &self.REQ53_EN0())
                .field("REQ54_EN0", &self.REQ54_EN0())
                .field("REQ57_EN0", &self.REQ57_EN0())
                .field("REQ58_EN0", &self.REQ58_EN0())
                .field("REQ59_EN0", &self.REQ59_EN0())
                .field("REQ60_EN0", &self.REQ60_EN0())
                .field("REQ61_EN0", &self.REQ61_EN0())
                .field("REQ62_EN0", &self.REQ62_EN0())
                .field("REQ63_EN0", &self.REQ63_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE1_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE1_SET {
                REQ32_EN0: bool,
                REQ33_EN0: bool,
                REQ34_EN0: bool,
                REQ35_EN0: bool,
                REQ36_EN0: bool,
                REQ37_EN0: bool,
                REQ38_EN0: bool,
                REQ39_EN0: bool,
                REQ40_EN0: bool,
                REQ41_EN0: bool,
                REQ42_EN0: bool,
                REQ43_EN0: bool,
                REQ44_EN0: bool,
                REQ45_EN0: bool,
                REQ46_EN0: bool,
                REQ47_EN0: bool,
                REQ48_EN0: bool,
                REQ49_EN0: bool,
                REQ50_EN0: bool,
                REQ51_EN0: bool,
                REQ52_EN0: bool,
                REQ53_EN0: bool,
                REQ54_EN0: bool,
                REQ57_EN0: bool,
                REQ58_EN0: bool,
                REQ59_EN0: bool,
                REQ60_EN0: bool,
                REQ61_EN0: bool,
                REQ62_EN0: bool,
                REQ63_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE1_SET {
                REQ32_EN0: self.REQ32_EN0(),
                REQ33_EN0: self.REQ33_EN0(),
                REQ34_EN0: self.REQ34_EN0(),
                REQ35_EN0: self.REQ35_EN0(),
                REQ36_EN0: self.REQ36_EN0(),
                REQ37_EN0: self.REQ37_EN0(),
                REQ38_EN0: self.REQ38_EN0(),
                REQ39_EN0: self.REQ39_EN0(),
                REQ40_EN0: self.REQ40_EN0(),
                REQ41_EN0: self.REQ41_EN0(),
                REQ42_EN0: self.REQ42_EN0(),
                REQ43_EN0: self.REQ43_EN0(),
                REQ44_EN0: self.REQ44_EN0(),
                REQ45_EN0: self.REQ45_EN0(),
                REQ46_EN0: self.REQ46_EN0(),
                REQ47_EN0: self.REQ47_EN0(),
                REQ48_EN0: self.REQ48_EN0(),
                REQ49_EN0: self.REQ49_EN0(),
                REQ50_EN0: self.REQ50_EN0(),
                REQ51_EN0: self.REQ51_EN0(),
                REQ52_EN0: self.REQ52_EN0(),
                REQ53_EN0: self.REQ53_EN0(),
                REQ54_EN0: self.REQ54_EN0(),
                REQ57_EN0: self.REQ57_EN0(),
                REQ58_EN0: self.REQ58_EN0(),
                REQ59_EN0: self.REQ59_EN0(),
                REQ60_EN0: self.REQ60_EN0(),
                REQ61_EN0: self.REQ61_EN0(),
                REQ62_EN0: self.REQ62_EN0(),
                REQ63_EN0: self.REQ63_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE1_TOG(pub u32);
    impl DMA0_REQ_ENABLE1_TOG {
        #[inline(always)]
        pub const fn REQ32_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE1_TOG {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE1_TOG {
            DMA0_REQ_ENABLE1_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE1_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE1_TOG")
                .field("REQ32_EN0", &self.REQ32_EN0())
                .field("REQ33_EN0", &self.REQ33_EN0())
                .field("REQ34_EN0", &self.REQ34_EN0())
                .field("REQ35_EN0", &self.REQ35_EN0())
                .field("REQ36_EN0", &self.REQ36_EN0())
                .field("REQ37_EN0", &self.REQ37_EN0())
                .field("REQ38_EN0", &self.REQ38_EN0())
                .field("REQ39_EN0", &self.REQ39_EN0())
                .field("REQ40_EN0", &self.REQ40_EN0())
                .field("REQ41_EN0", &self.REQ41_EN0())
                .field("REQ42_EN0", &self.REQ42_EN0())
                .field("REQ43_EN0", &self.REQ43_EN0())
                .field("REQ44_EN0", &self.REQ44_EN0())
                .field("REQ45_EN0", &self.REQ45_EN0())
                .field("REQ46_EN0", &self.REQ46_EN0())
                .field("REQ47_EN0", &self.REQ47_EN0())
                .field("REQ48_EN0", &self.REQ48_EN0())
                .field("REQ49_EN0", &self.REQ49_EN0())
                .field("REQ50_EN0", &self.REQ50_EN0())
                .field("REQ51_EN0", &self.REQ51_EN0())
                .field("REQ52_EN0", &self.REQ52_EN0())
                .field("REQ53_EN0", &self.REQ53_EN0())
                .field("REQ54_EN0", &self.REQ54_EN0())
                .field("REQ57_EN0", &self.REQ57_EN0())
                .field("REQ58_EN0", &self.REQ58_EN0())
                .field("REQ59_EN0", &self.REQ59_EN0())
                .field("REQ60_EN0", &self.REQ60_EN0())
                .field("REQ61_EN0", &self.REQ61_EN0())
                .field("REQ62_EN0", &self.REQ62_EN0())
                .field("REQ63_EN0", &self.REQ63_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE1_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE1_TOG {
                REQ32_EN0: bool,
                REQ33_EN0: bool,
                REQ34_EN0: bool,
                REQ35_EN0: bool,
                REQ36_EN0: bool,
                REQ37_EN0: bool,
                REQ38_EN0: bool,
                REQ39_EN0: bool,
                REQ40_EN0: bool,
                REQ41_EN0: bool,
                REQ42_EN0: bool,
                REQ43_EN0: bool,
                REQ44_EN0: bool,
                REQ45_EN0: bool,
                REQ46_EN0: bool,
                REQ47_EN0: bool,
                REQ48_EN0: bool,
                REQ49_EN0: bool,
                REQ50_EN0: bool,
                REQ51_EN0: bool,
                REQ52_EN0: bool,
                REQ53_EN0: bool,
                REQ54_EN0: bool,
                REQ57_EN0: bool,
                REQ58_EN0: bool,
                REQ59_EN0: bool,
                REQ60_EN0: bool,
                REQ61_EN0: bool,
                REQ62_EN0: bool,
                REQ63_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE1_TOG {
                REQ32_EN0: self.REQ32_EN0(),
                REQ33_EN0: self.REQ33_EN0(),
                REQ34_EN0: self.REQ34_EN0(),
                REQ35_EN0: self.REQ35_EN0(),
                REQ36_EN0: self.REQ36_EN0(),
                REQ37_EN0: self.REQ37_EN0(),
                REQ38_EN0: self.REQ38_EN0(),
                REQ39_EN0: self.REQ39_EN0(),
                REQ40_EN0: self.REQ40_EN0(),
                REQ41_EN0: self.REQ41_EN0(),
                REQ42_EN0: self.REQ42_EN0(),
                REQ43_EN0: self.REQ43_EN0(),
                REQ44_EN0: self.REQ44_EN0(),
                REQ45_EN0: self.REQ45_EN0(),
                REQ46_EN0: self.REQ46_EN0(),
                REQ47_EN0: self.REQ47_EN0(),
                REQ48_EN0: self.REQ48_EN0(),
                REQ49_EN0: self.REQ49_EN0(),
                REQ50_EN0: self.REQ50_EN0(),
                REQ51_EN0: self.REQ51_EN0(),
                REQ52_EN0: self.REQ52_EN0(),
                REQ53_EN0: self.REQ53_EN0(),
                REQ54_EN0: self.REQ54_EN0(),
                REQ57_EN0: self.REQ57_EN0(),
                REQ58_EN0: self.REQ58_EN0(),
                REQ59_EN0: self.REQ59_EN0(),
                REQ60_EN0: self.REQ60_EN0(),
                REQ61_EN0: self.REQ61_EN0(),
                REQ62_EN0: self.REQ62_EN0(),
                REQ63_EN0: self.REQ63_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE2(pub u32);
    impl DMA0_REQ_ENABLE2 {
        #[inline(always)]
        pub const fn REQ64_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE2 {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE2 {
            DMA0_REQ_ENABLE2(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE2")
                .field("REQ64_EN0", &self.REQ64_EN0())
                .field("REQ65_EN0", &self.REQ65_EN0())
                .field("REQ66_EN0", &self.REQ66_EN0())
                .field("REQ67_EN0", &self.REQ67_EN0())
                .field("REQ68_EN0", &self.REQ68_EN0())
                .field("REQ69_EN0", &self.REQ69_EN0())
                .field("REQ70_EN0", &self.REQ70_EN0())
                .field("REQ71_EN0", &self.REQ71_EN0())
                .field("REQ72_EN0", &self.REQ72_EN0())
                .field("REQ73_EN0", &self.REQ73_EN0())
                .field("REQ74_EN0", &self.REQ74_EN0())
                .field("REQ75_EN0", &self.REQ75_EN0())
                .field("REQ76_EN0", &self.REQ76_EN0())
                .field("REQ77_EN0", &self.REQ77_EN0())
                .field("REQ78_EN0", &self.REQ78_EN0())
                .field("REQ79_EN0", &self.REQ79_EN0())
                .field("REQ80_EN0", &self.REQ80_EN0())
                .field("REQ81_EN0", &self.REQ81_EN0())
                .field("REQ82_EN0", &self.REQ82_EN0())
                .field("REQ83_EN0", &self.REQ83_EN0())
                .field("REQ84_EN0", &self.REQ84_EN0())
                .field("REQ85_EN0", &self.REQ85_EN0())
                .field("REQ86_EN0", &self.REQ86_EN0())
                .field("REQ87_EN0", &self.REQ87_EN0())
                .field("REQ88_EN0", &self.REQ88_EN0())
                .field("REQ91_EN0", &self.REQ91_EN0())
                .field("REQ92_EN0", &self.REQ92_EN0())
                .field("REQ93_EN0", &self.REQ93_EN0())
                .field("REQ94_EN0", &self.REQ94_EN0())
                .field("REQ95_EN0", &self.REQ95_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE2 {
                REQ64_EN0: bool,
                REQ65_EN0: bool,
                REQ66_EN0: bool,
                REQ67_EN0: bool,
                REQ68_EN0: bool,
                REQ69_EN0: bool,
                REQ70_EN0: bool,
                REQ71_EN0: bool,
                REQ72_EN0: bool,
                REQ73_EN0: bool,
                REQ74_EN0: bool,
                REQ75_EN0: bool,
                REQ76_EN0: bool,
                REQ77_EN0: bool,
                REQ78_EN0: bool,
                REQ79_EN0: bool,
                REQ80_EN0: bool,
                REQ81_EN0: bool,
                REQ82_EN0: bool,
                REQ83_EN0: bool,
                REQ84_EN0: bool,
                REQ85_EN0: bool,
                REQ86_EN0: bool,
                REQ87_EN0: bool,
                REQ88_EN0: bool,
                REQ91_EN0: bool,
                REQ92_EN0: bool,
                REQ93_EN0: bool,
                REQ94_EN0: bool,
                REQ95_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE2 {
                REQ64_EN0: self.REQ64_EN0(),
                REQ65_EN0: self.REQ65_EN0(),
                REQ66_EN0: self.REQ66_EN0(),
                REQ67_EN0: self.REQ67_EN0(),
                REQ68_EN0: self.REQ68_EN0(),
                REQ69_EN0: self.REQ69_EN0(),
                REQ70_EN0: self.REQ70_EN0(),
                REQ71_EN0: self.REQ71_EN0(),
                REQ72_EN0: self.REQ72_EN0(),
                REQ73_EN0: self.REQ73_EN0(),
                REQ74_EN0: self.REQ74_EN0(),
                REQ75_EN0: self.REQ75_EN0(),
                REQ76_EN0: self.REQ76_EN0(),
                REQ77_EN0: self.REQ77_EN0(),
                REQ78_EN0: self.REQ78_EN0(),
                REQ79_EN0: self.REQ79_EN0(),
                REQ80_EN0: self.REQ80_EN0(),
                REQ81_EN0: self.REQ81_EN0(),
                REQ82_EN0: self.REQ82_EN0(),
                REQ83_EN0: self.REQ83_EN0(),
                REQ84_EN0: self.REQ84_EN0(),
                REQ85_EN0: self.REQ85_EN0(),
                REQ86_EN0: self.REQ86_EN0(),
                REQ87_EN0: self.REQ87_EN0(),
                REQ88_EN0: self.REQ88_EN0(),
                REQ91_EN0: self.REQ91_EN0(),
                REQ92_EN0: self.REQ92_EN0(),
                REQ93_EN0: self.REQ93_EN0(),
                REQ94_EN0: self.REQ94_EN0(),
                REQ95_EN0: self.REQ95_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE2_CLR(pub u32);
    impl DMA0_REQ_ENABLE2_CLR {
        #[inline(always)]
        pub const fn REQ64_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE2_CLR {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE2_CLR {
            DMA0_REQ_ENABLE2_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE2_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE2_CLR")
                .field("REQ64_EN0", &self.REQ64_EN0())
                .field("REQ65_EN0", &self.REQ65_EN0())
                .field("REQ66_EN0", &self.REQ66_EN0())
                .field("REQ67_EN0", &self.REQ67_EN0())
                .field("REQ68_EN0", &self.REQ68_EN0())
                .field("REQ69_EN0", &self.REQ69_EN0())
                .field("REQ70_EN0", &self.REQ70_EN0())
                .field("REQ71_EN0", &self.REQ71_EN0())
                .field("REQ72_EN0", &self.REQ72_EN0())
                .field("REQ73_EN0", &self.REQ73_EN0())
                .field("REQ74_EN0", &self.REQ74_EN0())
                .field("REQ75_EN0", &self.REQ75_EN0())
                .field("REQ76_EN0", &self.REQ76_EN0())
                .field("REQ77_EN0", &self.REQ77_EN0())
                .field("REQ78_EN0", &self.REQ78_EN0())
                .field("REQ79_EN0", &self.REQ79_EN0())
                .field("REQ80_EN0", &self.REQ80_EN0())
                .field("REQ81_EN0", &self.REQ81_EN0())
                .field("REQ82_EN0", &self.REQ82_EN0())
                .field("REQ83_EN0", &self.REQ83_EN0())
                .field("REQ84_EN0", &self.REQ84_EN0())
                .field("REQ85_EN0", &self.REQ85_EN0())
                .field("REQ86_EN0", &self.REQ86_EN0())
                .field("REQ87_EN0", &self.REQ87_EN0())
                .field("REQ88_EN0", &self.REQ88_EN0())
                .field("REQ91_EN0", &self.REQ91_EN0())
                .field("REQ92_EN0", &self.REQ92_EN0())
                .field("REQ93_EN0", &self.REQ93_EN0())
                .field("REQ94_EN0", &self.REQ94_EN0())
                .field("REQ95_EN0", &self.REQ95_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE2_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE2_CLR {
                REQ64_EN0: bool,
                REQ65_EN0: bool,
                REQ66_EN0: bool,
                REQ67_EN0: bool,
                REQ68_EN0: bool,
                REQ69_EN0: bool,
                REQ70_EN0: bool,
                REQ71_EN0: bool,
                REQ72_EN0: bool,
                REQ73_EN0: bool,
                REQ74_EN0: bool,
                REQ75_EN0: bool,
                REQ76_EN0: bool,
                REQ77_EN0: bool,
                REQ78_EN0: bool,
                REQ79_EN0: bool,
                REQ80_EN0: bool,
                REQ81_EN0: bool,
                REQ82_EN0: bool,
                REQ83_EN0: bool,
                REQ84_EN0: bool,
                REQ85_EN0: bool,
                REQ86_EN0: bool,
                REQ87_EN0: bool,
                REQ88_EN0: bool,
                REQ91_EN0: bool,
                REQ92_EN0: bool,
                REQ93_EN0: bool,
                REQ94_EN0: bool,
                REQ95_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE2_CLR {
                REQ64_EN0: self.REQ64_EN0(),
                REQ65_EN0: self.REQ65_EN0(),
                REQ66_EN0: self.REQ66_EN0(),
                REQ67_EN0: self.REQ67_EN0(),
                REQ68_EN0: self.REQ68_EN0(),
                REQ69_EN0: self.REQ69_EN0(),
                REQ70_EN0: self.REQ70_EN0(),
                REQ71_EN0: self.REQ71_EN0(),
                REQ72_EN0: self.REQ72_EN0(),
                REQ73_EN0: self.REQ73_EN0(),
                REQ74_EN0: self.REQ74_EN0(),
                REQ75_EN0: self.REQ75_EN0(),
                REQ76_EN0: self.REQ76_EN0(),
                REQ77_EN0: self.REQ77_EN0(),
                REQ78_EN0: self.REQ78_EN0(),
                REQ79_EN0: self.REQ79_EN0(),
                REQ80_EN0: self.REQ80_EN0(),
                REQ81_EN0: self.REQ81_EN0(),
                REQ82_EN0: self.REQ82_EN0(),
                REQ83_EN0: self.REQ83_EN0(),
                REQ84_EN0: self.REQ84_EN0(),
                REQ85_EN0: self.REQ85_EN0(),
                REQ86_EN0: self.REQ86_EN0(),
                REQ87_EN0: self.REQ87_EN0(),
                REQ88_EN0: self.REQ88_EN0(),
                REQ91_EN0: self.REQ91_EN0(),
                REQ92_EN0: self.REQ92_EN0(),
                REQ93_EN0: self.REQ93_EN0(),
                REQ94_EN0: self.REQ94_EN0(),
                REQ95_EN0: self.REQ95_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE2_SET(pub u32);
    impl DMA0_REQ_ENABLE2_SET {
        #[inline(always)]
        pub const fn REQ64_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE2_SET {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE2_SET {
            DMA0_REQ_ENABLE2_SET(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE2_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE2_SET")
                .field("REQ64_EN0", &self.REQ64_EN0())
                .field("REQ65_EN0", &self.REQ65_EN0())
                .field("REQ66_EN0", &self.REQ66_EN0())
                .field("REQ67_EN0", &self.REQ67_EN0())
                .field("REQ68_EN0", &self.REQ68_EN0())
                .field("REQ69_EN0", &self.REQ69_EN0())
                .field("REQ70_EN0", &self.REQ70_EN0())
                .field("REQ71_EN0", &self.REQ71_EN0())
                .field("REQ72_EN0", &self.REQ72_EN0())
                .field("REQ73_EN0", &self.REQ73_EN0())
                .field("REQ74_EN0", &self.REQ74_EN0())
                .field("REQ75_EN0", &self.REQ75_EN0())
                .field("REQ76_EN0", &self.REQ76_EN0())
                .field("REQ77_EN0", &self.REQ77_EN0())
                .field("REQ78_EN0", &self.REQ78_EN0())
                .field("REQ79_EN0", &self.REQ79_EN0())
                .field("REQ80_EN0", &self.REQ80_EN0())
                .field("REQ81_EN0", &self.REQ81_EN0())
                .field("REQ82_EN0", &self.REQ82_EN0())
                .field("REQ83_EN0", &self.REQ83_EN0())
                .field("REQ84_EN0", &self.REQ84_EN0())
                .field("REQ85_EN0", &self.REQ85_EN0())
                .field("REQ86_EN0", &self.REQ86_EN0())
                .field("REQ87_EN0", &self.REQ87_EN0())
                .field("REQ88_EN0", &self.REQ88_EN0())
                .field("REQ91_EN0", &self.REQ91_EN0())
                .field("REQ92_EN0", &self.REQ92_EN0())
                .field("REQ93_EN0", &self.REQ93_EN0())
                .field("REQ94_EN0", &self.REQ94_EN0())
                .field("REQ95_EN0", &self.REQ95_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE2_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE2_SET {
                REQ64_EN0: bool,
                REQ65_EN0: bool,
                REQ66_EN0: bool,
                REQ67_EN0: bool,
                REQ68_EN0: bool,
                REQ69_EN0: bool,
                REQ70_EN0: bool,
                REQ71_EN0: bool,
                REQ72_EN0: bool,
                REQ73_EN0: bool,
                REQ74_EN0: bool,
                REQ75_EN0: bool,
                REQ76_EN0: bool,
                REQ77_EN0: bool,
                REQ78_EN0: bool,
                REQ79_EN0: bool,
                REQ80_EN0: bool,
                REQ81_EN0: bool,
                REQ82_EN0: bool,
                REQ83_EN0: bool,
                REQ84_EN0: bool,
                REQ85_EN0: bool,
                REQ86_EN0: bool,
                REQ87_EN0: bool,
                REQ88_EN0: bool,
                REQ91_EN0: bool,
                REQ92_EN0: bool,
                REQ93_EN0: bool,
                REQ94_EN0: bool,
                REQ95_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE2_SET {
                REQ64_EN0: self.REQ64_EN0(),
                REQ65_EN0: self.REQ65_EN0(),
                REQ66_EN0: self.REQ66_EN0(),
                REQ67_EN0: self.REQ67_EN0(),
                REQ68_EN0: self.REQ68_EN0(),
                REQ69_EN0: self.REQ69_EN0(),
                REQ70_EN0: self.REQ70_EN0(),
                REQ71_EN0: self.REQ71_EN0(),
                REQ72_EN0: self.REQ72_EN0(),
                REQ73_EN0: self.REQ73_EN0(),
                REQ74_EN0: self.REQ74_EN0(),
                REQ75_EN0: self.REQ75_EN0(),
                REQ76_EN0: self.REQ76_EN0(),
                REQ77_EN0: self.REQ77_EN0(),
                REQ78_EN0: self.REQ78_EN0(),
                REQ79_EN0: self.REQ79_EN0(),
                REQ80_EN0: self.REQ80_EN0(),
                REQ81_EN0: self.REQ81_EN0(),
                REQ82_EN0: self.REQ82_EN0(),
                REQ83_EN0: self.REQ83_EN0(),
                REQ84_EN0: self.REQ84_EN0(),
                REQ85_EN0: self.REQ85_EN0(),
                REQ86_EN0: self.REQ86_EN0(),
                REQ87_EN0: self.REQ87_EN0(),
                REQ88_EN0: self.REQ88_EN0(),
                REQ91_EN0: self.REQ91_EN0(),
                REQ92_EN0: self.REQ92_EN0(),
                REQ93_EN0: self.REQ93_EN0(),
                REQ94_EN0: self.REQ94_EN0(),
                REQ95_EN0: self.REQ95_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE2_TOG(pub u32);
    impl DMA0_REQ_ENABLE2_TOG {
        #[inline(always)]
        pub const fn REQ64_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE2_TOG {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE2_TOG {
            DMA0_REQ_ENABLE2_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE2_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE2_TOG")
                .field("REQ64_EN0", &self.REQ64_EN0())
                .field("REQ65_EN0", &self.REQ65_EN0())
                .field("REQ66_EN0", &self.REQ66_EN0())
                .field("REQ67_EN0", &self.REQ67_EN0())
                .field("REQ68_EN0", &self.REQ68_EN0())
                .field("REQ69_EN0", &self.REQ69_EN0())
                .field("REQ70_EN0", &self.REQ70_EN0())
                .field("REQ71_EN0", &self.REQ71_EN0())
                .field("REQ72_EN0", &self.REQ72_EN0())
                .field("REQ73_EN0", &self.REQ73_EN0())
                .field("REQ74_EN0", &self.REQ74_EN0())
                .field("REQ75_EN0", &self.REQ75_EN0())
                .field("REQ76_EN0", &self.REQ76_EN0())
                .field("REQ77_EN0", &self.REQ77_EN0())
                .field("REQ78_EN0", &self.REQ78_EN0())
                .field("REQ79_EN0", &self.REQ79_EN0())
                .field("REQ80_EN0", &self.REQ80_EN0())
                .field("REQ81_EN0", &self.REQ81_EN0())
                .field("REQ82_EN0", &self.REQ82_EN0())
                .field("REQ83_EN0", &self.REQ83_EN0())
                .field("REQ84_EN0", &self.REQ84_EN0())
                .field("REQ85_EN0", &self.REQ85_EN0())
                .field("REQ86_EN0", &self.REQ86_EN0())
                .field("REQ87_EN0", &self.REQ87_EN0())
                .field("REQ88_EN0", &self.REQ88_EN0())
                .field("REQ91_EN0", &self.REQ91_EN0())
                .field("REQ92_EN0", &self.REQ92_EN0())
                .field("REQ93_EN0", &self.REQ93_EN0())
                .field("REQ94_EN0", &self.REQ94_EN0())
                .field("REQ95_EN0", &self.REQ95_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE2_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE2_TOG {
                REQ64_EN0: bool,
                REQ65_EN0: bool,
                REQ66_EN0: bool,
                REQ67_EN0: bool,
                REQ68_EN0: bool,
                REQ69_EN0: bool,
                REQ70_EN0: bool,
                REQ71_EN0: bool,
                REQ72_EN0: bool,
                REQ73_EN0: bool,
                REQ74_EN0: bool,
                REQ75_EN0: bool,
                REQ76_EN0: bool,
                REQ77_EN0: bool,
                REQ78_EN0: bool,
                REQ79_EN0: bool,
                REQ80_EN0: bool,
                REQ81_EN0: bool,
                REQ82_EN0: bool,
                REQ83_EN0: bool,
                REQ84_EN0: bool,
                REQ85_EN0: bool,
                REQ86_EN0: bool,
                REQ87_EN0: bool,
                REQ88_EN0: bool,
                REQ91_EN0: bool,
                REQ92_EN0: bool,
                REQ93_EN0: bool,
                REQ94_EN0: bool,
                REQ95_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE2_TOG {
                REQ64_EN0: self.REQ64_EN0(),
                REQ65_EN0: self.REQ65_EN0(),
                REQ66_EN0: self.REQ66_EN0(),
                REQ67_EN0: self.REQ67_EN0(),
                REQ68_EN0: self.REQ68_EN0(),
                REQ69_EN0: self.REQ69_EN0(),
                REQ70_EN0: self.REQ70_EN0(),
                REQ71_EN0: self.REQ71_EN0(),
                REQ72_EN0: self.REQ72_EN0(),
                REQ73_EN0: self.REQ73_EN0(),
                REQ74_EN0: self.REQ74_EN0(),
                REQ75_EN0: self.REQ75_EN0(),
                REQ76_EN0: self.REQ76_EN0(),
                REQ77_EN0: self.REQ77_EN0(),
                REQ78_EN0: self.REQ78_EN0(),
                REQ79_EN0: self.REQ79_EN0(),
                REQ80_EN0: self.REQ80_EN0(),
                REQ81_EN0: self.REQ81_EN0(),
                REQ82_EN0: self.REQ82_EN0(),
                REQ83_EN0: self.REQ83_EN0(),
                REQ84_EN0: self.REQ84_EN0(),
                REQ85_EN0: self.REQ85_EN0(),
                REQ86_EN0: self.REQ86_EN0(),
                REQ87_EN0: self.REQ87_EN0(),
                REQ88_EN0: self.REQ88_EN0(),
                REQ91_EN0: self.REQ91_EN0(),
                REQ92_EN0: self.REQ92_EN0(),
                REQ93_EN0: self.REQ93_EN0(),
                REQ94_EN0: self.REQ94_EN0(),
                REQ95_EN0: self.REQ95_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE3(pub u32);
    impl DMA0_REQ_ENABLE3 {
        #[inline(always)]
        pub const fn REQ96_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE3 {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE3 {
            DMA0_REQ_ENABLE3(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE3")
                .field("REQ96_EN0", &self.REQ96_EN0())
                .field("REQ97_EN0", &self.REQ97_EN0())
                .field("REQ98_EN0", &self.REQ98_EN0())
                .field("REQ99_EN0", &self.REQ99_EN0())
                .field("REQ100_EN0", &self.REQ100_EN0())
                .field("REQ101_EN0", &self.REQ101_EN0())
                .field("REQ102_EN0", &self.REQ102_EN0())
                .field("REQ103_EN0", &self.REQ103_EN0())
                .field("REQ104_EN0", &self.REQ104_EN0())
                .field("REQ105_EN0", &self.REQ105_EN0())
                .field("REQ106_EN0", &self.REQ106_EN0())
                .field("REQ107_EN0", &self.REQ107_EN0())
                .field("REQ108_EN0", &self.REQ108_EN0())
                .field("REQ109_EN0", &self.REQ109_EN0())
                .field("REQ110_EN0", &self.REQ110_EN0())
                .field("REQ111_EN0", &self.REQ111_EN0())
                .field("REQ112_EN0", &self.REQ112_EN0())
                .field("REQ113_EN0", &self.REQ113_EN0())
                .field("REQ114_EN0", &self.REQ114_EN0())
                .field("REQ115_EN0", &self.REQ115_EN0())
                .field("REQ116_EN0", &self.REQ116_EN0())
                .field("REQ117_EN0", &self.REQ117_EN0())
                .field("REQ118_EN0", &self.REQ118_EN0())
                .field("REQ119_EN0", &self.REQ119_EN0())
                .field("REQ120_EN0", &self.REQ120_EN0())
                .field("REQ121_EN0", &self.REQ121_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE3 {
                REQ96_EN0: bool,
                REQ97_EN0: bool,
                REQ98_EN0: bool,
                REQ99_EN0: bool,
                REQ100_EN0: bool,
                REQ101_EN0: bool,
                REQ102_EN0: bool,
                REQ103_EN0: bool,
                REQ104_EN0: bool,
                REQ105_EN0: bool,
                REQ106_EN0: bool,
                REQ107_EN0: bool,
                REQ108_EN0: bool,
                REQ109_EN0: bool,
                REQ110_EN0: bool,
                REQ111_EN0: bool,
                REQ112_EN0: bool,
                REQ113_EN0: bool,
                REQ114_EN0: bool,
                REQ115_EN0: bool,
                REQ116_EN0: bool,
                REQ117_EN0: bool,
                REQ118_EN0: bool,
                REQ119_EN0: bool,
                REQ120_EN0: bool,
                REQ121_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE3 {
                REQ96_EN0: self.REQ96_EN0(),
                REQ97_EN0: self.REQ97_EN0(),
                REQ98_EN0: self.REQ98_EN0(),
                REQ99_EN0: self.REQ99_EN0(),
                REQ100_EN0: self.REQ100_EN0(),
                REQ101_EN0: self.REQ101_EN0(),
                REQ102_EN0: self.REQ102_EN0(),
                REQ103_EN0: self.REQ103_EN0(),
                REQ104_EN0: self.REQ104_EN0(),
                REQ105_EN0: self.REQ105_EN0(),
                REQ106_EN0: self.REQ106_EN0(),
                REQ107_EN0: self.REQ107_EN0(),
                REQ108_EN0: self.REQ108_EN0(),
                REQ109_EN0: self.REQ109_EN0(),
                REQ110_EN0: self.REQ110_EN0(),
                REQ111_EN0: self.REQ111_EN0(),
                REQ112_EN0: self.REQ112_EN0(),
                REQ113_EN0: self.REQ113_EN0(),
                REQ114_EN0: self.REQ114_EN0(),
                REQ115_EN0: self.REQ115_EN0(),
                REQ116_EN0: self.REQ116_EN0(),
                REQ117_EN0: self.REQ117_EN0(),
                REQ118_EN0: self.REQ118_EN0(),
                REQ119_EN0: self.REQ119_EN0(),
                REQ120_EN0: self.REQ120_EN0(),
                REQ121_EN0: self.REQ121_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE3_CLR(pub u32);
    impl DMA0_REQ_ENABLE3_CLR {
        #[inline(always)]
        pub const fn REQ96_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE3_CLR {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE3_CLR {
            DMA0_REQ_ENABLE3_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE3_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE3_CLR")
                .field("REQ96_EN0", &self.REQ96_EN0())
                .field("REQ97_EN0", &self.REQ97_EN0())
                .field("REQ98_EN0", &self.REQ98_EN0())
                .field("REQ99_EN0", &self.REQ99_EN0())
                .field("REQ100_EN0", &self.REQ100_EN0())
                .field("REQ101_EN0", &self.REQ101_EN0())
                .field("REQ102_EN0", &self.REQ102_EN0())
                .field("REQ103_EN0", &self.REQ103_EN0())
                .field("REQ104_EN0", &self.REQ104_EN0())
                .field("REQ105_EN0", &self.REQ105_EN0())
                .field("REQ106_EN0", &self.REQ106_EN0())
                .field("REQ107_EN0", &self.REQ107_EN0())
                .field("REQ108_EN0", &self.REQ108_EN0())
                .field("REQ109_EN0", &self.REQ109_EN0())
                .field("REQ110_EN0", &self.REQ110_EN0())
                .field("REQ111_EN0", &self.REQ111_EN0())
                .field("REQ112_EN0", &self.REQ112_EN0())
                .field("REQ113_EN0", &self.REQ113_EN0())
                .field("REQ114_EN0", &self.REQ114_EN0())
                .field("REQ115_EN0", &self.REQ115_EN0())
                .field("REQ116_EN0", &self.REQ116_EN0())
                .field("REQ117_EN0", &self.REQ117_EN0())
                .field("REQ118_EN0", &self.REQ118_EN0())
                .field("REQ119_EN0", &self.REQ119_EN0())
                .field("REQ120_EN0", &self.REQ120_EN0())
                .field("REQ121_EN0", &self.REQ121_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE3_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE3_CLR {
                REQ96_EN0: bool,
                REQ97_EN0: bool,
                REQ98_EN0: bool,
                REQ99_EN0: bool,
                REQ100_EN0: bool,
                REQ101_EN0: bool,
                REQ102_EN0: bool,
                REQ103_EN0: bool,
                REQ104_EN0: bool,
                REQ105_EN0: bool,
                REQ106_EN0: bool,
                REQ107_EN0: bool,
                REQ108_EN0: bool,
                REQ109_EN0: bool,
                REQ110_EN0: bool,
                REQ111_EN0: bool,
                REQ112_EN0: bool,
                REQ113_EN0: bool,
                REQ114_EN0: bool,
                REQ115_EN0: bool,
                REQ116_EN0: bool,
                REQ117_EN0: bool,
                REQ118_EN0: bool,
                REQ119_EN0: bool,
                REQ120_EN0: bool,
                REQ121_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE3_CLR {
                REQ96_EN0: self.REQ96_EN0(),
                REQ97_EN0: self.REQ97_EN0(),
                REQ98_EN0: self.REQ98_EN0(),
                REQ99_EN0: self.REQ99_EN0(),
                REQ100_EN0: self.REQ100_EN0(),
                REQ101_EN0: self.REQ101_EN0(),
                REQ102_EN0: self.REQ102_EN0(),
                REQ103_EN0: self.REQ103_EN0(),
                REQ104_EN0: self.REQ104_EN0(),
                REQ105_EN0: self.REQ105_EN0(),
                REQ106_EN0: self.REQ106_EN0(),
                REQ107_EN0: self.REQ107_EN0(),
                REQ108_EN0: self.REQ108_EN0(),
                REQ109_EN0: self.REQ109_EN0(),
                REQ110_EN0: self.REQ110_EN0(),
                REQ111_EN0: self.REQ111_EN0(),
                REQ112_EN0: self.REQ112_EN0(),
                REQ113_EN0: self.REQ113_EN0(),
                REQ114_EN0: self.REQ114_EN0(),
                REQ115_EN0: self.REQ115_EN0(),
                REQ116_EN0: self.REQ116_EN0(),
                REQ117_EN0: self.REQ117_EN0(),
                REQ118_EN0: self.REQ118_EN0(),
                REQ119_EN0: self.REQ119_EN0(),
                REQ120_EN0: self.REQ120_EN0(),
                REQ121_EN0: self.REQ121_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA0 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA0_REQ_ENABLE3_SET(pub u32);
    impl DMA0_REQ_ENABLE3_SET {
        #[inline(always)]
        pub const fn REQ96_EN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA0_REQ_ENABLE3_SET {
        #[inline(always)]
        fn default() -> DMA0_REQ_ENABLE3_SET {
            DMA0_REQ_ENABLE3_SET(0)
        }
    }
    impl core::fmt::Debug for DMA0_REQ_ENABLE3_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA0_REQ_ENABLE3_SET")
                .field("REQ96_EN0", &self.REQ96_EN0())
                .field("REQ97_EN0", &self.REQ97_EN0())
                .field("REQ98_EN0", &self.REQ98_EN0())
                .field("REQ99_EN0", &self.REQ99_EN0())
                .field("REQ100_EN0", &self.REQ100_EN0())
                .field("REQ101_EN0", &self.REQ101_EN0())
                .field("REQ102_EN0", &self.REQ102_EN0())
                .field("REQ103_EN0", &self.REQ103_EN0())
                .field("REQ104_EN0", &self.REQ104_EN0())
                .field("REQ105_EN0", &self.REQ105_EN0())
                .field("REQ106_EN0", &self.REQ106_EN0())
                .field("REQ107_EN0", &self.REQ107_EN0())
                .field("REQ108_EN0", &self.REQ108_EN0())
                .field("REQ109_EN0", &self.REQ109_EN0())
                .field("REQ110_EN0", &self.REQ110_EN0())
                .field("REQ111_EN0", &self.REQ111_EN0())
                .field("REQ112_EN0", &self.REQ112_EN0())
                .field("REQ113_EN0", &self.REQ113_EN0())
                .field("REQ114_EN0", &self.REQ114_EN0())
                .field("REQ115_EN0", &self.REQ115_EN0())
                .field("REQ116_EN0", &self.REQ116_EN0())
                .field("REQ117_EN0", &self.REQ117_EN0())
                .field("REQ118_EN0", &self.REQ118_EN0())
                .field("REQ119_EN0", &self.REQ119_EN0())
                .field("REQ120_EN0", &self.REQ120_EN0())
                .field("REQ121_EN0", &self.REQ121_EN0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA0_REQ_ENABLE3_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA0_REQ_ENABLE3_SET {
                REQ96_EN0: bool,
                REQ97_EN0: bool,
                REQ98_EN0: bool,
                REQ99_EN0: bool,
                REQ100_EN0: bool,
                REQ101_EN0: bool,
                REQ102_EN0: bool,
                REQ103_EN0: bool,
                REQ104_EN0: bool,
                REQ105_EN0: bool,
                REQ106_EN0: bool,
                REQ107_EN0: bool,
                REQ108_EN0: bool,
                REQ109_EN0: bool,
                REQ110_EN0: bool,
                REQ111_EN0: bool,
                REQ112_EN0: bool,
                REQ113_EN0: bool,
                REQ114_EN0: bool,
                REQ115_EN0: bool,
                REQ116_EN0: bool,
                REQ117_EN0: bool,
                REQ118_EN0: bool,
                REQ119_EN0: bool,
                REQ120_EN0: bool,
                REQ121_EN0: bool,
            }
            let proxy = DMA0_REQ_ENABLE3_SET {
                REQ96_EN0: self.REQ96_EN0(),
                REQ97_EN0: self.REQ97_EN0(),
                REQ98_EN0: self.REQ98_EN0(),
                REQ99_EN0: self.REQ99_EN0(),
                REQ100_EN0: self.REQ100_EN0(),
                REQ101_EN0: self.REQ101_EN0(),
                REQ102_EN0: self.REQ102_EN0(),
                REQ103_EN0: self.REQ103_EN0(),
                REQ104_EN0: self.REQ104_EN0(),
                REQ105_EN0: self.REQ105_EN0(),
                REQ106_EN0: self.REQ106_EN0(),
                REQ107_EN0: self.REQ107_EN0(),
                REQ108_EN0: self.REQ108_EN0(),
                REQ109_EN0: self.REQ109_EN0(),
                REQ110_EN0: self.REQ110_EN0(),
                REQ111_EN0: self.REQ111_EN0(),
                REQ112_EN0: self.REQ112_EN0(),
                REQ113_EN0: self.REQ113_EN0(),
                REQ114_EN0: self.REQ114_EN0(),
                REQ115_EN0: self.REQ115_EN0(),
                REQ116_EN0: self.REQ116_EN0(),
                REQ117_EN0: self.REQ117_EN0(),
                REQ118_EN0: self.REQ118_EN0(),
                REQ119_EN0: self.REQ119_EN0(),
                REQ120_EN0: self.REQ120_EN0(),
                REQ121_EN0: self.REQ121_EN0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE0(pub u32);
    impl DMA1_REQ_ENABLE0 {
        #[inline(always)]
        pub const fn REQ1_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE0 {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE0 {
            DMA1_REQ_ENABLE0(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE0")
                .field("REQ1_EN1", &self.REQ1_EN1())
                .field("REQ2_EN1", &self.REQ2_EN1())
                .field("REQ3_EN1", &self.REQ3_EN1())
                .field("REQ4_EN1", &self.REQ4_EN1())
                .field("REQ5_EN1", &self.REQ5_EN1())
                .field("REQ6_EN1", &self.REQ6_EN1())
                .field("REQ7_EN1", &self.REQ7_EN1())
                .field("REQ8_EN1", &self.REQ8_EN1())
                .field("REQ9_EN1", &self.REQ9_EN1())
                .field("REQ10_EN1", &self.REQ10_EN1())
                .field("REQ11_EN1", &self.REQ11_EN1())
                .field("REQ12_EN1", &self.REQ12_EN1())
                .field("REQ13_EN1", &self.REQ13_EN1())
                .field("REQ14_EN1", &self.REQ14_EN1())
                .field("REQ15_EN1", &self.REQ15_EN1())
                .field("REQ16_EN1", &self.REQ16_EN1())
                .field("REQ17_EN1", &self.REQ17_EN1())
                .field("REQ18_EN1", &self.REQ18_EN1())
                .field("REQ19_EN1", &self.REQ19_EN1())
                .field("REQ20_EN1", &self.REQ20_EN1())
                .field("REQ21_EN1", &self.REQ21_EN1())
                .field("REQ22_EN1", &self.REQ22_EN1())
                .field("REQ23_EN1", &self.REQ23_EN1())
                .field("REQ24_EN1", &self.REQ24_EN1())
                .field("REQ25_EN1", &self.REQ25_EN1())
                .field("REQ26_EN1", &self.REQ26_EN1())
                .field("REQ27_EN1", &self.REQ27_EN1())
                .field("REQ28_EN1", &self.REQ28_EN1())
                .field("REQ29_EN1", &self.REQ29_EN1())
                .field("REQ30_EN1", &self.REQ30_EN1())
                .field("REQ31_EN1", &self.REQ31_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE0 {
                REQ1_EN1: bool,
                REQ2_EN1: bool,
                REQ3_EN1: bool,
                REQ4_EN1: bool,
                REQ5_EN1: bool,
                REQ6_EN1: bool,
                REQ7_EN1: bool,
                REQ8_EN1: bool,
                REQ9_EN1: bool,
                REQ10_EN1: bool,
                REQ11_EN1: bool,
                REQ12_EN1: bool,
                REQ13_EN1: bool,
                REQ14_EN1: bool,
                REQ15_EN1: bool,
                REQ16_EN1: bool,
                REQ17_EN1: bool,
                REQ18_EN1: bool,
                REQ19_EN1: bool,
                REQ20_EN1: bool,
                REQ21_EN1: bool,
                REQ22_EN1: bool,
                REQ23_EN1: bool,
                REQ24_EN1: bool,
                REQ25_EN1: bool,
                REQ26_EN1: bool,
                REQ27_EN1: bool,
                REQ28_EN1: bool,
                REQ29_EN1: bool,
                REQ30_EN1: bool,
                REQ31_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE0 {
                REQ1_EN1: self.REQ1_EN1(),
                REQ2_EN1: self.REQ2_EN1(),
                REQ3_EN1: self.REQ3_EN1(),
                REQ4_EN1: self.REQ4_EN1(),
                REQ5_EN1: self.REQ5_EN1(),
                REQ6_EN1: self.REQ6_EN1(),
                REQ7_EN1: self.REQ7_EN1(),
                REQ8_EN1: self.REQ8_EN1(),
                REQ9_EN1: self.REQ9_EN1(),
                REQ10_EN1: self.REQ10_EN1(),
                REQ11_EN1: self.REQ11_EN1(),
                REQ12_EN1: self.REQ12_EN1(),
                REQ13_EN1: self.REQ13_EN1(),
                REQ14_EN1: self.REQ14_EN1(),
                REQ15_EN1: self.REQ15_EN1(),
                REQ16_EN1: self.REQ16_EN1(),
                REQ17_EN1: self.REQ17_EN1(),
                REQ18_EN1: self.REQ18_EN1(),
                REQ19_EN1: self.REQ19_EN1(),
                REQ20_EN1: self.REQ20_EN1(),
                REQ21_EN1: self.REQ21_EN1(),
                REQ22_EN1: self.REQ22_EN1(),
                REQ23_EN1: self.REQ23_EN1(),
                REQ24_EN1: self.REQ24_EN1(),
                REQ25_EN1: self.REQ25_EN1(),
                REQ26_EN1: self.REQ26_EN1(),
                REQ27_EN1: self.REQ27_EN1(),
                REQ28_EN1: self.REQ28_EN1(),
                REQ29_EN1: self.REQ29_EN1(),
                REQ30_EN1: self.REQ30_EN1(),
                REQ31_EN1: self.REQ31_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE0_CLR(pub u32);
    impl DMA1_REQ_ENABLE0_CLR {
        #[inline(always)]
        pub const fn REQ1_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE0_CLR {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE0_CLR {
            DMA1_REQ_ENABLE0_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE0_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE0_CLR")
                .field("REQ1_EN1", &self.REQ1_EN1())
                .field("REQ2_EN1", &self.REQ2_EN1())
                .field("REQ3_EN1", &self.REQ3_EN1())
                .field("REQ4_EN1", &self.REQ4_EN1())
                .field("REQ5_EN1", &self.REQ5_EN1())
                .field("REQ6_EN1", &self.REQ6_EN1())
                .field("REQ7_EN1", &self.REQ7_EN1())
                .field("REQ8_EN1", &self.REQ8_EN1())
                .field("REQ9_EN1", &self.REQ9_EN1())
                .field("REQ10_EN1", &self.REQ10_EN1())
                .field("REQ11_EN1", &self.REQ11_EN1())
                .field("REQ12_EN1", &self.REQ12_EN1())
                .field("REQ13_EN1", &self.REQ13_EN1())
                .field("REQ14_EN1", &self.REQ14_EN1())
                .field("REQ15_EN1", &self.REQ15_EN1())
                .field("REQ16_EN1", &self.REQ16_EN1())
                .field("REQ17_EN1", &self.REQ17_EN1())
                .field("REQ18_EN1", &self.REQ18_EN1())
                .field("REQ19_EN1", &self.REQ19_EN1())
                .field("REQ20_EN1", &self.REQ20_EN1())
                .field("REQ21_EN1", &self.REQ21_EN1())
                .field("REQ22_EN1", &self.REQ22_EN1())
                .field("REQ23_EN1", &self.REQ23_EN1())
                .field("REQ24_EN1", &self.REQ24_EN1())
                .field("REQ25_EN1", &self.REQ25_EN1())
                .field("REQ26_EN1", &self.REQ26_EN1())
                .field("REQ27_EN1", &self.REQ27_EN1())
                .field("REQ28_EN1", &self.REQ28_EN1())
                .field("REQ29_EN1", &self.REQ29_EN1())
                .field("REQ30_EN1", &self.REQ30_EN1())
                .field("REQ31_EN1", &self.REQ31_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE0_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE0_CLR {
                REQ1_EN1: bool,
                REQ2_EN1: bool,
                REQ3_EN1: bool,
                REQ4_EN1: bool,
                REQ5_EN1: bool,
                REQ6_EN1: bool,
                REQ7_EN1: bool,
                REQ8_EN1: bool,
                REQ9_EN1: bool,
                REQ10_EN1: bool,
                REQ11_EN1: bool,
                REQ12_EN1: bool,
                REQ13_EN1: bool,
                REQ14_EN1: bool,
                REQ15_EN1: bool,
                REQ16_EN1: bool,
                REQ17_EN1: bool,
                REQ18_EN1: bool,
                REQ19_EN1: bool,
                REQ20_EN1: bool,
                REQ21_EN1: bool,
                REQ22_EN1: bool,
                REQ23_EN1: bool,
                REQ24_EN1: bool,
                REQ25_EN1: bool,
                REQ26_EN1: bool,
                REQ27_EN1: bool,
                REQ28_EN1: bool,
                REQ29_EN1: bool,
                REQ30_EN1: bool,
                REQ31_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE0_CLR {
                REQ1_EN1: self.REQ1_EN1(),
                REQ2_EN1: self.REQ2_EN1(),
                REQ3_EN1: self.REQ3_EN1(),
                REQ4_EN1: self.REQ4_EN1(),
                REQ5_EN1: self.REQ5_EN1(),
                REQ6_EN1: self.REQ6_EN1(),
                REQ7_EN1: self.REQ7_EN1(),
                REQ8_EN1: self.REQ8_EN1(),
                REQ9_EN1: self.REQ9_EN1(),
                REQ10_EN1: self.REQ10_EN1(),
                REQ11_EN1: self.REQ11_EN1(),
                REQ12_EN1: self.REQ12_EN1(),
                REQ13_EN1: self.REQ13_EN1(),
                REQ14_EN1: self.REQ14_EN1(),
                REQ15_EN1: self.REQ15_EN1(),
                REQ16_EN1: self.REQ16_EN1(),
                REQ17_EN1: self.REQ17_EN1(),
                REQ18_EN1: self.REQ18_EN1(),
                REQ19_EN1: self.REQ19_EN1(),
                REQ20_EN1: self.REQ20_EN1(),
                REQ21_EN1: self.REQ21_EN1(),
                REQ22_EN1: self.REQ22_EN1(),
                REQ23_EN1: self.REQ23_EN1(),
                REQ24_EN1: self.REQ24_EN1(),
                REQ25_EN1: self.REQ25_EN1(),
                REQ26_EN1: self.REQ26_EN1(),
                REQ27_EN1: self.REQ27_EN1(),
                REQ28_EN1: self.REQ28_EN1(),
                REQ29_EN1: self.REQ29_EN1(),
                REQ30_EN1: self.REQ30_EN1(),
                REQ31_EN1: self.REQ31_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE0_SET(pub u32);
    impl DMA1_REQ_ENABLE0_SET {
        #[inline(always)]
        pub const fn REQ1_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE0_SET {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE0_SET {
            DMA1_REQ_ENABLE0_SET(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE0_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE0_SET")
                .field("REQ1_EN1", &self.REQ1_EN1())
                .field("REQ2_EN1", &self.REQ2_EN1())
                .field("REQ3_EN1", &self.REQ3_EN1())
                .field("REQ4_EN1", &self.REQ4_EN1())
                .field("REQ5_EN1", &self.REQ5_EN1())
                .field("REQ6_EN1", &self.REQ6_EN1())
                .field("REQ7_EN1", &self.REQ7_EN1())
                .field("REQ8_EN1", &self.REQ8_EN1())
                .field("REQ9_EN1", &self.REQ9_EN1())
                .field("REQ10_EN1", &self.REQ10_EN1())
                .field("REQ11_EN1", &self.REQ11_EN1())
                .field("REQ12_EN1", &self.REQ12_EN1())
                .field("REQ13_EN1", &self.REQ13_EN1())
                .field("REQ14_EN1", &self.REQ14_EN1())
                .field("REQ15_EN1", &self.REQ15_EN1())
                .field("REQ16_EN1", &self.REQ16_EN1())
                .field("REQ17_EN1", &self.REQ17_EN1())
                .field("REQ18_EN1", &self.REQ18_EN1())
                .field("REQ19_EN1", &self.REQ19_EN1())
                .field("REQ20_EN1", &self.REQ20_EN1())
                .field("REQ21_EN1", &self.REQ21_EN1())
                .field("REQ22_EN1", &self.REQ22_EN1())
                .field("REQ23_EN1", &self.REQ23_EN1())
                .field("REQ24_EN1", &self.REQ24_EN1())
                .field("REQ25_EN1", &self.REQ25_EN1())
                .field("REQ26_EN1", &self.REQ26_EN1())
                .field("REQ27_EN1", &self.REQ27_EN1())
                .field("REQ28_EN1", &self.REQ28_EN1())
                .field("REQ29_EN1", &self.REQ29_EN1())
                .field("REQ30_EN1", &self.REQ30_EN1())
                .field("REQ31_EN1", &self.REQ31_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE0_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE0_SET {
                REQ1_EN1: bool,
                REQ2_EN1: bool,
                REQ3_EN1: bool,
                REQ4_EN1: bool,
                REQ5_EN1: bool,
                REQ6_EN1: bool,
                REQ7_EN1: bool,
                REQ8_EN1: bool,
                REQ9_EN1: bool,
                REQ10_EN1: bool,
                REQ11_EN1: bool,
                REQ12_EN1: bool,
                REQ13_EN1: bool,
                REQ14_EN1: bool,
                REQ15_EN1: bool,
                REQ16_EN1: bool,
                REQ17_EN1: bool,
                REQ18_EN1: bool,
                REQ19_EN1: bool,
                REQ20_EN1: bool,
                REQ21_EN1: bool,
                REQ22_EN1: bool,
                REQ23_EN1: bool,
                REQ24_EN1: bool,
                REQ25_EN1: bool,
                REQ26_EN1: bool,
                REQ27_EN1: bool,
                REQ28_EN1: bool,
                REQ29_EN1: bool,
                REQ30_EN1: bool,
                REQ31_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE0_SET {
                REQ1_EN1: self.REQ1_EN1(),
                REQ2_EN1: self.REQ2_EN1(),
                REQ3_EN1: self.REQ3_EN1(),
                REQ4_EN1: self.REQ4_EN1(),
                REQ5_EN1: self.REQ5_EN1(),
                REQ6_EN1: self.REQ6_EN1(),
                REQ7_EN1: self.REQ7_EN1(),
                REQ8_EN1: self.REQ8_EN1(),
                REQ9_EN1: self.REQ9_EN1(),
                REQ10_EN1: self.REQ10_EN1(),
                REQ11_EN1: self.REQ11_EN1(),
                REQ12_EN1: self.REQ12_EN1(),
                REQ13_EN1: self.REQ13_EN1(),
                REQ14_EN1: self.REQ14_EN1(),
                REQ15_EN1: self.REQ15_EN1(),
                REQ16_EN1: self.REQ16_EN1(),
                REQ17_EN1: self.REQ17_EN1(),
                REQ18_EN1: self.REQ18_EN1(),
                REQ19_EN1: self.REQ19_EN1(),
                REQ20_EN1: self.REQ20_EN1(),
                REQ21_EN1: self.REQ21_EN1(),
                REQ22_EN1: self.REQ22_EN1(),
                REQ23_EN1: self.REQ23_EN1(),
                REQ24_EN1: self.REQ24_EN1(),
                REQ25_EN1: self.REQ25_EN1(),
                REQ26_EN1: self.REQ26_EN1(),
                REQ27_EN1: self.REQ27_EN1(),
                REQ28_EN1: self.REQ28_EN1(),
                REQ29_EN1: self.REQ29_EN1(),
                REQ30_EN1: self.REQ30_EN1(),
                REQ31_EN1: self.REQ31_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE0_TOG(pub u32);
    impl DMA1_REQ_ENABLE0_TOG {
        #[inline(always)]
        pub const fn REQ1_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ1_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ2_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ2_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ3_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ3_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ4_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ4_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ5_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ5_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ6_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ6_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ7_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ7_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ8_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ8_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ9_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ9_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ10_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ10_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ11_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ11_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ12_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ12_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ13_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ13_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ14_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ14_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ15_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ15_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ16_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ16_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ17_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ17_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ18_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ18_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ19_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ19_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ20_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ20_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ21_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ21_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ22_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ22_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ23_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ23_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ24_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ24_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ25_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ25_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ26_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ26_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ27_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ27_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ28_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ28_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ29_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ29_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ30_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ30_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ31_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ31_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE0_TOG {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE0_TOG {
            DMA1_REQ_ENABLE0_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE0_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE0_TOG")
                .field("REQ1_EN1", &self.REQ1_EN1())
                .field("REQ2_EN1", &self.REQ2_EN1())
                .field("REQ3_EN1", &self.REQ3_EN1())
                .field("REQ4_EN1", &self.REQ4_EN1())
                .field("REQ5_EN1", &self.REQ5_EN1())
                .field("REQ6_EN1", &self.REQ6_EN1())
                .field("REQ7_EN1", &self.REQ7_EN1())
                .field("REQ8_EN1", &self.REQ8_EN1())
                .field("REQ9_EN1", &self.REQ9_EN1())
                .field("REQ10_EN1", &self.REQ10_EN1())
                .field("REQ11_EN1", &self.REQ11_EN1())
                .field("REQ12_EN1", &self.REQ12_EN1())
                .field("REQ13_EN1", &self.REQ13_EN1())
                .field("REQ14_EN1", &self.REQ14_EN1())
                .field("REQ15_EN1", &self.REQ15_EN1())
                .field("REQ16_EN1", &self.REQ16_EN1())
                .field("REQ17_EN1", &self.REQ17_EN1())
                .field("REQ18_EN1", &self.REQ18_EN1())
                .field("REQ19_EN1", &self.REQ19_EN1())
                .field("REQ20_EN1", &self.REQ20_EN1())
                .field("REQ21_EN1", &self.REQ21_EN1())
                .field("REQ22_EN1", &self.REQ22_EN1())
                .field("REQ23_EN1", &self.REQ23_EN1())
                .field("REQ24_EN1", &self.REQ24_EN1())
                .field("REQ25_EN1", &self.REQ25_EN1())
                .field("REQ26_EN1", &self.REQ26_EN1())
                .field("REQ27_EN1", &self.REQ27_EN1())
                .field("REQ28_EN1", &self.REQ28_EN1())
                .field("REQ29_EN1", &self.REQ29_EN1())
                .field("REQ30_EN1", &self.REQ30_EN1())
                .field("REQ31_EN1", &self.REQ31_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE0_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE0_TOG {
                REQ1_EN1: bool,
                REQ2_EN1: bool,
                REQ3_EN1: bool,
                REQ4_EN1: bool,
                REQ5_EN1: bool,
                REQ6_EN1: bool,
                REQ7_EN1: bool,
                REQ8_EN1: bool,
                REQ9_EN1: bool,
                REQ10_EN1: bool,
                REQ11_EN1: bool,
                REQ12_EN1: bool,
                REQ13_EN1: bool,
                REQ14_EN1: bool,
                REQ15_EN1: bool,
                REQ16_EN1: bool,
                REQ17_EN1: bool,
                REQ18_EN1: bool,
                REQ19_EN1: bool,
                REQ20_EN1: bool,
                REQ21_EN1: bool,
                REQ22_EN1: bool,
                REQ23_EN1: bool,
                REQ24_EN1: bool,
                REQ25_EN1: bool,
                REQ26_EN1: bool,
                REQ27_EN1: bool,
                REQ28_EN1: bool,
                REQ29_EN1: bool,
                REQ30_EN1: bool,
                REQ31_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE0_TOG {
                REQ1_EN1: self.REQ1_EN1(),
                REQ2_EN1: self.REQ2_EN1(),
                REQ3_EN1: self.REQ3_EN1(),
                REQ4_EN1: self.REQ4_EN1(),
                REQ5_EN1: self.REQ5_EN1(),
                REQ6_EN1: self.REQ6_EN1(),
                REQ7_EN1: self.REQ7_EN1(),
                REQ8_EN1: self.REQ8_EN1(),
                REQ9_EN1: self.REQ9_EN1(),
                REQ10_EN1: self.REQ10_EN1(),
                REQ11_EN1: self.REQ11_EN1(),
                REQ12_EN1: self.REQ12_EN1(),
                REQ13_EN1: self.REQ13_EN1(),
                REQ14_EN1: self.REQ14_EN1(),
                REQ15_EN1: self.REQ15_EN1(),
                REQ16_EN1: self.REQ16_EN1(),
                REQ17_EN1: self.REQ17_EN1(),
                REQ18_EN1: self.REQ18_EN1(),
                REQ19_EN1: self.REQ19_EN1(),
                REQ20_EN1: self.REQ20_EN1(),
                REQ21_EN1: self.REQ21_EN1(),
                REQ22_EN1: self.REQ22_EN1(),
                REQ23_EN1: self.REQ23_EN1(),
                REQ24_EN1: self.REQ24_EN1(),
                REQ25_EN1: self.REQ25_EN1(),
                REQ26_EN1: self.REQ26_EN1(),
                REQ27_EN1: self.REQ27_EN1(),
                REQ28_EN1: self.REQ28_EN1(),
                REQ29_EN1: self.REQ29_EN1(),
                REQ30_EN1: self.REQ30_EN1(),
                REQ31_EN1: self.REQ31_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE1(pub u32);
    impl DMA1_REQ_ENABLE1 {
        #[inline(always)]
        pub const fn REQ32_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE1 {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE1 {
            DMA1_REQ_ENABLE1(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE1")
                .field("REQ32_EN1", &self.REQ32_EN1())
                .field("REQ33_EN1", &self.REQ33_EN1())
                .field("REQ34_EN1", &self.REQ34_EN1())
                .field("REQ35_EN1", &self.REQ35_EN1())
                .field("REQ36_EN1", &self.REQ36_EN1())
                .field("REQ37_EN1", &self.REQ37_EN1())
                .field("REQ38_EN1", &self.REQ38_EN1())
                .field("REQ39_EN1", &self.REQ39_EN1())
                .field("REQ40_EN1", &self.REQ40_EN1())
                .field("REQ41_EN1", &self.REQ41_EN1())
                .field("REQ42_EN1", &self.REQ42_EN1())
                .field("REQ43_EN1", &self.REQ43_EN1())
                .field("REQ44_EN1", &self.REQ44_EN1())
                .field("REQ45_EN1", &self.REQ45_EN1())
                .field("REQ46_EN1", &self.REQ46_EN1())
                .field("REQ47_EN1", &self.REQ47_EN1())
                .field("REQ48_EN1", &self.REQ48_EN1())
                .field("REQ49_EN1", &self.REQ49_EN1())
                .field("REQ50_EN1", &self.REQ50_EN1())
                .field("REQ51_EN1", &self.REQ51_EN1())
                .field("REQ52_EN1", &self.REQ52_EN1())
                .field("REQ53_EN1", &self.REQ53_EN1())
                .field("REQ54_EN1", &self.REQ54_EN1())
                .field("REQ57_EN1", &self.REQ57_EN1())
                .field("REQ58_EN1", &self.REQ58_EN1())
                .field("REQ59_EN1", &self.REQ59_EN1())
                .field("REQ60_EN1", &self.REQ60_EN1())
                .field("REQ61_EN1", &self.REQ61_EN1())
                .field("REQ62_EN1", &self.REQ62_EN1())
                .field("REQ63_EN1", &self.REQ63_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE1 {
                REQ32_EN1: bool,
                REQ33_EN1: bool,
                REQ34_EN1: bool,
                REQ35_EN1: bool,
                REQ36_EN1: bool,
                REQ37_EN1: bool,
                REQ38_EN1: bool,
                REQ39_EN1: bool,
                REQ40_EN1: bool,
                REQ41_EN1: bool,
                REQ42_EN1: bool,
                REQ43_EN1: bool,
                REQ44_EN1: bool,
                REQ45_EN1: bool,
                REQ46_EN1: bool,
                REQ47_EN1: bool,
                REQ48_EN1: bool,
                REQ49_EN1: bool,
                REQ50_EN1: bool,
                REQ51_EN1: bool,
                REQ52_EN1: bool,
                REQ53_EN1: bool,
                REQ54_EN1: bool,
                REQ57_EN1: bool,
                REQ58_EN1: bool,
                REQ59_EN1: bool,
                REQ60_EN1: bool,
                REQ61_EN1: bool,
                REQ62_EN1: bool,
                REQ63_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE1 {
                REQ32_EN1: self.REQ32_EN1(),
                REQ33_EN1: self.REQ33_EN1(),
                REQ34_EN1: self.REQ34_EN1(),
                REQ35_EN1: self.REQ35_EN1(),
                REQ36_EN1: self.REQ36_EN1(),
                REQ37_EN1: self.REQ37_EN1(),
                REQ38_EN1: self.REQ38_EN1(),
                REQ39_EN1: self.REQ39_EN1(),
                REQ40_EN1: self.REQ40_EN1(),
                REQ41_EN1: self.REQ41_EN1(),
                REQ42_EN1: self.REQ42_EN1(),
                REQ43_EN1: self.REQ43_EN1(),
                REQ44_EN1: self.REQ44_EN1(),
                REQ45_EN1: self.REQ45_EN1(),
                REQ46_EN1: self.REQ46_EN1(),
                REQ47_EN1: self.REQ47_EN1(),
                REQ48_EN1: self.REQ48_EN1(),
                REQ49_EN1: self.REQ49_EN1(),
                REQ50_EN1: self.REQ50_EN1(),
                REQ51_EN1: self.REQ51_EN1(),
                REQ52_EN1: self.REQ52_EN1(),
                REQ53_EN1: self.REQ53_EN1(),
                REQ54_EN1: self.REQ54_EN1(),
                REQ57_EN1: self.REQ57_EN1(),
                REQ58_EN1: self.REQ58_EN1(),
                REQ59_EN1: self.REQ59_EN1(),
                REQ60_EN1: self.REQ60_EN1(),
                REQ61_EN1: self.REQ61_EN1(),
                REQ62_EN1: self.REQ62_EN1(),
                REQ63_EN1: self.REQ63_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE1_CLR(pub u32);
    impl DMA1_REQ_ENABLE1_CLR {
        #[inline(always)]
        pub const fn REQ32_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE1_CLR {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE1_CLR {
            DMA1_REQ_ENABLE1_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE1_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE1_CLR")
                .field("REQ32_EN1", &self.REQ32_EN1())
                .field("REQ33_EN1", &self.REQ33_EN1())
                .field("REQ34_EN1", &self.REQ34_EN1())
                .field("REQ35_EN1", &self.REQ35_EN1())
                .field("REQ36_EN1", &self.REQ36_EN1())
                .field("REQ37_EN1", &self.REQ37_EN1())
                .field("REQ38_EN1", &self.REQ38_EN1())
                .field("REQ39_EN1", &self.REQ39_EN1())
                .field("REQ40_EN1", &self.REQ40_EN1())
                .field("REQ41_EN1", &self.REQ41_EN1())
                .field("REQ42_EN1", &self.REQ42_EN1())
                .field("REQ43_EN1", &self.REQ43_EN1())
                .field("REQ44_EN1", &self.REQ44_EN1())
                .field("REQ45_EN1", &self.REQ45_EN1())
                .field("REQ46_EN1", &self.REQ46_EN1())
                .field("REQ47_EN1", &self.REQ47_EN1())
                .field("REQ48_EN1", &self.REQ48_EN1())
                .field("REQ49_EN1", &self.REQ49_EN1())
                .field("REQ50_EN1", &self.REQ50_EN1())
                .field("REQ51_EN1", &self.REQ51_EN1())
                .field("REQ52_EN1", &self.REQ52_EN1())
                .field("REQ53_EN1", &self.REQ53_EN1())
                .field("REQ54_EN1", &self.REQ54_EN1())
                .field("REQ57_EN1", &self.REQ57_EN1())
                .field("REQ58_EN1", &self.REQ58_EN1())
                .field("REQ59_EN1", &self.REQ59_EN1())
                .field("REQ60_EN1", &self.REQ60_EN1())
                .field("REQ61_EN1", &self.REQ61_EN1())
                .field("REQ62_EN1", &self.REQ62_EN1())
                .field("REQ63_EN1", &self.REQ63_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE1_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE1_CLR {
                REQ32_EN1: bool,
                REQ33_EN1: bool,
                REQ34_EN1: bool,
                REQ35_EN1: bool,
                REQ36_EN1: bool,
                REQ37_EN1: bool,
                REQ38_EN1: bool,
                REQ39_EN1: bool,
                REQ40_EN1: bool,
                REQ41_EN1: bool,
                REQ42_EN1: bool,
                REQ43_EN1: bool,
                REQ44_EN1: bool,
                REQ45_EN1: bool,
                REQ46_EN1: bool,
                REQ47_EN1: bool,
                REQ48_EN1: bool,
                REQ49_EN1: bool,
                REQ50_EN1: bool,
                REQ51_EN1: bool,
                REQ52_EN1: bool,
                REQ53_EN1: bool,
                REQ54_EN1: bool,
                REQ57_EN1: bool,
                REQ58_EN1: bool,
                REQ59_EN1: bool,
                REQ60_EN1: bool,
                REQ61_EN1: bool,
                REQ62_EN1: bool,
                REQ63_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE1_CLR {
                REQ32_EN1: self.REQ32_EN1(),
                REQ33_EN1: self.REQ33_EN1(),
                REQ34_EN1: self.REQ34_EN1(),
                REQ35_EN1: self.REQ35_EN1(),
                REQ36_EN1: self.REQ36_EN1(),
                REQ37_EN1: self.REQ37_EN1(),
                REQ38_EN1: self.REQ38_EN1(),
                REQ39_EN1: self.REQ39_EN1(),
                REQ40_EN1: self.REQ40_EN1(),
                REQ41_EN1: self.REQ41_EN1(),
                REQ42_EN1: self.REQ42_EN1(),
                REQ43_EN1: self.REQ43_EN1(),
                REQ44_EN1: self.REQ44_EN1(),
                REQ45_EN1: self.REQ45_EN1(),
                REQ46_EN1: self.REQ46_EN1(),
                REQ47_EN1: self.REQ47_EN1(),
                REQ48_EN1: self.REQ48_EN1(),
                REQ49_EN1: self.REQ49_EN1(),
                REQ50_EN1: self.REQ50_EN1(),
                REQ51_EN1: self.REQ51_EN1(),
                REQ52_EN1: self.REQ52_EN1(),
                REQ53_EN1: self.REQ53_EN1(),
                REQ54_EN1: self.REQ54_EN1(),
                REQ57_EN1: self.REQ57_EN1(),
                REQ58_EN1: self.REQ58_EN1(),
                REQ59_EN1: self.REQ59_EN1(),
                REQ60_EN1: self.REQ60_EN1(),
                REQ61_EN1: self.REQ61_EN1(),
                REQ62_EN1: self.REQ62_EN1(),
                REQ63_EN1: self.REQ63_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE1_SET(pub u32);
    impl DMA1_REQ_ENABLE1_SET {
        #[inline(always)]
        pub const fn REQ32_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE1_SET {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE1_SET {
            DMA1_REQ_ENABLE1_SET(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE1_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE1_SET")
                .field("REQ32_EN1", &self.REQ32_EN1())
                .field("REQ33_EN1", &self.REQ33_EN1())
                .field("REQ34_EN1", &self.REQ34_EN1())
                .field("REQ35_EN1", &self.REQ35_EN1())
                .field("REQ36_EN1", &self.REQ36_EN1())
                .field("REQ37_EN1", &self.REQ37_EN1())
                .field("REQ38_EN1", &self.REQ38_EN1())
                .field("REQ39_EN1", &self.REQ39_EN1())
                .field("REQ40_EN1", &self.REQ40_EN1())
                .field("REQ41_EN1", &self.REQ41_EN1())
                .field("REQ42_EN1", &self.REQ42_EN1())
                .field("REQ43_EN1", &self.REQ43_EN1())
                .field("REQ44_EN1", &self.REQ44_EN1())
                .field("REQ45_EN1", &self.REQ45_EN1())
                .field("REQ46_EN1", &self.REQ46_EN1())
                .field("REQ47_EN1", &self.REQ47_EN1())
                .field("REQ48_EN1", &self.REQ48_EN1())
                .field("REQ49_EN1", &self.REQ49_EN1())
                .field("REQ50_EN1", &self.REQ50_EN1())
                .field("REQ51_EN1", &self.REQ51_EN1())
                .field("REQ52_EN1", &self.REQ52_EN1())
                .field("REQ53_EN1", &self.REQ53_EN1())
                .field("REQ54_EN1", &self.REQ54_EN1())
                .field("REQ57_EN1", &self.REQ57_EN1())
                .field("REQ58_EN1", &self.REQ58_EN1())
                .field("REQ59_EN1", &self.REQ59_EN1())
                .field("REQ60_EN1", &self.REQ60_EN1())
                .field("REQ61_EN1", &self.REQ61_EN1())
                .field("REQ62_EN1", &self.REQ62_EN1())
                .field("REQ63_EN1", &self.REQ63_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE1_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE1_SET {
                REQ32_EN1: bool,
                REQ33_EN1: bool,
                REQ34_EN1: bool,
                REQ35_EN1: bool,
                REQ36_EN1: bool,
                REQ37_EN1: bool,
                REQ38_EN1: bool,
                REQ39_EN1: bool,
                REQ40_EN1: bool,
                REQ41_EN1: bool,
                REQ42_EN1: bool,
                REQ43_EN1: bool,
                REQ44_EN1: bool,
                REQ45_EN1: bool,
                REQ46_EN1: bool,
                REQ47_EN1: bool,
                REQ48_EN1: bool,
                REQ49_EN1: bool,
                REQ50_EN1: bool,
                REQ51_EN1: bool,
                REQ52_EN1: bool,
                REQ53_EN1: bool,
                REQ54_EN1: bool,
                REQ57_EN1: bool,
                REQ58_EN1: bool,
                REQ59_EN1: bool,
                REQ60_EN1: bool,
                REQ61_EN1: bool,
                REQ62_EN1: bool,
                REQ63_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE1_SET {
                REQ32_EN1: self.REQ32_EN1(),
                REQ33_EN1: self.REQ33_EN1(),
                REQ34_EN1: self.REQ34_EN1(),
                REQ35_EN1: self.REQ35_EN1(),
                REQ36_EN1: self.REQ36_EN1(),
                REQ37_EN1: self.REQ37_EN1(),
                REQ38_EN1: self.REQ38_EN1(),
                REQ39_EN1: self.REQ39_EN1(),
                REQ40_EN1: self.REQ40_EN1(),
                REQ41_EN1: self.REQ41_EN1(),
                REQ42_EN1: self.REQ42_EN1(),
                REQ43_EN1: self.REQ43_EN1(),
                REQ44_EN1: self.REQ44_EN1(),
                REQ45_EN1: self.REQ45_EN1(),
                REQ46_EN1: self.REQ46_EN1(),
                REQ47_EN1: self.REQ47_EN1(),
                REQ48_EN1: self.REQ48_EN1(),
                REQ49_EN1: self.REQ49_EN1(),
                REQ50_EN1: self.REQ50_EN1(),
                REQ51_EN1: self.REQ51_EN1(),
                REQ52_EN1: self.REQ52_EN1(),
                REQ53_EN1: self.REQ53_EN1(),
                REQ54_EN1: self.REQ54_EN1(),
                REQ57_EN1: self.REQ57_EN1(),
                REQ58_EN1: self.REQ58_EN1(),
                REQ59_EN1: self.REQ59_EN1(),
                REQ60_EN1: self.REQ60_EN1(),
                REQ61_EN1: self.REQ61_EN1(),
                REQ62_EN1: self.REQ62_EN1(),
                REQ63_EN1: self.REQ63_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE1_TOG(pub u32);
    impl DMA1_REQ_ENABLE1_TOG {
        #[inline(always)]
        pub const fn REQ32_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ32_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ33_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ33_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ34_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ34_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ35_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ35_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ36_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ36_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ37_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ37_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ38_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ38_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ39_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ39_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ40_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ40_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ41_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ41_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ42_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ42_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ43_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ43_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ44_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ44_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ45_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ45_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ46_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ46_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ47_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ47_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ48_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ48_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ49_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ49_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ50_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ50_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ51_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ51_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ52_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ52_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ53_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ53_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ54_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ54_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ57_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ57_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ58_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ58_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ59_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ59_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ60_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ60_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ61_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ61_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ62_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ62_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ63_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ63_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE1_TOG {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE1_TOG {
            DMA1_REQ_ENABLE1_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE1_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE1_TOG")
                .field("REQ32_EN1", &self.REQ32_EN1())
                .field("REQ33_EN1", &self.REQ33_EN1())
                .field("REQ34_EN1", &self.REQ34_EN1())
                .field("REQ35_EN1", &self.REQ35_EN1())
                .field("REQ36_EN1", &self.REQ36_EN1())
                .field("REQ37_EN1", &self.REQ37_EN1())
                .field("REQ38_EN1", &self.REQ38_EN1())
                .field("REQ39_EN1", &self.REQ39_EN1())
                .field("REQ40_EN1", &self.REQ40_EN1())
                .field("REQ41_EN1", &self.REQ41_EN1())
                .field("REQ42_EN1", &self.REQ42_EN1())
                .field("REQ43_EN1", &self.REQ43_EN1())
                .field("REQ44_EN1", &self.REQ44_EN1())
                .field("REQ45_EN1", &self.REQ45_EN1())
                .field("REQ46_EN1", &self.REQ46_EN1())
                .field("REQ47_EN1", &self.REQ47_EN1())
                .field("REQ48_EN1", &self.REQ48_EN1())
                .field("REQ49_EN1", &self.REQ49_EN1())
                .field("REQ50_EN1", &self.REQ50_EN1())
                .field("REQ51_EN1", &self.REQ51_EN1())
                .field("REQ52_EN1", &self.REQ52_EN1())
                .field("REQ53_EN1", &self.REQ53_EN1())
                .field("REQ54_EN1", &self.REQ54_EN1())
                .field("REQ57_EN1", &self.REQ57_EN1())
                .field("REQ58_EN1", &self.REQ58_EN1())
                .field("REQ59_EN1", &self.REQ59_EN1())
                .field("REQ60_EN1", &self.REQ60_EN1())
                .field("REQ61_EN1", &self.REQ61_EN1())
                .field("REQ62_EN1", &self.REQ62_EN1())
                .field("REQ63_EN1", &self.REQ63_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE1_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE1_TOG {
                REQ32_EN1: bool,
                REQ33_EN1: bool,
                REQ34_EN1: bool,
                REQ35_EN1: bool,
                REQ36_EN1: bool,
                REQ37_EN1: bool,
                REQ38_EN1: bool,
                REQ39_EN1: bool,
                REQ40_EN1: bool,
                REQ41_EN1: bool,
                REQ42_EN1: bool,
                REQ43_EN1: bool,
                REQ44_EN1: bool,
                REQ45_EN1: bool,
                REQ46_EN1: bool,
                REQ47_EN1: bool,
                REQ48_EN1: bool,
                REQ49_EN1: bool,
                REQ50_EN1: bool,
                REQ51_EN1: bool,
                REQ52_EN1: bool,
                REQ53_EN1: bool,
                REQ54_EN1: bool,
                REQ57_EN1: bool,
                REQ58_EN1: bool,
                REQ59_EN1: bool,
                REQ60_EN1: bool,
                REQ61_EN1: bool,
                REQ62_EN1: bool,
                REQ63_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE1_TOG {
                REQ32_EN1: self.REQ32_EN1(),
                REQ33_EN1: self.REQ33_EN1(),
                REQ34_EN1: self.REQ34_EN1(),
                REQ35_EN1: self.REQ35_EN1(),
                REQ36_EN1: self.REQ36_EN1(),
                REQ37_EN1: self.REQ37_EN1(),
                REQ38_EN1: self.REQ38_EN1(),
                REQ39_EN1: self.REQ39_EN1(),
                REQ40_EN1: self.REQ40_EN1(),
                REQ41_EN1: self.REQ41_EN1(),
                REQ42_EN1: self.REQ42_EN1(),
                REQ43_EN1: self.REQ43_EN1(),
                REQ44_EN1: self.REQ44_EN1(),
                REQ45_EN1: self.REQ45_EN1(),
                REQ46_EN1: self.REQ46_EN1(),
                REQ47_EN1: self.REQ47_EN1(),
                REQ48_EN1: self.REQ48_EN1(),
                REQ49_EN1: self.REQ49_EN1(),
                REQ50_EN1: self.REQ50_EN1(),
                REQ51_EN1: self.REQ51_EN1(),
                REQ52_EN1: self.REQ52_EN1(),
                REQ53_EN1: self.REQ53_EN1(),
                REQ54_EN1: self.REQ54_EN1(),
                REQ57_EN1: self.REQ57_EN1(),
                REQ58_EN1: self.REQ58_EN1(),
                REQ59_EN1: self.REQ59_EN1(),
                REQ60_EN1: self.REQ60_EN1(),
                REQ61_EN1: self.REQ61_EN1(),
                REQ62_EN1: self.REQ62_EN1(),
                REQ63_EN1: self.REQ63_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE2(pub u32);
    impl DMA1_REQ_ENABLE2 {
        #[inline(always)]
        pub const fn REQ64_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE2 {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE2 {
            DMA1_REQ_ENABLE2(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE2")
                .field("REQ64_EN1", &self.REQ64_EN1())
                .field("REQ65_EN1", &self.REQ65_EN1())
                .field("REQ66_EN1", &self.REQ66_EN1())
                .field("REQ67_EN1", &self.REQ67_EN1())
                .field("REQ68_EN1", &self.REQ68_EN1())
                .field("REQ69_EN1", &self.REQ69_EN1())
                .field("REQ70_EN1", &self.REQ70_EN1())
                .field("REQ71_EN1", &self.REQ71_EN1())
                .field("REQ72_EN1", &self.REQ72_EN1())
                .field("REQ73_EN1", &self.REQ73_EN1())
                .field("REQ74_EN1", &self.REQ74_EN1())
                .field("REQ75_EN1", &self.REQ75_EN1())
                .field("REQ76_EN1", &self.REQ76_EN1())
                .field("REQ77_EN1", &self.REQ77_EN1())
                .field("REQ78_EN1", &self.REQ78_EN1())
                .field("REQ79_EN1", &self.REQ79_EN1())
                .field("REQ80_EN1", &self.REQ80_EN1())
                .field("REQ81_EN1", &self.REQ81_EN1())
                .field("REQ82_EN1", &self.REQ82_EN1())
                .field("REQ83_EN1", &self.REQ83_EN1())
                .field("REQ84_EN1", &self.REQ84_EN1())
                .field("REQ85_EN1", &self.REQ85_EN1())
                .field("REQ86_EN1", &self.REQ86_EN1())
                .field("REQ87_EN1", &self.REQ87_EN1())
                .field("REQ88_EN1", &self.REQ88_EN1())
                .field("REQ91_EN1", &self.REQ91_EN1())
                .field("REQ92_EN1", &self.REQ92_EN1())
                .field("REQ93_EN1", &self.REQ93_EN1())
                .field("REQ94_EN1", &self.REQ94_EN1())
                .field("REQ95_EN1", &self.REQ95_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE2 {
                REQ64_EN1: bool,
                REQ65_EN1: bool,
                REQ66_EN1: bool,
                REQ67_EN1: bool,
                REQ68_EN1: bool,
                REQ69_EN1: bool,
                REQ70_EN1: bool,
                REQ71_EN1: bool,
                REQ72_EN1: bool,
                REQ73_EN1: bool,
                REQ74_EN1: bool,
                REQ75_EN1: bool,
                REQ76_EN1: bool,
                REQ77_EN1: bool,
                REQ78_EN1: bool,
                REQ79_EN1: bool,
                REQ80_EN1: bool,
                REQ81_EN1: bool,
                REQ82_EN1: bool,
                REQ83_EN1: bool,
                REQ84_EN1: bool,
                REQ85_EN1: bool,
                REQ86_EN1: bool,
                REQ87_EN1: bool,
                REQ88_EN1: bool,
                REQ91_EN1: bool,
                REQ92_EN1: bool,
                REQ93_EN1: bool,
                REQ94_EN1: bool,
                REQ95_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE2 {
                REQ64_EN1: self.REQ64_EN1(),
                REQ65_EN1: self.REQ65_EN1(),
                REQ66_EN1: self.REQ66_EN1(),
                REQ67_EN1: self.REQ67_EN1(),
                REQ68_EN1: self.REQ68_EN1(),
                REQ69_EN1: self.REQ69_EN1(),
                REQ70_EN1: self.REQ70_EN1(),
                REQ71_EN1: self.REQ71_EN1(),
                REQ72_EN1: self.REQ72_EN1(),
                REQ73_EN1: self.REQ73_EN1(),
                REQ74_EN1: self.REQ74_EN1(),
                REQ75_EN1: self.REQ75_EN1(),
                REQ76_EN1: self.REQ76_EN1(),
                REQ77_EN1: self.REQ77_EN1(),
                REQ78_EN1: self.REQ78_EN1(),
                REQ79_EN1: self.REQ79_EN1(),
                REQ80_EN1: self.REQ80_EN1(),
                REQ81_EN1: self.REQ81_EN1(),
                REQ82_EN1: self.REQ82_EN1(),
                REQ83_EN1: self.REQ83_EN1(),
                REQ84_EN1: self.REQ84_EN1(),
                REQ85_EN1: self.REQ85_EN1(),
                REQ86_EN1: self.REQ86_EN1(),
                REQ87_EN1: self.REQ87_EN1(),
                REQ88_EN1: self.REQ88_EN1(),
                REQ91_EN1: self.REQ91_EN1(),
                REQ92_EN1: self.REQ92_EN1(),
                REQ93_EN1: self.REQ93_EN1(),
                REQ94_EN1: self.REQ94_EN1(),
                REQ95_EN1: self.REQ95_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE2_CLR(pub u32);
    impl DMA1_REQ_ENABLE2_CLR {
        #[inline(always)]
        pub const fn REQ64_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ89_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ89_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ90_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ90_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE2_CLR {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE2_CLR {
            DMA1_REQ_ENABLE2_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE2_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE2_CLR")
                .field("REQ64_EN1", &self.REQ64_EN1())
                .field("REQ65_EN1", &self.REQ65_EN1())
                .field("REQ66_EN1", &self.REQ66_EN1())
                .field("REQ67_EN1", &self.REQ67_EN1())
                .field("REQ68_EN1", &self.REQ68_EN1())
                .field("REQ69_EN1", &self.REQ69_EN1())
                .field("REQ70_EN1", &self.REQ70_EN1())
                .field("REQ71_EN1", &self.REQ71_EN1())
                .field("REQ72_EN1", &self.REQ72_EN1())
                .field("REQ73_EN1", &self.REQ73_EN1())
                .field("REQ74_EN1", &self.REQ74_EN1())
                .field("REQ75_EN1", &self.REQ75_EN1())
                .field("REQ76_EN1", &self.REQ76_EN1())
                .field("REQ77_EN1", &self.REQ77_EN1())
                .field("REQ78_EN1", &self.REQ78_EN1())
                .field("REQ79_EN1", &self.REQ79_EN1())
                .field("REQ80_EN1", &self.REQ80_EN1())
                .field("REQ81_EN1", &self.REQ81_EN1())
                .field("REQ82_EN1", &self.REQ82_EN1())
                .field("REQ83_EN1", &self.REQ83_EN1())
                .field("REQ84_EN1", &self.REQ84_EN1())
                .field("REQ85_EN1", &self.REQ85_EN1())
                .field("REQ86_EN1", &self.REQ86_EN1())
                .field("REQ87_EN1", &self.REQ87_EN1())
                .field("REQ88_EN1", &self.REQ88_EN1())
                .field("REQ89_EN1", &self.REQ89_EN1())
                .field("REQ90_EN1", &self.REQ90_EN1())
                .field("REQ91_EN1", &self.REQ91_EN1())
                .field("REQ92_EN1", &self.REQ92_EN1())
                .field("REQ93_EN1", &self.REQ93_EN1())
                .field("REQ94_EN1", &self.REQ94_EN1())
                .field("REQ95_EN1", &self.REQ95_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE2_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE2_CLR {
                REQ64_EN1: bool,
                REQ65_EN1: bool,
                REQ66_EN1: bool,
                REQ67_EN1: bool,
                REQ68_EN1: bool,
                REQ69_EN1: bool,
                REQ70_EN1: bool,
                REQ71_EN1: bool,
                REQ72_EN1: bool,
                REQ73_EN1: bool,
                REQ74_EN1: bool,
                REQ75_EN1: bool,
                REQ76_EN1: bool,
                REQ77_EN1: bool,
                REQ78_EN1: bool,
                REQ79_EN1: bool,
                REQ80_EN1: bool,
                REQ81_EN1: bool,
                REQ82_EN1: bool,
                REQ83_EN1: bool,
                REQ84_EN1: bool,
                REQ85_EN1: bool,
                REQ86_EN1: bool,
                REQ87_EN1: bool,
                REQ88_EN1: bool,
                REQ89_EN1: bool,
                REQ90_EN1: bool,
                REQ91_EN1: bool,
                REQ92_EN1: bool,
                REQ93_EN1: bool,
                REQ94_EN1: bool,
                REQ95_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE2_CLR {
                REQ64_EN1: self.REQ64_EN1(),
                REQ65_EN1: self.REQ65_EN1(),
                REQ66_EN1: self.REQ66_EN1(),
                REQ67_EN1: self.REQ67_EN1(),
                REQ68_EN1: self.REQ68_EN1(),
                REQ69_EN1: self.REQ69_EN1(),
                REQ70_EN1: self.REQ70_EN1(),
                REQ71_EN1: self.REQ71_EN1(),
                REQ72_EN1: self.REQ72_EN1(),
                REQ73_EN1: self.REQ73_EN1(),
                REQ74_EN1: self.REQ74_EN1(),
                REQ75_EN1: self.REQ75_EN1(),
                REQ76_EN1: self.REQ76_EN1(),
                REQ77_EN1: self.REQ77_EN1(),
                REQ78_EN1: self.REQ78_EN1(),
                REQ79_EN1: self.REQ79_EN1(),
                REQ80_EN1: self.REQ80_EN1(),
                REQ81_EN1: self.REQ81_EN1(),
                REQ82_EN1: self.REQ82_EN1(),
                REQ83_EN1: self.REQ83_EN1(),
                REQ84_EN1: self.REQ84_EN1(),
                REQ85_EN1: self.REQ85_EN1(),
                REQ86_EN1: self.REQ86_EN1(),
                REQ87_EN1: self.REQ87_EN1(),
                REQ88_EN1: self.REQ88_EN1(),
                REQ89_EN1: self.REQ89_EN1(),
                REQ90_EN1: self.REQ90_EN1(),
                REQ91_EN1: self.REQ91_EN1(),
                REQ92_EN1: self.REQ92_EN1(),
                REQ93_EN1: self.REQ93_EN1(),
                REQ94_EN1: self.REQ94_EN1(),
                REQ95_EN1: self.REQ95_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE2_SET(pub u32);
    impl DMA1_REQ_ENABLE2_SET {
        #[inline(always)]
        pub const fn REQ64_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ89_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ89_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ90_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ90_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE2_SET {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE2_SET {
            DMA1_REQ_ENABLE2_SET(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE2_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE2_SET")
                .field("REQ64_EN1", &self.REQ64_EN1())
                .field("REQ65_EN1", &self.REQ65_EN1())
                .field("REQ66_EN1", &self.REQ66_EN1())
                .field("REQ67_EN1", &self.REQ67_EN1())
                .field("REQ68_EN1", &self.REQ68_EN1())
                .field("REQ69_EN1", &self.REQ69_EN1())
                .field("REQ70_EN1", &self.REQ70_EN1())
                .field("REQ71_EN1", &self.REQ71_EN1())
                .field("REQ72_EN1", &self.REQ72_EN1())
                .field("REQ73_EN1", &self.REQ73_EN1())
                .field("REQ74_EN1", &self.REQ74_EN1())
                .field("REQ75_EN1", &self.REQ75_EN1())
                .field("REQ76_EN1", &self.REQ76_EN1())
                .field("REQ77_EN1", &self.REQ77_EN1())
                .field("REQ78_EN1", &self.REQ78_EN1())
                .field("REQ79_EN1", &self.REQ79_EN1())
                .field("REQ80_EN1", &self.REQ80_EN1())
                .field("REQ81_EN1", &self.REQ81_EN1())
                .field("REQ82_EN1", &self.REQ82_EN1())
                .field("REQ83_EN1", &self.REQ83_EN1())
                .field("REQ84_EN1", &self.REQ84_EN1())
                .field("REQ85_EN1", &self.REQ85_EN1())
                .field("REQ86_EN1", &self.REQ86_EN1())
                .field("REQ87_EN1", &self.REQ87_EN1())
                .field("REQ88_EN1", &self.REQ88_EN1())
                .field("REQ89_EN1", &self.REQ89_EN1())
                .field("REQ90_EN1", &self.REQ90_EN1())
                .field("REQ91_EN1", &self.REQ91_EN1())
                .field("REQ92_EN1", &self.REQ92_EN1())
                .field("REQ93_EN1", &self.REQ93_EN1())
                .field("REQ94_EN1", &self.REQ94_EN1())
                .field("REQ95_EN1", &self.REQ95_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE2_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE2_SET {
                REQ64_EN1: bool,
                REQ65_EN1: bool,
                REQ66_EN1: bool,
                REQ67_EN1: bool,
                REQ68_EN1: bool,
                REQ69_EN1: bool,
                REQ70_EN1: bool,
                REQ71_EN1: bool,
                REQ72_EN1: bool,
                REQ73_EN1: bool,
                REQ74_EN1: bool,
                REQ75_EN1: bool,
                REQ76_EN1: bool,
                REQ77_EN1: bool,
                REQ78_EN1: bool,
                REQ79_EN1: bool,
                REQ80_EN1: bool,
                REQ81_EN1: bool,
                REQ82_EN1: bool,
                REQ83_EN1: bool,
                REQ84_EN1: bool,
                REQ85_EN1: bool,
                REQ86_EN1: bool,
                REQ87_EN1: bool,
                REQ88_EN1: bool,
                REQ89_EN1: bool,
                REQ90_EN1: bool,
                REQ91_EN1: bool,
                REQ92_EN1: bool,
                REQ93_EN1: bool,
                REQ94_EN1: bool,
                REQ95_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE2_SET {
                REQ64_EN1: self.REQ64_EN1(),
                REQ65_EN1: self.REQ65_EN1(),
                REQ66_EN1: self.REQ66_EN1(),
                REQ67_EN1: self.REQ67_EN1(),
                REQ68_EN1: self.REQ68_EN1(),
                REQ69_EN1: self.REQ69_EN1(),
                REQ70_EN1: self.REQ70_EN1(),
                REQ71_EN1: self.REQ71_EN1(),
                REQ72_EN1: self.REQ72_EN1(),
                REQ73_EN1: self.REQ73_EN1(),
                REQ74_EN1: self.REQ74_EN1(),
                REQ75_EN1: self.REQ75_EN1(),
                REQ76_EN1: self.REQ76_EN1(),
                REQ77_EN1: self.REQ77_EN1(),
                REQ78_EN1: self.REQ78_EN1(),
                REQ79_EN1: self.REQ79_EN1(),
                REQ80_EN1: self.REQ80_EN1(),
                REQ81_EN1: self.REQ81_EN1(),
                REQ82_EN1: self.REQ82_EN1(),
                REQ83_EN1: self.REQ83_EN1(),
                REQ84_EN1: self.REQ84_EN1(),
                REQ85_EN1: self.REQ85_EN1(),
                REQ86_EN1: self.REQ86_EN1(),
                REQ87_EN1: self.REQ87_EN1(),
                REQ88_EN1: self.REQ88_EN1(),
                REQ89_EN1: self.REQ89_EN1(),
                REQ90_EN1: self.REQ90_EN1(),
                REQ91_EN1: self.REQ91_EN1(),
                REQ92_EN1: self.REQ92_EN1(),
                REQ93_EN1: self.REQ93_EN1(),
                REQ94_EN1: self.REQ94_EN1(),
                REQ95_EN1: self.REQ95_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE2_TOG(pub u32);
    impl DMA1_REQ_ENABLE2_TOG {
        #[inline(always)]
        pub const fn REQ64_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ64_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ65_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ65_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ66_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ66_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ67_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ67_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ68_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ68_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ69_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ69_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ70_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ70_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ71_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ71_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ72_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ72_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ73_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ73_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ74_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ74_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ75_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ75_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ76_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ76_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ77_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ77_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ78_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ78_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ79_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ79_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ80_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ80_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ81_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ81_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ82_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ82_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ83_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ83_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ84_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ84_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ85_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ85_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ86_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ86_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ87_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ87_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ88_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ88_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ89_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ89_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REQ90_EN1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ90_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REQ91_EN1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ91_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REQ92_EN1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ92_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REQ93_EN1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ93_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REQ94_EN1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ94_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REQ95_EN1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ95_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE2_TOG {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE2_TOG {
            DMA1_REQ_ENABLE2_TOG(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE2_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE2_TOG")
                .field("REQ64_EN1", &self.REQ64_EN1())
                .field("REQ65_EN1", &self.REQ65_EN1())
                .field("REQ66_EN1", &self.REQ66_EN1())
                .field("REQ67_EN1", &self.REQ67_EN1())
                .field("REQ68_EN1", &self.REQ68_EN1())
                .field("REQ69_EN1", &self.REQ69_EN1())
                .field("REQ70_EN1", &self.REQ70_EN1())
                .field("REQ71_EN1", &self.REQ71_EN1())
                .field("REQ72_EN1", &self.REQ72_EN1())
                .field("REQ73_EN1", &self.REQ73_EN1())
                .field("REQ74_EN1", &self.REQ74_EN1())
                .field("REQ75_EN1", &self.REQ75_EN1())
                .field("REQ76_EN1", &self.REQ76_EN1())
                .field("REQ77_EN1", &self.REQ77_EN1())
                .field("REQ78_EN1", &self.REQ78_EN1())
                .field("REQ79_EN1", &self.REQ79_EN1())
                .field("REQ80_EN1", &self.REQ80_EN1())
                .field("REQ81_EN1", &self.REQ81_EN1())
                .field("REQ82_EN1", &self.REQ82_EN1())
                .field("REQ83_EN1", &self.REQ83_EN1())
                .field("REQ84_EN1", &self.REQ84_EN1())
                .field("REQ85_EN1", &self.REQ85_EN1())
                .field("REQ86_EN1", &self.REQ86_EN1())
                .field("REQ87_EN1", &self.REQ87_EN1())
                .field("REQ88_EN1", &self.REQ88_EN1())
                .field("REQ89_EN1", &self.REQ89_EN1())
                .field("REQ90_EN1", &self.REQ90_EN1())
                .field("REQ91_EN1", &self.REQ91_EN1())
                .field("REQ92_EN1", &self.REQ92_EN1())
                .field("REQ93_EN1", &self.REQ93_EN1())
                .field("REQ94_EN1", &self.REQ94_EN1())
                .field("REQ95_EN1", &self.REQ95_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE2_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE2_TOG {
                REQ64_EN1: bool,
                REQ65_EN1: bool,
                REQ66_EN1: bool,
                REQ67_EN1: bool,
                REQ68_EN1: bool,
                REQ69_EN1: bool,
                REQ70_EN1: bool,
                REQ71_EN1: bool,
                REQ72_EN1: bool,
                REQ73_EN1: bool,
                REQ74_EN1: bool,
                REQ75_EN1: bool,
                REQ76_EN1: bool,
                REQ77_EN1: bool,
                REQ78_EN1: bool,
                REQ79_EN1: bool,
                REQ80_EN1: bool,
                REQ81_EN1: bool,
                REQ82_EN1: bool,
                REQ83_EN1: bool,
                REQ84_EN1: bool,
                REQ85_EN1: bool,
                REQ86_EN1: bool,
                REQ87_EN1: bool,
                REQ88_EN1: bool,
                REQ89_EN1: bool,
                REQ90_EN1: bool,
                REQ91_EN1: bool,
                REQ92_EN1: bool,
                REQ93_EN1: bool,
                REQ94_EN1: bool,
                REQ95_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE2_TOG {
                REQ64_EN1: self.REQ64_EN1(),
                REQ65_EN1: self.REQ65_EN1(),
                REQ66_EN1: self.REQ66_EN1(),
                REQ67_EN1: self.REQ67_EN1(),
                REQ68_EN1: self.REQ68_EN1(),
                REQ69_EN1: self.REQ69_EN1(),
                REQ70_EN1: self.REQ70_EN1(),
                REQ71_EN1: self.REQ71_EN1(),
                REQ72_EN1: self.REQ72_EN1(),
                REQ73_EN1: self.REQ73_EN1(),
                REQ74_EN1: self.REQ74_EN1(),
                REQ75_EN1: self.REQ75_EN1(),
                REQ76_EN1: self.REQ76_EN1(),
                REQ77_EN1: self.REQ77_EN1(),
                REQ78_EN1: self.REQ78_EN1(),
                REQ79_EN1: self.REQ79_EN1(),
                REQ80_EN1: self.REQ80_EN1(),
                REQ81_EN1: self.REQ81_EN1(),
                REQ82_EN1: self.REQ82_EN1(),
                REQ83_EN1: self.REQ83_EN1(),
                REQ84_EN1: self.REQ84_EN1(),
                REQ85_EN1: self.REQ85_EN1(),
                REQ86_EN1: self.REQ86_EN1(),
                REQ87_EN1: self.REQ87_EN1(),
                REQ88_EN1: self.REQ88_EN1(),
                REQ89_EN1: self.REQ89_EN1(),
                REQ90_EN1: self.REQ90_EN1(),
                REQ91_EN1: self.REQ91_EN1(),
                REQ92_EN1: self.REQ92_EN1(),
                REQ93_EN1: self.REQ93_EN1(),
                REQ94_EN1: self.REQ94_EN1(),
                REQ95_EN1: self.REQ95_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE3(pub u32);
    impl DMA1_REQ_ENABLE3 {
        #[inline(always)]
        pub const fn REQ96_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE3 {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE3 {
            DMA1_REQ_ENABLE3(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE3")
                .field("REQ96_EN1", &self.REQ96_EN1())
                .field("REQ97_EN1", &self.REQ97_EN1())
                .field("REQ98_EN1", &self.REQ98_EN1())
                .field("REQ99_EN1", &self.REQ99_EN1())
                .field("REQ100_EN1", &self.REQ100_EN1())
                .field("REQ101_EN1", &self.REQ101_EN1())
                .field("REQ102_EN1", &self.REQ102_EN1())
                .field("REQ103_EN1", &self.REQ103_EN1())
                .field("REQ104_EN1", &self.REQ104_EN1())
                .field("REQ105_EN1", &self.REQ105_EN1())
                .field("REQ106_EN1", &self.REQ106_EN1())
                .field("REQ107_EN1", &self.REQ107_EN1())
                .field("REQ108_EN1", &self.REQ108_EN1())
                .field("REQ109_EN1", &self.REQ109_EN1())
                .field("REQ110_EN1", &self.REQ110_EN1())
                .field("REQ111_EN1", &self.REQ111_EN1())
                .field("REQ112_EN1", &self.REQ112_EN1())
                .field("REQ113_EN1", &self.REQ113_EN1())
                .field("REQ114_EN1", &self.REQ114_EN1())
                .field("REQ115_EN1", &self.REQ115_EN1())
                .field("REQ116_EN1", &self.REQ116_EN1())
                .field("REQ117_EN1", &self.REQ117_EN1())
                .field("REQ118_EN1", &self.REQ118_EN1())
                .field("REQ119_EN1", &self.REQ119_EN1())
                .field("REQ120_EN1", &self.REQ120_EN1())
                .field("REQ121_EN1", &self.REQ121_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE3 {
                REQ96_EN1: bool,
                REQ97_EN1: bool,
                REQ98_EN1: bool,
                REQ99_EN1: bool,
                REQ100_EN1: bool,
                REQ101_EN1: bool,
                REQ102_EN1: bool,
                REQ103_EN1: bool,
                REQ104_EN1: bool,
                REQ105_EN1: bool,
                REQ106_EN1: bool,
                REQ107_EN1: bool,
                REQ108_EN1: bool,
                REQ109_EN1: bool,
                REQ110_EN1: bool,
                REQ111_EN1: bool,
                REQ112_EN1: bool,
                REQ113_EN1: bool,
                REQ114_EN1: bool,
                REQ115_EN1: bool,
                REQ116_EN1: bool,
                REQ117_EN1: bool,
                REQ118_EN1: bool,
                REQ119_EN1: bool,
                REQ120_EN1: bool,
                REQ121_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE3 {
                REQ96_EN1: self.REQ96_EN1(),
                REQ97_EN1: self.REQ97_EN1(),
                REQ98_EN1: self.REQ98_EN1(),
                REQ99_EN1: self.REQ99_EN1(),
                REQ100_EN1: self.REQ100_EN1(),
                REQ101_EN1: self.REQ101_EN1(),
                REQ102_EN1: self.REQ102_EN1(),
                REQ103_EN1: self.REQ103_EN1(),
                REQ104_EN1: self.REQ104_EN1(),
                REQ105_EN1: self.REQ105_EN1(),
                REQ106_EN1: self.REQ106_EN1(),
                REQ107_EN1: self.REQ107_EN1(),
                REQ108_EN1: self.REQ108_EN1(),
                REQ109_EN1: self.REQ109_EN1(),
                REQ110_EN1: self.REQ110_EN1(),
                REQ111_EN1: self.REQ111_EN1(),
                REQ112_EN1: self.REQ112_EN1(),
                REQ113_EN1: self.REQ113_EN1(),
                REQ114_EN1: self.REQ114_EN1(),
                REQ115_EN1: self.REQ115_EN1(),
                REQ116_EN1: self.REQ116_EN1(),
                REQ117_EN1: self.REQ117_EN1(),
                REQ118_EN1: self.REQ118_EN1(),
                REQ119_EN1: self.REQ119_EN1(),
                REQ120_EN1: self.REQ120_EN1(),
                REQ121_EN1: self.REQ121_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE3_CLR(pub u32);
    impl DMA1_REQ_ENABLE3_CLR {
        #[inline(always)]
        pub const fn REQ96_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE3_CLR {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE3_CLR {
            DMA1_REQ_ENABLE3_CLR(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE3_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE3_CLR")
                .field("REQ96_EN1", &self.REQ96_EN1())
                .field("REQ97_EN1", &self.REQ97_EN1())
                .field("REQ98_EN1", &self.REQ98_EN1())
                .field("REQ99_EN1", &self.REQ99_EN1())
                .field("REQ100_EN1", &self.REQ100_EN1())
                .field("REQ101_EN1", &self.REQ101_EN1())
                .field("REQ102_EN1", &self.REQ102_EN1())
                .field("REQ103_EN1", &self.REQ103_EN1())
                .field("REQ104_EN1", &self.REQ104_EN1())
                .field("REQ105_EN1", &self.REQ105_EN1())
                .field("REQ106_EN1", &self.REQ106_EN1())
                .field("REQ107_EN1", &self.REQ107_EN1())
                .field("REQ108_EN1", &self.REQ108_EN1())
                .field("REQ109_EN1", &self.REQ109_EN1())
                .field("REQ110_EN1", &self.REQ110_EN1())
                .field("REQ111_EN1", &self.REQ111_EN1())
                .field("REQ112_EN1", &self.REQ112_EN1())
                .field("REQ113_EN1", &self.REQ113_EN1())
                .field("REQ114_EN1", &self.REQ114_EN1())
                .field("REQ115_EN1", &self.REQ115_EN1())
                .field("REQ116_EN1", &self.REQ116_EN1())
                .field("REQ117_EN1", &self.REQ117_EN1())
                .field("REQ118_EN1", &self.REQ118_EN1())
                .field("REQ119_EN1", &self.REQ119_EN1())
                .field("REQ120_EN1", &self.REQ120_EN1())
                .field("REQ121_EN1", &self.REQ121_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE3_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE3_CLR {
                REQ96_EN1: bool,
                REQ97_EN1: bool,
                REQ98_EN1: bool,
                REQ99_EN1: bool,
                REQ100_EN1: bool,
                REQ101_EN1: bool,
                REQ102_EN1: bool,
                REQ103_EN1: bool,
                REQ104_EN1: bool,
                REQ105_EN1: bool,
                REQ106_EN1: bool,
                REQ107_EN1: bool,
                REQ108_EN1: bool,
                REQ109_EN1: bool,
                REQ110_EN1: bool,
                REQ111_EN1: bool,
                REQ112_EN1: bool,
                REQ113_EN1: bool,
                REQ114_EN1: bool,
                REQ115_EN1: bool,
                REQ116_EN1: bool,
                REQ117_EN1: bool,
                REQ118_EN1: bool,
                REQ119_EN1: bool,
                REQ120_EN1: bool,
                REQ121_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE3_CLR {
                REQ96_EN1: self.REQ96_EN1(),
                REQ97_EN1: self.REQ97_EN1(),
                REQ98_EN1: self.REQ98_EN1(),
                REQ99_EN1: self.REQ99_EN1(),
                REQ100_EN1: self.REQ100_EN1(),
                REQ101_EN1: self.REQ101_EN1(),
                REQ102_EN1: self.REQ102_EN1(),
                REQ103_EN1: self.REQ103_EN1(),
                REQ104_EN1: self.REQ104_EN1(),
                REQ105_EN1: self.REQ105_EN1(),
                REQ106_EN1: self.REQ106_EN1(),
                REQ107_EN1: self.REQ107_EN1(),
                REQ108_EN1: self.REQ108_EN1(),
                REQ109_EN1: self.REQ109_EN1(),
                REQ110_EN1: self.REQ110_EN1(),
                REQ111_EN1: self.REQ111_EN1(),
                REQ112_EN1: self.REQ112_EN1(),
                REQ113_EN1: self.REQ113_EN1(),
                REQ114_EN1: self.REQ114_EN1(),
                REQ115_EN1: self.REQ115_EN1(),
                REQ116_EN1: self.REQ116_EN1(),
                REQ117_EN1: self.REQ117_EN1(),
                REQ118_EN1: self.REQ118_EN1(),
                REQ119_EN1: self.REQ119_EN1(),
                REQ120_EN1: self.REQ120_EN1(),
                REQ121_EN1: self.REQ121_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA1 Request Enable3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA1_REQ_ENABLE3_SET(pub u32);
    impl DMA1_REQ_ENABLE3_SET {
        #[inline(always)]
        pub const fn REQ96_EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ96_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ97_EN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ97_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REQ98_EN1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ98_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REQ99_EN1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ99_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REQ100_EN1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ100_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REQ101_EN1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ101_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REQ102_EN1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ102_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REQ103_EN1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ103_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REQ104_EN1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ104_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REQ105_EN1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ105_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REQ106_EN1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ106_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REQ107_EN1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ107_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REQ108_EN1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ108_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REQ109_EN1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ109_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REQ110_EN1(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ110_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REQ111_EN1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ111_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REQ112_EN1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ112_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REQ113_EN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ113_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REQ114_EN1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ114_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REQ115_EN1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ115_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REQ116_EN1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ116_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REQ117_EN1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ117_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REQ118_EN1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ118_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REQ119_EN1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ119_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REQ120_EN1(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ120_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REQ121_EN1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ121_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for DMA1_REQ_ENABLE3_SET {
        #[inline(always)]
        fn default() -> DMA1_REQ_ENABLE3_SET {
            DMA1_REQ_ENABLE3_SET(0)
        }
    }
    impl core::fmt::Debug for DMA1_REQ_ENABLE3_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA1_REQ_ENABLE3_SET")
                .field("REQ96_EN1", &self.REQ96_EN1())
                .field("REQ97_EN1", &self.REQ97_EN1())
                .field("REQ98_EN1", &self.REQ98_EN1())
                .field("REQ99_EN1", &self.REQ99_EN1())
                .field("REQ100_EN1", &self.REQ100_EN1())
                .field("REQ101_EN1", &self.REQ101_EN1())
                .field("REQ102_EN1", &self.REQ102_EN1())
                .field("REQ103_EN1", &self.REQ103_EN1())
                .field("REQ104_EN1", &self.REQ104_EN1())
                .field("REQ105_EN1", &self.REQ105_EN1())
                .field("REQ106_EN1", &self.REQ106_EN1())
                .field("REQ107_EN1", &self.REQ107_EN1())
                .field("REQ108_EN1", &self.REQ108_EN1())
                .field("REQ109_EN1", &self.REQ109_EN1())
                .field("REQ110_EN1", &self.REQ110_EN1())
                .field("REQ111_EN1", &self.REQ111_EN1())
                .field("REQ112_EN1", &self.REQ112_EN1())
                .field("REQ113_EN1", &self.REQ113_EN1())
                .field("REQ114_EN1", &self.REQ114_EN1())
                .field("REQ115_EN1", &self.REQ115_EN1())
                .field("REQ116_EN1", &self.REQ116_EN1())
                .field("REQ117_EN1", &self.REQ117_EN1())
                .field("REQ118_EN1", &self.REQ118_EN1())
                .field("REQ119_EN1", &self.REQ119_EN1())
                .field("REQ120_EN1", &self.REQ120_EN1())
                .field("REQ121_EN1", &self.REQ121_EN1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA1_REQ_ENABLE3_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMA1_REQ_ENABLE3_SET {
                REQ96_EN1: bool,
                REQ97_EN1: bool,
                REQ98_EN1: bool,
                REQ99_EN1: bool,
                REQ100_EN1: bool,
                REQ101_EN1: bool,
                REQ102_EN1: bool,
                REQ103_EN1: bool,
                REQ104_EN1: bool,
                REQ105_EN1: bool,
                REQ106_EN1: bool,
                REQ107_EN1: bool,
                REQ108_EN1: bool,
                REQ109_EN1: bool,
                REQ110_EN1: bool,
                REQ111_EN1: bool,
                REQ112_EN1: bool,
                REQ113_EN1: bool,
                REQ114_EN1: bool,
                REQ115_EN1: bool,
                REQ116_EN1: bool,
                REQ117_EN1: bool,
                REQ118_EN1: bool,
                REQ119_EN1: bool,
                REQ120_EN1: bool,
                REQ121_EN1: bool,
            }
            let proxy = DMA1_REQ_ENABLE3_SET {
                REQ96_EN1: self.REQ96_EN1(),
                REQ97_EN1: self.REQ97_EN1(),
                REQ98_EN1: self.REQ98_EN1(),
                REQ99_EN1: self.REQ99_EN1(),
                REQ100_EN1: self.REQ100_EN1(),
                REQ101_EN1: self.REQ101_EN1(),
                REQ102_EN1: self.REQ102_EN1(),
                REQ103_EN1: self.REQ103_EN1(),
                REQ104_EN1: self.REQ104_EN1(),
                REQ105_EN1: self.REQ105_EN1(),
                REQ106_EN1: self.REQ106_EN1(),
                REQ107_EN1: self.REQ107_EN1(),
                REQ108_EN1: self.REQ108_EN1(),
                REQ109_EN1: self.REQ109_EN1(),
                REQ110_EN1: self.REQ110_EN1(),
                REQ111_EN1: self.REQ111_EN1(),
                REQ112_EN1: self.REQ112_EN1(),
                REQ113_EN1: self.REQ113_EN1(),
                REQ114_EN1: self.REQ114_EN1(),
                REQ115_EN1: self.REQ115_EN1(),
                REQ116_EN1: self.REQ116_EN1(),
                REQ117_EN1: self.REQ117_EN1(),
                REQ118_EN1: self.REQ118_EN1(),
                REQ119_EN1: self.REQ119_EN1(),
                REQ120_EN1: self.REQ120_EN1(),
                REQ121_EN1: self.REQ121_EN1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EVTG Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_TRIG(pub u32);
    impl EVTG_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for EVTG_TRIG {
        #[inline(always)]
        fn default() -> EVTG_TRIG {
            EVTG_TRIG(0)
        }
    }
    impl core::fmt::Debug for EVTG_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EVTG_TRIG {
                INP: u8,
            }
            let proxy = EVTG_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXT Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EXT_TRIG(pub u32);
    impl EXT_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for EXT_TRIG {
        #[inline(always)]
        fn default() -> EXT_TRIG {
            EXT_TRIG(0)
        }
    }
    impl core::fmt::Debug for EXT_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EXT_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EXT_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EXT_TRIG {
                INP: u8,
            }
            let proxy = EXT_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM0_TRIG(pub u32);
    impl FLEXCOMM0_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM0_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM0_TRIG {
            FLEXCOMM0_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM0_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM0_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM0_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM1_TRIG(pub u32);
    impl FLEXCOMM1_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM1_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM1_TRIG {
            FLEXCOMM1_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM1_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM1_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM1_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM1_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM1_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM2 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM2_TRIG(pub u32);
    impl FLEXCOMM2_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM2_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM2_TRIG {
            FLEXCOMM2_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM2_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM2_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM2_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM2_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM2_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM3 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM3_TRIG(pub u32);
    impl FLEXCOMM3_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM3_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM3_TRIG {
            FLEXCOMM3_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM3_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM3_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM3_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM3_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM3_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM4 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM4_TRIG(pub u32);
    impl FLEXCOMM4_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM4_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM4_TRIG {
            FLEXCOMM4_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM4_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM4_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM4_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM4_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM4_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM5 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM5_TRIG(pub u32);
    impl FLEXCOMM5_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM5_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM5_TRIG {
            FLEXCOMM5_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM5_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM5_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM5_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM5_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM5_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM6 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM6_TRIG(pub u32);
    impl FLEXCOMM6_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM6_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM6_TRIG {
            FLEXCOMM6_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM6_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM6_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM6_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM6_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM6_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM7 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM7_TRIG(pub u32);
    impl FLEXCOMM7_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM7_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM7_TRIG {
            FLEXCOMM7_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM7_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM7_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM7_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM7_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM7_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM8 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM8_TRIG(pub u32);
    impl FLEXCOMM8_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM8_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM8_TRIG {
            FLEXCOMM8_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM8_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM8_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM8_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM8_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM8_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "LP_FLEXCOMM9 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXCOMM9_TRIG(pub u32);
    impl FLEXCOMM9_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXCOMM9_TRIG {
        #[inline(always)]
        fn default() -> FLEXCOMM9_TRIG {
            FLEXCOMM9_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXCOMM9_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXCOMM9_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXCOMM9_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXCOMM9_TRIG {
                INP: u8,
            }
            let proxy = FLEXCOMM9_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FlexIO Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXIO_TRIG(pub u32);
    impl FLEXIO_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for FLEXIO_TRIG {
        #[inline(always)]
        fn default() -> FLEXIO_TRIG {
            FLEXIO_TRIG(0)
        }
    }
    impl core::fmt::Debug for FLEXIO_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXIO_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXIO_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXIO_TRIG {
                INP: u8,
            }
            let proxy = FLEXIO_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 External Force Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_EXTFORCE(pub u32);
    impl FLEXPWM0_EXTFORCE {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM0_EXTFORCE {
        #[inline(always)]
        fn default() -> FLEXPWM0_EXTFORCE {
            FLEXPWM0_EXTFORCE(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_EXTFORCE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_EXTFORCE")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_EXTFORCE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM0_EXTFORCE {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM0_EXTFORCE {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 Fault Input Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FAULT(pub u32);
    impl FLEXPWM0_FAULT {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM0_FAULT {
        #[inline(always)]
        fn default() -> FLEXPWM0_FAULT {
            FLEXPWM0_FAULT(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_FAULT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FAULT")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FAULT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM0_FAULT {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM0_FAULT {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 Input Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM_EXTA(pub u32);
    impl FLEXPWM0_SM_EXTA {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM0_SM_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM_EXTA {
            FLEXPWM0_SM_EXTA(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM_EXTA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM_EXTA")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM_EXTA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM0_SM_EXTA {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM0_SM_EXTA {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 External Synchronization"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM_EXTSYNC(pub u32);
    impl FLEXPWM0_SM_EXTSYNC {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM0_SM_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM_EXTSYNC {
            FLEXPWM0_SM_EXTSYNC(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM_EXTSYNC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM_EXTSYNC")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM_EXTSYNC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM0_SM_EXTSYNC {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM0_SM_EXTSYNC {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 External Force Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_EXTFORCE(pub u32);
    impl FLEXPWM1_EXTFORCE {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM1_EXTFORCE {
        #[inline(always)]
        fn default() -> FLEXPWM1_EXTFORCE {
            FLEXPWM1_EXTFORCE(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM1_EXTFORCE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM1_EXTFORCE")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM1_EXTFORCE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM1_EXTFORCE {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM1_EXTFORCE {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 Fault Input Trigger Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_FAULT(pub u32);
    impl FLEXPWM1_FAULT {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM1_FAULT {
        #[inline(always)]
        fn default() -> FLEXPWM1_FAULT {
            FLEXPWM1_FAULT(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM1_FAULT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM1_FAULT")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM1_FAULT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM1_FAULT {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM1_FAULT {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 Input EXTA Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM_EXTA(pub u32);
    impl FLEXPWM1_SM_EXTA {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM1_SM_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM_EXTA {
            FLEXPWM1_SM_EXTA(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM1_SM_EXTA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM1_SM_EXTA")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM1_SM_EXTA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM1_SM_EXTA {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM1_SM_EXTA {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 External Synchronization"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM_EXTSYNC(pub u32);
    impl FLEXPWM1_SM_EXTSYNC {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FLEXPWM1_SM_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM_EXTSYNC {
            FLEXPWM1_SM_EXTSYNC(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM1_SM_EXTSYNC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM1_SM_EXTSYNC")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM1_SM_EXTSYNC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FLEXPWM1_SM_EXTSYNC {
                TRIGIN: u8,
            }
            let proxy = FLEXPWM1_SM_EXTSYNC {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Selection for Frequency Measurement Reference Clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_REF(pub u32);
    impl FREQMEAS_REF {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FREQMEAS_REF {
        #[inline(always)]
        fn default() -> FREQMEAS_REF {
            FREQMEAS_REF(0)
        }
    }
    impl core::fmt::Debug for FREQMEAS_REF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FREQMEAS_REF")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FREQMEAS_REF {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FREQMEAS_REF {
                INP: u8,
            }
            let proxy = FREQMEAS_REF { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Selection for Frequency Measurement Target Clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_TAR(pub u32);
    impl FREQMEAS_TAR {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for FREQMEAS_TAR {
        #[inline(always)]
        fn default() -> FREQMEAS_TAR {
            FREQMEAS_TAR(0)
        }
    }
    impl core::fmt::Debug for FREQMEAS_TAR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FREQMEAS_TAR")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FREQMEAS_TAR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FREQMEAS_TAR {
                INP: u8,
            }
            let proxy = FREQMEAS_TAR { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "OPAMP Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OPAMP_TRIG(pub u32);
    impl OPAMP_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for OPAMP_TRIG {
        #[inline(always)]
        fn default() -> OPAMP_TRIG {
            OPAMP_TRIG(0)
        }
    }
    impl core::fmt::Debug for OPAMP_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OPAMP_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OPAMP_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OPAMP_TRIG {
                INP: u8,
            }
            let proxy = OPAMP_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Pin Interrupt Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINTSEL(pub u32);
    impl PINTSEL {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for PINTSEL {
        #[inline(always)]
        fn default() -> PINTSEL {
            PINTSEL(0)
        }
    }
    impl core::fmt::Debug for PINTSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PINTSEL").field("INP", &self.INP()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PINTSEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PINTSEL {
                INP: u8,
            }
            let proxy = PINTSEL { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM0 External Clock Trigger"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM0_EXT_CLK(pub u32);
    impl PWM0_EXT_CLK {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for PWM0_EXT_CLK {
        #[inline(always)]
        fn default() -> PWM0_EXT_CLK {
            PWM0_EXT_CLK(0)
        }
    }
    impl core::fmt::Debug for PWM0_EXT_CLK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM0_EXT_CLK")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM0_EXT_CLK {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWM0_EXT_CLK {
                TRIGIN: u8,
            }
            let proxy = PWM0_EXT_CLK {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWM1 External Clock Trigger"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM1_EXT_CLK(pub u32);
    impl PWM1_EXT_CLK {
        #[inline(always)]
        pub const fn TRIGIN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PWM1_EXT_CLK {
        #[inline(always)]
        fn default() -> PWM1_EXT_CLK {
            PWM1_EXT_CLK(0)
        }
    }
    impl core::fmt::Debug for PWM1_EXT_CLK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM1_EXT_CLK")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM1_EXT_CLK {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWM1_EXT_CLK {
                TRIGIN: u8,
            }
            let proxy = PWM1_EXT_CLK {
                TRIGIN: self.TRIGIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "QDC0 Input Connections..QDC1 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDCN_QDC_HOME(pub u32);
    impl QDCN_QDC_HOME {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for QDCN_QDC_HOME {
        #[inline(always)]
        fn default() -> QDCN_QDC_HOME {
            QDCN_QDC_HOME(0)
        }
    }
    impl core::fmt::Debug for QDCN_QDC_HOME {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDCN_QDC_HOME")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDCN_QDC_HOME {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct QDCN_QDC_HOME {
                INP: u8,
            }
            let proxy = QDCN_QDC_HOME { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "QDC0 Input Connections..QDC1 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDCN_QDC_INDEX(pub u32);
    impl QDCN_QDC_INDEX {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for QDCN_QDC_INDEX {
        #[inline(always)]
        fn default() -> QDCN_QDC_INDEX {
            QDCN_QDC_INDEX(0)
        }
    }
    impl core::fmt::Debug for QDCN_QDC_INDEX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDCN_QDC_INDEX")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDCN_QDC_INDEX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct QDCN_QDC_INDEX {
                INP: u8,
            }
            let proxy = QDCN_QDC_INDEX { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "QDC0 Input Connections..QDC1 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDCN_QDC_PHASEA(pub u32);
    impl QDCN_QDC_PHASEA {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for QDCN_QDC_PHASEA {
        #[inline(always)]
        fn default() -> QDCN_QDC_PHASEA {
            QDCN_QDC_PHASEA(0)
        }
    }
    impl core::fmt::Debug for QDCN_QDC_PHASEA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDCN_QDC_PHASEA")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDCN_QDC_PHASEA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct QDCN_QDC_PHASEA {
                INP: u8,
            }
            let proxy = QDCN_QDC_PHASEA { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "QDC0 Input Connections..QDC1 Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDCN_QDC_PHASEB(pub u32);
    impl QDCN_QDC_PHASEB {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for QDCN_QDC_PHASEB {
        #[inline(always)]
        fn default() -> QDCN_QDC_PHASEB {
            QDCN_QDC_PHASEB(0)
        }
    }
    impl core::fmt::Debug for QDCN_QDC_PHASEB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDCN_QDC_PHASEB")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDCN_QDC_PHASEB {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct QDCN_QDC_PHASEB {
                INP: u8,
            }
            let proxy = QDCN_QDC_PHASEB { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "QDC0 Trigger Input Connections..QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDCN_QDC_TRIG(pub u32);
    impl QDCN_QDC_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for QDCN_QDC_TRIG {
        #[inline(always)]
        fn default() -> QDCN_QDC_TRIG {
            QDCN_QDC_TRIG(0)
        }
    }
    impl core::fmt::Debug for QDCN_QDC_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDCN_QDC_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDCN_QDC_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct QDCN_QDC_TRIG {
                INP: u8,
            }
            let proxy = QDCN_QDC_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Inputmux Register for SCT0 Input"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCT0_INMUX(pub u32);
    impl SCT0_INMUX {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for SCT0_INMUX {
        #[inline(always)]
        fn default() -> SCT0_INMUX {
            SCT0_INMUX(0)
        }
    }
    impl core::fmt::Debug for SCT0_INMUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCT0_INMUX")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCT0_INMUX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCT0_INMUX {
                INP: u8,
            }
            let proxy = SCT0_INMUX { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SINC_FILTER_CH(pub u32);
    impl SINC_FILTER_CH {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for SINC_FILTER_CH {
        #[inline(always)]
        fn default() -> SINC_FILTER_CH {
            SINC_FILTER_CH(0)
        }
    }
    impl core::fmt::Debug for SINC_FILTER_CH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SINC_FILTER_CH")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SINC_FILTER_CH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SINC_FILTER_CH {
                INP: u8,
            }
            let proxy = SINC_FILTER_CH { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Inputmux Register for SMARTDMA Arch B Inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMARTDMAARCHB_INMUX(pub u32);
    impl SMARTDMAARCHB_INMUX {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for SMARTDMAARCHB_INMUX {
        #[inline(always)]
        fn default() -> SMARTDMAARCHB_INMUX {
            SMARTDMAARCHB_INMUX(0)
        }
    }
    impl core::fmt::Debug for SMARTDMAARCHB_INMUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMARTDMAARCHB_INMUX")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMARTDMAARCHB_INMUX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SMARTDMAARCHB_INMUX {
                INP: u8,
            }
            let proxy = SMARTDMAARCHB_INMUX { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trigger Register for CTIMER"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER0TRIG(pub u32);
    impl TIMER0TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TIMER0TRIG {
        #[inline(always)]
        fn default() -> TIMER0TRIG {
            TIMER0TRIG(0)
        }
    }
    impl core::fmt::Debug for TIMER0TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER0TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER0TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TIMER0TRIG {
                INP: u8,
            }
            let proxy = TIMER0TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trigger Register for CTIMER"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER1TRIG(pub u32);
    impl TIMER1TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TIMER1TRIG {
        #[inline(always)]
        fn default() -> TIMER1TRIG {
            TIMER1TRIG(0)
        }
    }
    impl core::fmt::Debug for TIMER1TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER1TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER1TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TIMER1TRIG {
                INP: u8,
            }
            let proxy = TIMER1TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trigger Register for CTIMER"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER2TRIG(pub u32);
    impl TIMER2TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TIMER2TRIG {
        #[inline(always)]
        fn default() -> TIMER2TRIG {
            TIMER2TRIG(0)
        }
    }
    impl core::fmt::Debug for TIMER2TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER2TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER2TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TIMER2TRIG {
                INP: u8,
            }
            let proxy = TIMER2TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trigger Register for CTIMER"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER3TRIG(pub u32);
    impl TIMER3TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TIMER3TRIG {
        #[inline(always)]
        fn default() -> TIMER3TRIG {
            TIMER3TRIG(0)
        }
    }
    impl core::fmt::Debug for TIMER3TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER3TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER3TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TIMER3TRIG {
                INP: u8,
            }
            let proxy = TIMER3TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trigger Register for CTIMER"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER4TRIG(pub u32);
    impl TIMER4TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for TIMER4TRIG {
        #[inline(always)]
        fn default() -> TIMER4TRIG {
            TIMER4TRIG(0)
        }
    }
    impl core::fmt::Debug for TIMER4TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER4TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER4TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TIMER4TRIG {
                INP: u8,
            }
            let proxy = TIMER4TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TSI Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TSI_TRIG(pub u32);
    impl TSI_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for TSI_TRIG {
        #[inline(always)]
        fn default() -> TSI_TRIG {
            TSI_TRIG(0)
        }
    }
    impl core::fmt::Debug for TSI_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TSI_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TSI_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TSI_TRIG {
                INP: u8,
            }
            let proxy = TSI_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB-FS Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBFS_TRIG(pub u32);
    impl USBFS_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for USBFS_TRIG {
        #[inline(always)]
        fn default() -> USBFS_TRIG {
            USBFS_TRIG(0)
        }
    }
    impl core::fmt::Debug for USBFS_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBFS_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBFS_TRIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USBFS_TRIG {
                INP: u8,
            }
            let proxy = USBFS_TRIG { INP: self.INP() };
            defmt::write!(f, "{}", proxy)
        }
    }
}
