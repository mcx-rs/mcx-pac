#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKETIMER {
    ptr: *mut u8,
}
unsafe impl Send for WAKETIMER {}
unsafe impl Sync for WAKETIMER {}
impl WAKETIMER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn WAKE_TIMER_CTRL(
        self,
    ) -> crate::common::Reg<regs::WAKE_TIMER_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKE_TIMER_CNT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Wake Timer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKE_TIMER_CTRL(pub u32);
    impl WAKE_TIMER_CTRL {
        #[inline(always)]
        pub const fn WAKE_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKE_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLR_WAKE_TIMER(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLR_WAKE_TIMER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn OSC_DIV_ENA(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_DIV_ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INTR_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for WAKE_TIMER_CTRL {
        #[inline(always)]
        fn default() -> WAKE_TIMER_CTRL {
            WAKE_TIMER_CTRL(0)
        }
    }
    impl core::fmt::Debug for WAKE_TIMER_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKE_TIMER_CTRL")
                .field("WAKE_FLAG", &self.WAKE_FLAG())
                .field("CLR_WAKE_TIMER", &self.CLR_WAKE_TIMER())
                .field("OSC_DIV_ENA", &self.OSC_DIV_ENA())
                .field("INTR_EN", &self.INTR_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKE_TIMER_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct WAKE_TIMER_CTRL {
                WAKE_FLAG: bool,
                CLR_WAKE_TIMER: bool,
                OSC_DIV_ENA: bool,
                INTR_EN: bool,
            }
            let proxy = WAKE_TIMER_CTRL {
                WAKE_FLAG: self.WAKE_FLAG(),
                CLR_WAKE_TIMER: self.CLR_WAKE_TIMER(),
                OSC_DIV_ENA: self.OSC_DIV_ENA(),
                INTR_EN: self.INTR_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
