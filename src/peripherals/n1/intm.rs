#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTM {
    ptr: *mut u8,
}
unsafe impl Send for INTM {}
unsafe impl Sync for INTM {}
impl INTM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn INTM_MM(self) -> crate::common::Reg<regs::INTM_MM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn INTM_IACK(self) -> crate::common::Reg<regs::INTM_IACK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MON(self, n: usize) -> MON {
        assert!(n < 4usize);
        unsafe { MON::from_ptr(self.ptr.add(0x08usize + n * 16usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MON {
    ptr: *mut u8,
}
unsafe impl Send for MON {}
unsafe impl Sync for MON {}
impl MON {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn INTM_IRQSEL(self) -> crate::common::Reg<regs::MON_INTM_IRQSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn INTM_LATENCY(
        self,
    ) -> crate::common::Reg<regs::MON_INTM_LATENCY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn INTM_TIMER(self) -> crate::common::Reg<regs::MON_INTM_TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn INTM_STATUS(self) -> crate::common::Reg<regs::MON_INTM_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt Acknowledge"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTM_IACK(pub u32);
    impl INTM_IACK {
        #[must_use]
        #[inline(always)]
        pub const fn IRQ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_IRQ(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for INTM_IACK {
        #[inline(always)]
        fn default() -> INTM_IACK {
            INTM_IACK(0)
        }
    }
    impl core::fmt::Debug for INTM_IACK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTM_IACK")
                .field("IRQ", &self.IRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTM_IACK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "INTM_IACK {{ IRQ: {=u16:?} }}", self.IRQ())
        }
    }
    #[doc = "Monitor Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTM_MM(pub u32);
    impl INTM_MM {
        #[must_use]
        #[inline(always)]
        pub const fn MM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for INTM_MM {
        #[inline(always)]
        fn default() -> INTM_MM {
            INTM_MM(0)
        }
    }
    impl core::fmt::Debug for INTM_MM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTM_MM").field("MM", &self.MM()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTM_MM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "INTM_MM {{ MM: {=bool:?} }}", self.MM())
        }
    }
    #[doc = "Interrupt Request Select for Monitor 0..Interrupt Request Select for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_IRQSEL(pub u32);
    impl MON_INTM_IRQSEL {
        #[must_use]
        #[inline(always)]
        pub const fn IRQ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_IRQ(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for MON_INTM_IRQSEL {
        #[inline(always)]
        fn default() -> MON_INTM_IRQSEL {
            MON_INTM_IRQSEL(0)
        }
    }
    impl core::fmt::Debug for MON_INTM_IRQSEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MON_INTM_IRQSEL")
                .field("IRQ", &self.IRQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MON_INTM_IRQSEL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MON_INTM_IRQSEL {{ IRQ: {=u16:?} }}", self.IRQ())
        }
    }
    #[doc = "Interrupt Latency for Monitor 0..Interrupt Latency for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_LATENCY(pub u32);
    impl MON_INTM_LATENCY {
        #[must_use]
        #[inline(always)]
        pub const fn LAT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_LAT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MON_INTM_LATENCY {
        #[inline(always)]
        fn default() -> MON_INTM_LATENCY {
            MON_INTM_LATENCY(0)
        }
    }
    impl core::fmt::Debug for MON_INTM_LATENCY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MON_INTM_LATENCY")
                .field("LAT", &self.LAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MON_INTM_LATENCY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MON_INTM_LATENCY {{ LAT: {=u32:?} }}", self.LAT())
        }
    }
    #[doc = "Status for Monitor 0..Status for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_STATUS(pub u32);
    impl MON_INTM_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MON_INTM_STATUS {
        #[inline(always)]
        fn default() -> MON_INTM_STATUS {
            MON_INTM_STATUS(0)
        }
    }
    impl core::fmt::Debug for MON_INTM_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MON_INTM_STATUS")
                .field("STATUS", &self.STATUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MON_INTM_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MON_INTM_STATUS {{ STATUS: {=bool:?} }}", self.STATUS())
        }
    }
    #[doc = "Timer for Monitor 0..Timer for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_TIMER(pub u32);
    impl MON_INTM_TIMER {
        #[must_use]
        #[inline(always)]
        pub const fn TIMER(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TIMER(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MON_INTM_TIMER {
        #[inline(always)]
        fn default() -> MON_INTM_TIMER {
            MON_INTM_TIMER(0)
        }
    }
    impl core::fmt::Debug for MON_INTM_TIMER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MON_INTM_TIMER")
                .field("TIMER", &self.TIMER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MON_INTM_TIMER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MON_INTM_TIMER {{ TIMER: {=u32:?} }}", self.TIMER())
        }
    }
}
