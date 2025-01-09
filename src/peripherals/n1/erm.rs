#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERM {
    ptr: *mut u8,
}
unsafe impl Send for ERM {}
unsafe impl Sync for ERM {}
impl ERM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CR0(self) -> crate::common::Reg<regs::CR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CR1(self) -> crate::common::Reg<regs::CR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SR0(self) -> crate::common::Reg<regs::SR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SR1(self) -> crate::common::Reg<regs::SR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn EAR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN0(self) -> crate::common::Reg<regs::SYN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT0(self) -> crate::common::Reg<regs::CORR_ERR_CNT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn EAR1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN1(self) -> crate::common::Reg<regs::SYN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT1(self) -> crate::common::Reg<regs::CORR_ERR_CNT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn EAR2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN2(self) -> crate::common::Reg<regs::SYN2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT2(self) -> crate::common::Reg<regs::CORR_ERR_CNT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn EAR3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN3(self) -> crate::common::Reg<regs::SYN3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT3(self) -> crate::common::Reg<regs::CORR_ERR_CNT3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn EAR4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN4(self) -> crate::common::Reg<regs::SYN4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT4(self) -> crate::common::Reg<regs::CORR_ERR_CNT4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT5(self) -> crate::common::Reg<regs::CORR_ERR_CNT5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT6(self) -> crate::common::Reg<regs::CORR_ERR_CNT6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT7(self) -> crate::common::Reg<regs::CORR_ERR_CNT7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn SYN8(self) -> crate::common::Reg<regs::SYN8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT8(self) -> crate::common::Reg<regs::CORR_ERR_CNT8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn CORR_ERR_CNT9(self) -> crate::common::Reg<regs::CORR_ERR_CNT9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
}
pub mod regs {
    #[doc = "ERM Memory 0 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT0(pub u32);
    impl CORR_ERR_CNT0 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT0 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT0 {
            CORR_ERR_CNT0(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT0")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT0 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT0 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 1 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT1(pub u32);
    impl CORR_ERR_CNT1 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT1 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT1 {
            CORR_ERR_CNT1(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT1")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT1 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT1 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 2 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT2(pub u32);
    impl CORR_ERR_CNT2 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT2 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT2 {
            CORR_ERR_CNT2(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT2")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT2 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT2 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 3 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT3(pub u32);
    impl CORR_ERR_CNT3 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT3 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT3 {
            CORR_ERR_CNT3(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT3")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT3 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT3 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 4 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT4(pub u32);
    impl CORR_ERR_CNT4 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT4 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT4 {
            CORR_ERR_CNT4(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT4")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT4 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT4 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 5 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT5(pub u32);
    impl CORR_ERR_CNT5 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT5 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT5 {
            CORR_ERR_CNT5(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT5")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT5 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT5 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 6 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT6(pub u32);
    impl CORR_ERR_CNT6 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT6 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT6 {
            CORR_ERR_CNT6(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT6")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT6 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT6 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT6 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 7 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT7(pub u32);
    impl CORR_ERR_CNT7 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT7 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT7 {
            CORR_ERR_CNT7(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT7")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT7 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT7 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT7 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 8 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT8(pub u32);
    impl CORR_ERR_CNT8 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT8 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT8 {
            CORR_ERR_CNT8(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT8")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT8 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT8 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT8 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 9 Correctable Error Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORR_ERR_CNT9(pub u32);
    impl CORR_ERR_CNT9 {
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CORR_ERR_CNT9 {
        #[inline(always)]
        fn default() -> CORR_ERR_CNT9 {
            CORR_ERR_CNT9(0)
        }
    }
    impl core::fmt::Debug for CORR_ERR_CNT9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORR_ERR_CNT9")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORR_ERR_CNT9 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CORR_ERR_CNT9 {
                COUNT: u8,
            }
            let proxy = CORR_ERR_CNT9 {
                COUNT: self.COUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Configuration Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR0(pub u32);
    impl CR0 {
        #[inline(always)]
        pub const fn ENCIE7(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ESCIE7(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENCIE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ESCIE6(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ENCIE5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ESCIE5(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ENCIE4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ESCIE4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ENCIE3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ESCIE3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ENCIE2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn ESCIE2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn ENCIE1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ESCIE1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ENCIE0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ESCIE0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CR0 {
        #[inline(always)]
        fn default() -> CR0 {
            CR0(0)
        }
    }
    impl core::fmt::Debug for CR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR0")
                .field("ENCIE7", &self.ENCIE7())
                .field("ESCIE7", &self.ESCIE7())
                .field("ENCIE6", &self.ENCIE6())
                .field("ESCIE6", &self.ESCIE6())
                .field("ENCIE5", &self.ENCIE5())
                .field("ESCIE5", &self.ESCIE5())
                .field("ENCIE4", &self.ENCIE4())
                .field("ESCIE4", &self.ESCIE4())
                .field("ENCIE3", &self.ENCIE3())
                .field("ESCIE3", &self.ESCIE3())
                .field("ENCIE2", &self.ENCIE2())
                .field("ESCIE2", &self.ESCIE2())
                .field("ENCIE1", &self.ENCIE1())
                .field("ESCIE1", &self.ESCIE1())
                .field("ENCIE0", &self.ENCIE0())
                .field("ESCIE0", &self.ESCIE0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CR0 {
                ENCIE7: bool,
                ESCIE7: bool,
                ENCIE6: bool,
                ESCIE6: bool,
                ENCIE5: bool,
                ESCIE5: bool,
                ENCIE4: bool,
                ESCIE4: bool,
                ENCIE3: bool,
                ESCIE3: bool,
                ENCIE2: bool,
                ESCIE2: bool,
                ENCIE1: bool,
                ESCIE1: bool,
                ENCIE0: bool,
                ESCIE0: bool,
            }
            let proxy = CR0 {
                ENCIE7: self.ENCIE7(),
                ESCIE7: self.ESCIE7(),
                ENCIE6: self.ENCIE6(),
                ESCIE6: self.ESCIE6(),
                ENCIE5: self.ENCIE5(),
                ESCIE5: self.ESCIE5(),
                ENCIE4: self.ENCIE4(),
                ESCIE4: self.ESCIE4(),
                ENCIE3: self.ENCIE3(),
                ESCIE3: self.ESCIE3(),
                ENCIE2: self.ENCIE2(),
                ESCIE2: self.ESCIE2(),
                ENCIE1: self.ENCIE1(),
                ESCIE1: self.ESCIE1(),
                ENCIE0: self.ENCIE0(),
                ESCIE0: self.ESCIE0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Configuration Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR1(pub u32);
    impl CR1 {
        #[inline(always)]
        pub const fn ENCIE9(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ESCIE9(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ENCIE8(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENCIE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ESCIE8(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ESCIE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CR1 {
        #[inline(always)]
        fn default() -> CR1 {
            CR1(0)
        }
    }
    impl core::fmt::Debug for CR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR1")
                .field("ENCIE9", &self.ENCIE9())
                .field("ESCIE9", &self.ESCIE9())
                .field("ENCIE8", &self.ENCIE8())
                .field("ESCIE8", &self.ESCIE8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CR1 {
                ENCIE9: bool,
                ESCIE9: bool,
                ENCIE8: bool,
                ESCIE8: bool,
            }
            let proxy = CR1 {
                ENCIE9: self.ENCIE9(),
                ESCIE9: self.ESCIE9(),
                ENCIE8: self.ENCIE8(),
                ESCIE8: self.ESCIE8(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Status Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR0(pub u32);
    impl SR0 {
        #[inline(always)]
        pub const fn NCE7(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SBC7(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn NCE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SBC6(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NCE5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SBC5(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn NCE4(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SBC4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn NCE3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn SBC3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn NCE2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn SBC2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn NCE1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SBC1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn NCE0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SBC0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SR0 {
        #[inline(always)]
        fn default() -> SR0 {
            SR0(0)
        }
    }
    impl core::fmt::Debug for SR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SR0")
                .field("NCE7", &self.NCE7())
                .field("SBC7", &self.SBC7())
                .field("NCE6", &self.NCE6())
                .field("SBC6", &self.SBC6())
                .field("NCE5", &self.NCE5())
                .field("SBC5", &self.SBC5())
                .field("NCE4", &self.NCE4())
                .field("SBC4", &self.SBC4())
                .field("NCE3", &self.NCE3())
                .field("SBC3", &self.SBC3())
                .field("NCE2", &self.NCE2())
                .field("SBC2", &self.SBC2())
                .field("NCE1", &self.NCE1())
                .field("SBC1", &self.SBC1())
                .field("NCE0", &self.NCE0())
                .field("SBC0", &self.SBC0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SR0 {
                NCE7: bool,
                SBC7: bool,
                NCE6: bool,
                SBC6: bool,
                NCE5: bool,
                SBC5: bool,
                NCE4: bool,
                SBC4: bool,
                NCE3: bool,
                SBC3: bool,
                NCE2: bool,
                SBC2: bool,
                NCE1: bool,
                SBC1: bool,
                NCE0: bool,
                SBC0: bool,
            }
            let proxy = SR0 {
                NCE7: self.NCE7(),
                SBC7: self.SBC7(),
                NCE6: self.NCE6(),
                SBC6: self.SBC6(),
                NCE5: self.NCE5(),
                SBC5: self.SBC5(),
                NCE4: self.NCE4(),
                SBC4: self.SBC4(),
                NCE3: self.NCE3(),
                SBC3: self.SBC3(),
                NCE2: self.NCE2(),
                SBC2: self.SBC2(),
                NCE1: self.NCE1(),
                SBC1: self.SBC1(),
                NCE0: self.NCE0(),
                SBC0: self.SBC0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Status Register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR1(pub u32);
    impl SR1 {
        #[inline(always)]
        pub const fn NCE9(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SBC9(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn NCE8(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SBC8(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBC8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SR1 {
        #[inline(always)]
        fn default() -> SR1 {
            SR1(0)
        }
    }
    impl core::fmt::Debug for SR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SR1")
                .field("NCE9", &self.NCE9())
                .field("SBC9", &self.SBC9())
                .field("NCE8", &self.NCE8())
                .field("SBC8", &self.SBC8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SR1 {
                NCE9: bool,
                SBC9: bool,
                NCE8: bool,
                SBC8: bool,
            }
            let proxy = SR1 {
                NCE9: self.NCE9(),
                SBC9: self.SBC9(),
                NCE8: self.NCE8(),
                SBC8: self.SBC8(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 0 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN0(pub u32);
    impl SYN0 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN0 {
        #[inline(always)]
        fn default() -> SYN0 {
            SYN0(0)
        }
    }
    impl core::fmt::Debug for SYN0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN0")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN0 {
                SYNDROME: u8,
            }
            let proxy = SYN0 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 1 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN1(pub u32);
    impl SYN1 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN1 {
        #[inline(always)]
        fn default() -> SYN1 {
            SYN1(0)
        }
    }
    impl core::fmt::Debug for SYN1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN1")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN1 {
                SYNDROME: u8,
            }
            let proxy = SYN1 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 2 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN2(pub u32);
    impl SYN2 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN2 {
        #[inline(always)]
        fn default() -> SYN2 {
            SYN2(0)
        }
    }
    impl core::fmt::Debug for SYN2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN2")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN2 {
                SYNDROME: u8,
            }
            let proxy = SYN2 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 3 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN3(pub u32);
    impl SYN3 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN3 {
        #[inline(always)]
        fn default() -> SYN3 {
            SYN3(0)
        }
    }
    impl core::fmt::Debug for SYN3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN3")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN3 {
                SYNDROME: u8,
            }
            let proxy = SYN3 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 4 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN4(pub u32);
    impl SYN4 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN4 {
        #[inline(always)]
        fn default() -> SYN4 {
            SYN4(0)
        }
    }
    impl core::fmt::Debug for SYN4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN4")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN4 {
                SYNDROME: u8,
            }
            let proxy = SYN4 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "ERM Memory 8 Syndrome Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYN8(pub u32);
    impl SYN8 {
        #[inline(always)]
        pub const fn SYNDROME(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SYNDROME(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SYN8 {
        #[inline(always)]
        fn default() -> SYN8 {
            SYN8(0)
        }
    }
    impl core::fmt::Debug for SYN8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYN8")
                .field("SYNDROME", &self.SYNDROME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYN8 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYN8 {
                SYNDROME: u8,
            }
            let proxy = SYN8 {
                SYNDROME: self.SYNDROME(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
