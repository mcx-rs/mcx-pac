#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
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
    pub const fn CORELDO_CFG(self) -> crate::common::Reg<regs::CORELDO_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
}
pub mod regs {
    #[doc = "Active Power Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACTIVE_CFG(pub u32);
    impl ACTIVE_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORELDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CORELDO_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGMODE(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BGMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VDD_VD_DISABLE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VDD_VD_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORE_LVDE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORE_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYS_LVDE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYS_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYS_HVDE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYS_HVDE(&mut self, val: bool) {
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
            defmt :: write ! (f , "ACTIVE_CFG {{ CORELDO_VDD_DS: {=bool:?}, CORELDO_VDD_LVL: {=u8:?}, BGMODE: {=u8:?}, VDD_VD_DISABLE: {=bool:?}, CORE_LVDE: {=bool:?}, SYS_LVDE: {=bool:?}, SYS_HVDE: {=bool:?} }}" , self . CORELDO_VDD_DS () , self . CORELDO_VDD_LVL () , self . BGMODE () , self . VDD_VD_DISABLE () , self . CORE_LVDE () , self . SYS_LVDE () , self . SYS_HVDE ())
        }
    }
    #[doc = "Active Voltage Trim Delay"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ACTIVE_VDELAY(pub u32);
    impl ACTIVE_VDELAY {
        #[must_use]
        #[inline(always)]
        pub const fn ACTIVE_VDELAY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ACTIVE_VDELAY(&mut self, val: u16) {
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
            defmt::write!(
                f,
                "ACTIVE_VDELAY {{ ACTIVE_VDELAY: {=u16:?} }}",
                self.ACTIVE_VDELAY()
            )
        }
    }
    #[doc = "LDO_CORE Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORELDO_CFG(pub u32);
    impl CORELDO_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_SPARE0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORELDO_SPARE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for CORELDO_CFG {
        #[inline(always)]
        fn default() -> CORELDO_CFG {
            CORELDO_CFG(0)
        }
    }
    impl core::fmt::Debug for CORELDO_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORELDO_CFG")
                .field("CORELDO_SPARE0", &self.CORELDO_SPARE0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORELDO_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CORELDO_CFG {{ CORELDO_SPARE0: {=bool:?} }}",
                self.CORELDO_SPARE0()
            )
        }
    }
    #[doc = "External Voltage Domain Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVD_CFG(pub u32);
    impl EVD_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn EVDISO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVDLPISO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDLPISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVDSTAT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDSTAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
            defmt::write!(
                f,
                "EVD_CFG {{ EVDISO: {=u8:?}, EVDLPISO: {=u8:?}, EVDSTAT: {=u8:?} }}",
                self.EVDISO(),
                self.EVDLPISO(),
                self.EVDSTAT()
            )
        }
    }
    #[doc = "Low-Power Request Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPREQ_CFG(pub u32);
    impl LPREQ_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn LPREQOE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPREQOE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPREQPOL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPREQPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPREQOV(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPREQOV(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "LPREQ_CFG {{ LPREQOE: {=bool:?}, LPREQPOL: {=bool:?}, LPREQOV: {=u8:?} }}",
                self.LPREQOE(),
                self.LPREQPOL(),
                self.LPREQOV()
            )
        }
    }
    #[doc = "Low Power Wake-Up Delay"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPWKUP_DELAY(pub u32);
    impl LPWKUP_DELAY {
        #[must_use]
        #[inline(always)]
        pub const fn LPWKUP_DELAY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LPWKUP_DELAY(&mut self, val: u16) {
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
            defmt::write!(
                f,
                "LPWKUP_DELAY {{ LPWKUP_DELAY: {=u16:?} }}",
                self.LPWKUP_DELAY()
            )
        }
    }
    #[doc = "Low-Power Mode Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LP_CFG(pub u32);
    impl LP_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORELDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CORELDO_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRAMLDO_DPD_ON(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SRAMLDO_DPD_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BGMODE(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BGMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_IREFEN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LP_IREFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORE_LVDE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORE_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYS_LVDE(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYS_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYS_HVDE(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYS_HVDE(&mut self, val: bool) {
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
            defmt :: write ! (f , "LP_CFG {{ CORELDO_VDD_DS: {=bool:?}, CORELDO_VDD_LVL: {=u8:?}, SRAMLDO_DPD_ON: {=bool:?}, BGMODE: {=u8:?}, LP_IREFEN: {=bool:?}, CORE_LVDE: {=bool:?}, SYS_LVDE: {=bool:?}, SYS_HVDE: {=bool:?} }}" , self . CORELDO_VDD_DS () , self . CORELDO_VDD_LVL () , self . SRAMLDO_DPD_ON () , self . BGMODE () , self . LP_IREFEN () , self . CORE_LVDE () , self . SYS_LVDE () , self . SYS_HVDE ())
        }
    }
    #[doc = "SPC Power Domain Mode Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PD_STATUS(pub u32);
    impl PD_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn PWR_REQ_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWR_REQ_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PD_LP_REQ(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PD_LP_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LP_MODE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LP_MODE(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "PD_STATUS {{ PWR_REQ_STATUS: {=bool:?}, PD_LP_REQ: {=bool:?}, LP_MODE: {=u8:?} }}",
                self.PWR_REQ_STATUS(),
                self.PD_LP_REQ(),
                self.LP_MODE()
            )
        }
    }
    #[doc = "Status Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SC(pub u32);
    impl SC {
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPC_LP_REQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPC_LP_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPC_LP_MODE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SPC_LP_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ISO_CLR(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ISO_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SC {{ BUSY: {=bool:?}, SPC_LP_REQ: {=bool:?}, SPC_LP_MODE: {=u8:?}, ISO_CLR: {=bool:?} }}" , self . BUSY () , self . SPC_LP_REQ () , self . SPC_LP_MODE () , self . ISO_CLR ())
        }
    }
    #[doc = "SRAM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMCTL(pub u32);
    impl SRAMCTL {
        #[must_use]
        #[inline(always)]
        pub const fn VSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REQ(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACK(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "SRAMCTL {{ VSM: {=u8:?}, REQ: {=bool:?}, ACK: {=bool:?} }}",
                self.VSM(),
                self.REQ(),
                self.ACK()
            )
        }
    }
    #[doc = "SRAM Retention LDO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMRETLDO_CNTRL(pub u32);
    impl SRAMRETLDO_CNTRL {
        #[must_use]
        #[inline(always)]
        pub const fn SRAMLDO_ON(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SRAMLDO_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SRAM_RET_EN(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SRAM_RET_EN(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "SRAMRETLDO_CNTRL {{ SRAMLDO_ON: {=bool:?}, SRAM_RET_EN: {=u8:?} }}",
                self.SRAMLDO_ON(),
                self.SRAM_RET_EN()
            )
        }
    }
    #[doc = "SRAM Retention Reference Trim"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMRETLDO_REFTRIM(pub u32);
    impl SRAMRETLDO_REFTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn REFTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_REFTRIM(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "SRAMRETLDO_REFTRIM {{ REFTRIM: {=u8:?} }}",
                self.REFTRIM()
            )
        }
    }
    #[doc = "Core Voltage Detect Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_CORE_CFG(pub u32);
    impl VD_CORE_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn LVDRE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LVDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "VD_CORE_CFG {{ LVDRE: {=bool:?}, LVDIE: {=bool:?}, LOCK: {=bool:?} }}",
                self.LVDRE(),
                self.LVDIE(),
                self.LOCK()
            )
        }
    }
    #[doc = "Voltage Detect Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_STAT(pub u32);
    impl VD_STAT {
        #[must_use]
        #[inline(always)]
        pub const fn COREVDD_LVDF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COREVDD_LVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYSVDD_LVDF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSVDD_LVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYSVDD_HVDF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSVDD_HVDF(&mut self, val: bool) {
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
            defmt :: write ! (f , "VD_STAT {{ COREVDD_LVDF: {=bool:?}, SYSVDD_LVDF: {=bool:?}, SYSVDD_HVDF: {=bool:?} }}" , self . COREVDD_LVDF () , self . SYSVDD_LVDF () , self . SYSVDD_HVDF ())
        }
    }
    #[doc = "System Voltage Detect Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_SYS_CFG(pub u32);
    impl VD_SYS_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn LVDRE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LVDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HVDRE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HVDRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HVDIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HVDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: bool) {
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
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_SYS_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VD_SYS_CFG {{ LVDRE: {=bool:?}, LVDIE: {=bool:?}, HVDRE: {=bool:?}, HVDIE: {=bool:?}, LOCK: {=bool:?} }}" , self . LVDRE () , self . LVDIE () , self . HVDRE () , self . HVDIE () , self . LOCK ())
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[must_use]
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAJOR(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
                self.FEATURE(),
                self.MINOR(),
                self.MAJOR()
            )
        }
    }
}
