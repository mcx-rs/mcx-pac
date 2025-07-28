#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPCMP {
    ptr: *mut u8,
}
unsafe impl Send for LPCMP {}
unsafe impl Sync for LPCMP {}
impl LPCMP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<regs::VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CCR0(self) -> crate::common::Reg<regs::CCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CCR1(self) -> crate::common::Reg<regs::CCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CCR2(self) -> crate::common::Reg<regs::CCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn DCR(self) -> crate::common::Reg<regs::DCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<regs::CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn RRCR0(self) -> crate::common::Reg<regs::RRCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn RRCR1(self) -> crate::common::Reg<regs::RRCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn RRCSR(self) -> crate::common::Reg<regs::RRCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn RRSR(self) -> crate::common::Reg<regs::RRSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn RRCR2(self) -> crate::common::Reg<regs::RRCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator Control Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR0(pub u32);
    impl CCR0 {
        #[must_use]
        #[inline(always)]
        pub const fn CMP_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CCR0 {
        #[inline(always)]
        fn default() -> CCR0 {
            CCR0(0)
        }
    }
    impl core::fmt::Debug for CCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CCR0")
                .field("CMP_EN", &self.CMP_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CCR0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CCR0 {{ CMP_EN: {=bool:?} }}", self.CMP_EN())
        }
    }
    #[doc = "Comparator Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR1(pub u32);
    impl CCR1 {
        #[must_use]
        #[inline(always)]
        pub const fn WINDOW_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WINDOW_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAMPLE_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SAMPLE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMA_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMA_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUT_INV(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUT_INV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUT_SEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUT_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUT_PEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUT_PEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUTA_OWEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUTA_OWEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUTA_OW(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUTA_OW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WINDOW_INV(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WINDOW_INV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WINDOW_CLS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WINDOW_CLS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVT_SEL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVT_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FUNC_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FUNC_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for CCR1 {
        #[inline(always)]
        fn default() -> CCR1 {
            CCR1(0)
        }
    }
    impl core::fmt::Debug for CCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CCR1")
                .field("WINDOW_EN", &self.WINDOW_EN())
                .field("SAMPLE_EN", &self.SAMPLE_EN())
                .field("DMA_EN", &self.DMA_EN())
                .field("COUT_INV", &self.COUT_INV())
                .field("COUT_SEL", &self.COUT_SEL())
                .field("COUT_PEN", &self.COUT_PEN())
                .field("COUTA_OWEN", &self.COUTA_OWEN())
                .field("COUTA_OW", &self.COUTA_OW())
                .field("WINDOW_INV", &self.WINDOW_INV())
                .field("WINDOW_CLS", &self.WINDOW_CLS())
                .field("EVT_SEL", &self.EVT_SEL())
                .field("FUNC_CLK_SEL", &self.FUNC_CLK_SEL())
                .field("FILT_CNT", &self.FILT_CNT())
                .field("FILT_PER", &self.FILT_PER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CCR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CCR1 {{ WINDOW_EN: {=bool:?}, SAMPLE_EN: {=bool:?}, DMA_EN: {=bool:?}, COUT_INV: {=bool:?}, COUT_SEL: {=bool:?}, COUT_PEN: {=bool:?}, COUTA_OWEN: {=bool:?}, COUTA_OW: {=bool:?}, WINDOW_INV: {=bool:?}, WINDOW_CLS: {=bool:?}, EVT_SEL: {=u8:?}, FUNC_CLK_SEL: {=u8:?}, FILT_CNT: {=u8:?}, FILT_PER: {=u8:?} }}" , self . WINDOW_EN () , self . SAMPLE_EN () , self . DMA_EN () , self . COUT_INV () , self . COUT_SEL () , self . COUT_PEN () , self . COUTA_OWEN () , self . COUTA_OW () , self . WINDOW_INV () , self . WINDOW_CLS () , self . EVT_SEL () , self . FUNC_CLK_SEL () , self . FILT_CNT () , self . FILT_PER ())
        }
    }
    #[doc = "Comparator Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR2(pub u32);
    impl CCR2 {
        #[must_use]
        #[inline(always)]
        pub const fn CMP_HPMD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMP_HPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMP_NPMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CMP_NPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HYSTCTR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HYSTCTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PSEL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for CCR2 {
        #[inline(always)]
        fn default() -> CCR2 {
            CCR2(0)
        }
    }
    impl core::fmt::Debug for CCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CCR2")
                .field("CMP_HPMD", &self.CMP_HPMD())
                .field("CMP_NPMD", &self.CMP_NPMD())
                .field("HYSTCTR", &self.HYSTCTR())
                .field("PSEL", &self.PSEL())
                .field("MSEL", &self.MSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CCR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CCR2 {{ CMP_HPMD: {=bool:?}, CMP_NPMD: {=bool:?}, HYSTCTR: {=u8:?}, PSEL: {=u8:?}, MSEL: {=u8:?} }}" , self . CMP_HPMD () , self . CMP_NPMD () , self . HYSTCTR () , self . PSEL () , self . MSEL ())
        }
    }
    #[doc = "Comparator Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[must_use]
        #[inline(always)]
        pub const fn CFR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RRF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for CSR {
        #[inline(always)]
        fn default() -> CSR {
            CSR(0)
        }
    }
    impl core::fmt::Debug for CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSR")
                .field("CFR", &self.CFR())
                .field("CFF", &self.CFF())
                .field("RRF", &self.RRF())
                .field("COUT", &self.COUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CSR {{ CFR: {=bool:?}, CFF: {=bool:?}, RRF: {=bool:?}, COUT: {=bool:?} }}",
                self.CFR(),
                self.CFF(),
                self.RRF(),
                self.COUT()
            )
        }
    }
    #[doc = "DAC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DCR(pub u32);
    impl DCR {
        #[must_use]
        #[inline(always)]
        pub const fn DAC_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAC_HPMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAC_HPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VRSEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VRSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAC_DATA(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DAC_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DCR {
        #[inline(always)]
        fn default() -> DCR {
            DCR(0)
        }
    }
    impl core::fmt::Debug for DCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DCR")
                .field("DAC_EN", &self.DAC_EN())
                .field("DAC_HPMD", &self.DAC_HPMD())
                .field("VRSEL", &self.VRSEL())
                .field("DAC_DATA", &self.DAC_DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DCR {{ DAC_EN: {=bool:?}, DAC_HPMD: {=bool:?}, VRSEL: {=bool:?}, DAC_DATA: {=u8:?} }}" , self . DAC_EN () , self . DAC_HPMD () , self . VRSEL () , self . DAC_DATA ())
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[must_use]
        #[inline(always)]
        pub const fn CFR_IE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFF_IE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CFF_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RRF_IE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RRF_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for IER {
        #[inline(always)]
        fn default() -> IER {
            IER(0)
        }
    }
    impl core::fmt::Debug for IER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IER")
                .field("CFR_IE", &self.CFR_IE())
                .field("CFF_IE", &self.CFF_IE())
                .field("RRF_IE", &self.RRF_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IER {{ CFR_IE: {=bool:?}, CFF_IE: {=bool:?}, RRF_IE: {=bool:?} }}",
                self.CFR_IE(),
                self.CFF_IE(),
                self.RRF_IE()
            )
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn DAC_RES(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DAC_RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    impl core::fmt::Debug for PARAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAM")
                .field("DAC_RES", &self.DAC_RES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PARAM {{ DAC_RES: {=u8:?} }}", self.DAC_RES())
        }
    }
    #[doc = "Round Robin Control Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR0(pub u32);
    impl RRCR0 {
        #[must_use]
        #[inline(always)]
        pub const fn RR_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_TRG_SEL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_TRG_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_NSAM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RR_NSAM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RR_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_INITMOD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RR_INITMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_SAMPLE_CNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RR_SAMPLE_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_SAMPLE_THRESHOLD(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RR_SAMPLE_THRESHOLD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for RRCR0 {
        #[inline(always)]
        fn default() -> RRCR0 {
            RRCR0(0)
        }
    }
    impl core::fmt::Debug for RRCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RRCR0")
                .field("RR_EN", &self.RR_EN())
                .field("RR_TRG_SEL", &self.RR_TRG_SEL())
                .field("RR_NSAM", &self.RR_NSAM())
                .field("RR_CLK_SEL", &self.RR_CLK_SEL())
                .field("RR_INITMOD", &self.RR_INITMOD())
                .field("RR_SAMPLE_CNT", &self.RR_SAMPLE_CNT())
                .field("RR_SAMPLE_THRESHOLD", &self.RR_SAMPLE_THRESHOLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RRCR0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RRCR0 {{ RR_EN: {=bool:?}, RR_TRG_SEL: {=bool:?}, RR_NSAM: {=u8:?}, RR_CLK_SEL: {=u8:?}, RR_INITMOD: {=u8:?}, RR_SAMPLE_CNT: {=u8:?}, RR_SAMPLE_THRESHOLD: {=u8:?} }}" , self . RR_EN () , self . RR_TRG_SEL () , self . RR_NSAM () , self . RR_CLK_SEL () , self . RR_INITMOD () , self . RR_SAMPLE_CNT () , self . RR_SAMPLE_THRESHOLD ())
        }
    }
    #[doc = "Round Robin Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR1(pub u32);
    impl RRCR1 {
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH0EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH0EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH1EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH2EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH2EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH3EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH3EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH4EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH4EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH5EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH5EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH6EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH6EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH7EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH7EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIXP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIXP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIXCH(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIXCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for RRCR1 {
        #[inline(always)]
        fn default() -> RRCR1 {
            RRCR1(0)
        }
    }
    impl core::fmt::Debug for RRCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RRCR1")
                .field("RR_CH0EN", &self.RR_CH0EN())
                .field("RR_CH1EN", &self.RR_CH1EN())
                .field("RR_CH2EN", &self.RR_CH2EN())
                .field("RR_CH3EN", &self.RR_CH3EN())
                .field("RR_CH4EN", &self.RR_CH4EN())
                .field("RR_CH5EN", &self.RR_CH5EN())
                .field("RR_CH6EN", &self.RR_CH6EN())
                .field("RR_CH7EN", &self.RR_CH7EN())
                .field("FIXP", &self.FIXP())
                .field("FIXCH", &self.FIXCH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RRCR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RRCR1 {{ RR_CH0EN: {=bool:?}, RR_CH1EN: {=bool:?}, RR_CH2EN: {=bool:?}, RR_CH3EN: {=bool:?}, RR_CH4EN: {=bool:?}, RR_CH5EN: {=bool:?}, RR_CH6EN: {=bool:?}, RR_CH7EN: {=bool:?}, FIXP: {=bool:?}, FIXCH: {=u8:?} }}" , self . RR_CH0EN () , self . RR_CH1EN () , self . RR_CH2EN () , self . RR_CH3EN () , self . RR_CH4EN () , self . RR_CH5EN () , self . RR_CH6EN () , self . RR_CH7EN () , self . FIXP () , self . FIXCH ())
        }
    }
    #[doc = "Round Robin Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR2(pub u32);
    impl RRCR2 {
        #[must_use]
        #[inline(always)]
        pub const fn RR_TIMER_RELOAD(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RR_TIMER_RELOAD(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_TIMER_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_TIMER_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RRCR2 {
        #[inline(always)]
        fn default() -> RRCR2 {
            RRCR2(0)
        }
    }
    impl core::fmt::Debug for RRCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RRCR2")
                .field("RR_TIMER_RELOAD", &self.RR_TIMER_RELOAD())
                .field("RR_TIMER_EN", &self.RR_TIMER_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RRCR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RRCR2 {{ RR_TIMER_RELOAD: {=u32:?}, RR_TIMER_EN: {=bool:?} }}",
                self.RR_TIMER_RELOAD(),
                self.RR_TIMER_EN()
            )
        }
    }
    #[doc = "Round Robin Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCSR(pub u32);
    impl RRCSR {
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH0OUT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH0OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH1OUT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH1OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH2OUT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH2OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH3OUT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH3OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH4OUT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH4OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH5OUT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH5OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH6OUT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH6OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH7OUT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH7OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for RRCSR {
        #[inline(always)]
        fn default() -> RRCSR {
            RRCSR(0)
        }
    }
    impl core::fmt::Debug for RRCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RRCSR")
                .field("RR_CH0OUT", &self.RR_CH0OUT())
                .field("RR_CH1OUT", &self.RR_CH1OUT())
                .field("RR_CH2OUT", &self.RR_CH2OUT())
                .field("RR_CH3OUT", &self.RR_CH3OUT())
                .field("RR_CH4OUT", &self.RR_CH4OUT())
                .field("RR_CH5OUT", &self.RR_CH5OUT())
                .field("RR_CH6OUT", &self.RR_CH6OUT())
                .field("RR_CH7OUT", &self.RR_CH7OUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RRCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RRCSR {{ RR_CH0OUT: {=bool:?}, RR_CH1OUT: {=bool:?}, RR_CH2OUT: {=bool:?}, RR_CH3OUT: {=bool:?}, RR_CH4OUT: {=bool:?}, RR_CH5OUT: {=bool:?}, RR_CH6OUT: {=bool:?}, RR_CH7OUT: {=bool:?} }}" , self . RR_CH0OUT () , self . RR_CH1OUT () , self . RR_CH2OUT () , self . RR_CH3OUT () , self . RR_CH4OUT () , self . RR_CH5OUT () , self . RR_CH6OUT () , self . RR_CH7OUT ())
        }
    }
    #[doc = "Round Robin Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRSR(pub u32);
    impl RRSR {
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH0F(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH0F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH1F(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH2F(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH2F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH3F(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH3F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH4F(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH4F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH5F(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH5F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH6F(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH6F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RR_CH7F(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RR_CH7F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for RRSR {
        #[inline(always)]
        fn default() -> RRSR {
            RRSR(0)
        }
    }
    impl core::fmt::Debug for RRSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RRSR")
                .field("RR_CH0F", &self.RR_CH0F())
                .field("RR_CH1F", &self.RR_CH1F())
                .field("RR_CH2F", &self.RR_CH2F())
                .field("RR_CH3F", &self.RR_CH3F())
                .field("RR_CH4F", &self.RR_CH4F())
                .field("RR_CH5F", &self.RR_CH5F())
                .field("RR_CH6F", &self.RR_CH6F())
                .field("RR_CH7F", &self.RR_CH7F())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RRSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RRSR {{ RR_CH0F: {=bool:?}, RR_CH1F: {=bool:?}, RR_CH2F: {=bool:?}, RR_CH3F: {=bool:?}, RR_CH4F: {=bool:?}, RR_CH5F: {=bool:?}, RR_CH6F: {=bool:?}, RR_CH7F: {=bool:?} }}" , self . RR_CH0F () , self . RR_CH1F () , self . RR_CH2F () , self . RR_CH3F () , self . RR_CH4F () , self . RR_CH5F () , self . RR_CH6F () , self . RR_CH7F ())
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[must_use]
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAJOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for VERID {
        #[inline(always)]
        fn default() -> VERID {
            VERID(0)
        }
    }
    impl core::fmt::Debug for VERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERID")
                .field("FEATURE", &self.FEATURE())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERID {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
                self.FEATURE(),
                self.MINOR(),
                self.MAJOR()
            )
        }
    }
}
