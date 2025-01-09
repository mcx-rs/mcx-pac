#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
        #[inline(always)]
        pub const fn CMP_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP_EN(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct CCR0 {
                CMP_EN: bool,
            }
            let proxy = CCR0 {
                CMP_EN: self.CMP_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Comparator Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR1(pub u32);
    impl CCR1 {
        #[inline(always)]
        pub const fn WINDOW_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WINDOW_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SAMPLE_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAMPLE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DMA_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMA_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn COUT_INV(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUT_INV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn COUT_SEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUT_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn COUT_PEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUT_PEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn COUTA_OWEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUTA_OWEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn COUTA_OW(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUTA_OW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WINDOW_INV(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WINDOW_INV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WINDOW_CLS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WINDOW_CLS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn EVT_SEL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_EVT_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn FUNC_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FUNC_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PER(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct CCR1 {
                WINDOW_EN: bool,
                SAMPLE_EN: bool,
                DMA_EN: bool,
                COUT_INV: bool,
                COUT_SEL: bool,
                COUT_PEN: bool,
                COUTA_OWEN: bool,
                COUTA_OW: bool,
                WINDOW_INV: bool,
                WINDOW_CLS: bool,
                EVT_SEL: u8,
                FUNC_CLK_SEL: u8,
                FILT_CNT: u8,
                FILT_PER: u8,
            }
            let proxy = CCR1 {
                WINDOW_EN: self.WINDOW_EN(),
                SAMPLE_EN: self.SAMPLE_EN(),
                DMA_EN: self.DMA_EN(),
                COUT_INV: self.COUT_INV(),
                COUT_SEL: self.COUT_SEL(),
                COUT_PEN: self.COUT_PEN(),
                COUTA_OWEN: self.COUTA_OWEN(),
                COUTA_OW: self.COUTA_OW(),
                WINDOW_INV: self.WINDOW_INV(),
                WINDOW_CLS: self.WINDOW_CLS(),
                EVT_SEL: self.EVT_SEL(),
                FUNC_CLK_SEL: self.FUNC_CLK_SEL(),
                FILT_CNT: self.FILT_CNT(),
                FILT_PER: self.FILT_PER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Comparator Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CCR2(pub u32);
    impl CCR2 {
        #[inline(always)]
        pub const fn CMP_HPMD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP_HPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CMP_NPMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CMP_NPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HYSTCTR(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_HYSTCTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PSEL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MSEL(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct CCR2 {
                CMP_HPMD: bool,
                CMP_NPMD: bool,
                HYSTCTR: u8,
                PSEL: u8,
                MSEL: u8,
            }
            let proxy = CCR2 {
                CMP_HPMD: self.CMP_HPMD(),
                CMP_NPMD: self.CMP_NPMD(),
                HYSTCTR: self.HYSTCTR(),
                PSEL: self.PSEL(),
                MSEL: self.MSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Comparator Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[inline(always)]
        pub const fn CFR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CFF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RRF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn COUT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COUT(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct CSR {
                CFR: bool,
                CFF: bool,
                RRF: bool,
                COUT: bool,
            }
            let proxy = CSR {
                CFR: self.CFR(),
                CFF: self.CFF(),
                RRF: self.RRF(),
                COUT: self.COUT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DAC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DCR(pub u32);
    impl DCR {
        #[inline(always)]
        pub const fn DAC_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DAC_HPMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DAC_HPMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn VRSEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VRSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn DAC_DATA(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DAC_DATA(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct DCR {
                DAC_EN: bool,
                DAC_HPMD: bool,
                VRSEL: bool,
                DAC_DATA: u8,
            }
            let proxy = DCR {
                DAC_EN: self.DAC_EN(),
                DAC_HPMD: self.DAC_HPMD(),
                VRSEL: self.VRSEL(),
                DAC_DATA: self.DAC_DATA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[inline(always)]
        pub const fn CFR_IE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CFF_IE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CFF_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RRF_IE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RRF_IE(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct IER {
                CFR_IE: bool,
                CFF_IE: bool,
                RRF_IE: bool,
            }
            let proxy = IER {
                CFR_IE: self.CFR_IE(),
                CFF_IE: self.CFF_IE(),
                RRF_IE: self.RRF_IE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn DAC_RES(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DAC_RES(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct PARAM {
                DAC_RES: u8,
            }
            let proxy = PARAM {
                DAC_RES: self.DAC_RES(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Round Robin Control Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR0(pub u32);
    impl RRCR0 {
        #[inline(always)]
        pub const fn RR_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RR_TRG_SEL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_TRG_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RR_NSAM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RR_NSAM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn RR_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RR_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn RR_INITMOD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RR_INITMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[inline(always)]
        pub const fn RR_SAMPLE_CNT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RR_SAMPLE_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn RR_SAMPLE_THRESHOLD(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RR_SAMPLE_THRESHOLD(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct RRCR0 {
                RR_EN: bool,
                RR_TRG_SEL: bool,
                RR_NSAM: u8,
                RR_CLK_SEL: u8,
                RR_INITMOD: u8,
                RR_SAMPLE_CNT: u8,
                RR_SAMPLE_THRESHOLD: u8,
            }
            let proxy = RRCR0 {
                RR_EN: self.RR_EN(),
                RR_TRG_SEL: self.RR_TRG_SEL(),
                RR_NSAM: self.RR_NSAM(),
                RR_CLK_SEL: self.RR_CLK_SEL(),
                RR_INITMOD: self.RR_INITMOD(),
                RR_SAMPLE_CNT: self.RR_SAMPLE_CNT(),
                RR_SAMPLE_THRESHOLD: self.RR_SAMPLE_THRESHOLD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Round Robin Control Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR1(pub u32);
    impl RRCR1 {
        #[inline(always)]
        pub const fn RR_CH0EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH0EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RR_CH1EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RR_CH2EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH2EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RR_CH3EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH3EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RR_CH4EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH4EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RR_CH5EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH5EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RR_CH6EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH6EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RR_CH7EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH7EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FIXP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIXP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn FIXCH(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIXCH(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct RRCR1 {
                RR_CH0EN: bool,
                RR_CH1EN: bool,
                RR_CH2EN: bool,
                RR_CH3EN: bool,
                RR_CH4EN: bool,
                RR_CH5EN: bool,
                RR_CH6EN: bool,
                RR_CH7EN: bool,
                FIXP: bool,
                FIXCH: u8,
            }
            let proxy = RRCR1 {
                RR_CH0EN: self.RR_CH0EN(),
                RR_CH1EN: self.RR_CH1EN(),
                RR_CH2EN: self.RR_CH2EN(),
                RR_CH3EN: self.RR_CH3EN(),
                RR_CH4EN: self.RR_CH4EN(),
                RR_CH5EN: self.RR_CH5EN(),
                RR_CH6EN: self.RR_CH6EN(),
                RR_CH7EN: self.RR_CH7EN(),
                FIXP: self.FIXP(),
                FIXCH: self.FIXCH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Round Robin Control Register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCR2(pub u32);
    impl RRCR2 {
        #[inline(always)]
        pub const fn RR_TIMER_RELOAD(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RR_TIMER_RELOAD(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn RR_TIMER_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_TIMER_EN(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct RRCR2 {
                RR_TIMER_RELOAD: u32,
                RR_TIMER_EN: bool,
            }
            let proxy = RRCR2 {
                RR_TIMER_RELOAD: self.RR_TIMER_RELOAD(),
                RR_TIMER_EN: self.RR_TIMER_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Round Robin Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRCSR(pub u32);
    impl RRCSR {
        #[inline(always)]
        pub const fn RR_CH0OUT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH0OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RR_CH1OUT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH1OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RR_CH2OUT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH2OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RR_CH3OUT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH3OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RR_CH4OUT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH4OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RR_CH5OUT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH5OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RR_CH6OUT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH6OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RR_CH7OUT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH7OUT(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct RRCSR {
                RR_CH0OUT: bool,
                RR_CH1OUT: bool,
                RR_CH2OUT: bool,
                RR_CH3OUT: bool,
                RR_CH4OUT: bool,
                RR_CH5OUT: bool,
                RR_CH6OUT: bool,
                RR_CH7OUT: bool,
            }
            let proxy = RRCSR {
                RR_CH0OUT: self.RR_CH0OUT(),
                RR_CH1OUT: self.RR_CH1OUT(),
                RR_CH2OUT: self.RR_CH2OUT(),
                RR_CH3OUT: self.RR_CH3OUT(),
                RR_CH4OUT: self.RR_CH4OUT(),
                RR_CH5OUT: self.RR_CH5OUT(),
                RR_CH6OUT: self.RR_CH6OUT(),
                RR_CH7OUT: self.RR_CH7OUT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Round Robin Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RRSR(pub u32);
    impl RRSR {
        #[inline(always)]
        pub const fn RR_CH0F(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH0F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RR_CH1F(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RR_CH2F(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH2F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RR_CH3F(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH3F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RR_CH4F(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH4F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RR_CH5F(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH5F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RR_CH6F(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH6F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RR_CH7F(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RR_CH7F(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct RRSR {
                RR_CH0F: bool,
                RR_CH1F: bool,
                RR_CH2F: bool,
                RR_CH3F: bool,
                RR_CH4F: bool,
                RR_CH5F: bool,
                RR_CH6F: bool,
                RR_CH7F: bool,
            }
            let proxy = RRSR {
                RR_CH0F: self.RR_CH0F(),
                RR_CH1F: self.RR_CH1F(),
                RR_CH2F: self.RR_CH2F(),
                RR_CH3F: self.RR_CH3F(),
                RR_CH4F: self.RR_CH4F(),
                RR_CH5F: self.RR_CH5F(),
                RR_CH6F: self.RR_CH6F(),
                RR_CH7F: self.RR_CH7F(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR(&mut self, val: u8) {
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
            #[derive(defmt :: Format)]
            struct VERID {
                FEATURE: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERID {
                FEATURE: self.FEATURE(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
