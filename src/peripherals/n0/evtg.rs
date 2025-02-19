#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTG {
    ptr: *mut u8,
}
unsafe impl Send for EVTG {}
unsafe impl Sync for EVTG {}
impl EVTG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn EVTG_INST(self, n: usize) -> EVTG_INST {
        assert!(n < 4usize);
        unsafe { EVTG_INST::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTG_INST {
    ptr: *mut u8,
}
unsafe impl Send for EVTG_INST {}
unsafe impl Sync for EVTG_INST {}
impl EVTG_INST {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn EVTG_AOI0_BFT01(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI0_BFT01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_AOI0_BFT23(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI0_BFT23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_AOI1_BFT01(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI1_BFT01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_AOI1_BFT23(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI1_BFT23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_CTRL(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_AOI0_FILT(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI0_FILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn EVTG_AOI1_FILT(
        self,
    ) -> crate::common::Reg<regs::EVTG_INST_EVTG_AOI1_FILT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "AOI0 Boolean Function Term 0 and 1 Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI0_BFT01(pub u16);
    impl EVTG_INST_EVTG_AOI0_BFT01 {
        #[inline(always)]
        pub const fn PT1_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PT1_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PT1_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PT1_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn PT0_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn PT0_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PT0_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn PT0_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI0_BFT01 {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI0_BFT01 {
            EVTG_INST_EVTG_AOI0_BFT01(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI0_BFT01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI0_BFT01")
                .field("PT1_DC", &self.PT1_DC())
                .field("PT1_CC", &self.PT1_CC())
                .field("PT1_BC", &self.PT1_BC())
                .field("PT1_AC", &self.PT1_AC())
                .field("PT0_DC", &self.PT0_DC())
                .field("PT0_CC", &self.PT0_CC())
                .field("PT0_BC", &self.PT0_BC())
                .field("PT0_AC", &self.PT0_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI0_BFT01 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EVTG_INST_EVTG_AOI0_BFT01 {{ PT1_DC: {=u8:?}, PT1_CC: {=u8:?}, PT1_BC: {=u8:?}, PT1_AC: {=u8:?}, PT0_DC: {=u8:?}, PT0_CC: {=u8:?}, PT0_BC: {=u8:?}, PT0_AC: {=u8:?} }}" , self . PT1_DC () , self . PT1_CC () , self . PT1_BC () , self . PT1_AC () , self . PT0_DC () , self . PT0_CC () , self . PT0_BC () , self . PT0_AC ())
        }
    }
    #[doc = "AOI0 Boolean Function Term 2 and 3 Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI0_BFT23(pub u16);
    impl EVTG_INST_EVTG_AOI0_BFT23 {
        #[inline(always)]
        pub const fn PT3_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PT3_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PT3_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PT3_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn PT2_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn PT2_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PT2_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn PT2_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI0_BFT23 {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI0_BFT23 {
            EVTG_INST_EVTG_AOI0_BFT23(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI0_BFT23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI0_BFT23")
                .field("PT3_DC", &self.PT3_DC())
                .field("PT3_CC", &self.PT3_CC())
                .field("PT3_BC", &self.PT3_BC())
                .field("PT3_AC", &self.PT3_AC())
                .field("PT2_DC", &self.PT2_DC())
                .field("PT2_CC", &self.PT2_CC())
                .field("PT2_BC", &self.PT2_BC())
                .field("PT2_AC", &self.PT2_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI0_BFT23 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EVTG_INST_EVTG_AOI0_BFT23 {{ PT3_DC: {=u8:?}, PT3_CC: {=u8:?}, PT3_BC: {=u8:?}, PT3_AC: {=u8:?}, PT2_DC: {=u8:?}, PT2_CC: {=u8:?}, PT2_BC: {=u8:?}, PT2_AC: {=u8:?} }}" , self . PT3_DC () , self . PT3_CC () , self . PT3_BC () , self . PT3_AC () , self . PT2_DC () , self . PT2_CC () , self . PT2_BC () , self . PT2_AC ())
        }
    }
    #[doc = "AOI0 Output Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI0_FILT(pub u16);
    impl EVTG_INST_EVTG_AOI0_FILT {
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI0_FILT {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI0_FILT {
            EVTG_INST_EVTG_AOI0_FILT(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI0_FILT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI0_FILT")
                .field("FILT_PER", &self.FILT_PER())
                .field("FILT_CNT", &self.FILT_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI0_FILT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EVTG_INST_EVTG_AOI0_FILT {{ FILT_PER: {=u8:?}, FILT_CNT: {=u8:?} }}",
                self.FILT_PER(),
                self.FILT_CNT()
            )
        }
    }
    #[doc = "AOI1 Boolean Function Term 0 and 1 Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI1_BFT01(pub u16);
    impl EVTG_INST_EVTG_AOI1_BFT01 {
        #[inline(always)]
        pub const fn PT1_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PT1_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PT1_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PT1_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT1_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn PT0_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn PT0_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PT0_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn PT0_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT0_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI1_BFT01 {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI1_BFT01 {
            EVTG_INST_EVTG_AOI1_BFT01(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI1_BFT01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI1_BFT01")
                .field("PT1_DC", &self.PT1_DC())
                .field("PT1_CC", &self.PT1_CC())
                .field("PT1_BC", &self.PT1_BC())
                .field("PT1_AC", &self.PT1_AC())
                .field("PT0_DC", &self.PT0_DC())
                .field("PT0_CC", &self.PT0_CC())
                .field("PT0_BC", &self.PT0_BC())
                .field("PT0_AC", &self.PT0_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI1_BFT01 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EVTG_INST_EVTG_AOI1_BFT01 {{ PT1_DC: {=u8:?}, PT1_CC: {=u8:?}, PT1_BC: {=u8:?}, PT1_AC: {=u8:?}, PT0_DC: {=u8:?}, PT0_CC: {=u8:?}, PT0_BC: {=u8:?}, PT0_AC: {=u8:?} }}" , self . PT1_DC () , self . PT1_CC () , self . PT1_BC () , self . PT1_AC () , self . PT0_DC () , self . PT0_CC () , self . PT0_BC () , self . PT0_AC ())
        }
    }
    #[doc = "AOI1 Boolean Function Term 2 and 3 Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI1_BFT23(pub u16);
    impl EVTG_INST_EVTG_AOI1_BFT23 {
        #[inline(always)]
        pub const fn PT3_DC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PT3_CC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PT3_BC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn PT3_AC(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT3_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn PT2_DC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_DC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn PT2_CC(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_CC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn PT2_BC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_BC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn PT2_AC(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PT2_AC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI1_BFT23 {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI1_BFT23 {
            EVTG_INST_EVTG_AOI1_BFT23(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI1_BFT23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI1_BFT23")
                .field("PT3_DC", &self.PT3_DC())
                .field("PT3_CC", &self.PT3_CC())
                .field("PT3_BC", &self.PT3_BC())
                .field("PT3_AC", &self.PT3_AC())
                .field("PT2_DC", &self.PT2_DC())
                .field("PT2_CC", &self.PT2_CC())
                .field("PT2_BC", &self.PT2_BC())
                .field("PT2_AC", &self.PT2_AC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI1_BFT23 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EVTG_INST_EVTG_AOI1_BFT23 {{ PT3_DC: {=u8:?}, PT3_CC: {=u8:?}, PT3_BC: {=u8:?}, PT3_AC: {=u8:?}, PT2_DC: {=u8:?}, PT2_CC: {=u8:?}, PT2_BC: {=u8:?}, PT2_AC: {=u8:?} }}" , self . PT3_DC () , self . PT3_CC () , self . PT3_BC () , self . PT3_AC () , self . PT2_DC () , self . PT2_CC () , self . PT2_BC () , self . PT2_AC ())
        }
    }
    #[doc = "AOI1 Output Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_AOI1_FILT(pub u16);
    impl EVTG_INST_EVTG_AOI1_FILT {
        #[inline(always)]
        pub const fn FILT_PER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_PER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILT_CNT(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILT_CNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for EVTG_INST_EVTG_AOI1_FILT {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_AOI1_FILT {
            EVTG_INST_EVTG_AOI1_FILT(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_AOI1_FILT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_AOI1_FILT")
                .field("FILT_PER", &self.FILT_PER())
                .field("FILT_CNT", &self.FILT_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_AOI1_FILT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EVTG_INST_EVTG_AOI1_FILT {{ FILT_PER: {=u8:?}, FILT_CNT: {=u8:?} }}",
                self.FILT_PER(),
                self.FILT_CNT()
            )
        }
    }
    #[doc = "Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTG_INST_EVTG_CTRL(pub u16);
    impl EVTG_INST_EVTG_CTRL {
        #[inline(always)]
        pub const fn FF_INIT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FF_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INIT_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INIT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MODE_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MODE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u16) & 0x07) << 2usize);
        }
        #[inline(always)]
        pub const fn FB_OVRD(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FB_OVRD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn SYNC_CTRL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNC_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn FORCE_BYPASS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FORCE_BYPASS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
    }
    impl Default for EVTG_INST_EVTG_CTRL {
        #[inline(always)]
        fn default() -> EVTG_INST_EVTG_CTRL {
            EVTG_INST_EVTG_CTRL(0)
        }
    }
    impl core::fmt::Debug for EVTG_INST_EVTG_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTG_INST_EVTG_CTRL")
                .field("FF_INIT", &self.FF_INIT())
                .field("INIT_EN", &self.INIT_EN())
                .field("MODE_SEL", &self.MODE_SEL())
                .field("FB_OVRD", &self.FB_OVRD())
                .field("SYNC_CTRL", &self.SYNC_CTRL())
                .field("FORCE_BYPASS", &self.FORCE_BYPASS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTG_INST_EVTG_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EVTG_INST_EVTG_CTRL {{ FF_INIT: {=bool:?}, INIT_EN: {=bool:?}, MODE_SEL: {=u8:?}, FB_OVRD: {=u8:?}, SYNC_CTRL: {=u8:?}, FORCE_BYPASS: {=u8:?} }}" , self . FF_INIT () , self . INIT_EN () , self . MODE_SEL () , self . FB_OVRD () , self . SYNC_CTRL () , self . FORCE_BYPASS ())
        }
    }
}
