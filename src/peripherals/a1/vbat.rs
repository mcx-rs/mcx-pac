#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VBAT {
    ptr: *mut u8,
}
unsafe impl Send for VBAT {}
unsafe impl Sync for VBAT {}
impl VBAT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<regs::VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn FROCTLA(self) -> crate::common::Reg<regs::FROCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn FROLCKA(self) -> crate::common::Reg<regs::FROLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FROCLKE(self) -> crate::common::Reg<regs::FROCLKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKEUP(self, n: usize) -> WAKEUP {
        assert!(n < 2usize);
        unsafe { WAKEUP::from_ptr(self.ptr.add(0x0700usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKLCKA(self) -> crate::common::Reg<regs::WAKLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEUP {
    ptr: *mut u8,
}
unsafe impl Send for WAKEUP {}
unsafe impl Sync for WAKEUP {}
impl WAKEUP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn WAKEUPA(self) -> crate::common::Reg<regs::WAKEUP_WAKEUPA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "FRO16K Clock Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCLKE(pub u32);
    impl FROCLKE {
        #[inline(always)]
        pub const fn CLKE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for FROCLKE {
        #[inline(always)]
        fn default() -> FROCLKE {
            FROCLKE(0)
        }
    }
    #[doc = "FRO16K Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCTLA(pub u32);
    impl FROCTLA {
        #[inline(always)]
        pub const fn FRO_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROCTLA {
        #[inline(always)]
        fn default() -> FROCTLA {
            FROCTLA(0)
        }
    }
    #[doc = "FRO16K Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROLCKA(pub u32);
    impl FROLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROLCKA {
        #[inline(always)]
        fn default() -> FROLCKA {
            FROLCKA(0)
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for VERID {
        #[inline(always)]
        fn default() -> VERID {
            VERID(0)
        }
    }
    #[doc = "Wakeup 0 Register A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKEUP_WAKEUPA(pub u32);
    impl WAKEUP_WAKEUPA {
        #[inline(always)]
        pub const fn REG(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_REG(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WAKEUP_WAKEUPA {
        #[inline(always)]
        fn default() -> WAKEUP_WAKEUPA {
            WAKEUP_WAKEUPA(0)
        }
    }
    #[doc = "Wakeup Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKLCKA(pub u32);
    impl WAKLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WAKLCKA {
        #[inline(always)]
        fn default() -> WAKLCKA {
            WAKLCKA(0)
        }
    }
}
