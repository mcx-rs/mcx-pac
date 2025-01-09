#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRNG {
    ptr: *mut u8,
}
unsafe impl Send for TRNG {}
unsafe impl Sync for TRNG {}
impl TRNG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCTL(self) -> crate::common::Reg<regs::MCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn SCMISC(self) -> crate::common::Reg<regs::SCMISC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRRNG(self) -> crate::common::Reg<regs::PKRRNG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRMAX(self) -> crate::common::Reg<regs::PKRMAX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn PKRSQ(self) -> crate::common::Reg<regs::PKRSQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SDCTL(self) -> crate::common::Reg<regs::SDCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SBLIM(self) -> crate::common::Reg<regs::SBLIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TOTSAM(self) -> crate::common::Reg<regs::TOTSAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn FRQMIN(self) -> crate::common::Reg<regs::FRQMIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn OSC2_FRQCNT(self) -> crate::common::Reg<regs::OSC2_FRQCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn FRQCNT(self) -> crate::common::Reg<regs::FRQCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn FRQMAX(self) -> crate::common::Reg<regs::FRQMAX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCMC(self) -> crate::common::Reg<regs::SCMC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SCML(self) -> crate::common::Reg<regs::SCML, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR1C(self) -> crate::common::Reg<regs::SCR1C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR1L(self) -> crate::common::Reg<regs::SCR1L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR2C(self) -> crate::common::Reg<regs::SCR2C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR2L(self) -> crate::common::Reg<regs::SCR2L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR3C(self) -> crate::common::Reg<regs::SCR3C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCR3L(self) -> crate::common::Reg<regs::SCR3L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCR4C(self) -> crate::common::Reg<regs::SCR4C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR4L(self) -> crate::common::Reg<regs::SCR4L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR5C(self) -> crate::common::Reg<regs::SCR5C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR5L(self) -> crate::common::Reg<regs::SCR5L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR6PC(self) -> crate::common::Reg<regs::SCR6PC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR6PL(self) -> crate::common::Reg<regs::SCR6PL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn ENT(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNT10(self) -> crate::common::Reg<regs::PKRCNT10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNT32(self) -> crate::common::Reg<regs::PKRCNT32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNT54(self) -> crate::common::Reg<regs::PKRCNT54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNT76(self) -> crate::common::Reg<regs::PKRCNT76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNT98(self) -> crate::common::Reg<regs::PKRCNT98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNTBA(self) -> crate::common::Reg<regs::PKRCNTBA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNTDC(self) -> crate::common::Reg<regs::PKRCNTDC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn PKRCNTFE(self) -> crate::common::Reg<regs::PKRCNTFE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_CFG(self) -> crate::common::Reg<regs::SEC_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_CTRL(self) -> crate::common::Reg<regs::INT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_MASK(self) -> crate::common::Reg<regs::INT_MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn INT_STATUS(self) -> crate::common::Reg<regs::INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn CSER(self) -> crate::common::Reg<regs::CSER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn CSCLR(self) -> crate::common::Reg<regs::CSCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn OSC2_CTL(self) -> crate::common::Reg<regs::OSC2_CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn VID1(self) -> crate::common::Reg<regs::VID1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn VID2(self) -> crate::common::Reg<regs::VID2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Common Security Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSCLR(pub u32);
    impl CSCLR {
        #[inline(always)]
        pub const fn RED_SIGS_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RED_SIGS_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RED_FSM_CLR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RED_FSM_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LOCAL_EDC_CLR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCAL_EDC_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn BUS_EDC_CLR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUS_EDC_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for CSCLR {
        #[inline(always)]
        fn default() -> CSCLR {
            CSCLR(0)
        }
    }
    impl core::fmt::Debug for CSCLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSCLR")
                .field("RED_SIGS_CLR", &self.RED_SIGS_CLR())
                .field("RED_FSM_CLR", &self.RED_FSM_CLR())
                .field("LOCAL_EDC_CLR", &self.LOCAL_EDC_CLR())
                .field("BUS_EDC_CLR", &self.BUS_EDC_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSCLR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CSCLR {
                RED_SIGS_CLR: bool,
                RED_FSM_CLR: bool,
                LOCAL_EDC_CLR: bool,
                BUS_EDC_CLR: bool,
            }
            let proxy = CSCLR {
                RED_SIGS_CLR: self.RED_SIGS_CLR(),
                RED_FSM_CLR: self.RED_FSM_CLR(),
                LOCAL_EDC_CLR: self.LOCAL_EDC_CLR(),
                BUS_EDC_CLR: self.BUS_EDC_CLR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Common Security Error Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSER(pub u32);
    impl CSER {
        #[inline(always)]
        pub const fn RED_SIGS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RED_SIGS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RED_FSM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RED_FSM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LOCAL_EDC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCAL_EDC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn BUS_EDC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUS_EDC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for CSER {
        #[inline(always)]
        fn default() -> CSER {
            CSER(0)
        }
    }
    impl core::fmt::Debug for CSER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSER")
                .field("RED_SIGS", &self.RED_SIGS())
                .field("RED_FSM", &self.RED_FSM())
                .field("LOCAL_EDC", &self.LOCAL_EDC())
                .field("BUS_EDC", &self.BUS_EDC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CSER {
                RED_SIGS: bool,
                RED_FSM: bool,
                LOCAL_EDC: bool,
                BUS_EDC: bool,
            }
            let proxy = CSER {
                RED_SIGS: self.RED_SIGS(),
                RED_FSM: self.RED_FSM(),
                LOCAL_EDC: self.LOCAL_EDC(),
                BUS_EDC: self.BUS_EDC(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Frequency Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRQCNT(pub u32);
    impl FRQCNT {
        #[inline(always)]
        pub const fn FRQ_CT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FRQ_CT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for FRQCNT {
        #[inline(always)]
        fn default() -> FRQCNT {
            FRQCNT(0)
        }
    }
    impl core::fmt::Debug for FRQCNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRQCNT")
                .field("FRQ_CT", &self.FRQ_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRQCNT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRQCNT {
                FRQ_CT: u32,
            }
            let proxy = FRQCNT {
                FRQ_CT: self.FRQ_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Frequency Count Maximum Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRQMAX(pub u32);
    impl FRQMAX {
        #[inline(always)]
        pub const fn FRQ_MAX(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FRQ_MAX(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for FRQMAX {
        #[inline(always)]
        fn default() -> FRQMAX {
            FRQMAX(0)
        }
    }
    impl core::fmt::Debug for FRQMAX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRQMAX")
                .field("FRQ_MAX", &self.FRQ_MAX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRQMAX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRQMAX {
                FRQ_MAX: u32,
            }
            let proxy = FRQMAX {
                FRQ_MAX: self.FRQ_MAX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Frequency Count Minimum Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FRQMIN(pub u32);
    impl FRQMIN {
        #[inline(always)]
        pub const fn FRQ_MIN(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FRQ_MIN(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for FRQMIN {
        #[inline(always)]
        fn default() -> FRQMIN {
            FRQMIN(0)
        }
    }
    impl core::fmt::Debug for FRQMIN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FRQMIN")
                .field("FRQ_MIN", &self.FRQ_MIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FRQMIN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FRQMIN {
                FRQ_MIN: u32,
            }
            let proxy = FRQMIN {
                FRQ_MIN: self.FRQ_MIN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_CTRL(pub u32);
    impl INT_CTRL {
        #[inline(always)]
        pub const fn HW_ERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HW_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENT_VAL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRQ_CT_FAIL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRQ_CT_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTG_FLT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_FLT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for INT_CTRL {
        #[inline(always)]
        fn default() -> INT_CTRL {
            INT_CTRL(0)
        }
    }
    impl core::fmt::Debug for INT_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_CTRL")
                .field("HW_ERR", &self.HW_ERR())
                .field("ENT_VAL", &self.ENT_VAL())
                .field("FRQ_CT_FAIL", &self.FRQ_CT_FAIL())
                .field("INTG_FLT", &self.INTG_FLT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INT_CTRL {
                HW_ERR: bool,
                ENT_VAL: bool,
                FRQ_CT_FAIL: bool,
                INTG_FLT: bool,
            }
            let proxy = INT_CTRL {
                HW_ERR: self.HW_ERR(),
                ENT_VAL: self.ENT_VAL(),
                FRQ_CT_FAIL: self.FRQ_CT_FAIL(),
                INTG_FLT: self.INTG_FLT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Mask Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_MASK(pub u32);
    impl INT_MASK {
        #[inline(always)]
        pub const fn HW_ERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HW_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENT_VAL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRQ_CT_FAIL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRQ_CT_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTG_FLT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_FLT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for INT_MASK {
        #[inline(always)]
        fn default() -> INT_MASK {
            INT_MASK(0)
        }
    }
    impl core::fmt::Debug for INT_MASK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_MASK")
                .field("HW_ERR", &self.HW_ERR())
                .field("ENT_VAL", &self.ENT_VAL())
                .field("FRQ_CT_FAIL", &self.FRQ_CT_FAIL())
                .field("INTG_FLT", &self.INTG_FLT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_MASK {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INT_MASK {
                HW_ERR: bool,
                ENT_VAL: bool,
                FRQ_CT_FAIL: bool,
                INTG_FLT: bool,
            }
            let proxy = INT_MASK {
                HW_ERR: self.HW_ERR(),
                ENT_VAL: self.ENT_VAL(),
                FRQ_CT_FAIL: self.FRQ_CT_FAIL(),
                INTG_FLT: self.INTG_FLT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INT_STATUS(pub u32);
    impl INT_STATUS {
        #[inline(always)]
        pub const fn HW_ERR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HW_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENT_VAL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRQ_CT_FAIL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRQ_CT_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INTG_FLT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_FLT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for INT_STATUS {
        #[inline(always)]
        fn default() -> INT_STATUS {
            INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INT_STATUS")
                .field("HW_ERR", &self.HW_ERR())
                .field("ENT_VAL", &self.ENT_VAL())
                .field("FRQ_CT_FAIL", &self.FRQ_CT_FAIL())
                .field("INTG_FLT", &self.INTG_FLT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct INT_STATUS {
                HW_ERR: bool,
                ENT_VAL: bool,
                FRQ_CT_FAIL: bool,
                INTG_FLT: bool,
            }
            let proxy = INT_STATUS {
                HW_ERR: self.HW_ERR(),
                ENT_VAL: self.ENT_VAL(),
                FRQ_CT_FAIL: self.FRQ_CT_FAIL(),
                INTG_FLT: self.INTG_FLT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Miscellaneous Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCTL(pub u32);
    impl MCTL {
        #[inline(always)]
        pub const fn OSC_DIV(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OSC_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DIS_SLF_TST(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_SLF_TST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TRNG_ACC(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRNG_ACC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RST_DEF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RST_DEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn FCT_FAIL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FCT_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FCT_VAL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FCT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ENT_VAL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ERR(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn TSTOP_OK(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSTOP_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn OSC2_FAIL(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC2_FAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn PRGM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PRGM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn INTG_ERR(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTG_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MCTL {
        #[inline(always)]
        fn default() -> MCTL {
            MCTL(0)
        }
    }
    impl core::fmt::Debug for MCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCTL")
                .field("OSC_DIV", &self.OSC_DIV())
                .field("DIS_SLF_TST", &self.DIS_SLF_TST())
                .field("TRNG_ACC", &self.TRNG_ACC())
                .field("RST_DEF", &self.RST_DEF())
                .field("FCT_FAIL", &self.FCT_FAIL())
                .field("FCT_VAL", &self.FCT_VAL())
                .field("ENT_VAL", &self.ENT_VAL())
                .field("ERR", &self.ERR())
                .field("TSTOP_OK", &self.TSTOP_OK())
                .field("OSC2_FAIL", &self.OSC2_FAIL())
                .field("PRGM", &self.PRGM())
                .field("INTG_ERR", &self.INTG_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCTL {
                OSC_DIV: u8,
                DIS_SLF_TST: bool,
                TRNG_ACC: bool,
                RST_DEF: bool,
                FCT_FAIL: bool,
                FCT_VAL: bool,
                ENT_VAL: bool,
                ERR: bool,
                TSTOP_OK: bool,
                OSC2_FAIL: bool,
                PRGM: bool,
                INTG_ERR: bool,
            }
            let proxy = MCTL {
                OSC_DIV: self.OSC_DIV(),
                DIS_SLF_TST: self.DIS_SLF_TST(),
                TRNG_ACC: self.TRNG_ACC(),
                RST_DEF: self.RST_DEF(),
                FCT_FAIL: self.FCT_FAIL(),
                FCT_VAL: self.FCT_VAL(),
                ENT_VAL: self.ENT_VAL(),
                ERR: self.ERR(),
                TSTOP_OK: self.TSTOP_OK(),
                OSC2_FAIL: self.OSC2_FAIL(),
                PRGM: self.PRGM(),
                INTG_ERR: self.INTG_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TRNG Oscillator 2 Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSC2_CTL(pub u32);
    impl OSC2_CTL {
        #[inline(always)]
        pub const fn TRNG_ENT_CTL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRNG_ENT_CTL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn OSC2_DIV(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OSC2_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn OSC2_OUT_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC2_OUT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OSC2_FCT_VAL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC2_FCT_VAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn OSC_FAILSAFE_LMT(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_OSC_FAILSAFE_LMT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn OSC_FAILSAFE_TEST(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_FAILSAFE_TEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for OSC2_CTL {
        #[inline(always)]
        fn default() -> OSC2_CTL {
            OSC2_CTL(0)
        }
    }
    impl core::fmt::Debug for OSC2_CTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSC2_CTL")
                .field("TRNG_ENT_CTL", &self.TRNG_ENT_CTL())
                .field("OSC2_DIV", &self.OSC2_DIV())
                .field("OSC2_OUT_EN", &self.OSC2_OUT_EN())
                .field("OSC2_FCT_VAL", &self.OSC2_FCT_VAL())
                .field("OSC_FAILSAFE_LMT", &self.OSC_FAILSAFE_LMT())
                .field("OSC_FAILSAFE_TEST", &self.OSC_FAILSAFE_TEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSC2_CTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OSC2_CTL {
                TRNG_ENT_CTL: u8,
                OSC2_DIV: u8,
                OSC2_OUT_EN: bool,
                OSC2_FCT_VAL: bool,
                OSC_FAILSAFE_LMT: u8,
                OSC_FAILSAFE_TEST: bool,
            }
            let proxy = OSC2_CTL {
                TRNG_ENT_CTL: self.TRNG_ENT_CTL(),
                OSC2_DIV: self.OSC2_DIV(),
                OSC2_OUT_EN: self.OSC2_OUT_EN(),
                OSC2_FCT_VAL: self.OSC2_FCT_VAL(),
                OSC_FAILSAFE_LMT: self.OSC_FAILSAFE_LMT(),
                OSC_FAILSAFE_TEST: self.OSC_FAILSAFE_TEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Oscillator-2 Frequency Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSC2_FRQCNT(pub u32);
    impl OSC2_FRQCNT {
        #[inline(always)]
        pub const fn OSC2_FRQ_CT(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OSC2_FRQ_CT(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for OSC2_FRQCNT {
        #[inline(always)]
        fn default() -> OSC2_FRQCNT {
            OSC2_FRQCNT(0)
        }
    }
    impl core::fmt::Debug for OSC2_FRQCNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSC2_FRQCNT")
                .field("OSC2_FRQ_CT", &self.OSC2_FRQ_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSC2_FRQCNT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct OSC2_FRQCNT {
                OSC2_FRQ_CT: u32,
            }
            let proxy = OSC2_FRQCNT {
                OSC2_FRQ_CT: self.OSC2_FRQ_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count 1 and 0 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNT10(pub u32);
    impl PKRCNT10 {
        #[inline(always)]
        pub const fn PKR_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNT10 {
        #[inline(always)]
        fn default() -> PKRCNT10 {
            PKRCNT10(0)
        }
    }
    impl core::fmt::Debug for PKRCNT10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNT10")
                .field("PKR_0_CT", &self.PKR_0_CT())
                .field("PKR_1_CT", &self.PKR_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNT10 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNT10 {
                PKR_0_CT: u16,
                PKR_1_CT: u16,
            }
            let proxy = PKRCNT10 {
                PKR_0_CT: self.PKR_0_CT(),
                PKR_1_CT: self.PKR_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count 3 and 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNT32(pub u32);
    impl PKRCNT32 {
        #[inline(always)]
        pub const fn PKR_2_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_2_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_3_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_3_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNT32 {
        #[inline(always)]
        fn default() -> PKRCNT32 {
            PKRCNT32(0)
        }
    }
    impl core::fmt::Debug for PKRCNT32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNT32")
                .field("PKR_2_CT", &self.PKR_2_CT())
                .field("PKR_3_CT", &self.PKR_3_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNT32 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNT32 {
                PKR_2_CT: u16,
                PKR_3_CT: u16,
            }
            let proxy = PKRCNT32 {
                PKR_2_CT: self.PKR_2_CT(),
                PKR_3_CT: self.PKR_3_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count 5 and 4 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNT54(pub u32);
    impl PKRCNT54 {
        #[inline(always)]
        pub const fn PKR_4_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_4_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_5_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_5_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNT54 {
        #[inline(always)]
        fn default() -> PKRCNT54 {
            PKRCNT54(0)
        }
    }
    impl core::fmt::Debug for PKRCNT54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNT54")
                .field("PKR_4_CT", &self.PKR_4_CT())
                .field("PKR_5_CT", &self.PKR_5_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNT54 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNT54 {
                PKR_4_CT: u16,
                PKR_5_CT: u16,
            }
            let proxy = PKRCNT54 {
                PKR_4_CT: self.PKR_4_CT(),
                PKR_5_CT: self.PKR_5_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count 7 and 6 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNT76(pub u32);
    impl PKRCNT76 {
        #[inline(always)]
        pub const fn PKR_6_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_6_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_7_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_7_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNT76 {
        #[inline(always)]
        fn default() -> PKRCNT76 {
            PKRCNT76(0)
        }
    }
    impl core::fmt::Debug for PKRCNT76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNT76")
                .field("PKR_6_CT", &self.PKR_6_CT())
                .field("PKR_7_CT", &self.PKR_7_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNT76 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNT76 {
                PKR_6_CT: u16,
                PKR_7_CT: u16,
            }
            let proxy = PKRCNT76 {
                PKR_6_CT: self.PKR_6_CT(),
                PKR_7_CT: self.PKR_7_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count 9 and 8 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNT98(pub u32);
    impl PKRCNT98 {
        #[inline(always)]
        pub const fn PKR_8_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_8_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_9_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_9_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNT98 {
        #[inline(always)]
        fn default() -> PKRCNT98 {
            PKRCNT98(0)
        }
    }
    impl core::fmt::Debug for PKRCNT98 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNT98")
                .field("PKR_8_CT", &self.PKR_8_CT())
                .field("PKR_9_CT", &self.PKR_9_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNT98 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNT98 {
                PKR_8_CT: u16,
                PKR_9_CT: u16,
            }
            let proxy = PKRCNT98 {
                PKR_8_CT: self.PKR_8_CT(),
                PKR_9_CT: self.PKR_9_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count B and A Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNTBA(pub u32);
    impl PKRCNTBA {
        #[inline(always)]
        pub const fn PKR_A_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_A_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_B_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_B_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNTBA {
        #[inline(always)]
        fn default() -> PKRCNTBA {
            PKRCNTBA(0)
        }
    }
    impl core::fmt::Debug for PKRCNTBA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNTBA")
                .field("PKR_A_CT", &self.PKR_A_CT())
                .field("PKR_B_CT", &self.PKR_B_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNTBA {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNTBA {
                PKR_A_CT: u16,
                PKR_B_CT: u16,
            }
            let proxy = PKRCNTBA {
                PKR_A_CT: self.PKR_A_CT(),
                PKR_B_CT: self.PKR_B_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count D and C Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNTDC(pub u32);
    impl PKRCNTDC {
        #[inline(always)]
        pub const fn PKR_C_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_C_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_D_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_D_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNTDC {
        #[inline(always)]
        fn default() -> PKRCNTDC {
            PKRCNTDC(0)
        }
    }
    impl core::fmt::Debug for PKRCNTDC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNTDC")
                .field("PKR_C_CT", &self.PKR_C_CT())
                .field("PKR_D_CT", &self.PKR_D_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNTDC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNTDC {
                PKR_C_CT: u16,
                PKR_D_CT: u16,
            }
            let proxy = PKRCNTDC {
                PKR_C_CT: self.PKR_C_CT(),
                PKR_D_CT: self.PKR_D_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Poker Count F and E Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRCNTFE(pub u32);
    impl PKRCNTFE {
        #[inline(always)]
        pub const fn PKR_E_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_E_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn PKR_F_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_F_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PKRCNTFE {
        #[inline(always)]
        fn default() -> PKRCNTFE {
            PKRCNTFE(0)
        }
    }
    impl core::fmt::Debug for PKRCNTFE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRCNTFE")
                .field("PKR_E_CT", &self.PKR_E_CT())
                .field("PKR_F_CT", &self.PKR_F_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRCNTFE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRCNTFE {
                PKR_E_CT: u16,
                PKR_F_CT: u16,
            }
            let proxy = PKRCNTFE {
                PKR_E_CT: self.PKR_E_CT(),
                PKR_F_CT: self.PKR_F_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Poker Maximum Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRMAX(pub u32);
    impl PKRMAX {
        #[inline(always)]
        pub const fn PKR_MAX(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PKR_MAX(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for PKRMAX {
        #[inline(always)]
        fn default() -> PKRMAX {
            PKRMAX(0)
        }
    }
    impl core::fmt::Debug for PKRMAX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRMAX")
                .field("PKR_MAX", &self.PKR_MAX())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRMAX {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRMAX {
                PKR_MAX: u32,
            }
            let proxy = PKRMAX {
                PKR_MAX: self.PKR_MAX(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Poker Range Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRRNG(pub u32);
    impl PKRRNG {
        #[inline(always)]
        pub const fn PKR_RNG(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PKR_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PKRRNG {
        #[inline(always)]
        fn default() -> PKRRNG {
            PKRRNG(0)
        }
    }
    impl core::fmt::Debug for PKRRNG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRRNG")
                .field("PKR_RNG", &self.PKR_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRRNG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRRNG {
                PKR_RNG: u16,
            }
            let proxy = PKRRNG {
                PKR_RNG: self.PKR_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Poker Square Calculation Result Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PKRSQ(pub u32);
    impl PKRSQ {
        #[inline(always)]
        pub const fn PKR_SQ(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PKR_SQ(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for PKRSQ {
        #[inline(always)]
        fn default() -> PKRSQ {
            PKRSQ(0)
        }
    }
    impl core::fmt::Debug for PKRSQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PKRSQ")
                .field("PKR_SQ", &self.PKR_SQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PKRSQ {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PKRSQ {
                PKR_SQ: u32,
            }
            let proxy = PKRSQ {
                PKR_SQ: self.PKR_SQ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Sparse Bit Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SBLIM(pub u32);
    impl SBLIM {
        #[inline(always)]
        pub const fn SB_LIM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SB_LIM(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for SBLIM {
        #[inline(always)]
        fn default() -> SBLIM {
            SBLIM(0)
        }
    }
    impl core::fmt::Debug for SBLIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SBLIM")
                .field("SB_LIM", &self.SB_LIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SBLIM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SBLIM {
                SB_LIM: u16,
            }
            let proxy = SBLIM {
                SB_LIM: self.SB_LIM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Monobit Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCMC(pub u32);
    impl SCMC {
        #[inline(always)]
        pub const fn MONO_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MONO_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SCMC {
        #[inline(always)]
        fn default() -> SCMC {
            SCMC(0)
        }
    }
    impl core::fmt::Debug for SCMC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCMC")
                .field("MONO_CT", &self.MONO_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCMC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCMC {
                MONO_CT: u16,
            }
            let proxy = SCMC {
                MONO_CT: self.MONO_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Miscellaneous Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCMISC(pub u32);
    impl SCMISC {
        #[inline(always)]
        pub const fn LRUN_MAX(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_LRUN_MAX(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RTY_CT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RTY_CT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for SCMISC {
        #[inline(always)]
        fn default() -> SCMISC {
            SCMISC(0)
        }
    }
    impl core::fmt::Debug for SCMISC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCMISC")
                .field("LRUN_MAX", &self.LRUN_MAX())
                .field("RTY_CT", &self.RTY_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCMISC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCMISC {
                LRUN_MAX: u8,
                RTY_CT: u8,
            }
            let proxy = SCMISC {
                LRUN_MAX: self.LRUN_MAX(),
                RTY_CT: self.RTY_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Monobit Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCML(pub u32);
    impl SCML {
        #[inline(always)]
        pub const fn MONO_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MONO_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MONO_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MONO_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SCML {
        #[inline(always)]
        fn default() -> SCML {
            SCML(0)
        }
    }
    impl core::fmt::Debug for SCML {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCML")
                .field("MONO_MAX", &self.MONO_MAX())
                .field("MONO_RNG", &self.MONO_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCML {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCML {
                MONO_MAX: u16,
                MONO_RNG: u16,
            }
            let proxy = SCML {
                MONO_MAX: self.MONO_MAX(),
                MONO_RNG: self.MONO_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 1 Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR1C(pub u32);
    impl SCR1C {
        #[inline(always)]
        pub const fn R1_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R1_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[inline(always)]
        pub const fn R1_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R1_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for SCR1C {
        #[inline(always)]
        fn default() -> SCR1C {
            SCR1C(0)
        }
    }
    impl core::fmt::Debug for SCR1C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR1C")
                .field("R1_0_CT", &self.R1_0_CT())
                .field("R1_1_CT", &self.R1_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR1C {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR1C {
                R1_0_CT: u16,
                R1_1_CT: u16,
            }
            let proxy = SCR1C {
                R1_0_CT: self.R1_0_CT(),
                R1_1_CT: self.R1_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 1 Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR1L(pub u32);
    impl SCR1L {
        #[inline(always)]
        pub const fn RUN1_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN1_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN1_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN1_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for SCR1L {
        #[inline(always)]
        fn default() -> SCR1L {
            SCR1L(0)
        }
    }
    impl core::fmt::Debug for SCR1L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR1L")
                .field("RUN1_MAX", &self.RUN1_MAX())
                .field("RUN1_RNG", &self.RUN1_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR1L {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR1L {
                RUN1_MAX: u16,
                RUN1_RNG: u16,
            }
            let proxy = SCR1L {
                RUN1_MAX: self.RUN1_MAX(),
                RUN1_RNG: self.RUN1_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 2 Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR2C(pub u32);
    impl SCR2C {
        #[inline(always)]
        pub const fn R2_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R2_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[inline(always)]
        pub const fn R2_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R2_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for SCR2C {
        #[inline(always)]
        fn default() -> SCR2C {
            SCR2C(0)
        }
    }
    impl core::fmt::Debug for SCR2C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR2C")
                .field("R2_0_CT", &self.R2_0_CT())
                .field("R2_1_CT", &self.R2_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR2C {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR2C {
                R2_0_CT: u16,
                R2_1_CT: u16,
            }
            let proxy = SCR2C {
                R2_0_CT: self.R2_0_CT(),
                R2_1_CT: self.R2_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 2 Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR2L(pub u32);
    impl SCR2L {
        #[inline(always)]
        pub const fn RUN2_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN2_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN2_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN2_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for SCR2L {
        #[inline(always)]
        fn default() -> SCR2L {
            SCR2L(0)
        }
    }
    impl core::fmt::Debug for SCR2L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR2L")
                .field("RUN2_MAX", &self.RUN2_MAX())
                .field("RUN2_RNG", &self.RUN2_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR2L {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR2L {
                RUN2_MAX: u16,
                RUN2_RNG: u16,
            }
            let proxy = SCR2L {
                RUN2_MAX: self.RUN2_MAX(),
                RUN2_RNG: self.RUN2_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 3 Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR3C(pub u32);
    impl SCR3C {
        #[inline(always)]
        pub const fn R3_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R3_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[inline(always)]
        pub const fn R3_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R3_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for SCR3C {
        #[inline(always)]
        fn default() -> SCR3C {
            SCR3C(0)
        }
    }
    impl core::fmt::Debug for SCR3C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR3C")
                .field("R3_0_CT", &self.R3_0_CT())
                .field("R3_1_CT", &self.R3_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR3C {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR3C {
                R3_0_CT: u16,
                R3_1_CT: u16,
            }
            let proxy = SCR3C {
                R3_0_CT: self.R3_0_CT(),
                R3_1_CT: self.R3_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 3 Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR3L(pub u32);
    impl SCR3L {
        #[inline(always)]
        pub const fn RUN3_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN3_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN3_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN3_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for SCR3L {
        #[inline(always)]
        fn default() -> SCR3L {
            SCR3L(0)
        }
    }
    impl core::fmt::Debug for SCR3L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR3L")
                .field("RUN3_MAX", &self.RUN3_MAX())
                .field("RUN3_RNG", &self.RUN3_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR3L {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR3L {
                RUN3_MAX: u16,
                RUN3_RNG: u16,
            }
            let proxy = SCR3L {
                RUN3_MAX: self.RUN3_MAX(),
                RUN3_RNG: self.RUN3_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 4 Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR4C(pub u32);
    impl SCR4C {
        #[inline(always)]
        pub const fn R4_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R4_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[inline(always)]
        pub const fn R4_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R4_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for SCR4C {
        #[inline(always)]
        fn default() -> SCR4C {
            SCR4C(0)
        }
    }
    impl core::fmt::Debug for SCR4C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR4C")
                .field("R4_0_CT", &self.R4_0_CT())
                .field("R4_1_CT", &self.R4_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR4C {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR4C {
                R4_0_CT: u16,
                R4_1_CT: u16,
            }
            let proxy = SCR4C {
                R4_0_CT: self.R4_0_CT(),
                R4_1_CT: self.R4_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 4 Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR4L(pub u32);
    impl SCR4L {
        #[inline(always)]
        pub const fn RUN4_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN4_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN4_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN4_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for SCR4L {
        #[inline(always)]
        fn default() -> SCR4L {
            SCR4L(0)
        }
    }
    impl core::fmt::Debug for SCR4L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR4L")
                .field("RUN4_MAX", &self.RUN4_MAX())
                .field("RUN4_RNG", &self.RUN4_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR4L {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR4L {
                RUN4_MAX: u16,
                RUN4_RNG: u16,
            }
            let proxy = SCR4L {
                RUN4_MAX: self.RUN4_MAX(),
                RUN4_RNG: self.RUN4_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 5 Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR5C(pub u32);
    impl SCR5C {
        #[inline(always)]
        pub const fn R5_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R5_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn R5_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R5_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for SCR5C {
        #[inline(always)]
        fn default() -> SCR5C {
            SCR5C(0)
        }
    }
    impl core::fmt::Debug for SCR5C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR5C")
                .field("R5_0_CT", &self.R5_0_CT())
                .field("R5_1_CT", &self.R5_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR5C {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR5C {
                R5_0_CT: u16,
                R5_1_CT: u16,
            }
            let proxy = SCR5C {
                R5_0_CT: self.R5_0_CT(),
                R5_1_CT: self.R5_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 5 Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR5L(pub u32);
    impl SCR5L {
        #[inline(always)]
        pub const fn RUN5_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN5_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN5_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN5_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for SCR5L {
        #[inline(always)]
        fn default() -> SCR5L {
            SCR5L(0)
        }
    }
    impl core::fmt::Debug for SCR5L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR5L")
                .field("RUN5_MAX", &self.RUN5_MAX())
                .field("RUN5_RNG", &self.RUN5_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR5L {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR5L {
                RUN5_MAX: u16,
                RUN5_RNG: u16,
            }
            let proxy = SCR5L {
                RUN5_MAX: self.RUN5_MAX(),
                RUN5_RNG: self.RUN5_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 6+ Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR6PC(pub u32);
    impl SCR6PC {
        #[inline(always)]
        pub const fn R6P_0_CT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R6P_0_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn R6P_1_CT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_R6P_1_CT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for SCR6PC {
        #[inline(always)]
        fn default() -> SCR6PC {
            SCR6PC(0)
        }
    }
    impl core::fmt::Debug for SCR6PC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR6PC")
                .field("R6P_0_CT", &self.R6P_0_CT())
                .field("R6P_1_CT", &self.R6P_1_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR6PC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR6PC {
                R6P_0_CT: u16,
                R6P_1_CT: u16,
            }
            let proxy = SCR6PC {
                R6P_0_CT: self.R6P_0_CT(),
                R6P_1_CT: self.R6P_1_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Statistical Check Run Length 6+ Limit Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR6PL(pub u32);
    impl SCR6PL {
        #[inline(always)]
        pub const fn RUN6P_MAX(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN6P_MAX(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn RUN6P_RNG(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RUN6P_RNG(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for SCR6PL {
        #[inline(always)]
        fn default() -> SCR6PL {
            SCR6PL(0)
        }
    }
    impl core::fmt::Debug for SCR6PL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR6PL")
                .field("RUN6P_MAX", &self.RUN6P_MAX())
                .field("RUN6P_RNG", &self.RUN6P_RNG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR6PL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR6PL {
                RUN6P_MAX: u16,
                RUN6P_RNG: u16,
            }
            let proxy = SCR6PL {
                RUN6P_MAX: self.RUN6P_MAX(),
                RUN6P_RNG: self.RUN6P_RNG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Seed Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDCTL(pub u32);
    impl SDCTL {
        #[inline(always)]
        pub const fn SAMP_SIZE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SAMP_SIZE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn ENT_DLY(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ENT_DLY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SDCTL {
        #[inline(always)]
        fn default() -> SDCTL {
            SDCTL(0)
        }
    }
    impl core::fmt::Debug for SDCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDCTL")
                .field("SAMP_SIZE", &self.SAMP_SIZE())
                .field("ENT_DLY", &self.ENT_DLY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDCTL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SDCTL {
                SAMP_SIZE: u16,
                ENT_DLY: u16,
            }
            let proxy = SDCTL {
                SAMP_SIZE: self.SAMP_SIZE(),
                ENT_DLY: self.ENT_DLY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Security Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_CFG(pub u32);
    impl SEC_CFG {
        #[inline(always)]
        pub const fn NO_PRGM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NO_PRGM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SEC_CFG {
        #[inline(always)]
        fn default() -> SEC_CFG {
            SEC_CFG(0)
        }
    }
    impl core::fmt::Debug for SEC_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_CFG")
                .field("NO_PRGM", &self.NO_PRGM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SEC_CFG {
                NO_PRGM: bool,
            }
            let proxy = SEC_CFG {
                NO_PRGM: self.NO_PRGM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[inline(always)]
        pub const fn TF1BR0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF1BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TF1BR1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF1BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TF2BR0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF2BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TF2BR1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF2BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn TF3BR0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF3BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn TF3BR1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF3BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn TF4BR0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF4BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn TF4BR1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF4BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn TF5BR0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF5BR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TF5BR1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF5BR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn TF6PBR0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF6PBR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn TF6PBR1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TF6PBR1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn TFSB(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TFSB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn TFLR(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TFLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn TFP(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TFP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn TFMB(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TFMB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn RETRY_CT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RETRY_CT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
                .field("TF1BR0", &self.TF1BR0())
                .field("TF1BR1", &self.TF1BR1())
                .field("TF2BR0", &self.TF2BR0())
                .field("TF2BR1", &self.TF2BR1())
                .field("TF3BR0", &self.TF3BR0())
                .field("TF3BR1", &self.TF3BR1())
                .field("TF4BR0", &self.TF4BR0())
                .field("TF4BR1", &self.TF4BR1())
                .field("TF5BR0", &self.TF5BR0())
                .field("TF5BR1", &self.TF5BR1())
                .field("TF6PBR0", &self.TF6PBR0())
                .field("TF6PBR1", &self.TF6PBR1())
                .field("TFSB", &self.TFSB())
                .field("TFLR", &self.TFLR())
                .field("TFP", &self.TFP())
                .field("TFMB", &self.TFMB())
                .field("RETRY_CT", &self.RETRY_CT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STATUS {
                TF1BR0: bool,
                TF1BR1: bool,
                TF2BR0: bool,
                TF2BR1: bool,
                TF3BR0: bool,
                TF3BR1: bool,
                TF4BR0: bool,
                TF4BR1: bool,
                TF5BR0: bool,
                TF5BR1: bool,
                TF6PBR0: bool,
                TF6PBR1: bool,
                TFSB: bool,
                TFLR: bool,
                TFP: bool,
                TFMB: bool,
                RETRY_CT: u8,
            }
            let proxy = STATUS {
                TF1BR0: self.TF1BR0(),
                TF1BR1: self.TF1BR1(),
                TF2BR0: self.TF2BR0(),
                TF2BR1: self.TF2BR1(),
                TF3BR0: self.TF3BR0(),
                TF3BR1: self.TF3BR1(),
                TF4BR0: self.TF4BR0(),
                TF4BR1: self.TF4BR1(),
                TF5BR0: self.TF5BR0(),
                TF5BR1: self.TF5BR1(),
                TF6PBR0: self.TF6PBR0(),
                TF6PBR1: self.TF6PBR1(),
                TFSB: self.TFSB(),
                TFLR: self.TFLR(),
                TFP: self.TFP(),
                TFMB: self.TFMB(),
                RETRY_CT: self.RETRY_CT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Total Samples Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TOTSAM(pub u32);
    impl TOTSAM {
        #[inline(always)]
        pub const fn TOT_SAM(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TOT_SAM(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for TOTSAM {
        #[inline(always)]
        fn default() -> TOTSAM {
            TOTSAM(0)
        }
    }
    impl core::fmt::Debug for TOTSAM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TOTSAM")
                .field("TOT_SAM", &self.TOT_SAM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TOTSAM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct TOTSAM {
                TOT_SAM: u32,
            }
            let proxy = TOTSAM {
                TOT_SAM: self.TOT_SAM(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version ID Register (MS)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VID1(pub u32);
    impl VID1 {
        #[inline(always)]
        pub const fn MIN_REV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MIN_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn MAJ_REV(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJ_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn IP_ID(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IP_ID(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for VID1 {
        #[inline(always)]
        fn default() -> VID1 {
            VID1(0)
        }
    }
    impl core::fmt::Debug for VID1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VID1")
                .field("MIN_REV", &self.MIN_REV())
                .field("MAJ_REV", &self.MAJ_REV())
                .field("IP_ID", &self.IP_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VID1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VID1 {
                MIN_REV: u8,
                MAJ_REV: u8,
                IP_ID: u16,
            }
            let proxy = VID1 {
                MIN_REV: self.MIN_REV(),
                MAJ_REV: self.MAJ_REV(),
                IP_ID: self.IP_ID(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Version ID Register (LS)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VID2(pub u32);
    impl VID2 {
        #[inline(always)]
        pub const fn CONFIG_OPT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_CONFIG_OPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn ECO_REV(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ECO_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn INTG_OPT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INTG_OPT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn ERA(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ERA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for VID2 {
        #[inline(always)]
        fn default() -> VID2 {
            VID2(0)
        }
    }
    impl core::fmt::Debug for VID2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VID2")
                .field("CONFIG_OPT", &self.CONFIG_OPT())
                .field("ECO_REV", &self.ECO_REV())
                .field("INTG_OPT", &self.INTG_OPT())
                .field("ERA", &self.ERA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VID2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct VID2 {
                CONFIG_OPT: u8,
                ECO_REV: u8,
                INTG_OPT: u8,
                ERA: u8,
            }
            let proxy = VID2 {
                CONFIG_OPT: self.CONFIG_OPT(),
                ECO_REV: self.ECO_REV(),
                INTG_OPT: self.INTG_OPT(),
                ERA: self.ERA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
