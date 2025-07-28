#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
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
    pub const fn REV(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ADDINFO(self) -> crate::common::Reg<regs::ADDINFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn OTGISTAT(self) -> crate::common::Reg<regs::OTGISTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn OTGICR(self) -> crate::common::Reg<regs::OTGICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn OTGSTAT(self) -> crate::common::Reg<regs::OTGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
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
    pub const fn FRMNUML(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn FRMNUMH(self) -> crate::common::Reg<regs::FRMNUMH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn TOKEN(self) -> crate::common::Reg<regs::TOKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn SOFTHLD(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn BDTPAGE2(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn BDTPAGE3(self) -> crate::common::Reg<u8, crate::common::RW> {
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
    pub const fn USBFRMADJUST(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
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
    #[doc = "Peripheral Additional Information"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADDINFO(pub u8);
    impl ADDINFO {
        #[must_use]
        #[inline(always)]
        pub const fn IEHOST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IEHOST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for ADDINFO {
        #[inline(always)]
        fn default() -> ADDINFO {
            ADDINFO(0)
        }
    }
    impl core::fmt::Debug for ADDINFO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ADDINFO")
                .field("IEHOST", &self.IEHOST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADDINFO {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ADDINFO {{ IEHOST: {=bool:?} }}", self.IEHOST())
        }
    }
    #[doc = "Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ADDR(pub u8);
    impl ADDR {
        #[must_use]
        #[inline(always)]
        pub const fn ADDR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LSEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            f.debug_struct("ADDR")
                .field("ADDR", &self.ADDR())
                .field("LSEN", &self.LSEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ADDR {{ ADDR: {=u8:?}, LSEN: {=bool:?} }}",
                self.ADDR(),
                self.LSEN()
            )
        }
    }
    #[doc = "BDT Page 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BDTPAGE1(pub u8);
    impl BDTPAGE1 {
        #[must_use]
        #[inline(always)]
        pub const fn BDTBA(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BDTBA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
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
            defmt::write!(f, "BDTPAGE1 {{ BDTBA: {=u8:?} }}", self.BDTBA())
        }
    }
    #[doc = "USB Clock Recovery Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_CTRL(pub u8);
    impl CLK_RECOVER_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn TRIM_INIT_VAL_SEL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRIM_INIT_VAL_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESTART_IFRTRIM_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESTART_IFRTRIM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESET_RESUME_ROUGH_EN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESET_RESUME_ROUGH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLOCK_RECOVER_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CLOCK_RECOVER_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "CLK_RECOVER_CTRL {{ TRIM_INIT_VAL_SEL: {=bool:?}, RESTART_IFRTRIM_EN: {=bool:?}, RESET_RESUME_ROUGH_EN: {=bool:?}, CLOCK_RECOVER_EN: {=bool:?} }}" , self . TRIM_INIT_VAL_SEL () , self . RESTART_IFRTRIM_EN () , self . RESET_RESUME_ROUGH_EN () , self . CLOCK_RECOVER_EN ())
        }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_INT_EN(pub u8);
    impl CLK_RECOVER_INT_EN {
        #[must_use]
        #[inline(always)]
        pub const fn OVF_ERROR_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OVF_ERROR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
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
            defmt::write!(
                f,
                "CLK_RECOVER_INT_EN {{ OVF_ERROR_EN: {=bool:?} }}",
                self.OVF_ERROR_EN()
            )
        }
    }
    #[doc = "Clock Recovery Separated Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_INT_STATUS(pub u8);
    impl CLK_RECOVER_INT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn OVF_ERROR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OVF_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
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
            defmt::write!(
                f,
                "CLK_RECOVER_INT_STATUS {{ OVF_ERROR: {=bool:?} }}",
                self.OVF_ERROR()
            )
        }
    }
    #[doc = "FIRC Oscillator Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLK_RECOVER_IRC_EN(pub u8);
    impl CLK_RECOVER_IRC_EN {
        #[must_use]
        #[inline(always)]
        pub const fn IRC_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IRC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
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
            defmt::write!(
                f,
                "CLK_RECOVER_IRC_EN {{ IRC_EN: {=bool:?} }}",
                self.IRC_EN()
            )
        }
    }
    #[doc = "USB OTG Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONTROL(pub u8);
    impl CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn VBUS_SOURCE_SEL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VBUS_SOURCE_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SESS_VLD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SESS_VLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPPULLUPNONOTG(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPPULLUPNONOTG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
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
            defmt :: write ! (f , "CONTROL {{ VBUS_SOURCE_SEL: {=bool:?}, SESS_VLD: {=bool:?}, DPPULLUPNONOTG: {=bool:?} }}" , self . VBUS_SOURCE_SEL () , self . SESS_VLD () , self . DPPULLUPNONOTG ())
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTL(pub u8);
    impl CTL {
        #[must_use]
        #[inline(always)]
        pub const fn USBENSOFEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USBENSOFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ODDRST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ODDRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESUME(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESUME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HOSTMODEEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HOSTMODEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSUSPENDTOKENBUSY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSUSPENDTOKENBUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SE0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn JSTATE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JSTATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("HOSTMODEEN", &self.HOSTMODEEN())
                .field("RESET", &self.RESET())
                .field("TXSUSPENDTOKENBUSY", &self.TXSUSPENDTOKENBUSY())
                .field("SE0", &self.SE0())
                .field("JSTATE", &self.JSTATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTL {{ USBENSOFEN: {=bool:?}, ODDRST: {=bool:?}, RESUME: {=bool:?}, HOSTMODEEN: {=bool:?}, RESET: {=bool:?}, TXSUSPENDTOKENBUSY: {=bool:?}, SE0: {=bool:?}, JSTATE: {=bool:?} }}" , self . USBENSOFEN () , self . ODDRST () , self . RESUME () , self . HOSTMODEEN () , self . RESET () , self . TXSUSPENDTOKENBUSY () , self . SE0 () , self . JSTATE ())
        }
    }
    #[doc = "Endpoint Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ENDPOINT_ENDPT(pub u8);
    impl ENDPOINT_ENDPT {
        #[must_use]
        #[inline(always)]
        pub const fn EPHSHK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EPHSHK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EPSTALL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EPSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EPTXEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EPTXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EPRXEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EPRXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EPCTLDIS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EPCTLDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RETRYDIS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RETRYDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HOSTWOHUB(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HOSTWOHUB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("RETRYDIS", &self.RETRYDIS())
                .field("HOSTWOHUB", &self.HOSTWOHUB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ENDPOINT_ENDPT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ENDPOINT_ENDPT {{ EPHSHK: {=bool:?}, EPSTALL: {=bool:?}, EPTXEN: {=bool:?}, EPRXEN: {=bool:?}, EPCTLDIS: {=bool:?}, RETRYDIS: {=bool:?}, HOSTWOHUB: {=bool:?} }}" , self . EPHSHK () , self . EPSTALL () , self . EPTXEN () , self . EPRXEN () , self . EPCTLDIS () , self . RETRYDIS () , self . HOSTWOHUB ())
        }
    }
    #[doc = "Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERREN(pub u8);
    impl ERREN {
        #[must_use]
        #[inline(always)]
        pub const fn PIDERREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIDERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC5EOFEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC5EOFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC16EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC16EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFN8EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFN8EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BTOERREN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BTOERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAERREN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OWNERREN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OWNERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BTSERREN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BTSERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "ERREN {{ PIDERREN: {=bool:?}, CRC5EOFEN: {=bool:?}, CRC16EN: {=bool:?}, DFN8EN: {=bool:?}, BTOERREN: {=bool:?}, DMAERREN: {=bool:?}, OWNERREN: {=bool:?}, BTSERREN: {=bool:?} }}" , self . PIDERREN () , self . CRC5EOFEN () , self . CRC16EN () , self . DFN8EN () , self . BTOERREN () , self . DMAERREN () , self . OWNERREN () , self . BTSERREN ())
        }
    }
    #[doc = "Error Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ERRSTAT(pub u8);
    impl ERRSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn PIDERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIDERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC5EOF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC5EOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRC16(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CRC16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DFN8(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DFN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BTOERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BTOERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMAERR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMAERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OWNERR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OWNERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BTSERR(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BTSERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "ERRSTAT {{ PIDERR: {=bool:?}, CRC5EOF: {=bool:?}, CRC16: {=bool:?}, DFN8: {=bool:?}, BTOERR: {=bool:?}, DMAERR: {=bool:?}, OWNERR: {=bool:?}, BTSERR: {=bool:?} }}" , self . PIDERR () , self . CRC5EOF () , self . CRC16 () , self . DFN8 () , self . BTOERR () , self . DMAERR () , self . OWNERR () , self . BTSERR ())
        }
    }
    #[doc = "Frame Number Register High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRMNUMH(pub u8);
    impl FRMNUMH {
        #[must_use]
        #[inline(always)]
        pub const fn FRM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
            defmt::write!(f, "FRMNUMH {{ FRM: {=u8:?} }}", self.FRM())
        }
    }
    #[doc = "Peripheral ID Complement"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IDCOMP(pub u8);
    impl IDCOMP {
        #[must_use]
        #[inline(always)]
        pub const fn NID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
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
            defmt::write!(f, "IDCOMP {{ NID: {=u8:?} }}", self.NID())
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTEN(pub u8);
    impl INTEN {
        #[must_use]
        #[inline(always)]
        pub const fn USBRSTEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USBRSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERROREN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERROREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOFTOKEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOFTOKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOKDNEEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TOKDNEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLEEPEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLEEPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESUMEEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESUMEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ATTACHEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ATTACHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALLEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALLEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("ATTACHEN", &self.ATTACHEN())
                .field("STALLEN", &self.STALLEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INTEN {{ USBRSTEN: {=bool:?}, ERROREN: {=bool:?}, SOFTOKEN: {=bool:?}, TOKDNEEN: {=bool:?}, SLEEPEN: {=bool:?}, RESUMEEN: {=bool:?}, ATTACHEN: {=bool:?}, STALLEN: {=bool:?} }}" , self . USBRSTEN () , self . ERROREN () , self . SOFTOKEN () , self . TOKDNEEN () , self . SLEEPEN () , self . RESUMEEN () , self . ATTACHEN () , self . STALLEN ())
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISTAT(pub u8);
    impl ISTAT {
        #[must_use]
        #[inline(always)]
        pub const fn USBRST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USBRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERROR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOFTOK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOFTOK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOKDNE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TOKDNE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLEEP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SLEEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RESUME(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RESUME(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ATTACH(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ATTACH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("ATTACH", &self.ATTACH())
                .field("STALL", &self.STALL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ISTAT {{ USBRST: {=bool:?}, ERROR: {=bool:?}, SOFTOK: {=bool:?}, TOKDNE: {=bool:?}, SLEEP: {=bool:?}, RESUME: {=bool:?}, ATTACH: {=bool:?}, STALL: {=bool:?} }}" , self . USBRST () , self . ERROR () , self . SOFTOK () , self . TOKDNE () , self . SLEEP () , self . RESUME () , self . ATTACH () , self . STALL ())
        }
    }
    #[doc = "Miscellaneous Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISCCTRL(pub u8);
    impl MISCCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn SOFDYNTHLD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOFDYNTHLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SOFBUSSET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOFBUSSET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OWNERRISODIS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OWNERRISODIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREDG_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VREDG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VFEDG_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VFEDG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STL_ADJ_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STL_ADJ_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("SOFDYNTHLD", &self.SOFDYNTHLD())
                .field("SOFBUSSET", &self.SOFBUSSET())
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
            defmt :: write ! (f , "MISCCTRL {{ SOFDYNTHLD: {=bool:?}, SOFBUSSET: {=bool:?}, OWNERRISODIS: {=bool:?}, VREDG_EN: {=bool:?}, VFEDG_EN: {=bool:?}, STL_ADJ_EN: {=bool:?} }}" , self . SOFDYNTHLD () , self . SOFBUSSET () , self . OWNERRISODIS () , self . VREDG_EN () , self . VFEDG_EN () , self . STL_ADJ_EN ())
        }
    }
    #[doc = "USB OTG Observe"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OBSERVE(pub u8);
    impl OBSERVE {
        #[must_use]
        #[inline(always)]
        pub const fn DMPD(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPPD(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPPD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPPU(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt::write!(
                f,
                "OBSERVE {{ DMPD: {=bool:?}, DPPD: {=bool:?}, DPPU: {=bool:?} }}",
                self.DMPD(),
                self.DPPD(),
                self.DPPU()
            )
        }
    }
    #[doc = "OTG Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGCTL(pub u8);
    impl OTGCTL {
        #[must_use]
        #[inline(always)]
        pub const fn OTGEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OTGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DMLOW(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DMLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPLOW(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DPHIGH(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPHIGH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("OTGEN", &self.OTGEN())
                .field("DMLOW", &self.DMLOW())
                .field("DPLOW", &self.DPLOW())
                .field("DPHIGH", &self.DPHIGH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OTGCTL {{ OTGEN: {=bool:?}, DMLOW: {=bool:?}, DPLOW: {=bool:?}, DPHIGH: {=bool:?} }}" , self . OTGEN () , self . DMLOW () , self . DPLOW () , self . DPHIGH ())
        }
    }
    #[doc = "OTG Interrupt Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGICR(pub u8);
    impl OTGICR {
        #[must_use]
        #[inline(always)]
        pub const fn LINESTATEEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LINESTATEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONEMSECEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONEMSECEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for OTGICR {
        #[inline(always)]
        fn default() -> OTGICR {
            OTGICR(0)
        }
    }
    impl core::fmt::Debug for OTGICR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTGICR")
                .field("LINESTATEEN", &self.LINESTATEEN())
                .field("ONEMSECEN", &self.ONEMSECEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGICR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OTGICR {{ LINESTATEEN: {=bool:?}, ONEMSECEN: {=bool:?} }}",
                self.LINESTATEEN(),
                self.ONEMSECEN()
            )
        }
    }
    #[doc = "OTG Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGISTAT(pub u8);
    impl OTGISTAT {
        #[must_use]
        #[inline(always)]
        pub const fn LINE_STATE_CHG(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LINE_STATE_CHG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONEMSEC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONEMSEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for OTGISTAT {
        #[inline(always)]
        fn default() -> OTGISTAT {
            OTGISTAT(0)
        }
    }
    impl core::fmt::Debug for OTGISTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTGISTAT")
                .field("LINE_STATE_CHG", &self.LINE_STATE_CHG())
                .field("ONEMSEC", &self.ONEMSEC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGISTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OTGISTAT {{ LINE_STATE_CHG: {=bool:?}, ONEMSEC: {=bool:?} }}",
                self.LINE_STATE_CHG(),
                self.ONEMSEC()
            )
        }
    }
    #[doc = "OTG Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OTGSTAT(pub u8);
    impl OTGSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn LINESTATESTABLE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LINESTATESTABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ONEMSEC(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ONEMSEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for OTGSTAT {
        #[inline(always)]
        fn default() -> OTGSTAT {
            OTGSTAT(0)
        }
    }
    impl core::fmt::Debug for OTGSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OTGSTAT")
                .field("LINESTATESTABLE", &self.LINESTATESTABLE())
                .field("ONEMSEC", &self.ONEMSEC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OTGSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OTGSTAT {{ LINESTATESTABLE: {=bool:?}, ONEMSEC: {=bool:?} }}",
                self.LINESTATESTABLE(),
                self.ONEMSEC()
            )
        }
    }
    #[doc = "Peripheral ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PERID(pub u8);
    impl PERID {
        #[must_use]
        #[inline(always)]
        pub const fn ID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
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
            defmt::write!(f, "PERID {{ ID: {=u8:?} }}", self.ID())
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_IH_DIS(pub u8);
    impl STALL_IH_DIS {
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "STALL_IH_DIS {{ STALL_I_DIS8: {=bool:?}, STALL_I_DIS9: {=bool:?}, STALL_I_DIS10: {=bool:?}, STALL_I_DIS11: {=bool:?}, STALL_I_DIS12: {=bool:?}, STALL_I_DIS13: {=bool:?}, STALL_I_DIS14: {=bool:?}, STALL_I_DIS15: {=bool:?} }}" , self . STALL_I_DIS8 () , self . STALL_I_DIS9 () , self . STALL_I_DIS10 () , self . STALL_I_DIS11 () , self . STALL_I_DIS12 () , self . STALL_I_DIS13 () , self . STALL_I_DIS14 () , self . STALL_I_DIS15 ())
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_IL_DIS(pub u8);
    impl STALL_IL_DIS {
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_I_DIS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_I_DIS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "STALL_IL_DIS {{ STALL_I_DIS0: {=bool:?}, STALL_I_DIS1: {=bool:?}, STALL_I_DIS2: {=bool:?}, STALL_I_DIS3: {=bool:?}, STALL_I_DIS4: {=bool:?}, STALL_I_DIS5: {=bool:?}, STALL_I_DIS6: {=bool:?}, STALL_I_DIS7: {=bool:?} }}" , self . STALL_I_DIS0 () , self . STALL_I_DIS1 () , self . STALL_I_DIS2 () , self . STALL_I_DIS3 () , self . STALL_I_DIS4 () , self . STALL_I_DIS5 () , self . STALL_I_DIS6 () , self . STALL_I_DIS7 ())
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_OH_DIS(pub u8);
    impl STALL_OH_DIS {
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS8(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS9(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS10(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS11(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS12(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS13(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS14(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS15(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "STALL_OH_DIS {{ STALL_O_DIS8: {=bool:?}, STALL_O_DIS9: {=bool:?}, STALL_O_DIS10: {=bool:?}, STALL_O_DIS11: {=bool:?}, STALL_O_DIS12: {=bool:?}, STALL_O_DIS13: {=bool:?}, STALL_O_DIS14: {=bool:?}, STALL_O_DIS15: {=bool:?} }}" , self . STALL_O_DIS8 () , self . STALL_O_DIS9 () , self . STALL_O_DIS10 () , self . STALL_O_DIS11 () , self . STALL_O_DIS12 () , self . STALL_O_DIS13 () , self . STALL_O_DIS14 () , self . STALL_O_DIS15 ())
        }
    }
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STALL_OL_DIS(pub u8);
    impl STALL_OL_DIS {
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STALL_O_DIS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STALL_O_DIS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "STALL_OL_DIS {{ STALL_O_DIS0: {=bool:?}, STALL_O_DIS1: {=bool:?}, STALL_O_DIS2: {=bool:?}, STALL_O_DIS3: {=bool:?}, STALL_O_DIS4: {=bool:?}, STALL_O_DIS5: {=bool:?}, STALL_O_DIS6: {=bool:?}, STALL_O_DIS7: {=bool:?} }}" , self . STALL_O_DIS0 () , self . STALL_O_DIS1 () , self . STALL_O_DIS2 () , self . STALL_O_DIS3 () , self . STALL_O_DIS4 () , self . STALL_O_DIS5 () , self . STALL_O_DIS6 () , self . STALL_O_DIS7 ())
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u8);
    impl STAT {
        #[must_use]
        #[inline(always)]
        pub const fn ODD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ODD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TX(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ENDP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ENDP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
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
            defmt::write!(
                f,
                "STAT {{ ODD: {=bool:?}, TX: {=bool:?}, ENDP: {=u8:?} }}",
                self.ODD(),
                self.TX(),
                self.ENDP()
            )
        }
    }
    #[doc = "Token"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TOKEN(pub u8);
    impl TOKEN {
        #[must_use]
        #[inline(always)]
        pub const fn TOKENENDPT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOKENENDPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TOKENPID(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TOKENPID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for TOKEN {
        #[inline(always)]
        fn default() -> TOKEN {
            TOKEN(0)
        }
    }
    impl core::fmt::Debug for TOKEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TOKEN")
                .field("TOKENENDPT", &self.TOKENENDPT())
                .field("TOKENPID", &self.TOKENPID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TOKEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TOKEN {{ TOKENENDPT: {=u8:?}, TOKENPID: {=u8:?} }}",
                self.TOKENENDPT(),
                self.TOKENPID()
            )
        }
    }
    #[doc = "USB Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBCTRL(pub u8);
    impl USBCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DPDM_LANE_REVERSE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPDM_LANE_REVERSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HOST_LS_EOP(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HOST_LS_EOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UARTSEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UARTSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UARTCHLS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UARTCHLS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PDE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SUSP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SUSP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("HOST_LS_EOP", &self.HOST_LS_EOP())
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
            defmt :: write ! (f , "USBCTRL {{ DPDM_LANE_REVERSE: {=bool:?}, HOST_LS_EOP: {=bool:?}, UARTSEL: {=bool:?}, UARTCHLS: {=bool:?}, PDE: {=bool:?}, SUSP: {=bool:?} }}" , self . DPDM_LANE_REVERSE () , self . HOST_LS_EOP () , self . UARTSEL () , self . UARTCHLS () , self . PDE () , self . SUSP ())
        }
    }
    #[doc = "USB Transceiver Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct USBTRC0(pub u8);
    impl USBTRC0 {
        #[must_use]
        #[inline(always)]
        pub const fn USB_RESUME_INT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USB_RESUME_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYNC_DET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYNC_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USB_CLK_RECOVERY_INT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USB_CLK_RECOVERY_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREDG_DET(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VREDG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VFEDG_DET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VFEDG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBRESMEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USBRESMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VREGIN_STS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VREGIN_STS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USBRESET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USBRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
            defmt :: write ! (f , "USBTRC0 {{ USB_RESUME_INT: {=bool:?}, SYNC_DET: {=bool:?}, USB_CLK_RECOVERY_INT: {=bool:?}, VREDG_DET: {=bool:?}, VFEDG_DET: {=bool:?}, USBRESMEN: {=bool:?}, VREGIN_STS: {=bool:?}, USBRESET: {=bool:?} }}" , self . USB_RESUME_INT () , self . SYNC_DET () , self . USB_CLK_RECOVERY_INT () , self . VREDG_DET () , self . VFEDG_DET () , self . USBRESMEN () , self . VREGIN_STS () , self . USBRESET ())
        }
    }
}
