#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO {
    ptr: *mut u8,
}
unsafe impl Send for FLEXIO {}
unsafe impl Sync for FLEXIO {}
impl FLEXIO {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PIN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSTAT(self) -> crate::common::Reg<regs::SHIFTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTERR(self) -> crate::common::Reg<regs::SHIFTERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMSTAT(self) -> crate::common::Reg<regs::TIMSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSIEN(self) -> crate::common::Reg<regs::SHIFTSIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTEIEN(self) -> crate::common::Reg<regs::SHIFTEIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMIEN(self) -> crate::common::Reg<regs::TIMIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSDEN(self) -> crate::common::Reg<regs::SHIFTSDEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMERSDEN(self) -> crate::common::Reg<regs::TIMERSDEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSTATE(self) -> crate::common::Reg<regs::SHIFTSTATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn TRGSTAT(self) -> crate::common::Reg<regs::TRGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIGIEN(self) -> crate::common::Reg<regs::TRIGIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINSTAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PINIEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn PINREN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn PINFEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTDIS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTCLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTSET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTTOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCTL(self, n: usize) -> crate::common::Reg<regs::SHIFTCTL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCFG(self, n: usize) -> crate::common::Reg<regs::SHIFTCFG, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUF(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBIS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBYS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBBS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCTL(self, n: usize) -> crate::common::Reg<regs::TIMCTL, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCFG(self, n: usize) -> crate::common::Reg<regs::TIMCFG, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCMP(self, n: usize) -> crate::common::Reg<regs::TIMCMP, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNBS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHWS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNIS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFOES(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFEOS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHBS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "FLEXIO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn FLEXEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLEXEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SWRST(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FASTACC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FASTACC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DBGE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn DOZEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("FLEXEN", &self.FLEXEN())
                .field("SWRST", &self.SWRST())
                .field("FASTACC", &self.FASTACC())
                .field("DBGE", &self.DBGE())
                .field("DOZEN", &self.DOZEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ FLEXEN: {=bool:?}, SWRST: {=bool:?}, FASTACC: {=bool:?}, DBGE: {=bool:?}, DOZEN: {=bool:?} }}" , self . FLEXEN () , self . SWRST () , self . FASTACC () , self . DBGE () , self . DOZEN ())
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn SHIFTER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SHIFTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TIMER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn PIN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_PIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn TRIGGER(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIGGER(&mut self, val: u8) {
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
                .field("SHIFTER", &self.SHIFTER())
                .field("TIMER", &self.TIMER())
                .field("PIN", &self.PIN())
                .field("TRIGGER", &self.TRIGGER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PARAM {{ SHIFTER: {=u8:?}, TIMER: {=u8:?}, PIN: {=u8:?}, TRIGGER: {=u8:?} }}",
                self.SHIFTER(),
                self.TIMER(),
                self.PIN(),
                self.TRIGGER()
            )
        }
    }
    #[doc = "Shifter Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTCFG(pub u32);
    impl SHIFTCFG {
        #[inline(always)]
        pub const fn SSTART(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSTART(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn SSTOP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSTOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn INSRC(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn LATST(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LATST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn SSIZE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SSIZE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn PWIDTH(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PWIDTH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for SHIFTCFG {
        #[inline(always)]
        fn default() -> SHIFTCFG {
            SHIFTCFG(0)
        }
    }
    impl core::fmt::Debug for SHIFTCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTCFG")
                .field("SSTART", &self.SSTART())
                .field("SSTOP", &self.SSTOP())
                .field("INSRC", &self.INSRC())
                .field("LATST", &self.LATST())
                .field("SSIZE", &self.SSIZE())
                .field("PWIDTH", &self.PWIDTH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SHIFTCFG {{ SSTART: {=u8:?}, SSTOP: {=u8:?}, INSRC: {=bool:?}, LATST: {=bool:?}, SSIZE: {=bool:?}, PWIDTH: {=u8:?} }}" , self . SSTART () , self . SSTOP () , self . INSRC () , self . LATST () , self . SSIZE () , self . PWIDTH ())
        }
    }
    #[doc = "Shifter Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTCTL(pub u32);
    impl SHIFTCTL {
        #[inline(always)]
        pub const fn SMOD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn PINPOL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PINPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn PINSEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn PINCFG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn TIMPOL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn TIMSEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for SHIFTCTL {
        #[inline(always)]
        fn default() -> SHIFTCTL {
            SHIFTCTL(0)
        }
    }
    impl core::fmt::Debug for SHIFTCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTCTL")
                .field("SMOD", &self.SMOD())
                .field("PINPOL", &self.PINPOL())
                .field("PINSEL", &self.PINSEL())
                .field("PINCFG", &self.PINCFG())
                .field("TIMPOL", &self.TIMPOL())
                .field("TIMSEL", &self.TIMSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SHIFTCTL {{ SMOD: {=u8:?}, PINPOL: {=bool:?}, PINSEL: {=u8:?}, PINCFG: {=u8:?}, TIMPOL: {=bool:?}, TIMSEL: {=u8:?} }}" , self . SMOD () , self . PINPOL () , self . PINSEL () , self . PINCFG () , self . TIMPOL () , self . TIMSEL ())
        }
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTEIEN(pub u32);
    impl SHIFTEIEN {
        #[inline(always)]
        pub const fn SEIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for SHIFTEIEN {
        #[inline(always)]
        fn default() -> SHIFTEIEN {
            SHIFTEIEN(0)
        }
    }
    impl core::fmt::Debug for SHIFTEIEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTEIEN")
                .field("SEIE", &self.SEIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTEIEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTEIEN {{ SEIE: {=u8:?} }}", self.SEIE())
        }
    }
    #[doc = "Shifter Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTERR(pub u32);
    impl SHIFTERR {
        #[inline(always)]
        pub const fn SEF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for SHIFTERR {
        #[inline(always)]
        fn default() -> SHIFTERR {
            SHIFTERR(0)
        }
    }
    impl core::fmt::Debug for SHIFTERR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTERR")
                .field("SEF", &self.SEF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTERR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTERR {{ SEF: {=u8:?} }}", self.SEF())
        }
    }
    #[doc = "Shifter Status DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSDEN(pub u32);
    impl SHIFTSDEN {
        #[inline(always)]
        pub const fn SSDE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for SHIFTSDEN {
        #[inline(always)]
        fn default() -> SHIFTSDEN {
            SHIFTSDEN(0)
        }
    }
    impl core::fmt::Debug for SHIFTSDEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTSDEN")
                .field("SSDE", &self.SSDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTSDEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTSDEN {{ SSDE: {=u8:?} }}", self.SSDE())
        }
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSIEN(pub u32);
    impl SHIFTSIEN {
        #[inline(always)]
        pub const fn SSIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for SHIFTSIEN {
        #[inline(always)]
        fn default() -> SHIFTSIEN {
            SHIFTSIEN(0)
        }
    }
    impl core::fmt::Debug for SHIFTSIEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTSIEN")
                .field("SSIE", &self.SSIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTSIEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTSIEN {{ SSIE: {=u8:?} }}", self.SSIE())
        }
    }
    #[doc = "Shifter Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSTAT(pub u32);
    impl SHIFTSTAT {
        #[inline(always)]
        pub const fn SSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for SHIFTSTAT {
        #[inline(always)]
        fn default() -> SHIFTSTAT {
            SHIFTSTAT(0)
        }
    }
    impl core::fmt::Debug for SHIFTSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTSTAT")
                .field("SSF", &self.SSF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTSTAT {{ SSF: {=u8:?} }}", self.SSF())
        }
    }
    #[doc = "Shifter State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSTATE(pub u32);
    impl SHIFTSTATE {
        #[inline(always)]
        pub const fn STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for SHIFTSTATE {
        #[inline(always)]
        fn default() -> SHIFTSTATE {
            SHIFTSTATE(0)
        }
    }
    impl core::fmt::Debug for SHIFTSTATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SHIFTSTATE")
                .field("STATE", &self.STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SHIFTSTATE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SHIFTSTATE {{ STATE: {=u8:?} }}", self.STATE())
        }
    }
    #[doc = "Timer Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMCFG(pub u32);
    impl TIMCFG {
        #[inline(always)]
        pub const fn TSTART(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TSTART(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TSTOP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSTOP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn TIMENA(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMENA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn TIMDIS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMDIS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[inline(always)]
        pub const fn TIMRST(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMRST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[inline(always)]
        pub const fn TIMDEC(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMDEC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[inline(always)]
        pub const fn TIMOUT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMOUT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for TIMCFG {
        #[inline(always)]
        fn default() -> TIMCFG {
            TIMCFG(0)
        }
    }
    impl core::fmt::Debug for TIMCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMCFG")
                .field("TSTART", &self.TSTART())
                .field("TSTOP", &self.TSTOP())
                .field("TIMENA", &self.TIMENA())
                .field("TIMDIS", &self.TIMDIS())
                .field("TIMRST", &self.TIMRST())
                .field("TIMDEC", &self.TIMDEC())
                .field("TIMOUT", &self.TIMOUT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TIMCFG {{ TSTART: {=bool:?}, TSTOP: {=u8:?}, TIMENA: {=u8:?}, TIMDIS: {=u8:?}, TIMRST: {=u8:?}, TIMDEC: {=u8:?}, TIMOUT: {=u8:?} }}" , self . TSTART () , self . TSTOP () , self . TIMENA () , self . TIMDIS () , self . TIMRST () , self . TIMDEC () , self . TIMOUT ())
        }
    }
    #[doc = "Timer Compare"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMCMP(pub u32);
    impl TIMCMP {
        #[inline(always)]
        pub const fn CMP(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CMP(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TIMCMP {
        #[inline(always)]
        fn default() -> TIMCMP {
            TIMCMP(0)
        }
    }
    impl core::fmt::Debug for TIMCMP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMCMP").field("CMP", &self.CMP()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMCMP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TIMCMP {{ CMP: {=u16:?} }}", self.CMP())
        }
    }
    #[doc = "Timer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMCTL(pub u32);
    impl TIMCTL {
        #[inline(always)]
        pub const fn TIMOD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMOD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn ONETIM(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ONETIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn PININS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PININS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn PINPOL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PINPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn PINSEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[inline(always)]
        pub const fn PINCFG(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PINCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn TRGSRC(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRGSRC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn TRGPOL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRGPOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn TRGSEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRGSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for TIMCTL {
        #[inline(always)]
        fn default() -> TIMCTL {
            TIMCTL(0)
        }
    }
    impl core::fmt::Debug for TIMCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMCTL")
                .field("TIMOD", &self.TIMOD())
                .field("ONETIM", &self.ONETIM())
                .field("PININS", &self.PININS())
                .field("PINPOL", &self.PINPOL())
                .field("PINSEL", &self.PINSEL())
                .field("PINCFG", &self.PINCFG())
                .field("TRGSRC", &self.TRGSRC())
                .field("TRGPOL", &self.TRGPOL())
                .field("TRGSEL", &self.TRGSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TIMCTL {{ TIMOD: {=u8:?}, ONETIM: {=bool:?}, PININS: {=bool:?}, PINPOL: {=bool:?}, PINSEL: {=u8:?}, PINCFG: {=u8:?}, TRGSRC: {=bool:?}, TRGPOL: {=bool:?}, TRGSEL: {=u8:?} }}" , self . TIMOD () , self . ONETIM () , self . PININS () , self . PINPOL () , self . PINSEL () , self . PINCFG () , self . TRGSRC () , self . TRGPOL () , self . TRGSEL ())
        }
    }
    #[doc = "Timer Status DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMERSDEN(pub u32);
    impl TIMERSDEN {
        #[inline(always)]
        pub const fn TSDE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TIMERSDEN {
        #[inline(always)]
        fn default() -> TIMERSDEN {
            TIMERSDEN(0)
        }
    }
    impl core::fmt::Debug for TIMERSDEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMERSDEN")
                .field("TSDE", &self.TSDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMERSDEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TIMERSDEN {{ TSDE: {=u8:?} }}", self.TSDE())
        }
    }
    #[doc = "Timer Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMIEN(pub u32);
    impl TIMIEN {
        #[inline(always)]
        pub const fn TEIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TIMIEN {
        #[inline(always)]
        fn default() -> TIMIEN {
            TIMIEN(0)
        }
    }
    impl core::fmt::Debug for TIMIEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMIEN")
                .field("TEIE", &self.TEIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMIEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TIMIEN {{ TEIE: {=u8:?} }}", self.TEIE())
        }
    }
    #[doc = "Timer Status Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMSTAT(pub u32);
    impl TIMSTAT {
        #[inline(always)]
        pub const fn TSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TIMSTAT {
        #[inline(always)]
        fn default() -> TIMSTAT {
            TIMSTAT(0)
        }
    }
    impl core::fmt::Debug for TIMSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TIMSTAT").field("TSF", &self.TSF()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TIMSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TIMSTAT {{ TSF: {=u8:?} }}", self.TSF())
        }
    }
    #[doc = "Trigger Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRGSTAT(pub u32);
    impl TRGSTAT {
        #[inline(always)]
        pub const fn ETSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TRGSTAT {
        #[inline(always)]
        fn default() -> TRGSTAT {
            TRGSTAT(0)
        }
    }
    impl core::fmt::Debug for TRGSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRGSTAT")
                .field("ETSF", &self.ETSF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRGSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TRGSTAT {{ ETSF: {=u8:?} }}", self.ETSF())
        }
    }
    #[doc = "External Trigger Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIGIEN(pub u32);
    impl TRIGIEN {
        #[inline(always)]
        pub const fn TRIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TRIGIEN {
        #[inline(always)]
        fn default() -> TRIGIEN {
            TRIGIEN(0)
        }
    }
    impl core::fmt::Debug for TRIGIEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIGIEN")
                .field("TRIE", &self.TRIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIGIEN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TRIGIEN {{ TRIE: {=u8:?} }}", self.TRIE())
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
