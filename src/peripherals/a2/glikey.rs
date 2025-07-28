#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GLIKEY {
    ptr: *mut u8,
}
unsafe impl Send for GLIKEY {}
unsafe impl Sync for GLIKEY {}
impl GLIKEY {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL_0(self) -> crate::common::Reg<regs::CTRL_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_1(self) -> crate::common::Reg<regs::CTRL_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn INTR_CTRL(self) -> crate::common::Reg<regs::INTR_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn VERSION(self) -> crate::common::Reg<regs::VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Register 0 SFR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_0(pub u32);
    impl CTRL_0 {
        #[must_use]
        #[inline(always)]
        pub const fn WRITE_INDEX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WRITE_INDEX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED15(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RESERVED15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_EN_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WR_EN_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SFT_RST(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SFT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RESERVED31(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
        }
    }
    impl Default for CTRL_0 {
        #[inline(always)]
        fn default() -> CTRL_0 {
            CTRL_0(0)
        }
    }
    impl core::fmt::Debug for CTRL_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_0")
                .field("WRITE_INDEX", &self.WRITE_INDEX())
                .field("RESERVED15", &self.RESERVED15())
                .field("WR_EN_0", &self.WR_EN_0())
                .field("SFT_RST", &self.SFT_RST())
                .field("RESERVED31", &self.RESERVED31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL_0 {{ WRITE_INDEX: {=u8:?}, RESERVED15: {=u8:?}, WR_EN_0: {=u8:?}, SFT_RST: {=bool:?}, RESERVED31: {=u16:?} }}" , self . WRITE_INDEX () , self . RESERVED15 () , self . WR_EN_0 () , self . SFT_RST () , self . RESERVED31 ())
        }
    }
    #[doc = "Control Register 1 SFR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_1(pub u32);
    impl CTRL_1 {
        #[must_use]
        #[inline(always)]
        pub const fn READ_INDEX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_READ_INDEX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED15(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RESERVED15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WR_EN_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WR_EN_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SFR_LOCK(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SFR_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u16 {
            let val = (self.0 >> 22usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RESERVED31(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
        }
    }
    impl Default for CTRL_1 {
        #[inline(always)]
        fn default() -> CTRL_1 {
            CTRL_1(0)
        }
    }
    impl core::fmt::Debug for CTRL_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_1")
                .field("READ_INDEX", &self.READ_INDEX())
                .field("RESERVED15", &self.RESERVED15())
                .field("WR_EN_1", &self.WR_EN_1())
                .field("SFR_LOCK", &self.SFR_LOCK())
                .field("RESERVED31", &self.RESERVED31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL_1 {{ READ_INDEX: {=u8:?}, RESERVED15: {=u8:?}, WR_EN_1: {=u8:?}, SFR_LOCK: {=u8:?}, RESERVED31: {=u16:?} }}" , self . READ_INDEX () , self . RESERVED15 () , self . WR_EN_1 () , self . SFR_LOCK () , self . RESERVED31 ())
        }
    }
    #[doc = "Interrupt Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTR_CTRL(pub u32);
    impl INTR_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn INT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT_CLR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INT_SET(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RESERVED31(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for INTR_CTRL {
        #[inline(always)]
        fn default() -> INTR_CTRL {
            INTR_CTRL(0)
        }
    }
    impl core::fmt::Debug for INTR_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTR_CTRL")
                .field("INT_EN", &self.INT_EN())
                .field("INT_CLR", &self.INT_CLR())
                .field("INT_SET", &self.INT_SET())
                .field("RESERVED31", &self.RESERVED31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTR_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INTR_CTRL {{ INT_EN: {=bool:?}, INT_CLR: {=bool:?}, INT_SET: {=bool:?}, RESERVED31: {=u32:?} }}" , self . INT_EN () , self . INT_CLR () , self . INT_SET () , self . RESERVED31 ())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn INT_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_STATUS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERROR_STATUS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERROR_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESERVED18(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RESERVED18(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 5usize)) | (((val as u32) & 0x3fff) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FSM_STATE(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FSM_STATE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
        }
    }
    impl Default for STATUS {
        #[inline(always)]
        fn default() -> STATUS {
            STATUS(0)
        }
    }
    impl core::fmt::Debug for STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS")
                .field("INT_STATUS", &self.INT_STATUS())
                .field("LOCK_STATUS", &self.LOCK_STATUS())
                .field("ERROR_STATUS", &self.ERROR_STATUS())
                .field("RESERVED18", &self.RESERVED18())
                .field("FSM_STATE", &self.FSM_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS {{ INT_STATUS: {=bool:?}, LOCK_STATUS: {=bool:?}, ERROR_STATUS: {=u8:?}, RESERVED18: {=u16:?}, FSM_STATE: {=u16:?} }}" , self . INT_STATUS () , self . LOCK_STATUS () , self . ERROR_STATUS () , self . RESERVED18 () , self . FSM_STATE ())
        }
    }
    #[doc = "IP Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERSION(pub u32);
    impl VERSION {
        #[must_use]
        #[inline(always)]
        pub const fn Reserved3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved7(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved11(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved15(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MILESTONE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MILESTONE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FSM_CONFIG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FSM_CONFIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INDEX_CONFIG(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INDEX_CONFIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 19usize)) | (((val as u32) & 0xff) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Reserved31(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Reserved31(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for VERSION {
        #[inline(always)]
        fn default() -> VERSION {
            VERSION(0)
        }
    }
    impl core::fmt::Debug for VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERSION")
                .field("Reserved3", &self.Reserved3())
                .field("Reserved7", &self.Reserved7())
                .field("Reserved11", &self.Reserved11())
                .field("Reserved15", &self.Reserved15())
                .field("MILESTONE", &self.MILESTONE())
                .field("FSM_CONFIG", &self.FSM_CONFIG())
                .field("INDEX_CONFIG", &self.INDEX_CONFIG())
                .field("Reserved31", &self.Reserved31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERSION {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VERSION {{ Reserved3: {=u8:?}, Reserved7: {=u8:?}, Reserved11: {=u8:?}, Reserved15: {=u8:?}, MILESTONE: {=u8:?}, FSM_CONFIG: {=bool:?}, INDEX_CONFIG: {=u8:?}, Reserved31: {=u8:?} }}" , self . Reserved3 () , self . Reserved7 () , self . Reserved11 () , self . Reserved15 () , self . MILESTONE () , self . FSM_CONFIG () , self . INDEX_CONFIG () , self . Reserved31 ())
        }
    }
}
