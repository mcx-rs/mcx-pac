#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
        #[inline(always)]
        pub const fn WRITE_INDEX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_WRITE_INDEX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RESERVED15(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RESERVED15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn WR_EN_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WR_EN_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn SFT_RST(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFT_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RESERVED31(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
        }
    }
    impl Default for CTRL_0 {
        #[inline(always)]
        fn default() -> CTRL_0 {
            CTRL_0(0)
        }
    }
    #[doc = "Control Register 1 SFR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_1(pub u32);
    impl CTRL_1 {
        #[inline(always)]
        pub const fn READ_INDEX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_READ_INDEX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RESERVED15(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RESERVED15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn WR_EN_1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_WR_EN_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn SFR_LOCK(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SFR_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u16 {
            let val = (self.0 >> 22usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RESERVED31(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
        }
    }
    impl Default for CTRL_1 {
        #[inline(always)]
        fn default() -> CTRL_1 {
            CTRL_1(0)
        }
    }
    #[doc = "Interrupt Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTR_CTRL(pub u32);
    impl INTR_CTRL {
        #[inline(always)]
        pub const fn INT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INT_CLR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INT_SET(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RESERVED31(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RESERVED31(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for INTR_CTRL {
        #[inline(always)]
        fn default() -> INTR_CTRL {
            INTR_CTRL(0)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[inline(always)]
        pub const fn INT_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LOCK_STATUS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ERROR_STATUS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERROR_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[inline(always)]
        pub const fn RESERVED18(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RESERVED18(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 5usize)) | (((val as u32) & 0x3fff) << 5usize);
        }
        #[inline(always)]
        pub const fn FSM_STATE(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FSM_STATE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
        }
    }
    impl Default for STATUS {
        #[inline(always)]
        fn default() -> STATUS {
            STATUS(0)
        }
    }
    #[doc = "IP Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERSION(pub u32);
    impl VERSION {
        #[inline(always)]
        pub const fn Reserved3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Reserved3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn Reserved7(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Reserved7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn Reserved11(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Reserved11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn Reserved15(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Reserved15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn MILESTONE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MILESTONE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn FSM_CONFIG(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FSM_CONFIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn INDEX_CONFIG(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INDEX_CONFIG(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 19usize)) | (((val as u32) & 0xff) << 19usize);
        }
        #[inline(always)]
        pub const fn Reserved31(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_Reserved31(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for VERSION {
        #[inline(always)]
        fn default() -> VERSION {
            VERSION(0)
        }
    }
}
