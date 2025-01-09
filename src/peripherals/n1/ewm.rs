#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EWM {
    ptr: *mut u8,
}
unsafe impl Send for EWM {}
unsafe impl Sync for EWM {}
impl EWM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn SERV(self) -> crate::common::Reg<regs::SERV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[inline(always)]
    pub const fn CMPL(self) -> crate::common::Reg<regs::CMPL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn CMPH(self) -> crate::common::Reg<regs::CMPH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKCTRL(self) -> crate::common::Reg<regs::CLKCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKPRESCALER(self) -> crate::common::Reg<regs::CLKPRESCALER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKCTRL(pub u32);
    impl CLKCTRL {
        #[inline(always)]
        pub const fn CLKSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for CLKCTRL {
        #[inline(always)]
        fn default() -> CLKCTRL {
            CLKCTRL(0)
        }
    }
    impl core::fmt::Debug for CLKCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKCTRL")
                .field("CLKSEL", &self.CLKSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKCTRL {
                CLKSEL: u8,
            }
            let proxy = CLKCTRL {
                CLKSEL: self.CLKSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock Prescaler"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKPRESCALER(pub u32);
    impl CLKPRESCALER {
        #[inline(always)]
        pub const fn CLK_DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLK_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CLKPRESCALER {
        #[inline(always)]
        fn default() -> CLKPRESCALER {
            CLKPRESCALER(0)
        }
    }
    impl core::fmt::Debug for CLKPRESCALER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKPRESCALER")
                .field("CLK_DIV", &self.CLK_DIV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKPRESCALER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLKPRESCALER {
                CLK_DIV: u8,
            }
            let proxy = CLKPRESCALER {
                CLK_DIV: self.CLK_DIV(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Compare High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMPH(pub u32);
    impl CMPH {
        #[inline(always)]
        pub const fn COMPAREH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COMPAREH(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CMPH {
        #[inline(always)]
        fn default() -> CMPH {
            CMPH(0)
        }
    }
    impl core::fmt::Debug for CMPH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMPH")
                .field("COMPAREH", &self.COMPAREH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMPH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMPH {
                COMPAREH: u8,
            }
            let proxy = CMPH {
                COMPAREH: self.COMPAREH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Compare Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMPL(pub u32);
    impl CMPL {
        #[inline(always)]
        pub const fn COMPAREL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COMPAREL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CMPL {
        #[inline(always)]
        fn default() -> CMPL {
            CMPL(0)
        }
    }
    impl core::fmt::Debug for CMPL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMPL")
                .field("COMPAREL", &self.COMPAREL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMPL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CMPL {
                COMPAREL: u8,
            }
            let proxy = CMPL {
                COMPAREL: self.COMPAREL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn EWMEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EWMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ASSIN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASSIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for CTRL {
        #[inline(always)]
        fn default() -> CTRL {
            CTRL(0)
        }
    }
    impl core::fmt::Debug for CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL")
                .field("EWMEN", &self.EWMEN())
                .field("ASSIN", &self.ASSIN())
                .field("INEN", &self.INEN())
                .field("INTEN", &self.INTEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL {
                EWMEN: bool,
                ASSIN: bool,
                INEN: bool,
                INTEN: bool,
            }
            let proxy = CTRL {
                EWMEN: self.EWMEN(),
                ASSIN: self.ASSIN(),
                INEN: self.INEN(),
                INTEN: self.INTEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Service"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SERV(pub u32);
    impl SERV {
        #[inline(always)]
        pub const fn SERVICE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SERVICE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SERV {
        #[inline(always)]
        fn default() -> SERV {
            SERV(0)
        }
    }
    impl core::fmt::Debug for SERV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SERV")
                .field("SERVICE", &self.SERVICE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SERV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SERV {
                SERVICE: u8,
            }
            let proxy = SERV {
                SERVICE: self.SERVICE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
