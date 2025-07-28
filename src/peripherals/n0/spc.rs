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
    pub const fn CNTRL(self) -> crate::common::Reg<regs::CNTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
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
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMCTL(self) -> crate::common::Reg<regs::SRAMCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
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
    pub const fn VD_IO_CFG(self) -> crate::common::Reg<regs::VD_IO_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[inline(always)]
    pub const fn EVD_CFG(self) -> crate::common::Reg<regs::EVD_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn GLITCH_DETECT_SC(
        self,
    ) -> crate::common::Reg<regs::GLITCH_DETECT_SC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn CORELDO_CFG(self) -> crate::common::Reg<regs::CORELDO_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn SYSLDO_CFG(self) -> crate::common::Reg<regs::SYSLDO_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn DCDC_CFG(self) -> crate::common::Reg<regs::DCDC_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn DCDC_BURST_CFG(
        self,
    ) -> crate::common::Reg<regs::DCDC_BURST_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
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
        pub const fn SYSLDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSLDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYSLDO_VDD_LVL(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSLDO_VDD_LVL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCDC_VDD_DS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCDC_VDD_DS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCDC_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCDC_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GLITCH_DETECT_DISABLE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GLITCH_DETECT_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPBUFF_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPBUFF_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
        pub const fn IO_LVDE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IO_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORE_HVDE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORE_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
        #[must_use]
        #[inline(always)]
        pub const fn IO_HVDE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IO_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("SYSLDO_VDD_DS", &self.SYSLDO_VDD_DS())
                .field("SYSLDO_VDD_LVL", &self.SYSLDO_VDD_LVL())
                .field("DCDC_VDD_DS", &self.DCDC_VDD_DS())
                .field("DCDC_VDD_LVL", &self.DCDC_VDD_LVL())
                .field("GLITCH_DETECT_DISABLE", &self.GLITCH_DETECT_DISABLE())
                .field("LPBUFF_EN", &self.LPBUFF_EN())
                .field("BGMODE", &self.BGMODE())
                .field("VDD_VD_DISABLE", &self.VDD_VD_DISABLE())
                .field("CORE_LVDE", &self.CORE_LVDE())
                .field("SYS_LVDE", &self.SYS_LVDE())
                .field("IO_LVDE", &self.IO_LVDE())
                .field("CORE_HVDE", &self.CORE_HVDE())
                .field("SYS_HVDE", &self.SYS_HVDE())
                .field("IO_HVDE", &self.IO_HVDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ACTIVE_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ACTIVE_CFG {{ CORELDO_VDD_DS: {=bool:?}, CORELDO_VDD_LVL: {=u8:?}, SYSLDO_VDD_DS: {=bool:?}, SYSLDO_VDD_LVL: {=bool:?}, DCDC_VDD_DS: {=u8:?}, DCDC_VDD_LVL: {=u8:?}, GLITCH_DETECT_DISABLE: {=bool:?}, LPBUFF_EN: {=bool:?}, BGMODE: {=u8:?}, VDD_VD_DISABLE: {=bool:?}, CORE_LVDE: {=bool:?}, SYS_LVDE: {=bool:?}, IO_LVDE: {=bool:?}, CORE_HVDE: {=bool:?}, SYS_HVDE: {=bool:?}, IO_HVDE: {=bool:?} }}" , self . CORELDO_VDD_DS () , self . CORELDO_VDD_LVL () , self . SYSLDO_VDD_DS () , self . SYSLDO_VDD_LVL () , self . DCDC_VDD_DS () , self . DCDC_VDD_LVL () , self . GLITCH_DETECT_DISABLE () , self . LPBUFF_EN () , self . BGMODE () , self . VDD_VD_DISABLE () , self . CORE_LVDE () , self . SYS_LVDE () , self . IO_LVDE () , self . CORE_HVDE () , self . SYS_HVDE () , self . IO_HVDE ())
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
    #[doc = "SPC Regulator Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CNTRL(pub u32);
    impl CNTRL {
        #[must_use]
        #[inline(always)]
        pub const fn CORELDO_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORELDO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SYSLDO_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSLDO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCDC_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCDC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CNTRL {
        #[inline(always)]
        fn default() -> CNTRL {
            CNTRL(0)
        }
    }
    impl core::fmt::Debug for CNTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CNTRL")
                .field("CORELDO_EN", &self.CORELDO_EN())
                .field("SYSLDO_EN", &self.SYSLDO_EN())
                .field("DCDC_EN", &self.DCDC_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CNTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CNTRL {{ CORELDO_EN: {=bool:?}, SYSLDO_EN: {=bool:?}, DCDC_EN: {=bool:?} }}",
                self.CORELDO_EN(),
                self.SYSLDO_EN(),
                self.DCDC_EN()
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
        pub const fn DPDOWN_PULLDOWN_DISABLE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DPDOWN_PULLDOWN_DISABLE(&mut self, val: bool) {
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
                .field("DPDOWN_PULLDOWN_DISABLE", &self.DPDOWN_PULLDOWN_DISABLE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORELDO_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CORELDO_CFG {{ DPDOWN_PULLDOWN_DISABLE: {=bool:?} }}",
                self.DPDOWN_PULLDOWN_DISABLE()
            )
        }
    }
    #[doc = "DCDC Burst Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DCDC_BURST_CFG(pub u32);
    impl DCDC_BURST_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn BURST_REQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BURST_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXT_BURST_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXT_BURST_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BURST_ACK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BURST_ACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PULSE_REFRESH_CNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PULSE_REFRESH_CNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DCDC_BURST_CFG {
        #[inline(always)]
        fn default() -> DCDC_BURST_CFG {
            DCDC_BURST_CFG(0)
        }
    }
    impl core::fmt::Debug for DCDC_BURST_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DCDC_BURST_CFG")
                .field("BURST_REQ", &self.BURST_REQ())
                .field("EXT_BURST_EN", &self.EXT_BURST_EN())
                .field("BURST_ACK", &self.BURST_ACK())
                .field("PULSE_REFRESH_CNT", &self.PULSE_REFRESH_CNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DCDC_BURST_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DCDC_BURST_CFG {{ BURST_REQ: {=bool:?}, EXT_BURST_EN: {=bool:?}, BURST_ACK: {=bool:?}, PULSE_REFRESH_CNT: {=u16:?} }}" , self . BURST_REQ () , self . EXT_BURST_EN () , self . BURST_ACK () , self . PULSE_REFRESH_CNT ())
        }
    }
    #[doc = "DCDC Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DCDC_CFG(pub u32);
    impl DCDC_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn FREQ_CNTRL_ON(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FREQ_CNTRL_ON(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FREQ_CNTRL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FREQ_CNTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BLEED_EN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BLEED_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for DCDC_CFG {
        #[inline(always)]
        fn default() -> DCDC_CFG {
            DCDC_CFG(0)
        }
    }
    impl core::fmt::Debug for DCDC_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DCDC_CFG")
                .field("FREQ_CNTRL_ON", &self.FREQ_CNTRL_ON())
                .field("FREQ_CNTRL", &self.FREQ_CNTRL())
                .field("BLEED_EN", &self.BLEED_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DCDC_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DCDC_CFG {{ FREQ_CNTRL_ON: {=bool:?}, FREQ_CNTRL: {=u8:?}, BLEED_EN: {=bool:?} }}",
                self.FREQ_CNTRL_ON(),
                self.FREQ_CNTRL(),
                self.BLEED_EN()
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
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVDLPISO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDLPISO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVDSTAT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVDSTAT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
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
    #[doc = "Glitch Detect Status Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GLITCH_DETECT_SC(pub u32);
    impl GLITCH_DETECT_SC {
        #[must_use]
        #[inline(always)]
        pub const fn CNT_SELECT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CNT_SELECT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIMEOUT(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TIMEOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GLITCH_DETECT_FLAG(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GLITCH_DETECT_FLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
    impl Default for GLITCH_DETECT_SC {
        #[inline(always)]
        fn default() -> GLITCH_DETECT_SC {
            GLITCH_DETECT_SC(0)
        }
    }
    impl core::fmt::Debug for GLITCH_DETECT_SC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GLITCH_DETECT_SC")
                .field("CNT_SELECT", &self.CNT_SELECT())
                .field("TIMEOUT", &self.TIMEOUT())
                .field("RE", &self.RE())
                .field("IE", &self.IE())
                .field("GLITCH_DETECT_FLAG", &self.GLITCH_DETECT_FLAG())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GLITCH_DETECT_SC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GLITCH_DETECT_SC {{ CNT_SELECT: {=u8:?}, TIMEOUT: {=u8:?}, RE: {=bool:?}, IE: {=bool:?}, GLITCH_DETECT_FLAG: {=u8:?}, LOCK: {=bool:?} }}" , self . CNT_SELECT () , self . TIMEOUT () , self . RE () , self . IE () , self . GLITCH_DETECT_FLAG () , self . LOCK ())
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
        pub const fn SYSLDO_VDD_DS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SYSLDO_VDD_DS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCDC_VDD_DS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCDC_VDD_DS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCDC_VDD_LVL(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCDC_VDD_LVL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GLITCH_DETECT_DISABLE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GLITCH_DETECT_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COREVDD_IVS_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COREVDD_IVS_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPBUFF_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPBUFF_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
        pub const fn IO_LVDE(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IO_LVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CORE_HVDE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CORE_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
        #[must_use]
        #[inline(always)]
        pub const fn IO_HVDE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IO_HVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("SYSLDO_VDD_DS", &self.SYSLDO_VDD_DS())
                .field("DCDC_VDD_DS", &self.DCDC_VDD_DS())
                .field("DCDC_VDD_LVL", &self.DCDC_VDD_LVL())
                .field("GLITCH_DETECT_DISABLE", &self.GLITCH_DETECT_DISABLE())
                .field("COREVDD_IVS_EN", &self.COREVDD_IVS_EN())
                .field("LPBUFF_EN", &self.LPBUFF_EN())
                .field("BGMODE", &self.BGMODE())
                .field("LP_IREFEN", &self.LP_IREFEN())
                .field("CORE_LVDE", &self.CORE_LVDE())
                .field("SYS_LVDE", &self.SYS_LVDE())
                .field("IO_LVDE", &self.IO_LVDE())
                .field("CORE_HVDE", &self.CORE_HVDE())
                .field("SYS_HVDE", &self.SYS_HVDE())
                .field("IO_HVDE", &self.IO_HVDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LP_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LP_CFG {{ CORELDO_VDD_DS: {=bool:?}, CORELDO_VDD_LVL: {=u8:?}, SYSLDO_VDD_DS: {=bool:?}, DCDC_VDD_DS: {=u8:?}, DCDC_VDD_LVL: {=u8:?}, GLITCH_DETECT_DISABLE: {=bool:?}, COREVDD_IVS_EN: {=bool:?}, LPBUFF_EN: {=bool:?}, BGMODE: {=u8:?}, LP_IREFEN: {=bool:?}, CORE_LVDE: {=bool:?}, SYS_LVDE: {=bool:?}, IO_LVDE: {=bool:?}, CORE_HVDE: {=bool:?}, SYS_HVDE: {=bool:?}, IO_HVDE: {=bool:?} }}" , self . CORELDO_VDD_DS () , self . CORELDO_VDD_LVL () , self . SYSLDO_VDD_DS () , self . DCDC_VDD_DS () , self . DCDC_VDD_LVL () , self . GLITCH_DETECT_DISABLE () , self . COREVDD_IVS_EN () , self . LPBUFF_EN () , self . BGMODE () , self . LP_IREFEN () , self . CORE_LVDE () , self . SYS_LVDE () , self . IO_LVDE () , self . CORE_HVDE () , self . SYS_HVDE () , self . IO_HVDE ())
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
        pub const fn ISO_CLR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ISO_CLR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
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
            defmt :: write ! (f , "SC {{ BUSY: {=bool:?}, SPC_LP_REQ: {=bool:?}, SPC_LP_MODE: {=u8:?}, ISO_CLR: {=u8:?} }}" , self . BUSY () , self . SPC_LP_REQ () , self . SPC_LP_MODE () , self . ISO_CLR ())
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
    #[doc = "LDO_SYS Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SYSLDO_CFG(pub u32);
    impl SYSLDO_CFG {
        #[must_use]
        #[inline(always)]
        pub const fn ISINKEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ISINKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SYSLDO_CFG {
        #[inline(always)]
        fn default() -> SYSLDO_CFG {
            SYSLDO_CFG(0)
        }
    }
    impl core::fmt::Debug for SYSLDO_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SYSLDO_CFG")
                .field("ISINKEN", &self.ISINKEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SYSLDO_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SYSLDO_CFG {{ ISINKEN: {=bool:?} }}", self.ISINKEN())
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
                .field("HVDRE", &self.HVDRE())
                .field("HVDIE", &self.HVDIE())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_CORE_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VD_CORE_CFG {{ LVDRE: {=bool:?}, LVDIE: {=bool:?}, HVDRE: {=bool:?}, HVDIE: {=bool:?}, LOCK: {=bool:?} }}" , self . LVDRE () , self . LVDIE () , self . HVDRE () , self . HVDIE () , self . LOCK ())
        }
    }
    #[doc = "IO Voltage Detect Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VD_IO_CFG(pub u32);
    impl VD_IO_CFG {
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
        pub const fn LVSEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LVSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
    impl Default for VD_IO_CFG {
        #[inline(always)]
        fn default() -> VD_IO_CFG {
            VD_IO_CFG(0)
        }
    }
    impl core::fmt::Debug for VD_IO_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VD_IO_CFG")
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
    impl defmt::Format for VD_IO_CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VD_IO_CFG {{ LVDRE: {=bool:?}, LVDIE: {=bool:?}, HVDRE: {=bool:?}, HVDIE: {=bool:?}, LVSEL: {=bool:?}, LOCK: {=bool:?} }}" , self . LVDRE () , self . LVDIE () , self . HVDRE () , self . HVDIE () , self . LVSEL () , self . LOCK ())
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
        pub const fn IOVDD_LVDF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IOVDD_LVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COREVDD_HVDF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COREVDD_HVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        #[must_use]
        #[inline(always)]
        pub const fn IOVDD_HVDF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IOVDD_HVDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("IOVDD_LVDF", &self.IOVDD_LVDF())
                .field("COREVDD_HVDF", &self.COREVDD_HVDF())
                .field("SYSVDD_HVDF", &self.SYSVDD_HVDF())
                .field("IOVDD_HVDF", &self.IOVDD_HVDF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VD_STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VD_STAT {{ COREVDD_LVDF: {=bool:?}, SYSVDD_LVDF: {=bool:?}, IOVDD_LVDF: {=bool:?}, COREVDD_HVDF: {=bool:?}, SYSVDD_HVDF: {=bool:?}, IOVDD_HVDF: {=bool:?} }}" , self . COREVDD_LVDF () , self . SYSVDD_LVDF () , self . IOVDD_LVDF () , self . COREVDD_HVDF () , self . SYSVDD_HVDF () , self . IOVDD_HVDF ())
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
