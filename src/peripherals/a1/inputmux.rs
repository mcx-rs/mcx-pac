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
    pub const fn CTIMER0CAP(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER0TRIG(self) -> crate::common::Reg<regs::TIMER0TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER1TRIG(self) -> crate::common::Reg<regs::TIMER1TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER2TRIG(self) -> crate::common::Reg<regs::TIMER2TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
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
    pub const fn CTIMER3CAP(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER3TRIG(self) -> crate::common::Reg<regs::TIMER3TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER4CAP(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER4TRIG(self) -> crate::common::Reg<regs::TIMER4TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[inline(always)]
    pub const fn AOI1_INPUT(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
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
    pub const fn QDC0_TRIG(self) -> crate::common::Reg<regs::QDC0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_HOME(self) -> crate::common::Reg<regs::QDC0_HOME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_INDEX(self) -> crate::common::Reg<regs::QDC0_INDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0368usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_PHASEB(self) -> crate::common::Reg<regs::QDC0_PHASEB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_PHASEA(self) -> crate::common::Reg<regs::QDC0_PHASEA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_ICAP1(self) -> crate::common::Reg<regs::QDC0_ICAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_ICAP2(self) -> crate::common::Reg<regs::QDC0_ICAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC0_ICAP3(self) -> crate::common::Reg<regs::QDC0_ICAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_TRIG(self) -> crate::common::Reg<regs::QDC1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_HOME(self) -> crate::common::Reg<regs::QDC1_HOME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_INDEX(self) -> crate::common::Reg<regs::QDC1_INDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_PHASEB(self) -> crate::common::Reg<regs::QDC1_PHASEB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_PHASEA(self) -> crate::common::Reg<regs::QDC1_PHASEA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_ICAP1(self) -> crate::common::Reg<regs::QDC1_ICAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_ICAP2(self) -> crate::common::Reg<regs::QDC1_ICAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[inline(always)]
    pub const fn QDC1_ICAP3(self) -> crate::common::Reg<regs::QDC1_ICAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM0_EXTA0(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM0_EXTA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM0_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM0_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM1_EXTA(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM1_EXTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM1_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM1_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM2_EXTA(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM2_EXTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM2_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM2_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLEXPWM0_FAULT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FORCE(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FORCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM0_EXTA0(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM0_EXTA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM0_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM0_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM1_EXTA(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM1_EXTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM1_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM1_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM2_EXTA(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM2_EXTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_SM2_EXTSYNC(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_SM2_EXTSYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_FAULT(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FLEXPWM1_FAULT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM1_FORCE(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM1_FORCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
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
    pub const fn AOI0_INPUT(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn USBFS_TRIG(self) -> crate::common::Reg<regs::USBFS_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
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
    pub const fn LPI2C2_TRIG(self) -> crate::common::Reg<regs::LPI2C2_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[inline(always)]
    pub const fn OPAMP0_TRIG(self) -> crate::common::Reg<regs::OPAMP0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[inline(always)]
    pub const fn LPI2C0_TRIG(self) -> crate::common::Reg<regs::LPI2C0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[inline(always)]
    pub const fn LPI2C1_TRIG(self) -> crate::common::Reg<regs::LPI2C1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[inline(always)]
    pub const fn LPSPI0_TRIG(self) -> crate::common::Reg<regs::LPSPI0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[inline(always)]
    pub const fn LPSPI1_TRIG(self) -> crate::common::Reg<regs::LPSPI1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn LPUART0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[inline(always)]
    pub const fn LPUART1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[inline(always)]
    pub const fn LPUART2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[inline(always)]
    pub const fn LPUART3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[inline(always)]
    pub const fn LPUART4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXIO_TRIG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC Trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC0_TRIG(pub u32);
    impl ADC0_TRIG {
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
    impl Default for ADC0_TRIG {
        #[inline(always)]
        fn default() -> ADC0_TRIG {
            ADC0_TRIG(0)
        }
    }
    #[doc = "ADC Trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADC1_TRIG(pub u32);
    impl ADC1_TRIG {
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
    impl Default for ADC1_TRIG {
        #[inline(always)]
        fn default() -> ADC1_TRIG {
            ADC1_TRIG(0)
        }
    }
    #[doc = "AOI0 trigger input connections 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AOI0_INPUT(pub u32);
    impl AOI0_INPUT {
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
    impl Default for AOI0_INPUT {
        #[inline(always)]
        fn default() -> AOI0_INPUT {
            AOI0_INPUT(0)
        }
    }
    #[doc = "AOI1 trigger input connections 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AOI1_INPUT(pub u32);
    impl AOI1_INPUT {
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
    impl Default for AOI1_INPUT {
        #[inline(always)]
        fn default() -> AOI1_INPUT {
            AOI1_INPUT(0)
        }
    }
    #[doc = "CMP0 input connections"]
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
    #[doc = "CMP1 input connections"]
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
    #[doc = "Capture select register for CTIMER inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER0CAP(pub u32);
    impl CTIMER0CAP {
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
    impl Default for CTIMER0CAP {
        #[inline(always)]
        fn default() -> CTIMER0CAP {
            CTIMER0CAP(0)
        }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER1CAP(pub u32);
    impl CTIMER1CAP {
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
    impl Default for CTIMER1CAP {
        #[inline(always)]
        fn default() -> CTIMER1CAP {
            CTIMER1CAP(0)
        }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER2CAP(pub u32);
    impl CTIMER2CAP {
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
    impl Default for CTIMER2CAP {
        #[inline(always)]
        fn default() -> CTIMER2CAP {
            CTIMER2CAP(0)
        }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER3CAP(pub u32);
    impl CTIMER3CAP {
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
    impl Default for CTIMER3CAP {
        #[inline(always)]
        fn default() -> CTIMER3CAP {
            CTIMER3CAP(0)
        }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMER4CAP(pub u32);
    impl CTIMER4CAP {
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
    impl Default for CTIMER4CAP {
        #[inline(always)]
        fn default() -> CTIMER4CAP {
            CTIMER4CAP(0)
        }
    }
    #[doc = "This register selects the DAC0 trigger inputs."]
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
    #[doc = "EXT trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EXT_TRIG(pub u32);
    impl EXT_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for EXT_TRIG {
        #[inline(always)]
        fn default() -> EXT_TRIG {
            EXT_TRIG(0)
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
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FORCE(pub u32);
    impl FLEXPWM0_FORCE {
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
    impl Default for FLEXPWM0_FORCE {
        #[inline(always)]
        fn default() -> FLEXPWM0_FORCE {
            FLEXPWM0_FORCE(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM0_EXTA0(pub u32);
    impl FLEXPWM0_SM0_EXTA0 {
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
    impl Default for FLEXPWM0_SM0_EXTA0 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM0_EXTA0 {
            FLEXPWM0_SM0_EXTA0(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM0_EXTSYNC(pub u32);
    impl FLEXPWM0_SM0_EXTSYNC {
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
    impl Default for FLEXPWM0_SM0_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM0_EXTSYNC {
            FLEXPWM0_SM0_EXTSYNC(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM1_EXTA(pub u32);
    impl FLEXPWM0_SM1_EXTA {
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
    impl Default for FLEXPWM0_SM1_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM1_EXTA {
            FLEXPWM0_SM1_EXTA(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM1_EXTSYNC(pub u32);
    impl FLEXPWM0_SM1_EXTSYNC {
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
    impl Default for FLEXPWM0_SM1_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM1_EXTSYNC {
            FLEXPWM0_SM1_EXTSYNC(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM2_EXTA(pub u32);
    impl FLEXPWM0_SM2_EXTA {
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
    impl Default for FLEXPWM0_SM2_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM2_EXTA {
            FLEXPWM0_SM2_EXTA(0)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM2_EXTSYNC(pub u32);
    impl FLEXPWM0_SM2_EXTSYNC {
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
    impl Default for FLEXPWM0_SM2_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM2_EXTSYNC {
            FLEXPWM0_SM2_EXTSYNC(0)
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
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_FORCE(pub u32);
    impl FLEXPWM1_FORCE {
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
    impl Default for FLEXPWM1_FORCE {
        #[inline(always)]
        fn default() -> FLEXPWM1_FORCE {
            FLEXPWM1_FORCE(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM0_EXTA0(pub u32);
    impl FLEXPWM1_SM0_EXTA0 {
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
    impl Default for FLEXPWM1_SM0_EXTA0 {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM0_EXTA0 {
            FLEXPWM1_SM0_EXTA0(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM0_EXTSYNC(pub u32);
    impl FLEXPWM1_SM0_EXTSYNC {
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
    impl Default for FLEXPWM1_SM0_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM0_EXTSYNC {
            FLEXPWM1_SM0_EXTSYNC(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM1_EXTA(pub u32);
    impl FLEXPWM1_SM1_EXTA {
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
    impl Default for FLEXPWM1_SM1_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM1_EXTA {
            FLEXPWM1_SM1_EXTA(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM1_EXTSYNC(pub u32);
    impl FLEXPWM1_SM1_EXTSYNC {
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
    impl Default for FLEXPWM1_SM1_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM1_EXTSYNC {
            FLEXPWM1_SM1_EXTSYNC(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM2_EXTA(pub u32);
    impl FLEXPWM1_SM2_EXTA {
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
    impl Default for FLEXPWM1_SM2_EXTA {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM2_EXTA {
            FLEXPWM1_SM2_EXTA(0)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM1_SM2_EXTSYNC(pub u32);
    impl FLEXPWM1_SM2_EXTSYNC {
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
    impl Default for FLEXPWM1_SM2_EXTSYNC {
        #[inline(always)]
        fn default() -> FLEXPWM1_SM2_EXTSYNC {
            FLEXPWM1_SM2_EXTSYNC(0)
        }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_REF(pub u32);
    impl FREQMEAS_REF {
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
    impl Default for FREQMEAS_REF {
        #[inline(always)]
        fn default() -> FREQMEAS_REF {
            FREQMEAS_REF(0)
        }
    }
    #[doc = "Selection for frequency measurement target clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_TAR(pub u32);
    impl FREQMEAS_TAR {
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
    impl Default for FREQMEAS_TAR {
        #[inline(always)]
        fn default() -> FREQMEAS_TAR {
            FREQMEAS_TAR(0)
        }
    }
    #[doc = "LPI2C0 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPI2C0_TRIG(pub u32);
    impl LPI2C0_TRIG {
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
    impl Default for LPI2C0_TRIG {
        #[inline(always)]
        fn default() -> LPI2C0_TRIG {
            LPI2C0_TRIG(0)
        }
    }
    #[doc = "LPI2C1 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPI2C1_TRIG(pub u32);
    impl LPI2C1_TRIG {
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
    impl Default for LPI2C1_TRIG {
        #[inline(always)]
        fn default() -> LPI2C1_TRIG {
            LPI2C1_TRIG(0)
        }
    }
    #[doc = "LPI2C2 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPI2C2_TRIG(pub u32);
    impl LPI2C2_TRIG {
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
    impl Default for LPI2C2_TRIG {
        #[inline(always)]
        fn default() -> LPI2C2_TRIG {
            LPI2C2_TRIG(0)
        }
    }
    #[doc = "LPSPI0 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPSPI0_TRIG(pub u32);
    impl LPSPI0_TRIG {
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
    impl Default for LPSPI0_TRIG {
        #[inline(always)]
        fn default() -> LPSPI0_TRIG {
            LPSPI0_TRIG(0)
        }
    }
    #[doc = "LPSPI1 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPSPI1_TRIG(pub u32);
    impl LPSPI1_TRIG {
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
    impl Default for LPSPI1_TRIG {
        #[inline(always)]
        fn default() -> LPSPI1_TRIG {
            LPSPI1_TRIG(0)
        }
    }
    #[doc = "LPUART0 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPUART0r(pub u32);
    impl LPUART0r {
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
    impl Default for LPUART0r {
        #[inline(always)]
        fn default() -> LPUART0r {
            LPUART0r(0)
        }
    }
    #[doc = "LPUART1 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPUART1r(pub u32);
    impl LPUART1r {
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
    impl Default for LPUART1r {
        #[inline(always)]
        fn default() -> LPUART1r {
            LPUART1r(0)
        }
    }
    #[doc = "LPUART2 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPUART2r(pub u32);
    impl LPUART2r {
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
    impl Default for LPUART2r {
        #[inline(always)]
        fn default() -> LPUART2r {
            LPUART2r(0)
        }
    }
    #[doc = "LPUART3 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPUART3r(pub u32);
    impl LPUART3r {
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
    impl Default for LPUART3r {
        #[inline(always)]
        fn default() -> LPUART3r {
            LPUART3r(0)
        }
    }
    #[doc = "LPUART4 trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPUART4r(pub u32);
    impl LPUART4r {
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
    impl Default for LPUART4r {
        #[inline(always)]
        fn default() -> LPUART4r {
            LPUART4r(0)
        }
    }
    #[doc = "OPAMP0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OPAMP0_TRIG(pub u32);
    impl OPAMP0_TRIG {
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
    impl Default for OPAMP0_TRIG {
        #[inline(always)]
        fn default() -> OPAMP0_TRIG {
            OPAMP0_TRIG(0)
        }
    }
    #[doc = "PWM0 external clock trigger"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM0_EXT_CLK(pub u32);
    impl PWM0_EXT_CLK {
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
    impl Default for PWM0_EXT_CLK {
        #[inline(always)]
        fn default() -> PWM0_EXT_CLK {
            PWM0_EXT_CLK(0)
        }
    }
    #[doc = "PWM1 external clock trigger"]
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
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_HOME(pub u32);
    impl QDC0_HOME {
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
    impl Default for QDC0_HOME {
        #[inline(always)]
        fn default() -> QDC0_HOME {
            QDC0_HOME(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP1(pub u32);
    impl QDC0_ICAP1 {
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
    impl Default for QDC0_ICAP1 {
        #[inline(always)]
        fn default() -> QDC0_ICAP1 {
            QDC0_ICAP1(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP2(pub u32);
    impl QDC0_ICAP2 {
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
    impl Default for QDC0_ICAP2 {
        #[inline(always)]
        fn default() -> QDC0_ICAP2 {
            QDC0_ICAP2(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP3(pub u32);
    impl QDC0_ICAP3 {
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
    impl Default for QDC0_ICAP3 {
        #[inline(always)]
        fn default() -> QDC0_ICAP3 {
            QDC0_ICAP3(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_INDEX(pub u32);
    impl QDC0_INDEX {
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
    impl Default for QDC0_INDEX {
        #[inline(always)]
        fn default() -> QDC0_INDEX {
            QDC0_INDEX(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_PHASEA(pub u32);
    impl QDC0_PHASEA {
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
    impl Default for QDC0_PHASEA {
        #[inline(always)]
        fn default() -> QDC0_PHASEA {
            QDC0_PHASEA(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_PHASEB(pub u32);
    impl QDC0_PHASEB {
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
    impl Default for QDC0_PHASEB {
        #[inline(always)]
        fn default() -> QDC0_PHASEB {
            QDC0_PHASEB(0)
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_TRIG(pub u32);
    impl QDC0_TRIG {
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
    impl Default for QDC0_TRIG {
        #[inline(always)]
        fn default() -> QDC0_TRIG {
            QDC0_TRIG(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_HOME(pub u32);
    impl QDC1_HOME {
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
    impl Default for QDC1_HOME {
        #[inline(always)]
        fn default() -> QDC1_HOME {
            QDC1_HOME(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_ICAP1(pub u32);
    impl QDC1_ICAP1 {
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
    impl Default for QDC1_ICAP1 {
        #[inline(always)]
        fn default() -> QDC1_ICAP1 {
            QDC1_ICAP1(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_ICAP2(pub u32);
    impl QDC1_ICAP2 {
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
    impl Default for QDC1_ICAP2 {
        #[inline(always)]
        fn default() -> QDC1_ICAP2 {
            QDC1_ICAP2(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_ICAP3(pub u32);
    impl QDC1_ICAP3 {
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
    impl Default for QDC1_ICAP3 {
        #[inline(always)]
        fn default() -> QDC1_ICAP3 {
            QDC1_ICAP3(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_INDEX(pub u32);
    impl QDC1_INDEX {
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
    impl Default for QDC1_INDEX {
        #[inline(always)]
        fn default() -> QDC1_INDEX {
            QDC1_INDEX(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_PHASEA(pub u32);
    impl QDC1_PHASEA {
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
    impl Default for QDC1_PHASEA {
        #[inline(always)]
        fn default() -> QDC1_PHASEA {
            QDC1_PHASEA(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_PHASEB(pub u32);
    impl QDC1_PHASEB {
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
    impl Default for QDC1_PHASEB {
        #[inline(always)]
        fn default() -> QDC1_PHASEB {
            QDC1_PHASEB(0)
        }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC1_TRIG(pub u32);
    impl QDC1_TRIG {
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
    impl Default for QDC1_TRIG {
        #[inline(always)]
        fn default() -> QDC1_TRIG {
            QDC1_TRIG(0)
        }
    }
    #[doc = "Trigger register for TIMER0"]
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
    #[doc = "Trigger register for TIMER1"]
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
    #[doc = "Trigger register for TIMER2 inputs"]
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
    #[doc = "Trigger register for TIMER3"]
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
    #[doc = "Trigger register for TIMER4"]
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
    #[doc = "USB-FS trigger input connections"]
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
}
