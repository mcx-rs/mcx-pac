#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPOINT {
    ptr: *mut u8,
}
unsafe impl Send for ENDPOINT {}
unsafe impl Sync for ENDPOINT {}
impl ENDPOINT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ENDPT(self) -> crate::common::Reg<regs::ENDPOINT_ENDPT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB {
    ptr: *mut u8,
}
unsafe impl Send for USB {}
unsafe impl Sync for USB {}
impl USB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn PERID(self) -> crate::common::Reg<regs::PERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn IDCOMP(self) -> crate::common::Reg<regs::IDCOMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn REV(self) -> crate::common::Reg<regs::REV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn OTGCTL(self) -> crate::common::Reg<regs::OTGCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn ISTAT(self) -> crate::common::Reg<regs::ISTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<regs::INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn ERRSTAT(self) -> crate::common::Reg<regs::ERRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn ERREN(self) -> crate::common::Reg<regs::ERREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn ADDR(self) -> crate::common::Reg<regs::ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn BDTPAGE1(self) -> crate::common::Reg<regs::BDTPAGE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn FRMNUML(self) -> crate::common::Reg<regs::FRMNUML, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn FRMNUMH(self) -> crate::common::Reg<regs::FRMNUMH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn BDTPAGE2(self) -> crate::common::Reg<regs::BDTPAGE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn BDTPAGE3(self) -> crate::common::Reg<regs::BDTPAGE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn ENDPOINT(self, n: usize) -> ENDPOINT {
        assert!(n < 16usize);
        unsafe { ENDPOINT::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn USBCTRL(self) -> crate::common::Reg<regs::USBCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn OBSERVE(self) -> crate::common::Reg<regs::OBSERVE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn USBTRC0(self) -> crate::common::Reg<regs::USBTRC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn KEEP_ALIVE_CTRL_RSVD(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn KEEP_ALIVE_WKCTRL_RSVD(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn MISCCTRL(self) -> crate::common::Reg<regs::MISCCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn STALL_IL_DIS(self) -> crate::common::Reg<regs::STALL_IL_DIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn STALL_IH_DIS(self) -> crate::common::Reg<regs::STALL_IH_DIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn STALL_OL_DIS(self) -> crate::common::Reg<regs::STALL_OL_DIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn STALL_OH_DIS(self) -> crate::common::Reg<regs::STALL_OH_DIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[inline(always)]
    pub const fn CLK_RECOVER_CTRL(
        self,
    ) -> crate::common::Reg<regs::CLK_RECOVER_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn CLK_RECOVER_IRC_EN(
        self,
    ) -> crate::common::Reg<regs::CLK_RECOVER_IRC_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn CLK_RECOVER_INT_EN(
        self,
    ) -> crate::common::Reg<regs::CLK_RECOVER_INT_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn CLK_RECOVER_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::CLK_RECOVER_INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADDR(pub u32);
    impl ADDR {
        #[inline(always)]
        pub const fn ADDR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for ADDR {
        #[inline(always)]
        fn default() -> ADDR {
            ADDR(0)
        }
    }
    impl core::fmt::Debug for ADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADDR").field("ADDR", &self.ADDR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ADDR {
                ADDR: u8,
            }
            let proxy = ADDR { ADDR: self.ADDR() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "BDT Page 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BDTPAGE1(pub u32);
    impl BDTPAGE1 {
        #[inline(always)]
        pub const fn BDTBA(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BDTBA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for BDTPAGE1 {
        #[inline(always)]
        fn default() -> BDTPAGE1 {
            BDTPAGE1(0)
        }
    }
    impl core::fmt::Debug for BDTPAGE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BDTPAGE1")
                .field("BDTBA", &self.BDTBA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BDTPAGE1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BDTPAGE1 {
                BDTBA: u8,
            }
            let proxy = BDTPAGE1 {
                BDTBA: self.BDTBA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "BDT Page 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BDTPAGE2(pub u32);
    impl BDTPAGE2 {
        #[inline(always)]
        pub const fn BDTBA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_BDTBA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for BDTPAGE2 {
        #[inline(always)]
        fn default() -> BDTPAGE2 {
            BDTPAGE2(0)
        }
    }
    impl core::fmt::Debug for BDTPAGE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BDTPAGE2")
                .field("BDTBA", &self.BDTBA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BDTPAGE2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BDTPAGE2 {
                BDTBA: u8,
            }
            let proxy = BDTPAGE2 {
                BDTBA: self.BDTBA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "BDT Page 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BDTPAGE3(pub u32);
    impl BDTPAGE3 {
        #[inline(always)]
        pub const fn BDTBA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_BDTBA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for BDTPAGE3 {
        #[inline(always)]
        fn default() -> BDTPAGE3 {
            BDTPAGE3(0)
        }
    }
    impl core::fmt::Debug for BDTPAGE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BDTPAGE3")
                .field("BDTBA", &self.BDTBA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BDTPAGE3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BDTPAGE3 {
                BDTBA: u8,
            }
            let proxy = BDTPAGE3 {
                BDTBA: self.BDTBA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB Clock Recovery Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_CTRL(pub u32);
    impl CLK_RECOVER_CTRL {
        #[inline(always)]
        pub const fn TRIM_INIT_VAL_SEL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIM_INIT_VAL_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RESTART_IFRTRIM_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESTART_IFRTRIM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RESET_RESUME_ROUGH_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET_RESUME_ROUGH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CLOCK_RECOVER_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLOCK_RECOVER_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CLK_RECOVER_CTRL {
        #[inline(always)]
        fn default() -> CLK_RECOVER_CTRL {
            CLK_RECOVER_CTRL(0)
        }
    }
    impl core::fmt::Debug for CLK_RECOVER_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLK_RECOVER_CTRL")
                .field("TRIM_INIT_VAL_SEL", &self.TRIM_INIT_VAL_SEL())
                .field("RESTART_IFRTRIM_EN", &self.RESTART_IFRTRIM_EN())
                .field("RESET_RESUME_ROUGH_EN", &self.RESET_RESUME_ROUGH_EN())
                .field("CLOCK_RECOVER_EN", &self.CLOCK_RECOVER_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLK_RECOVER_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLK_RECOVER_CTRL {
                TRIM_INIT_VAL_SEL: bool,
                RESTART_IFRTRIM_EN: bool,
                RESET_RESUME_ROUGH_EN: bool,
                CLOCK_RECOVER_EN: bool,
            }
            let proxy = CLK_RECOVER_CTRL {
                TRIM_INIT_VAL_SEL: self.TRIM_INIT_VAL_SEL(),
                RESTART_IFRTRIM_EN: self.RESTART_IFRTRIM_EN(),
                RESET_RESUME_ROUGH_EN: self.RESET_RESUME_ROUGH_EN(),
                CLOCK_RECOVER_EN: self.CLOCK_RECOVER_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_INT_EN(pub u32);
    impl CLK_RECOVER_INT_EN {
        #[inline(always)]
        pub const fn OVF_ERROR_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVF_ERROR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CLK_RECOVER_INT_EN {
        #[inline(always)]
        fn default() -> CLK_RECOVER_INT_EN {
            CLK_RECOVER_INT_EN(0)
        }
    }
    impl core::fmt::Debug for CLK_RECOVER_INT_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLK_RECOVER_INT_EN")
                .field("OVF_ERROR_EN", &self.OVF_ERROR_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLK_RECOVER_INT_EN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLK_RECOVER_INT_EN {
                OVF_ERROR_EN: bool,
            }
            let proxy = CLK_RECOVER_INT_EN {
                OVF_ERROR_EN: self.OVF_ERROR_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock Recovery Separated Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_INT_STATUS(pub u32);
    impl CLK_RECOVER_INT_STATUS {
        #[inline(always)]
        pub const fn OVF_ERROR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVF_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CLK_RECOVER_INT_STATUS {
        #[inline(always)]
        fn default() -> CLK_RECOVER_INT_STATUS {
            CLK_RECOVER_INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for CLK_RECOVER_INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLK_RECOVER_INT_STATUS")
                .field("OVF_ERROR", &self.OVF_ERROR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLK_RECOVER_INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLK_RECOVER_INT_STATUS {
                OVF_ERROR: bool,
            }
            let proxy = CLK_RECOVER_INT_STATUS {
                OVF_ERROR: self.OVF_ERROR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FIRC Oscillator Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_IRC_EN(pub u32);
    impl CLK_RECOVER_IRC_EN {
        #[inline(always)]
        pub const fn IRC_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for CLK_RECOVER_IRC_EN {
        #[inline(always)]
        fn default() -> CLK_RECOVER_IRC_EN {
            CLK_RECOVER_IRC_EN(0)
        }
    }
    impl core::fmt::Debug for CLK_RECOVER_IRC_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLK_RECOVER_IRC_EN")
                .field("IRC_EN", &self.IRC_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLK_RECOVER_IRC_EN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CLK_RECOVER_IRC_EN {
                IRC_EN: bool,
            }
            let proxy = CLK_RECOVER_IRC_EN {
                IRC_EN: self.IRC_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB OTG Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONTROL(pub u32);
    impl CONTROL {
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VBUS_SOURCE_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SESS_VLD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SESS_VLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DPPULLUPNONOTG(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPPULLUPNONOTG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CONTROL {
        #[inline(always)]
        fn default() -> CONTROL {
            CONTROL(0)
        }
    }
    impl core::fmt::Debug for CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONTROL")
                .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
                .field("SESS_VLD", &self.SESS_VLD())
                .field("DPPULLUPNONOTG", &self.DPPULLUPNONOTG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONTROL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONTROL {
                VBUS_SOURCE_SEL: bool,
                SESS_VLD: bool,
                DPPULLUPNONOTG: bool,
            }
            let proxy = CONTROL {
                VBUS_SOURCE_SEL: self.VBUS_SOURCE_SEL(),
                SESS_VLD: self.SESS_VLD(),
                DPPULLUPNONOTG: self.DPPULLUPNONOTG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTL(pub u32);
    impl CTL {
        #[inline(always)]
        pub const fn USBENSOFEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBENSOFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ODDRST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ODDRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RESUME(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TXSUSPENDTOKENBUSY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXSUSPENDTOKENBUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SE0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for CTL {
        #[inline(always)]
        fn default() -> CTL {
            CTL(0)
        }
    }
    impl core::fmt::Debug for CTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTL")
                .field("USBENSOFEN", &self.USBENSOFEN())
                .field("ODDRST", &self.ODDRST())
                .field("RESUME", &self.RESUME())
                .field("TXSUSPENDTOKENBUSY", &self.TXSUSPENDTOKENBUSY())
                .field("SE0", &self.SE0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTL {
                USBENSOFEN: bool,
                ODDRST: bool,
                RESUME: bool,
                TXSUSPENDTOKENBUSY: bool,
                SE0: bool,
            }
            let proxy = CTL {
                USBENSOFEN: self.USBENSOFEN(),
                ODDRST: self.ODDRST(),
                RESUME: self.RESUME(),
                TXSUSPENDTOKENBUSY: self.TXSUSPENDTOKENBUSY(),
                SE0: self.SE0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Endpoint Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPOINT_ENDPT(pub u32);
    impl ENDPOINT_ENDPT {
        #[inline(always)]
        pub const fn EPHSHK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPHSHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EPSTALL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EPTXEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPTXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn EPRXEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPRXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn EPCTLDIS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPCTLDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for ENDPOINT_ENDPT {
        #[inline(always)]
        fn default() -> ENDPOINT_ENDPT {
            ENDPOINT_ENDPT(0)
        }
    }
    impl core::fmt::Debug for ENDPOINT_ENDPT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ENDPOINT_ENDPT")
                .field("EPHSHK", &self.EPHSHK())
                .field("EPSTALL", &self.EPSTALL())
                .field("EPTXEN", &self.EPTXEN())
                .field("EPRXEN", &self.EPRXEN())
                .field("EPCTLDIS", &self.EPCTLDIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPOINT_ENDPT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ENDPOINT_ENDPT {
                EPHSHK: bool,
                EPSTALL: bool,
                EPTXEN: bool,
                EPRXEN: bool,
                EPCTLDIS: bool,
            }
            let proxy = ENDPOINT_ENDPT {
                EPHSHK: self.EPHSHK(),
                EPSTALL: self.EPSTALL(),
                EPTXEN: self.EPTXEN(),
                EPRXEN: self.EPRXEN(),
                EPCTLDIS: self.EPCTLDIS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERREN(pub u32);
    impl ERREN {
        #[inline(always)]
        pub const fn PIDERREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PIDERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CRC5EOFEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC5EOFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CRC16EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC16EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DFN8EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DFN8EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BTOERREN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTOERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DMAERREN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OWNERREN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OWNERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn BTSERREN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTSERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for ERREN {
        #[inline(always)]
        fn default() -> ERREN {
            ERREN(0)
        }
    }
    impl core::fmt::Debug for ERREN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERREN")
                .field("PIDERREN", &self.PIDERREN())
                .field("CRC5EOFEN", &self.CRC5EOFEN())
                .field("CRC16EN", &self.CRC16EN())
                .field("DFN8EN", &self.DFN8EN())
                .field("BTOERREN", &self.BTOERREN())
                .field("DMAERREN", &self.DMAERREN())
                .field("OWNERREN", &self.OWNERREN())
                .field("BTSERREN", &self.BTSERREN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ERREN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ERREN {
                PIDERREN: bool,
                CRC5EOFEN: bool,
                CRC16EN: bool,
                DFN8EN: bool,
                BTOERREN: bool,
                DMAERREN: bool,
                OWNERREN: bool,
                BTSERREN: bool,
            }
            let proxy = ERREN {
                PIDERREN: self.PIDERREN(),
                CRC5EOFEN: self.CRC5EOFEN(),
                CRC16EN: self.CRC16EN(),
                DFN8EN: self.DFN8EN(),
                BTOERREN: self.BTOERREN(),
                DMAERREN: self.DMAERREN(),
                OWNERREN: self.OWNERREN(),
                BTSERREN: self.BTSERREN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Error Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERRSTAT(pub u32);
    impl ERRSTAT {
        #[inline(always)]
        pub const fn PIDERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PIDERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CRC5EOF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC5EOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CRC16(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CRC16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DFN8(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DFN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BTOERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTOERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DMAERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMAERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OWNERR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OWNERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn BTSERR(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BTSERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for ERRSTAT {
        #[inline(always)]
        fn default() -> ERRSTAT {
            ERRSTAT(0)
        }
    }
    impl core::fmt::Debug for ERRSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ERRSTAT")
                .field("PIDERR", &self.PIDERR())
                .field("CRC5EOF", &self.CRC5EOF())
                .field("CRC16", &self.CRC16())
                .field("DFN8", &self.DFN8())
                .field("BTOERR", &self.BTOERR())
                .field("DMAERR", &self.DMAERR())
                .field("OWNERR", &self.OWNERR())
                .field("BTSERR", &self.BTSERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ERRSTAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ERRSTAT {
                PIDERR: bool,
                CRC5EOF: bool,
                CRC16: bool,
                DFN8: bool,
                BTOERR: bool,
                DMAERR: bool,
                OWNERR: bool,
                BTSERR: bool,
            }
            let proxy = ERRSTAT {
                PIDERR: self.PIDERR(),
                CRC5EOF: self.CRC5EOF(),
                CRC16: self.CRC16(),
                DFN8: self.DFN8(),
                BTOERR: self.BTOERR(),
                DMAERR: self.DMAERR(),
                OWNERR: self.OWNERR(),
                BTSERR: self.BTSERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Frame Number Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRMNUMH(pub u32);
    impl FRMNUMH {
        #[inline(always)]
        pub const fn FRM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for FRMNUMH {
        #[inline(always)]
        fn default() -> FRMNUMH {
            FRMNUMH(0)
        }
    }
    impl core::fmt::Debug for FRMNUMH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRMNUMH").field("FRM", &self.FRM()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRMNUMH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRMNUMH {
                FRM: u8,
            }
            let proxy = FRMNUMH { FRM: self.FRM() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Frame Number Register Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRMNUML(pub u32);
    impl FRMNUML {
        #[inline(always)]
        pub const fn FRM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for FRMNUML {
        #[inline(always)]
        fn default() -> FRMNUML {
            FRMNUML(0)
        }
    }
    impl core::fmt::Debug for FRMNUML {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRMNUML").field("FRM", &self.FRM()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRMNUML {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRMNUML {
                FRM: u8,
            }
            let proxy = FRMNUML { FRM: self.FRM() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral ID Complement"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IDCOMP(pub u32);
    impl IDCOMP {
        #[inline(always)]
        pub const fn NID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_NID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for IDCOMP {
        #[inline(always)]
        fn default() -> IDCOMP {
            IDCOMP(0)
        }
    }
    impl core::fmt::Debug for IDCOMP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IDCOMP").field("NID", &self.NID()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IDCOMP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IDCOMP {
                NID: u8,
            }
            let proxy = IDCOMP { NID: self.NID() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTEN(pub u32);
    impl INTEN {
        #[inline(always)]
        pub const fn USBRSTEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBRSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ERROREN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SOFTOKEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOFTOKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TOKDNEEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TOKDNEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SLEEPEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLEEPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RESUMEEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALLEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALLEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for INTEN {
        #[inline(always)]
        fn default() -> INTEN {
            INTEN(0)
        }
    }
    impl core::fmt::Debug for INTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTEN")
                .field("USBRSTEN", &self.USBRSTEN())
                .field("ERROREN", &self.ERROREN())
                .field("SOFTOKEN", &self.SOFTOKEN())
                .field("TOKDNEEN", &self.TOKDNEEN())
                .field("SLEEPEN", &self.SLEEPEN())
                .field("RESUMEEN", &self.RESUMEEN())
                .field("STALLEN", &self.STALLEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INTEN {
                USBRSTEN: bool,
                ERROREN: bool,
                SOFTOKEN: bool,
                TOKDNEEN: bool,
                SLEEPEN: bool,
                RESUMEEN: bool,
                STALLEN: bool,
            }
            let proxy = INTEN {
                USBRSTEN: self.USBRSTEN(),
                ERROREN: self.ERROREN(),
                SOFTOKEN: self.SOFTOKEN(),
                TOKDNEEN: self.TOKDNEEN(),
                SLEEPEN: self.SLEEPEN(),
                RESUMEEN: self.RESUMEEN(),
                STALLEN: self.STALLEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISTAT(pub u32);
    impl ISTAT {
        #[inline(always)]
        pub const fn USBRST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ERROR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SOFTOK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOFTOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TOKDNE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TOKDNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SLEEP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SLEEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn RESUME(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for ISTAT {
        #[inline(always)]
        fn default() -> ISTAT {
            ISTAT(0)
        }
    }
    impl core::fmt::Debug for ISTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISTAT")
                .field("USBRST", &self.USBRST())
                .field("ERROR", &self.ERROR())
                .field("SOFTOK", &self.SOFTOK())
                .field("TOKDNE", &self.TOKDNE())
                .field("SLEEP", &self.SLEEP())
                .field("RESUME", &self.RESUME())
                .field("STALL", &self.STALL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISTAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ISTAT {
                USBRST: bool,
                ERROR: bool,
                SOFTOK: bool,
                TOKDNE: bool,
                SLEEP: bool,
                RESUME: bool,
                STALL: bool,
            }
            let proxy = ISTAT {
                USBRST: self.USBRST(),
                ERROR: self.ERROR(),
                SOFTOK: self.SOFTOK(),
                TOKDNE: self.TOKDNE(),
                SLEEP: self.SLEEP(),
                RESUME: self.RESUME(),
                STALL: self.STALL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Miscellaneous Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISCCTRL(pub u32);
    impl MISCCTRL {
        #[inline(always)]
        pub const fn OWNERRISODIS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OWNERRISODIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VREDG_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VREDG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VFEDG_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VFEDG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn STL_ADJ_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STL_ADJ_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for MISCCTRL {
        #[inline(always)]
        fn default() -> MISCCTRL {
            MISCCTRL(0)
        }
    }
    impl core::fmt::Debug for MISCCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MISCCTRL")
                .field("OWNERRISODIS", &self.OWNERRISODIS())
                .field("VREDG_EN", &self.VREDG_EN())
                .field("VFEDG_EN", &self.VFEDG_EN())
                .field("STL_ADJ_EN", &self.STL_ADJ_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MISCCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MISCCTRL {
                OWNERRISODIS: bool,
                VREDG_EN: bool,
                VFEDG_EN: bool,
                STL_ADJ_EN: bool,
            }
            let proxy = MISCCTRL {
                OWNERRISODIS: self.OWNERRISODIS(),
                VREDG_EN: self.VREDG_EN(),
                VFEDG_EN: self.VFEDG_EN(),
                STL_ADJ_EN: self.STL_ADJ_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB OTG Observe"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OBSERVE(pub u32);
    impl OBSERVE {
        #[inline(always)]
        pub const fn DMPD(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DPPD(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DPPU(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for OBSERVE {
        #[inline(always)]
        fn default() -> OBSERVE {
            OBSERVE(0)
        }
    }
    impl core::fmt::Debug for OBSERVE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OBSERVE")
                .field("DMPD", &self.DMPD())
                .field("DPPD", &self.DPPD())
                .field("DPPU", &self.DPPU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OBSERVE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OBSERVE {
                DMPD: bool,
                DPPD: bool,
                DPPU: bool,
            }
            let proxy = OBSERVE {
                DMPD: self.DMPD(),
                DPPD: self.DPPD(),
                DPPU: self.DPPU(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "OTG Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGCTL(pub u32);
    impl OTGCTL {
        #[inline(always)]
        pub const fn DPHIGH(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPHIGH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for OTGCTL {
        #[inline(always)]
        fn default() -> OTGCTL {
            OTGCTL(0)
        }
    }
    impl core::fmt::Debug for OTGCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTGCTL")
                .field("DPHIGH", &self.DPHIGH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OTGCTL {
                DPHIGH: bool,
            }
            let proxy = OTGCTL {
                DPHIGH: self.DPHIGH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PERID(pub u32);
    impl PERID {
        #[inline(always)]
        pub const fn ID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for PERID {
        #[inline(always)]
        fn default() -> PERID {
            PERID(0)
        }
    }
    impl core::fmt::Debug for PERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PERID").field("ID", &self.ID()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PERID {
                ID: u8,
            }
            let proxy = PERID { ID: self.ID() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Revision"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REV(pub u32);
    impl REV {
        #[inline(always)]
        pub const fn REV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for REV {
        #[inline(always)]
        fn default() -> REV {
            REV(0)
        }
    }
    impl core::fmt::Debug for REV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REV").field("REV", &self.REV()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REV {
                REV: u8,
            }
            let proxy = REV { REV: self.REV() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_IH_DIS(pub u32);
    impl STALL_IH_DIS {
        #[inline(always)]
        pub const fn STALL_I_DIS8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for STALL_IH_DIS {
        #[inline(always)]
        fn default() -> STALL_IH_DIS {
            STALL_IH_DIS(0)
        }
    }
    impl core::fmt::Debug for STALL_IH_DIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STALL_IH_DIS")
                .field("STALL_I_DIS8", &self.STALL_I_DIS8())
                .field("STALL_I_DIS9", &self.STALL_I_DIS9())
                .field("STALL_I_DIS10", &self.STALL_I_DIS10())
                .field("STALL_I_DIS11", &self.STALL_I_DIS11())
                .field("STALL_I_DIS12", &self.STALL_I_DIS12())
                .field("STALL_I_DIS13", &self.STALL_I_DIS13())
                .field("STALL_I_DIS14", &self.STALL_I_DIS14())
                .field("STALL_I_DIS15", &self.STALL_I_DIS15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STALL_IH_DIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STALL_IH_DIS {
                STALL_I_DIS8: bool,
                STALL_I_DIS9: bool,
                STALL_I_DIS10: bool,
                STALL_I_DIS11: bool,
                STALL_I_DIS12: bool,
                STALL_I_DIS13: bool,
                STALL_I_DIS14: bool,
                STALL_I_DIS15: bool,
            }
            let proxy = STALL_IH_DIS {
                STALL_I_DIS8: self.STALL_I_DIS8(),
                STALL_I_DIS9: self.STALL_I_DIS9(),
                STALL_I_DIS10: self.STALL_I_DIS10(),
                STALL_I_DIS11: self.STALL_I_DIS11(),
                STALL_I_DIS12: self.STALL_I_DIS12(),
                STALL_I_DIS13: self.STALL_I_DIS13(),
                STALL_I_DIS14: self.STALL_I_DIS14(),
                STALL_I_DIS15: self.STALL_I_DIS15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_IL_DIS(pub u32);
    impl STALL_IL_DIS {
        #[inline(always)]
        pub const fn STALL_I_DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn STALL_I_DIS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_I_DIS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for STALL_IL_DIS {
        #[inline(always)]
        fn default() -> STALL_IL_DIS {
            STALL_IL_DIS(0)
        }
    }
    impl core::fmt::Debug for STALL_IL_DIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STALL_IL_DIS")
                .field("STALL_I_DIS0", &self.STALL_I_DIS0())
                .field("STALL_I_DIS1", &self.STALL_I_DIS1())
                .field("STALL_I_DIS2", &self.STALL_I_DIS2())
                .field("STALL_I_DIS3", &self.STALL_I_DIS3())
                .field("STALL_I_DIS4", &self.STALL_I_DIS4())
                .field("STALL_I_DIS5", &self.STALL_I_DIS5())
                .field("STALL_I_DIS6", &self.STALL_I_DIS6())
                .field("STALL_I_DIS7", &self.STALL_I_DIS7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STALL_IL_DIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STALL_IL_DIS {
                STALL_I_DIS0: bool,
                STALL_I_DIS1: bool,
                STALL_I_DIS2: bool,
                STALL_I_DIS3: bool,
                STALL_I_DIS4: bool,
                STALL_I_DIS5: bool,
                STALL_I_DIS6: bool,
                STALL_I_DIS7: bool,
            }
            let proxy = STALL_IL_DIS {
                STALL_I_DIS0: self.STALL_I_DIS0(),
                STALL_I_DIS1: self.STALL_I_DIS1(),
                STALL_I_DIS2: self.STALL_I_DIS2(),
                STALL_I_DIS3: self.STALL_I_DIS3(),
                STALL_I_DIS4: self.STALL_I_DIS4(),
                STALL_I_DIS5: self.STALL_I_DIS5(),
                STALL_I_DIS6: self.STALL_I_DIS6(),
                STALL_I_DIS7: self.STALL_I_DIS7(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_OH_DIS(pub u32);
    impl STALL_OH_DIS {
        #[inline(always)]
        pub const fn STALL_O_DIS8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for STALL_OH_DIS {
        #[inline(always)]
        fn default() -> STALL_OH_DIS {
            STALL_OH_DIS(0)
        }
    }
    impl core::fmt::Debug for STALL_OH_DIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STALL_OH_DIS")
                .field("STALL_O_DIS8", &self.STALL_O_DIS8())
                .field("STALL_O_DIS9", &self.STALL_O_DIS9())
                .field("STALL_O_DIS10", &self.STALL_O_DIS10())
                .field("STALL_O_DIS11", &self.STALL_O_DIS11())
                .field("STALL_O_DIS12", &self.STALL_O_DIS12())
                .field("STALL_O_DIS13", &self.STALL_O_DIS13())
                .field("STALL_O_DIS14", &self.STALL_O_DIS14())
                .field("STALL_O_DIS15", &self.STALL_O_DIS15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STALL_OH_DIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STALL_OH_DIS {
                STALL_O_DIS8: bool,
                STALL_O_DIS9: bool,
                STALL_O_DIS10: bool,
                STALL_O_DIS11: bool,
                STALL_O_DIS12: bool,
                STALL_O_DIS13: bool,
                STALL_O_DIS14: bool,
                STALL_O_DIS15: bool,
            }
            let proxy = STALL_OH_DIS {
                STALL_O_DIS8: self.STALL_O_DIS8(),
                STALL_O_DIS9: self.STALL_O_DIS9(),
                STALL_O_DIS10: self.STALL_O_DIS10(),
                STALL_O_DIS11: self.STALL_O_DIS11(),
                STALL_O_DIS12: self.STALL_O_DIS12(),
                STALL_O_DIS13: self.STALL_O_DIS13(),
                STALL_O_DIS14: self.STALL_O_DIS14(),
                STALL_O_DIS15: self.STALL_O_DIS15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_OL_DIS(pub u32);
    impl STALL_OL_DIS {
        #[inline(always)]
        pub const fn STALL_O_DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn STALL_O_DIS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STALL_O_DIS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for STALL_OL_DIS {
        #[inline(always)]
        fn default() -> STALL_OL_DIS {
            STALL_OL_DIS(0)
        }
    }
    impl core::fmt::Debug for STALL_OL_DIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STALL_OL_DIS")
                .field("STALL_O_DIS0", &self.STALL_O_DIS0())
                .field("STALL_O_DIS1", &self.STALL_O_DIS1())
                .field("STALL_O_DIS2", &self.STALL_O_DIS2())
                .field("STALL_O_DIS3", &self.STALL_O_DIS3())
                .field("STALL_O_DIS4", &self.STALL_O_DIS4())
                .field("STALL_O_DIS5", &self.STALL_O_DIS5())
                .field("STALL_O_DIS6", &self.STALL_O_DIS6())
                .field("STALL_O_DIS7", &self.STALL_O_DIS7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STALL_OL_DIS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STALL_OL_DIS {
                STALL_O_DIS0: bool,
                STALL_O_DIS1: bool,
                STALL_O_DIS2: bool,
                STALL_O_DIS3: bool,
                STALL_O_DIS4: bool,
                STALL_O_DIS5: bool,
                STALL_O_DIS6: bool,
                STALL_O_DIS7: bool,
            }
            let proxy = STALL_OL_DIS {
                STALL_O_DIS0: self.STALL_O_DIS0(),
                STALL_O_DIS1: self.STALL_O_DIS1(),
                STALL_O_DIS2: self.STALL_O_DIS2(),
                STALL_O_DIS3: self.STALL_O_DIS3(),
                STALL_O_DIS4: self.STALL_O_DIS4(),
                STALL_O_DIS5: self.STALL_O_DIS5(),
                STALL_O_DIS6: self.STALL_O_DIS6(),
                STALL_O_DIS7: self.STALL_O_DIS7(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[inline(always)]
        pub const fn ODD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ODD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TX(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ENDP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ENDP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for STAT {
        #[inline(always)]
        fn default() -> STAT {
            STAT(0)
        }
    }
    impl core::fmt::Debug for STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STAT")
                .field("ODD", &self.ODD())
                .field("TX", &self.TX())
                .field("ENDP", &self.ENDP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STAT {
                ODD: bool,
                TX: bool,
                ENDP: u8,
            }
            let proxy = STAT {
                ODD: self.ODD(),
                TX: self.TX(),
                ENDP: self.ENDP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBCTRL(pub u32);
    impl USBCTRL {
        #[inline(always)]
        pub const fn DPDM_LANE_REVERSE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DPDM_LANE_REVERSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn UARTSEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UARTSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn UARTCHLS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UARTCHLS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn SUSP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SUSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for USBCTRL {
        #[inline(always)]
        fn default() -> USBCTRL {
            USBCTRL(0)
        }
    }
    impl core::fmt::Debug for USBCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBCTRL")
                .field("DPDM_LANE_REVERSE", &self.DPDM_LANE_REVERSE())
                .field("UARTSEL", &self.UARTSEL())
                .field("UARTCHLS", &self.UARTCHLS())
                .field("PDE", &self.PDE())
                .field("SUSP", &self.SUSP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USBCTRL {
                DPDM_LANE_REVERSE: bool,
                UARTSEL: bool,
                UARTCHLS: bool,
                PDE: bool,
                SUSP: bool,
            }
            let proxy = USBCTRL {
                DPDM_LANE_REVERSE: self.DPDM_LANE_REVERSE(),
                UARTSEL: self.UARTSEL(),
                UARTCHLS: self.UARTCHLS(),
                PDE: self.PDE(),
                SUSP: self.SUSP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "USB Transceiver Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBTRC0(pub u32);
    impl USBTRC0 {
        #[inline(always)]
        pub const fn USB_RESUME_INT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_RESUME_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SYNC_DET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYNC_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn USB_CLK_RECOVERY_INT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USB_CLK_RECOVERY_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn VREDG_DET(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VREDG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn VFEDG_DET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VFEDG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn USBRESMEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBRESMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn VREGIN_STS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VREGIN_STS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn USBRESET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_USBRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for USBTRC0 {
        #[inline(always)]
        fn default() -> USBTRC0 {
            USBTRC0(0)
        }
    }
    impl core::fmt::Debug for USBTRC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("USBTRC0")
                .field("USB_RESUME_INT", &self.USB_RESUME_INT())
                .field("SYNC_DET", &self.SYNC_DET())
                .field("USB_CLK_RECOVERY_INT", &self.USB_CLK_RECOVERY_INT())
                .field("VREDG_DET", &self.VREDG_DET())
                .field("VFEDG_DET", &self.VFEDG_DET())
                .field("USBRESMEN", &self.USBRESMEN())
                .field("VREGIN_STS", &self.VREGIN_STS())
                .field("USBRESET", &self.USBRESET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for USBTRC0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct USBTRC0 {
                USB_RESUME_INT: bool,
                SYNC_DET: bool,
                USB_CLK_RECOVERY_INT: bool,
                VREDG_DET: bool,
                VFEDG_DET: bool,
                USBRESMEN: bool,
                VREGIN_STS: bool,
                USBRESET: bool,
            }
            let proxy = USBTRC0 {
                USB_RESUME_INT: self.USB_RESUME_INT(),
                SYNC_DET: self.SYNC_DET(),
                USB_CLK_RECOVERY_INT: self.USB_CLK_RECOVERY_INT(),
                VREDG_DET: self.VREDG_DET(),
                VFEDG_DET: self.VFEDG_DET(),
                USBRESMEN: self.USBRESMEN(),
                VREGIN_STS: self.VREGIN_STS(),
                USBRESET: self.USBRESET(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
