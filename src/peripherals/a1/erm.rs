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
    pub const fn SR0(self) -> crate::common::Reg<regs::SR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
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
    pub const fn CORR_ERR_CNT1(self) -> crate::common::Reg<regs::CORR_ERR_CNT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
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
    #[doc = "ERM Configuration Register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR0(pub u32);
    impl CR0 {
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
                ENCIE1: bool,
                ESCIE1: bool,
                ENCIE0: bool,
                ESCIE0: bool,
            }
            let proxy = CR0 {
                ENCIE1: self.ENCIE1(),
                ESCIE1: self.ESCIE1(),
                ENCIE0: self.ENCIE0(),
                ESCIE0: self.ESCIE0(),
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
                NCE1: bool,
                SBC1: bool,
                NCE0: bool,
                SBC0: bool,
            }
            let proxy = SR0 {
                NCE1: self.NCE1(),
                SBC1: self.SBC1(),
                NCE0: self.NCE0(),
                SBC0: self.SBC0(),
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
}
