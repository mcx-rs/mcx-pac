#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAU {
    ptr: *mut u8,
}
unsafe impl Send for MAU {}
unsafe impl Sync for MAU {}
impl MAU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SYS_CTLR(self) -> crate::common::Reg<regs::SYS_CTLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn GEXP_STATUS_IE(
        self,
    ) -> crate::common::Reg<regs::GEXP_STATUS_IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn GEXP_STATUS(self) -> crate::common::Reg<regs::GEXP_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn OP_CTRL(self) -> crate::common::Reg<regs::OP_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn RES_STATUS_IE(self) -> crate::common::Reg<regs::RES_STATUS_IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn RES_STATUS(self) -> crate::common::Reg<regs::RES_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn RES0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn RES1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn RES2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn RES3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "General Exception Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GEXP_STATUS(pub u32);
    impl GEXP_STATUS {
        #[inline(always)]
        pub const fn ERROR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for GEXP_STATUS {
        #[inline(always)]
        fn default() -> GEXP_STATUS {
            GEXP_STATUS(0)
        }
    }
    impl core::fmt::Debug for GEXP_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GEXP_STATUS")
                .field("ERROR", &self.ERROR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GEXP_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GEXP_STATUS {
                ERROR: bool,
            }
            let proxy = GEXP_STATUS {
                ERROR: self.ERROR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "General Exception Status Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GEXP_STATUS_IE(pub u32);
    impl GEXP_STATUS_IE {
        #[inline(always)]
        pub const fn ERROR_IE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for GEXP_STATUS_IE {
        #[inline(always)]
        fn default() -> GEXP_STATUS_IE {
            GEXP_STATUS_IE(0)
        }
    }
    impl core::fmt::Debug for GEXP_STATUS_IE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GEXP_STATUS_IE")
                .field("ERROR_IE", &self.ERROR_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GEXP_STATUS_IE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct GEXP_STATUS_IE {
                ERROR_IE: bool,
            }
            let proxy = GEXP_STATUS_IE {
                ERROR_IE: self.ERROR_IE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Operation Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OP_CTRL(pub u32);
    impl OP_CTRL {
        #[inline(always)]
        pub const fn OVDT_EN_RES0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVDT_EN_RES0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OVDT_RES0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OVDT_RES0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn OVDT_EN_RES1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVDT_EN_RES1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OVDT_RES1(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OVDT_RES1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn OVDT_EN_RES2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVDT_EN_RES2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn OVDT_RES2(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OVDT_RES2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[inline(always)]
        pub const fn OVDT_EN_RES3(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVDT_EN_RES3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn OVDT_RES3(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OVDT_RES3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for OP_CTRL {
        #[inline(always)]
        fn default() -> OP_CTRL {
            OP_CTRL(0)
        }
    }
    impl core::fmt::Debug for OP_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OP_CTRL")
                .field("OVDT_EN_RES0", &self.OVDT_EN_RES0())
                .field("OVDT_RES0", &self.OVDT_RES0())
                .field("OVDT_EN_RES1", &self.OVDT_EN_RES1())
                .field("OVDT_RES1", &self.OVDT_RES1())
                .field("OVDT_EN_RES2", &self.OVDT_EN_RES2())
                .field("OVDT_RES2", &self.OVDT_RES2())
                .field("OVDT_EN_RES3", &self.OVDT_EN_RES3())
                .field("OVDT_RES3", &self.OVDT_RES3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OP_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OP_CTRL {
                OVDT_EN_RES0: bool,
                OVDT_RES0: u8,
                OVDT_EN_RES1: bool,
                OVDT_RES1: u8,
                OVDT_EN_RES2: bool,
                OVDT_RES2: u8,
                OVDT_EN_RES3: bool,
                OVDT_RES3: u8,
            }
            let proxy = OP_CTRL {
                OVDT_EN_RES0: self.OVDT_EN_RES0(),
                OVDT_RES0: self.OVDT_RES0(),
                OVDT_EN_RES1: self.OVDT_EN_RES1(),
                OVDT_RES1: self.OVDT_RES1(),
                OVDT_EN_RES2: self.OVDT_EN_RES2(),
                OVDT_RES2: self.OVDT_RES2(),
                OVDT_EN_RES3: self.OVDT_EN_RES3(),
                OVDT_RES3: self.OVDT_RES3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Result Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RES_STATUS(pub u32);
    impl RES_STATUS {
        #[inline(always)]
        pub const fn RES0_NX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_NX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RES0_UF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_UF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RES0_OF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_OF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RES0_DZ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_DZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RES0_NV(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_NV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RES0_ERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RES0_OVWR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_OVWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RES0_FULL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RES1_NX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_NX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RES1_UF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_UF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RES1_OF(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_OF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn RES1_DZ(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_DZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn RES1_NV(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_NV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RES1_ERR(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn RES1_OVWR(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_OVWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn RES1_FULL(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn RES2_NX(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_NX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn RES2_UF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_UF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RES2_OF(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_OF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RES2_DZ(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_DZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RES2_NV(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_NV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn RES2_ERR(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn RES2_OVWR(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_OVWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn RES2_FULL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn RES3_NX(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_NX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn RES3_UF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_UF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn RES3_OF(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_OF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn RES3_DZ(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_DZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn RES3_NV(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_NV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn RES3_ERR(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn RES3_OVWR(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_OVWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn RES3_FULL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RES_STATUS {
        #[inline(always)]
        fn default() -> RES_STATUS {
            RES_STATUS(0)
        }
    }
    impl core::fmt::Debug for RES_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RES_STATUS")
                .field("RES0_NX", &self.RES0_NX())
                .field("RES0_UF", &self.RES0_UF())
                .field("RES0_OF", &self.RES0_OF())
                .field("RES0_DZ", &self.RES0_DZ())
                .field("RES0_NV", &self.RES0_NV())
                .field("RES0_ERR", &self.RES0_ERR())
                .field("RES0_OVWR", &self.RES0_OVWR())
                .field("RES0_FULL", &self.RES0_FULL())
                .field("RES1_NX", &self.RES1_NX())
                .field("RES1_UF", &self.RES1_UF())
                .field("RES1_OF", &self.RES1_OF())
                .field("RES1_DZ", &self.RES1_DZ())
                .field("RES1_NV", &self.RES1_NV())
                .field("RES1_ERR", &self.RES1_ERR())
                .field("RES1_OVWR", &self.RES1_OVWR())
                .field("RES1_FULL", &self.RES1_FULL())
                .field("RES2_NX", &self.RES2_NX())
                .field("RES2_UF", &self.RES2_UF())
                .field("RES2_OF", &self.RES2_OF())
                .field("RES2_DZ", &self.RES2_DZ())
                .field("RES2_NV", &self.RES2_NV())
                .field("RES2_ERR", &self.RES2_ERR())
                .field("RES2_OVWR", &self.RES2_OVWR())
                .field("RES2_FULL", &self.RES2_FULL())
                .field("RES3_NX", &self.RES3_NX())
                .field("RES3_UF", &self.RES3_UF())
                .field("RES3_OF", &self.RES3_OF())
                .field("RES3_DZ", &self.RES3_DZ())
                .field("RES3_NV", &self.RES3_NV())
                .field("RES3_ERR", &self.RES3_ERR())
                .field("RES3_OVWR", &self.RES3_OVWR())
                .field("RES3_FULL", &self.RES3_FULL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RES_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RES_STATUS {
                RES0_NX: bool,
                RES0_UF: bool,
                RES0_OF: bool,
                RES0_DZ: bool,
                RES0_NV: bool,
                RES0_ERR: bool,
                RES0_OVWR: bool,
                RES0_FULL: bool,
                RES1_NX: bool,
                RES1_UF: bool,
                RES1_OF: bool,
                RES1_DZ: bool,
                RES1_NV: bool,
                RES1_ERR: bool,
                RES1_OVWR: bool,
                RES1_FULL: bool,
                RES2_NX: bool,
                RES2_UF: bool,
                RES2_OF: bool,
                RES2_DZ: bool,
                RES2_NV: bool,
                RES2_ERR: bool,
                RES2_OVWR: bool,
                RES2_FULL: bool,
                RES3_NX: bool,
                RES3_UF: bool,
                RES3_OF: bool,
                RES3_DZ: bool,
                RES3_NV: bool,
                RES3_ERR: bool,
                RES3_OVWR: bool,
                RES3_FULL: bool,
            }
            let proxy = RES_STATUS {
                RES0_NX: self.RES0_NX(),
                RES0_UF: self.RES0_UF(),
                RES0_OF: self.RES0_OF(),
                RES0_DZ: self.RES0_DZ(),
                RES0_NV: self.RES0_NV(),
                RES0_ERR: self.RES0_ERR(),
                RES0_OVWR: self.RES0_OVWR(),
                RES0_FULL: self.RES0_FULL(),
                RES1_NX: self.RES1_NX(),
                RES1_UF: self.RES1_UF(),
                RES1_OF: self.RES1_OF(),
                RES1_DZ: self.RES1_DZ(),
                RES1_NV: self.RES1_NV(),
                RES1_ERR: self.RES1_ERR(),
                RES1_OVWR: self.RES1_OVWR(),
                RES1_FULL: self.RES1_FULL(),
                RES2_NX: self.RES2_NX(),
                RES2_UF: self.RES2_UF(),
                RES2_OF: self.RES2_OF(),
                RES2_DZ: self.RES2_DZ(),
                RES2_NV: self.RES2_NV(),
                RES2_ERR: self.RES2_ERR(),
                RES2_OVWR: self.RES2_OVWR(),
                RES2_FULL: self.RES2_FULL(),
                RES3_NX: self.RES3_NX(),
                RES3_UF: self.RES3_UF(),
                RES3_OF: self.RES3_OF(),
                RES3_DZ: self.RES3_DZ(),
                RES3_NV: self.RES3_NV(),
                RES3_ERR: self.RES3_ERR(),
                RES3_OVWR: self.RES3_OVWR(),
                RES3_FULL: self.RES3_FULL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Result Status Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RES_STATUS_IE(pub u32);
    impl RES_STATUS_IE {
        #[inline(always)]
        pub const fn RES0_IE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES0_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RES1_IE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES1_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RES2_IE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES2_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RES3_IE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES3_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for RES_STATUS_IE {
        #[inline(always)]
        fn default() -> RES_STATUS_IE {
            RES_STATUS_IE(0)
        }
    }
    impl core::fmt::Debug for RES_STATUS_IE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RES_STATUS_IE")
                .field("RES0_IE", &self.RES0_IE())
                .field("RES1_IE", &self.RES1_IE())
                .field("RES2_IE", &self.RES2_IE())
                .field("RES3_IE", &self.RES3_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RES_STATUS_IE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RES_STATUS_IE {
                RES0_IE: bool,
                RES1_IE: bool,
                RES2_IE: bool,
                RES3_IE: bool,
            }
            let proxy = RES_STATUS_IE {
                RES0_IE: self.RES0_IE(),
                RES1_IE: self.RES1_IE(),
                RES2_IE: self.RES2_IE(),
                RES3_IE: self.RES3_IE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "System Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYS_CTLR(pub u32);
    impl SYS_CTLR {
        #[inline(always)]
        pub const fn ACG_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SYS_CTLR {
        #[inline(always)]
        fn default() -> SYS_CTLR {
            SYS_CTLR(0)
        }
    }
    impl core::fmt::Debug for SYS_CTLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYS_CTLR")
                .field("ACG_EN", &self.ACG_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYS_CTLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SYS_CTLR {
                ACG_EN: bool,
            }
            let proxy = SYS_CTLR {
                ACG_EN: self.ACG_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
