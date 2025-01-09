#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHANNEL {
    ptr: *mut u8,
}
unsafe impl Send for CHANNEL {}
unsafe impl Sync for CHANNEL {}
impl CHANNEL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn INTVAL(self) -> crate::common::Reg<regs::CHANNEL_INTVAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER(self) -> crate::common::Reg<regs::CHANNEL_TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CHANNEL_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::CHANNEL_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MRT {
    ptr: *mut u8,
}
unsafe impl Send for MRT {}
unsafe impl Sync for MRT {}
impl MRT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CHANNEL(self, n: usize) -> CHANNEL {
        assert!(n < 4usize);
        unsafe { CHANNEL::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn MODCFG(self) -> crate::common::Reg<regs::MODCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn IDLE_CH(self) -> crate::common::Reg<regs::IDLE_CH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQ_FLAG(self) -> crate::common::Reg<regs::IRQ_FLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_CTRL(pub u32);
    impl CHANNEL_CTRL {
        #[inline(always)]
        pub const fn INTEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn MODE(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
    }
    impl Default for CHANNEL_CTRL {
        #[inline(always)]
        fn default() -> CHANNEL_CTRL {
            CHANNEL_CTRL(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_CTRL")
                .field("INTEN", &self.INTEN())
                .field("MODE", &self.MODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_CTRL {
                INTEN: bool,
                MODE: u8,
            }
            let proxy = CHANNEL_CTRL {
                INTEN: self.INTEN(),
                MODE: self.MODE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Time Interval Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_INTVAL(pub u32);
    impl CHANNEL_INTVAL {
        #[inline(always)]
        pub const fn IVALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_IVALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn LOAD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CHANNEL_INTVAL {
        #[inline(always)]
        fn default() -> CHANNEL_INTVAL {
            CHANNEL_INTVAL(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_INTVAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_INTVAL")
                .field("IVALUE", &self.IVALUE())
                .field("LOAD", &self.LOAD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_INTVAL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_INTVAL {
                IVALUE: u32,
                LOAD: bool,
            }
            let proxy = CHANNEL_INTVAL {
                IVALUE: self.IVALUE(),
                LOAD: self.LOAD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_STAT(pub u32);
    impl CHANNEL_STAT {
        #[inline(always)]
        pub const fn INTFLAG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTFLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RUN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INUSE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INUSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CHANNEL_STAT {
        #[inline(always)]
        fn default() -> CHANNEL_STAT {
            CHANNEL_STAT(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_STAT")
                .field("INTFLAG", &self.INTFLAG())
                .field("RUN", &self.RUN())
                .field("INUSE", &self.INUSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_STAT {
                INTFLAG: bool,
                RUN: bool,
                INUSE: bool,
            }
            let proxy = CHANNEL_STAT {
                INTFLAG: self.INTFLAG(),
                RUN: self.RUN(),
                INUSE: self.INUSE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CHANNEL_TIMER(pub u32);
    impl CHANNEL_TIMER {
        #[inline(always)]
        pub const fn VALUE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_VALUE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CHANNEL_TIMER {
        #[inline(always)]
        fn default() -> CHANNEL_TIMER {
            CHANNEL_TIMER(0)
        }
    }
    impl core::fmt::Debug for CHANNEL_TIMER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CHANNEL_TIMER")
                .field("VALUE", &self.VALUE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CHANNEL_TIMER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CHANNEL_TIMER {
                VALUE: u32,
            }
            let proxy = CHANNEL_TIMER {
                VALUE: self.VALUE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Idle Channel"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IDLE_CH(pub u32);
    impl IDLE_CH {
        #[inline(always)]
        pub const fn CHAN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHAN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for IDLE_CH {
        #[inline(always)]
        fn default() -> IDLE_CH {
            IDLE_CH(0)
        }
    }
    impl core::fmt::Debug for IDLE_CH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IDLE_CH")
                .field("CHAN", &self.CHAN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IDLE_CH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IDLE_CH {
                CHAN: u8,
            }
            let proxy = IDLE_CH { CHAN: self.CHAN() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Global Interrupt Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRQ_FLAG(pub u32);
    impl IRQ_FLAG {
        #[inline(always)]
        pub const fn GFLAG0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GFLAG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn GFLAG1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GFLAG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn GFLAG2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GFLAG2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn GFLAG3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GFLAG3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for IRQ_FLAG {
        #[inline(always)]
        fn default() -> IRQ_FLAG {
            IRQ_FLAG(0)
        }
    }
    impl core::fmt::Debug for IRQ_FLAG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IRQ_FLAG")
                .field("GFLAG0", &self.GFLAG0())
                .field("GFLAG1", &self.GFLAG1())
                .field("GFLAG2", &self.GFLAG2())
                .field("GFLAG3", &self.GFLAG3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IRQ_FLAG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IRQ_FLAG {
                GFLAG0: bool,
                GFLAG1: bool,
                GFLAG2: bool,
                GFLAG3: bool,
            }
            let proxy = IRQ_FLAG {
                GFLAG0: self.GFLAG0(),
                GFLAG1: self.GFLAG1(),
                GFLAG2: self.GFLAG2(),
                GFLAG3: self.GFLAG3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Module Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MODCFG(pub u32);
    impl MODCFG {
        #[inline(always)]
        pub const fn NOC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NOC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn NOB(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NOB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[inline(always)]
        pub const fn MULTITASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MULTITASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MODCFG {
        #[inline(always)]
        fn default() -> MODCFG {
            MODCFG(0)
        }
    }
    impl core::fmt::Debug for MODCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MODCFG")
                .field("NOC", &self.NOC())
                .field("NOB", &self.NOB())
                .field("MULTITASK", &self.MULTITASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MODCFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MODCFG {
                NOC: u8,
                NOB: u8,
                MULTITASK: bool,
            }
            let proxy = MODCFG {
                NOC: self.NOC(),
                NOB: self.NOB(),
                MULTITASK: self.MULTITASK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
