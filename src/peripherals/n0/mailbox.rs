#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAILBOX {
    ptr: *mut u8,
}
unsafe impl Send for MAILBOX {}
unsafe impl Sync for MAILBOX {}
impl MAILBOX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MBOXIRQ(self, n: usize) -> MBOXIRQ {
        assert!(n < 2usize);
        unsafe { MBOXIRQ::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn MUTEX(self) -> crate::common::Reg<regs::MUTEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MBOXIRQ {
    ptr: *mut u8,
}
unsafe impl Send for MBOXIRQ {}
unsafe impl Sync for MBOXIRQ {}
impl MBOXIRQ {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn IRQ(self) -> crate::common::Reg<regs::MBOXIRQ_IRQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<regs::MBOXIRQ_IRQSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQCLR(self) -> crate::common::Reg<regs::MBOXIRQ_IRQCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Cortex-M33 (CPU0) Interrupt..CoolFlux (CPU1) Interrupt"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBOXIRQ_IRQ(pub u32);
    impl MBOXIRQ_IRQ {
        #[inline(always)]
        pub const fn INTREQ(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INTREQ(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MBOXIRQ_IRQ {
        #[inline(always)]
        fn default() -> MBOXIRQ_IRQ {
            MBOXIRQ_IRQ(0)
        }
    }
    #[doc = "Cortex-M33 (CPU0) Interrupt Clear..CoolFlux (CPU1) Interrupt Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBOXIRQ_IRQCLR(pub u32);
    impl MBOXIRQ_IRQCLR {
        #[inline(always)]
        pub const fn INTREQCLR(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INTREQCLR(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MBOXIRQ_IRQCLR {
        #[inline(always)]
        fn default() -> MBOXIRQ_IRQCLR {
            MBOXIRQ_IRQCLR(0)
        }
    }
    #[doc = "Cortex-M33 (CPU0) Interrupt Set..CoolFlux (CPU1) Interrupt Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBOXIRQ_IRQSET(pub u32);
    impl MBOXIRQ_IRQSET {
        #[inline(always)]
        pub const fn INTREQSET(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INTREQSET(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MBOXIRQ_IRQSET {
        #[inline(always)]
        fn default() -> MBOXIRQ_IRQSET {
            MBOXIRQ_IRQSET(0)
        }
    }
    #[doc = "Mutual Exclusion"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MUTEX(pub u32);
    impl MUTEX {
        #[inline(always)]
        pub const fn EX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MUTEX {
        #[inline(always)]
        fn default() -> MUTEX {
            MUTEX(0)
        }
    }
}
