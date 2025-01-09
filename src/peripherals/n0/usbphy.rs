#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBPHY {
    ptr: *mut u8,
}
unsafe impl Send for USBPHY {}
unsafe impl Sync for USBPHY {}
impl USBPHY {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn PWD(self) -> crate::common::Reg<regs::PWD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PWD_SET(self) -> crate::common::Reg<regs::PWD_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn PWD_CLR(self) -> crate::common::Reg<regs::PWD_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PWD_TOG(self) -> crate::common::Reg<regs::PWD_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn TX(self) -> crate::common::Reg<regs::TX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_SET(self) -> crate::common::Reg<regs::TX_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_CLR(self) -> crate::common::Reg<regs::TX_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn TX_TOG(self) -> crate::common::Reg<regs::TX_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn RX(self) -> crate::common::Reg<regs::RX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn RX_SET(self) -> crate::common::Reg<regs::RX_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn RX_CLR(self) -> crate::common::Reg<regs::RX_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn RX_TOG(self) -> crate::common::Reg<regs::RX_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_SET(self) -> crate::common::Reg<regs::CTRL_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_CLR(self) -> crate::common::Reg<regs::CTRL_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL_TOG(self) -> crate::common::Reg<regs::CTRL_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG0(self) -> crate::common::Reg<regs::DEBUG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG0_SET(self) -> crate::common::Reg<regs::DEBUG0_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG0_CLR(self) -> crate::common::Reg<regs::DEBUG0_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG0_TOG(self) -> crate::common::Reg<regs::DEBUG0_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn VERSION(self) -> crate::common::Reg<regs::VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn IP(self) -> crate::common::Reg<regs::IP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn IP_SET(self) -> crate::common::Reg<regs::IP_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn IP_CLR(self) -> crate::common::Reg<regs::IP_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn IP_TOG(self) -> crate::common::Reg<regs::IP_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn PLL_SIC(self) -> crate::common::Reg<regs::PLL_SIC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn PLL_SIC_SET(self) -> crate::common::Reg<regs::PLL_SIC_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn PLL_SIC_CLR(self) -> crate::common::Reg<regs::PLL_SIC_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn PLL_SIC_TOG(self) -> crate::common::Reg<regs::PLL_SIC_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_SET(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_CLR(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_TOG(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DET_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_SET(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DET_STAT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_CLR(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DET_STAT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_TOG(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DET_STAT_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DETECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_SET(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DETECT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_CLR(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DETECT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_TOG(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DETECT_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DET_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_SET(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DET_STAT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_CLR(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DET_STAT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_TOG(
        self,
    ) -> crate::common::Reg<regs::USB1_CHRG_DET_STAT_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn ANACTRL(self) -> crate::common::Reg<regs::ANACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn ANACTRL_SET(self) -> crate::common::Reg<regs::ANACTRL_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn ANACTRL_CLR(self) -> crate::common::Reg<regs::ANACTRL_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn ANACTRL_TOG(self) -> crate::common::Reg<regs::ANACTRL_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN(
        self,
    ) -> crate::common::Reg<regs::TRIM_OVERRIDE_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_SET(
        self,
    ) -> crate::common::Reg<regs::TRIM_OVERRIDE_EN_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_CLR(
        self,
    ) -> crate::common::Reg<regs::TRIM_OVERRIDE_EN_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_TOG(
        self,
    ) -> crate::common::Reg<regs::TRIM_OVERRIDE_EN_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[inline(always)]
    pub const fn PFDA(self) -> crate::common::Reg<regs::PFDA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn PFDA_SET(self) -> crate::common::Reg<regs::PFDA_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn PFDA_CLR(self) -> crate::common::Reg<regs::PFDA_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn PFDA_TOG(self) -> crate::common::Reg<regs::PFDA_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Analog Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ANACTRL(pub u32);
    impl ANACTRL {
        #[inline(always)]
        pub const fn LVI_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVI_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_PULLDOWN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_PULLDOWN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for ANACTRL {
        #[inline(always)]
        fn default() -> ANACTRL {
            ANACTRL(0)
        }
    }
    impl core::fmt::Debug for ANACTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ANACTRL")
                .field("LVI_EN", &self.LVI_EN())
                .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
                .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ANACTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ANACTRL {
                LVI_EN: bool,
                PFD_CLK_SEL: u8,
                DEV_PULLDOWN: bool,
            }
            let proxy = ANACTRL {
                LVI_EN: self.LVI_EN(),
                PFD_CLK_SEL: self.PFD_CLK_SEL(),
                DEV_PULLDOWN: self.DEV_PULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Analog Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ANACTRL_CLR(pub u32);
    impl ANACTRL_CLR {
        #[inline(always)]
        pub const fn LVI_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVI_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_PULLDOWN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_PULLDOWN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for ANACTRL_CLR {
        #[inline(always)]
        fn default() -> ANACTRL_CLR {
            ANACTRL_CLR(0)
        }
    }
    impl core::fmt::Debug for ANACTRL_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ANACTRL_CLR")
                .field("LVI_EN", &self.LVI_EN())
                .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
                .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ANACTRL_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ANACTRL_CLR {
                LVI_EN: bool,
                PFD_CLK_SEL: u8,
                DEV_PULLDOWN: bool,
            }
            let proxy = ANACTRL_CLR {
                LVI_EN: self.LVI_EN(),
                PFD_CLK_SEL: self.PFD_CLK_SEL(),
                DEV_PULLDOWN: self.DEV_PULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Analog Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ANACTRL_SET(pub u32);
    impl ANACTRL_SET {
        #[inline(always)]
        pub const fn LVI_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVI_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_PULLDOWN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_PULLDOWN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for ANACTRL_SET {
        #[inline(always)]
        fn default() -> ANACTRL_SET {
            ANACTRL_SET(0)
        }
    }
    impl core::fmt::Debug for ANACTRL_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ANACTRL_SET")
                .field("LVI_EN", &self.LVI_EN())
                .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
                .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ANACTRL_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ANACTRL_SET {
                LVI_EN: bool,
                PFD_CLK_SEL: u8,
                DEV_PULLDOWN: bool,
            }
            let proxy = ANACTRL_SET {
                LVI_EN: self.LVI_EN(),
                PFD_CLK_SEL: self.PFD_CLK_SEL(),
                DEV_PULLDOWN: self.DEV_PULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Analog Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ANACTRL_TOG(pub u32);
    impl ANACTRL_TOG {
        #[inline(always)]
        pub const fn LVI_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVI_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD_CLK_SEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD_CLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_PULLDOWN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_PULLDOWN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for ANACTRL_TOG {
        #[inline(always)]
        fn default() -> ANACTRL_TOG {
            ANACTRL_TOG(0)
        }
    }
    impl core::fmt::Debug for ANACTRL_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ANACTRL_TOG")
                .field("LVI_EN", &self.LVI_EN())
                .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
                .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ANACTRL_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ANACTRL_TOG {
                LVI_EN: bool,
                PFD_CLK_SEL: u8,
                DEV_PULLDOWN: bool,
            }
            let proxy = ANACTRL_TOG {
                LVI_EN: self.LVI_EN(),
                PFD_CLK_SEL: self.PFD_CLK_SEL(),
                DEV_PULLDOWN: self.DEV_PULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "General Purpose Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENHOSTDISCONDETECT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ENIRQHOSTDISCON(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENDEVPLUGINDETECT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ENOTGIDDETECT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTGIDDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RESUMEIRQSTICKY(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ENIRQRESUMEDETECT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RESUME_IRQ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ENIRQDEVPLUGIN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_IRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DATA_ON_LRADC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATA_ON_LRADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ENIRQWAKEUP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQWAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_IRQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn AUTORESUME_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTORESUME_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_VALUE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_VALUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn UTMI_SUSPENDM(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTMI_SUSPENDM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn CLKGATE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SFTRST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
                .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
                .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
                .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
                .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
                .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
                .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
                .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
                .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
                .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
                .field("RESUME_IRQ", &self.RESUME_IRQ())
                .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
                .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
                .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
                .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
                .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
                .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
                .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
                .field("AUTORESUME_EN", &self.AUTORESUME_EN())
                .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
                .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
                .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
                .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
                .field("CLKGATE", &self.CLKGATE())
                .field("SFTRST", &self.SFTRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL {
                ENOTG_ID_CHG_IRQ: bool,
                ENHOSTDISCONDETECT: bool,
                ENIRQHOSTDISCON: bool,
                HOSTDISCONDETECT_IRQ: bool,
                ENDEVPLUGINDETECT: bool,
                DEVPLUGIN_POLARITY: bool,
                OTG_ID_CHG_IRQ: bool,
                ENOTGIDDETECT: bool,
                RESUMEIRQSTICKY: bool,
                ENIRQRESUMEDETECT: bool,
                RESUME_IRQ: bool,
                ENIRQDEVPLUGIN: bool,
                DEVPLUGIN_IRQ: bool,
                DATA_ON_LRADC: bool,
                ENUTMILEVEL2: bool,
                ENUTMILEVEL3: bool,
                ENIRQWAKEUP: bool,
                WAKEUP_IRQ: bool,
                AUTORESUME_EN: bool,
                ENAUTOCLR_CLKGATE: bool,
                ENAUTOCLR_PHY_PWD: bool,
                OTG_ID_VALUE: bool,
                UTMI_SUSPENDM: bool,
                CLKGATE: bool,
                SFTRST: bool,
            }
            let proxy = CTRL {
                ENOTG_ID_CHG_IRQ: self.ENOTG_ID_CHG_IRQ(),
                ENHOSTDISCONDETECT: self.ENHOSTDISCONDETECT(),
                ENIRQHOSTDISCON: self.ENIRQHOSTDISCON(),
                HOSTDISCONDETECT_IRQ: self.HOSTDISCONDETECT_IRQ(),
                ENDEVPLUGINDETECT: self.ENDEVPLUGINDETECT(),
                DEVPLUGIN_POLARITY: self.DEVPLUGIN_POLARITY(),
                OTG_ID_CHG_IRQ: self.OTG_ID_CHG_IRQ(),
                ENOTGIDDETECT: self.ENOTGIDDETECT(),
                RESUMEIRQSTICKY: self.RESUMEIRQSTICKY(),
                ENIRQRESUMEDETECT: self.ENIRQRESUMEDETECT(),
                RESUME_IRQ: self.RESUME_IRQ(),
                ENIRQDEVPLUGIN: self.ENIRQDEVPLUGIN(),
                DEVPLUGIN_IRQ: self.DEVPLUGIN_IRQ(),
                DATA_ON_LRADC: self.DATA_ON_LRADC(),
                ENUTMILEVEL2: self.ENUTMILEVEL2(),
                ENUTMILEVEL3: self.ENUTMILEVEL3(),
                ENIRQWAKEUP: self.ENIRQWAKEUP(),
                WAKEUP_IRQ: self.WAKEUP_IRQ(),
                AUTORESUME_EN: self.AUTORESUME_EN(),
                ENAUTOCLR_CLKGATE: self.ENAUTOCLR_CLKGATE(),
                ENAUTOCLR_PHY_PWD: self.ENAUTOCLR_PHY_PWD(),
                OTG_ID_VALUE: self.OTG_ID_VALUE(),
                UTMI_SUSPENDM: self.UTMI_SUSPENDM(),
                CLKGATE: self.CLKGATE(),
                SFTRST: self.SFTRST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "General Purpose Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_CLR(pub u32);
    impl CTRL_CLR {
        #[inline(always)]
        pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENHOSTDISCONDETECT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ENIRQHOSTDISCON(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENDEVPLUGINDETECT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ENOTGIDDETECT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTGIDDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RESUMEIRQSTICKY(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ENIRQRESUMEDETECT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RESUME_IRQ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ENIRQDEVPLUGIN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_IRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DATA_ON_LRADC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATA_ON_LRADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ENIRQWAKEUP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQWAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_IRQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn AUTORESUME_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTORESUME_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_VALUE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_VALUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn UTMI_SUSPENDM(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTMI_SUSPENDM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn CLKGATE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SFTRST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_CLR {
        #[inline(always)]
        fn default() -> CTRL_CLR {
            CTRL_CLR(0)
        }
    }
    impl core::fmt::Debug for CTRL_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_CLR")
                .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
                .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
                .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
                .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
                .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
                .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
                .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
                .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
                .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
                .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
                .field("RESUME_IRQ", &self.RESUME_IRQ())
                .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
                .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
                .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
                .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
                .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
                .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
                .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
                .field("AUTORESUME_EN", &self.AUTORESUME_EN())
                .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
                .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
                .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
                .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
                .field("CLKGATE", &self.CLKGATE())
                .field("SFTRST", &self.SFTRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_CLR {
                ENOTG_ID_CHG_IRQ: bool,
                ENHOSTDISCONDETECT: bool,
                ENIRQHOSTDISCON: bool,
                HOSTDISCONDETECT_IRQ: bool,
                ENDEVPLUGINDETECT: bool,
                DEVPLUGIN_POLARITY: bool,
                OTG_ID_CHG_IRQ: bool,
                ENOTGIDDETECT: bool,
                RESUMEIRQSTICKY: bool,
                ENIRQRESUMEDETECT: bool,
                RESUME_IRQ: bool,
                ENIRQDEVPLUGIN: bool,
                DEVPLUGIN_IRQ: bool,
                DATA_ON_LRADC: bool,
                ENUTMILEVEL2: bool,
                ENUTMILEVEL3: bool,
                ENIRQWAKEUP: bool,
                WAKEUP_IRQ: bool,
                AUTORESUME_EN: bool,
                ENAUTOCLR_CLKGATE: bool,
                ENAUTOCLR_PHY_PWD: bool,
                OTG_ID_VALUE: bool,
                UTMI_SUSPENDM: bool,
                CLKGATE: bool,
                SFTRST: bool,
            }
            let proxy = CTRL_CLR {
                ENOTG_ID_CHG_IRQ: self.ENOTG_ID_CHG_IRQ(),
                ENHOSTDISCONDETECT: self.ENHOSTDISCONDETECT(),
                ENIRQHOSTDISCON: self.ENIRQHOSTDISCON(),
                HOSTDISCONDETECT_IRQ: self.HOSTDISCONDETECT_IRQ(),
                ENDEVPLUGINDETECT: self.ENDEVPLUGINDETECT(),
                DEVPLUGIN_POLARITY: self.DEVPLUGIN_POLARITY(),
                OTG_ID_CHG_IRQ: self.OTG_ID_CHG_IRQ(),
                ENOTGIDDETECT: self.ENOTGIDDETECT(),
                RESUMEIRQSTICKY: self.RESUMEIRQSTICKY(),
                ENIRQRESUMEDETECT: self.ENIRQRESUMEDETECT(),
                RESUME_IRQ: self.RESUME_IRQ(),
                ENIRQDEVPLUGIN: self.ENIRQDEVPLUGIN(),
                DEVPLUGIN_IRQ: self.DEVPLUGIN_IRQ(),
                DATA_ON_LRADC: self.DATA_ON_LRADC(),
                ENUTMILEVEL2: self.ENUTMILEVEL2(),
                ENUTMILEVEL3: self.ENUTMILEVEL3(),
                ENIRQWAKEUP: self.ENIRQWAKEUP(),
                WAKEUP_IRQ: self.WAKEUP_IRQ(),
                AUTORESUME_EN: self.AUTORESUME_EN(),
                ENAUTOCLR_CLKGATE: self.ENAUTOCLR_CLKGATE(),
                ENAUTOCLR_PHY_PWD: self.ENAUTOCLR_PHY_PWD(),
                OTG_ID_VALUE: self.OTG_ID_VALUE(),
                UTMI_SUSPENDM: self.UTMI_SUSPENDM(),
                CLKGATE: self.CLKGATE(),
                SFTRST: self.SFTRST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "General Purpose Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_SET(pub u32);
    impl CTRL_SET {
        #[inline(always)]
        pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENHOSTDISCONDETECT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ENIRQHOSTDISCON(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENDEVPLUGINDETECT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ENOTGIDDETECT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTGIDDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RESUMEIRQSTICKY(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ENIRQRESUMEDETECT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RESUME_IRQ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ENIRQDEVPLUGIN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_IRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DATA_ON_LRADC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATA_ON_LRADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ENIRQWAKEUP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQWAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_IRQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn AUTORESUME_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTORESUME_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_VALUE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_VALUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn UTMI_SUSPENDM(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTMI_SUSPENDM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn CLKGATE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SFTRST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_SET {
        #[inline(always)]
        fn default() -> CTRL_SET {
            CTRL_SET(0)
        }
    }
    impl core::fmt::Debug for CTRL_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_SET")
                .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
                .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
                .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
                .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
                .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
                .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
                .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
                .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
                .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
                .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
                .field("RESUME_IRQ", &self.RESUME_IRQ())
                .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
                .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
                .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
                .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
                .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
                .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
                .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
                .field("AUTORESUME_EN", &self.AUTORESUME_EN())
                .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
                .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
                .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
                .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
                .field("CLKGATE", &self.CLKGATE())
                .field("SFTRST", &self.SFTRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_SET {
                ENOTG_ID_CHG_IRQ: bool,
                ENHOSTDISCONDETECT: bool,
                ENIRQHOSTDISCON: bool,
                HOSTDISCONDETECT_IRQ: bool,
                ENDEVPLUGINDETECT: bool,
                DEVPLUGIN_POLARITY: bool,
                OTG_ID_CHG_IRQ: bool,
                ENOTGIDDETECT: bool,
                RESUMEIRQSTICKY: bool,
                ENIRQRESUMEDETECT: bool,
                RESUME_IRQ: bool,
                ENIRQDEVPLUGIN: bool,
                DEVPLUGIN_IRQ: bool,
                DATA_ON_LRADC: bool,
                ENUTMILEVEL2: bool,
                ENUTMILEVEL3: bool,
                ENIRQWAKEUP: bool,
                WAKEUP_IRQ: bool,
                AUTORESUME_EN: bool,
                ENAUTOCLR_CLKGATE: bool,
                ENAUTOCLR_PHY_PWD: bool,
                OTG_ID_VALUE: bool,
                UTMI_SUSPENDM: bool,
                CLKGATE: bool,
                SFTRST: bool,
            }
            let proxy = CTRL_SET {
                ENOTG_ID_CHG_IRQ: self.ENOTG_ID_CHG_IRQ(),
                ENHOSTDISCONDETECT: self.ENHOSTDISCONDETECT(),
                ENIRQHOSTDISCON: self.ENIRQHOSTDISCON(),
                HOSTDISCONDETECT_IRQ: self.HOSTDISCONDETECT_IRQ(),
                ENDEVPLUGINDETECT: self.ENDEVPLUGINDETECT(),
                DEVPLUGIN_POLARITY: self.DEVPLUGIN_POLARITY(),
                OTG_ID_CHG_IRQ: self.OTG_ID_CHG_IRQ(),
                ENOTGIDDETECT: self.ENOTGIDDETECT(),
                RESUMEIRQSTICKY: self.RESUMEIRQSTICKY(),
                ENIRQRESUMEDETECT: self.ENIRQRESUMEDETECT(),
                RESUME_IRQ: self.RESUME_IRQ(),
                ENIRQDEVPLUGIN: self.ENIRQDEVPLUGIN(),
                DEVPLUGIN_IRQ: self.DEVPLUGIN_IRQ(),
                DATA_ON_LRADC: self.DATA_ON_LRADC(),
                ENUTMILEVEL2: self.ENUTMILEVEL2(),
                ENUTMILEVEL3: self.ENUTMILEVEL3(),
                ENIRQWAKEUP: self.ENIRQWAKEUP(),
                WAKEUP_IRQ: self.WAKEUP_IRQ(),
                AUTORESUME_EN: self.AUTORESUME_EN(),
                ENAUTOCLR_CLKGATE: self.ENAUTOCLR_CLKGATE(),
                ENAUTOCLR_PHY_PWD: self.ENAUTOCLR_PHY_PWD(),
                OTG_ID_VALUE: self.OTG_ID_VALUE(),
                UTMI_SUSPENDM: self.UTMI_SUSPENDM(),
                CLKGATE: self.CLKGATE(),
                SFTRST: self.SFTRST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "General Purpose Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_TOG(pub u32);
    impl CTRL_TOG {
        #[inline(always)]
        pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENHOSTDISCONDETECT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ENIRQHOSTDISCON(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENDEVPLUGINDETECT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ENOTGIDDETECT(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENOTGIDDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn RESUMEIRQSTICKY(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ENIRQRESUMEDETECT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RESUME_IRQ(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ENIRQDEVPLUGIN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_IRQ(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DATA_ON_LRADC(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATA_ON_LRADC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ENUTMILEVEL3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENUTMILEVEL3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ENIRQWAKEUP(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENIRQWAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_IRQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn AUTORESUME_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTORESUME_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn OTG_ID_VALUE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTG_ID_VALUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn UTMI_SUSPENDM(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UTMI_SUSPENDM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn CLKGATE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SFTRST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SFTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CTRL_TOG {
        #[inline(always)]
        fn default() -> CTRL_TOG {
            CTRL_TOG(0)
        }
    }
    impl core::fmt::Debug for CTRL_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_TOG")
                .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
                .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
                .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
                .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
                .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
                .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
                .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
                .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
                .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
                .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
                .field("RESUME_IRQ", &self.RESUME_IRQ())
                .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
                .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
                .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
                .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
                .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
                .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
                .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
                .field("AUTORESUME_EN", &self.AUTORESUME_EN())
                .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
                .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
                .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
                .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
                .field("CLKGATE", &self.CLKGATE())
                .field("SFTRST", &self.SFTRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_TOG {
                ENOTG_ID_CHG_IRQ: bool,
                ENHOSTDISCONDETECT: bool,
                ENIRQHOSTDISCON: bool,
                HOSTDISCONDETECT_IRQ: bool,
                ENDEVPLUGINDETECT: bool,
                DEVPLUGIN_POLARITY: bool,
                OTG_ID_CHG_IRQ: bool,
                ENOTGIDDETECT: bool,
                RESUMEIRQSTICKY: bool,
                ENIRQRESUMEDETECT: bool,
                RESUME_IRQ: bool,
                ENIRQDEVPLUGIN: bool,
                DEVPLUGIN_IRQ: bool,
                DATA_ON_LRADC: bool,
                ENUTMILEVEL2: bool,
                ENUTMILEVEL3: bool,
                ENIRQWAKEUP: bool,
                WAKEUP_IRQ: bool,
                AUTORESUME_EN: bool,
                ENAUTOCLR_CLKGATE: bool,
                ENAUTOCLR_PHY_PWD: bool,
                OTG_ID_VALUE: bool,
                UTMI_SUSPENDM: bool,
                CLKGATE: bool,
                SFTRST: bool,
            }
            let proxy = CTRL_TOG {
                ENOTG_ID_CHG_IRQ: self.ENOTG_ID_CHG_IRQ(),
                ENHOSTDISCONDETECT: self.ENHOSTDISCONDETECT(),
                ENIRQHOSTDISCON: self.ENIRQHOSTDISCON(),
                HOSTDISCONDETECT_IRQ: self.HOSTDISCONDETECT_IRQ(),
                ENDEVPLUGINDETECT: self.ENDEVPLUGINDETECT(),
                DEVPLUGIN_POLARITY: self.DEVPLUGIN_POLARITY(),
                OTG_ID_CHG_IRQ: self.OTG_ID_CHG_IRQ(),
                ENOTGIDDETECT: self.ENOTGIDDETECT(),
                RESUMEIRQSTICKY: self.RESUMEIRQSTICKY(),
                ENIRQRESUMEDETECT: self.ENIRQRESUMEDETECT(),
                RESUME_IRQ: self.RESUME_IRQ(),
                ENIRQDEVPLUGIN: self.ENIRQDEVPLUGIN(),
                DEVPLUGIN_IRQ: self.DEVPLUGIN_IRQ(),
                DATA_ON_LRADC: self.DATA_ON_LRADC(),
                ENUTMILEVEL2: self.ENUTMILEVEL2(),
                ENUTMILEVEL3: self.ENUTMILEVEL3(),
                ENIRQWAKEUP: self.ENIRQWAKEUP(),
                WAKEUP_IRQ: self.WAKEUP_IRQ(),
                AUTORESUME_EN: self.AUTORESUME_EN(),
                ENAUTOCLR_CLKGATE: self.ENAUTOCLR_CLKGATE(),
                ENAUTOCLR_PHY_PWD: self.ENAUTOCLR_PHY_PWD(),
                OTG_ID_VALUE: self.OTG_ID_VALUE(),
                UTMI_SUSPENDM: self.UTMI_SUSPENDM(),
                CLKGATE: self.CLKGATE(),
                SFTRST: self.SFTRST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Debug 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG0(pub u32);
    impl DEBUG0 {
        #[inline(always)]
        pub const fn OTGIDPIOLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTGIDPIOLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_HSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn ENHSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENHSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for DEBUG0 {
        #[inline(always)]
        fn default() -> DEBUG0 {
            DEBUG0(0)
        }
    }
    impl core::fmt::Debug for DEBUG0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG0")
                .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
                .field("HSTPULLDOWN", &self.HSTPULLDOWN())
                .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG0 {
                OTGIDPIOLOCK: bool,
                HSTPULLDOWN: u8,
                ENHSTPULLDOWN: u8,
            }
            let proxy = DEBUG0 {
                OTGIDPIOLOCK: self.OTGIDPIOLOCK(),
                HSTPULLDOWN: self.HSTPULLDOWN(),
                ENHSTPULLDOWN: self.ENHSTPULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Debug 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG0_CLR(pub u32);
    impl DEBUG0_CLR {
        #[inline(always)]
        pub const fn OTGIDPIOLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTGIDPIOLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_HSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn ENHSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENHSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for DEBUG0_CLR {
        #[inline(always)]
        fn default() -> DEBUG0_CLR {
            DEBUG0_CLR(0)
        }
    }
    impl core::fmt::Debug for DEBUG0_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG0_CLR")
                .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
                .field("HSTPULLDOWN", &self.HSTPULLDOWN())
                .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG0_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG0_CLR {
                OTGIDPIOLOCK: bool,
                HSTPULLDOWN: u8,
                ENHSTPULLDOWN: u8,
            }
            let proxy = DEBUG0_CLR {
                OTGIDPIOLOCK: self.OTGIDPIOLOCK(),
                HSTPULLDOWN: self.HSTPULLDOWN(),
                ENHSTPULLDOWN: self.ENHSTPULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Debug 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG0_SET(pub u32);
    impl DEBUG0_SET {
        #[inline(always)]
        pub const fn OTGIDPIOLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTGIDPIOLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_HSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn ENHSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENHSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for DEBUG0_SET {
        #[inline(always)]
        fn default() -> DEBUG0_SET {
            DEBUG0_SET(0)
        }
    }
    impl core::fmt::Debug for DEBUG0_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG0_SET")
                .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
                .field("HSTPULLDOWN", &self.HSTPULLDOWN())
                .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG0_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG0_SET {
                OTGIDPIOLOCK: bool,
                HSTPULLDOWN: u8,
                ENHSTPULLDOWN: u8,
            }
            let proxy = DEBUG0_SET {
                OTGIDPIOLOCK: self.OTGIDPIOLOCK(),
                HSTPULLDOWN: self.HSTPULLDOWN(),
                ENHSTPULLDOWN: self.ENHSTPULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Debug 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG0_TOG(pub u32);
    impl DEBUG0_TOG {
        #[inline(always)]
        pub const fn OTGIDPIOLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTGIDPIOLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_HSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn ENHSTPULLDOWN(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENHSTPULLDOWN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for DEBUG0_TOG {
        #[inline(always)]
        fn default() -> DEBUG0_TOG {
            DEBUG0_TOG(0)
        }
    }
    impl core::fmt::Debug for DEBUG0_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG0_TOG")
                .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
                .field("HSTPULLDOWN", &self.HSTPULLDOWN())
                .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG0_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DEBUG0_TOG {
                OTGIDPIOLOCK: bool,
                HSTPULLDOWN: u8,
                ENHSTPULLDOWN: u8,
            }
            let proxy = DEBUG0_TOG {
                OTGIDPIOLOCK: self.OTGIDPIOLOCK(),
                HSTPULLDOWN: self.HSTPULLDOWN(),
                ENHSTPULLDOWN: self.ENHSTPULLDOWN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "IP Block"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IP(pub u32);
    impl IP {
        #[inline(always)]
        pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IP {
        #[inline(always)]
        fn default() -> IP {
            IP(0)
        }
    }
    impl core::fmt::Debug for IP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IP")
                .field(
                    "POWER_CONTROL_SUSPEND_OPTION",
                    &self.POWER_CONTROL_SUSPEND_OPTION(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IP {
                POWER_CONTROL_SUSPEND_OPTION: bool,
            }
            let proxy = IP {
                POWER_CONTROL_SUSPEND_OPTION: self.POWER_CONTROL_SUSPEND_OPTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "IP Block"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IP_CLR(pub u32);
    impl IP_CLR {
        #[inline(always)]
        pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IP_CLR {
        #[inline(always)]
        fn default() -> IP_CLR {
            IP_CLR(0)
        }
    }
    impl core::fmt::Debug for IP_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IP_CLR")
                .field(
                    "POWER_CONTROL_SUSPEND_OPTION",
                    &self.POWER_CONTROL_SUSPEND_OPTION(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IP_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IP_CLR {
                POWER_CONTROL_SUSPEND_OPTION: bool,
            }
            let proxy = IP_CLR {
                POWER_CONTROL_SUSPEND_OPTION: self.POWER_CONTROL_SUSPEND_OPTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "IP Block"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IP_SET(pub u32);
    impl IP_SET {
        #[inline(always)]
        pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IP_SET {
        #[inline(always)]
        fn default() -> IP_SET {
            IP_SET(0)
        }
    }
    impl core::fmt::Debug for IP_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IP_SET")
                .field(
                    "POWER_CONTROL_SUSPEND_OPTION",
                    &self.POWER_CONTROL_SUSPEND_OPTION(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IP_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IP_SET {
                POWER_CONTROL_SUSPEND_OPTION: bool,
            }
            let proxy = IP_SET {
                POWER_CONTROL_SUSPEND_OPTION: self.POWER_CONTROL_SUSPEND_OPTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "IP Block"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IP_TOG(pub u32);
    impl IP_TOG {
        #[inline(always)]
        pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IP_TOG {
        #[inline(always)]
        fn default() -> IP_TOG {
            IP_TOG(0)
        }
    }
    impl core::fmt::Debug for IP_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IP_TOG")
                .field(
                    "POWER_CONTROL_SUSPEND_OPTION",
                    &self.POWER_CONTROL_SUSPEND_OPTION(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IP_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IP_TOG {
                POWER_CONTROL_SUSPEND_OPTION: bool,
            }
            let proxy = IP_TOG {
                POWER_CONTROL_SUSPEND_OPTION: self.POWER_CONTROL_SUSPEND_OPTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PFD A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PFDA(pub u32);
    impl PFDA {
        #[inline(always)]
        pub const fn PFD0_CLKGATE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFD0_FRAC(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD0_FRAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD0_STABLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_STABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for PFDA {
        #[inline(always)]
        fn default() -> PFDA {
            PFDA(0)
        }
    }
    impl core::fmt::Debug for PFDA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PFDA")
                .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
                .field("PFD0_FRAC", &self.PFD0_FRAC())
                .field("PFD0_STABLE", &self.PFD0_STABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PFDA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PFDA {
                PFD0_CLKGATE: bool,
                PFD0_FRAC: u8,
                PFD0_STABLE: bool,
            }
            let proxy = PFDA {
                PFD0_CLKGATE: self.PFD0_CLKGATE(),
                PFD0_FRAC: self.PFD0_FRAC(),
                PFD0_STABLE: self.PFD0_STABLE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PFD A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PFDA_CLR(pub u32);
    impl PFDA_CLR {
        #[inline(always)]
        pub const fn PFD0_CLKGATE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFD0_FRAC(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD0_FRAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD0_STABLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_STABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for PFDA_CLR {
        #[inline(always)]
        fn default() -> PFDA_CLR {
            PFDA_CLR(0)
        }
    }
    impl core::fmt::Debug for PFDA_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PFDA_CLR")
                .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
                .field("PFD0_FRAC", &self.PFD0_FRAC())
                .field("PFD0_STABLE", &self.PFD0_STABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PFDA_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PFDA_CLR {
                PFD0_CLKGATE: bool,
                PFD0_FRAC: u8,
                PFD0_STABLE: bool,
            }
            let proxy = PFDA_CLR {
                PFD0_CLKGATE: self.PFD0_CLKGATE(),
                PFD0_FRAC: self.PFD0_FRAC(),
                PFD0_STABLE: self.PFD0_STABLE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PFD A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PFDA_SET(pub u32);
    impl PFDA_SET {
        #[inline(always)]
        pub const fn PFD0_CLKGATE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFD0_FRAC(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD0_FRAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD0_STABLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_STABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for PFDA_SET {
        #[inline(always)]
        fn default() -> PFDA_SET {
            PFDA_SET(0)
        }
    }
    impl core::fmt::Debug for PFDA_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PFDA_SET")
                .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
                .field("PFD0_FRAC", &self.PFD0_FRAC())
                .field("PFD0_STABLE", &self.PFD0_STABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PFDA_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PFDA_SET {
                PFD0_CLKGATE: bool,
                PFD0_FRAC: u8,
                PFD0_STABLE: bool,
            }
            let proxy = PFDA_SET {
                PFD0_CLKGATE: self.PFD0_CLKGATE(),
                PFD0_FRAC: self.PFD0_FRAC(),
                PFD0_STABLE: self.PFD0_STABLE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PFD A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PFDA_TOG(pub u32);
    impl PFDA_TOG {
        #[inline(always)]
        pub const fn PFD0_CLKGATE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_CLKGATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PFD0_FRAC(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PFD0_FRAC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[inline(always)]
        pub const fn PFD0_STABLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PFD0_STABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for PFDA_TOG {
        #[inline(always)]
        fn default() -> PFDA_TOG {
            PFDA_TOG(0)
        }
    }
    impl core::fmt::Debug for PFDA_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PFDA_TOG")
                .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
                .field("PFD0_FRAC", &self.PFD0_FRAC())
                .field("PFD0_STABLE", &self.PFD0_STABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PFDA_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PFDA_TOG {
                PFD0_CLKGATE: bool,
                PFD0_FRAC: u8,
                PFD0_STABLE: bool,
            }
            let proxy = PFDA_TOG {
                PFD0_CLKGATE: self.PFD0_CLKGATE(),
                PFD0_FRAC: self.PFD0_FRAC(),
                PFD0_STABLE: self.PFD0_STABLE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL SIC"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL_SIC(pub u32);
    impl PLL_SIC {
        #[inline(always)]
        pub const fn MISC2_CONTROL0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MISC2_CONTROL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PLL_EN_USB_CLKS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PLL_POWER(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_POWER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLL_ENABLE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PLL_BYPASS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD_SEL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn PLL_REG_ENABLE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_REG_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn PLL_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[inline(always)]
        pub const fn PLL_LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PLL_SIC {
        #[inline(always)]
        fn default() -> PLL_SIC {
            PLL_SIC(0)
        }
    }
    impl core::fmt::Debug for PLL_SIC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL_SIC")
                .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
                .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
                .field("PLL_POWER", &self.PLL_POWER())
                .field("PLL_ENABLE", &self.PLL_ENABLE())
                .field("PLL_BYPASS", &self.PLL_BYPASS())
                .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
                .field("REFBIAS_PWD", &self.REFBIAS_PWD())
                .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
                .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
                .field("PLL_LOCK", &self.PLL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL_SIC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL_SIC {
                MISC2_CONTROL0: bool,
                PLL_EN_USB_CLKS: bool,
                PLL_POWER: bool,
                PLL_ENABLE: bool,
                PLL_BYPASS: bool,
                REFBIAS_PWD_SEL: bool,
                REFBIAS_PWD: bool,
                PLL_REG_ENABLE: bool,
                PLL_DIV_SEL: u8,
                PLL_LOCK: bool,
            }
            let proxy = PLL_SIC {
                MISC2_CONTROL0: self.MISC2_CONTROL0(),
                PLL_EN_USB_CLKS: self.PLL_EN_USB_CLKS(),
                PLL_POWER: self.PLL_POWER(),
                PLL_ENABLE: self.PLL_ENABLE(),
                PLL_BYPASS: self.PLL_BYPASS(),
                REFBIAS_PWD_SEL: self.REFBIAS_PWD_SEL(),
                REFBIAS_PWD: self.REFBIAS_PWD(),
                PLL_REG_ENABLE: self.PLL_REG_ENABLE(),
                PLL_DIV_SEL: self.PLL_DIV_SEL(),
                PLL_LOCK: self.PLL_LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL SIC"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL_SIC_CLR(pub u32);
    impl PLL_SIC_CLR {
        #[inline(always)]
        pub const fn MISC2_CONTROL0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MISC2_CONTROL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PLL_EN_USB_CLKS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PLL_POWER(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_POWER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLL_ENABLE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PLL_BYPASS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD_SEL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn PLL_REG_ENABLE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_REG_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn PLL_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[inline(always)]
        pub const fn PLL_LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PLL_SIC_CLR {
        #[inline(always)]
        fn default() -> PLL_SIC_CLR {
            PLL_SIC_CLR(0)
        }
    }
    impl core::fmt::Debug for PLL_SIC_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL_SIC_CLR")
                .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
                .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
                .field("PLL_POWER", &self.PLL_POWER())
                .field("PLL_ENABLE", &self.PLL_ENABLE())
                .field("PLL_BYPASS", &self.PLL_BYPASS())
                .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
                .field("REFBIAS_PWD", &self.REFBIAS_PWD())
                .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
                .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
                .field("PLL_LOCK", &self.PLL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL_SIC_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL_SIC_CLR {
                MISC2_CONTROL0: bool,
                PLL_EN_USB_CLKS: bool,
                PLL_POWER: bool,
                PLL_ENABLE: bool,
                PLL_BYPASS: bool,
                REFBIAS_PWD_SEL: bool,
                REFBIAS_PWD: bool,
                PLL_REG_ENABLE: bool,
                PLL_DIV_SEL: u8,
                PLL_LOCK: bool,
            }
            let proxy = PLL_SIC_CLR {
                MISC2_CONTROL0: self.MISC2_CONTROL0(),
                PLL_EN_USB_CLKS: self.PLL_EN_USB_CLKS(),
                PLL_POWER: self.PLL_POWER(),
                PLL_ENABLE: self.PLL_ENABLE(),
                PLL_BYPASS: self.PLL_BYPASS(),
                REFBIAS_PWD_SEL: self.REFBIAS_PWD_SEL(),
                REFBIAS_PWD: self.REFBIAS_PWD(),
                PLL_REG_ENABLE: self.PLL_REG_ENABLE(),
                PLL_DIV_SEL: self.PLL_DIV_SEL(),
                PLL_LOCK: self.PLL_LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL SIC"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL_SIC_SET(pub u32);
    impl PLL_SIC_SET {
        #[inline(always)]
        pub const fn MISC2_CONTROL0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MISC2_CONTROL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PLL_EN_USB_CLKS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PLL_POWER(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_POWER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLL_ENABLE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PLL_BYPASS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD_SEL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn PLL_REG_ENABLE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_REG_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn PLL_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[inline(always)]
        pub const fn PLL_LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PLL_SIC_SET {
        #[inline(always)]
        fn default() -> PLL_SIC_SET {
            PLL_SIC_SET(0)
        }
    }
    impl core::fmt::Debug for PLL_SIC_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL_SIC_SET")
                .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
                .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
                .field("PLL_POWER", &self.PLL_POWER())
                .field("PLL_ENABLE", &self.PLL_ENABLE())
                .field("PLL_BYPASS", &self.PLL_BYPASS())
                .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
                .field("REFBIAS_PWD", &self.REFBIAS_PWD())
                .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
                .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
                .field("PLL_LOCK", &self.PLL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL_SIC_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL_SIC_SET {
                MISC2_CONTROL0: bool,
                PLL_EN_USB_CLKS: bool,
                PLL_POWER: bool,
                PLL_ENABLE: bool,
                PLL_BYPASS: bool,
                REFBIAS_PWD_SEL: bool,
                REFBIAS_PWD: bool,
                PLL_REG_ENABLE: bool,
                PLL_DIV_SEL: u8,
                PLL_LOCK: bool,
            }
            let proxy = PLL_SIC_SET {
                MISC2_CONTROL0: self.MISC2_CONTROL0(),
                PLL_EN_USB_CLKS: self.PLL_EN_USB_CLKS(),
                PLL_POWER: self.PLL_POWER(),
                PLL_ENABLE: self.PLL_ENABLE(),
                PLL_BYPASS: self.PLL_BYPASS(),
                REFBIAS_PWD_SEL: self.REFBIAS_PWD_SEL(),
                REFBIAS_PWD: self.REFBIAS_PWD(),
                PLL_REG_ENABLE: self.PLL_REG_ENABLE(),
                PLL_DIV_SEL: self.PLL_DIV_SEL(),
                PLL_LOCK: self.PLL_LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL SIC"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL_SIC_TOG(pub u32);
    impl PLL_SIC_TOG {
        #[inline(always)]
        pub const fn MISC2_CONTROL0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MISC2_CONTROL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PLL_EN_USB_CLKS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PLL_POWER(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_POWER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLL_ENABLE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn PLL_BYPASS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD_SEL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REFBIAS_PWD(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFBIAS_PWD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn PLL_REG_ENABLE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_REG_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn PLL_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
        #[inline(always)]
        pub const fn PLL_LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PLL_SIC_TOG {
        #[inline(always)]
        fn default() -> PLL_SIC_TOG {
            PLL_SIC_TOG(0)
        }
    }
    impl core::fmt::Debug for PLL_SIC_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL_SIC_TOG")
                .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
                .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
                .field("PLL_POWER", &self.PLL_POWER())
                .field("PLL_ENABLE", &self.PLL_ENABLE())
                .field("PLL_BYPASS", &self.PLL_BYPASS())
                .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
                .field("REFBIAS_PWD", &self.REFBIAS_PWD())
                .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
                .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
                .field("PLL_LOCK", &self.PLL_LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL_SIC_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL_SIC_TOG {
                MISC2_CONTROL0: bool,
                PLL_EN_USB_CLKS: bool,
                PLL_POWER: bool,
                PLL_ENABLE: bool,
                PLL_BYPASS: bool,
                REFBIAS_PWD_SEL: bool,
                REFBIAS_PWD: bool,
                PLL_REG_ENABLE: bool,
                PLL_DIV_SEL: u8,
                PLL_LOCK: bool,
            }
            let proxy = PLL_SIC_TOG {
                MISC2_CONTROL0: self.MISC2_CONTROL0(),
                PLL_EN_USB_CLKS: self.PLL_EN_USB_CLKS(),
                PLL_POWER: self.PLL_POWER(),
                PLL_ENABLE: self.PLL_ENABLE(),
                PLL_BYPASS: self.PLL_BYPASS(),
                REFBIAS_PWD_SEL: self.REFBIAS_PWD_SEL(),
                REFBIAS_PWD: self.REFBIAS_PWD(),
                PLL_REG_ENABLE: self.PLL_REG_ENABLE(),
                PLL_DIV_SEL: self.PLL_DIV_SEL(),
                PLL_LOCK: self.PLL_LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Power Down"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWD(pub u32);
    impl PWD {
        #[inline(always)]
        pub const fn TXPWDFS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TXPWDIBIAS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDIBIAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TXPWDV2I(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDV2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RXPWDENV(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDENV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RXPWD1PT1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWD1PT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RXPWDDIFF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDDIFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RXPWDRX(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDRX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for PWD {
        #[inline(always)]
        fn default() -> PWD {
            PWD(0)
        }
    }
    impl core::fmt::Debug for PWD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWD")
                .field("TXPWDFS", &self.TXPWDFS())
                .field("TXPWDIBIAS", &self.TXPWDIBIAS())
                .field("TXPWDV2I", &self.TXPWDV2I())
                .field("RXPWDENV", &self.RXPWDENV())
                .field("RXPWD1PT1", &self.RXPWD1PT1())
                .field("RXPWDDIFF", &self.RXPWDDIFF())
                .field("RXPWDRX", &self.RXPWDRX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWD {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWD {
                TXPWDFS: bool,
                TXPWDIBIAS: bool,
                TXPWDV2I: bool,
                RXPWDENV: bool,
                RXPWD1PT1: bool,
                RXPWDDIFF: bool,
                RXPWDRX: bool,
            }
            let proxy = PWD {
                TXPWDFS: self.TXPWDFS(),
                TXPWDIBIAS: self.TXPWDIBIAS(),
                TXPWDV2I: self.TXPWDV2I(),
                RXPWDENV: self.RXPWDENV(),
                RXPWD1PT1: self.RXPWD1PT1(),
                RXPWDDIFF: self.RXPWDDIFF(),
                RXPWDRX: self.RXPWDRX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Power Down"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWD_CLR(pub u32);
    impl PWD_CLR {
        #[inline(always)]
        pub const fn TXPWDFS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TXPWDIBIAS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDIBIAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TXPWDV2I(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDV2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RXPWDENV(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDENV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RXPWD1PT1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWD1PT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RXPWDDIFF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDDIFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RXPWDRX(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDRX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for PWD_CLR {
        #[inline(always)]
        fn default() -> PWD_CLR {
            PWD_CLR(0)
        }
    }
    impl core::fmt::Debug for PWD_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWD_CLR")
                .field("TXPWDFS", &self.TXPWDFS())
                .field("TXPWDIBIAS", &self.TXPWDIBIAS())
                .field("TXPWDV2I", &self.TXPWDV2I())
                .field("RXPWDENV", &self.RXPWDENV())
                .field("RXPWD1PT1", &self.RXPWD1PT1())
                .field("RXPWDDIFF", &self.RXPWDDIFF())
                .field("RXPWDRX", &self.RXPWDRX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWD_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWD_CLR {
                TXPWDFS: bool,
                TXPWDIBIAS: bool,
                TXPWDV2I: bool,
                RXPWDENV: bool,
                RXPWD1PT1: bool,
                RXPWDDIFF: bool,
                RXPWDRX: bool,
            }
            let proxy = PWD_CLR {
                TXPWDFS: self.TXPWDFS(),
                TXPWDIBIAS: self.TXPWDIBIAS(),
                TXPWDV2I: self.TXPWDV2I(),
                RXPWDENV: self.RXPWDENV(),
                RXPWD1PT1: self.RXPWD1PT1(),
                RXPWDDIFF: self.RXPWDDIFF(),
                RXPWDRX: self.RXPWDRX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Power Down"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWD_SET(pub u32);
    impl PWD_SET {
        #[inline(always)]
        pub const fn TXPWDFS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TXPWDIBIAS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDIBIAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TXPWDV2I(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDV2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RXPWDENV(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDENV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RXPWD1PT1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWD1PT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RXPWDDIFF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDDIFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RXPWDRX(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDRX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for PWD_SET {
        #[inline(always)]
        fn default() -> PWD_SET {
            PWD_SET(0)
        }
    }
    impl core::fmt::Debug for PWD_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWD_SET")
                .field("TXPWDFS", &self.TXPWDFS())
                .field("TXPWDIBIAS", &self.TXPWDIBIAS())
                .field("TXPWDV2I", &self.TXPWDV2I())
                .field("RXPWDENV", &self.RXPWDENV())
                .field("RXPWD1PT1", &self.RXPWD1PT1())
                .field("RXPWDDIFF", &self.RXPWDDIFF())
                .field("RXPWDRX", &self.RXPWDRX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWD_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWD_SET {
                TXPWDFS: bool,
                TXPWDIBIAS: bool,
                TXPWDV2I: bool,
                RXPWDENV: bool,
                RXPWD1PT1: bool,
                RXPWDDIFF: bool,
                RXPWDRX: bool,
            }
            let proxy = PWD_SET {
                TXPWDFS: self.TXPWDFS(),
                TXPWDIBIAS: self.TXPWDIBIAS(),
                TXPWDV2I: self.TXPWDV2I(),
                RXPWDENV: self.RXPWDENV(),
                RXPWD1PT1: self.RXPWD1PT1(),
                RXPWDDIFF: self.RXPWDDIFF(),
                RXPWDRX: self.RXPWDRX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Power Down"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWD_TOG(pub u32);
    impl PWD_TOG {
        #[inline(always)]
        pub const fn TXPWDFS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TXPWDIBIAS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDIBIAS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TXPWDV2I(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXPWDV2I(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn RXPWDENV(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDENV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RXPWD1PT1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWD1PT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn RXPWDDIFF(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDDIFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RXPWDRX(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXPWDRX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for PWD_TOG {
        #[inline(always)]
        fn default() -> PWD_TOG {
            PWD_TOG(0)
        }
    }
    impl core::fmt::Debug for PWD_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWD_TOG")
                .field("TXPWDFS", &self.TXPWDFS())
                .field("TXPWDIBIAS", &self.TXPWDIBIAS())
                .field("TXPWDV2I", &self.TXPWDV2I())
                .field("RXPWDENV", &self.RXPWDENV())
                .field("RXPWD1PT1", &self.RXPWD1PT1())
                .field("RXPWDDIFF", &self.RXPWDDIFF())
                .field("RXPWDRX", &self.RXPWDRX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWD_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PWD_TOG {
                TXPWDFS: bool,
                TXPWDIBIAS: bool,
                TXPWDV2I: bool,
                RXPWDENV: bool,
                RXPWD1PT1: bool,
                RXPWDDIFF: bool,
                RXPWDRX: bool,
            }
            let proxy = PWD_TOG {
                TXPWDFS: self.TXPWDFS(),
                TXPWDIBIAS: self.TXPWDIBIAS(),
                TXPWDV2I: self.TXPWDV2I(),
                RXPWDENV: self.RXPWDENV(),
                RXPWD1PT1: self.RXPWD1PT1(),
                RXPWDDIFF: self.RXPWDDIFF(),
                RXPWDRX: self.RXPWDRX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX(pub u32);
    impl RX {
        #[inline(always)]
        pub const fn ENVADJ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENVADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DISCONADJ(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DISCONADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RX {
        #[inline(always)]
        fn default() -> RX {
            RX(0)
        }
    }
    impl core::fmt::Debug for RX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX")
                .field("ENVADJ", &self.ENVADJ())
                .field("DISCONADJ", &self.DISCONADJ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RX {
                ENVADJ: u8,
                DISCONADJ: u8,
            }
            let proxy = RX {
                ENVADJ: self.ENVADJ(),
                DISCONADJ: self.DISCONADJ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_CLR(pub u32);
    impl RX_CLR {
        #[inline(always)]
        pub const fn ENVADJ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENVADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DISCONADJ(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DISCONADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RX_CLR {
        #[inline(always)]
        fn default() -> RX_CLR {
            RX_CLR(0)
        }
    }
    impl core::fmt::Debug for RX_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_CLR")
                .field("ENVADJ", &self.ENVADJ())
                .field("DISCONADJ", &self.DISCONADJ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RX_CLR {
                ENVADJ: u8,
                DISCONADJ: u8,
            }
            let proxy = RX_CLR {
                ENVADJ: self.ENVADJ(),
                DISCONADJ: self.DISCONADJ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_SET(pub u32);
    impl RX_SET {
        #[inline(always)]
        pub const fn ENVADJ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENVADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DISCONADJ(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DISCONADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RX_SET {
        #[inline(always)]
        fn default() -> RX_SET {
            RX_SET(0)
        }
    }
    impl core::fmt::Debug for RX_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_SET")
                .field("ENVADJ", &self.ENVADJ())
                .field("DISCONADJ", &self.DISCONADJ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RX_SET {
                ENVADJ: u8,
                DISCONADJ: u8,
            }
            let proxy = RX_SET {
                ENVADJ: self.ENVADJ(),
                DISCONADJ: self.DISCONADJ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RX_TOG(pub u32);
    impl RX_TOG {
        #[inline(always)]
        pub const fn ENVADJ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENVADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn DISCONADJ(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_DISCONADJ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for RX_TOG {
        #[inline(always)]
        fn default() -> RX_TOG {
            RX_TOG(0)
        }
    }
    impl core::fmt::Debug for RX_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RX_TOG")
                .field("ENVADJ", &self.ENVADJ())
                .field("DISCONADJ", &self.DISCONADJ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RX_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RX_TOG {
                ENVADJ: u8,
                DISCONADJ: u8,
            }
            let proxy = RX_TOG {
                ENVADJ: self.ENVADJ(),
                DISCONADJ: self.DISCONADJ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[inline(always)]
        pub const fn OK_STATUS_3V(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OK_STATUS_3V(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HOSTDISCONDETECT_STATUS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HOSTDISCONDETECT_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DEVPLUGIN_STATUS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVPLUGIN_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn OTGID_STATUS(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OTGID_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RESUME_STATUS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("OK_STATUS_3V", &self.OK_STATUS_3V())
                .field("HOSTDISCONDETECT_STATUS", &self.HOSTDISCONDETECT_STATUS())
                .field("DEVPLUGIN_STATUS", &self.DEVPLUGIN_STATUS())
                .field("OTGID_STATUS", &self.OTGID_STATUS())
                .field("RESUME_STATUS", &self.RESUME_STATUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STATUS {
                OK_STATUS_3V: bool,
                HOSTDISCONDETECT_STATUS: bool,
                DEVPLUGIN_STATUS: bool,
                OTGID_STATUS: bool,
                RESUME_STATUS: bool,
            }
            let proxy = STATUS {
                OK_STATUS_3V: self.OK_STATUS_3V(),
                HOSTDISCONDETECT_STATUS: self.HOSTDISCONDETECT_STATUS(),
                DEVPLUGIN_STATUS: self.DEVPLUGIN_STATUS(),
                OTGID_STATUS: self.OTGID_STATUS(),
                RESUME_STATUS: self.RESUME_STATUS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_OVERRIDE_EN(pub u32);
    impl TRIM_OVERRIDE_EN {
        #[inline(always)]
        pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for TRIM_OVERRIDE_EN {
        #[inline(always)]
        fn default() -> TRIM_OVERRIDE_EN {
            TRIM_OVERRIDE_EN(0)
        }
    }
    impl core::fmt::Debug for TRIM_OVERRIDE_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_OVERRIDE_EN")
                .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
                .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
                .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
                .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
                .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
                .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
                .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
                .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_OVERRIDE_EN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRIM_OVERRIDE_EN {
                DIV_SEL_OVERRIDE: bool,
                TX_D_CAL_OVERRIDE: bool,
                TX_CAL45DP_OVERRIDE: bool,
                TX_CAL45DM_OVERRIDE: bool,
                PLL_CTRL0_DIV_SEL: u8,
                USBPHY_TX_D_CAL: u8,
                USBPHY_TX_CAL45DP: u8,
                USBPHY_TX_CAL45DN: u8,
            }
            let proxy = TRIM_OVERRIDE_EN {
                DIV_SEL_OVERRIDE: self.DIV_SEL_OVERRIDE(),
                TX_D_CAL_OVERRIDE: self.TX_D_CAL_OVERRIDE(),
                TX_CAL45DP_OVERRIDE: self.TX_CAL45DP_OVERRIDE(),
                TX_CAL45DM_OVERRIDE: self.TX_CAL45DM_OVERRIDE(),
                PLL_CTRL0_DIV_SEL: self.PLL_CTRL0_DIV_SEL(),
                USBPHY_TX_D_CAL: self.USBPHY_TX_D_CAL(),
                USBPHY_TX_CAL45DP: self.USBPHY_TX_CAL45DP(),
                USBPHY_TX_CAL45DN: self.USBPHY_TX_CAL45DN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_OVERRIDE_EN_CLR(pub u32);
    impl TRIM_OVERRIDE_EN_CLR {
        #[inline(always)]
        pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for TRIM_OVERRIDE_EN_CLR {
        #[inline(always)]
        fn default() -> TRIM_OVERRIDE_EN_CLR {
            TRIM_OVERRIDE_EN_CLR(0)
        }
    }
    impl core::fmt::Debug for TRIM_OVERRIDE_EN_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_OVERRIDE_EN_CLR")
                .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
                .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
                .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
                .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
                .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
                .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
                .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
                .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_OVERRIDE_EN_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRIM_OVERRIDE_EN_CLR {
                DIV_SEL_OVERRIDE: bool,
                TX_D_CAL_OVERRIDE: bool,
                TX_CAL45DP_OVERRIDE: bool,
                TX_CAL45DM_OVERRIDE: bool,
                PLL_CTRL0_DIV_SEL: u8,
                USBPHY_TX_D_CAL: u8,
                USBPHY_TX_CAL45DP: u8,
                USBPHY_TX_CAL45DN: u8,
            }
            let proxy = TRIM_OVERRIDE_EN_CLR {
                DIV_SEL_OVERRIDE: self.DIV_SEL_OVERRIDE(),
                TX_D_CAL_OVERRIDE: self.TX_D_CAL_OVERRIDE(),
                TX_CAL45DP_OVERRIDE: self.TX_CAL45DP_OVERRIDE(),
                TX_CAL45DM_OVERRIDE: self.TX_CAL45DM_OVERRIDE(),
                PLL_CTRL0_DIV_SEL: self.PLL_CTRL0_DIV_SEL(),
                USBPHY_TX_D_CAL: self.USBPHY_TX_D_CAL(),
                USBPHY_TX_CAL45DP: self.USBPHY_TX_CAL45DP(),
                USBPHY_TX_CAL45DN: self.USBPHY_TX_CAL45DN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_OVERRIDE_EN_SET(pub u32);
    impl TRIM_OVERRIDE_EN_SET {
        #[inline(always)]
        pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for TRIM_OVERRIDE_EN_SET {
        #[inline(always)]
        fn default() -> TRIM_OVERRIDE_EN_SET {
            TRIM_OVERRIDE_EN_SET(0)
        }
    }
    impl core::fmt::Debug for TRIM_OVERRIDE_EN_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_OVERRIDE_EN_SET")
                .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
                .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
                .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
                .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
                .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
                .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
                .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
                .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_OVERRIDE_EN_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRIM_OVERRIDE_EN_SET {
                DIV_SEL_OVERRIDE: bool,
                TX_D_CAL_OVERRIDE: bool,
                TX_CAL45DP_OVERRIDE: bool,
                TX_CAL45DM_OVERRIDE: bool,
                PLL_CTRL0_DIV_SEL: u8,
                USBPHY_TX_D_CAL: u8,
                USBPHY_TX_CAL45DP: u8,
                USBPHY_TX_CAL45DN: u8,
            }
            let proxy = TRIM_OVERRIDE_EN_SET {
                DIV_SEL_OVERRIDE: self.DIV_SEL_OVERRIDE(),
                TX_D_CAL_OVERRIDE: self.TX_D_CAL_OVERRIDE(),
                TX_CAL45DP_OVERRIDE: self.TX_CAL45DP_OVERRIDE(),
                TX_CAL45DM_OVERRIDE: self.TX_CAL45DM_OVERRIDE(),
                PLL_CTRL0_DIV_SEL: self.PLL_CTRL0_DIV_SEL(),
                USBPHY_TX_D_CAL: self.USBPHY_TX_D_CAL(),
                USBPHY_TX_CAL45DP: self.USBPHY_TX_CAL45DP(),
                USBPHY_TX_CAL45DN: self.USBPHY_TX_CAL45DN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_OVERRIDE_EN_TOG(pub u32);
    impl TRIM_OVERRIDE_EN_TOG {
        #[inline(always)]
        pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for TRIM_OVERRIDE_EN_TOG {
        #[inline(always)]
        fn default() -> TRIM_OVERRIDE_EN_TOG {
            TRIM_OVERRIDE_EN_TOG(0)
        }
    }
    impl core::fmt::Debug for TRIM_OVERRIDE_EN_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_OVERRIDE_EN_TOG")
                .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
                .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
                .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
                .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
                .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
                .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
                .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
                .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_OVERRIDE_EN_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TRIM_OVERRIDE_EN_TOG {
                DIV_SEL_OVERRIDE: bool,
                TX_D_CAL_OVERRIDE: bool,
                TX_CAL45DP_OVERRIDE: bool,
                TX_CAL45DM_OVERRIDE: bool,
                PLL_CTRL0_DIV_SEL: u8,
                USBPHY_TX_D_CAL: u8,
                USBPHY_TX_CAL45DP: u8,
                USBPHY_TX_CAL45DN: u8,
            }
            let proxy = TRIM_OVERRIDE_EN_TOG {
                DIV_SEL_OVERRIDE: self.DIV_SEL_OVERRIDE(),
                TX_D_CAL_OVERRIDE: self.TX_D_CAL_OVERRIDE(),
                TX_CAL45DP_OVERRIDE: self.TX_CAL45DP_OVERRIDE(),
                TX_CAL45DM_OVERRIDE: self.TX_CAL45DM_OVERRIDE(),
                PLL_CTRL0_DIV_SEL: self.PLL_CTRL0_DIV_SEL(),
                USBPHY_TX_D_CAL: self.USBPHY_TX_D_CAL(),
                USBPHY_TX_CAL45DP: self.USBPHY_TX_CAL45DP(),
                USBPHY_TX_CAL45DN: self.USBPHY_TX_CAL45DN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX(pub u32);
    impl TX {
        #[inline(always)]
        pub const fn D_CAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TX {
        #[inline(always)]
        fn default() -> TX {
            TX(0)
        }
    }
    impl core::fmt::Debug for TX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX")
                .field("D_CAL", &self.D_CAL())
                .field("TXCAL45DN", &self.TXCAL45DN())
                .field("TXCAL45DP", &self.TXCAL45DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TX {
                D_CAL: u8,
                TXCAL45DN: u8,
                TXCAL45DP: u8,
            }
            let proxy = TX {
                D_CAL: self.D_CAL(),
                TXCAL45DN: self.TXCAL45DN(),
                TXCAL45DP: self.TXCAL45DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_CLR(pub u32);
    impl TX_CLR {
        #[inline(always)]
        pub const fn D_CAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TX_CLR {
        #[inline(always)]
        fn default() -> TX_CLR {
            TX_CLR(0)
        }
    }
    impl core::fmt::Debug for TX_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_CLR")
                .field("D_CAL", &self.D_CAL())
                .field("TXCAL45DN", &self.TXCAL45DN())
                .field("TXCAL45DP", &self.TXCAL45DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TX_CLR {
                D_CAL: u8,
                TXCAL45DN: u8,
                TXCAL45DP: u8,
            }
            let proxy = TX_CLR {
                D_CAL: self.D_CAL(),
                TXCAL45DN: self.TXCAL45DN(),
                TXCAL45DP: self.TXCAL45DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_SET(pub u32);
    impl TX_SET {
        #[inline(always)]
        pub const fn D_CAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TX_SET {
        #[inline(always)]
        fn default() -> TX_SET {
            TX_SET(0)
        }
    }
    impl core::fmt::Debug for TX_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_SET")
                .field("D_CAL", &self.D_CAL())
                .field("TXCAL45DN", &self.TXCAL45DN())
                .field("TXCAL45DP", &self.TXCAL45DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TX_SET {
                D_CAL: u8,
                TXCAL45DN: u8,
                TXCAL45DP: u8,
            }
            let proxy = TX_SET {
                D_CAL: self.D_CAL(),
                TXCAL45DN: self.TXCAL45DN(),
                TXCAL45DP: self.TXCAL45DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TX Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TX_TOG(pub u32);
    impl TX_TOG {
        #[inline(always)]
        pub const fn D_CAL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_D_CAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn TXCAL45DP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCAL45DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TX_TOG {
        #[inline(always)]
        fn default() -> TX_TOG {
            TX_TOG(0)
        }
    }
    impl core::fmt::Debug for TX_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TX_TOG")
                .field("D_CAL", &self.D_CAL())
                .field("TXCAL45DN", &self.TXCAL45DN())
                .field("TXCAL45DP", &self.TXCAL45DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TX_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TX_TOG {
                D_CAL: u8,
                TXCAL45DN: u8,
                TXCAL45DP: u8,
            }
            let proxy = TX_TOG {
                D_CAL: self.D_CAL(),
                TXCAL45DN: self.TXCAL45DN(),
                TXCAL45DP: self.TXCAL45DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DETECT(pub u32);
    impl USB1_CHRG_DETECT {
        #[inline(always)]
        pub const fn DETECT_SEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DETECT_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PULLUP_DP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULLUP_DP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VDM_SRC_ENABLE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHK_CONTACT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CHK_CHRG_B(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CHRG_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EN_B(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn DCDSEL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for USB1_CHRG_DETECT {
        #[inline(always)]
        fn default() -> USB1_CHRG_DETECT {
            USB1_CHRG_DETECT(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DETECT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DETECT")
                .field("DETECT_SEC", &self.DETECT_SEC())
                .field("PULLUP_DP", &self.PULLUP_DP())
                .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
                .field("CHK_CONTACT", &self.CHK_CONTACT())
                .field("CHK_CHRG_B", &self.CHK_CHRG_B())
                .field("EN_B", &self.EN_B())
                .field("DCDSEL", &self.DCDSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DETECT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DETECT {
                DETECT_SEC: bool,
                PULLUP_DP: bool,
                VDM_SRC_ENABLE: bool,
                CHK_CONTACT: bool,
                CHK_CHRG_B: bool,
                EN_B: bool,
                DCDSEL: bool,
            }
            let proxy = USB1_CHRG_DETECT {
                DETECT_SEC: self.DETECT_SEC(),
                PULLUP_DP: self.PULLUP_DP(),
                VDM_SRC_ENABLE: self.VDM_SRC_ENABLE(),
                CHK_CONTACT: self.CHK_CONTACT(),
                CHK_CHRG_B: self.CHK_CHRG_B(),
                EN_B: self.EN_B(),
                DCDSEL: self.DCDSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DETECT_CLR(pub u32);
    impl USB1_CHRG_DETECT_CLR {
        #[inline(always)]
        pub const fn DETECT_SEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DETECT_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PULLUP_DP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULLUP_DP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VDM_SRC_ENABLE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHK_CONTACT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CHK_CHRG_B(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CHRG_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EN_B(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn DCDSEL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for USB1_CHRG_DETECT_CLR {
        #[inline(always)]
        fn default() -> USB1_CHRG_DETECT_CLR {
            USB1_CHRG_DETECT_CLR(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DETECT_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DETECT_CLR")
                .field("DETECT_SEC", &self.DETECT_SEC())
                .field("PULLUP_DP", &self.PULLUP_DP())
                .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
                .field("CHK_CONTACT", &self.CHK_CONTACT())
                .field("CHK_CHRG_B", &self.CHK_CHRG_B())
                .field("EN_B", &self.EN_B())
                .field("DCDSEL", &self.DCDSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DETECT_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DETECT_CLR {
                DETECT_SEC: bool,
                PULLUP_DP: bool,
                VDM_SRC_ENABLE: bool,
                CHK_CONTACT: bool,
                CHK_CHRG_B: bool,
                EN_B: bool,
                DCDSEL: bool,
            }
            let proxy = USB1_CHRG_DETECT_CLR {
                DETECT_SEC: self.DETECT_SEC(),
                PULLUP_DP: self.PULLUP_DP(),
                VDM_SRC_ENABLE: self.VDM_SRC_ENABLE(),
                CHK_CONTACT: self.CHK_CONTACT(),
                CHK_CHRG_B: self.CHK_CHRG_B(),
                EN_B: self.EN_B(),
                DCDSEL: self.DCDSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DETECT_SET(pub u32);
    impl USB1_CHRG_DETECT_SET {
        #[inline(always)]
        pub const fn DETECT_SEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DETECT_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PULLUP_DP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULLUP_DP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VDM_SRC_ENABLE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHK_CONTACT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CHK_CHRG_B(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CHRG_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EN_B(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn DCDSEL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for USB1_CHRG_DETECT_SET {
        #[inline(always)]
        fn default() -> USB1_CHRG_DETECT_SET {
            USB1_CHRG_DETECT_SET(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DETECT_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DETECT_SET")
                .field("DETECT_SEC", &self.DETECT_SEC())
                .field("PULLUP_DP", &self.PULLUP_DP())
                .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
                .field("CHK_CONTACT", &self.CHK_CONTACT())
                .field("CHK_CHRG_B", &self.CHK_CHRG_B())
                .field("EN_B", &self.EN_B())
                .field("DCDSEL", &self.DCDSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DETECT_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DETECT_SET {
                DETECT_SEC: bool,
                PULLUP_DP: bool,
                VDM_SRC_ENABLE: bool,
                CHK_CONTACT: bool,
                CHK_CHRG_B: bool,
                EN_B: bool,
                DCDSEL: bool,
            }
            let proxy = USB1_CHRG_DETECT_SET {
                DETECT_SEC: self.DETECT_SEC(),
                PULLUP_DP: self.PULLUP_DP(),
                VDM_SRC_ENABLE: self.VDM_SRC_ENABLE(),
                CHK_CONTACT: self.CHK_CONTACT(),
                CHK_CHRG_B: self.CHK_CHRG_B(),
                EN_B: self.EN_B(),
                DCDSEL: self.DCDSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DETECT_TOG(pub u32);
    impl USB1_CHRG_DETECT_TOG {
        #[inline(always)]
        pub const fn DETECT_SEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DETECT_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PULLUP_DP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PULLUP_DP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VDM_SRC_ENABLE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHK_CONTACT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CHK_CHRG_B(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHK_CHRG_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn EN_B(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn DCDSEL(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DCDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for USB1_CHRG_DETECT_TOG {
        #[inline(always)]
        fn default() -> USB1_CHRG_DETECT_TOG {
            USB1_CHRG_DETECT_TOG(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DETECT_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DETECT_TOG")
                .field("DETECT_SEC", &self.DETECT_SEC())
                .field("PULLUP_DP", &self.PULLUP_DP())
                .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
                .field("CHK_CONTACT", &self.CHK_CONTACT())
                .field("CHK_CHRG_B", &self.CHK_CHRG_B())
                .field("EN_B", &self.EN_B())
                .field("DCDSEL", &self.DCDSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DETECT_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DETECT_TOG {
                DETECT_SEC: bool,
                PULLUP_DP: bool,
                VDM_SRC_ENABLE: bool,
                CHK_CONTACT: bool,
                CHK_CHRG_B: bool,
                EN_B: bool,
                DCDSEL: bool,
            }
            let proxy = USB1_CHRG_DETECT_TOG {
                DETECT_SEC: self.DETECT_SEC(),
                PULLUP_DP: self.PULLUP_DP(),
                VDM_SRC_ENABLE: self.VDM_SRC_ENABLE(),
                CHK_CONTACT: self.CHK_CONTACT(),
                CHK_CHRG_B: self.CHK_CHRG_B(),
                EN_B: self.EN_B(),
                DCDSEL: self.DCDSEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DET_STAT(pub u32);
    impl USB1_CHRG_DET_STAT {
        #[inline(always)]
        pub const fn PLUG_CONTACT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLUG_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CHRG_DETECTED(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRG_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DM_STATE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DM_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DP_STATE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DP_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SECDET_DCP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SECDET_DCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for USB1_CHRG_DET_STAT {
        #[inline(always)]
        fn default() -> USB1_CHRG_DET_STAT {
            USB1_CHRG_DET_STAT(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DET_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DET_STAT")
                .field("PLUG_CONTACT", &self.PLUG_CONTACT())
                .field("CHRG_DETECTED", &self.CHRG_DETECTED())
                .field("DM_STATE", &self.DM_STATE())
                .field("DP_STATE", &self.DP_STATE())
                .field("SECDET_DCP", &self.SECDET_DCP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DET_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DET_STAT {
                PLUG_CONTACT: bool,
                CHRG_DETECTED: bool,
                DM_STATE: bool,
                DP_STATE: bool,
                SECDET_DCP: bool,
            }
            let proxy = USB1_CHRG_DET_STAT {
                PLUG_CONTACT: self.PLUG_CONTACT(),
                CHRG_DETECTED: self.CHRG_DETECTED(),
                DM_STATE: self.DM_STATE(),
                DP_STATE: self.DP_STATE(),
                SECDET_DCP: self.SECDET_DCP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DET_STAT_CLR(pub u32);
    impl USB1_CHRG_DET_STAT_CLR {
        #[inline(always)]
        pub const fn PLUG_CONTACT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLUG_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CHRG_DETECTED(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRG_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DM_STATE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DM_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DP_STATE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DP_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SECDET_DCP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SECDET_DCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for USB1_CHRG_DET_STAT_CLR {
        #[inline(always)]
        fn default() -> USB1_CHRG_DET_STAT_CLR {
            USB1_CHRG_DET_STAT_CLR(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DET_STAT_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DET_STAT_CLR")
                .field("PLUG_CONTACT", &self.PLUG_CONTACT())
                .field("CHRG_DETECTED", &self.CHRG_DETECTED())
                .field("DM_STATE", &self.DM_STATE())
                .field("DP_STATE", &self.DP_STATE())
                .field("SECDET_DCP", &self.SECDET_DCP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DET_STAT_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DET_STAT_CLR {
                PLUG_CONTACT: bool,
                CHRG_DETECTED: bool,
                DM_STATE: bool,
                DP_STATE: bool,
                SECDET_DCP: bool,
            }
            let proxy = USB1_CHRG_DET_STAT_CLR {
                PLUG_CONTACT: self.PLUG_CONTACT(),
                CHRG_DETECTED: self.CHRG_DETECTED(),
                DM_STATE: self.DM_STATE(),
                DP_STATE: self.DP_STATE(),
                SECDET_DCP: self.SECDET_DCP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DET_STAT_SET(pub u32);
    impl USB1_CHRG_DET_STAT_SET {
        #[inline(always)]
        pub const fn PLUG_CONTACT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLUG_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CHRG_DETECTED(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRG_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DM_STATE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DM_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DP_STATE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DP_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SECDET_DCP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SECDET_DCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for USB1_CHRG_DET_STAT_SET {
        #[inline(always)]
        fn default() -> USB1_CHRG_DET_STAT_SET {
            USB1_CHRG_DET_STAT_SET(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DET_STAT_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DET_STAT_SET")
                .field("PLUG_CONTACT", &self.PLUG_CONTACT())
                .field("CHRG_DETECTED", &self.CHRG_DETECTED())
                .field("DM_STATE", &self.DM_STATE())
                .field("DP_STATE", &self.DP_STATE())
                .field("SECDET_DCP", &self.SECDET_DCP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DET_STAT_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DET_STAT_SET {
                PLUG_CONTACT: bool,
                CHRG_DETECTED: bool,
                DM_STATE: bool,
                DP_STATE: bool,
                SECDET_DCP: bool,
            }
            let proxy = USB1_CHRG_DET_STAT_SET {
                PLUG_CONTACT: self.PLUG_CONTACT(),
                CHRG_DETECTED: self.CHRG_DETECTED(),
                DM_STATE: self.DM_STATE(),
                DP_STATE: self.DP_STATE(),
                SECDET_DCP: self.SECDET_DCP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Charger Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_CHRG_DET_STAT_TOG(pub u32);
    impl USB1_CHRG_DET_STAT_TOG {
        #[inline(always)]
        pub const fn PLUG_CONTACT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLUG_CONTACT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CHRG_DETECTED(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHRG_DETECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DM_STATE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DM_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DP_STATE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DP_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SECDET_DCP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SECDET_DCP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for USB1_CHRG_DET_STAT_TOG {
        #[inline(always)]
        fn default() -> USB1_CHRG_DET_STAT_TOG {
            USB1_CHRG_DET_STAT_TOG(0)
        }
    }
    impl core::fmt::Debug for USB1_CHRG_DET_STAT_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_CHRG_DET_STAT_TOG")
                .field("PLUG_CONTACT", &self.PLUG_CONTACT())
                .field("CHRG_DETECTED", &self.CHRG_DETECTED())
                .field("DM_STATE", &self.DM_STATE())
                .field("DP_STATE", &self.DP_STATE())
                .field("SECDET_DCP", &self.SECDET_DCP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_CHRG_DET_STAT_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_CHRG_DET_STAT_TOG {
                PLUG_CONTACT: bool,
                CHRG_DETECTED: bool,
                DM_STATE: bool,
                DP_STATE: bool,
                SECDET_DCP: bool,
            }
            let proxy = USB1_CHRG_DET_STAT_TOG {
                PLUG_CONTACT: self.PLUG_CONTACT(),
                CHRG_DETECTED: self.CHRG_DETECTED(),
                DM_STATE: self.DM_STATE(),
                DP_STATE: self.DP_STATE(),
                SECDET_DCP: self.SECDET_DCP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DETECT(pub u32);
    impl USB1_VBUS_DETECT {
        #[inline(always)]
        pub const fn VBUSVALID_THRESH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_THRESH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SESSEND_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn BVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn AVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_SEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_TO_B(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_TO_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn DISCHARGE_VBUS(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DISCHARGE_VBUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for USB1_VBUS_DETECT {
        #[inline(always)]
        fn default() -> USB1_VBUS_DETECT {
            USB1_VBUS_DETECT(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DETECT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DETECT")
                .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
                .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
                .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
                .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
                .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
                .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
                .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
                .field("ID_OVERRIDE", &self.ID_OVERRIDE())
                .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
                .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
                .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
                .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
                .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DETECT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DETECT {
                VBUSVALID_THRESH: u8,
                VBUS_OVERRIDE_EN: bool,
                SESSEND_OVERRIDE: bool,
                BVALID_OVERRIDE: bool,
                AVALID_OVERRIDE: bool,
                VBUSVALID_OVERRIDE: bool,
                VBUSVALID_SEL: bool,
                VBUS_SOURCE_SEL: u8,
                ID_OVERRIDE_EN: bool,
                ID_OVERRIDE: bool,
                EXT_ID_OVERRIDE_EN: bool,
                EXT_VBUS_OVERRIDE_EN: bool,
                VBUSVALID_TO_B: bool,
                VBUSVALID_PWRUP_CMPS: u8,
                DISCHARGE_VBUS: bool,
            }
            let proxy = USB1_VBUS_DETECT {
                VBUSVALID_THRESH: self.VBUSVALID_THRESH(),
                VBUS_OVERRIDE_EN: self.VBUS_OVERRIDE_EN(),
                SESSEND_OVERRIDE: self.SESSEND_OVERRIDE(),
                BVALID_OVERRIDE: self.BVALID_OVERRIDE(),
                AVALID_OVERRIDE: self.AVALID_OVERRIDE(),
                VBUSVALID_OVERRIDE: self.VBUSVALID_OVERRIDE(),
                VBUSVALID_SEL: self.VBUSVALID_SEL(),
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                ID_OVERRIDE_EN: self.ID_OVERRIDE_EN(),
                ID_OVERRIDE: self.ID_OVERRIDE(),
                EXT_ID_OVERRIDE_EN: self.EXT_ID_OVERRIDE_EN(),
                EXT_VBUS_OVERRIDE_EN: self.EXT_VBUS_OVERRIDE_EN(),
                VBUSVALID_TO_B: self.VBUSVALID_TO_B(),
                VBUSVALID_PWRUP_CMPS: self.VBUSVALID_PWRUP_CMPS(),
                DISCHARGE_VBUS: self.DISCHARGE_VBUS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DETECT_CLR(pub u32);
    impl USB1_VBUS_DETECT_CLR {
        #[inline(always)]
        pub const fn VBUSVALID_THRESH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_THRESH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SESSEND_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn BVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn AVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_SEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_TO_B(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_TO_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn DISCHARGE_VBUS(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DISCHARGE_VBUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for USB1_VBUS_DETECT_CLR {
        #[inline(always)]
        fn default() -> USB1_VBUS_DETECT_CLR {
            USB1_VBUS_DETECT_CLR(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DETECT_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DETECT_CLR")
                .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
                .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
                .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
                .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
                .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
                .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
                .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
                .field("ID_OVERRIDE", &self.ID_OVERRIDE())
                .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
                .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
                .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
                .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
                .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DETECT_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DETECT_CLR {
                VBUSVALID_THRESH: u8,
                VBUS_OVERRIDE_EN: bool,
                SESSEND_OVERRIDE: bool,
                BVALID_OVERRIDE: bool,
                AVALID_OVERRIDE: bool,
                VBUSVALID_OVERRIDE: bool,
                VBUSVALID_SEL: bool,
                VBUS_SOURCE_SEL: u8,
                ID_OVERRIDE_EN: bool,
                ID_OVERRIDE: bool,
                EXT_ID_OVERRIDE_EN: bool,
                EXT_VBUS_OVERRIDE_EN: bool,
                VBUSVALID_TO_B: bool,
                VBUSVALID_PWRUP_CMPS: u8,
                DISCHARGE_VBUS: bool,
            }
            let proxy = USB1_VBUS_DETECT_CLR {
                VBUSVALID_THRESH: self.VBUSVALID_THRESH(),
                VBUS_OVERRIDE_EN: self.VBUS_OVERRIDE_EN(),
                SESSEND_OVERRIDE: self.SESSEND_OVERRIDE(),
                BVALID_OVERRIDE: self.BVALID_OVERRIDE(),
                AVALID_OVERRIDE: self.AVALID_OVERRIDE(),
                VBUSVALID_OVERRIDE: self.VBUSVALID_OVERRIDE(),
                VBUSVALID_SEL: self.VBUSVALID_SEL(),
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                ID_OVERRIDE_EN: self.ID_OVERRIDE_EN(),
                ID_OVERRIDE: self.ID_OVERRIDE(),
                EXT_ID_OVERRIDE_EN: self.EXT_ID_OVERRIDE_EN(),
                EXT_VBUS_OVERRIDE_EN: self.EXT_VBUS_OVERRIDE_EN(),
                VBUSVALID_TO_B: self.VBUSVALID_TO_B(),
                VBUSVALID_PWRUP_CMPS: self.VBUSVALID_PWRUP_CMPS(),
                DISCHARGE_VBUS: self.DISCHARGE_VBUS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DETECT_SET(pub u32);
    impl USB1_VBUS_DETECT_SET {
        #[inline(always)]
        pub const fn VBUSVALID_THRESH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_THRESH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SESSEND_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn BVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn AVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_SEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_TO_B(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_TO_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn DISCHARGE_VBUS(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DISCHARGE_VBUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for USB1_VBUS_DETECT_SET {
        #[inline(always)]
        fn default() -> USB1_VBUS_DETECT_SET {
            USB1_VBUS_DETECT_SET(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DETECT_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DETECT_SET")
                .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
                .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
                .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
                .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
                .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
                .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
                .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
                .field("ID_OVERRIDE", &self.ID_OVERRIDE())
                .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
                .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
                .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
                .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
                .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DETECT_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DETECT_SET {
                VBUSVALID_THRESH: u8,
                VBUS_OVERRIDE_EN: bool,
                SESSEND_OVERRIDE: bool,
                BVALID_OVERRIDE: bool,
                AVALID_OVERRIDE: bool,
                VBUSVALID_OVERRIDE: bool,
                VBUSVALID_SEL: bool,
                VBUS_SOURCE_SEL: u8,
                ID_OVERRIDE_EN: bool,
                ID_OVERRIDE: bool,
                EXT_ID_OVERRIDE_EN: bool,
                EXT_VBUS_OVERRIDE_EN: bool,
                VBUSVALID_TO_B: bool,
                VBUSVALID_PWRUP_CMPS: u8,
                DISCHARGE_VBUS: bool,
            }
            let proxy = USB1_VBUS_DETECT_SET {
                VBUSVALID_THRESH: self.VBUSVALID_THRESH(),
                VBUS_OVERRIDE_EN: self.VBUS_OVERRIDE_EN(),
                SESSEND_OVERRIDE: self.SESSEND_OVERRIDE(),
                BVALID_OVERRIDE: self.BVALID_OVERRIDE(),
                AVALID_OVERRIDE: self.AVALID_OVERRIDE(),
                VBUSVALID_OVERRIDE: self.VBUSVALID_OVERRIDE(),
                VBUSVALID_SEL: self.VBUSVALID_SEL(),
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                ID_OVERRIDE_EN: self.ID_OVERRIDE_EN(),
                ID_OVERRIDE: self.ID_OVERRIDE(),
                EXT_ID_OVERRIDE_EN: self.EXT_ID_OVERRIDE_EN(),
                EXT_VBUS_OVERRIDE_EN: self.EXT_VBUS_OVERRIDE_EN(),
                VBUSVALID_TO_B: self.VBUSVALID_TO_B(),
                VBUSVALID_PWRUP_CMPS: self.VBUSVALID_PWRUP_CMPS(),
                DISCHARGE_VBUS: self.DISCHARGE_VBUS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DETECT_TOG(pub u32);
    impl USB1_VBUS_DETECT_TOG {
        #[inline(always)]
        pub const fn VBUSVALID_THRESH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_THRESH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SESSEND_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn BVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn AVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_SEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ID_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ID_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_TO_B(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUSVALID_TO_B(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn DISCHARGE_VBUS(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DISCHARGE_VBUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for USB1_VBUS_DETECT_TOG {
        #[inline(always)]
        fn default() -> USB1_VBUS_DETECT_TOG {
            USB1_VBUS_DETECT_TOG(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DETECT_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DETECT_TOG")
                .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
                .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
                .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
                .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
                .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
                .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
                .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
                .field("ID_OVERRIDE", &self.ID_OVERRIDE())
                .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
                .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
                .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
                .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
                .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DETECT_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DETECT_TOG {
                VBUSVALID_THRESH: u8,
                VBUS_OVERRIDE_EN: bool,
                SESSEND_OVERRIDE: bool,
                BVALID_OVERRIDE: bool,
                AVALID_OVERRIDE: bool,
                VBUSVALID_OVERRIDE: bool,
                VBUSVALID_SEL: bool,
                VBUS_SOURCE_SEL: u8,
                ID_OVERRIDE_EN: bool,
                ID_OVERRIDE: bool,
                EXT_ID_OVERRIDE_EN: bool,
                EXT_VBUS_OVERRIDE_EN: bool,
                VBUSVALID_TO_B: bool,
                VBUSVALID_PWRUP_CMPS: u8,
                DISCHARGE_VBUS: bool,
            }
            let proxy = USB1_VBUS_DETECT_TOG {
                VBUSVALID_THRESH: self.VBUSVALID_THRESH(),
                VBUS_OVERRIDE_EN: self.VBUS_OVERRIDE_EN(),
                SESSEND_OVERRIDE: self.SESSEND_OVERRIDE(),
                BVALID_OVERRIDE: self.BVALID_OVERRIDE(),
                AVALID_OVERRIDE: self.AVALID_OVERRIDE(),
                VBUSVALID_OVERRIDE: self.VBUSVALID_OVERRIDE(),
                VBUSVALID_SEL: self.VBUSVALID_SEL(),
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                ID_OVERRIDE_EN: self.ID_OVERRIDE_EN(),
                ID_OVERRIDE: self.ID_OVERRIDE(),
                EXT_ID_OVERRIDE_EN: self.EXT_ID_OVERRIDE_EN(),
                EXT_VBUS_OVERRIDE_EN: self.EXT_VBUS_OVERRIDE_EN(),
                VBUSVALID_TO_B: self.VBUSVALID_TO_B(),
                VBUSVALID_PWRUP_CMPS: self.VBUSVALID_PWRUP_CMPS(),
                DISCHARGE_VBUS: self.DISCHARGE_VBUS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DET_STAT(pub u32);
    impl USB1_VBUS_DET_STAT {
        #[inline(always)]
        pub const fn SESSEND(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BVALID(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVALID(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID_3V(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID_3V(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EXT_ID(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for USB1_VBUS_DET_STAT {
        #[inline(always)]
        fn default() -> USB1_VBUS_DET_STAT {
            USB1_VBUS_DET_STAT(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DET_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DET_STAT")
                .field("SESSEND", &self.SESSEND())
                .field("BVALID", &self.BVALID())
                .field("AVALID", &self.AVALID())
                .field("VBUS_VALID", &self.VBUS_VALID())
                .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
                .field("EXT_ID", &self.EXT_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DET_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DET_STAT {
                SESSEND: bool,
                BVALID: bool,
                AVALID: bool,
                VBUS_VALID: bool,
                VBUS_VALID_3V: bool,
                EXT_ID: bool,
            }
            let proxy = USB1_VBUS_DET_STAT {
                SESSEND: self.SESSEND(),
                BVALID: self.BVALID(),
                AVALID: self.AVALID(),
                VBUS_VALID: self.VBUS_VALID(),
                VBUS_VALID_3V: self.VBUS_VALID_3V(),
                EXT_ID: self.EXT_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DET_STAT_CLR(pub u32);
    impl USB1_VBUS_DET_STAT_CLR {
        #[inline(always)]
        pub const fn SESSEND(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BVALID(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVALID(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID_3V(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID_3V(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EXT_ID(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for USB1_VBUS_DET_STAT_CLR {
        #[inline(always)]
        fn default() -> USB1_VBUS_DET_STAT_CLR {
            USB1_VBUS_DET_STAT_CLR(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DET_STAT_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DET_STAT_CLR")
                .field("SESSEND", &self.SESSEND())
                .field("BVALID", &self.BVALID())
                .field("AVALID", &self.AVALID())
                .field("VBUS_VALID", &self.VBUS_VALID())
                .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
                .field("EXT_ID", &self.EXT_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DET_STAT_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DET_STAT_CLR {
                SESSEND: bool,
                BVALID: bool,
                AVALID: bool,
                VBUS_VALID: bool,
                VBUS_VALID_3V: bool,
                EXT_ID: bool,
            }
            let proxy = USB1_VBUS_DET_STAT_CLR {
                SESSEND: self.SESSEND(),
                BVALID: self.BVALID(),
                AVALID: self.AVALID(),
                VBUS_VALID: self.VBUS_VALID(),
                VBUS_VALID_3V: self.VBUS_VALID_3V(),
                EXT_ID: self.EXT_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DET_STAT_SET(pub u32);
    impl USB1_VBUS_DET_STAT_SET {
        #[inline(always)]
        pub const fn SESSEND(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BVALID(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVALID(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID_3V(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID_3V(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EXT_ID(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for USB1_VBUS_DET_STAT_SET {
        #[inline(always)]
        fn default() -> USB1_VBUS_DET_STAT_SET {
            USB1_VBUS_DET_STAT_SET(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DET_STAT_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DET_STAT_SET")
                .field("SESSEND", &self.SESSEND())
                .field("BVALID", &self.BVALID())
                .field("AVALID", &self.AVALID())
                .field("VBUS_VALID", &self.VBUS_VALID())
                .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
                .field("EXT_ID", &self.EXT_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DET_STAT_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DET_STAT_SET {
                SESSEND: bool,
                BVALID: bool,
                AVALID: bool,
                VBUS_VALID: bool,
                VBUS_VALID_3V: bool,
                EXT_ID: bool,
            }
            let proxy = USB1_VBUS_DET_STAT_SET {
                SESSEND: self.SESSEND(),
                BVALID: self.BVALID(),
                AVALID: self.AVALID(),
                VBUS_VALID: self.VBUS_VALID(),
                VBUS_VALID_3V: self.VBUS_VALID_3V(),
                EXT_ID: self.EXT_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "VBUS Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USB1_VBUS_DET_STAT_TOG(pub u32);
    impl USB1_VBUS_DET_STAT_TOG {
        #[inline(always)]
        pub const fn SESSEND(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESSEND(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BVALID(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVALID(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VBUS_VALID_3V(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_VALID_3V(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn EXT_ID(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXT_ID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for USB1_VBUS_DET_STAT_TOG {
        #[inline(always)]
        fn default() -> USB1_VBUS_DET_STAT_TOG {
            USB1_VBUS_DET_STAT_TOG(0)
        }
    }
    impl core::fmt::Debug for USB1_VBUS_DET_STAT_TOG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USB1_VBUS_DET_STAT_TOG")
                .field("SESSEND", &self.SESSEND())
                .field("BVALID", &self.BVALID())
                .field("AVALID", &self.AVALID())
                .field("VBUS_VALID", &self.VBUS_VALID())
                .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
                .field("EXT_ID", &self.EXT_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USB1_VBUS_DET_STAT_TOG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USB1_VBUS_DET_STAT_TOG {
                SESSEND: bool,
                BVALID: bool,
                AVALID: bool,
                VBUS_VALID: bool,
                VBUS_VALID_3V: bool,
                EXT_ID: bool,
            }
            let proxy = USB1_VBUS_DET_STAT_TOG {
                SESSEND: self.SESSEND(),
                BVALID: self.BVALID(),
                AVALID: self.AVALID(),
                VBUS_VALID: self.VBUS_VALID(),
                VBUS_VALID_3V: self.VBUS_VALID_3V(),
                EXT_ID: self.EXT_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERSION(pub u32);
    impl VERSION {
        #[inline(always)]
        pub const fn STEP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STEP(&mut self, val: u16) {
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
    impl Default for VERSION {
        #[inline(always)]
        fn default() -> VERSION {
            VERSION(0)
        }
    }
    impl core::fmt::Debug for VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERSION")
                .field("STEP", &self.STEP())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERSION {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VERSION {
                STEP: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERSION {
                STEP: self.STEP(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
