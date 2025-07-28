#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PINT {
    ptr: *mut u8,
}
unsafe impl Send for PINT {}
unsafe impl Sync for PINT {}
impl PINT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ISEL(self) -> crate::common::Reg<regs::ISEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn IENR(self) -> crate::common::Reg<regs::IENR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SIENR(self) -> crate::common::Reg<regs::SIENR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CIENR(self) -> crate::common::Reg<regs::CIENR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn IENF(self) -> crate::common::Reg<regs::IENF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SIENF(self) -> crate::common::Reg<regs::SIENF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CIENF(self) -> crate::common::Reg<regs::CIENF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn RISE(self) -> crate::common::Reg<regs::RISE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn FALL(self) -> crate::common::Reg<regs::FALL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn IST(self) -> crate::common::Reg<regs::IST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn PMCTRL(self) -> crate::common::Reg<regs::PMCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn PMSRC(self) -> crate::common::Reg<regs::PMSRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn PMCFG(self) -> crate::common::Reg<regs::PMCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CIENF(pub u32);
    impl CIENF {
        #[must_use]
        #[inline(always)]
        pub const fn CENAF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CENAF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CIENF {
        #[inline(always)]
        fn default() -> CIENF {
            CIENF(0)
        }
    }
    impl core::fmt::Debug for CIENF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CIENF")
                .field("CENAF", &self.CENAF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CIENF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CIENF {{ CENAF: {=u8:?} }}", self.CENAF())
        }
    }
    #[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CIENR(pub u32);
    impl CIENR {
        #[must_use]
        #[inline(always)]
        pub const fn CENRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CENRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CIENR {
        #[inline(always)]
        fn default() -> CIENR {
            CIENR(0)
        }
    }
    impl core::fmt::Debug for CIENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CIENR")
                .field("CENRL", &self.CENRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CIENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CIENR {{ CENRL: {=u8:?} }}", self.CENRL())
        }
    }
    #[doc = "Pin Interrupt Falling Edge"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FALL(pub u32);
    impl FALL {
        #[must_use]
        #[inline(always)]
        pub const fn FDET(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FDET(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for FALL {
        #[inline(always)]
        fn default() -> FALL {
            FALL(0)
        }
    }
    impl core::fmt::Debug for FALL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FALL").field("FDET", &self.FDET()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FALL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FALL {{ FDET: {=u8:?} }}", self.FDET())
        }
    }
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IENF(pub u32);
    impl IENF {
        #[must_use]
        #[inline(always)]
        pub const fn ENAF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENAF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IENF {
        #[inline(always)]
        fn default() -> IENF {
            IENF(0)
        }
    }
    impl core::fmt::Debug for IENF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IENF").field("ENAF", &self.ENAF()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IENF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IENF {{ ENAF: {=u8:?} }}", self.ENAF())
        }
    }
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IENR(pub u32);
    impl IENR {
        #[must_use]
        #[inline(always)]
        pub const fn ENRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IENR {
        #[inline(always)]
        fn default() -> IENR {
            IENR(0)
        }
    }
    impl core::fmt::Debug for IENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IENR").field("ENRL", &self.ENRL()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IENR {{ ENRL: {=u8:?} }}", self.ENRL())
        }
    }
    #[doc = "Pin Interrupt Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISEL(pub u32);
    impl ISEL {
        #[must_use]
        #[inline(always)]
        pub const fn PMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ISEL {
        #[inline(always)]
        fn default() -> ISEL {
            ISEL(0)
        }
    }
    impl core::fmt::Debug for ISEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISEL")
                .field("PMODE", &self.PMODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISEL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ISEL {{ PMODE: {=u8:?} }}", self.PMODE())
        }
    }
    #[doc = "Pin Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IST(pub u32);
    impl IST {
        #[must_use]
        #[inline(always)]
        pub const fn PSTAT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PSTAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IST {
        #[inline(always)]
        fn default() -> IST {
            IST(0)
        }
    }
    impl core::fmt::Debug for IST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IST").field("PSTAT", &self.PSTAT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IST {{ PSTAT: {=u8:?} }}", self.PSTAT())
        }
    }
    #[doc = "Pattern-Match Interrupt Bit Slice Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMCFG(pub u32);
    impl PMCFG {
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PROD_ENDPTS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PROD_ENDPTS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG2(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG3(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG4(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG5(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG6(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CFG7(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CFG7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for PMCFG {
        #[inline(always)]
        fn default() -> PMCFG {
            PMCFG(0)
        }
    }
    impl core::fmt::Debug for PMCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMCFG")
                .field("PROD_ENDPTS0", &self.PROD_ENDPTS0())
                .field("PROD_ENDPTS1", &self.PROD_ENDPTS1())
                .field("PROD_ENDPTS2", &self.PROD_ENDPTS2())
                .field("PROD_ENDPTS3", &self.PROD_ENDPTS3())
                .field("PROD_ENDPTS4", &self.PROD_ENDPTS4())
                .field("PROD_ENDPTS5", &self.PROD_ENDPTS5())
                .field("PROD_ENDPTS6", &self.PROD_ENDPTS6())
                .field("CFG0", &self.CFG0())
                .field("CFG1", &self.CFG1())
                .field("CFG2", &self.CFG2())
                .field("CFG3", &self.CFG3())
                .field("CFG4", &self.CFG4())
                .field("CFG5", &self.CFG5())
                .field("CFG6", &self.CFG6())
                .field("CFG7", &self.CFG7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PMCFG {{ PROD_ENDPTS0: {=bool:?}, PROD_ENDPTS1: {=bool:?}, PROD_ENDPTS2: {=bool:?}, PROD_ENDPTS3: {=bool:?}, PROD_ENDPTS4: {=bool:?}, PROD_ENDPTS5: {=bool:?}, PROD_ENDPTS6: {=bool:?}, CFG0: {=u8:?}, CFG1: {=u8:?}, CFG2: {=u8:?}, CFG3: {=u8:?}, CFG4: {=u8:?}, CFG5: {=u8:?}, CFG6: {=u8:?}, CFG7: {=u8:?} }}" , self . PROD_ENDPTS0 () , self . PROD_ENDPTS1 () , self . PROD_ENDPTS2 () , self . PROD_ENDPTS3 () , self . PROD_ENDPTS4 () , self . PROD_ENDPTS5 () , self . PROD_ENDPTS6 () , self . CFG0 () , self . CFG1 () , self . CFG2 () , self . CFG3 () , self . CFG4 () , self . CFG5 () , self . CFG6 () , self . CFG7 ())
        }
    }
    #[doc = "Pattern-Match Interrupt Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMCTRL(pub u32);
    impl PMCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn SEL_PMATCH(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SEL_PMATCH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENA_RXEV(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ENA_RXEV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PMAT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PMAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PMCTRL {
        #[inline(always)]
        fn default() -> PMCTRL {
            PMCTRL(0)
        }
    }
    impl core::fmt::Debug for PMCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMCTRL")
                .field("SEL_PMATCH", &self.SEL_PMATCH())
                .field("ENA_RXEV", &self.ENA_RXEV())
                .field("PMAT", &self.PMAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PMCTRL {{ SEL_PMATCH: {=bool:?}, ENA_RXEV: {=bool:?}, PMAT: {=u8:?} }}",
                self.SEL_PMATCH(),
                self.ENA_RXEV(),
                self.PMAT()
            )
        }
    }
    #[doc = "Pattern-Match Interrupt Bit-Slice Source"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMSRC(pub u32);
    impl PMSRC {
        #[must_use]
        #[inline(always)]
        pub const fn SRC0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC1(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC2(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC3(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC4(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC5(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC6(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRC7(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRC7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for PMSRC {
        #[inline(always)]
        fn default() -> PMSRC {
            PMSRC(0)
        }
    }
    impl core::fmt::Debug for PMSRC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMSRC")
                .field("SRC0", &self.SRC0())
                .field("SRC1", &self.SRC1())
                .field("SRC2", &self.SRC2())
                .field("SRC3", &self.SRC3())
                .field("SRC4", &self.SRC4())
                .field("SRC5", &self.SRC5())
                .field("SRC6", &self.SRC6())
                .field("SRC7", &self.SRC7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMSRC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PMSRC {{ SRC0: {=u8:?}, SRC1: {=u8:?}, SRC2: {=u8:?}, SRC3: {=u8:?}, SRC4: {=u8:?}, SRC5: {=u8:?}, SRC6: {=u8:?}, SRC7: {=u8:?} }}" , self . SRC0 () , self . SRC1 () , self . SRC2 () , self . SRC3 () , self . SRC4 () , self . SRC5 () , self . SRC6 () , self . SRC7 ())
        }
    }
    #[doc = "Pin Interrupt Rising Edge"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RISE(pub u32);
    impl RISE {
        #[must_use]
        #[inline(always)]
        pub const fn RDET(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RDET(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for RISE {
        #[inline(always)]
        fn default() -> RISE {
            RISE(0)
        }
    }
    impl core::fmt::Debug for RISE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RISE").field("RDET", &self.RDET()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RISE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RISE {{ RDET: {=u8:?} }}", self.RDET())
        }
    }
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIENF(pub u32);
    impl SIENF {
        #[must_use]
        #[inline(always)]
        pub const fn SETENAF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SETENAF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SIENF {
        #[inline(always)]
        fn default() -> SIENF {
            SIENF(0)
        }
    }
    impl core::fmt::Debug for SIENF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIENF")
                .field("SETENAF", &self.SETENAF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIENF {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SIENF {{ SETENAF: {=u8:?} }}", self.SETENAF())
        }
    }
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIENR(pub u32);
    impl SIENR {
        #[must_use]
        #[inline(always)]
        pub const fn SETENRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SETENRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SIENR {
        #[inline(always)]
        fn default() -> SIENR {
            SIENR(0)
        }
    }
    impl core::fmt::Debug for SIENR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIENR")
                .field("SETENRL", &self.SETENRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIENR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SIENR {{ SETENRL: {=u8:?} }}", self.SETENRL())
        }
    }
}
