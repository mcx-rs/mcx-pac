#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSTIMER {
    ptr: *mut u8,
}
unsafe impl Send for OSTIMER {}
unsafe impl Sync for OSTIMER {}
impl OSTIMER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn EVTIMERL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn EVTIMERH(self) -> crate::common::Reg<regs::EVTIMERH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTURE_L(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTURE_H(self) -> crate::common::Reg<regs::CAPTURE_H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH_L(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH_H(self) -> crate::common::Reg<regs::MATCH_H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn OSEVENT_CTRL(self) -> crate::common::Reg<regs::OSEVENT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Local Capture High for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPTURE_H(pub u32);
    impl CAPTURE_H {
        #[must_use]
        #[inline(always)]
        pub const fn CAPTURE_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CAPTURE_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for CAPTURE_H {
        #[inline(always)]
        fn default() -> CAPTURE_H {
            CAPTURE_H(0)
        }
    }
    impl core::fmt::Debug for CAPTURE_H {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAPTURE_H")
                .field("CAPTURE_VALUE", &self.CAPTURE_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAPTURE_H {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CAPTURE_H {{ CAPTURE_VALUE: {=u16:?} }}",
                self.CAPTURE_VALUE()
            )
        }
    }
    #[doc = "EVTIMER High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTIMERH(pub u32);
    impl EVTIMERH {
        #[must_use]
        #[inline(always)]
        pub const fn EVTIMER_COUNT_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_EVTIMER_COUNT_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for EVTIMERH {
        #[inline(always)]
        fn default() -> EVTIMERH {
            EVTIMERH(0)
        }
    }
    impl core::fmt::Debug for EVTIMERH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVTIMERH")
                .field("EVTIMER_COUNT_VALUE", &self.EVTIMER_COUNT_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVTIMERH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EVTIMERH {{ EVTIMER_COUNT_VALUE: {=u16:?} }}",
                self.EVTIMER_COUNT_VALUE()
            )
        }
    }
    #[doc = "Local Match High for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH_H(pub u32);
    impl MATCH_H {
        #[must_use]
        #[inline(always)]
        pub const fn MATCH_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MATCH_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for MATCH_H {
        #[inline(always)]
        fn default() -> MATCH_H {
            MATCH_H(0)
        }
    }
    impl core::fmt::Debug for MATCH_H {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCH_H")
                .field("MATCH_VALUE", &self.MATCH_VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCH_H {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MATCH_H {{ MATCH_VALUE: {=u16:?} }}", self.MATCH_VALUE())
        }
    }
    #[doc = "OSTIMER Control for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSEVENT_CTRL(pub u32);
    impl OSEVENT_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn OSTIMER_INTRFLAG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OSTIMER_INTRFLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSTIMER_INTENA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OSTIMER_INTENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MATCH_WR_RDY(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MATCH_WR_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBUG_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DEBUG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for OSEVENT_CTRL {
        #[inline(always)]
        fn default() -> OSEVENT_CTRL {
            OSEVENT_CTRL(0)
        }
    }
    impl core::fmt::Debug for OSEVENT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSEVENT_CTRL")
                .field("OSTIMER_INTRFLAG", &self.OSTIMER_INTRFLAG())
                .field("OSTIMER_INTENA", &self.OSTIMER_INTENA())
                .field("MATCH_WR_RDY", &self.MATCH_WR_RDY())
                .field("DEBUG_EN", &self.DEBUG_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSEVENT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OSEVENT_CTRL {{ OSTIMER_INTRFLAG: {=bool:?}, OSTIMER_INTENA: {=bool:?}, MATCH_WR_RDY: {=bool:?}, DEBUG_EN: {=bool:?} }}" , self . OSTIMER_INTRFLAG () , self . OSTIMER_INTENA () , self . MATCH_WR_RDY () , self . DEBUG_EN ())
        }
    }
}
