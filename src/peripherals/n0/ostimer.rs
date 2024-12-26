#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn EVTIMERH(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTURE_L(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CAPTURE_H(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH_L(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH_H(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn OSEVENT_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Local Capture High for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPTURE_H(pub u32);
    impl CAPTURE_H {
        #[inline(always)]
        pub const fn CAPTURE_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPTURE_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for CAPTURE_H {
        #[inline(always)]
        fn default() -> CAPTURE_H {
            CAPTURE_H(0)
        }
    }
    #[doc = "Local Capture Low for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPTURE_L(pub u32);
    impl CAPTURE_L {
        #[inline(always)]
        pub const fn CAPTURE_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CAPTURE_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CAPTURE_L {
        #[inline(always)]
        fn default() -> CAPTURE_L {
            CAPTURE_L(0)
        }
    }
    #[doc = "EVTIMER High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTIMERH(pub u32);
    impl EVTIMERH {
        #[inline(always)]
        pub const fn EVTIMER_COUNT_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_EVTIMER_COUNT_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for EVTIMERH {
        #[inline(always)]
        fn default() -> EVTIMERH {
            EVTIMERH(0)
        }
    }
    #[doc = "EVTIMER Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVTIMERL(pub u32);
    impl EVTIMERL {
        #[inline(always)]
        pub const fn EVTIMER_COUNT_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_EVTIMER_COUNT_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EVTIMERL {
        #[inline(always)]
        fn default() -> EVTIMERL {
            EVTIMERL(0)
        }
    }
    #[doc = "Local Match High for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH_H(pub u32);
    impl MATCH_H {
        #[inline(always)]
        pub const fn MATCH_VALUE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCH_VALUE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for MATCH_H {
        #[inline(always)]
        fn default() -> MATCH_H {
            MATCH_H(0)
        }
    }
    #[doc = "Local Match Low for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH_L(pub u32);
    impl MATCH_L {
        #[inline(always)]
        pub const fn MATCH_VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MATCH_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MATCH_L {
        #[inline(always)]
        fn default() -> MATCH_L {
            MATCH_L(0)
        }
    }
    #[doc = "OSTIMER Control for CPU"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSEVENT_CTRL(pub u32);
    impl OSEVENT_CTRL {
        #[inline(always)]
        pub const fn OSTIMER_INTRFLAG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER_INTRFLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSTIMER_INTENA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSTIMER_INTENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MATCH_WR_RDY(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MATCH_WR_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for OSEVENT_CTRL {
        #[inline(always)]
        fn default() -> OSEVENT_CTRL {
            OSEVENT_CTRL(0)
        }
    }
}
