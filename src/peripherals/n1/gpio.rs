#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO {
    ptr: *mut u8,
}
unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}
impl GPIO {
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
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn PCNS(self) -> crate::common::Reg<regs::PCNS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ICNS(self) -> crate::common::Reg<regs::ICNS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn PCNP(self) -> crate::common::Reg<regs::PCNP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn ICNP(self) -> crate::common::Reg<regs::ICNP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PDOR(self) -> crate::common::Reg<regs::PDOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn PSOR(self) -> crate::common::Reg<regs::PSOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn PCOR(self) -> crate::common::Reg<regs::PCOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn PTOR(self) -> crate::common::Reg<regs::PTOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn PDIR(self) -> crate::common::Reg<regs::PDIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PDDR(self) -> crate::common::Reg<regs::PDDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn PIDR(self) -> crate::common::Reg<regs::PIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn PDR(self, n: usize) -> crate::common::Reg<regs::PDR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 1usize) as _) }
    }
    #[inline(always)]
    pub const fn ICR(self, n: usize) -> crate::common::Reg<regs::ICR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn GICLR(self) -> crate::common::Reg<regs::GICLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn GICHR(self) -> crate::common::Reg<regs::GICHR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn ISFR(self, n: usize) -> crate::common::Reg<regs::ISFR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Global Interrupt Control High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GICHR(pub u32);
    impl GICHR {
        #[inline(always)]
        pub const fn GIWE16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn GIWE17(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn GIWE18(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn GIWE19(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn GIWE20(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GIWE21(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GIWE22(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GIWE23(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn GIWE24(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn GIWE25(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn GIWE26(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn GIWE27(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn GIWE28(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn GIWE29(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn GIWE30(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn GIWE31(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn GIWD(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GIWD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GICHR {
        #[inline(always)]
        fn default() -> GICHR {
            GICHR(0)
        }
    }
    impl core::fmt::Debug for GICHR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GICHR")
                .field("GIWE16", &self.GIWE16())
                .field("GIWE17", &self.GIWE17())
                .field("GIWE18", &self.GIWE18())
                .field("GIWE19", &self.GIWE19())
                .field("GIWE20", &self.GIWE20())
                .field("GIWE21", &self.GIWE21())
                .field("GIWE22", &self.GIWE22())
                .field("GIWE23", &self.GIWE23())
                .field("GIWE24", &self.GIWE24())
                .field("GIWE25", &self.GIWE25())
                .field("GIWE26", &self.GIWE26())
                .field("GIWE27", &self.GIWE27())
                .field("GIWE28", &self.GIWE28())
                .field("GIWE29", &self.GIWE29())
                .field("GIWE30", &self.GIWE30())
                .field("GIWE31", &self.GIWE31())
                .field("GIWD", &self.GIWD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GICHR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GICHR {{ GIWE16: {=bool:?}, GIWE17: {=bool:?}, GIWE18: {=bool:?}, GIWE19: {=bool:?}, GIWE20: {=bool:?}, GIWE21: {=bool:?}, GIWE22: {=bool:?}, GIWE23: {=bool:?}, GIWE24: {=bool:?}, GIWE25: {=bool:?}, GIWE26: {=bool:?}, GIWE27: {=bool:?}, GIWE28: {=bool:?}, GIWE29: {=bool:?}, GIWE30: {=bool:?}, GIWE31: {=bool:?}, GIWD: {=u16:?} }}" , self . GIWE16 () , self . GIWE17 () , self . GIWE18 () , self . GIWE19 () , self . GIWE20 () , self . GIWE21 () , self . GIWE22 () , self . GIWE23 () , self . GIWE24 () , self . GIWE25 () , self . GIWE26 () , self . GIWE27 () , self . GIWE28 () , self . GIWE29 () , self . GIWE30 () , self . GIWE31 () , self . GIWD ())
        }
    }
    #[doc = "Global Interrupt Control Low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GICLR(pub u32);
    impl GICLR {
        #[inline(always)]
        pub const fn GIWE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn GIWE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn GIWE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn GIWE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn GIWE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn GIWE5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GIWE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn GIWE7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn GIWE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn GIWE9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn GIWE10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn GIWE11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn GIWE12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn GIWE13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn GIWE14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn GIWE15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GIWE15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn GIWD(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_GIWD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GICLR {
        #[inline(always)]
        fn default() -> GICLR {
            GICLR(0)
        }
    }
    impl core::fmt::Debug for GICLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GICLR")
                .field("GIWE0", &self.GIWE0())
                .field("GIWE1", &self.GIWE1())
                .field("GIWE2", &self.GIWE2())
                .field("GIWE3", &self.GIWE3())
                .field("GIWE4", &self.GIWE4())
                .field("GIWE5", &self.GIWE5())
                .field("GIWE6", &self.GIWE6())
                .field("GIWE7", &self.GIWE7())
                .field("GIWE8", &self.GIWE8())
                .field("GIWE9", &self.GIWE9())
                .field("GIWE10", &self.GIWE10())
                .field("GIWE11", &self.GIWE11())
                .field("GIWE12", &self.GIWE12())
                .field("GIWE13", &self.GIWE13())
                .field("GIWE14", &self.GIWE14())
                .field("GIWE15", &self.GIWE15())
                .field("GIWD", &self.GIWD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GICLR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GICLR {{ GIWE0: {=bool:?}, GIWE1: {=bool:?}, GIWE2: {=bool:?}, GIWE3: {=bool:?}, GIWE4: {=bool:?}, GIWE5: {=bool:?}, GIWE6: {=bool:?}, GIWE7: {=bool:?}, GIWE8: {=bool:?}, GIWE9: {=bool:?}, GIWE10: {=bool:?}, GIWE11: {=bool:?}, GIWE12: {=bool:?}, GIWE13: {=bool:?}, GIWE14: {=bool:?}, GIWE15: {=bool:?}, GIWD: {=u16:?} }}" , self . GIWE0 () , self . GIWE1 () , self . GIWE2 () , self . GIWE3 () , self . GIWE4 () , self . GIWE5 () , self . GIWE6 () , self . GIWE7 () , self . GIWE8 () , self . GIWE9 () , self . GIWE10 () , self . GIWE11 () , self . GIWE12 () , self . GIWE13 () , self . GIWE14 () , self . GIWE15 () , self . GIWD ())
        }
    }
    #[doc = "Interrupt Control Nonprivilege"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ICNP(pub u32);
    impl ICNP {
        #[inline(always)]
        pub const fn NPE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NPE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for ICNP {
        #[inline(always)]
        fn default() -> ICNP {
            ICNP(0)
        }
    }
    impl core::fmt::Debug for ICNP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ICNP")
                .field("NPE0", &self.NPE0())
                .field("NPE1", &self.NPE1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ICNP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ICNP {{ NPE0: {=bool:?}, NPE1: {=bool:?} }}",
                self.NPE0(),
                self.NPE1()
            )
        }
    }
    #[doc = "Interrupt Control Nonsecure"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ICNS(pub u32);
    impl ICNS {
        #[inline(always)]
        pub const fn NSE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NSE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for ICNS {
        #[inline(always)]
        fn default() -> ICNS {
            ICNS(0)
        }
    }
    impl core::fmt::Debug for ICNS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ICNS")
                .field("NSE0", &self.NSE0())
                .field("NSE1", &self.NSE1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ICNS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ICNS {{ NSE0: {=bool:?}, NSE1: {=bool:?} }}",
                self.NSE0(),
                self.NSE1()
            )
        }
    }
    #[doc = "Interrupt Control 0..Interrupt Control 31"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ICR(pub u32);
    impl ICR {
        #[inline(always)]
        pub const fn IRQC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IRQC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn IRQS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn ISF(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ICR {
        #[inline(always)]
        fn default() -> ICR {
            ICR(0)
        }
    }
    impl core::fmt::Debug for ICR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ICR")
                .field("IRQC", &self.IRQC())
                .field("IRQS", &self.IRQS())
                .field("LK", &self.LK())
                .field("ISF", &self.ISF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ICR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ICR {{ IRQC: {=u8:?}, IRQS: {=bool:?}, LK: {=bool:?}, ISF: {=bool:?} }}",
                self.IRQC(),
                self.IRQS(),
                self.LK(),
                self.ISF()
            )
        }
    }
    #[doc = "Interrupt Status Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISFR(pub u32);
    impl ISFR {
        #[inline(always)]
        pub const fn ISF0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ISF1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ISF2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ISF3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ISF4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn ISF5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ISF6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ISF7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ISF8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ISF9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ISF10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ISF11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn ISF12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn ISF13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn ISF14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn ISF15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ISF16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ISF17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn ISF18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn ISF19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn ISF20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn ISF21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn ISF22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn ISF23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn ISF24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn ISF25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn ISF26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ISF27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ISF28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn ISF29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn ISF30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ISF31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISF31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ISFR {
        #[inline(always)]
        fn default() -> ISFR {
            ISFR(0)
        }
    }
    impl core::fmt::Debug for ISFR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISFR")
                .field("ISF0", &self.ISF0())
                .field("ISF1", &self.ISF1())
                .field("ISF2", &self.ISF2())
                .field("ISF3", &self.ISF3())
                .field("ISF4", &self.ISF4())
                .field("ISF5", &self.ISF5())
                .field("ISF6", &self.ISF6())
                .field("ISF7", &self.ISF7())
                .field("ISF8", &self.ISF8())
                .field("ISF9", &self.ISF9())
                .field("ISF10", &self.ISF10())
                .field("ISF11", &self.ISF11())
                .field("ISF12", &self.ISF12())
                .field("ISF13", &self.ISF13())
                .field("ISF14", &self.ISF14())
                .field("ISF15", &self.ISF15())
                .field("ISF16", &self.ISF16())
                .field("ISF17", &self.ISF17())
                .field("ISF18", &self.ISF18())
                .field("ISF19", &self.ISF19())
                .field("ISF20", &self.ISF20())
                .field("ISF21", &self.ISF21())
                .field("ISF22", &self.ISF22())
                .field("ISF23", &self.ISF23())
                .field("ISF24", &self.ISF24())
                .field("ISF25", &self.ISF25())
                .field("ISF26", &self.ISF26())
                .field("ISF27", &self.ISF27())
                .field("ISF28", &self.ISF28())
                .field("ISF29", &self.ISF29())
                .field("ISF30", &self.ISF30())
                .field("ISF31", &self.ISF31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISFR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ISFR {{ ISF0: {=bool:?}, ISF1: {=bool:?}, ISF2: {=bool:?}, ISF3: {=bool:?}, ISF4: {=bool:?}, ISF5: {=bool:?}, ISF6: {=bool:?}, ISF7: {=bool:?}, ISF8: {=bool:?}, ISF9: {=bool:?}, ISF10: {=bool:?}, ISF11: {=bool:?}, ISF12: {=bool:?}, ISF13: {=bool:?}, ISF14: {=bool:?}, ISF15: {=bool:?}, ISF16: {=bool:?}, ISF17: {=bool:?}, ISF18: {=bool:?}, ISF19: {=bool:?}, ISF20: {=bool:?}, ISF21: {=bool:?}, ISF22: {=bool:?}, ISF23: {=bool:?}, ISF24: {=bool:?}, ISF25: {=bool:?}, ISF26: {=bool:?}, ISF27: {=bool:?}, ISF28: {=bool:?}, ISF29: {=bool:?}, ISF30: {=bool:?}, ISF31: {=bool:?} }}" , self . ISF0 () , self . ISF1 () , self . ISF2 () , self . ISF3 () , self . ISF4 () , self . ISF5 () , self . ISF6 () , self . ISF7 () , self . ISF8 () , self . ISF9 () , self . ISF10 () , self . ISF11 () , self . ISF12 () , self . ISF13 () , self . ISF14 () , self . ISF15 () , self . ISF16 () , self . ISF17 () , self . ISF18 () , self . ISF19 () , self . ISF20 () , self . ISF21 () , self . ISF22 () , self . ISF23 () , self . ISF24 () , self . ISF25 () , self . ISF26 () , self . ISF27 () , self . ISF28 () , self . ISF29 () , self . ISF30 () , self . ISF31 ())
        }
    }
    #[doc = "Lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LOCK(pub u32);
    impl LOCK {
        #[inline(always)]
        pub const fn PCNS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PCNS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ICNS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ICNS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PCNP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PCNP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ICNP(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ICNP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for LOCK {
        #[inline(always)]
        fn default() -> LOCK {
            LOCK(0)
        }
    }
    impl core::fmt::Debug for LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LOCK")
                .field("PCNS", &self.PCNS())
                .field("ICNS", &self.ICNS())
                .field("PCNP", &self.PCNP())
                .field("ICNP", &self.ICNP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LOCK {{ PCNS: {=bool:?}, ICNS: {=bool:?}, PCNP: {=bool:?}, ICNP: {=bool:?} }}",
                self.PCNS(),
                self.ICNS(),
                self.PCNP(),
                self.ICNP()
            )
        }
    }
    #[doc = "Parameter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn IRQNUM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IRQNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
                .field("IRQNUM", &self.IRQNUM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PARAM {{ IRQNUM: {=u8:?} }}", self.IRQNUM())
        }
    }
    #[doc = "Pin Control Nonprivilege"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCNP(pub u32);
    impl PCNP {
        #[inline(always)]
        pub const fn NPE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NPE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn NPE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NPE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn NPE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn NPE5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn NPE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn NPE7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NPE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn NPE9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn NPE10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn NPE11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn NPE12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn NPE13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn NPE14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn NPE15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn NPE16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn NPE17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn NPE18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn NPE19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn NPE20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn NPE21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn NPE22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn NPE23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn NPE24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn NPE25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn NPE26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn NPE27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn NPE28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn NPE29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn NPE30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn NPE31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NPE31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PCNP {
        #[inline(always)]
        fn default() -> PCNP {
            PCNP(0)
        }
    }
    impl core::fmt::Debug for PCNP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCNP")
                .field("NPE0", &self.NPE0())
                .field("NPE1", &self.NPE1())
                .field("NPE2", &self.NPE2())
                .field("NPE3", &self.NPE3())
                .field("NPE4", &self.NPE4())
                .field("NPE5", &self.NPE5())
                .field("NPE6", &self.NPE6())
                .field("NPE7", &self.NPE7())
                .field("NPE8", &self.NPE8())
                .field("NPE9", &self.NPE9())
                .field("NPE10", &self.NPE10())
                .field("NPE11", &self.NPE11())
                .field("NPE12", &self.NPE12())
                .field("NPE13", &self.NPE13())
                .field("NPE14", &self.NPE14())
                .field("NPE15", &self.NPE15())
                .field("NPE16", &self.NPE16())
                .field("NPE17", &self.NPE17())
                .field("NPE18", &self.NPE18())
                .field("NPE19", &self.NPE19())
                .field("NPE20", &self.NPE20())
                .field("NPE21", &self.NPE21())
                .field("NPE22", &self.NPE22())
                .field("NPE23", &self.NPE23())
                .field("NPE24", &self.NPE24())
                .field("NPE25", &self.NPE25())
                .field("NPE26", &self.NPE26())
                .field("NPE27", &self.NPE27())
                .field("NPE28", &self.NPE28())
                .field("NPE29", &self.NPE29())
                .field("NPE30", &self.NPE30())
                .field("NPE31", &self.NPE31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCNP {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PCNP {{ NPE0: {=bool:?}, NPE1: {=bool:?}, NPE2: {=bool:?}, NPE3: {=bool:?}, NPE4: {=bool:?}, NPE5: {=bool:?}, NPE6: {=bool:?}, NPE7: {=bool:?}, NPE8: {=bool:?}, NPE9: {=bool:?}, NPE10: {=bool:?}, NPE11: {=bool:?}, NPE12: {=bool:?}, NPE13: {=bool:?}, NPE14: {=bool:?}, NPE15: {=bool:?}, NPE16: {=bool:?}, NPE17: {=bool:?}, NPE18: {=bool:?}, NPE19: {=bool:?}, NPE20: {=bool:?}, NPE21: {=bool:?}, NPE22: {=bool:?}, NPE23: {=bool:?}, NPE24: {=bool:?}, NPE25: {=bool:?}, NPE26: {=bool:?}, NPE27: {=bool:?}, NPE28: {=bool:?}, NPE29: {=bool:?}, NPE30: {=bool:?}, NPE31: {=bool:?} }}" , self . NPE0 () , self . NPE1 () , self . NPE2 () , self . NPE3 () , self . NPE4 () , self . NPE5 () , self . NPE6 () , self . NPE7 () , self . NPE8 () , self . NPE9 () , self . NPE10 () , self . NPE11 () , self . NPE12 () , self . NPE13 () , self . NPE14 () , self . NPE15 () , self . NPE16 () , self . NPE17 () , self . NPE18 () , self . NPE19 () , self . NPE20 () , self . NPE21 () , self . NPE22 () , self . NPE23 () , self . NPE24 () , self . NPE25 () , self . NPE26 () , self . NPE27 () , self . NPE28 () , self . NPE29 () , self . NPE30 () , self . NPE31 ())
        }
    }
    #[doc = "Pin Control Nonsecure"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCNS(pub u32);
    impl PCNS {
        #[inline(always)]
        pub const fn NSE0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn NSE1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn NSE2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn NSE3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn NSE4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn NSE5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn NSE6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn NSE7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn NSE8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn NSE9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn NSE10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn NSE11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn NSE12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn NSE13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn NSE14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn NSE15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn NSE16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn NSE17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn NSE18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn NSE19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn NSE20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn NSE21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn NSE22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn NSE23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn NSE24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn NSE25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn NSE26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn NSE27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn NSE28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn NSE29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn NSE30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn NSE31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NSE31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PCNS {
        #[inline(always)]
        fn default() -> PCNS {
            PCNS(0)
        }
    }
    impl core::fmt::Debug for PCNS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCNS")
                .field("NSE0", &self.NSE0())
                .field("NSE1", &self.NSE1())
                .field("NSE2", &self.NSE2())
                .field("NSE3", &self.NSE3())
                .field("NSE4", &self.NSE4())
                .field("NSE5", &self.NSE5())
                .field("NSE6", &self.NSE6())
                .field("NSE7", &self.NSE7())
                .field("NSE8", &self.NSE8())
                .field("NSE9", &self.NSE9())
                .field("NSE10", &self.NSE10())
                .field("NSE11", &self.NSE11())
                .field("NSE12", &self.NSE12())
                .field("NSE13", &self.NSE13())
                .field("NSE14", &self.NSE14())
                .field("NSE15", &self.NSE15())
                .field("NSE16", &self.NSE16())
                .field("NSE17", &self.NSE17())
                .field("NSE18", &self.NSE18())
                .field("NSE19", &self.NSE19())
                .field("NSE20", &self.NSE20())
                .field("NSE21", &self.NSE21())
                .field("NSE22", &self.NSE22())
                .field("NSE23", &self.NSE23())
                .field("NSE24", &self.NSE24())
                .field("NSE25", &self.NSE25())
                .field("NSE26", &self.NSE26())
                .field("NSE27", &self.NSE27())
                .field("NSE28", &self.NSE28())
                .field("NSE29", &self.NSE29())
                .field("NSE30", &self.NSE30())
                .field("NSE31", &self.NSE31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCNS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PCNS {{ NSE0: {=bool:?}, NSE1: {=bool:?}, NSE2: {=bool:?}, NSE3: {=bool:?}, NSE4: {=bool:?}, NSE5: {=bool:?}, NSE6: {=bool:?}, NSE7: {=bool:?}, NSE8: {=bool:?}, NSE9: {=bool:?}, NSE10: {=bool:?}, NSE11: {=bool:?}, NSE12: {=bool:?}, NSE13: {=bool:?}, NSE14: {=bool:?}, NSE15: {=bool:?}, NSE16: {=bool:?}, NSE17: {=bool:?}, NSE18: {=bool:?}, NSE19: {=bool:?}, NSE20: {=bool:?}, NSE21: {=bool:?}, NSE22: {=bool:?}, NSE23: {=bool:?}, NSE24: {=bool:?}, NSE25: {=bool:?}, NSE26: {=bool:?}, NSE27: {=bool:?}, NSE28: {=bool:?}, NSE29: {=bool:?}, NSE30: {=bool:?}, NSE31: {=bool:?} }}" , self . NSE0 () , self . NSE1 () , self . NSE2 () , self . NSE3 () , self . NSE4 () , self . NSE5 () , self . NSE6 () , self . NSE7 () , self . NSE8 () , self . NSE9 () , self . NSE10 () , self . NSE11 () , self . NSE12 () , self . NSE13 () , self . NSE14 () , self . NSE15 () , self . NSE16 () , self . NSE17 () , self . NSE18 () , self . NSE19 () , self . NSE20 () , self . NSE21 () , self . NSE22 () , self . NSE23 () , self . NSE24 () , self . NSE25 () , self . NSE26 () , self . NSE27 () , self . NSE28 () , self . NSE29 () , self . NSE30 () , self . NSE31 ())
        }
    }
    #[doc = "Port Clear Output"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PCOR(pub u32);
    impl PCOR {
        #[inline(always)]
        pub const fn PTCO(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PTCO(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PCOR {
        #[inline(always)]
        fn default() -> PCOR {
            PCOR(0)
        }
    }
    impl core::fmt::Debug for PCOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PCOR")
                .field("PTCO[0]", &self.PTCO(0usize))
                .field("PTCO[1]", &self.PTCO(1usize))
                .field("PTCO[2]", &self.PTCO(2usize))
                .field("PTCO[3]", &self.PTCO(3usize))
                .field("PTCO[4]", &self.PTCO(4usize))
                .field("PTCO[5]", &self.PTCO(5usize))
                .field("PTCO[6]", &self.PTCO(6usize))
                .field("PTCO[7]", &self.PTCO(7usize))
                .field("PTCO[8]", &self.PTCO(8usize))
                .field("PTCO[9]", &self.PTCO(9usize))
                .field("PTCO[10]", &self.PTCO(10usize))
                .field("PTCO[11]", &self.PTCO(11usize))
                .field("PTCO[12]", &self.PTCO(12usize))
                .field("PTCO[13]", &self.PTCO(13usize))
                .field("PTCO[14]", &self.PTCO(14usize))
                .field("PTCO[15]", &self.PTCO(15usize))
                .field("PTCO[16]", &self.PTCO(16usize))
                .field("PTCO[17]", &self.PTCO(17usize))
                .field("PTCO[18]", &self.PTCO(18usize))
                .field("PTCO[19]", &self.PTCO(19usize))
                .field("PTCO[20]", &self.PTCO(20usize))
                .field("PTCO[21]", &self.PTCO(21usize))
                .field("PTCO[22]", &self.PTCO(22usize))
                .field("PTCO[23]", &self.PTCO(23usize))
                .field("PTCO[24]", &self.PTCO(24usize))
                .field("PTCO[25]", &self.PTCO(25usize))
                .field("PTCO[26]", &self.PTCO(26usize))
                .field("PTCO[27]", &self.PTCO(27usize))
                .field("PTCO[28]", &self.PTCO(28usize))
                .field("PTCO[29]", &self.PTCO(29usize))
                .field("PTCO[30]", &self.PTCO(30usize))
                .field("PTCO[31]", &self.PTCO(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PCOR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PCOR {{ PTCO[0]: {=bool:?}, PTCO[1]: {=bool:?}, PTCO[2]: {=bool:?}, PTCO[3]: {=bool:?}, PTCO[4]: {=bool:?}, PTCO[5]: {=bool:?}, PTCO[6]: {=bool:?}, PTCO[7]: {=bool:?}, PTCO[8]: {=bool:?}, PTCO[9]: {=bool:?}, PTCO[10]: {=bool:?}, PTCO[11]: {=bool:?}, PTCO[12]: {=bool:?}, PTCO[13]: {=bool:?}, PTCO[14]: {=bool:?}, PTCO[15]: {=bool:?}, PTCO[16]: {=bool:?}, PTCO[17]: {=bool:?}, PTCO[18]: {=bool:?}, PTCO[19]: {=bool:?}, PTCO[20]: {=bool:?}, PTCO[21]: {=bool:?}, PTCO[22]: {=bool:?}, PTCO[23]: {=bool:?}, PTCO[24]: {=bool:?}, PTCO[25]: {=bool:?}, PTCO[26]: {=bool:?}, PTCO[27]: {=bool:?}, PTCO[28]: {=bool:?}, PTCO[29]: {=bool:?}, PTCO[30]: {=bool:?}, PTCO[31]: {=bool:?} }}" , self . PTCO (0usize) , self . PTCO (1usize) , self . PTCO (2usize) , self . PTCO (3usize) , self . PTCO (4usize) , self . PTCO (5usize) , self . PTCO (6usize) , self . PTCO (7usize) , self . PTCO (8usize) , self . PTCO (9usize) , self . PTCO (10usize) , self . PTCO (11usize) , self . PTCO (12usize) , self . PTCO (13usize) , self . PTCO (14usize) , self . PTCO (15usize) , self . PTCO (16usize) , self . PTCO (17usize) , self . PTCO (18usize) , self . PTCO (19usize) , self . PTCO (20usize) , self . PTCO (21usize) , self . PTCO (22usize) , self . PTCO (23usize) , self . PTCO (24usize) , self . PTCO (25usize) , self . PTCO (26usize) , self . PTCO (27usize) , self . PTCO (28usize) , self . PTCO (29usize) , self . PTCO (30usize) , self . PTCO (31usize))
        }
    }
    #[doc = "Port Data Direction"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDDR(pub u32);
    impl PDDR {
        #[inline(always)]
        pub const fn PDD(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDD(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PDDR {
        #[inline(always)]
        fn default() -> PDDR {
            PDDR(0)
        }
    }
    impl core::fmt::Debug for PDDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDDR")
                .field("PDD[0]", &self.PDD(0usize))
                .field("PDD[1]", &self.PDD(1usize))
                .field("PDD[2]", &self.PDD(2usize))
                .field("PDD[3]", &self.PDD(3usize))
                .field("PDD[4]", &self.PDD(4usize))
                .field("PDD[5]", &self.PDD(5usize))
                .field("PDD[6]", &self.PDD(6usize))
                .field("PDD[7]", &self.PDD(7usize))
                .field("PDD[8]", &self.PDD(8usize))
                .field("PDD[9]", &self.PDD(9usize))
                .field("PDD[10]", &self.PDD(10usize))
                .field("PDD[11]", &self.PDD(11usize))
                .field("PDD[12]", &self.PDD(12usize))
                .field("PDD[13]", &self.PDD(13usize))
                .field("PDD[14]", &self.PDD(14usize))
                .field("PDD[15]", &self.PDD(15usize))
                .field("PDD[16]", &self.PDD(16usize))
                .field("PDD[17]", &self.PDD(17usize))
                .field("PDD[18]", &self.PDD(18usize))
                .field("PDD[19]", &self.PDD(19usize))
                .field("PDD[20]", &self.PDD(20usize))
                .field("PDD[21]", &self.PDD(21usize))
                .field("PDD[22]", &self.PDD(22usize))
                .field("PDD[23]", &self.PDD(23usize))
                .field("PDD[24]", &self.PDD(24usize))
                .field("PDD[25]", &self.PDD(25usize))
                .field("PDD[26]", &self.PDD(26usize))
                .field("PDD[27]", &self.PDD(27usize))
                .field("PDD[28]", &self.PDD(28usize))
                .field("PDD[29]", &self.PDD(29usize))
                .field("PDD[30]", &self.PDD(30usize))
                .field("PDD[31]", &self.PDD(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDDR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PDDR {{ PDD[0]: {=bool:?}, PDD[1]: {=bool:?}, PDD[2]: {=bool:?}, PDD[3]: {=bool:?}, PDD[4]: {=bool:?}, PDD[5]: {=bool:?}, PDD[6]: {=bool:?}, PDD[7]: {=bool:?}, PDD[8]: {=bool:?}, PDD[9]: {=bool:?}, PDD[10]: {=bool:?}, PDD[11]: {=bool:?}, PDD[12]: {=bool:?}, PDD[13]: {=bool:?}, PDD[14]: {=bool:?}, PDD[15]: {=bool:?}, PDD[16]: {=bool:?}, PDD[17]: {=bool:?}, PDD[18]: {=bool:?}, PDD[19]: {=bool:?}, PDD[20]: {=bool:?}, PDD[21]: {=bool:?}, PDD[22]: {=bool:?}, PDD[23]: {=bool:?}, PDD[24]: {=bool:?}, PDD[25]: {=bool:?}, PDD[26]: {=bool:?}, PDD[27]: {=bool:?}, PDD[28]: {=bool:?}, PDD[29]: {=bool:?}, PDD[30]: {=bool:?}, PDD[31]: {=bool:?} }}" , self . PDD (0usize) , self . PDD (1usize) , self . PDD (2usize) , self . PDD (3usize) , self . PDD (4usize) , self . PDD (5usize) , self . PDD (6usize) , self . PDD (7usize) , self . PDD (8usize) , self . PDD (9usize) , self . PDD (10usize) , self . PDD (11usize) , self . PDD (12usize) , self . PDD (13usize) , self . PDD (14usize) , self . PDD (15usize) , self . PDD (16usize) , self . PDD (17usize) , self . PDD (18usize) , self . PDD (19usize) , self . PDD (20usize) , self . PDD (21usize) , self . PDD (22usize) , self . PDD (23usize) , self . PDD (24usize) , self . PDD (25usize) , self . PDD (26usize) , self . PDD (27usize) , self . PDD (28usize) , self . PDD (29usize) , self . PDD (30usize) , self . PDD (31usize))
        }
    }
    #[doc = "Port Data Input"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDIR(pub u32);
    impl PDIR {
        #[inline(always)]
        pub const fn PDI(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDI(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PDIR {
        #[inline(always)]
        fn default() -> PDIR {
            PDIR(0)
        }
    }
    impl core::fmt::Debug for PDIR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDIR")
                .field("PDI[0]", &self.PDI(0usize))
                .field("PDI[1]", &self.PDI(1usize))
                .field("PDI[2]", &self.PDI(2usize))
                .field("PDI[3]", &self.PDI(3usize))
                .field("PDI[4]", &self.PDI(4usize))
                .field("PDI[5]", &self.PDI(5usize))
                .field("PDI[6]", &self.PDI(6usize))
                .field("PDI[7]", &self.PDI(7usize))
                .field("PDI[8]", &self.PDI(8usize))
                .field("PDI[9]", &self.PDI(9usize))
                .field("PDI[10]", &self.PDI(10usize))
                .field("PDI[11]", &self.PDI(11usize))
                .field("PDI[12]", &self.PDI(12usize))
                .field("PDI[13]", &self.PDI(13usize))
                .field("PDI[14]", &self.PDI(14usize))
                .field("PDI[15]", &self.PDI(15usize))
                .field("PDI[16]", &self.PDI(16usize))
                .field("PDI[17]", &self.PDI(17usize))
                .field("PDI[18]", &self.PDI(18usize))
                .field("PDI[19]", &self.PDI(19usize))
                .field("PDI[20]", &self.PDI(20usize))
                .field("PDI[21]", &self.PDI(21usize))
                .field("PDI[22]", &self.PDI(22usize))
                .field("PDI[23]", &self.PDI(23usize))
                .field("PDI[24]", &self.PDI(24usize))
                .field("PDI[25]", &self.PDI(25usize))
                .field("PDI[26]", &self.PDI(26usize))
                .field("PDI[27]", &self.PDI(27usize))
                .field("PDI[28]", &self.PDI(28usize))
                .field("PDI[29]", &self.PDI(29usize))
                .field("PDI[30]", &self.PDI(30usize))
                .field("PDI[31]", &self.PDI(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDIR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PDIR {{ PDI[0]: {=bool:?}, PDI[1]: {=bool:?}, PDI[2]: {=bool:?}, PDI[3]: {=bool:?}, PDI[4]: {=bool:?}, PDI[5]: {=bool:?}, PDI[6]: {=bool:?}, PDI[7]: {=bool:?}, PDI[8]: {=bool:?}, PDI[9]: {=bool:?}, PDI[10]: {=bool:?}, PDI[11]: {=bool:?}, PDI[12]: {=bool:?}, PDI[13]: {=bool:?}, PDI[14]: {=bool:?}, PDI[15]: {=bool:?}, PDI[16]: {=bool:?}, PDI[17]: {=bool:?}, PDI[18]: {=bool:?}, PDI[19]: {=bool:?}, PDI[20]: {=bool:?}, PDI[21]: {=bool:?}, PDI[22]: {=bool:?}, PDI[23]: {=bool:?}, PDI[24]: {=bool:?}, PDI[25]: {=bool:?}, PDI[26]: {=bool:?}, PDI[27]: {=bool:?}, PDI[28]: {=bool:?}, PDI[29]: {=bool:?}, PDI[30]: {=bool:?}, PDI[31]: {=bool:?} }}" , self . PDI (0usize) , self . PDI (1usize) , self . PDI (2usize) , self . PDI (3usize) , self . PDI (4usize) , self . PDI (5usize) , self . PDI (6usize) , self . PDI (7usize) , self . PDI (8usize) , self . PDI (9usize) , self . PDI (10usize) , self . PDI (11usize) , self . PDI (12usize) , self . PDI (13usize) , self . PDI (14usize) , self . PDI (15usize) , self . PDI (16usize) , self . PDI (17usize) , self . PDI (18usize) , self . PDI (19usize) , self . PDI (20usize) , self . PDI (21usize) , self . PDI (22usize) , self . PDI (23usize) , self . PDI (24usize) , self . PDI (25usize) , self . PDI (26usize) , self . PDI (27usize) , self . PDI (28usize) , self . PDI (29usize) , self . PDI (30usize) , self . PDI (31usize))
        }
    }
    #[doc = "Port Data Output"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDOR(pub u32);
    impl PDOR {
        #[inline(always)]
        pub const fn PDO(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDO(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PDOR {
        #[inline(always)]
        fn default() -> PDOR {
            PDOR(0)
        }
    }
    impl core::fmt::Debug for PDOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDOR")
                .field("PDO[0]", &self.PDO(0usize))
                .field("PDO[1]", &self.PDO(1usize))
                .field("PDO[2]", &self.PDO(2usize))
                .field("PDO[3]", &self.PDO(3usize))
                .field("PDO[4]", &self.PDO(4usize))
                .field("PDO[5]", &self.PDO(5usize))
                .field("PDO[6]", &self.PDO(6usize))
                .field("PDO[7]", &self.PDO(7usize))
                .field("PDO[8]", &self.PDO(8usize))
                .field("PDO[9]", &self.PDO(9usize))
                .field("PDO[10]", &self.PDO(10usize))
                .field("PDO[11]", &self.PDO(11usize))
                .field("PDO[12]", &self.PDO(12usize))
                .field("PDO[13]", &self.PDO(13usize))
                .field("PDO[14]", &self.PDO(14usize))
                .field("PDO[15]", &self.PDO(15usize))
                .field("PDO[16]", &self.PDO(16usize))
                .field("PDO[17]", &self.PDO(17usize))
                .field("PDO[18]", &self.PDO(18usize))
                .field("PDO[19]", &self.PDO(19usize))
                .field("PDO[20]", &self.PDO(20usize))
                .field("PDO[21]", &self.PDO(21usize))
                .field("PDO[22]", &self.PDO(22usize))
                .field("PDO[23]", &self.PDO(23usize))
                .field("PDO[24]", &self.PDO(24usize))
                .field("PDO[25]", &self.PDO(25usize))
                .field("PDO[26]", &self.PDO(26usize))
                .field("PDO[27]", &self.PDO(27usize))
                .field("PDO[28]", &self.PDO(28usize))
                .field("PDO[29]", &self.PDO(29usize))
                .field("PDO[30]", &self.PDO(30usize))
                .field("PDO[31]", &self.PDO(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDOR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PDOR {{ PDO[0]: {=bool:?}, PDO[1]: {=bool:?}, PDO[2]: {=bool:?}, PDO[3]: {=bool:?}, PDO[4]: {=bool:?}, PDO[5]: {=bool:?}, PDO[6]: {=bool:?}, PDO[7]: {=bool:?}, PDO[8]: {=bool:?}, PDO[9]: {=bool:?}, PDO[10]: {=bool:?}, PDO[11]: {=bool:?}, PDO[12]: {=bool:?}, PDO[13]: {=bool:?}, PDO[14]: {=bool:?}, PDO[15]: {=bool:?}, PDO[16]: {=bool:?}, PDO[17]: {=bool:?}, PDO[18]: {=bool:?}, PDO[19]: {=bool:?}, PDO[20]: {=bool:?}, PDO[21]: {=bool:?}, PDO[22]: {=bool:?}, PDO[23]: {=bool:?}, PDO[24]: {=bool:?}, PDO[25]: {=bool:?}, PDO[26]: {=bool:?}, PDO[27]: {=bool:?}, PDO[28]: {=bool:?}, PDO[29]: {=bool:?}, PDO[30]: {=bool:?}, PDO[31]: {=bool:?} }}" , self . PDO (0usize) , self . PDO (1usize) , self . PDO (2usize) , self . PDO (3usize) , self . PDO (4usize) , self . PDO (5usize) , self . PDO (6usize) , self . PDO (7usize) , self . PDO (8usize) , self . PDO (9usize) , self . PDO (10usize) , self . PDO (11usize) , self . PDO (12usize) , self . PDO (13usize) , self . PDO (14usize) , self . PDO (15usize) , self . PDO (16usize) , self . PDO (17usize) , self . PDO (18usize) , self . PDO (19usize) , self . PDO (20usize) , self . PDO (21usize) , self . PDO (22usize) , self . PDO (23usize) , self . PDO (24usize) , self . PDO (25usize) , self . PDO (26usize) , self . PDO (27usize) , self . PDO (28usize) , self . PDO (29usize) , self . PDO (30usize) , self . PDO (31usize))
        }
    }
    #[doc = "Pin Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDR(pub u8);
    impl PDR {
        #[inline(always)]
        pub const fn PD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for PDR {
        #[inline(always)]
        fn default() -> PDR {
            PDR(0)
        }
    }
    impl core::fmt::Debug for PDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PDR").field("PD", &self.PD()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PDR {{ PD: {=bool:?} }}", self.PD())
        }
    }
    #[doc = "Port Input Disable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PIDR(pub u32);
    impl PIDR {
        #[inline(always)]
        pub const fn PID(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PID(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PIDR {
        #[inline(always)]
        fn default() -> PIDR {
            PIDR(0)
        }
    }
    impl core::fmt::Debug for PIDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PIDR")
                .field("PID[0]", &self.PID(0usize))
                .field("PID[1]", &self.PID(1usize))
                .field("PID[2]", &self.PID(2usize))
                .field("PID[3]", &self.PID(3usize))
                .field("PID[4]", &self.PID(4usize))
                .field("PID[5]", &self.PID(5usize))
                .field("PID[6]", &self.PID(6usize))
                .field("PID[7]", &self.PID(7usize))
                .field("PID[8]", &self.PID(8usize))
                .field("PID[9]", &self.PID(9usize))
                .field("PID[10]", &self.PID(10usize))
                .field("PID[11]", &self.PID(11usize))
                .field("PID[12]", &self.PID(12usize))
                .field("PID[13]", &self.PID(13usize))
                .field("PID[14]", &self.PID(14usize))
                .field("PID[15]", &self.PID(15usize))
                .field("PID[16]", &self.PID(16usize))
                .field("PID[17]", &self.PID(17usize))
                .field("PID[18]", &self.PID(18usize))
                .field("PID[19]", &self.PID(19usize))
                .field("PID[20]", &self.PID(20usize))
                .field("PID[21]", &self.PID(21usize))
                .field("PID[22]", &self.PID(22usize))
                .field("PID[23]", &self.PID(23usize))
                .field("PID[24]", &self.PID(24usize))
                .field("PID[25]", &self.PID(25usize))
                .field("PID[26]", &self.PID(26usize))
                .field("PID[27]", &self.PID(27usize))
                .field("PID[28]", &self.PID(28usize))
                .field("PID[29]", &self.PID(29usize))
                .field("PID[30]", &self.PID(30usize))
                .field("PID[31]", &self.PID(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PIDR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PIDR {{ PID[0]: {=bool:?}, PID[1]: {=bool:?}, PID[2]: {=bool:?}, PID[3]: {=bool:?}, PID[4]: {=bool:?}, PID[5]: {=bool:?}, PID[6]: {=bool:?}, PID[7]: {=bool:?}, PID[8]: {=bool:?}, PID[9]: {=bool:?}, PID[10]: {=bool:?}, PID[11]: {=bool:?}, PID[12]: {=bool:?}, PID[13]: {=bool:?}, PID[14]: {=bool:?}, PID[15]: {=bool:?}, PID[16]: {=bool:?}, PID[17]: {=bool:?}, PID[18]: {=bool:?}, PID[19]: {=bool:?}, PID[20]: {=bool:?}, PID[21]: {=bool:?}, PID[22]: {=bool:?}, PID[23]: {=bool:?}, PID[24]: {=bool:?}, PID[25]: {=bool:?}, PID[26]: {=bool:?}, PID[27]: {=bool:?}, PID[28]: {=bool:?}, PID[29]: {=bool:?}, PID[30]: {=bool:?}, PID[31]: {=bool:?} }}" , self . PID (0usize) , self . PID (1usize) , self . PID (2usize) , self . PID (3usize) , self . PID (4usize) , self . PID (5usize) , self . PID (6usize) , self . PID (7usize) , self . PID (8usize) , self . PID (9usize) , self . PID (10usize) , self . PID (11usize) , self . PID (12usize) , self . PID (13usize) , self . PID (14usize) , self . PID (15usize) , self . PID (16usize) , self . PID (17usize) , self . PID (18usize) , self . PID (19usize) , self . PID (20usize) , self . PID (21usize) , self . PID (22usize) , self . PID (23usize) , self . PID (24usize) , self . PID (25usize) , self . PID (26usize) , self . PID (27usize) , self . PID (28usize) , self . PID (29usize) , self . PID (30usize) , self . PID (31usize))
        }
    }
    #[doc = "Port Set Output"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PSOR(pub u32);
    impl PSOR {
        #[inline(always)]
        pub const fn PTSO(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PTSO(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PSOR {
        #[inline(always)]
        fn default() -> PSOR {
            PSOR(0)
        }
    }
    impl core::fmt::Debug for PSOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PSOR")
                .field("PTSO[0]", &self.PTSO(0usize))
                .field("PTSO[1]", &self.PTSO(1usize))
                .field("PTSO[2]", &self.PTSO(2usize))
                .field("PTSO[3]", &self.PTSO(3usize))
                .field("PTSO[4]", &self.PTSO(4usize))
                .field("PTSO[5]", &self.PTSO(5usize))
                .field("PTSO[6]", &self.PTSO(6usize))
                .field("PTSO[7]", &self.PTSO(7usize))
                .field("PTSO[8]", &self.PTSO(8usize))
                .field("PTSO[9]", &self.PTSO(9usize))
                .field("PTSO[10]", &self.PTSO(10usize))
                .field("PTSO[11]", &self.PTSO(11usize))
                .field("PTSO[12]", &self.PTSO(12usize))
                .field("PTSO[13]", &self.PTSO(13usize))
                .field("PTSO[14]", &self.PTSO(14usize))
                .field("PTSO[15]", &self.PTSO(15usize))
                .field("PTSO[16]", &self.PTSO(16usize))
                .field("PTSO[17]", &self.PTSO(17usize))
                .field("PTSO[18]", &self.PTSO(18usize))
                .field("PTSO[19]", &self.PTSO(19usize))
                .field("PTSO[20]", &self.PTSO(20usize))
                .field("PTSO[21]", &self.PTSO(21usize))
                .field("PTSO[22]", &self.PTSO(22usize))
                .field("PTSO[23]", &self.PTSO(23usize))
                .field("PTSO[24]", &self.PTSO(24usize))
                .field("PTSO[25]", &self.PTSO(25usize))
                .field("PTSO[26]", &self.PTSO(26usize))
                .field("PTSO[27]", &self.PTSO(27usize))
                .field("PTSO[28]", &self.PTSO(28usize))
                .field("PTSO[29]", &self.PTSO(29usize))
                .field("PTSO[30]", &self.PTSO(30usize))
                .field("PTSO[31]", &self.PTSO(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PSOR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PSOR {{ PTSO[0]: {=bool:?}, PTSO[1]: {=bool:?}, PTSO[2]: {=bool:?}, PTSO[3]: {=bool:?}, PTSO[4]: {=bool:?}, PTSO[5]: {=bool:?}, PTSO[6]: {=bool:?}, PTSO[7]: {=bool:?}, PTSO[8]: {=bool:?}, PTSO[9]: {=bool:?}, PTSO[10]: {=bool:?}, PTSO[11]: {=bool:?}, PTSO[12]: {=bool:?}, PTSO[13]: {=bool:?}, PTSO[14]: {=bool:?}, PTSO[15]: {=bool:?}, PTSO[16]: {=bool:?}, PTSO[17]: {=bool:?}, PTSO[18]: {=bool:?}, PTSO[19]: {=bool:?}, PTSO[20]: {=bool:?}, PTSO[21]: {=bool:?}, PTSO[22]: {=bool:?}, PTSO[23]: {=bool:?}, PTSO[24]: {=bool:?}, PTSO[25]: {=bool:?}, PTSO[26]: {=bool:?}, PTSO[27]: {=bool:?}, PTSO[28]: {=bool:?}, PTSO[29]: {=bool:?}, PTSO[30]: {=bool:?}, PTSO[31]: {=bool:?} }}" , self . PTSO (0usize) , self . PTSO (1usize) , self . PTSO (2usize) , self . PTSO (3usize) , self . PTSO (4usize) , self . PTSO (5usize) , self . PTSO (6usize) , self . PTSO (7usize) , self . PTSO (8usize) , self . PTSO (9usize) , self . PTSO (10usize) , self . PTSO (11usize) , self . PTSO (12usize) , self . PTSO (13usize) , self . PTSO (14usize) , self . PTSO (15usize) , self . PTSO (16usize) , self . PTSO (17usize) , self . PTSO (18usize) , self . PTSO (19usize) , self . PTSO (20usize) , self . PTSO (21usize) , self . PTSO (22usize) , self . PTSO (23usize) , self . PTSO (24usize) , self . PTSO (25usize) , self . PTSO (26usize) , self . PTSO (27usize) , self . PTSO (28usize) , self . PTSO (29usize) , self . PTSO (30usize) , self . PTSO (31usize))
        }
    }
    #[doc = "Port Toggle Output"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PTOR(pub u32);
    impl PTOR {
        #[inline(always)]
        pub const fn PTTO(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PTTO(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PTOR {
        #[inline(always)]
        fn default() -> PTOR {
            PTOR(0)
        }
    }
    impl core::fmt::Debug for PTOR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PTOR")
                .field("PTTO[0]", &self.PTTO(0usize))
                .field("PTTO[1]", &self.PTTO(1usize))
                .field("PTTO[2]", &self.PTTO(2usize))
                .field("PTTO[3]", &self.PTTO(3usize))
                .field("PTTO[4]", &self.PTTO(4usize))
                .field("PTTO[5]", &self.PTTO(5usize))
                .field("PTTO[6]", &self.PTTO(6usize))
                .field("PTTO[7]", &self.PTTO(7usize))
                .field("PTTO[8]", &self.PTTO(8usize))
                .field("PTTO[9]", &self.PTTO(9usize))
                .field("PTTO[10]", &self.PTTO(10usize))
                .field("PTTO[11]", &self.PTTO(11usize))
                .field("PTTO[12]", &self.PTTO(12usize))
                .field("PTTO[13]", &self.PTTO(13usize))
                .field("PTTO[14]", &self.PTTO(14usize))
                .field("PTTO[15]", &self.PTTO(15usize))
                .field("PTTO[16]", &self.PTTO(16usize))
                .field("PTTO[17]", &self.PTTO(17usize))
                .field("PTTO[18]", &self.PTTO(18usize))
                .field("PTTO[19]", &self.PTTO(19usize))
                .field("PTTO[20]", &self.PTTO(20usize))
                .field("PTTO[21]", &self.PTTO(21usize))
                .field("PTTO[22]", &self.PTTO(22usize))
                .field("PTTO[23]", &self.PTTO(23usize))
                .field("PTTO[24]", &self.PTTO(24usize))
                .field("PTTO[25]", &self.PTTO(25usize))
                .field("PTTO[26]", &self.PTTO(26usize))
                .field("PTTO[27]", &self.PTTO(27usize))
                .field("PTTO[28]", &self.PTTO(28usize))
                .field("PTTO[29]", &self.PTTO(29usize))
                .field("PTTO[30]", &self.PTTO(30usize))
                .field("PTTO[31]", &self.PTTO(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PTOR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PTOR {{ PTTO[0]: {=bool:?}, PTTO[1]: {=bool:?}, PTTO[2]: {=bool:?}, PTTO[3]: {=bool:?}, PTTO[4]: {=bool:?}, PTTO[5]: {=bool:?}, PTTO[6]: {=bool:?}, PTTO[7]: {=bool:?}, PTTO[8]: {=bool:?}, PTTO[9]: {=bool:?}, PTTO[10]: {=bool:?}, PTTO[11]: {=bool:?}, PTTO[12]: {=bool:?}, PTTO[13]: {=bool:?}, PTTO[14]: {=bool:?}, PTTO[15]: {=bool:?}, PTTO[16]: {=bool:?}, PTTO[17]: {=bool:?}, PTTO[18]: {=bool:?}, PTTO[19]: {=bool:?}, PTTO[20]: {=bool:?}, PTTO[21]: {=bool:?}, PTTO[22]: {=bool:?}, PTTO[23]: {=bool:?}, PTTO[24]: {=bool:?}, PTTO[25]: {=bool:?}, PTTO[26]: {=bool:?}, PTTO[27]: {=bool:?}, PTTO[28]: {=bool:?}, PTTO[29]: {=bool:?}, PTTO[30]: {=bool:?}, PTTO[31]: {=bool:?} }}" , self . PTTO (0usize) , self . PTTO (1usize) , self . PTTO (2usize) , self . PTTO (3usize) , self . PTTO (4usize) , self . PTTO (5usize) , self . PTTO (6usize) , self . PTTO (7usize) , self . PTTO (8usize) , self . PTTO (9usize) , self . PTTO (10usize) , self . PTTO (11usize) , self . PTTO (12usize) , self . PTTO (13usize) , self . PTTO (14usize) , self . PTTO (15usize) , self . PTTO (16usize) , self . PTTO (17usize) , self . PTTO (18usize) , self . PTTO (19usize) , self . PTTO (20usize) , self . PTTO (21usize) , self . PTTO (22usize) , self . PTTO (23usize) , self . PTTO (24usize) , self . PTTO (25usize) , self . PTTO (26usize) , self . PTTO (27usize) , self . PTTO (28usize) , self . PTTO (29usize) , self . PTTO (30usize) , self . PTTO (31usize))
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
