#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBDCD {
    ptr: *mut u8,
}
unsafe impl Send for USBDCD {}
unsafe impl Sync for USBDCD {}
impl USBDCD {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CLOCK(self) -> crate::common::Reg<regs::CLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn SIGNAL_OVERRIDE(
        self,
    ) -> crate::common::Reg<regs::SIGNAL_OVERRIDE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER0(self) -> crate::common::Reg<regs::TIMER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER1(self) -> crate::common::Reg<regs::TIMER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER2_BC11(self) -> crate::common::Reg<regs::TIMER2_BC11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER2_BC12(self) -> crate::common::Reg<regs::TIMER2_BC12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLOCK(pub u32);
    impl CLOCK {
        #[must_use]
        #[inline(always)]
        pub const fn CLOCK_UNIT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CLOCK_UNIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLOCK_SPEED(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CLOCK_SPEED(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
        }
    }
    impl Default for CLOCK {
        #[inline(always)]
        fn default() -> CLOCK {
            CLOCK(0)
        }
    }
    impl core::fmt::Debug for CLOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLOCK")
                .field("CLOCK_UNIT", &self.CLOCK_UNIT())
                .field("CLOCK_SPEED", &self.CLOCK_SPEED())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CLOCK {{ CLOCK_UNIT: {=bool:?}, CLOCK_SPEED: {=u16:?} }}",
                self.CLOCK_UNIT(),
                self.CLOCK_SPEED()
            )
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONTROL(pub u32);
    impl CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn IACK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BC12(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BC12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SR(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for CONTROL {
        #[inline(always)]
        fn default() -> CONTROL {
            CONTROL(0)
        }
    }
    impl core::fmt::Debug for CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONTROL")
                .field("IACK", &self.IACK())
                .field("IF", &self.IF())
                .field("IE", &self.IE())
                .field("BC12", &self.BC12())
                .field("START", &self.START())
                .field("SR", &self.SR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CONTROL {{ IACK: {=bool:?}, IF: {=bool:?}, IE: {=bool:?}, BC12: {=bool:?}, START: {=bool:?}, SR: {=bool:?} }}" , self . IACK () , self . IF () , self . IE () , self . BC12 () , self . START () , self . SR ())
        }
    }
    #[doc = "Signal Override"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIGNAL_OVERRIDE(pub u32);
    impl SIGNAL_OVERRIDE {
        #[must_use]
        #[inline(always)]
        pub const fn PS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SIGNAL_OVERRIDE {
        #[inline(always)]
        fn default() -> SIGNAL_OVERRIDE {
            SIGNAL_OVERRIDE(0)
        }
    }
    impl core::fmt::Debug for SIGNAL_OVERRIDE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIGNAL_OVERRIDE")
                .field("PS", &self.PS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIGNAL_OVERRIDE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SIGNAL_OVERRIDE {{ PS: {=u8:?} }}", self.PS())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn SEQ_RES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEQ_RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEQ_STAT(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEQ_STAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERR(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TO(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACTIVE(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for STATUS {
        #[inline(always)]
        fn default() -> STATUS {
            STATUS(0)
        }
    }
    impl core::fmt::Debug for STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS")
                .field("SEQ_RES", &self.SEQ_RES())
                .field("SEQ_STAT", &self.SEQ_STAT())
                .field("ERR", &self.ERR())
                .field("TO", &self.TO())
                .field("ACTIVE", &self.ACTIVE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS {{ SEQ_RES: {=u8:?}, SEQ_STAT: {=u8:?}, ERR: {=bool:?}, TO: {=bool:?}, ACTIVE: {=bool:?} }}" , self . SEQ_RES () , self . SEQ_STAT () , self . ERR () , self . TO () , self . ACTIVE ())
        }
    }
    #[doc = "TIMER0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER0(pub u32);
    impl TIMER0 {
        #[must_use]
        #[inline(always)]
        pub const fn TUNITCON(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TUNITCON(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSEQ_INIT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TSEQ_INIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for TIMER0 {
        #[inline(always)]
        fn default() -> TIMER0 {
            TIMER0(0)
        }
    }
    impl core::fmt::Debug for TIMER0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER0")
                .field("TUNITCON", &self.TUNITCON())
                .field("TSEQ_INIT", &self.TSEQ_INIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TIMER0 {{ TUNITCON: {=u16:?}, TSEQ_INIT: {=u16:?} }}",
                self.TUNITCON(),
                self.TSEQ_INIT()
            )
        }
    }
    #[doc = "TIMER1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER1(pub u32);
    impl TIMER1 {
        #[must_use]
        #[inline(always)]
        pub const fn TVDPSRC_ON(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TVDPSRC_ON(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDCD_DBNC(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TDCD_DBNC(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for TIMER1 {
        #[inline(always)]
        fn default() -> TIMER1 {
            TIMER1(0)
        }
    }
    impl core::fmt::Debug for TIMER1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER1")
                .field("TVDPSRC_ON", &self.TVDPSRC_ON())
                .field("TDCD_DBNC", &self.TDCD_DBNC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TIMER1 {{ TVDPSRC_ON: {=u16:?}, TDCD_DBNC: {=u16:?} }}",
                self.TVDPSRC_ON(),
                self.TDCD_DBNC()
            )
        }
    }
    #[doc = "TIMER2_BC11"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER2_BC11(pub u32);
    impl TIMER2_BC11 {
        #[must_use]
        #[inline(always)]
        pub const fn CHECK_DM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CHECK_DM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TVDPSRC_CON(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TVDPSRC_CON(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for TIMER2_BC11 {
        #[inline(always)]
        fn default() -> TIMER2_BC11 {
            TIMER2_BC11(0)
        }
    }
    impl core::fmt::Debug for TIMER2_BC11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER2_BC11")
                .field("CHECK_DM", &self.CHECK_DM())
                .field("TVDPSRC_CON", &self.TVDPSRC_CON())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER2_BC11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TIMER2_BC11 {{ CHECK_DM: {=u8:?}, TVDPSRC_CON: {=u16:?} }}",
                self.CHECK_DM(),
                self.TVDPSRC_CON()
            )
        }
    }
    #[doc = "TIMER2_BC12"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMER2_BC12(pub u32);
    impl TIMER2_BC12 {
        #[must_use]
        #[inline(always)]
        pub const fn TVDMSRC_ON(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TVDMSRC_ON(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TWAIT_AFTER_PRD(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TWAIT_AFTER_PRD(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for TIMER2_BC12 {
        #[inline(always)]
        fn default() -> TIMER2_BC12 {
            TIMER2_BC12(0)
        }
    }
    impl core::fmt::Debug for TIMER2_BC12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMER2_BC12")
                .field("TVDMSRC_ON", &self.TVDMSRC_ON())
                .field("TWAIT_AFTER_PRD", &self.TWAIT_AFTER_PRD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMER2_BC12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TIMER2_BC12 {{ TVDMSRC_ON: {=u16:?}, TWAIT_AFTER_PRD: {=u16:?} }}",
                self.TVDMSRC_ON(),
                self.TWAIT_AFTER_PRD()
            )
        }
    }
}
