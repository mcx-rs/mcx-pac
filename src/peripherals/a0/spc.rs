#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPC {
    ptr: *mut u8,
}
unsafe impl Send for SPC {}
unsafe impl Sync for SPC {}
impl SPC {
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
    pub const fn SC(self) -> crate::common::Reg<regs::SC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn LPREQ_CFG(self) -> crate::common::Reg<regs::LPREQ_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn PD_STATUS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PD_STATUS, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMCTL(self) -> crate::common::Reg<regs::SRAMCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMRETLDO_REFTRIM(
        self,
    ) -> crate::common::Reg<regs::SRAMRETLDO_REFTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMRETLDO_CNTRL(
        self,
    ) -> crate::common::Reg<regs::SRAMRETLDO_CNTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn ACTIVE_CFG(self) -> crate::common::Reg<regs::ACTIVE_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn ACTIVE_CFG1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn LP_CFG(self) -> crate::common::Reg<regs::LP_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn LP_CFG1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn LPWKUP_DELAY(self) -> crate::common::Reg<regs::LPWKUP_DELAY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn ACTIVE_VDELAY(self) -> crate::common::Reg<regs::ACTIVE_VDELAY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn VD_STAT(self) -> crate::common::Reg<regs::VD_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn VD_CORE_CFG(self) -> crate::common::Reg<regs::VD_CORE_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn VD_SYS_CFG(self) -> crate::common::Reg<regs::VD_SYS_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn EVD_CFG(self) -> crate::common::Reg<regs::EVD_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn CORELDO_CFG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
}
pub mod regs {
    #[doc = "Active Power Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACTIVE_CFG(pub u32);
    impl ACTIVE_CFG {
        #[inline(always)]
        pub const fn CORELDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CORELDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CORELDO_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CORELDO_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn BGMODE(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_BGMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn VDD_VD_DISABLE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VDD_VD_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn CORE_LVDE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CORE_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SYS_LVDE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYS_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SYS_HVDE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYS_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for ACTIVE_CFG {
        #[inline(always)]
        fn default() -> ACTIVE_CFG {
            ACTIVE_CFG(0)
        }
    }
    impl core::fmt::Debug for ACTIVE_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACTIVE_CFG")
                .field("CORELDO_VDD_DS", &self.CORELDO_VDD_DS())
                .field("CORELDO_VDD_LVL", &self.CORELDO_VDD_LVL())
                .field("BGMODE", &self.BGMODE())
                .field("VDD_VD_DISABLE", &self.VDD_VD_DISABLE())
                .field("CORE_LVDE", &self.CORE_LVDE())
                .field("SYS_LVDE", &self.SYS_LVDE())
                .field("SYS_HVDE", &self.SYS_HVDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACTIVE_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACTIVE_CFG {
                CORELDO_VDD_DS: bool,
                CORELDO_VDD_LVL: u8,
                BGMODE: u8,
                VDD_VD_DISABLE: bool,
                CORE_LVDE: bool,
                SYS_LVDE: bool,
                SYS_HVDE: bool,
            }
            let proxy = ACTIVE_CFG {
                CORELDO_VDD_DS: self.CORELDO_VDD_DS(),
                CORELDO_VDD_LVL: self.CORELDO_VDD_LVL(),
                BGMODE: self.BGMODE(),
                VDD_VD_DISABLE: self.VDD_VD_DISABLE(),
                CORE_LVDE: self.CORE_LVDE(),
                SYS_LVDE: self.SYS_LVDE(),
                SYS_HVDE: self.SYS_HVDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Active Voltage Trim Delay"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACTIVE_VDELAY(pub u32);
    impl ACTIVE_VDELAY {
        #[inline(always)]
        pub const fn ACTIVE_VDELAY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ACTIVE_VDELAY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ACTIVE_VDELAY {
        #[inline(always)]
        fn default() -> ACTIVE_VDELAY {
            ACTIVE_VDELAY(0)
        }
    }
    impl core::fmt::Debug for ACTIVE_VDELAY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ACTIVE_VDELAY")
                .field("ACTIVE_VDELAY", &self.ACTIVE_VDELAY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACTIVE_VDELAY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ACTIVE_VDELAY {
                ACTIVE_VDELAY: u16,
            }
            let proxy = ACTIVE_VDELAY {
                ACTIVE_VDELAY: self.ACTIVE_VDELAY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SPC Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG(pub u32);
    impl CFG {
        #[inline(always)]
        pub const fn INTG_PWSWTCH_SLEEP_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_PWSWTCH_SLEEP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INTG_PWSWTCH_WKUP_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_PWSWTCH_WKUP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INTG_PWSWTCH_SLEEP_ACTIVE_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_PWSWTCH_SLEEP_ACTIVE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTG_PWSWTCH_WKUP_ACTIVE_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_PWSWTCH_WKUP_ACTIVE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for CFG {
        #[inline(always)]
        fn default() -> CFG {
            CFG(0)
        }
    }
    impl core::fmt::Debug for CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFG")
                .field("INTG_PWSWTCH_SLEEP_EN", &self.INTG_PWSWTCH_SLEEP_EN())
                .field("INTG_PWSWTCH_WKUP_EN", &self.INTG_PWSWTCH_WKUP_EN())
                .field(
                    "INTG_PWSWTCH_SLEEP_ACTIVE_EN",
                    &self.INTG_PWSWTCH_SLEEP_ACTIVE_EN(),
                )
                .field(
                    "INTG_PWSWTCH_WKUP_ACTIVE_EN",
                    &self.INTG_PWSWTCH_WKUP_ACTIVE_EN(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CFG {
                INTG_PWSWTCH_SLEEP_EN: bool,
                INTG_PWSWTCH_WKUP_EN: bool,
                INTG_PWSWTCH_SLEEP_ACTIVE_EN: bool,
                INTG_PWSWTCH_WKUP_ACTIVE_EN: bool,
            }
            let proxy = CFG {
                INTG_PWSWTCH_SLEEP_EN: self.INTG_PWSWTCH_SLEEP_EN(),
                INTG_PWSWTCH_WKUP_EN: self.INTG_PWSWTCH_WKUP_EN(),
                INTG_PWSWTCH_SLEEP_ACTIVE_EN: self.INTG_PWSWTCH_SLEEP_ACTIVE_EN(),
                INTG_PWSWTCH_WKUP_ACTIVE_EN: self.INTG_PWSWTCH_WKUP_ACTIVE_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "External Voltage Domain Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVD_CFG(pub u32);
    impl EVD_CFG {
        #[inline(always)]
        pub const fn EVDISO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_EVDISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn EVDLPISO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_EVDLPISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn EVDSTAT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_EVDSTAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for EVD_CFG {
        #[inline(always)]
        fn default() -> EVD_CFG {
            EVD_CFG(0)
        }
    }
    impl core::fmt::Debug for EVD_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVD_CFG")
                .field("EVDISO", &self.EVDISO())
                .field("EVDLPISO", &self.EVDLPISO())
                .field("EVDSTAT", &self.EVDSTAT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVD_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EVD_CFG {
                EVDISO: u8,
                EVDLPISO: u8,
                EVDSTAT: u8,
            }
            let proxy = EVD_CFG {
                EVDISO: self.EVDISO(),
                EVDLPISO: self.EVDLPISO(),
                EVDSTAT: self.EVDSTAT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Low-Power Request Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPREQ_CFG(pub u32);
    impl LPREQ_CFG {
        #[inline(always)]
        pub const fn LPREQOE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPREQOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LPREQPOL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPREQPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LPREQOV(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_LPREQOV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for LPREQ_CFG {
        #[inline(always)]
        fn default() -> LPREQ_CFG {
            LPREQ_CFG(0)
        }
    }
    impl core::fmt::Debug for LPREQ_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPREQ_CFG")
                .field("LPREQOE", &self.LPREQOE())
                .field("LPREQPOL", &self.LPREQPOL())
                .field("LPREQOV", &self.LPREQOV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPREQ_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPREQ_CFG {
                LPREQOE: bool,
                LPREQPOL: bool,
                LPREQOV: u8,
            }
            let proxy = LPREQ_CFG {
                LPREQOE: self.LPREQOE(),
                LPREQPOL: self.LPREQPOL(),
                LPREQOV: self.LPREQOV(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Low Power Wake-Up Delay"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPWKUP_DELAY(pub u32);
    impl LPWKUP_DELAY {
        #[inline(always)]
        pub const fn LPWKUP_DELAY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LPWKUP_DELAY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LPWKUP_DELAY {
        #[inline(always)]
        fn default() -> LPWKUP_DELAY {
            LPWKUP_DELAY(0)
        }
    }
    impl core::fmt::Debug for LPWKUP_DELAY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPWKUP_DELAY")
                .field("LPWKUP_DELAY", &self.LPWKUP_DELAY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPWKUP_DELAY {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LPWKUP_DELAY {
                LPWKUP_DELAY: u16,
            }
            let proxy = LPWKUP_DELAY {
                LPWKUP_DELAY: self.LPWKUP_DELAY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Low-Power Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LP_CFG(pub u32);
    impl LP_CFG {
        #[inline(always)]
        pub const fn CORELDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CORELDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CORELDO_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CORELDO_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn SRAMLDO_DPD_ON(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRAMLDO_DPD_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn BGMODE(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_BGMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn LP_IREFEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LP_IREFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn CORE_LVDE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CORE_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SYS_LVDE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYS_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SYS_HVDE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYS_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for LP_CFG {
        #[inline(always)]
        fn default() -> LP_CFG {
            LP_CFG(0)
        }
    }
    impl core::fmt::Debug for LP_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LP_CFG")
                .field("CORELDO_VDD_DS", &self.CORELDO_VDD_DS())
                .field("CORELDO_VDD_LVL", &self.CORELDO_VDD_LVL())
                .field("SRAMLDO_DPD_ON", &self.SRAMLDO_DPD_ON())
                .field("BGMODE", &self.BGMODE())
                .field("LP_IREFEN", &self.LP_IREFEN())
                .field("CORE_LVDE", &self.CORE_LVDE())
                .field("SYS_LVDE", &self.SYS_LVDE())
                .field("SYS_HVDE", &self.SYS_HVDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LP_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LP_CFG {
                CORELDO_VDD_DS: bool,
                CORELDO_VDD_LVL: u8,
                SRAMLDO_DPD_ON: bool,
                BGMODE: u8,
                LP_IREFEN: bool,
                CORE_LVDE: bool,
                SYS_LVDE: bool,
                SYS_HVDE: bool,
            }
            let proxy = LP_CFG {
                CORELDO_VDD_DS: self.CORELDO_VDD_DS(),
                CORELDO_VDD_LVL: self.CORELDO_VDD_LVL(),
                SRAMLDO_DPD_ON: self.SRAMLDO_DPD_ON(),
                BGMODE: self.BGMODE(),
                LP_IREFEN: self.LP_IREFEN(),
                CORE_LVDE: self.CORE_LVDE(),
                SYS_LVDE: self.SYS_LVDE(),
                SYS_HVDE: self.SYS_HVDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SPC Power Domain Mode Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PD_STATUS(pub u32);
    impl PD_STATUS {
        #[inline(always)]
        pub const fn PWR_REQ_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWR_REQ_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn PD_LP_REQ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PD_LP_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn LP_MODE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LP_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for PD_STATUS {
        #[inline(always)]
        fn default() -> PD_STATUS {
            PD_STATUS(0)
        }
    }
    impl core::fmt::Debug for PD_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PD_STATUS")
                .field("PWR_REQ_STATUS", &self.PWR_REQ_STATUS())
                .field("PD_LP_REQ", &self.PD_LP_REQ())
                .field("LP_MODE", &self.LP_MODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PD_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PD_STATUS {
                PWR_REQ_STATUS: bool,
                PD_LP_REQ: bool,
                LP_MODE: u8,
            }
            let proxy = PD_STATUS {
                PWR_REQ_STATUS: self.PWR_REQ_STATUS(),
                PD_LP_REQ: self.PD_LP_REQ(),
                LP_MODE: self.LP_MODE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SC(pub u32);
    impl SC {
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SPC_LP_REQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPC_LP_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SPC_LP_MODE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SPC_LP_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn ISO_CLR(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISO_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SWITCH_STATE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWITCH_STATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SC {
        #[inline(always)]
        fn default() -> SC {
            SC(0)
        }
    }
    impl core::fmt::Debug for SC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SC")
                .field("BUSY", &self.BUSY())
                .field("SPC_LP_REQ", &self.SPC_LP_REQ())
                .field("SPC_LP_MODE", &self.SPC_LP_MODE())
                .field("ISO_CLR", &self.ISO_CLR())
                .field("SWITCH_STATE", &self.SWITCH_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SC {
                BUSY: bool,
                SPC_LP_REQ: bool,
                SPC_LP_MODE: u8,
                ISO_CLR: bool,
                SWITCH_STATE: bool,
            }
            let proxy = SC {
                BUSY: self.BUSY(),
                SPC_LP_REQ: self.SPC_LP_REQ(),
                SPC_LP_MODE: self.SPC_LP_MODE(),
                ISO_CLR: self.ISO_CLR(),
                SWITCH_STATE: self.SWITCH_STATE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SRAM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMCTL(pub u32);
    impl SRAMCTL {
        #[inline(always)]
        pub const fn VSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_VSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ACK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SRAMCTL {
        #[inline(always)]
        fn default() -> SRAMCTL {
            SRAMCTL(0)
        }
    }
    impl core::fmt::Debug for SRAMCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAMCTL")
                .field("VSM", &self.VSM())
                .field("REQ", &self.REQ())
                .field("ACK", &self.ACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAMCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAMCTL {
                VSM: u8,
                REQ: bool,
                ACK: bool,
            }
            let proxy = SRAMCTL {
                VSM: self.VSM(),
                REQ: self.REQ(),
                ACK: self.ACK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SRAM Retention LDO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMRETLDO_CNTRL(pub u32);
    impl SRAMRETLDO_CNTRL {
        #[inline(always)]
        pub const fn SRAMLDO_ON(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRAMLDO_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SRAM_RET_EN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SRAM_RET_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for SRAMRETLDO_CNTRL {
        #[inline(always)]
        fn default() -> SRAMRETLDO_CNTRL {
            SRAMRETLDO_CNTRL(0)
        }
    }
    impl core::fmt::Debug for SRAMRETLDO_CNTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAMRETLDO_CNTRL")
                .field("SRAMLDO_ON", &self.SRAMLDO_ON())
                .field("SRAM_RET_EN", &self.SRAM_RET_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAMRETLDO_CNTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAMRETLDO_CNTRL {
                SRAMLDO_ON: bool,
                SRAM_RET_EN: u8,
            }
            let proxy = SRAMRETLDO_CNTRL {
                SRAMLDO_ON: self.SRAMLDO_ON(),
                SRAM_RET_EN: self.SRAM_RET_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SRAM Retention Reference Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMRETLDO_REFTRIM(pub u32);
    impl SRAMRETLDO_REFTRIM {
        #[inline(always)]
        pub const fn REFTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_REFTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for SRAMRETLDO_REFTRIM {
        #[inline(always)]
        fn default() -> SRAMRETLDO_REFTRIM {
            SRAMRETLDO_REFTRIM(0)
        }
    }
    impl core::fmt::Debug for SRAMRETLDO_REFTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAMRETLDO_REFTRIM")
                .field("REFTRIM", &self.REFTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAMRETLDO_REFTRIM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAMRETLDO_REFTRIM {
                REFTRIM: u8,
            }
            let proxy = SRAMRETLDO_REFTRIM {
                REFTRIM: self.REFTRIM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Core Voltage Detect Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_CORE_CFG(pub u32);
    impl VD_CORE_CFG {
        #[inline(always)]
        pub const fn LVDRE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LVDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for VD_CORE_CFG {
        #[inline(always)]
        fn default() -> VD_CORE_CFG {
            VD_CORE_CFG(0)
        }
    }
    impl core::fmt::Debug for VD_CORE_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VD_CORE_CFG")
                .field("LVDRE", &self.LVDRE())
                .field("LVDIE", &self.LVDIE())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_CORE_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VD_CORE_CFG {
                LVDRE: bool,
                LVDIE: bool,
                LOCK: bool,
            }
            let proxy = VD_CORE_CFG {
                LVDRE: self.LVDRE(),
                LVDIE: self.LVDIE(),
                LOCK: self.LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Voltage Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_STAT(pub u32);
    impl VD_STAT {
        #[inline(always)]
        pub const fn COREVDD_LVDF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COREVDD_LVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SYSVDD_LVDF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYSVDD_LVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SYSVDD_HVDF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYSVDD_HVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for VD_STAT {
        #[inline(always)]
        fn default() -> VD_STAT {
            VD_STAT(0)
        }
    }
    impl core::fmt::Debug for VD_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VD_STAT")
                .field("COREVDD_LVDF", &self.COREVDD_LVDF())
                .field("SYSVDD_LVDF", &self.SYSVDD_LVDF())
                .field("SYSVDD_HVDF", &self.SYSVDD_HVDF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_STAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VD_STAT {
                COREVDD_LVDF: bool,
                SYSVDD_LVDF: bool,
                SYSVDD_HVDF: bool,
            }
            let proxy = VD_STAT {
                COREVDD_LVDF: self.COREVDD_LVDF(),
                SYSVDD_LVDF: self.SYSVDD_LVDF(),
                SYSVDD_HVDF: self.SYSVDD_HVDF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "System Voltage Detect Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_SYS_CFG(pub u32);
    impl VD_SYS_CFG {
        #[inline(always)]
        pub const fn LVDRE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LVDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HVDRE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HVDIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn LVSEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LVSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for VD_SYS_CFG {
        #[inline(always)]
        fn default() -> VD_SYS_CFG {
            VD_SYS_CFG(0)
        }
    }
    impl core::fmt::Debug for VD_SYS_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VD_SYS_CFG")
                .field("LVDRE", &self.LVDRE())
                .field("LVDIE", &self.LVDIE())
                .field("HVDRE", &self.HVDRE())
                .field("HVDIE", &self.HVDIE())
                .field("LVSEL", &self.LVSEL())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_SYS_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VD_SYS_CFG {
                LVDRE: bool,
                LVDIE: bool,
                HVDRE: bool,
                HVDIE: bool,
                LVSEL: bool,
                LOCK: bool,
            }
            let proxy = VD_SYS_CFG {
                LVDRE: self.LVDRE(),
                LVDIE: self.LVDIE(),
                HVDRE: self.HVDRE(),
                HVDIE: self.HVDIE(),
                LVSEL: self.LVSEL(),
                LOCK: self.LOCK(),
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for VERID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VERID")
                .field("FEATURE", &self.FEATURE())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VERID {
                FEATURE: u16,
                MINOR: u8,
                MAJOR: u8,
            }
            let proxy = VERID {
                FEATURE: self.FEATURE(),
                MINOR: self.MINOR(),
                MAJOR: self.MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
