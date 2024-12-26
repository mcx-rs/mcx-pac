#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn PIN(self) -> crate::common::Reg<regs::PIN, crate::common::RW> {
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
    pub const fn PINSTAT(self) -> crate::common::Reg<regs::PINSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PINIEN(self) -> crate::common::Reg<regs::PINIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn PINREN(self) -> crate::common::Reg<regs::PINREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn PINFEN(self) -> crate::common::Reg<regs::PINFEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTD(self) -> crate::common::Reg<regs::PINOUTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTE(self) -> crate::common::Reg<regs::PINOUTE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTDIS(self) -> crate::common::Reg<regs::PINOUTDIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTCLR(self) -> crate::common::Reg<regs::PINOUTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTSET(self) -> crate::common::Reg<regs::PINOUTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTTOG(self) -> crate::common::Reg<regs::PINOUTTOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCTL(self, n: usize) -> crate::common::Reg<regs::SHIFTCTL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCFG(self, n: usize) -> crate::common::Reg<regs::SHIFTCFG, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUF(self, n: usize) -> crate::common::Reg<regs::SHIFTBUF, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBIS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFBIS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBYS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFBYS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBBS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFBBS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCTL(self, n: usize) -> crate::common::Reg<regs::TIMCTL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCFG(self, n: usize) -> crate::common::Reg<regs::TIMCFG, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMCMP(self, n: usize) -> crate::common::Reg<regs::TIMCMP, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNBS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFNBS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHWS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFHWS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNIS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFNIS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFOES(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFOES, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFEOS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFEOS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHBS(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHIFTBUFHBS, crate::common::RW> {
        assert!(n < 8usize);
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
    #[doc = "Pin State"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PIN(pub u32);
    impl PIN {
        #[inline(always)]
        pub const fn PDI(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PDI(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PIN {
        #[inline(always)]
        fn default() -> PIN {
            PIN(0)
        }
    }
    #[doc = "Pin Falling Edge Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINFEN(pub u32);
    impl PINFEN {
        #[inline(always)]
        pub const fn PFE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PFE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINFEN {
        #[inline(always)]
        fn default() -> PINFEN {
            PINFEN(0)
        }
    }
    #[doc = "Pin Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINIEN(pub u32);
    impl PINIEN {
        #[inline(always)]
        pub const fn PSIE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PSIE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINIEN {
        #[inline(always)]
        fn default() -> PINIEN {
            PINIEN(0)
        }
    }
    #[doc = "Pin Output Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTCLR(pub u32);
    impl PINOUTCLR {
        #[inline(always)]
        pub const fn OUTCLR(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTCLR(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTCLR {
        #[inline(always)]
        fn default() -> PINOUTCLR {
            PINOUTCLR(0)
        }
    }
    #[doc = "Pin Output Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTD(pub u32);
    impl PINOUTD {
        #[inline(always)]
        pub const fn OUTD(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTD(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTD {
        #[inline(always)]
        fn default() -> PINOUTD {
            PINOUTD(0)
        }
    }
    #[doc = "Pin Output Disable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTDIS(pub u32);
    impl PINOUTDIS {
        #[inline(always)]
        pub const fn OUTDIS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTDIS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTDIS {
        #[inline(always)]
        fn default() -> PINOUTDIS {
            PINOUTDIS(0)
        }
    }
    #[doc = "Pin Output Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTE(pub u32);
    impl PINOUTE {
        #[inline(always)]
        pub const fn OUTE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTE {
        #[inline(always)]
        fn default() -> PINOUTE {
            PINOUTE(0)
        }
    }
    #[doc = "Pin Output Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTSET(pub u32);
    impl PINOUTSET {
        #[inline(always)]
        pub const fn OUTSET(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTSET(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTSET {
        #[inline(always)]
        fn default() -> PINOUTSET {
            PINOUTSET(0)
        }
    }
    #[doc = "Pin Output Toggle"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINOUTTOG(pub u32);
    impl PINOUTTOG {
        #[inline(always)]
        pub const fn OUTTOG(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_OUTTOG(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINOUTTOG {
        #[inline(always)]
        fn default() -> PINOUTTOG {
            PINOUTTOG(0)
        }
    }
    #[doc = "Pin Rising Edge Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINREN(pub u32);
    impl PINREN {
        #[inline(always)]
        pub const fn PRE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PRE(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINREN {
        #[inline(always)]
        fn default() -> PINREN {
            PINREN(0)
        }
    }
    #[doc = "Pin Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PINSTAT(pub u32);
    impl PINSTAT {
        #[inline(always)]
        pub const fn PSF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_PSF(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PINSTAT {
        #[inline(always)]
        fn default() -> PINSTAT {
            PINSTAT(0)
        }
    }
    #[doc = "Shifter Buffer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUF(pub u32);
    impl SHIFTBUF {
        #[inline(always)]
        pub const fn SHIFTBUF(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUF(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUF {
        #[inline(always)]
        fn default() -> SHIFTBUF {
            SHIFTBUF(0)
        }
    }
    #[doc = "Shifter Buffer Bit Byte Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFBBS(pub u32);
    impl SHIFTBUFBBS {
        #[inline(always)]
        pub const fn SHIFTBUFBBS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFBBS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFBBS {
        #[inline(always)]
        fn default() -> SHIFTBUFBBS {
            SHIFTBUFBBS(0)
        }
    }
    #[doc = "Shifter Buffer Bit Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFBIS(pub u32);
    impl SHIFTBUFBIS {
        #[inline(always)]
        pub const fn SHIFTBUFBIS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFBIS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFBIS {
        #[inline(always)]
        fn default() -> SHIFTBUFBIS {
            SHIFTBUFBIS(0)
        }
    }
    #[doc = "Shifter Buffer Byte Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFBYS(pub u32);
    impl SHIFTBUFBYS {
        #[inline(always)]
        pub const fn SHIFTBUFBYS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFBYS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFBYS {
        #[inline(always)]
        fn default() -> SHIFTBUFBYS {
            SHIFTBUFBYS(0)
        }
    }
    #[doc = "Shifter Buffer Even Odd Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFEOS(pub u32);
    impl SHIFTBUFEOS {
        #[inline(always)]
        pub const fn SHIFTBUFEOS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFEOS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFEOS {
        #[inline(always)]
        fn default() -> SHIFTBUFEOS {
            SHIFTBUFEOS(0)
        }
    }
    #[doc = "Shifter Buffer Halfword Byte Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFHBS(pub u32);
    impl SHIFTBUFHBS {
        #[inline(always)]
        pub const fn SHIFTBUFHBS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFHBS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFHBS {
        #[inline(always)]
        fn default() -> SHIFTBUFHBS {
            SHIFTBUFHBS(0)
        }
    }
    #[doc = "Shifter Buffer Halfword Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFHWS(pub u32);
    impl SHIFTBUFHWS {
        #[inline(always)]
        pub const fn SHIFTBUFHWS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFHWS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFHWS {
        #[inline(always)]
        fn default() -> SHIFTBUFHWS {
            SHIFTBUFHWS(0)
        }
    }
    #[doc = "Shifter Buffer Nibble Byte Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFNBS(pub u32);
    impl SHIFTBUFNBS {
        #[inline(always)]
        pub const fn SHIFTBUFNBS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFNBS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFNBS {
        #[inline(always)]
        fn default() -> SHIFTBUFNBS {
            SHIFTBUFNBS(0)
        }
    }
    #[doc = "Shifter Buffer Nibble Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFNIS(pub u32);
    impl SHIFTBUFNIS {
        #[inline(always)]
        pub const fn SHIFTBUFNIS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFNIS(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFNIS {
        #[inline(always)]
        fn default() -> SHIFTBUFNIS {
            SHIFTBUFNIS(0)
        }
    }
    #[doc = "Shifter Buffer Odd Even Swapped"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTBUFOES(pub u32);
    impl SHIFTBUFOES {
        #[inline(always)]
        pub const fn SHIFTBUFOES(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SHIFTBUFOES(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SHIFTBUFOES {
        #[inline(always)]
        fn default() -> SHIFTBUFOES {
            SHIFTBUFOES(0)
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
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for SHIFTCTL {
        #[inline(always)]
        fn default() -> SHIFTCTL {
            SHIFTCTL(0)
        }
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTEIEN(pub u32);
    impl SHIFTEIEN {
        #[inline(always)]
        pub const fn SEIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SHIFTEIEN {
        #[inline(always)]
        fn default() -> SHIFTEIEN {
            SHIFTEIEN(0)
        }
    }
    #[doc = "Shifter Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTERR(pub u32);
    impl SHIFTERR {
        #[inline(always)]
        pub const fn SEF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SHIFTERR {
        #[inline(always)]
        fn default() -> SHIFTERR {
            SHIFTERR(0)
        }
    }
    #[doc = "Shifter Status DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSDEN(pub u32);
    impl SHIFTSDEN {
        #[inline(always)]
        pub const fn SSDE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SHIFTSDEN {
        #[inline(always)]
        fn default() -> SHIFTSDEN {
            SHIFTSDEN(0)
        }
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSIEN(pub u32);
    impl SHIFTSIEN {
        #[inline(always)]
        pub const fn SSIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SHIFTSIEN {
        #[inline(always)]
        fn default() -> SHIFTSIEN {
            SHIFTSIEN(0)
        }
    }
    #[doc = "Shifter Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SHIFTSTAT(pub u32);
    impl SHIFTSTAT {
        #[inline(always)]
        pub const fn SSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_SSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SHIFTSTAT {
        #[inline(always)]
        fn default() -> SHIFTSTAT {
            SHIFTSTAT(0)
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
    #[doc = "Timer Status DMA Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMERSDEN(pub u32);
    impl TIMERSDEN {
        #[inline(always)]
        pub const fn TSDE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSDE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TIMERSDEN {
        #[inline(always)]
        fn default() -> TIMERSDEN {
            TIMERSDEN(0)
        }
    }
    #[doc = "Timer Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMIEN(pub u32);
    impl TIMIEN {
        #[inline(always)]
        pub const fn TEIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TEIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TIMIEN {
        #[inline(always)]
        fn default() -> TIMIEN {
            TIMIEN(0)
        }
    }
    #[doc = "Timer Status Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TIMSTAT(pub u32);
    impl TIMSTAT {
        #[inline(always)]
        pub const fn TSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TIMSTAT {
        #[inline(always)]
        fn default() -> TIMSTAT {
            TIMSTAT(0)
        }
    }
    #[doc = "Trigger Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRGSTAT(pub u32);
    impl TRGSTAT {
        #[inline(always)]
        pub const fn ETSF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ETSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TRGSTAT {
        #[inline(always)]
        fn default() -> TRGSTAT {
            TRGSTAT(0)
        }
    }
    #[doc = "External Trigger Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIGIEN(pub u32);
    impl TRIGIEN {
        #[inline(always)]
        pub const fn TRIE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TRIGIEN {
        #[inline(always)]
        fn default() -> TRIGIEN {
            TRIGIEN(0)
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
}