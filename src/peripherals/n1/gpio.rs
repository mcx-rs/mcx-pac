#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
                .field(
                    "PTCO",
                    &[
                        self.PTCO(0usize),
                        self.PTCO(1usize),
                        self.PTCO(2usize),
                        self.PTCO(3usize),
                        self.PTCO(4usize),
                        self.PTCO(5usize),
                        self.PTCO(6usize),
                        self.PTCO(7usize),
                        self.PTCO(8usize),
                        self.PTCO(9usize),
                        self.PTCO(10usize),
                        self.PTCO(11usize),
                        self.PTCO(12usize),
                        self.PTCO(13usize),
                        self.PTCO(14usize),
                        self.PTCO(15usize),
                        self.PTCO(16usize),
                        self.PTCO(17usize),
                        self.PTCO(18usize),
                        self.PTCO(19usize),
                        self.PTCO(20usize),
                        self.PTCO(21usize),
                        self.PTCO(22usize),
                        self.PTCO(23usize),
                        self.PTCO(24usize),
                        self.PTCO(25usize),
                        self.PTCO(26usize),
                        self.PTCO(27usize),
                        self.PTCO(28usize),
                        self.PTCO(29usize),
                        self.PTCO(30usize),
                        self.PTCO(31usize),
                    ],
                )
                .finish()
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
                .field(
                    "PDD",
                    &[
                        self.PDD(0usize),
                        self.PDD(1usize),
                        self.PDD(2usize),
                        self.PDD(3usize),
                        self.PDD(4usize),
                        self.PDD(5usize),
                        self.PDD(6usize),
                        self.PDD(7usize),
                        self.PDD(8usize),
                        self.PDD(9usize),
                        self.PDD(10usize),
                        self.PDD(11usize),
                        self.PDD(12usize),
                        self.PDD(13usize),
                        self.PDD(14usize),
                        self.PDD(15usize),
                        self.PDD(16usize),
                        self.PDD(17usize),
                        self.PDD(18usize),
                        self.PDD(19usize),
                        self.PDD(20usize),
                        self.PDD(21usize),
                        self.PDD(22usize),
                        self.PDD(23usize),
                        self.PDD(24usize),
                        self.PDD(25usize),
                        self.PDD(26usize),
                        self.PDD(27usize),
                        self.PDD(28usize),
                        self.PDD(29usize),
                        self.PDD(30usize),
                        self.PDD(31usize),
                    ],
                )
                .finish()
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
                .field(
                    "PDI",
                    &[
                        self.PDI(0usize),
                        self.PDI(1usize),
                        self.PDI(2usize),
                        self.PDI(3usize),
                        self.PDI(4usize),
                        self.PDI(5usize),
                        self.PDI(6usize),
                        self.PDI(7usize),
                        self.PDI(8usize),
                        self.PDI(9usize),
                        self.PDI(10usize),
                        self.PDI(11usize),
                        self.PDI(12usize),
                        self.PDI(13usize),
                        self.PDI(14usize),
                        self.PDI(15usize),
                        self.PDI(16usize),
                        self.PDI(17usize),
                        self.PDI(18usize),
                        self.PDI(19usize),
                        self.PDI(20usize),
                        self.PDI(21usize),
                        self.PDI(22usize),
                        self.PDI(23usize),
                        self.PDI(24usize),
                        self.PDI(25usize),
                        self.PDI(26usize),
                        self.PDI(27usize),
                        self.PDI(28usize),
                        self.PDI(29usize),
                        self.PDI(30usize),
                        self.PDI(31usize),
                    ],
                )
                .finish()
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
                .field(
                    "PDO",
                    &[
                        self.PDO(0usize),
                        self.PDO(1usize),
                        self.PDO(2usize),
                        self.PDO(3usize),
                        self.PDO(4usize),
                        self.PDO(5usize),
                        self.PDO(6usize),
                        self.PDO(7usize),
                        self.PDO(8usize),
                        self.PDO(9usize),
                        self.PDO(10usize),
                        self.PDO(11usize),
                        self.PDO(12usize),
                        self.PDO(13usize),
                        self.PDO(14usize),
                        self.PDO(15usize),
                        self.PDO(16usize),
                        self.PDO(17usize),
                        self.PDO(18usize),
                        self.PDO(19usize),
                        self.PDO(20usize),
                        self.PDO(21usize),
                        self.PDO(22usize),
                        self.PDO(23usize),
                        self.PDO(24usize),
                        self.PDO(25usize),
                        self.PDO(26usize),
                        self.PDO(27usize),
                        self.PDO(28usize),
                        self.PDO(29usize),
                        self.PDO(30usize),
                        self.PDO(31usize),
                    ],
                )
                .finish()
        }
    }
    #[doc = "Pin Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PDR(pub u32);
    impl PDR {
        #[inline(always)]
        pub const fn PD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
                .field(
                    "PID",
                    &[
                        self.PID(0usize),
                        self.PID(1usize),
                        self.PID(2usize),
                        self.PID(3usize),
                        self.PID(4usize),
                        self.PID(5usize),
                        self.PID(6usize),
                        self.PID(7usize),
                        self.PID(8usize),
                        self.PID(9usize),
                        self.PID(10usize),
                        self.PID(11usize),
                        self.PID(12usize),
                        self.PID(13usize),
                        self.PID(14usize),
                        self.PID(15usize),
                        self.PID(16usize),
                        self.PID(17usize),
                        self.PID(18usize),
                        self.PID(19usize),
                        self.PID(20usize),
                        self.PID(21usize),
                        self.PID(22usize),
                        self.PID(23usize),
                        self.PID(24usize),
                        self.PID(25usize),
                        self.PID(26usize),
                        self.PID(27usize),
                        self.PID(28usize),
                        self.PID(29usize),
                        self.PID(30usize),
                        self.PID(31usize),
                    ],
                )
                .finish()
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
                .field(
                    "PTSO",
                    &[
                        self.PTSO(0usize),
                        self.PTSO(1usize),
                        self.PTSO(2usize),
                        self.PTSO(3usize),
                        self.PTSO(4usize),
                        self.PTSO(5usize),
                        self.PTSO(6usize),
                        self.PTSO(7usize),
                        self.PTSO(8usize),
                        self.PTSO(9usize),
                        self.PTSO(10usize),
                        self.PTSO(11usize),
                        self.PTSO(12usize),
                        self.PTSO(13usize),
                        self.PTSO(14usize),
                        self.PTSO(15usize),
                        self.PTSO(16usize),
                        self.PTSO(17usize),
                        self.PTSO(18usize),
                        self.PTSO(19usize),
                        self.PTSO(20usize),
                        self.PTSO(21usize),
                        self.PTSO(22usize),
                        self.PTSO(23usize),
                        self.PTSO(24usize),
                        self.PTSO(25usize),
                        self.PTSO(26usize),
                        self.PTSO(27usize),
                        self.PTSO(28usize),
                        self.PTSO(29usize),
                        self.PTSO(30usize),
                        self.PTSO(31usize),
                    ],
                )
                .finish()
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
                .field(
                    "PTTO",
                    &[
                        self.PTTO(0usize),
                        self.PTTO(1usize),
                        self.PTTO(2usize),
                        self.PTTO(3usize),
                        self.PTTO(4usize),
                        self.PTTO(5usize),
                        self.PTTO(6usize),
                        self.PTTO(7usize),
                        self.PTTO(8usize),
                        self.PTTO(9usize),
                        self.PTTO(10usize),
                        self.PTTO(11usize),
                        self.PTTO(12usize),
                        self.PTTO(13usize),
                        self.PTTO(14usize),
                        self.PTTO(15usize),
                        self.PTTO(16usize),
                        self.PTTO(17usize),
                        self.PTTO(18usize),
                        self.PTTO(19usize),
                        self.PTTO(20usize),
                        self.PTTO(21usize),
                        self.PTTO(22usize),
                        self.PTTO(23usize),
                        self.PTTO(24usize),
                        self.PTTO(25usize),
                        self.PTTO(26usize),
                        self.PTTO(27usize),
                        self.PTTO(28usize),
                        self.PTTO(29usize),
                        self.PTTO(30usize),
                        self.PTTO(31usize),
                    ],
                )
                .finish()
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
}
