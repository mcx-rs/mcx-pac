#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BSP32 {
    ptr: *mut u8,
}
unsafe impl Send for BSP32 {}
unsafe impl Sync for BSP32 {}
impl BSP32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn OFFSET_PMEM(self) -> crate::common::Reg<regs::OFFSET_PMEM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn OFFSET_XMEM(self) -> crate::common::Reg<regs::OFFSET_XMEM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn OFFSET_YMEM(self) -> crate::common::Reg<regs::OFFSET_YMEM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn OFFSET_MAILBOX(
        self,
    ) -> crate::common::Reg<regs::OFFSET_MAILBOX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn INTERRUPTS_EXTERNAL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INTERRUPTS_STATUS(
        self,
    ) -> crate::common::Reg<regs::INTERRUPTS_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CF_GATING_OVERRIDE(
        self,
    ) -> crate::common::Reg<regs::CF_GATING_OVERRIDE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IVT_OFFSET(self) -> crate::common::Reg<regs::IVT_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn SLEEP_MODE(self) -> crate::common::Reg<regs::SLEEP_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn IVT0(self) -> crate::common::Reg<regs::IVT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn IVT1(self) -> crate::common::Reg<regs::IVT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn IVT2(self) -> crate::common::Reg<regs::IVT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn IVT3(self) -> crate::common::Reg<regs::IVT3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn IVT_DISABLE(self) -> crate::common::Reg<regs::IVT_DISABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "CoolFlux BSP32 gating override"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CF_GATING_OVERRIDE(pub u32);
    impl CF_GATING_OVERRIDE {
        #[inline(always)]
        pub const fn val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CF_GATING_OVERRIDE {
        #[inline(always)]
        fn default() -> CF_GATING_OVERRIDE {
            CF_GATING_OVERRIDE(0)
        }
    }
    impl core::fmt::Debug for CF_GATING_OVERRIDE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CF_GATING_OVERRIDE")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CF_GATING_OVERRIDE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CF_GATING_OVERRIDE {
                val: bool,
            }
            let proxy = CF_GATING_OVERRIDE { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTERRUPTS_STATUS(pub u32);
    impl INTERRUPTS_STATUS {
        #[inline(always)]
        pub const fn val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for INTERRUPTS_STATUS {
        #[inline(always)]
        fn default() -> INTERRUPTS_STATUS {
            INTERRUPTS_STATUS(0)
        }
    }
    impl core::fmt::Debug for INTERRUPTS_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTERRUPTS_STATUS")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTERRUPTS_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INTERRUPTS_STATUS {
                val: bool,
            }
            let proxy = INTERRUPTS_STATUS { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT0(pub u32);
    impl IVT0 {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IVT0 {
        #[inline(always)]
        fn default() -> IVT0 {
            IVT0(0)
        }
    }
    impl core::fmt::Debug for IVT0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT0").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT0 {
                val: u32,
            }
            let proxy = IVT0 { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT1(pub u32);
    impl IVT1 {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IVT1 {
        #[inline(always)]
        fn default() -> IVT1 {
            IVT1(0)
        }
    }
    impl core::fmt::Debug for IVT1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT1").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT1 {
                val: u32,
            }
            let proxy = IVT1 { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT2(pub u32);
    impl IVT2 {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IVT2 {
        #[inline(always)]
        fn default() -> IVT2 {
            IVT2(0)
        }
    }
    impl core::fmt::Debug for IVT2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT2").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT2 {
                val: u32,
            }
            let proxy = IVT2 { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT3(pub u32);
    impl IVT3 {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IVT3 {
        #[inline(always)]
        fn default() -> IVT3 {
            IVT3(0)
        }
    }
    impl core::fmt::Debug for IVT3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT3").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT3 {
                val: u32,
            }
            let proxy = IVT3 { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT disable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT_DISABLE(pub u32);
    impl IVT_DISABLE {
        #[inline(always)]
        pub const fn val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IVT_DISABLE {
        #[inline(always)]
        fn default() -> IVT_DISABLE {
            IVT_DISABLE(0)
        }
    }
    impl core::fmt::Debug for IVT_DISABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT_DISABLE")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT_DISABLE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT_DISABLE {
                val: bool,
            }
            let proxy = IVT_DISABLE { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 IVT offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IVT_OFFSET(pub u32);
    impl IVT_OFFSET {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IVT_OFFSET {
        #[inline(always)]
        fn default() -> IVT_OFFSET {
            IVT_OFFSET(0)
        }
    }
    impl core::fmt::Debug for IVT_OFFSET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IVT_OFFSET")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IVT_OFFSET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IVT_OFFSET {
                val: u32,
            }
            let proxy = IVT_OFFSET { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Offset address register for mailbox peripheral"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFFSET_MAILBOX(pub u32);
    impl OFFSET_MAILBOX {
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for OFFSET_MAILBOX {
        #[inline(always)]
        fn default() -> OFFSET_MAILBOX {
            OFFSET_MAILBOX(0)
        }
    }
    impl core::fmt::Debug for OFFSET_MAILBOX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OFFSET_MAILBOX")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OFFSET_MAILBOX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OFFSET_MAILBOX {
                val: u32,
            }
            let proxy = OFFSET_MAILBOX { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Offset address register for program memory"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFFSET_PMEM(pub u32);
    impl OFFSET_PMEM {
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for OFFSET_PMEM {
        #[inline(always)]
        fn default() -> OFFSET_PMEM {
            OFFSET_PMEM(0)
        }
    }
    impl core::fmt::Debug for OFFSET_PMEM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OFFSET_PMEM")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OFFSET_PMEM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OFFSET_PMEM {
                val: u8,
            }
            let proxy = OFFSET_PMEM { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Offset address register for X-data memory"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFFSET_XMEM(pub u32);
    impl OFFSET_XMEM {
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for OFFSET_XMEM {
        #[inline(always)]
        fn default() -> OFFSET_XMEM {
            OFFSET_XMEM(0)
        }
    }
    impl core::fmt::Debug for OFFSET_XMEM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OFFSET_XMEM")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OFFSET_XMEM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OFFSET_XMEM {
                val: u8,
            }
            let proxy = OFFSET_XMEM { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Offset address register for Y-data memory"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFFSET_YMEM(pub u32);
    impl OFFSET_YMEM {
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for OFFSET_YMEM {
        #[inline(always)]
        fn default() -> OFFSET_YMEM {
            OFFSET_YMEM(0)
        }
    }
    impl core::fmt::Debug for OFFSET_YMEM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OFFSET_YMEM")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OFFSET_YMEM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OFFSET_YMEM {
                val: u8,
            }
            let proxy = OFFSET_YMEM { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CoolFlux BSP32 sleep mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SLEEP_MODE(pub u32);
    impl SLEEP_MODE {
        #[inline(always)]
        pub const fn val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SLEEP_MODE {
        #[inline(always)]
        fn default() -> SLEEP_MODE {
            SLEEP_MODE(0)
        }
    }
    impl core::fmt::Debug for SLEEP_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SLEEP_MODE")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SLEEP_MODE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SLEEP_MODE {
                val: bool,
            }
            let proxy = SLEEP_MODE { val: self.val() };
            defmt::write!(f, "{}", proxy)
        }
    }
}
