#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
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
    pub const fn CTIMER0CAP(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CTIMER0CAP, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER0TRIG(self) -> crate::common::Reg<regs::TIMER0TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER1CAP(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CTIMER1CAP, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER1TRIG(self) -> crate::common::Reg<regs::TIMER1TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMER2CAP(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CTIMER2CAP, crate::common::RW> {
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
    pub const fn CMP0_TRIG(self) -> crate::common::Reg<regs::CMP0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn ADC0_TRIG(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ADC0_TRIG, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
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
    pub const fn FLEXPWM0_SM0_EXTA0(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM0_EXTA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM0_EXTSYNC0(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM0_EXTSYNC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM1_EXTA1(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM1_EXTA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM1_EXTSYNC1(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM1_EXTSYNC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM2_EXTA2(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM2_EXTA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_SM2_EXTSYNC2(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_SM2_EXTSYNC2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT0(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FAULT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT1(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FAULT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT2(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FAULT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c8usize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FAULT3(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FAULT3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ccusize) as _) }
    }
    #[inline(always)]
    pub const fn FLEXPWM0_FORCE(
        self,
    ) -> crate::common::Reg<regs::FLEXPWM0_FORCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM0_EXT_CLK(self) -> crate::common::Reg<regs::PWM0_EXT_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn AOI0_MUX(self, n: usize) -> crate::common::Reg<regs::AOI0_MUX, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn USBFS_TRIG(self) -> crate::common::Reg<regs::USBFS_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[inline(always)]
    pub const fn EXT_TRIG(self, n: usize) -> crate::common::Reg<regs::EXT_TRIG, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn EXT_TRIG6(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EXT_TRIG6, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d8usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CMP1_TRIG(self) -> crate::common::Reg<regs::CMP1_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[inline(always)]
    pub const fn LPI2C0_TRIG(self) -> crate::common::Reg<regs::LPI2C0_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
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
            defmt::write!(f, "ADC0_TRIG {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "AOI0 trigger input connections 0-15"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AOI0_MUX(pub u32);
    impl AOI0_MUX {
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
    impl Default for AOI0_MUX {
        #[inline(always)]
        fn default() -> AOI0_MUX {
            AOI0_MUX(0)
        }
    }
    impl core::fmt::Debug for AOI0_MUX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AOI0_MUX")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AOI0_MUX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AOI0_MUX {{ INP: {=u8:?} }}", self.INP())
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
            defmt::write!(f, "CMP0_TRIG {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
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
            defmt::write!(f, "CMP1_TRIG {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
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
    impl core::fmt::Debug for CTIMER0CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER0CAP")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER0CAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CTIMER0CAP {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for CTIMER1CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER1CAP")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER1CAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CTIMER1CAP {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for CTIMER2CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMER2CAP")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMER2CAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CTIMER2CAP {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "EXT trigger connections 0-4"]
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
            defmt::write!(f, "EXT_TRIG {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "EXT trigger connections 6-7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EXT_TRIG6(pub u32);
    impl EXT_TRIG6 {
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
    impl Default for EXT_TRIG6 {
        #[inline(always)]
        fn default() -> EXT_TRIG6 {
            EXT_TRIG6(0)
        }
    }
    impl core::fmt::Debug for EXT_TRIG6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EXT_TRIG6")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EXT_TRIG6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EXT_TRIG6 {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FAULT0(pub u32);
    impl FLEXPWM0_FAULT0 {
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
    impl Default for FLEXPWM0_FAULT0 {
        #[inline(always)]
        fn default() -> FLEXPWM0_FAULT0 {
            FLEXPWM0_FAULT0(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_FAULT0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FAULT0")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FAULT0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_FAULT0 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FAULT1(pub u32);
    impl FLEXPWM0_FAULT1 {
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
    impl Default for FLEXPWM0_FAULT1 {
        #[inline(always)]
        fn default() -> FLEXPWM0_FAULT1 {
            FLEXPWM0_FAULT1(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_FAULT1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FAULT1")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FAULT1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_FAULT1 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FAULT2(pub u32);
    impl FLEXPWM0_FAULT2 {
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
    impl Default for FLEXPWM0_FAULT2 {
        #[inline(always)]
        fn default() -> FLEXPWM0_FAULT2 {
            FLEXPWM0_FAULT2(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_FAULT2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FAULT2")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FAULT2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_FAULT2 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_FAULT3(pub u32);
    impl FLEXPWM0_FAULT3 {
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
    impl Default for FLEXPWM0_FAULT3 {
        #[inline(always)]
        fn default() -> FLEXPWM0_FAULT3 {
            FLEXPWM0_FAULT3(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_FAULT3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FAULT3")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FAULT3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_FAULT3 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
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
    impl core::fmt::Debug for FLEXPWM0_FORCE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_FORCE")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_FORCE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_FORCE {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
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
    impl core::fmt::Debug for FLEXPWM0_SM0_EXTA0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM0_EXTA0")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM0_EXTA0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_SM0_EXTA0 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM0_EXTSYNC0(pub u32);
    impl FLEXPWM0_SM0_EXTSYNC0 {
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
    impl Default for FLEXPWM0_SM0_EXTSYNC0 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM0_EXTSYNC0 {
            FLEXPWM0_SM0_EXTSYNC0(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM0_EXTSYNC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM0_EXTSYNC0")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM0_EXTSYNC0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLEXPWM0_SM0_EXTSYNC0 {{ TRIGIN: {=u8:?} }}",
                self.TRIGIN()
            )
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM1_EXTA1(pub u32);
    impl FLEXPWM0_SM1_EXTA1 {
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
    impl Default for FLEXPWM0_SM1_EXTA1 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM1_EXTA1 {
            FLEXPWM0_SM1_EXTA1(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM1_EXTA1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM1_EXTA1")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM1_EXTA1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_SM1_EXTA1 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM1_EXTSYNC1(pub u32);
    impl FLEXPWM0_SM1_EXTSYNC1 {
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
    impl Default for FLEXPWM0_SM1_EXTSYNC1 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM1_EXTSYNC1 {
            FLEXPWM0_SM1_EXTSYNC1(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM1_EXTSYNC1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM1_EXTSYNC1")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM1_EXTSYNC1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLEXPWM0_SM1_EXTSYNC1 {{ TRIGIN: {=u8:?} }}",
                self.TRIGIN()
            )
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM2_EXTA2(pub u32);
    impl FLEXPWM0_SM2_EXTA2 {
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
    impl Default for FLEXPWM0_SM2_EXTA2 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM2_EXTA2 {
            FLEXPWM0_SM2_EXTA2(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM2_EXTA2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM2_EXTA2")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM2_EXTA2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FLEXPWM0_SM2_EXTA2 {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLEXPWM0_SM2_EXTSYNC2(pub u32);
    impl FLEXPWM0_SM2_EXTSYNC2 {
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
    impl Default for FLEXPWM0_SM2_EXTSYNC2 {
        #[inline(always)]
        fn default() -> FLEXPWM0_SM2_EXTSYNC2 {
            FLEXPWM0_SM2_EXTSYNC2(0)
        }
    }
    impl core::fmt::Debug for FLEXPWM0_SM2_EXTSYNC2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLEXPWM0_SM2_EXTSYNC2")
                .field("TRIGIN", &self.TRIGIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLEXPWM0_SM2_EXTSYNC2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLEXPWM0_SM2_EXTSYNC2 {{ TRIGIN: {=u8:?} }}",
                self.TRIGIN()
            )
        }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_REF(pub u32);
    impl FREQMEAS_REF {
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
            defmt::write!(f, "FREQMEAS_REF {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FREQMEAS_TAR(pub u32);
    impl FREQMEAS_TAR {
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
            defmt::write!(f, "FREQMEAS_TAR {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPI2C0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPI2C0_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPI2C0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPI2C0_TRIG {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPSPI0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPSPI0_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPSPI0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPSPI0_TRIG {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPSPI1_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPSPI1_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPSPI1_TRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPSPI1_TRIG {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPUART0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPUART0r")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPUART0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPUART0r {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPUART1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPUART1r")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPUART1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPUART1r {{ INP: {=u8:?} }}", self.INP())
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
    impl core::fmt::Debug for LPUART2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPUART2r")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPUART2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LPUART2r {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "PWM0 external clock trigger"]
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
            defmt::write!(f, "PWM0_EXT_CLK {{ TRIGIN: {=u8:?} }}", self.TRIGIN())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_HOME(pub u32);
    impl QDC0_HOME {
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
    impl Default for QDC0_HOME {
        #[inline(always)]
        fn default() -> QDC0_HOME {
            QDC0_HOME(0)
        }
    }
    impl core::fmt::Debug for QDC0_HOME {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_HOME")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_HOME {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_HOME {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP1(pub u32);
    impl QDC0_ICAP1 {
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
    impl Default for QDC0_ICAP1 {
        #[inline(always)]
        fn default() -> QDC0_ICAP1 {
            QDC0_ICAP1(0)
        }
    }
    impl core::fmt::Debug for QDC0_ICAP1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_ICAP1")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_ICAP1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_ICAP1 {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP2(pub u32);
    impl QDC0_ICAP2 {
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
    impl Default for QDC0_ICAP2 {
        #[inline(always)]
        fn default() -> QDC0_ICAP2 {
            QDC0_ICAP2(0)
        }
    }
    impl core::fmt::Debug for QDC0_ICAP2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_ICAP2")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_ICAP2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_ICAP2 {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_ICAP3(pub u32);
    impl QDC0_ICAP3 {
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
    impl Default for QDC0_ICAP3 {
        #[inline(always)]
        fn default() -> QDC0_ICAP3 {
            QDC0_ICAP3(0)
        }
    }
    impl core::fmt::Debug for QDC0_ICAP3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_ICAP3")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_ICAP3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_ICAP3 {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_INDEX(pub u32);
    impl QDC0_INDEX {
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
    impl Default for QDC0_INDEX {
        #[inline(always)]
        fn default() -> QDC0_INDEX {
            QDC0_INDEX(0)
        }
    }
    impl core::fmt::Debug for QDC0_INDEX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_INDEX")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_INDEX {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_INDEX {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_PHASEA(pub u32);
    impl QDC0_PHASEA {
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
    impl Default for QDC0_PHASEA {
        #[inline(always)]
        fn default() -> QDC0_PHASEA {
            QDC0_PHASEA(0)
        }
    }
    impl core::fmt::Debug for QDC0_PHASEA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_PHASEA")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_PHASEA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_PHASEA {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_PHASEB(pub u32);
    impl QDC0_PHASEB {
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
    impl Default for QDC0_PHASEB {
        #[inline(always)]
        fn default() -> QDC0_PHASEB {
            QDC0_PHASEB(0)
        }
    }
    impl core::fmt::Debug for QDC0_PHASEB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_PHASEB")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_PHASEB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_PHASEB {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QDC0_TRIG(pub u32);
    impl QDC0_TRIG {
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
    impl Default for QDC0_TRIG {
        #[inline(always)]
        fn default() -> QDC0_TRIG {
            QDC0_TRIG(0)
        }
    }
    impl core::fmt::Debug for QDC0_TRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QDC0_TRIG")
                .field("INP", &self.INP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QDC0_TRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QDC0_TRIG {{ INP: {=u8:?} }}", self.INP())
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
            defmt::write!(f, "TIMER0TRIG {{ INP: {=u8:?} }}", self.INP())
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
            defmt::write!(f, "TIMER1TRIG {{ INP: {=u8:?} }}", self.INP())
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
            defmt::write!(f, "TIMER2TRIG {{ INP: {=u8:?} }}", self.INP())
        }
    }
    #[doc = "USB-FS trigger input connections"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBFS_TRIG(pub u32);
    impl USBFS_TRIG {
        #[inline(always)]
        pub const fn INP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_INP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
            defmt::write!(f, "USBFS_TRIG {{ INP: {=u8:?} }}", self.INP())
        }
    }
}
