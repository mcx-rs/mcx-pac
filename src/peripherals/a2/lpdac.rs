#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPDAC {
    ptr: *mut u8,
}
unsafe impl Send for LPDAC {}
unsafe impl Sync for LPDAC {}
impl LPDAC {
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
    pub const fn DATA(self) -> crate::common::Reg<regs::DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn GCR(self) -> crate::common::Reg<regs::GCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn FCR(self) -> crate::common::Reg<regs::FCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn FPR(self) -> crate::common::Reg<regs::FPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn FSR(self) -> crate::common::Reg<regs::FSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn DER(self) -> crate::common::Reg<regs::DER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR(self) -> crate::common::Reg<regs::RCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<regs::TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn PCR(self) -> crate::common::Reg<regs::PCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA(pub u32);
    impl DATA {
        #[must_use]
        #[inline(always)]
        pub const fn DATA(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DATA(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for DATA {
        #[inline(always)]
        fn default() -> DATA {
            DATA(0)
        }
    }
    impl core::fmt::Debug for DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATA").field("DATA", &self.DATA()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DATA {{ DATA: {=u16:?} }}", self.DATA())
        }
    }
    #[doc = "DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DER(pub u32);
    impl DER {
        #[must_use]
        #[inline(always)]
        pub const fn EMPTY_DMAEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EMPTY_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WM_DMAEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WM_DMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for DER {
        #[inline(always)]
        fn default() -> DER {
            DER(0)
        }
    }
    impl core::fmt::Debug for DER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DER")
                .field("EMPTY_DMAEN", &self.EMPTY_DMAEN())
                .field("WM_DMAEN", &self.WM_DMAEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DER {{ EMPTY_DMAEN: {=bool:?}, WM_DMAEN: {=bool:?} }}",
                self.EMPTY_DMAEN(),
                self.WM_DMAEN()
            )
        }
    }
    #[doc = "DAC FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCR(pub u32);
    impl FCR {
        #[must_use]
        #[inline(always)]
        pub const fn WML(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WML(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for FCR {
        #[inline(always)]
        fn default() -> FCR {
            FCR(0)
        }
    }
    impl core::fmt::Debug for FCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCR").field("WML", &self.WML()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FCR {{ WML: {=u8:?} }}", self.WML())
        }
    }
    #[doc = "DAC FIFO Pointer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FPR(pub u32);
    impl FPR {
        #[must_use]
        #[inline(always)]
        pub const fn FIFO_RPT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFO_RPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFO_WPT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFO_WPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FPR {
        #[inline(always)]
        fn default() -> FPR {
            FPR(0)
        }
    }
    impl core::fmt::Debug for FPR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FPR")
                .field("FIFO_RPT", &self.FIFO_RPT())
                .field("FIFO_WPT", &self.FIFO_WPT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FPR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FPR {{ FIFO_RPT: {=u8:?}, FIFO_WPT: {=u8:?} }}",
                self.FIFO_RPT(),
                self.FIFO_WPT()
            )
        }
    }
    #[doc = "FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FSR(pub u32);
    impl FSR {
        #[must_use]
        #[inline(always)]
        pub const fn FULL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMPTY(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWBK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWBK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UF(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTGCOCO(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PTGCOCO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for FSR {
        #[inline(always)]
        fn default() -> FSR {
            FSR(0)
        }
    }
    impl core::fmt::Debug for FSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FSR")
                .field("FULL", &self.FULL())
                .field("EMPTY", &self.EMPTY())
                .field("WM", &self.WM())
                .field("SWBK", &self.SWBK())
                .field("OF", &self.OF())
                .field("UF", &self.UF())
                .field("PTGCOCO", &self.PTGCOCO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FSR {{ FULL: {=bool:?}, EMPTY: {=bool:?}, WM: {=bool:?}, SWBK: {=bool:?}, OF: {=bool:?}, UF: {=bool:?}, PTGCOCO: {=bool:?} }}" , self . FULL () , self . EMPTY () , self . WM () , self . SWBK () , self . OF () , self . UF () , self . PTGCOCO ())
        }
    }
    #[doc = "Global Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCR(pub u32);
    impl GCR {
        #[must_use]
        #[inline(always)]
        pub const fn DACEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DACEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DACRFS(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DACRFS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFOEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIFOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWMD(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRGSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRGSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTGEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PTGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LATCH_CYC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LATCH_CYC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BUF_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUF_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IREF_PTAT_EXT_SEL(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IREF_PTAT_EXT_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IREF_ZTC_EXT_SEL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IREF_ZTC_EXT_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BUF_SPD_CTRL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUF_SPD_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for GCR {
        #[inline(always)]
        fn default() -> GCR {
            GCR(0)
        }
    }
    impl core::fmt::Debug for GCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GCR")
                .field("DACEN", &self.DACEN())
                .field("DACRFS", &self.DACRFS())
                .field("FIFOEN", &self.FIFOEN())
                .field("SWMD", &self.SWMD())
                .field("TRGSEL", &self.TRGSEL())
                .field("PTGEN", &self.PTGEN())
                .field("LATCH_CYC", &self.LATCH_CYC())
                .field("BUF_EN", &self.BUF_EN())
                .field("IREF_PTAT_EXT_SEL", &self.IREF_PTAT_EXT_SEL())
                .field("IREF_ZTC_EXT_SEL", &self.IREF_ZTC_EXT_SEL())
                .field("BUF_SPD_CTRL", &self.BUF_SPD_CTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GCR {{ DACEN: {=bool:?}, DACRFS: {=u8:?}, FIFOEN: {=bool:?}, SWMD: {=bool:?}, TRGSEL: {=bool:?}, PTGEN: {=bool:?}, LATCH_CYC: {=u8:?}, BUF_EN: {=bool:?}, IREF_PTAT_EXT_SEL: {=bool:?}, IREF_ZTC_EXT_SEL: {=bool:?}, BUF_SPD_CTRL: {=bool:?} }}" , self . DACEN () , self . DACRFS () , self . FIFOEN () , self . SWMD () , self . TRGSEL () , self . PTGEN () , self . LATCH_CYC () , self . BUF_EN () , self . IREF_PTAT_EXT_SEL () , self . IREF_ZTC_EXT_SEL () , self . BUF_SPD_CTRL ())
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[must_use]
        #[inline(always)]
        pub const fn FULL_IE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FULL_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EMPTY_IE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EMPTY_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WM_IE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WM_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWBK_IE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWBK_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OF_IE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OF_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UF_IE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UF_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTGCOCO_IE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PTGCOCO_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("FULL_IE", &self.FULL_IE())
                .field("EMPTY_IE", &self.EMPTY_IE())
                .field("WM_IE", &self.WM_IE())
                .field("SWBK_IE", &self.SWBK_IE())
                .field("OF_IE", &self.OF_IE())
                .field("UF_IE", &self.UF_IE())
                .field("PTGCOCO_IE", &self.PTGCOCO_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IER {{ FULL_IE: {=bool:?}, EMPTY_IE: {=bool:?}, WM_IE: {=bool:?}, SWBK_IE: {=bool:?}, OF_IE: {=bool:?}, UF_IE: {=bool:?}, PTGCOCO_IE: {=bool:?} }}" , self . FULL_IE () , self . EMPTY_IE () , self . WM_IE () , self . SWBK_IE () , self . OF_IE () , self . UF_IE () , self . PTGCOCO_IE ())
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn FIFOSZ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFOSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
                .field("FIFOSZ", &self.FIFOSZ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PARAM {{ FIFOSZ: {=u8:?} }}", self.FIFOSZ())
        }
    }
    #[doc = "Periodic Trigger Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCR(pub u32);
    impl PCR {
        #[must_use]
        #[inline(always)]
        pub const fn PTG_NUM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PTG_NUM(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTG_PERIOD(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PTG_PERIOD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PCR {
        #[inline(always)]
        fn default() -> PCR {
            PCR(0)
        }
    }
    impl core::fmt::Debug for PCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCR")
                .field("PTG_NUM", &self.PTG_NUM())
                .field("PTG_PERIOD", &self.PTG_PERIOD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PCR {{ PTG_NUM: {=u16:?}, PTG_PERIOD: {=u16:?} }}",
                self.PTG_NUM(),
                self.PTG_PERIOD()
            )
        }
    }
    #[doc = "Reset Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCR(pub u32);
    impl RCR {
        #[must_use]
        #[inline(always)]
        pub const fn SWRST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFORST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FIFORST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for RCR {
        #[inline(always)]
        fn default() -> RCR {
            RCR(0)
        }
    }
    impl core::fmt::Debug for RCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCR")
                .field("SWRST", &self.SWRST())
                .field("FIFORST", &self.FIFORST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RCR {{ SWRST: {=bool:?}, FIFORST: {=bool:?} }}",
                self.SWRST(),
                self.FIFORST()
            )
        }
    }
    #[doc = "Trigger Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCR(pub u32);
    impl TCR {
        #[must_use]
        #[inline(always)]
        pub const fn SWTRG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWTRG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TCR {
        #[inline(always)]
        fn default() -> TCR {
            TCR(0)
        }
    }
    impl core::fmt::Debug for TCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCR").field("SWTRG", &self.SWTRG()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TCR {{ SWTRG: {=bool:?} }}", self.SWTRG())
        }
    }
    #[doc = "Version Identifier"]
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
