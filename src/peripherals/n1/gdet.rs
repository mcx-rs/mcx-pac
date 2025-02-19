#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GDET {
    ptr: *mut u8,
}
unsafe impl Send for GDET {}
unsafe impl Sync for GDET {}
impl GDET {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GDET_CONF_0(self) -> crate::common::Reg<regs::GDET_CONF_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_1(self) -> crate::common::Reg<regs::GDET_CONF_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_ENABLE1(self) -> crate::common::Reg<regs::GDET_ENABLE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_2(self) -> crate::common::Reg<regs::GDET_CONF_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_3(self) -> crate::common::Reg<regs::GDET_CONF_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_4(self) -> crate::common::Reg<regs::GDET_CONF_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_CONF_5(self) -> crate::common::Reg<regs::GDET_CONF_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_RESET(self) -> crate::common::Reg<regs::GDET_RESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_TEST(self) -> crate::common::Reg<regs::GDET_TEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc4usize) as _) }
    }
    #[inline(always)]
    pub const fn GDET_DLY_CTRL(self) -> crate::common::Reg<regs::GDET_DLY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fccusize) as _) }
    }
}
pub mod regs {
    #[doc = "GDET Configuration 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_0(pub u32);
    impl GDET_CONF_0 {
        #[inline(always)]
        pub const fn FIELD_3_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_3_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn SBZ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 5usize) & 0x07ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
        }
    }
    impl Default for GDET_CONF_0 {
        #[inline(always)]
        fn default() -> GDET_CONF_0 {
            GDET_CONF_0(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_0")
                .field("FIELD_3_0", &self.FIELD_3_0())
                .field("SBZ", &self.SBZ())
                .field("RFU", &self.RFU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_CONF_0 {{ FIELD_3_0: {=u8:?}, SBZ: {=bool:?}, RFU: {=u32:?} }}",
                self.FIELD_3_0(),
                self.SBZ(),
                self.RFU()
            )
        }
    }
    #[doc = "GDET Configuration 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_1(pub u32);
    impl GDET_CONF_1 {
        #[inline(always)]
        pub const fn FIELD_1_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn FIELD_3_2(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_3_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn SBZ1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn SBZ2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SBZ3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FIELD_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIELD_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FIELD_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIELD_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SBZ4(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SBZ5(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 11usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
        }
    }
    impl Default for GDET_CONF_1 {
        #[inline(always)]
        fn default() -> GDET_CONF_1 {
            GDET_CONF_1(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_1")
                .field("FIELD_1_0", &self.FIELD_1_0())
                .field("FIELD_3_2", &self.FIELD_3_2())
                .field("SBZ1", &self.SBZ1())
                .field("SBZ2", &self.SBZ2())
                .field("SBZ3", &self.SBZ3())
                .field("FIELD_7", &self.FIELD_7())
                .field("FIELD_8", &self.FIELD_8())
                .field("SBZ4", &self.SBZ4())
                .field("SBZ5", &self.SBZ5())
                .field("RFU", &self.RFU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_1 {{ FIELD_1_0: {=u8:?}, FIELD_3_2: {=u8:?}, SBZ1: {=bool:?}, SBZ2: {=bool:?}, SBZ3: {=bool:?}, FIELD_7: {=bool:?}, FIELD_8: {=bool:?}, SBZ4: {=bool:?}, SBZ5: {=bool:?}, RFU: {=u32:?} }}" , self . FIELD_1_0 () , self . FIELD_3_2 () , self . SBZ1 () , self . SBZ2 () , self . SBZ3 () , self . FIELD_7 () , self . FIELD_8 () , self . SBZ4 () , self . SBZ5 () , self . RFU ())
        }
    }
    #[doc = "GDET Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_2(pub u32);
    impl GDET_CONF_2 {
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[inline(always)]
        pub const fn RFU1(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RFU1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
        }
        #[inline(always)]
        pub const fn FIELD_21_16(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_21_16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[inline(always)]
        pub const fn RFU2(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFU2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn FIELD_29_24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_29_24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[inline(always)]
        pub const fn RFU3(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFU3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for GDET_CONF_2 {
        #[inline(always)]
        fn default() -> GDET_CONF_2 {
            GDET_CONF_2(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_2")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("RFU1", &self.RFU1())
                .field("FIELD_21_16", &self.FIELD_21_16())
                .field("RFU2", &self.RFU2())
                .field("FIELD_29_24", &self.FIELD_29_24())
                .field("RFU3", &self.RFU3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GDET_CONF_2 {{ FIELD_6_0: {=u8:?}, RFU1: {=u16:?}, FIELD_21_16: {=u8:?}, RFU2: {=u8:?}, FIELD_29_24: {=u8:?}, RFU3: {=u8:?} }}" , self . FIELD_6_0 () , self . RFU1 () , self . FIELD_21_16 () , self . RFU2 () , self . FIELD_29_24 () , self . RFU3 ())
        }
    }
    #[doc = "GDET Configuration 3 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_3(pub u32);
    impl GDET_CONF_3 {
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for GDET_CONF_3 {
        #[inline(always)]
        fn default() -> GDET_CONF_3 {
            GDET_CONF_3(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_3")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("RFU1", &self.RFU1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_CONF_3 {{ FIELD_6_0: {=u8:?}, RFU1: {=u32:?} }}",
                self.FIELD_6_0(),
                self.RFU1()
            )
        }
    }
    #[doc = "GDET Configuration 4 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_4(pub u32);
    impl GDET_CONF_4 {
        #[inline(always)]
        pub const fn FIELD_6_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_6_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for GDET_CONF_4 {
        #[inline(always)]
        fn default() -> GDET_CONF_4 {
            GDET_CONF_4(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_4")
                .field("FIELD_6_0", &self.FIELD_6_0())
                .field("RFU1", &self.RFU1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_CONF_4 {{ FIELD_6_0: {=u8:?}, RFU1: {=u32:?} }}",
                self.FIELD_6_0(),
                self.RFU1()
            )
        }
    }
    #[doc = "GDET Configuration 5 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_CONF_5(pub u32);
    impl GDET_CONF_5 {
        #[inline(always)]
        pub const fn FIELD_5_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_5_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn FIELD_11_6(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIELD_11_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[inline(always)]
        pub const fn RFU1(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU1(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for GDET_CONF_5 {
        #[inline(always)]
        fn default() -> GDET_CONF_5 {
            GDET_CONF_5(0)
        }
    }
    impl core::fmt::Debug for GDET_CONF_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_CONF_5")
                .field("FIELD_5_0", &self.FIELD_5_0())
                .field("FIELD_11_6", &self.FIELD_11_6())
                .field("RFU1", &self.RFU1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_CONF_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_CONF_5 {{ FIELD_5_0: {=u8:?}, FIELD_11_6: {=u8:?}, RFU1: {=u32:?} }}",
                self.FIELD_5_0(),
                self.FIELD_11_6(),
                self.RFU1()
            )
        }
    }
    #[doc = "GDET Delay Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_DLY_CTRL(pub u32);
    impl GDET_DLY_CTRL {
        #[inline(always)]
        pub const fn VOL_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VOL_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn SW_VOL_CTRL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SW_VOL_CTRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for GDET_DLY_CTRL {
        #[inline(always)]
        fn default() -> GDET_DLY_CTRL {
            GDET_DLY_CTRL(0)
        }
    }
    impl core::fmt::Debug for GDET_DLY_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_DLY_CTRL")
                .field("VOL_SEL", &self.VOL_SEL())
                .field("SW_VOL_CTRL", &self.SW_VOL_CTRL())
                .field("RFU", &self.RFU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_DLY_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_DLY_CTRL {{ VOL_SEL: {=u8:?}, SW_VOL_CTRL: {=bool:?}, RFU: {=u32:?} }}",
                self.VOL_SEL(),
                self.SW_VOL_CTRL(),
                self.RFU()
            )
        }
    }
    #[doc = "GDET Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_ENABLE1(pub u32);
    impl GDET_ENABLE1 {
        #[inline(always)]
        pub const fn EN1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for GDET_ENABLE1 {
        #[inline(always)]
        fn default() -> GDET_ENABLE1 {
            GDET_ENABLE1(0)
        }
    }
    impl core::fmt::Debug for GDET_ENABLE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_ENABLE1")
                .field("EN1", &self.EN1())
                .field("RFU", &self.RFU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_ENABLE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_ENABLE1 {{ EN1: {=bool:?}, RFU: {=u32:?} }}",
                self.EN1(),
                self.RFU()
            )
        }
    }
    #[doc = "GDET Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_RESET(pub u32);
    impl GDET_RESET {
        #[inline(always)]
        pub const fn RFU1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RFU1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn SFT_RST(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RFU2(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for GDET_RESET {
        #[inline(always)]
        fn default() -> GDET_RESET {
            GDET_RESET(0)
        }
    }
    impl core::fmt::Debug for GDET_RESET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_RESET")
                .field("RFU1", &self.RFU1())
                .field("SFT_RST", &self.SFT_RST())
                .field("RFU2", &self.RFU2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_RESET {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_RESET {{ RFU1: {=u8:?}, SFT_RST: {=bool:?}, RFU2: {=u32:?} }}",
                self.RFU1(),
                self.SFT_RST(),
                self.RFU2()
            )
        }
    }
    #[doc = "GDET Test Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GDET_TEST(pub u32);
    impl GDET_TEST {
        #[inline(always)]
        pub const fn SBZ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RFU(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RFU(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for GDET_TEST {
        #[inline(always)]
        fn default() -> GDET_TEST {
            GDET_TEST(0)
        }
    }
    impl core::fmt::Debug for GDET_TEST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GDET_TEST")
                .field("SBZ", &self.SBZ())
                .field("RFU", &self.RFU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GDET_TEST {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GDET_TEST {{ SBZ: {=bool:?}, RFU: {=u32:?} }}",
                self.SBZ(),
                self.RFU()
            )
        }
    }
}
