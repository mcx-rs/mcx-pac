#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EV {
    ptr: *mut u8,
}
unsafe impl Send for EV {}
unsafe impl Sync for EV {}
impl EV {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn STATE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::EV_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT {
    ptr: *mut u8,
}
unsafe impl Send for OUT {}
unsafe impl Sync for OUT {}
impl OUT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SET(self) -> crate::common::Reg<regs::OUT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CLR(self) -> crate::common::Reg<regs::OUT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCT {
    ptr: *mut u8,
}
unsafe impl Send for SCT {}
unsafe impl Sync for SCT {}
impl SCT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRLL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRLH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn LIMIT(self) -> crate::common::Reg<regs::LIMIT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn LIMITL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn LIMITH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn HALT(self) -> crate::common::Reg<regs::HALT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn HALTL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn HALTH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn STOP(self) -> crate::common::Reg<regs::STOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STOPL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STOPH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[inline(always)]
    pub const fn START(self) -> crate::common::Reg<regs::START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn STARTL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn STARTH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[inline(always)]
    pub const fn DITHER(self) -> crate::common::Reg<regs::DITHER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn COUNT(self) -> crate::common::Reg<regs::COUNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn COUNTL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn COUNTH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[inline(always)]
    pub const fn STATE(self) -> crate::common::Reg<regs::STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn STATEL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn STATEH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x46usize) as _) }
    }
    #[inline(always)]
    pub const fn INPUT(self) -> crate::common::Reg<regs::INPUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn REGMODE(self) -> crate::common::Reg<regs::REGMODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn REGMODEL(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn REGMODEH(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4eusize) as _) }
    }
    #[inline(always)]
    pub const fn OUTPUT(self) -> crate::common::Reg<regs::OUTPUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn OUTPUTDIRCTRL(self) -> crate::common::Reg<regs::OUTPUTDIRCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn RES(self) -> crate::common::Reg<regs::RES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn DMAREQ0(self) -> crate::common::Reg<regs::DMAREQ0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMAREQ1(self) -> crate::common::Reg<regs::DMAREQ1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn EVEN(self) -> crate::common::Reg<regs::EVEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn EVFLAG(self) -> crate::common::Reg<regs::EVFLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn CONEN(self) -> crate::common::Reg<regs::CONEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn CONFLAG(self) -> crate::common::Reg<regs::CONFLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn MergeUnion(self, n: usize) -> crate::common::Reg<regs::CAP, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MergeUnion(self, n: usize) -> crate::common::Reg<regs::MATCH, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FRACMAT(self, n: usize) -> crate::common::Reg<regs::FRACMAT, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MergeUnion(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MergeUnion(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MATCHREL, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FRACMATREL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FRACMATREL, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn EV(self, n: usize) -> EV {
        assert!(n < 16usize);
        unsafe { EV::from_ptr(self.ptr.add(0x0300usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn OUT(self, n: usize) -> OUT {
        assert!(n < 10usize);
        unsafe { OUT::from_ptr(self.ptr.add(0x0500usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Capture Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAP(pub u32);
    impl CAP {
        #[inline(always)]
        pub const fn CAPn_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPn_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn CAPn_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPn_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CAP {
        #[inline(always)]
        fn default() -> CAP {
            CAP(0)
        }
    }
    impl core::fmt::Debug for CAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAP")
                .field("CAPn_L", &self.CAPn_L())
                .field("CAPn_H", &self.CAPn_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAP {
                CAPn_L: u16,
                CAPn_H: u16,
            }
            let proxy = CAP {
                CAPn_L: self.CAPn_L(),
                CAPn_H: self.CAPn_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Capture Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPCTRL(pub u32);
    impl CAPCTRL {
        #[inline(always)]
        pub const fn CAPCONn_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPCONn_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn CAPCONn_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPCONn_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CAPCTRL {
        #[inline(always)]
        fn default() -> CAPCTRL {
            CAPCTRL(0)
        }
    }
    impl core::fmt::Debug for CAPCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAPCTRL")
                .field("CAPCONn_L", &self.CAPCONn_L())
                .field("CAPCONn_H", &self.CAPCONn_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAPCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAPCTRL {
                CAPCONn_L: u16,
                CAPCONn_H: u16,
            }
            let proxy = CAPCTRL {
                CAPCONn_L: self.CAPCONn_L(),
                CAPCONn_H: self.CAPCONn_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CAPCTRLH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPCTRL_ACCESS16BIT_CAPCTRLH(pub u32);
    impl CAPCTRL_ACCESS16BIT_CAPCTRLH {
        #[inline(always)]
        pub const fn CAPCTRLH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPCTRLH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAPCTRL_ACCESS16BIT_CAPCTRLH {
        #[inline(always)]
        fn default() -> CAPCTRL_ACCESS16BIT_CAPCTRLH {
            CAPCTRL_ACCESS16BIT_CAPCTRLH(0)
        }
    }
    impl core::fmt::Debug for CAPCTRL_ACCESS16BIT_CAPCTRLH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAPCTRL_ACCESS16BIT_CAPCTRLH")
                .field("CAPCTRLH", &self.CAPCTRLH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAPCTRL_ACCESS16BIT_CAPCTRLH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAPCTRL_ACCESS16BIT_CAPCTRLH {
                CAPCTRLH: u16,
            }
            let proxy = CAPCTRL_ACCESS16BIT_CAPCTRLH {
                CAPCTRLH: self.CAPCTRLH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CAPCTRLL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAPCTRL_ACCESS16BIT_CAPCTRLL(pub u32);
    impl CAPCTRL_ACCESS16BIT_CAPCTRLL {
        #[inline(always)]
        pub const fn CAPCTRLL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPCTRLL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAPCTRL_ACCESS16BIT_CAPCTRLL {
        #[inline(always)]
        fn default() -> CAPCTRL_ACCESS16BIT_CAPCTRLL {
            CAPCTRL_ACCESS16BIT_CAPCTRLL(0)
        }
    }
    impl core::fmt::Debug for CAPCTRL_ACCESS16BIT_CAPCTRLL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAPCTRL_ACCESS16BIT_CAPCTRLL")
                .field("CAPCTRLL", &self.CAPCTRLL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAPCTRL_ACCESS16BIT_CAPCTRLL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAPCTRL_ACCESS16BIT_CAPCTRLL {
                CAPCTRLL: u16,
            }
            let proxy = CAPCTRL_ACCESS16BIT_CAPCTRLL {
                CAPCTRLL: self.CAPCTRLL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CAPH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAP_ACCESS16BIT_CAPH(pub u32);
    impl CAP_ACCESS16BIT_CAPH {
        #[inline(always)]
        pub const fn CAPH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAP_ACCESS16BIT_CAPH {
        #[inline(always)]
        fn default() -> CAP_ACCESS16BIT_CAPH {
            CAP_ACCESS16BIT_CAPH(0)
        }
    }
    impl core::fmt::Debug for CAP_ACCESS16BIT_CAPH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAP_ACCESS16BIT_CAPH")
                .field("CAPH", &self.CAPH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAP_ACCESS16BIT_CAPH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAP_ACCESS16BIT_CAPH {
                CAPH: u16,
            }
            let proxy = CAP_ACCESS16BIT_CAPH { CAPH: self.CAPH() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CAPL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CAP_ACCESS16BIT_CAPL(pub u32);
    impl CAP_ACCESS16BIT_CAPL {
        #[inline(always)]
        pub const fn CAPL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CAPL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CAP_ACCESS16BIT_CAPL {
        #[inline(always)]
        fn default() -> CAP_ACCESS16BIT_CAPL {
            CAP_ACCESS16BIT_CAPL(0)
        }
    }
    impl core::fmt::Debug for CAP_ACCESS16BIT_CAPL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CAP_ACCESS16BIT_CAPL")
                .field("CAPL", &self.CAPL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CAP_ACCESS16BIT_CAPL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CAP_ACCESS16BIT_CAPL {
                CAPL: u16,
            }
            let proxy = CAP_ACCESS16BIT_CAPL { CAPL: self.CAPL() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Conflict Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONEN(pub u32);
    impl CONEN {
        #[inline(always)]
        pub const fn NCEN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NCEN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn NCEN2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NCEN3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn NCEN4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn NCEN5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn NCEN6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn NCEN7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NCEN8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn NCEN9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCEN9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for CONEN {
        #[inline(always)]
        fn default() -> CONEN {
            CONEN(0)
        }
    }
    impl core::fmt::Debug for CONEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONEN")
                .field("NCEN0", &self.NCEN0())
                .field("NCEN1", &self.NCEN1())
                .field("NCEN2", &self.NCEN2())
                .field("NCEN3", &self.NCEN3())
                .field("NCEN4", &self.NCEN4())
                .field("NCEN5", &self.NCEN5())
                .field("NCEN6", &self.NCEN6())
                .field("NCEN7", &self.NCEN7())
                .field("NCEN8", &self.NCEN8())
                .field("NCEN9", &self.NCEN9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONEN {
                NCEN0: bool,
                NCEN1: bool,
                NCEN2: bool,
                NCEN3: bool,
                NCEN4: bool,
                NCEN5: bool,
                NCEN6: bool,
                NCEN7: bool,
                NCEN8: bool,
                NCEN9: bool,
            }
            let proxy = CONEN {
                NCEN0: self.NCEN0(),
                NCEN1: self.NCEN1(),
                NCEN2: self.NCEN2(),
                NCEN3: self.NCEN3(),
                NCEN4: self.NCEN4(),
                NCEN5: self.NCEN5(),
                NCEN6: self.NCEN6(),
                NCEN7: self.NCEN7(),
                NCEN8: self.NCEN8(),
                NCEN9: self.NCEN9(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIG(pub u32);
    impl CONFIG {
        #[inline(always)]
        pub const fn UNIFY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNIFY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLKMODE(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[inline(always)]
        pub const fn CKSEL(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CKSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[inline(always)]
        pub const fn NORELOAD_L(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NORELOAD_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NORELOAD_H(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NORELOAD_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn INSYNC(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INSYNC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
        }
        #[inline(always)]
        pub const fn AUTOLIMIT_L(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTOLIMIT_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn AUTOLIMIT_H(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTOLIMIT_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for CONFIG {
        #[inline(always)]
        fn default() -> CONFIG {
            CONFIG(0)
        }
    }
    impl core::fmt::Debug for CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIG")
                .field("UNIFY", &self.UNIFY())
                .field("CLKMODE", &self.CLKMODE())
                .field("CKSEL", &self.CKSEL())
                .field("NORELOAD_L", &self.NORELOAD_L())
                .field("NORELOAD_H", &self.NORELOAD_H())
                .field("INSYNC", &self.INSYNC())
                .field("AUTOLIMIT_L", &self.AUTOLIMIT_L())
                .field("AUTOLIMIT_H", &self.AUTOLIMIT_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONFIG {
                UNIFY: bool,
                CLKMODE: u8,
                CKSEL: u8,
                NORELOAD_L: bool,
                NORELOAD_H: bool,
                INSYNC: u8,
                AUTOLIMIT_L: bool,
                AUTOLIMIT_H: bool,
            }
            let proxy = CONFIG {
                UNIFY: self.UNIFY(),
                CLKMODE: self.CLKMODE(),
                CKSEL: self.CKSEL(),
                NORELOAD_L: self.NORELOAD_L(),
                NORELOAD_H: self.NORELOAD_H(),
                INSYNC: self.INSYNC(),
                AUTOLIMIT_L: self.AUTOLIMIT_L(),
                AUTOLIMIT_H: self.AUTOLIMIT_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Conflict Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFLAG(pub u32);
    impl CONFLAG {
        #[inline(always)]
        pub const fn NCFLAG0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NCFLAG1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn NCFLAG2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NCFLAG3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn NCFLAG4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn NCFLAG5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn NCFLAG6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn NCFLAG7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NCFLAG8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn NCFLAG9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NCFLAG9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn BUSERRL(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUSERRL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn BUSERRH(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUSERRH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CONFLAG {
        #[inline(always)]
        fn default() -> CONFLAG {
            CONFLAG(0)
        }
    }
    impl core::fmt::Debug for CONFLAG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFLAG")
                .field("NCFLAG0", &self.NCFLAG0())
                .field("NCFLAG1", &self.NCFLAG1())
                .field("NCFLAG2", &self.NCFLAG2())
                .field("NCFLAG3", &self.NCFLAG3())
                .field("NCFLAG4", &self.NCFLAG4())
                .field("NCFLAG5", &self.NCFLAG5())
                .field("NCFLAG6", &self.NCFLAG6())
                .field("NCFLAG7", &self.NCFLAG7())
                .field("NCFLAG8", &self.NCFLAG8())
                .field("NCFLAG9", &self.NCFLAG9())
                .field("BUSERRL", &self.BUSERRL())
                .field("BUSERRH", &self.BUSERRH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFLAG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONFLAG {
                NCFLAG0: bool,
                NCFLAG1: bool,
                NCFLAG2: bool,
                NCFLAG3: bool,
                NCFLAG4: bool,
                NCFLAG5: bool,
                NCFLAG6: bool,
                NCFLAG7: bool,
                NCFLAG8: bool,
                NCFLAG9: bool,
                BUSERRL: bool,
                BUSERRH: bool,
            }
            let proxy = CONFLAG {
                NCFLAG0: self.NCFLAG0(),
                NCFLAG1: self.NCFLAG1(),
                NCFLAG2: self.NCFLAG2(),
                NCFLAG3: self.NCFLAG3(),
                NCFLAG4: self.NCFLAG4(),
                NCFLAG5: self.NCFLAG5(),
                NCFLAG6: self.NCFLAG6(),
                NCFLAG7: self.NCFLAG7(),
                NCFLAG8: self.NCFLAG8(),
                NCFLAG9: self.NCFLAG9(),
                BUSERRL: self.BUSERRL(),
                BUSERRH: self.BUSERRH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Counter Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct COUNT(pub u32);
    impl COUNT {
        #[inline(always)]
        pub const fn CTR_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CTR_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn CTR_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CTR_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for COUNT {
        #[inline(always)]
        fn default() -> COUNT {
            COUNT(0)
        }
    }
    impl core::fmt::Debug for COUNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("COUNT")
                .field("CTR_L", &self.CTR_L())
                .field("CTR_H", &self.CTR_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for COUNT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct COUNT {
                CTR_L: u16,
                CTR_H: u16,
            }
            let proxy = COUNT {
                CTR_L: self.CTR_L(),
                CTR_H: self.CTR_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_COUNTH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct COUNT_ACCESS16BIT_COUNTH(pub u32);
    impl COUNT_ACCESS16BIT_COUNTH {
        #[inline(always)]
        pub const fn COUNTH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_COUNTH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for COUNT_ACCESS16BIT_COUNTH {
        #[inline(always)]
        fn default() -> COUNT_ACCESS16BIT_COUNTH {
            COUNT_ACCESS16BIT_COUNTH(0)
        }
    }
    impl core::fmt::Debug for COUNT_ACCESS16BIT_COUNTH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("COUNT_ACCESS16BIT_COUNTH")
                .field("COUNTH", &self.COUNTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for COUNT_ACCESS16BIT_COUNTH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct COUNT_ACCESS16BIT_COUNTH {
                COUNTH: u16,
            }
            let proxy = COUNT_ACCESS16BIT_COUNTH {
                COUNTH: self.COUNTH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_COUNTL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct COUNT_ACCESS16BIT_COUNTL(pub u32);
    impl COUNT_ACCESS16BIT_COUNTL {
        #[inline(always)]
        pub const fn COUNTL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_COUNTL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for COUNT_ACCESS16BIT_COUNTL {
        #[inline(always)]
        fn default() -> COUNT_ACCESS16BIT_COUNTL {
            COUNT_ACCESS16BIT_COUNTL(0)
        }
    }
    impl core::fmt::Debug for COUNT_ACCESS16BIT_COUNTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("COUNT_ACCESS16BIT_COUNTL")
                .field("COUNTL", &self.COUNTL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for COUNT_ACCESS16BIT_COUNTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct COUNT_ACCESS16BIT_COUNTL {
                COUNTL: u16,
            }
            let proxy = COUNT_ACCESS16BIT_COUNTL {
                COUNTL: self.COUNTL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn DOWN_L(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOWN_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STOP_L(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOP_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HALT_L(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLRCTR_L(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRCTR_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BIDIR_L(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIDIR_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PRE_L(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRE_L(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
        #[inline(always)]
        pub const fn DOWN_H(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOWN_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn STOP_H(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOP_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn HALT_H(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn CLRCTR_H(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRCTR_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn BIDIR_H(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIDIR_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn PRE_H(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRE_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
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
                .field("DOWN_L", &self.DOWN_L())
                .field("STOP_L", &self.STOP_L())
                .field("HALT_L", &self.HALT_L())
                .field("CLRCTR_L", &self.CLRCTR_L())
                .field("BIDIR_L", &self.BIDIR_L())
                .field("PRE_L", &self.PRE_L())
                .field("DOWN_H", &self.DOWN_H())
                .field("STOP_H", &self.STOP_H())
                .field("HALT_H", &self.HALT_H())
                .field("CLRCTR_H", &self.CLRCTR_H())
                .field("BIDIR_H", &self.BIDIR_H())
                .field("PRE_H", &self.PRE_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL {
                DOWN_L: bool,
                STOP_L: bool,
                HALT_L: bool,
                CLRCTR_L: bool,
                BIDIR_L: bool,
                PRE_L: u8,
                DOWN_H: bool,
                STOP_H: bool,
                HALT_H: bool,
                CLRCTR_H: bool,
                BIDIR_H: bool,
                PRE_H: u8,
            }
            let proxy = CTRL {
                DOWN_L: self.DOWN_L(),
                STOP_L: self.STOP_L(),
                HALT_L: self.HALT_L(),
                CLRCTR_L: self.CLRCTR_L(),
                BIDIR_L: self.BIDIR_L(),
                PRE_L: self.PRE_L(),
                DOWN_H: self.DOWN_H(),
                STOP_H: self.STOP_H(),
                HALT_H: self.HALT_H(),
                CLRCTR_H: self.CLRCTR_H(),
                BIDIR_H: self.BIDIR_H(),
                PRE_H: self.PRE_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CTRLH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_ACCESS16BIT_CTRLH(pub u32);
    impl CTRL_ACCESS16BIT_CTRLH {
        #[inline(always)]
        pub const fn DOWN_H(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOWN_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STOP_H(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOP_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HALT_H(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLRCTR_H(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRCTR_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BIDIR_H(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIDIR_H(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PRE_H(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRE_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
    }
    impl Default for CTRL_ACCESS16BIT_CTRLH {
        #[inline(always)]
        fn default() -> CTRL_ACCESS16BIT_CTRLH {
            CTRL_ACCESS16BIT_CTRLH(0)
        }
    }
    impl core::fmt::Debug for CTRL_ACCESS16BIT_CTRLH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_ACCESS16BIT_CTRLH")
                .field("DOWN_H", &self.DOWN_H())
                .field("STOP_H", &self.STOP_H())
                .field("HALT_H", &self.HALT_H())
                .field("CLRCTR_H", &self.CLRCTR_H())
                .field("BIDIR_H", &self.BIDIR_H())
                .field("PRE_H", &self.PRE_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_ACCESS16BIT_CTRLH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_ACCESS16BIT_CTRLH {
                DOWN_H: bool,
                STOP_H: bool,
                HALT_H: bool,
                CLRCTR_H: bool,
                BIDIR_H: bool,
                PRE_H: u8,
            }
            let proxy = CTRL_ACCESS16BIT_CTRLH {
                DOWN_H: self.DOWN_H(),
                STOP_H: self.STOP_H(),
                HALT_H: self.HALT_H(),
                CLRCTR_H: self.CLRCTR_H(),
                BIDIR_H: self.BIDIR_H(),
                PRE_H: self.PRE_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_CTRLL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL_ACCESS16BIT_CTRLL(pub u32);
    impl CTRL_ACCESS16BIT_CTRLL {
        #[inline(always)]
        pub const fn DOWN_L(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOWN_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn STOP_L(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOP_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HALT_L(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLRCTR_L(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRCTR_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BIDIR_L(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BIDIR_L(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PRE_L(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRE_L(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
    }
    impl Default for CTRL_ACCESS16BIT_CTRLL {
        #[inline(always)]
        fn default() -> CTRL_ACCESS16BIT_CTRLL {
            CTRL_ACCESS16BIT_CTRLL(0)
        }
    }
    impl core::fmt::Debug for CTRL_ACCESS16BIT_CTRLL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTRL_ACCESS16BIT_CTRLL")
                .field("DOWN_L", &self.DOWN_L())
                .field("STOP_L", &self.STOP_L())
                .field("HALT_L", &self.HALT_L())
                .field("CLRCTR_L", &self.CLRCTR_L())
                .field("BIDIR_L", &self.BIDIR_L())
                .field("PRE_L", &self.PRE_L())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL_ACCESS16BIT_CTRLL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CTRL_ACCESS16BIT_CTRLL {
                DOWN_L: bool,
                STOP_L: bool,
                HALT_L: bool,
                CLRCTR_L: bool,
                BIDIR_L: bool,
                PRE_L: u8,
            }
            let proxy = CTRL_ACCESS16BIT_CTRLL {
                DOWN_L: self.DOWN_L(),
                STOP_L: self.STOP_L(),
                HALT_L: self.HALT_L(),
                CLRCTR_L: self.CLRCTR_L(),
                BIDIR_L: self.BIDIR_L(),
                PRE_L: self.PRE_L(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Dither Condition"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DITHER(pub u32);
    impl DITHER {
        #[inline(always)]
        pub const fn DITHER_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DITHER_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DITHER_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DITHER_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DITHER {
        #[inline(always)]
        fn default() -> DITHER {
            DITHER(0)
        }
    }
    impl core::fmt::Debug for DITHER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DITHER")
                .field("DITHER_L", &self.DITHER_L())
                .field("DITHER_H", &self.DITHER_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DITHER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DITHER {
                DITHER_L: u16,
                DITHER_H: u16,
            }
            let proxy = DITHER {
                DITHER_L: self.DITHER_L(),
                DITHER_H: self.DITHER_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA Request 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMAREQ0(pub u32);
    impl DMAREQ0 {
        #[inline(always)]
        pub const fn DEV_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DEV_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DEV_2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DEV_4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEV_5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DEV_6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DEV_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn DEV_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn DEV_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn DEV_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DEV_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEV_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DEV_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DEV_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DEV_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn DRL0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DRL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn DRQ0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DRQ0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMAREQ0 {
        #[inline(always)]
        fn default() -> DMAREQ0 {
            DMAREQ0(0)
        }
    }
    impl core::fmt::Debug for DMAREQ0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMAREQ0")
                .field("DEV_0", &self.DEV_0())
                .field("DEV_1", &self.DEV_1())
                .field("DEV_2", &self.DEV_2())
                .field("DEV_3", &self.DEV_3())
                .field("DEV_4", &self.DEV_4())
                .field("DEV_5", &self.DEV_5())
                .field("DEV_6", &self.DEV_6())
                .field("DEV_7", &self.DEV_7())
                .field("DEV_8", &self.DEV_8())
                .field("DEV_9", &self.DEV_9())
                .field("DEV_10", &self.DEV_10())
                .field("DEV_11", &self.DEV_11())
                .field("DEV_12", &self.DEV_12())
                .field("DEV_13", &self.DEV_13())
                .field("DEV_14", &self.DEV_14())
                .field("DEV_15", &self.DEV_15())
                .field("DRL0", &self.DRL0())
                .field("DRQ0", &self.DRQ0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMAREQ0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMAREQ0 {
                DEV_0: bool,
                DEV_1: bool,
                DEV_2: bool,
                DEV_3: bool,
                DEV_4: bool,
                DEV_5: bool,
                DEV_6: bool,
                DEV_7: bool,
                DEV_8: bool,
                DEV_9: bool,
                DEV_10: bool,
                DEV_11: bool,
                DEV_12: bool,
                DEV_13: bool,
                DEV_14: bool,
                DEV_15: bool,
                DRL0: bool,
                DRQ0: bool,
            }
            let proxy = DMAREQ0 {
                DEV_0: self.DEV_0(),
                DEV_1: self.DEV_1(),
                DEV_2: self.DEV_2(),
                DEV_3: self.DEV_3(),
                DEV_4: self.DEV_4(),
                DEV_5: self.DEV_5(),
                DEV_6: self.DEV_6(),
                DEV_7: self.DEV_7(),
                DEV_8: self.DEV_8(),
                DEV_9: self.DEV_9(),
                DEV_10: self.DEV_10(),
                DEV_11: self.DEV_11(),
                DEV_12: self.DEV_12(),
                DEV_13: self.DEV_13(),
                DEV_14: self.DEV_14(),
                DEV_15: self.DEV_15(),
                DRL0: self.DRL0(),
                DRQ0: self.DRQ0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "DMA Request 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMAREQ1(pub u32);
    impl DMAREQ1 {
        #[inline(always)]
        pub const fn DEV_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DEV_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DEV_2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DEV_3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn DEV_4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DEV_5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DEV_6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DEV_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn DEV_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn DEV_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn DEV_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DEV_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DEV_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn DEV_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DEV_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn DEV_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEV_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn DRL1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DRL1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn DRQ1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DRQ1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMAREQ1 {
        #[inline(always)]
        fn default() -> DMAREQ1 {
            DMAREQ1(0)
        }
    }
    impl core::fmt::Debug for DMAREQ1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMAREQ1")
                .field("DEV_0", &self.DEV_0())
                .field("DEV_1", &self.DEV_1())
                .field("DEV_2", &self.DEV_2())
                .field("DEV_3", &self.DEV_3())
                .field("DEV_4", &self.DEV_4())
                .field("DEV_5", &self.DEV_5())
                .field("DEV_6", &self.DEV_6())
                .field("DEV_7", &self.DEV_7())
                .field("DEV_8", &self.DEV_8())
                .field("DEV_9", &self.DEV_9())
                .field("DEV_10", &self.DEV_10())
                .field("DEV_11", &self.DEV_11())
                .field("DEV_12", &self.DEV_12())
                .field("DEV_13", &self.DEV_13())
                .field("DEV_14", &self.DEV_14())
                .field("DEV_15", &self.DEV_15())
                .field("DRL1", &self.DRL1())
                .field("DRQ1", &self.DRQ1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMAREQ1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DMAREQ1 {
                DEV_0: bool,
                DEV_1: bool,
                DEV_2: bool,
                DEV_3: bool,
                DEV_4: bool,
                DEV_5: bool,
                DEV_6: bool,
                DEV_7: bool,
                DEV_8: bool,
                DEV_9: bool,
                DEV_10: bool,
                DEV_11: bool,
                DEV_12: bool,
                DEV_13: bool,
                DEV_14: bool,
                DEV_15: bool,
                DRL1: bool,
                DRQ1: bool,
            }
            let proxy = DMAREQ1 {
                DEV_0: self.DEV_0(),
                DEV_1: self.DEV_1(),
                DEV_2: self.DEV_2(),
                DEV_3: self.DEV_3(),
                DEV_4: self.DEV_4(),
                DEV_5: self.DEV_5(),
                DEV_6: self.DEV_6(),
                DEV_7: self.DEV_7(),
                DEV_8: self.DEV_8(),
                DEV_9: self.DEV_9(),
                DEV_10: self.DEV_10(),
                DEV_11: self.DEV_11(),
                DEV_12: self.DEV_12(),
                DEV_13: self.DEV_13(),
                DEV_14: self.DEV_14(),
                DEV_15: self.DEV_15(),
                DRL1: self.DRL1(),
                DRQ1: self.DRQ1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Event Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVEN(pub u32);
    impl EVEN {
        #[inline(always)]
        pub const fn IEN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IEN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IEN2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IEN3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn IEN4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IEN5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IEN6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn IEN7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn IEN8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IEN9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn IEN10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn IEN11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IEN12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IEN13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn IEN14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn IEN15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IEN15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for EVEN {
        #[inline(always)]
        fn default() -> EVEN {
            EVEN(0)
        }
    }
    impl core::fmt::Debug for EVEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVEN")
                .field("IEN0", &self.IEN0())
                .field("IEN1", &self.IEN1())
                .field("IEN2", &self.IEN2())
                .field("IEN3", &self.IEN3())
                .field("IEN4", &self.IEN4())
                .field("IEN5", &self.IEN5())
                .field("IEN6", &self.IEN6())
                .field("IEN7", &self.IEN7())
                .field("IEN8", &self.IEN8())
                .field("IEN9", &self.IEN9())
                .field("IEN10", &self.IEN10())
                .field("IEN11", &self.IEN11())
                .field("IEN12", &self.IEN12())
                .field("IEN13", &self.IEN13())
                .field("IEN14", &self.IEN14())
                .field("IEN15", &self.IEN15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EVEN {
                IEN0: bool,
                IEN1: bool,
                IEN2: bool,
                IEN3: bool,
                IEN4: bool,
                IEN5: bool,
                IEN6: bool,
                IEN7: bool,
                IEN8: bool,
                IEN9: bool,
                IEN10: bool,
                IEN11: bool,
                IEN12: bool,
                IEN13: bool,
                IEN14: bool,
                IEN15: bool,
            }
            let proxy = EVEN {
                IEN0: self.IEN0(),
                IEN1: self.IEN1(),
                IEN2: self.IEN2(),
                IEN3: self.IEN3(),
                IEN4: self.IEN4(),
                IEN5: self.IEN5(),
                IEN6: self.IEN6(),
                IEN7: self.IEN7(),
                IEN8: self.IEN8(),
                IEN9: self.IEN9(),
                IEN10: self.IEN10(),
                IEN11: self.IEN11(),
                IEN12: self.IEN12(),
                IEN13: self.IEN13(),
                IEN14: self.IEN14(),
                IEN15: self.IEN15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Event Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EVFLAG(pub u32);
    impl EVFLAG {
        #[inline(always)]
        pub const fn FLAG0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FLAG1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FLAG2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FLAG3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FLAG4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FLAG5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FLAG6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FLAG7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn FLAG8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FLAG9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn FLAG10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FLAG11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FLAG12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn FLAG13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn FLAG14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn FLAG15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLAG15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for EVFLAG {
        #[inline(always)]
        fn default() -> EVFLAG {
            EVFLAG(0)
        }
    }
    impl core::fmt::Debug for EVFLAG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EVFLAG")
                .field("FLAG0", &self.FLAG0())
                .field("FLAG1", &self.FLAG1())
                .field("FLAG2", &self.FLAG2())
                .field("FLAG3", &self.FLAG3())
                .field("FLAG4", &self.FLAG4())
                .field("FLAG5", &self.FLAG5())
                .field("FLAG6", &self.FLAG6())
                .field("FLAG7", &self.FLAG7())
                .field("FLAG8", &self.FLAG8())
                .field("FLAG9", &self.FLAG9())
                .field("FLAG10", &self.FLAG10())
                .field("FLAG11", &self.FLAG11())
                .field("FLAG12", &self.FLAG12())
                .field("FLAG13", &self.FLAG13())
                .field("FLAG14", &self.FLAG14())
                .field("FLAG15", &self.FLAG15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EVFLAG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EVFLAG {
                FLAG0: bool,
                FLAG1: bool,
                FLAG2: bool,
                FLAG3: bool,
                FLAG4: bool,
                FLAG5: bool,
                FLAG6: bool,
                FLAG7: bool,
                FLAG8: bool,
                FLAG9: bool,
                FLAG10: bool,
                FLAG11: bool,
                FLAG12: bool,
                FLAG13: bool,
                FLAG14: bool,
                FLAG15: bool,
            }
            let proxy = EVFLAG {
                FLAG0: self.FLAG0(),
                FLAG1: self.FLAG1(),
                FLAG2: self.FLAG2(),
                FLAG3: self.FLAG3(),
                FLAG4: self.FLAG4(),
                FLAG5: self.FLAG5(),
                FLAG6: self.FLAG6(),
                FLAG7: self.FLAG7(),
                FLAG8: self.FLAG8(),
                FLAG9: self.FLAG9(),
                FLAG10: self.FLAG10(),
                FLAG11: self.FLAG11(),
                FLAG12: self.FLAG12(),
                FLAG13: self.FLAG13(),
                FLAG14: self.FLAG14(),
                FLAG15: self.FLAG15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Event n Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EV_CTRL(pub u32);
    impl EV_CTRL {
        #[inline(always)]
        pub const fn MATCHSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MATCHSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn HEVENT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HEVENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OUTSEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUTSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IOSEL(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IOSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[inline(always)]
        pub const fn IOCOND(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IOCOND(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn COMBMODE(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_COMBMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn STATELD(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STATELD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn STATEV(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_STATEV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[inline(always)]
        pub const fn MATCHMEM(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MATCHMEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn DIRECTION(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIRECTION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
    }
    impl Default for EV_CTRL {
        #[inline(always)]
        fn default() -> EV_CTRL {
            EV_CTRL(0)
        }
    }
    impl core::fmt::Debug for EV_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EV_CTRL")
                .field("MATCHSEL", &self.MATCHSEL())
                .field("HEVENT", &self.HEVENT())
                .field("OUTSEL", &self.OUTSEL())
                .field("IOSEL", &self.IOSEL())
                .field("IOCOND", &self.IOCOND())
                .field("COMBMODE", &self.COMBMODE())
                .field("STATELD", &self.STATELD())
                .field("STATEV", &self.STATEV())
                .field("MATCHMEM", &self.MATCHMEM())
                .field("DIRECTION", &self.DIRECTION())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EV_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct EV_CTRL {
                MATCHSEL: u8,
                HEVENT: bool,
                OUTSEL: bool,
                IOSEL: u8,
                IOCOND: u8,
                COMBMODE: u8,
                STATELD: bool,
                STATEV: u8,
                MATCHMEM: bool,
                DIRECTION: u8,
            }
            let proxy = EV_CTRL {
                MATCHSEL: self.MATCHSEL(),
                HEVENT: self.HEVENT(),
                OUTSEL: self.OUTSEL(),
                IOSEL: self.IOSEL(),
                IOCOND: self.IOCOND(),
                COMBMODE: self.COMBMODE(),
                STATELD: self.STATELD(),
                STATEV: self.STATEV(),
                MATCHMEM: self.MATCHMEM(),
                DIRECTION: self.DIRECTION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fractional Match"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRACMAT(pub u32);
    impl FRACMAT {
        #[inline(always)]
        pub const fn FRACMAT_L(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRACMAT_L(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FRACMAT_H(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FRACMAT_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FRACMAT {
        #[inline(always)]
        fn default() -> FRACMAT {
            FRACMAT(0)
        }
    }
    impl core::fmt::Debug for FRACMAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRACMAT")
                .field("FRACMAT_L", &self.FRACMAT_L())
                .field("FRACMAT_H", &self.FRACMAT_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRACMAT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRACMAT {
                FRACMAT_L: u8,
                FRACMAT_H: u8,
            }
            let proxy = FRACMAT {
                FRACMAT_L: self.FRACMAT_L(),
                FRACMAT_H: self.FRACMAT_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Fractional Match Reload"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRACMATREL(pub u32);
    impl FRACMATREL {
        #[inline(always)]
        pub const fn RELFRAC_L(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RELFRAC_L(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RELFRAC_H(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RELFRAC_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FRACMATREL {
        #[inline(always)]
        fn default() -> FRACMATREL {
            FRACMATREL(0)
        }
    }
    impl core::fmt::Debug for FRACMATREL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRACMATREL")
                .field("RELFRAC_L", &self.RELFRAC_L())
                .field("RELFRAC_H", &self.RELFRAC_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRACMATREL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRACMATREL {
                RELFRAC_L: u8,
                RELFRAC_H: u8,
            }
            let proxy = FRACMATREL {
                RELFRAC_L: self.RELFRAC_L(),
                RELFRAC_H: self.RELFRAC_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Halt Event Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HALT(pub u32);
    impl HALT {
        #[inline(always)]
        pub const fn HALTMSK_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_HALTMSK_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn HALTMSK_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_HALTMSK_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for HALT {
        #[inline(always)]
        fn default() -> HALT {
            HALT(0)
        }
    }
    impl core::fmt::Debug for HALT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HALT")
                .field("HALTMSK_L", &self.HALTMSK_L())
                .field("HALTMSK_H", &self.HALTMSK_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HALT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HALT {
                HALTMSK_L: u16,
                HALTMSK_H: u16,
            }
            let proxy = HALT {
                HALTMSK_L: self.HALTMSK_L(),
                HALTMSK_H: self.HALTMSK_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_HALTH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HALT_ACCESS16BIT_HALTH(pub u32);
    impl HALT_ACCESS16BIT_HALTH {
        #[inline(always)]
        pub const fn HALTH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_HALTH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HALT_ACCESS16BIT_HALTH {
        #[inline(always)]
        fn default() -> HALT_ACCESS16BIT_HALTH {
            HALT_ACCESS16BIT_HALTH(0)
        }
    }
    impl core::fmt::Debug for HALT_ACCESS16BIT_HALTH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HALT_ACCESS16BIT_HALTH")
                .field("HALTH", &self.HALTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HALT_ACCESS16BIT_HALTH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HALT_ACCESS16BIT_HALTH {
                HALTH: u16,
            }
            let proxy = HALT_ACCESS16BIT_HALTH {
                HALTH: self.HALTH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_HALTL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HALT_ACCESS16BIT_HALTL(pub u32);
    impl HALT_ACCESS16BIT_HALTL {
        #[inline(always)]
        pub const fn HALTL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_HALTL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HALT_ACCESS16BIT_HALTL {
        #[inline(always)]
        fn default() -> HALT_ACCESS16BIT_HALTL {
            HALT_ACCESS16BIT_HALTL(0)
        }
    }
    impl core::fmt::Debug for HALT_ACCESS16BIT_HALTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HALT_ACCESS16BIT_HALTL")
                .field("HALTL", &self.HALTL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HALT_ACCESS16BIT_HALTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HALT_ACCESS16BIT_HALTL {
                HALTL: u16,
            }
            let proxy = HALT_ACCESS16BIT_HALTL {
                HALTL: self.HALTL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Input State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INPUT(pub u32);
    impl INPUT {
        #[inline(always)]
        pub const fn AIN0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn AIN1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AIN2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn AIN3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn AIN4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn AIN5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn AIN6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn AIN7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn AIN8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn AIN9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn AIN10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn AIN11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AIN12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn AIN13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn AIN14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn AIN15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AIN15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn SIN0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SIN1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn SIN2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn SIN3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn SIN4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn SIN5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn SIN6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn SIN7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn SIN8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SIN9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SIN10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SIN11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn SIN12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn SIN13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn SIN14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SIN15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIN15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for INPUT {
        #[inline(always)]
        fn default() -> INPUT {
            INPUT(0)
        }
    }
    impl core::fmt::Debug for INPUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INPUT")
                .field("AIN0", &self.AIN0())
                .field("AIN1", &self.AIN1())
                .field("AIN2", &self.AIN2())
                .field("AIN3", &self.AIN3())
                .field("AIN4", &self.AIN4())
                .field("AIN5", &self.AIN5())
                .field("AIN6", &self.AIN6())
                .field("AIN7", &self.AIN7())
                .field("AIN8", &self.AIN8())
                .field("AIN9", &self.AIN9())
                .field("AIN10", &self.AIN10())
                .field("AIN11", &self.AIN11())
                .field("AIN12", &self.AIN12())
                .field("AIN13", &self.AIN13())
                .field("AIN14", &self.AIN14())
                .field("AIN15", &self.AIN15())
                .field("SIN0", &self.SIN0())
                .field("SIN1", &self.SIN1())
                .field("SIN2", &self.SIN2())
                .field("SIN3", &self.SIN3())
                .field("SIN4", &self.SIN4())
                .field("SIN5", &self.SIN5())
                .field("SIN6", &self.SIN6())
                .field("SIN7", &self.SIN7())
                .field("SIN8", &self.SIN8())
                .field("SIN9", &self.SIN9())
                .field("SIN10", &self.SIN10())
                .field("SIN11", &self.SIN11())
                .field("SIN12", &self.SIN12())
                .field("SIN13", &self.SIN13())
                .field("SIN14", &self.SIN14())
                .field("SIN15", &self.SIN15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INPUT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INPUT {
                AIN0: bool,
                AIN1: bool,
                AIN2: bool,
                AIN3: bool,
                AIN4: bool,
                AIN5: bool,
                AIN6: bool,
                AIN7: bool,
                AIN8: bool,
                AIN9: bool,
                AIN10: bool,
                AIN11: bool,
                AIN12: bool,
                AIN13: bool,
                AIN14: bool,
                AIN15: bool,
                SIN0: bool,
                SIN1: bool,
                SIN2: bool,
                SIN3: bool,
                SIN4: bool,
                SIN5: bool,
                SIN6: bool,
                SIN7: bool,
                SIN8: bool,
                SIN9: bool,
                SIN10: bool,
                SIN11: bool,
                SIN12: bool,
                SIN13: bool,
                SIN14: bool,
                SIN15: bool,
            }
            let proxy = INPUT {
                AIN0: self.AIN0(),
                AIN1: self.AIN1(),
                AIN2: self.AIN2(),
                AIN3: self.AIN3(),
                AIN4: self.AIN4(),
                AIN5: self.AIN5(),
                AIN6: self.AIN6(),
                AIN7: self.AIN7(),
                AIN8: self.AIN8(),
                AIN9: self.AIN9(),
                AIN10: self.AIN10(),
                AIN11: self.AIN11(),
                AIN12: self.AIN12(),
                AIN13: self.AIN13(),
                AIN14: self.AIN14(),
                AIN15: self.AIN15(),
                SIN0: self.SIN0(),
                SIN1: self.SIN1(),
                SIN2: self.SIN2(),
                SIN3: self.SIN3(),
                SIN4: self.SIN4(),
                SIN5: self.SIN5(),
                SIN6: self.SIN6(),
                SIN7: self.SIN7(),
                SIN8: self.SIN8(),
                SIN9: self.SIN9(),
                SIN10: self.SIN10(),
                SIN11: self.SIN11(),
                SIN12: self.SIN12(),
                SIN13: self.SIN13(),
                SIN14: self.SIN14(),
                SIN15: self.SIN15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT Limit Event Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LIMIT(pub u32);
    impl LIMIT {
        #[inline(always)]
        pub const fn LIMMSK_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LIMMSK_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn LIMMSK_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LIMMSK_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for LIMIT {
        #[inline(always)]
        fn default() -> LIMIT {
            LIMIT(0)
        }
    }
    impl core::fmt::Debug for LIMIT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LIMIT")
                .field("LIMMSK_L", &self.LIMMSK_L())
                .field("LIMMSK_H", &self.LIMMSK_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LIMIT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LIMIT {
                LIMMSK_L: u16,
                LIMMSK_H: u16,
            }
            let proxy = LIMIT {
                LIMMSK_L: self.LIMMSK_L(),
                LIMMSK_H: self.LIMMSK_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_LIMITH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LIMIT_ACCESS16BIT_LIMITH(pub u32);
    impl LIMIT_ACCESS16BIT_LIMITH {
        #[inline(always)]
        pub const fn LIMITH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LIMITH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LIMIT_ACCESS16BIT_LIMITH {
        #[inline(always)]
        fn default() -> LIMIT_ACCESS16BIT_LIMITH {
            LIMIT_ACCESS16BIT_LIMITH(0)
        }
    }
    impl core::fmt::Debug for LIMIT_ACCESS16BIT_LIMITH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LIMIT_ACCESS16BIT_LIMITH")
                .field("LIMITH", &self.LIMITH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LIMIT_ACCESS16BIT_LIMITH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LIMIT_ACCESS16BIT_LIMITH {
                LIMITH: u16,
            }
            let proxy = LIMIT_ACCESS16BIT_LIMITH {
                LIMITH: self.LIMITH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_LIMITL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LIMIT_ACCESS16BIT_LIMITL(pub u32);
    impl LIMIT_ACCESS16BIT_LIMITL {
        #[inline(always)]
        pub const fn LIMITL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_LIMITL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LIMIT_ACCESS16BIT_LIMITL {
        #[inline(always)]
        fn default() -> LIMIT_ACCESS16BIT_LIMITL {
            LIMIT_ACCESS16BIT_LIMITL(0)
        }
    }
    impl core::fmt::Debug for LIMIT_ACCESS16BIT_LIMITL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LIMIT_ACCESS16BIT_LIMITL")
                .field("LIMITL", &self.LIMITL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LIMIT_ACCESS16BIT_LIMITL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct LIMIT_ACCESS16BIT_LIMITL {
                LIMITL: u16,
            }
            let proxy = LIMIT_ACCESS16BIT_LIMITL {
                LIMITL: self.LIMITL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Match Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH(pub u32);
    impl MATCH {
        #[inline(always)]
        pub const fn MATCHn_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHn_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MATCHn_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHn_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MATCH {
        #[inline(always)]
        fn default() -> MATCH {
            MATCH(0)
        }
    }
    impl core::fmt::Debug for MATCH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCH")
                .field("MATCHn_L", &self.MATCHn_L())
                .field("MATCHn_H", &self.MATCHn_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCH {
                MATCHn_L: u16,
                MATCHn_H: u16,
            }
            let proxy = MATCH {
                MATCHn_L: self.MATCHn_L(),
                MATCHn_H: self.MATCHn_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Match Reload Value"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCHREL(pub u32);
    impl MATCHREL {
        #[inline(always)]
        pub const fn RELOADn_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RELOADn_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn RELOADn_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RELOADn_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MATCHREL {
        #[inline(always)]
        fn default() -> MATCHREL {
            MATCHREL(0)
        }
    }
    impl core::fmt::Debug for MATCHREL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCHREL")
                .field("RELOADn_L", &self.RELOADn_L())
                .field("RELOADn_H", &self.RELOADn_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCHREL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCHREL {
                RELOADn_L: u16,
                RELOADn_H: u16,
            }
            let proxy = MATCHREL {
                RELOADn_L: self.RELOADn_L(),
                RELOADn_H: self.RELOADn_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_MATCHRELH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCHREL_ACCESS16BIT_MATCHRELH(pub u32);
    impl MATCHREL_ACCESS16BIT_MATCHRELH {
        #[inline(always)]
        pub const fn MATCHRELH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHRELH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MATCHREL_ACCESS16BIT_MATCHRELH {
        #[inline(always)]
        fn default() -> MATCHREL_ACCESS16BIT_MATCHRELH {
            MATCHREL_ACCESS16BIT_MATCHRELH(0)
        }
    }
    impl core::fmt::Debug for MATCHREL_ACCESS16BIT_MATCHRELH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCHREL_ACCESS16BIT_MATCHRELH")
                .field("MATCHRELH", &self.MATCHRELH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCHREL_ACCESS16BIT_MATCHRELH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCHREL_ACCESS16BIT_MATCHRELH {
                MATCHRELH: u16,
            }
            let proxy = MATCHREL_ACCESS16BIT_MATCHRELH {
                MATCHRELH: self.MATCHRELH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_MATCHRELL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCHREL_ACCESS16BIT_MATCHRELL(pub u32);
    impl MATCHREL_ACCESS16BIT_MATCHRELL {
        #[inline(always)]
        pub const fn MATCHRELL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHRELL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MATCHREL_ACCESS16BIT_MATCHRELL {
        #[inline(always)]
        fn default() -> MATCHREL_ACCESS16BIT_MATCHRELL {
            MATCHREL_ACCESS16BIT_MATCHRELL(0)
        }
    }
    impl core::fmt::Debug for MATCHREL_ACCESS16BIT_MATCHRELL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCHREL_ACCESS16BIT_MATCHRELL")
                .field("MATCHRELL", &self.MATCHRELL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCHREL_ACCESS16BIT_MATCHRELL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCHREL_ACCESS16BIT_MATCHRELL {
                MATCHRELL: u16,
            }
            let proxy = MATCHREL_ACCESS16BIT_MATCHRELL {
                MATCHRELL: self.MATCHRELL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_MATCHH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH_ACCESS16BIT_MATCHH(pub u32);
    impl MATCH_ACCESS16BIT_MATCHH {
        #[inline(always)]
        pub const fn MATCHH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MATCH_ACCESS16BIT_MATCHH {
        #[inline(always)]
        fn default() -> MATCH_ACCESS16BIT_MATCHH {
            MATCH_ACCESS16BIT_MATCHH(0)
        }
    }
    impl core::fmt::Debug for MATCH_ACCESS16BIT_MATCHH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCH_ACCESS16BIT_MATCHH")
                .field("MATCHH", &self.MATCHH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCH_ACCESS16BIT_MATCHH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCH_ACCESS16BIT_MATCHH {
                MATCHH: u16,
            }
            let proxy = MATCH_ACCESS16BIT_MATCHH {
                MATCHH: self.MATCHH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_MATCHL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MATCH_ACCESS16BIT_MATCHL(pub u32);
    impl MATCH_ACCESS16BIT_MATCHL {
        #[inline(always)]
        pub const fn MATCHL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MATCHL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MATCH_ACCESS16BIT_MATCHL {
        #[inline(always)]
        fn default() -> MATCH_ACCESS16BIT_MATCHL {
            MATCH_ACCESS16BIT_MATCHL(0)
        }
    }
    impl core::fmt::Debug for MATCH_ACCESS16BIT_MATCHL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MATCH_ACCESS16BIT_MATCHL")
                .field("MATCHL", &self.MATCHL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MATCH_ACCESS16BIT_MATCHL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MATCH_ACCESS16BIT_MATCHL {
                MATCHL: u16,
            }
            let proxy = MATCH_ACCESS16BIT_MATCHL {
                MATCHL: self.MATCHL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUT(pub u32);
    impl OUTPUT {
        #[inline(always)]
        pub const fn OUT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OUT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn OUT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn OUT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn OUT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OUT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn OUT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn OUT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn OUT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OUT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for OUTPUT {
        #[inline(always)]
        fn default() -> OUTPUT {
            OUTPUT(0)
        }
    }
    impl core::fmt::Debug for OUTPUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTPUT")
                .field("OUT0", &self.OUT0())
                .field("OUT1", &self.OUT1())
                .field("OUT2", &self.OUT2())
                .field("OUT3", &self.OUT3())
                .field("OUT4", &self.OUT4())
                .field("OUT5", &self.OUT5())
                .field("OUT6", &self.OUT6())
                .field("OUT7", &self.OUT7())
                .field("OUT8", &self.OUT8())
                .field("OUT9", &self.OUT9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTPUT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OUTPUT {
                OUT0: bool,
                OUT1: bool,
                OUT2: bool,
                OUT3: bool,
                OUT4: bool,
                OUT5: bool,
                OUT6: bool,
                OUT7: bool,
                OUT8: bool,
                OUT9: bool,
            }
            let proxy = OUTPUT {
                OUT0: self.OUT0(),
                OUT1: self.OUT1(),
                OUT2: self.OUT2(),
                OUT3: self.OUT3(),
                OUT4: self.OUT4(),
                OUT5: self.OUT5(),
                OUT6: self.OUT6(),
                OUT7: self.OUT7(),
                OUT8: self.OUT8(),
                OUT9: self.OUT9(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Counter Direction Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUTPUTDIRCTRL(pub u32);
    impl OUTPUTDIRCTRL {
        #[inline(always)]
        pub const fn SETCLR0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn SETCLR1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn SETCLR2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn SETCLR3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn SETCLR4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn SETCLR5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn SETCLR6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn SETCLR7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn SETCLR8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn SETCLR9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETCLR9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for OUTPUTDIRCTRL {
        #[inline(always)]
        fn default() -> OUTPUTDIRCTRL {
            OUTPUTDIRCTRL(0)
        }
    }
    impl core::fmt::Debug for OUTPUTDIRCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUTPUTDIRCTRL")
                .field("SETCLR0", &self.SETCLR0())
                .field("SETCLR1", &self.SETCLR1())
                .field("SETCLR2", &self.SETCLR2())
                .field("SETCLR3", &self.SETCLR3())
                .field("SETCLR4", &self.SETCLR4())
                .field("SETCLR5", &self.SETCLR5())
                .field("SETCLR6", &self.SETCLR6())
                .field("SETCLR7", &self.SETCLR7())
                .field("SETCLR8", &self.SETCLR8())
                .field("SETCLR9", &self.SETCLR9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUTPUTDIRCTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OUTPUTDIRCTRL {
                SETCLR0: u8,
                SETCLR1: u8,
                SETCLR2: u8,
                SETCLR3: u8,
                SETCLR4: u8,
                SETCLR5: u8,
                SETCLR6: u8,
                SETCLR7: u8,
                SETCLR8: u8,
                SETCLR9: u8,
            }
            let proxy = OUTPUTDIRCTRL {
                SETCLR0: self.SETCLR0(),
                SETCLR1: self.SETCLR1(),
                SETCLR2: self.SETCLR2(),
                SETCLR3: self.SETCLR3(),
                SETCLR4: self.SETCLR4(),
                SETCLR5: self.SETCLR5(),
                SETCLR6: self.SETCLR6(),
                SETCLR7: self.SETCLR7(),
                SETCLR8: self.SETCLR8(),
                SETCLR9: self.SETCLR9(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output n Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUT_CLR(pub u32);
    impl OUT_CLR {
        #[inline(always)]
        pub const fn CLR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CLR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OUT_CLR {
        #[inline(always)]
        fn default() -> OUT_CLR {
            OUT_CLR(0)
        }
    }
    impl core::fmt::Debug for OUT_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUT_CLR").field("CLR", &self.CLR()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUT_CLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OUT_CLR {
                CLR: u16,
            }
            let proxy = OUT_CLR { CLR: self.CLR() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output n Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUT_SET(pub u32);
    impl OUT_SET {
        #[inline(always)]
        pub const fn SET(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SET(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OUT_SET {
        #[inline(always)]
        fn default() -> OUT_SET {
            OUT_SET(0)
        }
    }
    impl core::fmt::Debug for OUT_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUT_SET").field("SET", &self.SET()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUT_SET {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OUT_SET {
                SET: u16,
            }
            let proxy = OUT_SET { SET: self.SET() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Match and Capture Register Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REGMODE(pub u32);
    impl REGMODE {
        #[inline(always)]
        pub const fn REGMOD_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_L15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGMOD_H15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for REGMODE {
        #[inline(always)]
        fn default() -> REGMODE {
            REGMODE(0)
        }
    }
    impl core::fmt::Debug for REGMODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REGMODE")
                .field("REGMOD_L", &self.REGMOD_L())
                .field("REGMOD_L0", &self.REGMOD_L0())
                .field("REGMOD_L1", &self.REGMOD_L1())
                .field("REGMOD_L2", &self.REGMOD_L2())
                .field("REGMOD_L3", &self.REGMOD_L3())
                .field("REGMOD_L4", &self.REGMOD_L4())
                .field("REGMOD_L5", &self.REGMOD_L5())
                .field("REGMOD_L6", &self.REGMOD_L6())
                .field("REGMOD_L7", &self.REGMOD_L7())
                .field("REGMOD_L8", &self.REGMOD_L8())
                .field("REGMOD_L9", &self.REGMOD_L9())
                .field("REGMOD_L10", &self.REGMOD_L10())
                .field("REGMOD_L11", &self.REGMOD_L11())
                .field("REGMOD_L12", &self.REGMOD_L12())
                .field("REGMOD_L13", &self.REGMOD_L13())
                .field("REGMOD_L14", &self.REGMOD_L14())
                .field("REGMOD_L15", &self.REGMOD_L15())
                .field("REGMOD_H", &self.REGMOD_H())
                .field("REGMOD_H0", &self.REGMOD_H0())
                .field("REGMOD_H1", &self.REGMOD_H1())
                .field("REGMOD_H2", &self.REGMOD_H2())
                .field("REGMOD_H3", &self.REGMOD_H3())
                .field("REGMOD_H4", &self.REGMOD_H4())
                .field("REGMOD_H5", &self.REGMOD_H5())
                .field("REGMOD_H6", &self.REGMOD_H6())
                .field("REGMOD_H7", &self.REGMOD_H7())
                .field("REGMOD_H8", &self.REGMOD_H8())
                .field("REGMOD_H9", &self.REGMOD_H9())
                .field("REGMOD_H10", &self.REGMOD_H10())
                .field("REGMOD_H11", &self.REGMOD_H11())
                .field("REGMOD_H12", &self.REGMOD_H12())
                .field("REGMOD_H13", &self.REGMOD_H13())
                .field("REGMOD_H14", &self.REGMOD_H14())
                .field("REGMOD_H15", &self.REGMOD_H15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REGMODE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REGMODE {
                REGMOD_L: u16,
                REGMOD_L0: bool,
                REGMOD_L1: bool,
                REGMOD_L2: bool,
                REGMOD_L3: bool,
                REGMOD_L4: bool,
                REGMOD_L5: bool,
                REGMOD_L6: bool,
                REGMOD_L7: bool,
                REGMOD_L8: bool,
                REGMOD_L9: bool,
                REGMOD_L10: bool,
                REGMOD_L11: bool,
                REGMOD_L12: bool,
                REGMOD_L13: bool,
                REGMOD_L14: bool,
                REGMOD_L15: bool,
                REGMOD_H: u16,
                REGMOD_H0: bool,
                REGMOD_H1: bool,
                REGMOD_H2: bool,
                REGMOD_H3: bool,
                REGMOD_H4: bool,
                REGMOD_H5: bool,
                REGMOD_H6: bool,
                REGMOD_H7: bool,
                REGMOD_H8: bool,
                REGMOD_H9: bool,
                REGMOD_H10: bool,
                REGMOD_H11: bool,
                REGMOD_H12: bool,
                REGMOD_H13: bool,
                REGMOD_H14: bool,
                REGMOD_H15: bool,
            }
            let proxy = REGMODE {
                REGMOD_L: self.REGMOD_L(),
                REGMOD_L0: self.REGMOD_L0(),
                REGMOD_L1: self.REGMOD_L1(),
                REGMOD_L2: self.REGMOD_L2(),
                REGMOD_L3: self.REGMOD_L3(),
                REGMOD_L4: self.REGMOD_L4(),
                REGMOD_L5: self.REGMOD_L5(),
                REGMOD_L6: self.REGMOD_L6(),
                REGMOD_L7: self.REGMOD_L7(),
                REGMOD_L8: self.REGMOD_L8(),
                REGMOD_L9: self.REGMOD_L9(),
                REGMOD_L10: self.REGMOD_L10(),
                REGMOD_L11: self.REGMOD_L11(),
                REGMOD_L12: self.REGMOD_L12(),
                REGMOD_L13: self.REGMOD_L13(),
                REGMOD_L14: self.REGMOD_L14(),
                REGMOD_L15: self.REGMOD_L15(),
                REGMOD_H: self.REGMOD_H(),
                REGMOD_H0: self.REGMOD_H0(),
                REGMOD_H1: self.REGMOD_H1(),
                REGMOD_H2: self.REGMOD_H2(),
                REGMOD_H3: self.REGMOD_H3(),
                REGMOD_H4: self.REGMOD_H4(),
                REGMOD_H5: self.REGMOD_H5(),
                REGMOD_H6: self.REGMOD_H6(),
                REGMOD_H7: self.REGMOD_H7(),
                REGMOD_H8: self.REGMOD_H8(),
                REGMOD_H9: self.REGMOD_H9(),
                REGMOD_H10: self.REGMOD_H10(),
                REGMOD_H11: self.REGMOD_H11(),
                REGMOD_H12: self.REGMOD_H12(),
                REGMOD_H13: self.REGMOD_H13(),
                REGMOD_H14: self.REGMOD_H14(),
                REGMOD_H15: self.REGMOD_H15(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_REGMODEH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REGMODE_ACCESS16BIT_REGMODEH(pub u32);
    impl REGMODE_ACCESS16BIT_REGMODEH {
        #[inline(always)]
        pub const fn REGMODEH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMODEH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for REGMODE_ACCESS16BIT_REGMODEH {
        #[inline(always)]
        fn default() -> REGMODE_ACCESS16BIT_REGMODEH {
            REGMODE_ACCESS16BIT_REGMODEH(0)
        }
    }
    impl core::fmt::Debug for REGMODE_ACCESS16BIT_REGMODEH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REGMODE_ACCESS16BIT_REGMODEH")
                .field("REGMODEH", &self.REGMODEH())
                .field("REGMOD_L", &self.REGMOD_L())
                .field("REGMOD_H", &self.REGMOD_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REGMODE_ACCESS16BIT_REGMODEH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REGMODE_ACCESS16BIT_REGMODEH {
                REGMODEH: u16,
                REGMOD_L: u16,
                REGMOD_H: u16,
            }
            let proxy = REGMODE_ACCESS16BIT_REGMODEH {
                REGMODEH: self.REGMODEH(),
                REGMOD_L: self.REGMOD_L(),
                REGMOD_H: self.REGMOD_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_REGMODEL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REGMODE_ACCESS16BIT_REGMODEL(pub u32);
    impl REGMODE_ACCESS16BIT_REGMODEL {
        #[inline(always)]
        pub const fn REGMODEL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMODEL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn REGMOD_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_REGMOD_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for REGMODE_ACCESS16BIT_REGMODEL {
        #[inline(always)]
        fn default() -> REGMODE_ACCESS16BIT_REGMODEL {
            REGMODE_ACCESS16BIT_REGMODEL(0)
        }
    }
    impl core::fmt::Debug for REGMODE_ACCESS16BIT_REGMODEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REGMODE_ACCESS16BIT_REGMODEL")
                .field("REGMODEL", &self.REGMODEL())
                .field("REGMOD_L", &self.REGMOD_L())
                .field("REGMOD_H", &self.REGMOD_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REGMODE_ACCESS16BIT_REGMODEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REGMODE_ACCESS16BIT_REGMODEL {
                REGMODEL: u16,
                REGMOD_L: u16,
                REGMOD_H: u16,
            }
            let proxy = REGMODE_ACCESS16BIT_REGMODEL {
                REGMODEL: self.REGMODEL(),
                REGMOD_L: self.REGMOD_L(),
                REGMOD_H: self.REGMOD_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Output Conflict Resolution"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RES(pub u32);
    impl RES {
        #[inline(always)]
        pub const fn O0RES(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O0RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn O1RES(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O1RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn O2RES(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O2RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn O3RES(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O3RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn O4RES(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O4RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn O5RES(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O5RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn O6RES(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O6RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn O7RES(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O7RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn O8RES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O8RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn O9RES(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_O9RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for RES {
        #[inline(always)]
        fn default() -> RES {
            RES(0)
        }
    }
    impl core::fmt::Debug for RES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RES")
                .field("O0RES", &self.O0RES())
                .field("O1RES", &self.O1RES())
                .field("O2RES", &self.O2RES())
                .field("O3RES", &self.O3RES())
                .field("O4RES", &self.O4RES())
                .field("O5RES", &self.O5RES())
                .field("O6RES", &self.O6RES())
                .field("O7RES", &self.O7RES())
                .field("O8RES", &self.O8RES())
                .field("O9RES", &self.O9RES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RES {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RES {
                O0RES: u8,
                O1RES: u8,
                O2RES: u8,
                O3RES: u8,
                O4RES: u8,
                O5RES: u8,
                O6RES: u8,
                O7RES: u8,
                O8RES: u8,
                O9RES: u8,
            }
            let proxy = RES {
                O0RES: self.O0RES(),
                O1RES: self.O1RES(),
                O2RES: self.O2RES(),
                O3RES: self.O3RES(),
                O4RES: self.O4RES(),
                O5RES: self.O5RES(),
                O6RES: self.O6RES(),
                O7RES: self.O7RES(),
                O8RES: self.O8RES(),
                O9RES: self.O9RES(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Start Event Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct START(pub u32);
    impl START {
        #[inline(always)]
        pub const fn STARTMSK_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STARTMSK_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STARTMSK_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STARTMSK_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for START {
        #[inline(always)]
        fn default() -> START {
            START(0)
        }
    }
    impl core::fmt::Debug for START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("START")
                .field("STARTMSK_L", &self.STARTMSK_L())
                .field("STARTMSK_H", &self.STARTMSK_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for START {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct START {
                STARTMSK_L: u16,
                STARTMSK_H: u16,
            }
            let proxy = START {
                STARTMSK_L: self.STARTMSK_L(),
                STARTMSK_H: self.STARTMSK_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STARTH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct START_ACCESS16BIT_STARTH(pub u32);
    impl START_ACCESS16BIT_STARTH {
        #[inline(always)]
        pub const fn STARTH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STARTH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for START_ACCESS16BIT_STARTH {
        #[inline(always)]
        fn default() -> START_ACCESS16BIT_STARTH {
            START_ACCESS16BIT_STARTH(0)
        }
    }
    impl core::fmt::Debug for START_ACCESS16BIT_STARTH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("START_ACCESS16BIT_STARTH")
                .field("STARTH", &self.STARTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for START_ACCESS16BIT_STARTH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct START_ACCESS16BIT_STARTH {
                STARTH: u16,
            }
            let proxy = START_ACCESS16BIT_STARTH {
                STARTH: self.STARTH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STARTL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct START_ACCESS16BIT_STARTL(pub u32);
    impl START_ACCESS16BIT_STARTL {
        #[inline(always)]
        pub const fn STARTL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STARTL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for START_ACCESS16BIT_STARTL {
        #[inline(always)]
        fn default() -> START_ACCESS16BIT_STARTL {
            START_ACCESS16BIT_STARTL(0)
        }
    }
    impl core::fmt::Debug for START_ACCESS16BIT_STARTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("START_ACCESS16BIT_STARTL")
                .field("STARTL", &self.STARTL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for START_ACCESS16BIT_STARTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct START_ACCESS16BIT_STARTL {
                STARTL: u16,
            }
            let proxy = START_ACCESS16BIT_STARTL {
                STARTL: self.STARTL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "State Variable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATE(pub u32);
    impl STATE {
        #[inline(always)]
        pub const fn STATE_L(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_STATE_L(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn STATE_H(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_STATE_H(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for STATE {
        #[inline(always)]
        fn default() -> STATE {
            STATE(0)
        }
    }
    impl core::fmt::Debug for STATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATE")
                .field("STATE_L", &self.STATE_L())
                .field("STATE_H", &self.STATE_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STATE {
                STATE_L: u8,
                STATE_H: u8,
            }
            let proxy = STATE {
                STATE_L: self.STATE_L(),
                STATE_H: self.STATE_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STATEH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATE_ACCESS16BIT_STATEH(pub u32);
    impl STATE_ACCESS16BIT_STATEH {
        #[inline(always)]
        pub const fn STATEH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STATEH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for STATE_ACCESS16BIT_STATEH {
        #[inline(always)]
        fn default() -> STATE_ACCESS16BIT_STATEH {
            STATE_ACCESS16BIT_STATEH(0)
        }
    }
    impl core::fmt::Debug for STATE_ACCESS16BIT_STATEH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATE_ACCESS16BIT_STATEH")
                .field("STATEH", &self.STATEH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATE_ACCESS16BIT_STATEH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STATE_ACCESS16BIT_STATEH {
                STATEH: u16,
            }
            let proxy = STATE_ACCESS16BIT_STATEH {
                STATEH: self.STATEH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STATEL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATE_ACCESS16BIT_STATEL(pub u32);
    impl STATE_ACCESS16BIT_STATEL {
        #[inline(always)]
        pub const fn STATEL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STATEL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for STATE_ACCESS16BIT_STATEL {
        #[inline(always)]
        fn default() -> STATE_ACCESS16BIT_STATEL {
            STATE_ACCESS16BIT_STATEL(0)
        }
    }
    impl core::fmt::Debug for STATE_ACCESS16BIT_STATEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATE_ACCESS16BIT_STATEL")
                .field("STATEL", &self.STATEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATE_ACCESS16BIT_STATEL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STATE_ACCESS16BIT_STATEL {
                STATEL: u16,
            }
            let proxy = STATE_ACCESS16BIT_STATEL {
                STATEL: self.STATEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Stop Event Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STOP(pub u32);
    impl STOP {
        #[inline(always)]
        pub const fn STOPMSK_L(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STOPMSK_L(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn STOPMSK_H(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STOPMSK_H(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for STOP {
        #[inline(always)]
        fn default() -> STOP {
            STOP(0)
        }
    }
    impl core::fmt::Debug for STOP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STOP")
                .field("STOPMSK_L", &self.STOPMSK_L())
                .field("STOPMSK_H", &self.STOPMSK_H())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STOP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STOP {
                STOPMSK_L: u16,
                STOPMSK_H: u16,
            }
            let proxy = STOP {
                STOPMSK_L: self.STOPMSK_L(),
                STOPMSK_H: self.STOPMSK_H(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STOPH register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STOP_ACCESS16BIT_STOPH(pub u32);
    impl STOP_ACCESS16BIT_STOPH {
        #[inline(always)]
        pub const fn STOPH(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STOPH(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for STOP_ACCESS16BIT_STOPH {
        #[inline(always)]
        fn default() -> STOP_ACCESS16BIT_STOPH {
            STOP_ACCESS16BIT_STOPH(0)
        }
    }
    impl core::fmt::Debug for STOP_ACCESS16BIT_STOPH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STOP_ACCESS16BIT_STOPH")
                .field("STOPH", &self.STOPH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STOP_ACCESS16BIT_STOPH {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STOP_ACCESS16BIT_STOPH {
                STOPH: u16,
            }
            let proxy = STOP_ACCESS16BIT_STOPH {
                STOPH: self.STOPH(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SCT_STOPL register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STOP_ACCESS16BIT_STOPL(pub u32);
    impl STOP_ACCESS16BIT_STOPL {
        #[inline(always)]
        pub const fn STOPL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_STOPL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for STOP_ACCESS16BIT_STOPL {
        #[inline(always)]
        fn default() -> STOP_ACCESS16BIT_STOPL {
            STOP_ACCESS16BIT_STOPL(0)
        }
    }
    impl core::fmt::Debug for STOP_ACCESS16BIT_STOPL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STOP_ACCESS16BIT_STOPL")
                .field("STOPL", &self.STOPL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STOP_ACCESS16BIT_STOPL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STOP_ACCESS16BIT_STOPL {
                STOPL: u16,
            }
            let proxy = STOP_ACCESS16BIT_STOPL {
                STOPL: self.STOPL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
