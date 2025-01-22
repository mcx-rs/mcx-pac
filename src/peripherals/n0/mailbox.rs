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
        unsafe { MBOXIRQ::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
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
    pub const fn IRQ(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQCLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
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
    impl core::fmt::Debug for MUTEX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MUTEX").field("EX", &self.EX()).finish()
        }
    }
}
