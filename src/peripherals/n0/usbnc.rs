#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBNC {
    ptr: *mut u8,
}
unsafe impl Send for USBNC {}
unsafe impl Sync for USBNC {}
impl USBNC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CTRL1(self) -> crate::common::Reg<regs::CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<regs::CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn HSIC_CTRL(self) -> crate::common::Reg<regs::HSIC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "USB OTG Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL1(pub u32);
    impl CTRL1 {
        #[inline(always)]
        pub const fn OVER_CUR_DIS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVER_CUR_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn OVER_CUR_POL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVER_CUR_POL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn PWR_POL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWR_POL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn WIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn WKUP_SW_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKUP_SW_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn WKUP_SW(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKUP_SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn WKUP_ID_EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKUP_ID_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WKUP_VBUS_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKUP_VBUS_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn WKUP_DPDM_EN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WKUP_DPDM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn WIR(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL1 {
        #[inline(always)]
        fn default() -> CTRL1 {
            CTRL1(0)
        }
    }
    impl core::fmt::Debug for CTRL1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL1")
                .field("OVER_CUR_DIS", &self.OVER_CUR_DIS())
                .field("OVER_CUR_POL", &self.OVER_CUR_POL())
                .field("PWR_POL", &self.PWR_POL())
                .field("WIE", &self.WIE())
                .field("WKUP_SW_EN", &self.WKUP_SW_EN())
                .field("WKUP_SW", &self.WKUP_SW())
                .field("WKUP_ID_EN", &self.WKUP_ID_EN())
                .field("WKUP_VBUS_EN", &self.WKUP_VBUS_EN())
                .field("WKUP_DPDM_EN", &self.WKUP_DPDM_EN())
                .field("WIR", &self.WIR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL1 {
                OVER_CUR_DIS: bool,
                OVER_CUR_POL: bool,
                PWR_POL: bool,
                WIE: bool,
                WKUP_SW_EN: bool,
                WKUP_SW: bool,
                WKUP_ID_EN: bool,
                WKUP_VBUS_EN: bool,
                WKUP_DPDM_EN: bool,
                WIR: bool,
            }
            let proxy = CTRL1 {
                OVER_CUR_DIS: self.OVER_CUR_DIS(),
                OVER_CUR_POL: self.OVER_CUR_POL(),
                PWR_POL: self.PWR_POL(),
                WIE: self.WIE(),
                WKUP_SW_EN: self.WKUP_SW_EN(),
                WKUP_SW: self.WKUP_SW(),
                WKUP_ID_EN: self.WKUP_ID_EN(),
                WKUP_VBUS_EN: self.WKUP_VBUS_EN(),
                WKUP_DPDM_EN: self.WKUP_DPDM_EN(),
                WIR: self.WIR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB OTG Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL2(pub u32);
    impl CTRL2 {
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn AUTURESUME_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTURESUME_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn LOWSPEED_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOWSPEED_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn UTMI_CLK_VLD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTMI_CLK_VLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL2 {
        #[inline(always)]
        fn default() -> CTRL2 {
            CTRL2(0)
        }
    }
    impl core::fmt::Debug for CTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL2")
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("AUTURESUME_EN", &self.AUTURESUME_EN())
                .field("LOWSPEED_EN", &self.LOWSPEED_EN())
                .field("UTMI_CLK_VLD", &self.UTMI_CLK_VLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL2 {
                VBUS_SOURCE_SEL: u8,
                AUTURESUME_EN: bool,
                LOWSPEED_EN: bool,
                UTMI_CLK_VLD: bool,
            }
            let proxy = CTRL2 {
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                AUTURESUME_EN: self.AUTURESUME_EN(),
                LOWSPEED_EN: self.LOWSPEED_EN(),
                UTMI_CLK_VLD: self.UTMI_CLK_VLD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB Host HSIC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HSIC_CTRL(pub u32);
    impl HSIC_CTRL {
        #[inline(always)]
        pub const fn HSIC_CLK_ON(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSIC_CLK_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn HSIC_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSIC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn CLK_VLD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK_VLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for HSIC_CTRL {
        #[inline(always)]
        fn default() -> HSIC_CTRL {
            HSIC_CTRL(0)
        }
    }
    impl core::fmt::Debug for HSIC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HSIC_CTRL")
                .field("HSIC_CLK_ON", &self.HSIC_CLK_ON())
                .field("HSIC_EN", &self.HSIC_EN())
                .field("CLK_VLD", &self.CLK_VLD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HSIC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HSIC_CTRL {
                HSIC_CLK_ON: bool,
                HSIC_EN: bool,
                CLK_VLD: bool,
            }
            let proxy = HSIC_CTRL {
                HSIC_CLK_ON: self.HSIC_CLK_ON(),
                HSIC_EN: self.HSIC_EN(),
                CLK_VLD: self.CLK_VLD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
