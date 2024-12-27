#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
        #[inline(always)]
        pub const fn IRQ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IRQ(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for INTM_IACK {
        #[inline(always)]
        fn default() -> INTM_IACK {
            INTM_IACK(0)
        }
    }
    #[doc = "Monitor Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTM_MM(pub u32);
    impl INTM_MM {
        #[inline(always)]
        pub const fn MM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for INTM_MM {
        #[inline(always)]
        fn default() -> INTM_MM {
            INTM_MM(0)
        }
    }
    #[doc = "Interrupt Request Select for Monitor 0..Interrupt Request Select for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_IRQSEL(pub u32);
    impl MON_INTM_IRQSEL {
        #[inline(always)]
        pub const fn IRQ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IRQ(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for MON_INTM_IRQSEL {
        #[inline(always)]
        fn default() -> MON_INTM_IRQSEL {
            MON_INTM_IRQSEL(0)
        }
    }
    #[doc = "Interrupt Latency for Monitor 0..Interrupt Latency for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_LATENCY(pub u32);
    impl MON_INTM_LATENCY {
        #[inline(always)]
        pub const fn LAT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_LAT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MON_INTM_LATENCY {
        #[inline(always)]
        fn default() -> MON_INTM_LATENCY {
            MON_INTM_LATENCY(0)
        }
    }
    #[doc = "Status for Monitor 0..Status for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_STATUS(pub u32);
    impl MON_INTM_STATUS {
        #[inline(always)]
        pub const fn STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MON_INTM_STATUS {
        #[inline(always)]
        fn default() -> MON_INTM_STATUS {
            MON_INTM_STATUS(0)
        }
    }
    #[doc = "Timer for Monitor 0..Timer for Monitor 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MON_INTM_TIMER(pub u32);
    impl MON_INTM_TIMER {
        #[inline(always)]
        pub const fn TIMER(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TIMER(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MON_INTM_TIMER {
        #[inline(always)]
        fn default() -> MON_INTM_TIMER {
            MON_INTM_TIMER(0)
        }
    }
}
