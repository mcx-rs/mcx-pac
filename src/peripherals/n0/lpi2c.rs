#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C {
    ptr: *mut u8,
}
unsafe impl Send for LPI2C {}
unsafe impl Sync for LPI2C {}
impl LPI2C {
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
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<regs::MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MIER(self) -> crate::common::Reg<regs::MIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn MDER(self) -> crate::common::Reg<regs::MDER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR0(self) -> crate::common::Reg<regs::MCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR1(self) -> crate::common::Reg<regs::MCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR2(self) -> crate::common::Reg<regs::MCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR3(self) -> crate::common::Reg<regs::MCFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn MDMR(self) -> crate::common::Reg<regs::MDMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn MCCR0(self) -> crate::common::Reg<regs::MCCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn MCCR1(self) -> crate::common::Reg<regs::MCCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn MFCR(self) -> crate::common::Reg<regs::MFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn MFSR(self) -> crate::common::Reg<regs::MFSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn MTDR(self) -> crate::common::Reg<regs::MTDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDR(self) -> crate::common::Reg<regs::MRDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDROR(self) -> crate::common::Reg<regs::MRDROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR(self) -> crate::common::Reg<regs::SCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn SSR(self) -> crate::common::Reg<regs::SSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn SIER(self) -> crate::common::Reg<regs::SIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn SDER(self) -> crate::common::Reg<regs::SDER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR0(self) -> crate::common::Reg<regs::SCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR1(self) -> crate::common::Reg<regs::SCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR2(self) -> crate::common::Reg<regs::SCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn SAMR(self) -> crate::common::Reg<regs::SAMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn SASR(self) -> crate::common::Reg<regs::SASR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn STAR(self) -> crate::common::Reg<regs::STAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn STDR(self) -> crate::common::Reg<regs::STDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDR(self) -> crate::common::Reg<regs::SRDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDROR(self) -> crate::common::Reg<regs::SRDROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn MTCBR(self, n: usize) -> crate::common::Reg<regs::MTCBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MTDBR(self, n: usize) -> crate::common::Reg<regs::MTDBR, crate::common::RW> {
        assert!(n < 253usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Controller Clock Configuration 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCCR0(pub u32);
    impl MCCR0 {
        #[inline(always)]
        pub const fn CLKLO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKLO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CLKHI(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKHI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn SETHOLD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETHOLD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[inline(always)]
        pub const fn DATAVD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATAVD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for MCCR0 {
        #[inline(always)]
        fn default() -> MCCR0 {
            MCCR0(0)
        }
    }
    impl core::fmt::Debug for MCCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCCR0")
                .field("CLKLO", &self.CLKLO())
                .field("CLKHI", &self.CLKHI())
                .field("SETHOLD", &self.SETHOLD())
                .field("DATAVD", &self.DATAVD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCCR0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCCR0 {
                CLKLO: u8,
                CLKHI: u8,
                SETHOLD: u8,
                DATAVD: u8,
            }
            let proxy = MCCR0 {
                CLKLO: self.CLKLO(),
                CLKHI: self.CLKHI(),
                SETHOLD: self.SETHOLD(),
                DATAVD: self.DATAVD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Clock Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCCR1(pub u32);
    impl MCCR1 {
        #[inline(always)]
        pub const fn CLKLO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKLO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CLKHI(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKHI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn SETHOLD(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SETHOLD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[inline(always)]
        pub const fn DATAVD(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATAVD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for MCCR1 {
        #[inline(always)]
        fn default() -> MCCR1 {
            MCCR1(0)
        }
    }
    impl core::fmt::Debug for MCCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCCR1")
                .field("CLKLO", &self.CLKLO())
                .field("CLKHI", &self.CLKHI())
                .field("SETHOLD", &self.SETHOLD())
                .field("DATAVD", &self.DATAVD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCCR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCCR1 {
                CLKLO: u8,
                CLKHI: u8,
                SETHOLD: u8,
                DATAVD: u8,
            }
            let proxy = MCCR1 {
                CLKLO: self.CLKLO(),
                CLKHI: self.CLKHI(),
                SETHOLD: self.SETHOLD(),
                DATAVD: self.DATAVD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Configuration 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCFGR0(pub u32);
    impl MCFGR0 {
        #[inline(always)]
        pub const fn HREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn HRPOL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HRPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn HRSEL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HRSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn HRDIR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HRDIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CIRFIFO(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CIRFIFO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RDMO(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDMO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RELAX(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RELAX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ABORT(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ABORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for MCFGR0 {
        #[inline(always)]
        fn default() -> MCFGR0 {
            MCFGR0(0)
        }
    }
    impl core::fmt::Debug for MCFGR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCFGR0")
                .field("HREN", &self.HREN())
                .field("HRPOL", &self.HRPOL())
                .field("HRSEL", &self.HRSEL())
                .field("HRDIR", &self.HRDIR())
                .field("CIRFIFO", &self.CIRFIFO())
                .field("RDMO", &self.RDMO())
                .field("RELAX", &self.RELAX())
                .field("ABORT", &self.ABORT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCFGR0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCFGR0 {
                HREN: bool,
                HRPOL: bool,
                HRSEL: bool,
                HRDIR: bool,
                CIRFIFO: bool,
                RDMO: bool,
                RELAX: bool,
                ABORT: bool,
            }
            let proxy = MCFGR0 {
                HREN: self.HREN(),
                HRPOL: self.HRPOL(),
                HRSEL: self.HRSEL(),
                HRDIR: self.HRDIR(),
                CIRFIFO: self.CIRFIFO(),
                RDMO: self.RDMO(),
                RELAX: self.RELAX(),
                ABORT: self.ABORT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCFGR1(pub u32);
    impl MCFGR1 {
        #[inline(always)]
        pub const fn PRESCALE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRESCALE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn AUTOSTOP(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AUTOSTOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IGNACK(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IGNACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn TIMECFG(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMECFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn STOPCFG(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOPCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn STARTCFG(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STARTCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn MATCFG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MATCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn PINCFG(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for MCFGR1 {
        #[inline(always)]
        fn default() -> MCFGR1 {
            MCFGR1(0)
        }
    }
    impl core::fmt::Debug for MCFGR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCFGR1")
                .field("PRESCALE", &self.PRESCALE())
                .field("AUTOSTOP", &self.AUTOSTOP())
                .field("IGNACK", &self.IGNACK())
                .field("TIMECFG", &self.TIMECFG())
                .field("STOPCFG", &self.STOPCFG())
                .field("STARTCFG", &self.STARTCFG())
                .field("MATCFG", &self.MATCFG())
                .field("PINCFG", &self.PINCFG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCFGR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCFGR1 {
                PRESCALE: u8,
                AUTOSTOP: bool,
                IGNACK: bool,
                TIMECFG: bool,
                STOPCFG: bool,
                STARTCFG: bool,
                MATCFG: u8,
                PINCFG: u8,
            }
            let proxy = MCFGR1 {
                PRESCALE: self.PRESCALE(),
                AUTOSTOP: self.AUTOSTOP(),
                IGNACK: self.IGNACK(),
                TIMECFG: self.TIMECFG(),
                STOPCFG: self.STOPCFG(),
                STARTCFG: self.STARTCFG(),
                MATCFG: self.MATCFG(),
                PINCFG: self.PINCFG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCFGR2(pub u32);
    impl MCFGR2 {
        #[inline(always)]
        pub const fn BUSIDLE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_BUSIDLE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[inline(always)]
        pub const fn FILTSCL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSCL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn FILTSDA(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSDA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for MCFGR2 {
        #[inline(always)]
        fn default() -> MCFGR2 {
            MCFGR2(0)
        }
    }
    impl core::fmt::Debug for MCFGR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCFGR2")
                .field("BUSIDLE", &self.BUSIDLE())
                .field("FILTSCL", &self.FILTSCL())
                .field("FILTSDA", &self.FILTSDA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCFGR2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCFGR2 {
                BUSIDLE: u16,
                FILTSCL: u8,
                FILTSDA: u8,
            }
            let proxy = MCFGR2 {
                BUSIDLE: self.BUSIDLE(),
                FILTSCL: self.FILTSCL(),
                FILTSDA: self.FILTSDA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Configuration 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCFGR3(pub u32);
    impl MCFGR3 {
        #[inline(always)]
        pub const fn PINLOW(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PINLOW(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
        }
    }
    impl Default for MCFGR3 {
        #[inline(always)]
        fn default() -> MCFGR3 {
            MCFGR3(0)
        }
    }
    impl core::fmt::Debug for MCFGR3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCFGR3")
                .field("PINLOW", &self.PINLOW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCFGR3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCFGR3 {
                PINLOW: u16,
            }
            let proxy = MCFGR3 {
                PINLOW: self.PINLOW(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR(pub u32);
    impl MCR {
        #[inline(always)]
        pub const fn MEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MEN(&mut self, val: bool) {
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
        pub const fn DBGEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RTF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RRF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for MCR {
        #[inline(always)]
        fn default() -> MCR {
            MCR(0)
        }
    }
    impl core::fmt::Debug for MCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR")
                .field("MEN", &self.MEN())
                .field("RST", &self.RST())
                .field("DOZEN", &self.DOZEN())
                .field("DBGEN", &self.DBGEN())
                .field("RTF", &self.RTF())
                .field("RRF", &self.RRF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MCR {
                MEN: bool,
                RST: bool,
                DOZEN: bool,
                DBGEN: bool,
                RTF: bool,
                RRF: bool,
            }
            let proxy = MCR {
                MEN: self.MEN(),
                RST: self.RST(),
                DOZEN: self.DOZEN(),
                DBGEN: self.DBGEN(),
                RTF: self.RTF(),
                RRF: self.RRF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDER(pub u32);
    impl MDER {
        #[inline(always)]
        pub const fn TDDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDDE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MDER {
        #[inline(always)]
        fn default() -> MDER {
            MDER(0)
        }
    }
    impl core::fmt::Debug for MDER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MDER")
                .field("TDDE", &self.TDDE())
                .field("RDDE", &self.RDDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MDER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MDER {
                TDDE: bool,
                RDDE: bool,
            }
            let proxy = MDER {
                TDDE: self.TDDE(),
                RDDE: self.RDDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Data Match"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDMR(pub u32);
    impl MDMR {
        #[inline(always)]
        pub const fn MATCH0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MATCH0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn MATCH1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_MATCH1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for MDMR {
        #[inline(always)]
        fn default() -> MDMR {
            MDMR(0)
        }
    }
    impl core::fmt::Debug for MDMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MDMR")
                .field("MATCH0", &self.MATCH0())
                .field("MATCH1", &self.MATCH1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MDMR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MDMR {
                MATCH0: u8,
                MATCH1: u8,
            }
            let proxy = MDMR {
                MATCH0: self.MATCH0(),
                MATCH1: self.MATCH1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MFCR(pub u32);
    impl MFCR {
        #[inline(always)]
        pub const fn TXWATER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXWATER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn RXWATER(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXWATER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for MFCR {
        #[inline(always)]
        fn default() -> MFCR {
            MFCR(0)
        }
    }
    impl core::fmt::Debug for MFCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MFCR")
                .field("TXWATER", &self.TXWATER())
                .field("RXWATER", &self.RXWATER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MFCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MFCR {
                TXWATER: u8,
                RXWATER: u8,
            }
            let proxy = MFCR {
                TXWATER: self.TXWATER(),
                RXWATER: self.RXWATER(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MFSR(pub u32);
    impl MFSR {
        #[inline(always)]
        pub const fn TXCOUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn RXCOUNT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXCOUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for MFSR {
        #[inline(always)]
        fn default() -> MFSR {
            MFSR(0)
        }
    }
    impl core::fmt::Debug for MFSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MFSR")
                .field("TXCOUNT", &self.TXCOUNT())
                .field("RXCOUNT", &self.RXCOUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MFSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MFSR {
                TXCOUNT: u8,
                RXCOUNT: u8,
            }
            let proxy = MFSR {
                TXCOUNT: self.TXCOUNT(),
                RXCOUNT: self.RXCOUNT(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MIER(pub u32);
    impl MIER {
        #[inline(always)]
        pub const fn TDIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EPIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SDIE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn NDIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ALIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FEIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLTIE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLTIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DMIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn STIE(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for MIER {
        #[inline(always)]
        fn default() -> MIER {
            MIER(0)
        }
    }
    impl core::fmt::Debug for MIER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MIER")
                .field("TDIE", &self.TDIE())
                .field("RDIE", &self.RDIE())
                .field("EPIE", &self.EPIE())
                .field("SDIE", &self.SDIE())
                .field("NDIE", &self.NDIE())
                .field("ALIE", &self.ALIE())
                .field("FEIE", &self.FEIE())
                .field("PLTIE", &self.PLTIE())
                .field("DMIE", &self.DMIE())
                .field("STIE", &self.STIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MIER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MIER {
                TDIE: bool,
                RDIE: bool,
                EPIE: bool,
                SDIE: bool,
                NDIE: bool,
                ALIE: bool,
                FEIE: bool,
                PLTIE: bool,
                DMIE: bool,
                STIE: bool,
            }
            let proxy = MIER {
                TDIE: self.TDIE(),
                RDIE: self.RDIE(),
                EPIE: self.EPIE(),
                SDIE: self.SDIE(),
                NDIE: self.NDIE(),
                ALIE: self.ALIE(),
                FEIE: self.FEIE(),
                PLTIE: self.PLTIE(),
                DMIE: self.DMIE(),
                STIE: self.STIE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Receive Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRDR(pub u32);
    impl MRDR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for MRDR {
        #[inline(always)]
        fn default() -> MRDR {
            MRDR(0)
        }
    }
    impl core::fmt::Debug for MRDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRDR")
                .field("DATA", &self.DATA())
                .field("RXEMPTY", &self.RXEMPTY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRDR {
                DATA: u8,
                RXEMPTY: bool,
            }
            let proxy = MRDR {
                DATA: self.DATA(),
                RXEMPTY: self.RXEMPTY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Receive Data Read Only"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MRDROR(pub u32);
    impl MRDROR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for MRDROR {
        #[inline(always)]
        fn default() -> MRDROR {
            MRDROR(0)
        }
    }
    impl core::fmt::Debug for MRDROR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MRDROR")
                .field("DATA", &self.DATA())
                .field("RXEMPTY", &self.RXEMPTY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MRDROR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MRDROR {
                DATA: u8,
                RXEMPTY: bool,
            }
            let proxy = MRDROR {
                DATA: self.DATA(),
                RXEMPTY: self.RXEMPTY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSR(pub u32);
    impl MSR {
        #[inline(always)]
        pub const fn TDF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn EPF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EPF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SDF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn NDF(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ALF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn FEF(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PLTF(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PLTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn DMF(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DMF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn STF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn MBF(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn BBF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MSR {
        #[inline(always)]
        fn default() -> MSR {
            MSR(0)
        }
    }
    impl core::fmt::Debug for MSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSR")
                .field("TDF", &self.TDF())
                .field("RDF", &self.RDF())
                .field("EPF", &self.EPF())
                .field("SDF", &self.SDF())
                .field("NDF", &self.NDF())
                .field("ALF", &self.ALF())
                .field("FEF", &self.FEF())
                .field("PLTF", &self.PLTF())
                .field("DMF", &self.DMF())
                .field("STF", &self.STF())
                .field("MBF", &self.MBF())
                .field("BBF", &self.BBF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MSR {
                TDF: bool,
                RDF: bool,
                EPF: bool,
                SDF: bool,
                NDF: bool,
                ALF: bool,
                FEF: bool,
                PLTF: bool,
                DMF: bool,
                STF: bool,
                MBF: bool,
                BBF: bool,
            }
            let proxy = MSR {
                TDF: self.TDF(),
                RDF: self.RDF(),
                EPF: self.EPF(),
                SDF: self.SDF(),
                NDF: self.NDF(),
                ALF: self.ALF(),
                FEF: self.FEF(),
                PLTF: self.PLTF(),
                DMF: self.DMF(),
                STF: self.STF(),
                MBF: self.MBF(),
                BBF: self.BBF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Transmit Command Burst"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTCBR(pub u32);
    impl MTCBR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CMD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for MTCBR {
        #[inline(always)]
        fn default() -> MTCBR {
            MTCBR(0)
        }
    }
    impl core::fmt::Debug for MTCBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTCBR")
                .field("DATA", &self.DATA())
                .field("CMD", &self.CMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTCBR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MTCBR {
                DATA: u8,
                CMD: u8,
            }
            let proxy = MTCBR {
                DATA: self.DATA(),
                CMD: self.CMD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Transmit Data Burst"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTDBR(pub u32);
    impl MTDBR {
        #[inline(always)]
        pub const fn DATA0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn DATA1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn DATA2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn DATA3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MTDBR {
        #[inline(always)]
        fn default() -> MTDBR {
            MTDBR(0)
        }
    }
    impl core::fmt::Debug for MTDBR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTDBR")
                .field("DATA0", &self.DATA0())
                .field("DATA1", &self.DATA1())
                .field("DATA2", &self.DATA2())
                .field("DATA3", &self.DATA3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTDBR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MTDBR {
                DATA0: u8,
                DATA1: u8,
                DATA2: u8,
                DATA3: u8,
            }
            let proxy = MTDBR {
                DATA0: self.DATA0(),
                DATA1: self.DATA1(),
                DATA2: self.DATA2(),
                DATA3: self.DATA3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controller Transmit Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTDR(pub u32);
    impl MTDR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn CMD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for MTDR {
        #[inline(always)]
        fn default() -> MTDR {
            MTDR(0)
        }
    }
    impl core::fmt::Debug for MTDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTDR")
                .field("DATA", &self.DATA())
                .field("CMD", &self.CMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MTDR {
                DATA: u8,
                CMD: u8,
            }
            let proxy = MTDR {
                DATA: self.DATA(),
                CMD: self.CMD(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn MTXFIFO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MTXFIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn MRXFIFO(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MRXFIFO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
                .field("MTXFIFO", &self.MTXFIFO())
                .field("MRXFIFO", &self.MRXFIFO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PARAM {
                MTXFIFO: u8,
                MRXFIFO: u8,
            }
            let proxy = PARAM {
                MTXFIFO: self.MTXFIFO(),
                MRXFIFO: self.MRXFIFO(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Address Match"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SAMR(pub u32);
    impl SAMR {
        #[inline(always)]
        pub const fn ADDR0(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ADDR0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
        }
        #[inline(always)]
        pub const fn ADDR1(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ADDR1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 17usize)) | (((val as u32) & 0x03ff) << 17usize);
        }
    }
    impl Default for SAMR {
        #[inline(always)]
        fn default() -> SAMR {
            SAMR(0)
        }
    }
    impl core::fmt::Debug for SAMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SAMR")
                .field("ADDR0", &self.ADDR0())
                .field("ADDR1", &self.ADDR1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SAMR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SAMR {
                ADDR0: u16,
                ADDR1: u16,
            }
            let proxy = SAMR {
                ADDR0: self.ADDR0(),
                ADDR1: self.ADDR1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Address Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SASR(pub u32);
    impl SASR {
        #[inline(always)]
        pub const fn RADDR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RADDR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[inline(always)]
        pub const fn ANV(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ANV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for SASR {
        #[inline(always)]
        fn default() -> SASR {
            SASR(0)
        }
    }
    impl core::fmt::Debug for SASR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SASR")
                .field("RADDR", &self.RADDR())
                .field("ANV", &self.ANV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SASR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SASR {
                RADDR: u16,
                ANV: bool,
            }
            let proxy = SASR {
                RADDR: self.RADDR(),
                ANV: self.ANV(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Configuration 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCFGR0(pub u32);
    impl SCFGR0 {
        #[inline(always)]
        pub const fn RDREQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDACK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SCFGR0 {
        #[inline(always)]
        fn default() -> SCFGR0 {
            SCFGR0(0)
        }
    }
    impl core::fmt::Debug for SCFGR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCFGR0")
                .field("RDREQ", &self.RDREQ())
                .field("RDACK", &self.RDACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCFGR0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCFGR0 {
                RDREQ: bool,
                RDACK: bool,
            }
            let proxy = SCFGR0 {
                RDREQ: self.RDREQ(),
                RDACK: self.RDACK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Configuration 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCFGR1(pub u32);
    impl SCFGR1 {
        #[inline(always)]
        pub const fn ADRSTALL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADRSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RXSTALL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TXDSTALL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXDSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ACKSTALL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACKSTALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RXNACK(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXNACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GCEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SAEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn TXCFG(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn RXCFG(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IGNACK(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IGNACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn HSMEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ADDRCFG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ADDRCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn RXALL(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn RSCFG(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SDCFG(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDCFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for SCFGR1 {
        #[inline(always)]
        fn default() -> SCFGR1 {
            SCFGR1(0)
        }
    }
    impl core::fmt::Debug for SCFGR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCFGR1")
                .field("ADRSTALL", &self.ADRSTALL())
                .field("RXSTALL", &self.RXSTALL())
                .field("TXDSTALL", &self.TXDSTALL())
                .field("ACKSTALL", &self.ACKSTALL())
                .field("RXNACK", &self.RXNACK())
                .field("GCEN", &self.GCEN())
                .field("SAEN", &self.SAEN())
                .field("TXCFG", &self.TXCFG())
                .field("RXCFG", &self.RXCFG())
                .field("IGNACK", &self.IGNACK())
                .field("HSMEN", &self.HSMEN())
                .field("ADDRCFG", &self.ADDRCFG())
                .field("RXALL", &self.RXALL())
                .field("RSCFG", &self.RSCFG())
                .field("SDCFG", &self.SDCFG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCFGR1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCFGR1 {
                ADRSTALL: bool,
                RXSTALL: bool,
                TXDSTALL: bool,
                ACKSTALL: bool,
                RXNACK: bool,
                GCEN: bool,
                SAEN: bool,
                TXCFG: bool,
                RXCFG: bool,
                IGNACK: bool,
                HSMEN: bool,
                ADDRCFG: u8,
                RXALL: bool,
                RSCFG: bool,
                SDCFG: bool,
            }
            let proxy = SCFGR1 {
                ADRSTALL: self.ADRSTALL(),
                RXSTALL: self.RXSTALL(),
                TXDSTALL: self.TXDSTALL(),
                ACKSTALL: self.ACKSTALL(),
                RXNACK: self.RXNACK(),
                GCEN: self.GCEN(),
                SAEN: self.SAEN(),
                TXCFG: self.TXCFG(),
                RXCFG: self.RXCFG(),
                IGNACK: self.IGNACK(),
                HSMEN: self.HSMEN(),
                ADDRCFG: self.ADDRCFG(),
                RXALL: self.RXALL(),
                RSCFG: self.RSCFG(),
                SDCFG: self.SDCFG(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Configuration 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCFGR2(pub u32);
    impl SCFGR2 {
        #[inline(always)]
        pub const fn CLKHOLD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKHOLD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn DATAVD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATAVD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn FILTSCL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSCL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn FILTSDA(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILTSDA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for SCFGR2 {
        #[inline(always)]
        fn default() -> SCFGR2 {
            SCFGR2(0)
        }
    }
    impl core::fmt::Debug for SCFGR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCFGR2")
                .field("CLKHOLD", &self.CLKHOLD())
                .field("DATAVD", &self.DATAVD())
                .field("FILTSCL", &self.FILTSCL())
                .field("FILTSDA", &self.FILTSDA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCFGR2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCFGR2 {
                CLKHOLD: u8,
                DATAVD: u8,
                FILTSCL: u8,
                FILTSDA: u8,
            }
            let proxy = SCFGR2 {
                CLKHOLD: self.CLKHOLD(),
                DATAVD: self.DATAVD(),
                FILTSCL: self.FILTSCL(),
                FILTSDA: self.FILTSDA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SCR(pub u32);
    impl SCR {
        #[inline(always)]
        pub const fn SEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEN(&mut self, val: bool) {
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
        pub const fn FILTEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FILTDZ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FILTDZ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn RTF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RTF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RRF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RRF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for SCR {
        #[inline(always)]
        fn default() -> SCR {
            SCR(0)
        }
    }
    impl core::fmt::Debug for SCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SCR")
                .field("SEN", &self.SEN())
                .field("RST", &self.RST())
                .field("FILTEN", &self.FILTEN())
                .field("FILTDZ", &self.FILTDZ())
                .field("RTF", &self.RTF())
                .field("RRF", &self.RRF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SCR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SCR {
                SEN: bool,
                RST: bool,
                FILTEN: bool,
                FILTDZ: bool,
                RTF: bool,
                RRF: bool,
            }
            let proxy = SCR {
                SEN: self.SEN(),
                RST: self.RST(),
                FILTEN: self.FILTEN(),
                FILTDZ: self.FILTDZ(),
                RTF: self.RTF(),
                RRF: self.RRF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SDER(pub u32);
    impl SDER {
        #[inline(always)]
        pub const fn TDDE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDDE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVDE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RSDE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SDDE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for SDER {
        #[inline(always)]
        fn default() -> SDER {
            SDER(0)
        }
    }
    impl core::fmt::Debug for SDER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SDER")
                .field("TDDE", &self.TDDE())
                .field("RDDE", &self.RDDE())
                .field("AVDE", &self.AVDE())
                .field("RSDE", &self.RSDE())
                .field("SDDE", &self.SDDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SDER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SDER {
                TDDE: bool,
                RDDE: bool,
                AVDE: bool,
                RSDE: bool,
                SDDE: bool,
            }
            let proxy = SDER {
                TDDE: self.TDDE(),
                RDDE: self.RDDE(),
                AVDE: self.AVDE(),
                RSDE: self.RSDE(),
                SDDE: self.SDDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIER(pub u32);
    impl SIER {
        #[inline(always)]
        pub const fn TDIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDIE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVIE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TAIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TAIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RSIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SDIE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn BEIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FEIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AM0IE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AM0IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn AM1IE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AM1IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn GCIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SARIE(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SARIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SIER {
        #[inline(always)]
        fn default() -> SIER {
            SIER(0)
        }
    }
    impl core::fmt::Debug for SIER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIER")
                .field("TDIE", &self.TDIE())
                .field("RDIE", &self.RDIE())
                .field("AVIE", &self.AVIE())
                .field("TAIE", &self.TAIE())
                .field("RSIE", &self.RSIE())
                .field("SDIE", &self.SDIE())
                .field("BEIE", &self.BEIE())
                .field("FEIE", &self.FEIE())
                .field("AM0IE", &self.AM0IE())
                .field("AM1IE", &self.AM1IE())
                .field("GCIE", &self.GCIE())
                .field("SARIE", &self.SARIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SIER {
                TDIE: bool,
                RDIE: bool,
                AVIE: bool,
                TAIE: bool,
                RSIE: bool,
                SDIE: bool,
                BEIE: bool,
                FEIE: bool,
                AM0IE: bool,
                AM1IE: bool,
                GCIE: bool,
                SARIE: bool,
            }
            let proxy = SIER {
                TDIE: self.TDIE(),
                RDIE: self.RDIE(),
                AVIE: self.AVIE(),
                TAIE: self.TAIE(),
                RSIE: self.RSIE(),
                SDIE: self.SDIE(),
                BEIE: self.BEIE(),
                FEIE: self.FEIE(),
                AM0IE: self.AM0IE(),
                AM1IE: self.AM1IE(),
                GCIE: self.GCIE(),
                SARIE: self.SARIE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Receive Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRDR(pub u32);
    impl SRDR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RADDR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SOF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SRDR {
        #[inline(always)]
        fn default() -> SRDR {
            SRDR(0)
        }
    }
    impl core::fmt::Debug for SRDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRDR")
                .field("DATA", &self.DATA())
                .field("RADDR", &self.RADDR())
                .field("RXEMPTY", &self.RXEMPTY())
                .field("SOF", &self.SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRDR {
                DATA: u8,
                RADDR: u8,
                RXEMPTY: bool,
                SOF: bool,
            }
            let proxy = SRDR {
                DATA: self.DATA(),
                RADDR: self.RADDR(),
                RXEMPTY: self.RXEMPTY(),
                SOF: self.SOF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Receive Data Read Only"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRDROR(pub u32);
    impl SRDROR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RADDR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_RADDR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn RXEMPTY(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXEMPTY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SOF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SRDROR {
        #[inline(always)]
        fn default() -> SRDROR {
            SRDROR(0)
        }
    }
    impl core::fmt::Debug for SRDROR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRDROR")
                .field("DATA", &self.DATA())
                .field("RADDR", &self.RADDR())
                .field("RXEMPTY", &self.RXEMPTY())
                .field("SOF", &self.SOF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRDROR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRDROR {
                DATA: u8,
                RADDR: u8,
                RXEMPTY: bool,
                SOF: bool,
            }
            let proxy = SRDROR {
                DATA: self.DATA(),
                RADDR: self.RADDR(),
                RXEMPTY: self.RXEMPTY(),
                SOF: self.SOF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSR(pub u32);
    impl SSR {
        #[inline(always)]
        pub const fn TDF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RDF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AVF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TAF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TAF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RSF(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SDF(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SDF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn BEF(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn FEF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn AM0F(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AM0F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn AM1F(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AM1F(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn GCF(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SARF(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SARF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn SBF(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn BBF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for SSR {
        #[inline(always)]
        fn default() -> SSR {
            SSR(0)
        }
    }
    impl core::fmt::Debug for SSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSR")
                .field("TDF", &self.TDF())
                .field("RDF", &self.RDF())
                .field("AVF", &self.AVF())
                .field("TAF", &self.TAF())
                .field("RSF", &self.RSF())
                .field("SDF", &self.SDF())
                .field("BEF", &self.BEF())
                .field("FEF", &self.FEF())
                .field("AM0F", &self.AM0F())
                .field("AM1F", &self.AM1F())
                .field("GCF", &self.GCF())
                .field("SARF", &self.SARF())
                .field("SBF", &self.SBF())
                .field("BBF", &self.BBF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SSR {
                TDF: bool,
                RDF: bool,
                AVF: bool,
                TAF: bool,
                RSF: bool,
                SDF: bool,
                BEF: bool,
                FEF: bool,
                AM0F: bool,
                AM1F: bool,
                GCF: bool,
                SARF: bool,
                SBF: bool,
                BBF: bool,
            }
            let proxy = SSR {
                TDF: self.TDF(),
                RDF: self.RDF(),
                AVF: self.AVF(),
                TAF: self.TAF(),
                RSF: self.RSF(),
                SDF: self.SDF(),
                BEF: self.BEF(),
                FEF: self.FEF(),
                AM0F: self.AM0F(),
                AM1F: self.AM1F(),
                GCF: self.GCF(),
                SARF: self.SARF(),
                SBF: self.SBF(),
                BBF: self.BBF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Transmit ACK"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STAR(pub u32);
    impl STAR {
        #[inline(always)]
        pub const fn TXNACK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXNACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for STAR {
        #[inline(always)]
        fn default() -> STAR {
            STAR(0)
        }
    }
    impl core::fmt::Debug for STAR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STAR")
                .field("TXNACK", &self.TXNACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STAR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STAR {
                TXNACK: bool,
            }
            let proxy = STAR {
                TXNACK: self.TXNACK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Target Transmit Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STDR(pub u32);
    impl STDR {
        #[inline(always)]
        pub const fn DATA(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for STDR {
        #[inline(always)]
        fn default() -> STDR {
            STDR(0)
        }
    }
    impl core::fmt::Debug for STDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STDR").field("DATA", &self.DATA()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STDR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct STDR {
                DATA: u8,
            }
            let proxy = STDR { DATA: self.DATA() };
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
