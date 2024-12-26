#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
        unsafe { CMD::from_ptr(self.ptr.add(0x0100usize + n * 56usize) as _) }
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
    pub const fn CAL_GAR0(self) -> crate::common::Reg<regs::CAL_GAR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR1(self) -> crate::common::Reg<regs::CAL_GAR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR2(self) -> crate::common::Reg<regs::CAL_GAR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR3(self) -> crate::common::Reg<regs::CAL_GAR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR4(self) -> crate::common::Reg<regs::CAL_GAR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR5(self) -> crate::common::Reg<regs::CAL_GAR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR6(self) -> crate::common::Reg<regs::CAL_GAR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR7(self) -> crate::common::Reg<regs::CAL_GAR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR8(self) -> crate::common::Reg<regs::CAL_GAR8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR9(self) -> crate::common::Reg<regs::CAL_GAR9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR10(self) -> crate::common::Reg<regs::CAL_GAR10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR11(self) -> crate::common::Reg<regs::CAL_GAR11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR12(self) -> crate::common::Reg<regs::CAL_GAR12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR13(self) -> crate::common::Reg<regs::CAL_GAR13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR14(self) -> crate::common::Reg<regs::CAL_GAR14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR15(self) -> crate::common::Reg<regs::CAL_GAR15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR16(self) -> crate::common::Reg<regs::CAL_GAR16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR17(self) -> crate::common::Reg<regs::CAL_GAR17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR18(self) -> crate::common::Reg<regs::CAL_GAR18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR19(self) -> crate::common::Reg<regs::CAL_GAR19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR20(self) -> crate::common::Reg<regs::CAL_GAR20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR21(self) -> crate::common::Reg<regs::CAL_GAR21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR22(self) -> crate::common::Reg<regs::CAL_GAR22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR23(self) -> crate::common::Reg<regs::CAL_GAR23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR24(self) -> crate::common::Reg<regs::CAL_GAR24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR25(self) -> crate::common::Reg<regs::CAL_GAR25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR26(self) -> crate::common::Reg<regs::CAL_GAR26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR27(self) -> crate::common::Reg<regs::CAL_GAR27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x046cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR28(self) -> crate::common::Reg<regs::CAL_GAR28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR29(self) -> crate::common::Reg<regs::CAL_GAR29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0474usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR30(self) -> crate::common::Reg<regs::CAL_GAR30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0478usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR31(self) -> crate::common::Reg<regs::CAL_GAR31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x047cusize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR32(self) -> crate::common::Reg<regs::CAL_GAR32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[inline(always)]
    pub const fn CAL_GAR33(self) -> crate::common::Reg<regs::CAL_GAR33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
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
    pub struct CAL_GAR0(pub u32);
    impl CAL_GAR0 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR0 {
        #[inline(always)]
        fn default() -> CAL_GAR0 {
            CAL_GAR0(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR1(pub u32);
    impl CAL_GAR1 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR1 {
        #[inline(always)]
        fn default() -> CAL_GAR1 {
            CAL_GAR1(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR10(pub u32);
    impl CAL_GAR10 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR10 {
        #[inline(always)]
        fn default() -> CAL_GAR10 {
            CAL_GAR10(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR11(pub u32);
    impl CAL_GAR11 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR11 {
        #[inline(always)]
        fn default() -> CAL_GAR11 {
            CAL_GAR11(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR12(pub u32);
    impl CAL_GAR12 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR12 {
        #[inline(always)]
        fn default() -> CAL_GAR12 {
            CAL_GAR12(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR13(pub u32);
    impl CAL_GAR13 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR13 {
        #[inline(always)]
        fn default() -> CAL_GAR13 {
            CAL_GAR13(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR14(pub u32);
    impl CAL_GAR14 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR14 {
        #[inline(always)]
        fn default() -> CAL_GAR14 {
            CAL_GAR14(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR15(pub u32);
    impl CAL_GAR15 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR15 {
        #[inline(always)]
        fn default() -> CAL_GAR15 {
            CAL_GAR15(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR16(pub u32);
    impl CAL_GAR16 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR16 {
        #[inline(always)]
        fn default() -> CAL_GAR16 {
            CAL_GAR16(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR17(pub u32);
    impl CAL_GAR17 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR17 {
        #[inline(always)]
        fn default() -> CAL_GAR17 {
            CAL_GAR17(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR18(pub u32);
    impl CAL_GAR18 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR18 {
        #[inline(always)]
        fn default() -> CAL_GAR18 {
            CAL_GAR18(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR19(pub u32);
    impl CAL_GAR19 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR19 {
        #[inline(always)]
        fn default() -> CAL_GAR19 {
            CAL_GAR19(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR2(pub u32);
    impl CAL_GAR2 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR2 {
        #[inline(always)]
        fn default() -> CAL_GAR2 {
            CAL_GAR2(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR20(pub u32);
    impl CAL_GAR20 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR20 {
        #[inline(always)]
        fn default() -> CAL_GAR20 {
            CAL_GAR20(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR21(pub u32);
    impl CAL_GAR21 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR21 {
        #[inline(always)]
        fn default() -> CAL_GAR21 {
            CAL_GAR21(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR22(pub u32);
    impl CAL_GAR22 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR22 {
        #[inline(always)]
        fn default() -> CAL_GAR22 {
            CAL_GAR22(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR23(pub u32);
    impl CAL_GAR23 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR23 {
        #[inline(always)]
        fn default() -> CAL_GAR23 {
            CAL_GAR23(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR24(pub u32);
    impl CAL_GAR24 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR24 {
        #[inline(always)]
        fn default() -> CAL_GAR24 {
            CAL_GAR24(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR25(pub u32);
    impl CAL_GAR25 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR25 {
        #[inline(always)]
        fn default() -> CAL_GAR25 {
            CAL_GAR25(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR26(pub u32);
    impl CAL_GAR26 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR26 {
        #[inline(always)]
        fn default() -> CAL_GAR26 {
            CAL_GAR26(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR27(pub u32);
    impl CAL_GAR27 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR27 {
        #[inline(always)]
        fn default() -> CAL_GAR27 {
            CAL_GAR27(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR28(pub u32);
    impl CAL_GAR28 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR28 {
        #[inline(always)]
        fn default() -> CAL_GAR28 {
            CAL_GAR28(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR29(pub u32);
    impl CAL_GAR29 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR29 {
        #[inline(always)]
        fn default() -> CAL_GAR29 {
            CAL_GAR29(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR3(pub u32);
    impl CAL_GAR3 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR3 {
        #[inline(always)]
        fn default() -> CAL_GAR3 {
            CAL_GAR3(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR30(pub u32);
    impl CAL_GAR30 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR30 {
        #[inline(always)]
        fn default() -> CAL_GAR30 {
            CAL_GAR30(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR31(pub u32);
    impl CAL_GAR31 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR31 {
        #[inline(always)]
        fn default() -> CAL_GAR31 {
            CAL_GAR31(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR32(pub u32);
    impl CAL_GAR32 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR32 {
        #[inline(always)]
        fn default() -> CAL_GAR32 {
            CAL_GAR32(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR33(pub u32);
    impl CAL_GAR33 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR33 {
        #[inline(always)]
        fn default() -> CAL_GAR33 {
            CAL_GAR33(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR4(pub u32);
    impl CAL_GAR4 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR4 {
        #[inline(always)]
        fn default() -> CAL_GAR4 {
            CAL_GAR4(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR5(pub u32);
    impl CAL_GAR5 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR5 {
        #[inline(always)]
        fn default() -> CAL_GAR5 {
            CAL_GAR5(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR6(pub u32);
    impl CAL_GAR6 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR6 {
        #[inline(always)]
        fn default() -> CAL_GAR6 {
            CAL_GAR6(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR7(pub u32);
    impl CAL_GAR7 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR7 {
        #[inline(always)]
        fn default() -> CAL_GAR7 {
            CAL_GAR7(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR8(pub u32);
    impl CAL_GAR8 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR8 {
        #[inline(always)]
        fn default() -> CAL_GAR8 {
            CAL_GAR8(0)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAL_GAR9(pub u32);
    impl CAL_GAR9 {
        #[inline(always)]
        pub const fn CAL_GAR_VAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAL_GAR_VAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAL_GAR9 {
        #[inline(always)]
        fn default() -> CAL_GAR9 {
            CAL_GAR9(0)
        }
    }
    #[doc = "Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG(pub u32);
    impl CFG {
        #[inline(always)]
        pub const fn TPRICTRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TPRICTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn PWRSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWRSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REFSEL(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_REFSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn TRES(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TCMDRES(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCMDRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn HPT_EXDI(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HPT_EXDI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn PUDLY(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PUDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn PWREN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for CFG {
        #[inline(always)]
        fn default() -> CFG {
            CFG(0)
        }
    }
    #[doc = "Configuration 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CFG2(pub u32);
    impl CFG2 {
        #[inline(always)]
        pub const fn JLEFT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_JLEFT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn HS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn HSEXTRA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSEXTRA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TUNE(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TUNE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for CFG2 {
        #[inline(always)]
        fn default() -> CFG2 {
            CFG2(0)
        }
    }
    #[doc = "Command High Buffer Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_CMDH(pub u32);
    impl CMD_CMDH {
        #[inline(always)]
        pub const fn CMPEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMPEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn WAIT_TRIG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAIT_TRIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn LWI(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LWI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn STS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_STS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn AVGS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_AVGS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn LOOP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn NEXT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_NEXT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for CMD_CMDH {
        #[inline(always)]
        fn default() -> CMD_CMDH {
            CMD_CMDH(0)
        }
    }
    #[doc = "Command Low Buffer Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CMD_CMDL(pub u32);
    impl CMD_CMDL {
        #[inline(always)]
        pub const fn ADCH(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ADCH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn CTYPE(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[inline(always)]
        pub const fn MODE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CMD_CMDL {
        #[inline(always)]
        fn default() -> CMD_CMDL {
            CMD_CMDL(0)
        }
    }
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn ADCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DOZEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CAL_REQ(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAL_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CALOFS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CALOFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CALHS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CALHS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RSTFIFO0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSTFIFO0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn CAL_AVGS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAL_AVGS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CTRL {
        #[inline(always)]
        fn default() -> CTRL {
            CTRL(0)
        }
    }
    #[doc = "Compare Value Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CV(pub u32);
    impl CV {
        #[inline(always)]
        pub const fn CVL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CVL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn CVH(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CVH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CV {
        #[inline(always)]
        fn default() -> CV {
            CV(0)
        }
    }
    #[doc = "DMA Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DE(pub u32);
    impl DE {
        #[inline(always)]
        pub const fn FWMDE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWMDE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DE {
        #[inline(always)]
        fn default() -> DE {
            DE(0)
        }
    }
    #[doc = "FIFO Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCTRL(pub u32);
    impl FCTRL {
        #[inline(always)]
        pub const fn FCOUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FWMARK(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_FWMARK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for FCTRL {
        #[inline(always)]
        fn default() -> FCTRL {
            FCTRL(0)
        }
    }
    #[doc = "Gain Calibration Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCC(pub u32);
    impl GCC {
        #[inline(always)]
        pub const fn GAIN_CAL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GAIN_CAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn RDY(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for GCC {
        #[inline(always)]
        fn default() -> GCC {
            GCC(0)
        }
    }
    #[doc = "Gain Calculation Result"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GCR(pub u32);
    impl GCR {
        #[inline(always)]
        pub const fn GCALR(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GCALR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn RDY(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for GCR {
        #[inline(always)]
        fn default() -> GCR {
            GCR(0)
        }
    }
    #[doc = "High Speed Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HSTRIM(pub u32);
    impl HSTRIM {
        #[inline(always)]
        pub const fn HSTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_HSTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for HSTRIM {
        #[inline(always)]
        fn default() -> HSTRIM {
            HSTRIM(0)
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IE(pub u32);
    impl IE {
        #[inline(always)]
        pub const fn FWMIE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FWMIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FOFIE0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOFIE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TEXC_IE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEXC_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TCOMP_IE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCOMP_IE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for IE {
        #[inline(always)]
        fn default() -> IE {
            IE(0)
        }
    }
    #[doc = "Offset Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OFSTRIM(pub u32);
    impl OFSTRIM {
        #[inline(always)]
        pub const fn OFSTRIM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_OFSTRIM(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for OFSTRIM {
        #[inline(always)]
        fn default() -> OFSTRIM {
            OFSTRIM(0)
        }
    }
    #[doc = "Parameter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn TRIG_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIG_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn FIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn CV_NUM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CV_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn CMD_NUM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMD_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PARAM {
        #[inline(always)]
        fn default() -> PARAM {
            PARAM(0)
        }
    }
    #[doc = "Pause Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PAUSE(pub u32);
    impl PAUSE {
        #[inline(always)]
        pub const fn PAUSEDLY(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PAUSEDLY(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[inline(always)]
        pub const fn PAUSEEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PAUSEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PAUSE {
        #[inline(always)]
        fn default() -> PAUSE {
            PAUSE(0)
        }
    }
    #[doc = "Data Result FIFO Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RESFIFO(pub u32);
    impl RESFIFO {
        #[inline(always)]
        pub const fn D(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_D(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn TSRC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn LOOPCNT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOOPCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn CMDSRC(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMDSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[inline(always)]
        pub const fn VALID(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RESFIFO {
        #[inline(always)]
        fn default() -> RESFIFO {
            RESFIFO(0)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAT(pub u32);
    impl STAT {
        #[inline(always)]
        pub const fn RDY0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDY0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FOF0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FOF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TEXC_INT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEXC_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TCOMP_INT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TCOMP_INT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CAL_RDY(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAL_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ADC_ACTIVE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADC_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TRGACT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRGACT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn CMDACT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMDACT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for STAT {
        #[inline(always)]
        fn default() -> STAT {
            STAT(0)
        }
    }
    #[doc = "Software Trigger Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWTRIG(pub u32);
    impl SWTRIG {
        #[inline(always)]
        pub const fn SWT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SWT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SWT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn SWT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SWTRIG {
        #[inline(always)]
        fn default() -> SWTRIG {
            SWTRIG(0)
        }
    }
    #[doc = "Trigger Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TCTRL(pub u32);
    impl TCTRL {
        #[inline(always)]
        pub const fn HTEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TPRI(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TPRI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn RSYNC(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSYNC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn TDLY(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TDLY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn TSYNC(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSYNC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn TCMD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for TCTRL {
        #[inline(always)]
        fn default() -> TCTRL {
            TCTRL(0)
        }
    }
    #[doc = "Trigger Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TSTAT(pub u32);
    impl TSTAT {
        #[inline(always)]
        pub const fn TEXC_NUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEXC_NUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn TCOMP_FLAG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCOMP_FLAG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for TSTAT {
        #[inline(always)]
        fn default() -> TSTAT {
            TSTAT(0)
        }
    }
    #[doc = "Version ID Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[inline(always)]
        pub const fn RES(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DIFFEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIFFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MVI(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MVI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CSW(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CSW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[inline(always)]
        pub const fn VR1RNGI(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VR1RNGI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IADCKI(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IADCKI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn CALOFSI(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CALOFSI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn NUM_SEC(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NUM_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn NUM_FIFO(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_NUM_FIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
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
}
