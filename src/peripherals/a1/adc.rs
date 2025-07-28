#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC {
    ptr: *mut u8,
}
unsafe impl Send for ADC {}
unsafe impl Sync for ADC {}
impl ADC {
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
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn IE(self) -> crate::common::Reg<regs::IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DE(self) -> crate::common::Reg<regs::DE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn PAUSE(self) -> crate::common::Reg<regs::PAUSE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn SWTRIG(self) -> crate::common::Reg<regs::SWTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn TSTAT(self) -> crate::common::Reg<regs::TSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn OFSTRIM(self) -> crate::common::Reg<regs::OFSTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn HSTRIM(self) -> crate::common::Reg<regs::HSTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn TCTRL(self, n: usize) -> crate::common::Reg<regs::TCTRL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FCTRL(self) -> crate::common::Reg<regs::FCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn GCC(self, n: usize) -> crate::common::Reg<regs::GCC, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn GCR(self, n: usize) -> crate::common::Reg<regs::GCR, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CMD(self, n: usize) -> CMD {
        assert!(n < 7usize);
        unsafe { CMD::from_ptr(self.ptr.add(0x0100usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn CV(self, n: usize) -> crate::common::Reg<regs::CV, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn RESFIFO(self) -> crate::common::Reg<regs::RESFIFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR(self, n: usize) -> crate::common::Reg<regs::CAL_GAR, crate::common::RW> {
        assert!(n < 34usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CFG2(self) -> crate::common::Reg<regs::CFG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMD {
    ptr: *mut u8,
}
unsafe impl Send for CMD {}
unsafe impl Sync for CMD {}
impl CMD {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CMDL(self) -> crate::common::Reg<regs::CMD_CMDL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMDH(self) -> crate::common::Reg<regs::CMD_CMDH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR(pub u32);
    impl CAL_GAR {
        #[must_use]
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR {
        #[inline(always)]
        fn default() -> CAL_GAR {
            CAL_GAR(0)
        }
    }
    impl core::fmt::Debug for CAL_GAR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAL_GAR")
                .field("CAL_GAR_VAL", &self.CAL_GAR_VAL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAL_GAR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CAL_GAR {{ CAL_GAR_VAL: {=u16:?} }}", self.CAL_GAR_VAL())
        }
    }
    #[doc = "Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG(pub u32);
    impl CFG {
        #[must_use]
        #[inline(always)]
        pub const fn TPRICTRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPRICTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWRSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWRSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REFSEL(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_REFSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRES(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCMDRES(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCMDRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HPT_EXDI(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HPT_EXDI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PUDLY(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PUDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWREN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("TPRICTRL", &self.TPRICTRL())
                .field("PWRSEL", &self.PWRSEL())
                .field("REFSEL", &self.REFSEL())
                .field("TRES", &self.TRES())
                .field("TCMDRES", &self.TCMDRES())
                .field("HPT_EXDI", &self.HPT_EXDI())
                .field("PUDLY", &self.PUDLY())
                .field("PWREN", &self.PWREN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CFG {{ TPRICTRL: {=u8:?}, PWRSEL: {=bool:?}, REFSEL: {=u8:?}, TRES: {=bool:?}, TCMDRES: {=bool:?}, HPT_EXDI: {=bool:?}, PUDLY: {=u8:?}, PWREN: {=bool:?} }}" , self . TPRICTRL () , self . PWRSEL () , self . REFSEL () , self . TRES () , self . TCMDRES () , self . HPT_EXDI () , self . PUDLY () , self . PWREN ())
        }
    }
    #[doc = "Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG2(pub u32);
    impl CFG2 {
        #[must_use]
        #[inline(always)]
        pub const fn JLEFT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JLEFT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HSEXTRA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HSEXTRA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TUNE(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TUNE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for CFG2 {
        #[inline(always)]
        fn default() -> CFG2 {
            CFG2(0)
        }
    }
    impl core::fmt::Debug for CFG2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CFG2")
                .field("JLEFT", &self.JLEFT())
                .field("HS", &self.HS())
                .field("HSEXTRA", &self.HSEXTRA())
                .field("TUNE", &self.TUNE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CFG2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CFG2 {{ JLEFT: {=bool:?}, HS: {=bool:?}, HSEXTRA: {=bool:?}, TUNE: {=u8:?} }}",
                self.JLEFT(),
                self.HS(),
                self.HSEXTRA(),
                self.TUNE()
            )
        }
    }
    #[doc = "Command High Buffer Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_CMDH(pub u32);
    impl CMD_CMDH {
        #[must_use]
        #[inline(always)]
        pub const fn CMPEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMPEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAIT_TRIG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAIT_TRIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LWI(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LWI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_STS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AVGS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AVGS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOOP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NEXT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for CMD_CMDH {
        #[inline(always)]
        fn default() -> CMD_CMDH {
            CMD_CMDH(0)
        }
    }
    impl core::fmt::Debug for CMD_CMDH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMD_CMDH")
                .field("CMPEN", &self.CMPEN())
                .field("WAIT_TRIG", &self.WAIT_TRIG())
                .field("LWI", &self.LWI())
                .field("STS", &self.STS())
                .field("AVGS", &self.AVGS())
                .field("LOOP", &self.LOOP())
                .field("NEXT", &self.NEXT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMD_CMDH {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CMD_CMDH {{ CMPEN: {=u8:?}, WAIT_TRIG: {=bool:?}, LWI: {=bool:?}, STS: {=u8:?}, AVGS: {=u8:?}, LOOP: {=u8:?}, NEXT: {=u8:?} }}" , self . CMPEN () , self . WAIT_TRIG () , self . LWI () , self . STS () , self . AVGS () , self . LOOP () , self . NEXT ())
        }
    }
    #[doc = "Command Low Buffer Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_CMDL(pub u32);
    impl CMD_CMDL {
        #[must_use]
        #[inline(always)]
        pub const fn ADCH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CTYPE(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CTYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MODE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CMD_CMDL {
        #[inline(always)]
        fn default() -> CMD_CMDL {
            CMD_CMDL(0)
        }
    }
    impl core::fmt::Debug for CMD_CMDL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CMD_CMDL")
                .field("ADCH", &self.ADCH())
                .field("CTYPE", &self.CTYPE())
                .field("MODE", &self.MODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CMD_CMDL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CMD_CMDL {{ ADCH: {=u8:?}, CTYPE: {=u8:?}, MODE: {=bool:?} }}",
                self.ADCH(),
                self.CTYPE(),
                self.MODE()
            )
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn ADCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DOZEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DOZEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAL_REQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAL_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CALOFS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CALOFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CALHS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CALHS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTFIFO0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTFIFO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAL_AVGS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CAL_AVGS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
                .field("ADCEN", &self.ADCEN())
                .field("RST", &self.RST())
                .field("DOZEN", &self.DOZEN())
                .field("CAL_REQ", &self.CAL_REQ())
                .field("CALOFS", &self.CALOFS())
                .field("CALHS", &self.CALHS())
                .field("RSTFIFO0", &self.RSTFIFO0())
                .field("CAL_AVGS", &self.CAL_AVGS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ ADCEN: {=bool:?}, RST: {=bool:?}, DOZEN: {=bool:?}, CAL_REQ: {=bool:?}, CALOFS: {=bool:?}, CALHS: {=bool:?}, RSTFIFO0: {=bool:?}, CAL_AVGS: {=u8:?} }}" , self . ADCEN () , self . RST () , self . DOZEN () , self . CAL_REQ () , self . CALOFS () , self . CALHS () , self . RSTFIFO0 () , self . CAL_AVGS ())
        }
    }
    #[doc = "Compare Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CV(pub u32);
    impl CV {
        #[must_use]
        #[inline(always)]
        pub const fn CVL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CVL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CVH(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_CVH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CV {
        #[inline(always)]
        fn default() -> CV {
            CV(0)
        }
    }
    impl core::fmt::Debug for CV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CV")
                .field("CVL", &self.CVL())
                .field("CVH", &self.CVH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CV {{ CVL: {=u16:?}, CVH: {=u16:?} }}",
                self.CVL(),
                self.CVH()
            )
        }
    }
    #[doc = "DMA Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DE(pub u32);
    impl DE {
        #[must_use]
        #[inline(always)]
        pub const fn FWMDE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FWMDE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DE {
        #[inline(always)]
        fn default() -> DE {
            DE(0)
        }
    }
    impl core::fmt::Debug for DE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DE")
                .field("FWMDE0", &self.FWMDE0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DE {{ FWMDE0: {=bool:?} }}", self.FWMDE0())
        }
    }
    #[doc = "FIFO Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u32);
    impl FCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn FCOUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FWMARK(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FWMARK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for FCTRL {
        #[inline(always)]
        fn default() -> FCTRL {
            FCTRL(0)
        }
    }
    impl core::fmt::Debug for FCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FCTRL")
                .field("FCOUNT", &self.FCOUNT())
                .field("FWMARK", &self.FWMARK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FCTRL {{ FCOUNT: {=u8:?}, FWMARK: {=u8:?} }}",
                self.FCOUNT(),
                self.FWMARK()
            )
        }
    }
    #[doc = "Gain Calibration Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCC(pub u32);
    impl GCC {
        #[must_use]
        #[inline(always)]
        pub const fn GAIN_CAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_GAIN_CAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDY(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for GCC {
        #[inline(always)]
        fn default() -> GCC {
            GCC(0)
        }
    }
    impl core::fmt::Debug for GCC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GCC")
                .field("GAIN_CAL", &self.GAIN_CAL())
                .field("RDY", &self.RDY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GCC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GCC {{ GAIN_CAL: {=u16:?}, RDY: {=bool:?} }}",
                self.GAIN_CAL(),
                self.RDY()
            )
        }
    }
    #[doc = "Gain Calculation Result"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCR(pub u32);
    impl GCR {
        #[must_use]
        #[inline(always)]
        pub const fn GCALR(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_GCALR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDY(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for GCR {
        #[inline(always)]
        fn default() -> GCR {
            GCR(0)
        }
    }
    impl core::fmt::Debug for GCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GCR")
                .field("GCALR", &self.GCALR())
                .field("RDY", &self.RDY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GCR {{ GCALR: {=u32:?}, RDY: {=bool:?} }}",
                self.GCALR(),
                self.RDY()
            )
        }
    }
    #[doc = "High Speed Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HSTRIM(pub u32);
    impl HSTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn HSTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HSTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for HSTRIM {
        #[inline(always)]
        fn default() -> HSTRIM {
            HSTRIM(0)
        }
    }
    impl core::fmt::Debug for HSTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HSTRIM")
                .field("HSTRIM", &self.HSTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HSTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HSTRIM {{ HSTRIM: {=u8:?} }}", self.HSTRIM())
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IE(pub u32);
    impl IE {
        #[must_use]
        #[inline(always)]
        pub const fn FWMIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FWMIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FOFIE0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FOFIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TEXC_IE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TEXC_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCOMP_IE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCOMP_IE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for IE {
        #[inline(always)]
        fn default() -> IE {
            IE(0)
        }
    }
    impl core::fmt::Debug for IE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IE")
                .field("FWMIE0", &self.FWMIE0())
                .field("FOFIE0", &self.FOFIE0())
                .field("TEXC_IE", &self.TEXC_IE())
                .field("TCOMP_IE", &self.TCOMP_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IE {{ FWMIE0: {=bool:?}, FOFIE0: {=bool:?}, TEXC_IE: {=bool:?}, TCOMP_IE: {=u8:?} }}" , self . FWMIE0 () , self . FOFIE0 () , self . TEXC_IE () , self . TCOMP_IE ())
        }
    }
    #[doc = "Offset Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFSTRIM(pub u32);
    impl OFSTRIM {
        #[must_use]
        #[inline(always)]
        pub const fn OFSTRIM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_OFSTRIM(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for OFSTRIM {
        #[inline(always)]
        fn default() -> OFSTRIM {
            OFSTRIM(0)
        }
    }
    impl core::fmt::Debug for OFSTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OFSTRIM")
                .field("OFSTRIM", &self.OFSTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OFSTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OFSTRIM {{ OFSTRIM: {=u16:?} }}", self.OFSTRIM())
        }
    }
    #[doc = "Parameter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[must_use]
        #[inline(always)]
        pub const fn TRIG_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRIG_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CV_NUM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CV_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMD_NUM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMD_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    impl core::fmt::Debug for PARAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PARAM")
                .field("TRIG_NUM", &self.TRIG_NUM())
                .field("FIFOSIZE", &self.FIFOSIZE())
                .field("CV_NUM", &self.CV_NUM())
                .field("CMD_NUM", &self.CMD_NUM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PARAM {{ TRIG_NUM: {=u8:?}, FIFOSIZE: {=u8:?}, CV_NUM: {=u8:?}, CMD_NUM: {=u8:?} }}" , self . TRIG_NUM () , self . FIFOSIZE () , self . CV_NUM () , self . CMD_NUM ())
        }
    }
    #[doc = "Pause Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PAUSE(pub u32);
    impl PAUSE {
        #[must_use]
        #[inline(always)]
        pub const fn PAUSEDLY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PAUSEDLY(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PAUSEEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PAUSEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PAUSE {
        #[inline(always)]
        fn default() -> PAUSE {
            PAUSE(0)
        }
    }
    impl core::fmt::Debug for PAUSE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PAUSE")
                .field("PAUSEDLY", &self.PAUSEDLY())
                .field("PAUSEEN", &self.PAUSEEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PAUSE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PAUSE {{ PAUSEDLY: {=u16:?}, PAUSEEN: {=bool:?} }}",
                self.PAUSEDLY(),
                self.PAUSEEN()
            )
        }
    }
    #[doc = "Data Result FIFO Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RESFIFO(pub u32);
    impl RESFIFO {
        #[must_use]
        #[inline(always)]
        pub const fn D(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_D(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSRC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOOPCNT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOOPCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDSRC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VALID(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RESFIFO {
        #[inline(always)]
        fn default() -> RESFIFO {
            RESFIFO(0)
        }
    }
    impl core::fmt::Debug for RESFIFO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RESFIFO")
                .field("D", &self.D())
                .field("TSRC", &self.TSRC())
                .field("LOOPCNT", &self.LOOPCNT())
                .field("CMDSRC", &self.CMDSRC())
                .field("VALID", &self.VALID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RESFIFO {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RESFIFO {{ D: {=u16:?}, TSRC: {=u8:?}, LOOPCNT: {=u8:?}, CMDSRC: {=u8:?}, VALID: {=bool:?} }}" , self . D () , self . TSRC () , self . LOOPCNT () , self . CMDSRC () , self . VALID ())
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[must_use]
        #[inline(always)]
        pub const fn RDY0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDY0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FOF0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FOF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TEXC_INT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TEXC_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCOMP_INT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TCOMP_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CAL_RDY(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CAL_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADC_ACTIVE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADC_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRGACT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRGACT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMDACT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMDACT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
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
                .field("RDY0", &self.RDY0())
                .field("FOF0", &self.FOF0())
                .field("TEXC_INT", &self.TEXC_INT())
                .field("TCOMP_INT", &self.TCOMP_INT())
                .field("CAL_RDY", &self.CAL_RDY())
                .field("ADC_ACTIVE", &self.ADC_ACTIVE())
                .field("TRGACT", &self.TRGACT())
                .field("CMDACT", &self.CMDACT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STAT {{ RDY0: {=bool:?}, FOF0: {=bool:?}, TEXC_INT: {=bool:?}, TCOMP_INT: {=bool:?}, CAL_RDY: {=bool:?}, ADC_ACTIVE: {=bool:?}, TRGACT: {=u8:?}, CMDACT: {=u8:?} }}" , self . RDY0 () , self . FOF0 () , self . TEXC_INT () , self . TCOMP_INT () , self . CAL_RDY () , self . ADC_ACTIVE () , self . TRGACT () , self . CMDACT ())
        }
    }
    #[doc = "Software Trigger Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWTRIG(pub u32);
    impl SWTRIG {
        #[must_use]
        #[inline(always)]
        pub const fn SWT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SWT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SWTRIG {
        #[inline(always)]
        fn default() -> SWTRIG {
            SWTRIG(0)
        }
    }
    impl core::fmt::Debug for SWTRIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWTRIG")
                .field("SWT0", &self.SWT0())
                .field("SWT1", &self.SWT1())
                .field("SWT2", &self.SWT2())
                .field("SWT3", &self.SWT3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWTRIG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SWTRIG {{ SWT0: {=bool:?}, SWT1: {=bool:?}, SWT2: {=bool:?}, SWT3: {=bool:?} }}",
                self.SWT0(),
                self.SWT1(),
                self.SWT2(),
                self.SWT3()
            )
        }
    }
    #[doc = "Trigger Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCTRL(pub u32);
    impl TCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn HTEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPRI(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPRI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSYNC(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSYNC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDLY(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCMD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for TCTRL {
        #[inline(always)]
        fn default() -> TCTRL {
            TCTRL(0)
        }
    }
    impl core::fmt::Debug for TCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TCTRL")
                .field("HTEN", &self.HTEN())
                .field("TPRI", &self.TPRI())
                .field("RSYNC", &self.RSYNC())
                .field("TDLY", &self.TDLY())
                .field("TCMD", &self.TCMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TCTRL {{ HTEN: {=bool:?}, TPRI: {=u8:?}, RSYNC: {=bool:?}, TDLY: {=u8:?}, TCMD: {=u8:?} }}" , self . HTEN () , self . TPRI () , self . RSYNC () , self . TDLY () , self . TCMD ())
        }
    }
    #[doc = "Trigger Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TSTAT(pub u32);
    impl TSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn TEXC_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TEXC_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCOMP_FLAG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCOMP_FLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TSTAT {
        #[inline(always)]
        fn default() -> TSTAT {
            TSTAT(0)
        }
    }
    impl core::fmt::Debug for TSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TSTAT")
                .field("TEXC_NUM", &self.TEXC_NUM())
                .field("TCOMP_FLAG", &self.TCOMP_FLAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TSTAT {{ TEXC_NUM: {=u8:?}, TCOMP_FLAG: {=u8:?} }}",
                self.TEXC_NUM(),
                self.TCOMP_FLAG()
            )
        }
    }
    #[doc = "Version ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[must_use]
        #[inline(always)]
        pub const fn RES(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIFFEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIFFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MVI(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MVI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CSW(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CSW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VR1RNGI(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VR1RNGI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IADCKI(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IADCKI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CALOFSI(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CALOFSI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUM_SEC(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NUM_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUM_FIFO(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUM_FIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
                .field("RES", &self.RES())
                .field("DIFFEN", &self.DIFFEN())
                .field("MVI", &self.MVI())
                .field("CSW", &self.CSW())
                .field("VR1RNGI", &self.VR1RNGI())
                .field("IADCKI", &self.IADCKI())
                .field("CALOFSI", &self.CALOFSI())
                .field("NUM_SEC", &self.NUM_SEC())
                .field("NUM_FIFO", &self.NUM_FIFO())
                .field("MINOR", &self.MINOR())
                .field("MAJOR", &self.MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VERID {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VERID {{ RES: {=bool:?}, DIFFEN: {=bool:?}, MVI: {=bool:?}, CSW: {=u8:?}, VR1RNGI: {=bool:?}, IADCKI: {=bool:?}, CALOFSI: {=bool:?}, NUM_SEC: {=bool:?}, NUM_FIFO: {=u8:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}" , self . RES () , self . DIFFEN () , self . MVI () , self . CSW () , self . VR1RNGI () , self . IADCKI () , self . CALOFSI () , self . NUM_SEC () , self . NUM_FIFO () , self . MINOR () , self . MAJOR ())
        }
    }
}
