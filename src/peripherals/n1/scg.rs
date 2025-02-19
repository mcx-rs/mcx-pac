#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCG {
    ptr: *mut u8,
}
unsafe impl Send for SCG {}
unsafe impl Sync for SCG {}
impl SCG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIM_LOCK(self) -> crate::common::Reg<regs::TRIM_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<regs::CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn RCCR(self) -> crate::common::Reg<regs::RCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn SOSCCSR(self) -> crate::common::Reg<regs::SOSCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn SOSCCFG(self) -> crate::common::Reg<regs::SOSCCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCCSR(self) -> crate::common::Reg<regs::SIRCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCTCFG(self) -> crate::common::Reg<regs::SIRCTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCTRIM(self) -> crate::common::Reg<regs::SIRCTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn SIRCSTAT(self) -> crate::common::Reg<regs::SIRCSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCCSR(self) -> crate::common::Reg<regs::FIRCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCCFG(self) -> crate::common::Reg<regs::FIRCCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCTCFG(self) -> crate::common::Reg<regs::FIRCTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCTRIM(self) -> crate::common::Reg<regs::FIRCTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[inline(always)]
    pub const fn FIRCSTAT(self) -> crate::common::Reg<regs::FIRCSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[inline(always)]
    pub const fn ROSCCSR(self) -> crate::common::Reg<regs::ROSCCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLCSR(self) -> crate::common::Reg<regs::APLLCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLCTRL(self) -> crate::common::Reg<regs::APLLCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLSTAT(self) -> crate::common::Reg<regs::APLLSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLNDIV(self) -> crate::common::Reg<regs::APLLNDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[inline(always)]
    pub const fn APLLMDIV(self) -> crate::common::Reg<regs::APLLMDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLPDIV(self) -> crate::common::Reg<regs::APLLPDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLLOCK_CNFG(self) -> crate::common::Reg<regs::APLLLOCK_CNFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLSSCGSTAT(self) -> crate::common::Reg<regs::APLLSSCGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLSSCG0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[inline(always)]
    pub const fn APLLSSCG1(self) -> crate::common::Reg<regs::APLLSSCG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[inline(always)]
    pub const fn APLL_OVRD(self) -> crate::common::Reg<regs::APLL_OVRD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLCSR(self) -> crate::common::Reg<regs::SPLLCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLCTRL(self) -> crate::common::Reg<regs::SPLLCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLSTAT(self) -> crate::common::Reg<regs::SPLLSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLNDIV(self) -> crate::common::Reg<regs::SPLLNDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLMDIV(self) -> crate::common::Reg<regs::SPLLMDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLPDIV(self) -> crate::common::Reg<regs::SPLLPDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLLOCK_CNFG(self) -> crate::common::Reg<regs::SPLLLOCK_CNFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLSSCGSTAT(self) -> crate::common::Reg<regs::SPLLSSCGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLSSCG0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLLSSCG1(self) -> crate::common::Reg<regs::SPLLSSCG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[inline(always)]
    pub const fn SPLL_OVRD(self) -> crate::common::Reg<regs::SPLL_OVRD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f4usize) as _) }
    }
    #[inline(always)]
    pub const fn UPLLCSR(self) -> crate::common::Reg<regs::UPLLCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOCSR(self) -> crate::common::Reg<regs::LDOCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[inline(always)]
    pub const fn TROCSR(self) -> crate::common::Reg<regs::TROCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
}
pub mod regs {
    #[doc = "APLL Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLCSR(pub u32);
    impl APLLCSR {
        #[inline(always)]
        pub const fn APLLPWREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLPWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APLLCLKEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLCLKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn APLLSTEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FRM_CLOCKSTABLE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRM_CLOCKSTABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn APLLCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn APLLCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        pub const fn APLL_LOCK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn APLLSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn APLLERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn APLL_LOCK_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLL_LOCK_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for APLLCSR {
        #[inline(always)]
        fn default() -> APLLCSR {
            APLLCSR(0)
        }
    }
    impl core::fmt::Debug for APLLCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLCSR")
                .field("APLLPWREN", &self.APLLPWREN())
                .field("APLLCLKEN", &self.APLLCLKEN())
                .field("APLLSTEN", &self.APLLSTEN())
                .field("FRM_CLOCKSTABLE", &self.FRM_CLOCKSTABLE())
                .field("APLLCM", &self.APLLCM())
                .field("APLLCMRE", &self.APLLCMRE())
                .field("LK", &self.LK())
                .field("APLL_LOCK", &self.APLL_LOCK())
                .field("APLLSEL", &self.APLLSEL())
                .field("APLLERR", &self.APLLERR())
                .field("APLL_LOCK_IE", &self.APLL_LOCK_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APLLCSR {{ APLLPWREN: {=bool:?}, APLLCLKEN: {=bool:?}, APLLSTEN: {=bool:?}, FRM_CLOCKSTABLE: {=bool:?}, APLLCM: {=bool:?}, APLLCMRE: {=bool:?}, LK: {=bool:?}, APLL_LOCK: {=bool:?}, APLLSEL: {=bool:?}, APLLERR: {=bool:?}, APLL_LOCK_IE: {=bool:?} }}" , self . APLLPWREN () , self . APLLCLKEN () , self . APLLSTEN () , self . FRM_CLOCKSTABLE () , self . APLLCM () , self . APLLCMRE () , self . LK () , self . APLL_LOCK () , self . APLLSEL () , self . APLLERR () , self . APLL_LOCK_IE ())
        }
    }
    #[doc = "APLL Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLCTRL(pub u32);
    impl APLLCTRL {
        #[inline(always)]
        pub const fn SELR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn SELI(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[inline(always)]
        pub const fn SELP(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[inline(always)]
        pub const fn BYPASSPOSTDIV2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPOSTDIV2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn LIMUPOFF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIMUPOFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn BANDDIRECT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BANDDIRECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn BYPASSPREDIV(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPREDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn BYPASSPOSTDIV(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPOSTDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn FRM(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn SOURCE(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SOURCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for APLLCTRL {
        #[inline(always)]
        fn default() -> APLLCTRL {
            APLLCTRL(0)
        }
    }
    impl core::fmt::Debug for APLLCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLCTRL")
                .field("SELR", &self.SELR())
                .field("SELI", &self.SELI())
                .field("SELP", &self.SELP())
                .field("BYPASSPOSTDIV2", &self.BYPASSPOSTDIV2())
                .field("LIMUPOFF", &self.LIMUPOFF())
                .field("BANDDIRECT", &self.BANDDIRECT())
                .field("BYPASSPREDIV", &self.BYPASSPREDIV())
                .field("BYPASSPOSTDIV", &self.BYPASSPOSTDIV())
                .field("FRM", &self.FRM())
                .field("SOURCE", &self.SOURCE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APLLCTRL {{ SELR: {=u8:?}, SELI: {=u8:?}, SELP: {=u8:?}, BYPASSPOSTDIV2: {=bool:?}, LIMUPOFF: {=bool:?}, BANDDIRECT: {=bool:?}, BYPASSPREDIV: {=bool:?}, BYPASSPOSTDIV: {=bool:?}, FRM: {=bool:?}, SOURCE: {=u8:?} }}" , self . SELR () , self . SELI () , self . SELP () , self . BYPASSPOSTDIV2 () , self . LIMUPOFF () , self . BANDDIRECT () , self . BYPASSPREDIV () , self . BYPASSPOSTDIV () , self . FRM () , self . SOURCE ())
        }
    }
    #[doc = "APLL LOCK Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLLOCK_CNFG(pub u32);
    impl APLLLOCK_CNFG {
        #[inline(always)]
        pub const fn LOCK_TIME(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_LOCK_TIME(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for APLLLOCK_CNFG {
        #[inline(always)]
        fn default() -> APLLLOCK_CNFG {
            APLLLOCK_CNFG(0)
        }
    }
    impl core::fmt::Debug for APLLLOCK_CNFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLLOCK_CNFG")
                .field("LOCK_TIME", &self.LOCK_TIME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLLOCK_CNFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APLLLOCK_CNFG {{ LOCK_TIME: {=u32:?} }}",
                self.LOCK_TIME()
            )
        }
    }
    #[doc = "APLL M Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLMDIV(pub u32);
    impl APLLMDIV {
        #[inline(always)]
        pub const fn MDIV(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for APLLMDIV {
        #[inline(always)]
        fn default() -> APLLMDIV {
            APLLMDIV(0)
        }
    }
    impl core::fmt::Debug for APLLMDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLMDIV")
                .field("MDIV", &self.MDIV())
                .field("MREQ", &self.MREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLMDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APLLMDIV {{ MDIV: {=u16:?}, MREQ: {=bool:?} }}",
                self.MDIV(),
                self.MREQ()
            )
        }
    }
    #[doc = "APLL N Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLNDIV(pub u32);
    impl APLLNDIV {
        #[inline(always)]
        pub const fn NDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_NDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for APLLNDIV {
        #[inline(always)]
        fn default() -> APLLNDIV {
            APLLNDIV(0)
        }
    }
    impl core::fmt::Debug for APLLNDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLNDIV")
                .field("NDIV", &self.NDIV())
                .field("NREQ", &self.NREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLNDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APLLNDIV {{ NDIV: {=u8:?}, NREQ: {=bool:?} }}",
                self.NDIV(),
                self.NREQ()
            )
        }
    }
    #[doc = "APLL P Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLPDIV(pub u32);
    impl APLLPDIV {
        #[inline(always)]
        pub const fn PDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn PREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for APLLPDIV {
        #[inline(always)]
        fn default() -> APLLPDIV {
            APLLPDIV(0)
        }
    }
    impl core::fmt::Debug for APLLPDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLPDIV")
                .field("PDIV", &self.PDIV())
                .field("PREQ", &self.PREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLPDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APLLPDIV {{ PDIV: {=u8:?}, PREQ: {=bool:?} }}",
                self.PDIV(),
                self.PREQ()
            )
        }
    }
    #[doc = "APLL Spread Spectrum Control 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLSSCG1(pub u32);
    impl APLLSSCG1 {
        #[inline(always)]
        pub const fn SS_MDIV_MSB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_MSB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SS_MDIV_REQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MF(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[inline(always)]
        pub const fn MR(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[inline(always)]
        pub const fn MC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn DITHER(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DITHER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEL_SS_MDIV(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEL_SS_MDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn SS_PD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_PD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for APLLSSCG1 {
        #[inline(always)]
        fn default() -> APLLSSCG1 {
            APLLSSCG1(0)
        }
    }
    impl core::fmt::Debug for APLLSSCG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLSSCG1")
                .field("SS_MDIV_MSB", &self.SS_MDIV_MSB())
                .field("SS_MDIV_REQ", &self.SS_MDIV_REQ())
                .field("MF", &self.MF())
                .field("MR", &self.MR())
                .field("MC", &self.MC())
                .field("DITHER", &self.DITHER())
                .field("SEL_SS_MDIV", &self.SEL_SS_MDIV())
                .field("SS_PD", &self.SS_PD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLSSCG1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APLLSSCG1 {{ SS_MDIV_MSB: {=bool:?}, SS_MDIV_REQ: {=bool:?}, MF: {=u8:?}, MR: {=u8:?}, MC: {=u8:?}, DITHER: {=bool:?}, SEL_SS_MDIV: {=bool:?}, SS_PD: {=bool:?} }}" , self . SS_MDIV_MSB () , self . SS_MDIV_REQ () , self . MF () , self . MR () , self . MC () , self . DITHER () , self . SEL_SS_MDIV () , self . SS_PD ())
        }
    }
    #[doc = "APLL SSCG Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLSSCGSTAT(pub u32);
    impl APLLSSCGSTAT {
        #[inline(always)]
        pub const fn SS_MDIV_ACK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_ACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for APLLSSCGSTAT {
        #[inline(always)]
        fn default() -> APLLSSCGSTAT {
            APLLSSCGSTAT(0)
        }
    }
    impl core::fmt::Debug for APLLSSCGSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLSSCGSTAT")
                .field("SS_MDIV_ACK", &self.SS_MDIV_ACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLSSCGSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "APLLSSCGSTAT {{ SS_MDIV_ACK: {=bool:?} }}",
                self.SS_MDIV_ACK()
            )
        }
    }
    #[doc = "APLL Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLLSTAT(pub u32);
    impl APLLSTAT {
        #[inline(always)]
        pub const fn NDIVACK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MDIVACK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PDIVACK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FRMDET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRMDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for APLLSTAT {
        #[inline(always)]
        fn default() -> APLLSTAT {
            APLLSTAT(0)
        }
    }
    impl core::fmt::Debug for APLLSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLLSTAT")
                .field("NDIVACK", &self.NDIVACK())
                .field("MDIVACK", &self.MDIVACK())
                .field("PDIVACK", &self.PDIVACK())
                .field("FRMDET", &self.FRMDET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLLSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APLLSTAT {{ NDIVACK: {=bool:?}, MDIVACK: {=bool:?}, PDIVACK: {=bool:?}, FRMDET: {=bool:?} }}" , self . NDIVACK () , self . MDIVACK () , self . PDIVACK () , self . FRMDET ())
        }
    }
    #[doc = "APLL Override Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct APLL_OVRD(pub u32);
    impl APLL_OVRD {
        #[inline(always)]
        pub const fn APLLPWREN_OVRD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLPWREN_OVRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APLLCLKEN_OVRD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLCLKEN_OVRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn APLL_OVRD_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLL_OVRD_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for APLL_OVRD {
        #[inline(always)]
        fn default() -> APLL_OVRD {
            APLL_OVRD(0)
        }
    }
    impl core::fmt::Debug for APLL_OVRD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("APLL_OVRD")
                .field("APLLPWREN_OVRD", &self.APLLPWREN_OVRD())
                .field("APLLCLKEN_OVRD", &self.APLLCLKEN_OVRD())
                .field("APLL_OVRD_EN", &self.APLL_OVRD_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for APLL_OVRD {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "APLL_OVRD {{ APLLPWREN_OVRD: {=bool:?}, APLLCLKEN_OVRD: {=bool:?}, APLL_OVRD_EN: {=bool:?} }}" , self . APLLPWREN_OVRD () , self . APLLCLKEN_OVRD () , self . APLL_OVRD_EN ())
        }
    }
    #[doc = "Clock Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSR(pub u32);
    impl CSR {
        #[inline(always)]
        pub const fn SCS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for CSR {
        #[inline(always)]
        fn default() -> CSR {
            CSR(0)
        }
    }
    impl core::fmt::Debug for CSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSR").field("SCS", &self.SCS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CSR {{ SCS: {=u8:?} }}", self.SCS())
        }
    }
    #[doc = "FIRC Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCCFG(pub u32);
    impl FIRCCFG {
        #[inline(always)]
        pub const fn RANGE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RANGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FIRCCFG {
        #[inline(always)]
        fn default() -> FIRCCFG {
            FIRCCFG(0)
        }
    }
    impl core::fmt::Debug for FIRCCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCCFG")
                .field("RANGE", &self.RANGE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FIRCCFG {{ RANGE: {=bool:?} }}", self.RANGE())
        }
    }
    #[doc = "FIRC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCCSR(pub u32);
    impl FIRCCSR {
        #[inline(always)]
        pub const fn FIRCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn FIRCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FIRC_SCLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRC_SCLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn FIRC_FCLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRC_FCLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn FIRCTREN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCTREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn FIRCTRUP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCTRUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn TRIM_LOCK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIM_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn COARSE_TRIM_BYPASS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COARSE_TRIM_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        pub const fn FIRCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn FIRCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn FIRCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn FIRCERR_IE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCERR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn FIRCACC_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCACC_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn FIRCACC(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCACC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FIRCCSR {
        #[inline(always)]
        fn default() -> FIRCCSR {
            FIRCCSR(0)
        }
    }
    impl core::fmt::Debug for FIRCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCCSR")
                .field("FIRCEN", &self.FIRCEN())
                .field("FIRCSTEN", &self.FIRCSTEN())
                .field("FIRC_SCLK_PERIPH_EN", &self.FIRC_SCLK_PERIPH_EN())
                .field("FIRC_FCLK_PERIPH_EN", &self.FIRC_FCLK_PERIPH_EN())
                .field("FIRCTREN", &self.FIRCTREN())
                .field("FIRCTRUP", &self.FIRCTRUP())
                .field("TRIM_LOCK", &self.TRIM_LOCK())
                .field("COARSE_TRIM_BYPASS", &self.COARSE_TRIM_BYPASS())
                .field("LK", &self.LK())
                .field("FIRCVLD", &self.FIRCVLD())
                .field("FIRCSEL", &self.FIRCSEL())
                .field("FIRCERR", &self.FIRCERR())
                .field("FIRCERR_IE", &self.FIRCERR_IE())
                .field("FIRCACC_IE", &self.FIRCACC_IE())
                .field("FIRCACC", &self.FIRCACC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FIRCCSR {{ FIRCEN: {=bool:?}, FIRCSTEN: {=bool:?}, FIRC_SCLK_PERIPH_EN: {=bool:?}, FIRC_FCLK_PERIPH_EN: {=bool:?}, FIRCTREN: {=bool:?}, FIRCTRUP: {=bool:?}, TRIM_LOCK: {=bool:?}, COARSE_TRIM_BYPASS: {=bool:?}, LK: {=bool:?}, FIRCVLD: {=bool:?}, FIRCSEL: {=bool:?}, FIRCERR: {=bool:?}, FIRCERR_IE: {=bool:?}, FIRCACC_IE: {=bool:?}, FIRCACC: {=bool:?} }}" , self . FIRCEN () , self . FIRCSTEN () , self . FIRC_SCLK_PERIPH_EN () , self . FIRC_FCLK_PERIPH_EN () , self . FIRCTREN () , self . FIRCTRUP () , self . TRIM_LOCK () , self . COARSE_TRIM_BYPASS () , self . LK () , self . FIRCVLD () , self . FIRCSEL () , self . FIRCERR () , self . FIRCERR_IE () , self . FIRCACC_IE () , self . FIRCACC ())
        }
    }
    #[doc = "FIRC Auto-trimming Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCSTAT(pub u32);
    impl FIRCSTAT {
        #[inline(always)]
        pub const fn TRIMFINE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMFINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TRIMCOAR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMCOAR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for FIRCSTAT {
        #[inline(always)]
        fn default() -> FIRCSTAT {
            FIRCSTAT(0)
        }
    }
    impl core::fmt::Debug for FIRCSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCSTAT")
                .field("TRIMFINE", &self.TRIMFINE())
                .field("TRIMCOAR", &self.TRIMCOAR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCSTAT {{ TRIMFINE: {=u8:?}, TRIMCOAR: {=u8:?} }}",
                self.TRIMFINE(),
                self.TRIMCOAR()
            )
        }
    }
    #[doc = "FIRC Trim Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCTCFG(pub u32);
    impl FIRCTCFG {
        #[inline(always)]
        pub const fn TRIMSRC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn TRIMDIV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for FIRCTCFG {
        #[inline(always)]
        fn default() -> FIRCTCFG {
            FIRCTCFG(0)
        }
    }
    impl core::fmt::Debug for FIRCTCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCTCFG")
                .field("TRIMSRC", &self.TRIMSRC())
                .field("TRIMDIV", &self.TRIMDIV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCTCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FIRCTCFG {{ TRIMSRC: {=u8:?}, TRIMDIV: {=u8:?} }}",
                self.TRIMSRC(),
                self.TRIMDIV()
            )
        }
    }
    #[doc = "FIRC Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FIRCTRIM(pub u32);
    impl FIRCTRIM {
        #[inline(always)]
        pub const fn TRIMFINE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMFINE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn TRIMCOAR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMCOAR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn TRIMTEMP(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMTEMP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn TRIMSTART(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMSTART(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for FIRCTRIM {
        #[inline(always)]
        fn default() -> FIRCTRIM {
            FIRCTRIM(0)
        }
    }
    impl core::fmt::Debug for FIRCTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FIRCTRIM")
                .field("TRIMFINE", &self.TRIMFINE())
                .field("TRIMCOAR", &self.TRIMCOAR())
                .field("TRIMTEMP", &self.TRIMTEMP())
                .field("TRIMSTART", &self.TRIMSTART())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FIRCTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FIRCTRIM {{ TRIMFINE: {=u8:?}, TRIMCOAR: {=u8:?}, TRIMTEMP: {=u8:?}, TRIMSTART: {=u8:?} }}" , self . TRIMFINE () , self . TRIMCOAR () , self . TRIMTEMP () , self . TRIMSTART ())
        }
    }
    #[doc = "LDO Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOCSR(pub u32);
    impl LDOCSR {
        #[inline(always)]
        pub const fn LDOEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn VOUT_SEL(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_VOUT_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[inline(always)]
        pub const fn LDOBYPASS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDOBYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn VOUT_OK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOUT_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LDOCSR {
        #[inline(always)]
        fn default() -> LDOCSR {
            LDOCSR(0)
        }
    }
    impl core::fmt::Debug for LDOCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOCSR")
                .field("LDOEN", &self.LDOEN())
                .field("VOUT_SEL", &self.VOUT_SEL())
                .field("LDOBYPASS", &self.LDOBYPASS())
                .field("VOUT_OK", &self.VOUT_OK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LDOCSR {{ LDOEN: {=bool:?}, VOUT_SEL: {=u8:?}, LDOBYPASS: {=bool:?}, VOUT_OK: {=bool:?} }}" , self . LDOEN () , self . VOUT_SEL () , self . LDOBYPASS () , self . VOUT_OK ())
        }
    }
    #[doc = "Parameter Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PARAM(pub u32);
    impl PARAM {
        #[inline(always)]
        pub const fn SOSCCLKPRES(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SIRCCLKPRES(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FIRCCLKPRES(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FIRCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ROSCCLKPRES(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn APLLCLKPRES(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APLLCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SPLLCLKPRES(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn UPLLCLKPRES(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn TROCLKPRES(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROCLKPRES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("SOSCCLKPRES", &self.SOSCCLKPRES())
                .field("SIRCCLKPRES", &self.SIRCCLKPRES())
                .field("FIRCCLKPRES", &self.FIRCCLKPRES())
                .field("ROSCCLKPRES", &self.ROSCCLKPRES())
                .field("APLLCLKPRES", &self.APLLCLKPRES())
                .field("SPLLCLKPRES", &self.SPLLCLKPRES())
                .field("UPLLCLKPRES", &self.UPLLCLKPRES())
                .field("TROCLKPRES", &self.TROCLKPRES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PARAM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PARAM {{ SOSCCLKPRES: {=bool:?}, SIRCCLKPRES: {=bool:?}, FIRCCLKPRES: {=bool:?}, ROSCCLKPRES: {=bool:?}, APLLCLKPRES: {=bool:?}, SPLLCLKPRES: {=bool:?}, UPLLCLKPRES: {=bool:?}, TROCLKPRES: {=bool:?} }}" , self . SOSCCLKPRES () , self . SIRCCLKPRES () , self . FIRCCLKPRES () , self . ROSCCLKPRES () , self . APLLCLKPRES () , self . SPLLCLKPRES () , self . UPLLCLKPRES () , self . TROCLKPRES ())
        }
    }
    #[doc = "Run Clock Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RCCR(pub u32);
    impl RCCR {
        #[inline(always)]
        pub const fn SCS(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for RCCR {
        #[inline(always)]
        fn default() -> RCCR {
            RCCR(0)
        }
    }
    impl core::fmt::Debug for RCCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RCCR").field("SCS", &self.SCS()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RCCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RCCR {{ SCS: {=u8:?} }}", self.SCS())
        }
    }
    #[doc = "ROSC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ROSCCSR(pub u32);
    impl ROSCCSR {
        #[inline(always)]
        pub const fn ROSCCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn ROSCCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        pub const fn ROSCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn ROSCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn ROSCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ROSCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for ROSCCSR {
        #[inline(always)]
        fn default() -> ROSCCSR {
            ROSCCSR(0)
        }
    }
    impl core::fmt::Debug for ROSCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ROSCCSR")
                .field("ROSCCM", &self.ROSCCM())
                .field("ROSCCMRE", &self.ROSCCMRE())
                .field("LK", &self.LK())
                .field("ROSCVLD", &self.ROSCVLD())
                .field("ROSCSEL", &self.ROSCSEL())
                .field("ROSCERR", &self.ROSCERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ROSCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ROSCCSR {{ ROSCCM: {=bool:?}, ROSCCMRE: {=bool:?}, LK: {=bool:?}, ROSCVLD: {=bool:?}, ROSCSEL: {=bool:?}, ROSCERR: {=bool:?} }}" , self . ROSCCM () , self . ROSCCMRE () , self . LK () , self . ROSCVLD () , self . ROSCSEL () , self . ROSCERR ())
        }
    }
    #[doc = "SIRC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCCSR(pub u32);
    impl SIRCCSR {
        #[inline(always)]
        pub const fn SIRCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SIRC_CLK_PERIPH_EN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRC_CLK_PERIPH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn SIRCTREN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCTREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SIRCTRUP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCTRUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn TRIM_LOCK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIM_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn COARSE_TRIM_BYPASS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COARSE_TRIM_BYPASS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        pub const fn SIRCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SIRCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SIRCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SIRCERR_IE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SIRCERR_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SIRCCSR {
        #[inline(always)]
        fn default() -> SIRCCSR {
            SIRCCSR(0)
        }
    }
    impl core::fmt::Debug for SIRCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCCSR")
                .field("SIRCSTEN", &self.SIRCSTEN())
                .field("SIRC_CLK_PERIPH_EN", &self.SIRC_CLK_PERIPH_EN())
                .field("SIRCTREN", &self.SIRCTREN())
                .field("SIRCTRUP", &self.SIRCTRUP())
                .field("TRIM_LOCK", &self.TRIM_LOCK())
                .field("COARSE_TRIM_BYPASS", &self.COARSE_TRIM_BYPASS())
                .field("LK", &self.LK())
                .field("SIRCVLD", &self.SIRCVLD())
                .field("SIRCSEL", &self.SIRCSEL())
                .field("SIRCERR", &self.SIRCERR())
                .field("SIRCERR_IE", &self.SIRCERR_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SIRCCSR {{ SIRCSTEN: {=bool:?}, SIRC_CLK_PERIPH_EN: {=bool:?}, SIRCTREN: {=bool:?}, SIRCTRUP: {=bool:?}, TRIM_LOCK: {=bool:?}, COARSE_TRIM_BYPASS: {=bool:?}, LK: {=bool:?}, SIRCVLD: {=bool:?}, SIRCSEL: {=bool:?}, SIRCERR: {=bool:?}, SIRCERR_IE: {=bool:?} }}" , self . SIRCSTEN () , self . SIRC_CLK_PERIPH_EN () , self . SIRCTREN () , self . SIRCTRUP () , self . TRIM_LOCK () , self . COARSE_TRIM_BYPASS () , self . LK () , self . SIRCVLD () , self . SIRCSEL () , self . SIRCERR () , self . SIRCERR_IE ())
        }
    }
    #[doc = "SIRC Auto-trimming Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCSTAT(pub u32);
    impl SIRCSTAT {
        #[inline(always)]
        pub const fn CCOTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CCOTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CLTRIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for SIRCSTAT {
        #[inline(always)]
        fn default() -> SIRCSTAT {
            SIRCSTAT(0)
        }
    }
    impl core::fmt::Debug for SIRCSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCSTAT")
                .field("CCOTRIM", &self.CCOTRIM())
                .field("CLTRIM", &self.CLTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SIRCSTAT {{ CCOTRIM: {=u8:?}, CLTRIM: {=u8:?} }}",
                self.CCOTRIM(),
                self.CLTRIM()
            )
        }
    }
    #[doc = "SIRC Trim Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCTCFG(pub u32);
    impl SIRCTCFG {
        #[inline(always)]
        pub const fn TRIMSRC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn TRIMDIV(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRIMDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for SIRCTCFG {
        #[inline(always)]
        fn default() -> SIRCTCFG {
            SIRCTCFG(0)
        }
    }
    impl core::fmt::Debug for SIRCTCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCTCFG")
                .field("TRIMSRC", &self.TRIMSRC())
                .field("TRIMDIV", &self.TRIMDIV())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCTCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SIRCTCFG {{ TRIMSRC: {=u8:?}, TRIMDIV: {=u8:?} }}",
                self.TRIMSRC(),
                self.TRIMDIV()
            )
        }
    }
    #[doc = "SIRC Trim Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SIRCTRIM(pub u32);
    impl SIRCTRIM {
        #[inline(always)]
        pub const fn CCOTRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CCOTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[inline(always)]
        pub const fn CLTRIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn TCTRIM(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn FVCHTRIM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FVCHTRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for SIRCTRIM {
        #[inline(always)]
        fn default() -> SIRCTRIM {
            SIRCTRIM(0)
        }
    }
    impl core::fmt::Debug for SIRCTRIM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SIRCTRIM")
                .field("CCOTRIM", &self.CCOTRIM())
                .field("CLTRIM", &self.CLTRIM())
                .field("TCTRIM", &self.TCTRIM())
                .field("FVCHTRIM", &self.FVCHTRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SIRCTRIM {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SIRCTRIM {{ CCOTRIM: {=u8:?}, CLTRIM: {=u8:?}, TCTRIM: {=u8:?}, FVCHTRIM: {=u8:?} }}" , self . CCOTRIM () , self . CLTRIM () , self . TCTRIM () , self . FVCHTRIM ())
        }
    }
    #[doc = "SOSC Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SOSCCFG(pub u32);
    impl SOSCCFG {
        #[inline(always)]
        pub const fn EREFS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EREFS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RANGE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RANGE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for SOSCCFG {
        #[inline(always)]
        fn default() -> SOSCCFG {
            SOSCCFG(0)
        }
    }
    impl core::fmt::Debug for SOSCCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SOSCCFG")
                .field("EREFS", &self.EREFS())
                .field("RANGE", &self.RANGE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SOSCCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SOSCCFG {{ EREFS: {=bool:?}, RANGE: {=u8:?} }}",
                self.EREFS(),
                self.RANGE()
            )
        }
    }
    #[doc = "SOSC Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SOSCCSR(pub u32);
    impl SOSCCSR {
        #[inline(always)]
        pub const fn SOSCEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SOSCSTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SOSCCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SOSCCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        pub const fn SOSCVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SOSCSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SOSCERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SOSCVLD_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOSCVLD_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for SOSCCSR {
        #[inline(always)]
        fn default() -> SOSCCSR {
            SOSCCSR(0)
        }
    }
    impl core::fmt::Debug for SOSCCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SOSCCSR")
                .field("SOSCEN", &self.SOSCEN())
                .field("SOSCSTEN", &self.SOSCSTEN())
                .field("SOSCCM", &self.SOSCCM())
                .field("SOSCCMRE", &self.SOSCCMRE())
                .field("LK", &self.LK())
                .field("SOSCVLD", &self.SOSCVLD())
                .field("SOSCSEL", &self.SOSCSEL())
                .field("SOSCERR", &self.SOSCERR())
                .field("SOSCVLD_IE", &self.SOSCVLD_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SOSCCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SOSCCSR {{ SOSCEN: {=bool:?}, SOSCSTEN: {=bool:?}, SOSCCM: {=bool:?}, SOSCCMRE: {=bool:?}, LK: {=bool:?}, SOSCVLD: {=bool:?}, SOSCSEL: {=bool:?}, SOSCERR: {=bool:?}, SOSCVLD_IE: {=bool:?} }}" , self . SOSCEN () , self . SOSCSTEN () , self . SOSCCM () , self . SOSCCMRE () , self . LK () , self . SOSCVLD () , self . SOSCSEL () , self . SOSCERR () , self . SOSCVLD_IE ())
        }
    }
    #[doc = "SPLL Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLCSR(pub u32);
    impl SPLLCSR {
        #[inline(always)]
        pub const fn SPLLPWREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLPWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SPLLCLKEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLCLKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SPLLSTEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn FRM_CLOCKSTABLE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRM_CLOCKSTABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SPLLCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn SPLLCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        pub const fn SPLL_LOCK(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLL_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn SPLLSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn SPLLERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn SPLL_LOCK_IE(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLL_LOCK_IE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for SPLLCSR {
        #[inline(always)]
        fn default() -> SPLLCSR {
            SPLLCSR(0)
        }
    }
    impl core::fmt::Debug for SPLLCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLCSR")
                .field("SPLLPWREN", &self.SPLLPWREN())
                .field("SPLLCLKEN", &self.SPLLCLKEN())
                .field("SPLLSTEN", &self.SPLLSTEN())
                .field("FRM_CLOCKSTABLE", &self.FRM_CLOCKSTABLE())
                .field("SPLLCM", &self.SPLLCM())
                .field("SPLLCMRE", &self.SPLLCMRE())
                .field("LK", &self.LK())
                .field("SPLL_LOCK", &self.SPLL_LOCK())
                .field("SPLLSEL", &self.SPLLSEL())
                .field("SPLLERR", &self.SPLLERR())
                .field("SPLL_LOCK_IE", &self.SPLL_LOCK_IE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SPLLCSR {{ SPLLPWREN: {=bool:?}, SPLLCLKEN: {=bool:?}, SPLLSTEN: {=bool:?}, FRM_CLOCKSTABLE: {=bool:?}, SPLLCM: {=bool:?}, SPLLCMRE: {=bool:?}, LK: {=bool:?}, SPLL_LOCK: {=bool:?}, SPLLSEL: {=bool:?}, SPLLERR: {=bool:?}, SPLL_LOCK_IE: {=bool:?} }}" , self . SPLLPWREN () , self . SPLLCLKEN () , self . SPLLSTEN () , self . FRM_CLOCKSTABLE () , self . SPLLCM () , self . SPLLCMRE () , self . LK () , self . SPLL_LOCK () , self . SPLLSEL () , self . SPLLERR () , self . SPLL_LOCK_IE ())
        }
    }
    #[doc = "SPLL Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLCTRL(pub u32);
    impl SPLLCTRL {
        #[inline(always)]
        pub const fn SELR(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn SELI(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELI(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[inline(always)]
        pub const fn SELP(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SELP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[inline(always)]
        pub const fn BYPASSPOSTDIV2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPOSTDIV2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn LIMUPOFF(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIMUPOFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn BANDDIRECT(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BANDDIRECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn BYPASSPREDIV(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPREDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn BYPASSPOSTDIV(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BYPASSPOSTDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn FRM(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn SOURCE(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SOURCE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for SPLLCTRL {
        #[inline(always)]
        fn default() -> SPLLCTRL {
            SPLLCTRL(0)
        }
    }
    impl core::fmt::Debug for SPLLCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLCTRL")
                .field("SELR", &self.SELR())
                .field("SELI", &self.SELI())
                .field("SELP", &self.SELP())
                .field("BYPASSPOSTDIV2", &self.BYPASSPOSTDIV2())
                .field("LIMUPOFF", &self.LIMUPOFF())
                .field("BANDDIRECT", &self.BANDDIRECT())
                .field("BYPASSPREDIV", &self.BYPASSPREDIV())
                .field("BYPASSPOSTDIV", &self.BYPASSPOSTDIV())
                .field("FRM", &self.FRM())
                .field("SOURCE", &self.SOURCE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SPLLCTRL {{ SELR: {=u8:?}, SELI: {=u8:?}, SELP: {=u8:?}, BYPASSPOSTDIV2: {=bool:?}, LIMUPOFF: {=bool:?}, BANDDIRECT: {=bool:?}, BYPASSPREDIV: {=bool:?}, BYPASSPOSTDIV: {=bool:?}, FRM: {=bool:?}, SOURCE: {=u8:?} }}" , self . SELR () , self . SELI () , self . SELP () , self . BYPASSPOSTDIV2 () , self . LIMUPOFF () , self . BANDDIRECT () , self . BYPASSPREDIV () , self . BYPASSPOSTDIV () , self . FRM () , self . SOURCE ())
        }
    }
    #[doc = "SPLL LOCK Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLLOCK_CNFG(pub u32);
    impl SPLLLOCK_CNFG {
        #[inline(always)]
        pub const fn LOCK_TIME(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_LOCK_TIME(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for SPLLLOCK_CNFG {
        #[inline(always)]
        fn default() -> SPLLLOCK_CNFG {
            SPLLLOCK_CNFG(0)
        }
    }
    impl core::fmt::Debug for SPLLLOCK_CNFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLLOCK_CNFG")
                .field("LOCK_TIME", &self.LOCK_TIME())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLLOCK_CNFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SPLLLOCK_CNFG {{ LOCK_TIME: {=u32:?} }}",
                self.LOCK_TIME()
            )
        }
    }
    #[doc = "SPLL M Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLMDIV(pub u32);
    impl SPLLMDIV {
        #[inline(always)]
        pub const fn MDIV(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_MDIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn MREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SPLLMDIV {
        #[inline(always)]
        fn default() -> SPLLMDIV {
            SPLLMDIV(0)
        }
    }
    impl core::fmt::Debug for SPLLMDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLMDIV")
                .field("MDIV", &self.MDIV())
                .field("MREQ", &self.MREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLMDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SPLLMDIV {{ MDIV: {=u16:?}, MREQ: {=bool:?} }}",
                self.MDIV(),
                self.MREQ()
            )
        }
    }
    #[doc = "SPLL N Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLNDIV(pub u32);
    impl SPLLNDIV {
        #[inline(always)]
        pub const fn NDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_NDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SPLLNDIV {
        #[inline(always)]
        fn default() -> SPLLNDIV {
            SPLLNDIV(0)
        }
    }
    impl core::fmt::Debug for SPLLNDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLNDIV")
                .field("NDIV", &self.NDIV())
                .field("NREQ", &self.NREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLNDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SPLLNDIV {{ NDIV: {=u8:?}, NREQ: {=bool:?} }}",
                self.NDIV(),
                self.NREQ()
            )
        }
    }
    #[doc = "SPLL P Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLPDIV(pub u32);
    impl SPLLPDIV {
        #[inline(always)]
        pub const fn PDIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn PREQ(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SPLLPDIV {
        #[inline(always)]
        fn default() -> SPLLPDIV {
            SPLLPDIV(0)
        }
    }
    impl core::fmt::Debug for SPLLPDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLPDIV")
                .field("PDIV", &self.PDIV())
                .field("PREQ", &self.PREQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLPDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SPLLPDIV {{ PDIV: {=u8:?}, PREQ: {=bool:?} }}",
                self.PDIV(),
                self.PREQ()
            )
        }
    }
    #[doc = "SPLL Spread Spectrum Control 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLSSCG1(pub u32);
    impl SPLLSSCG1 {
        #[inline(always)]
        pub const fn SS_MDIV_MSB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_MSB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SS_MDIV_REQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MF(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[inline(always)]
        pub const fn MR(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_MR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[inline(always)]
        pub const fn MC(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn DITHER(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DITHER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEL_SS_MDIV(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEL_SS_MDIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn SS_PD(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_PD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SPLLSSCG1 {
        #[inline(always)]
        fn default() -> SPLLSSCG1 {
            SPLLSSCG1(0)
        }
    }
    impl core::fmt::Debug for SPLLSSCG1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLSSCG1")
                .field("SS_MDIV_MSB", &self.SS_MDIV_MSB())
                .field("SS_MDIV_REQ", &self.SS_MDIV_REQ())
                .field("MF", &self.MF())
                .field("MR", &self.MR())
                .field("MC", &self.MC())
                .field("DITHER", &self.DITHER())
                .field("SEL_SS_MDIV", &self.SEL_SS_MDIV())
                .field("SS_PD", &self.SS_PD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLSSCG1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SPLLSSCG1 {{ SS_MDIV_MSB: {=bool:?}, SS_MDIV_REQ: {=bool:?}, MF: {=u8:?}, MR: {=u8:?}, MC: {=u8:?}, DITHER: {=bool:?}, SEL_SS_MDIV: {=bool:?}, SS_PD: {=bool:?} }}" , self . SS_MDIV_MSB () , self . SS_MDIV_REQ () , self . MF () , self . MR () , self . MC () , self . DITHER () , self . SEL_SS_MDIV () , self . SS_PD ())
        }
    }
    #[doc = "SPLL SSCG Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLSSCGSTAT(pub u32);
    impl SPLLSSCGSTAT {
        #[inline(always)]
        pub const fn SS_MDIV_ACK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SS_MDIV_ACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SPLLSSCGSTAT {
        #[inline(always)]
        fn default() -> SPLLSSCGSTAT {
            SPLLSSCGSTAT(0)
        }
    }
    impl core::fmt::Debug for SPLLSSCGSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLSSCGSTAT")
                .field("SS_MDIV_ACK", &self.SS_MDIV_ACK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLSSCGSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SPLLSSCGSTAT {{ SS_MDIV_ACK: {=bool:?} }}",
                self.SS_MDIV_ACK()
            )
        }
    }
    #[doc = "SPLL Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLLSTAT(pub u32);
    impl SPLLSTAT {
        #[inline(always)]
        pub const fn NDIVACK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn MDIVACK(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn PDIVACK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PDIVACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn FRMDET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRMDET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for SPLLSTAT {
        #[inline(always)]
        fn default() -> SPLLSTAT {
            SPLLSTAT(0)
        }
    }
    impl core::fmt::Debug for SPLLSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLLSTAT")
                .field("NDIVACK", &self.NDIVACK())
                .field("MDIVACK", &self.MDIVACK())
                .field("PDIVACK", &self.PDIVACK())
                .field("FRMDET", &self.FRMDET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLLSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SPLLSTAT {{ NDIVACK: {=bool:?}, MDIVACK: {=bool:?}, PDIVACK: {=bool:?}, FRMDET: {=bool:?} }}" , self . NDIVACK () , self . MDIVACK () , self . PDIVACK () , self . FRMDET ())
        }
    }
    #[doc = "SPLL Override Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPLL_OVRD(pub u32);
    impl SPLL_OVRD {
        #[inline(always)]
        pub const fn SPLLPWREN_OVRD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLPWREN_OVRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SPLLCLKEN_OVRD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLLCLKEN_OVRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SPLL_OVRD_EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLL_OVRD_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SPLL_OVRD {
        #[inline(always)]
        fn default() -> SPLL_OVRD {
            SPLL_OVRD(0)
        }
    }
    impl core::fmt::Debug for SPLL_OVRD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SPLL_OVRD")
                .field("SPLLPWREN_OVRD", &self.SPLLPWREN_OVRD())
                .field("SPLLCLKEN_OVRD", &self.SPLLCLKEN_OVRD())
                .field("SPLL_OVRD_EN", &self.SPLL_OVRD_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SPLL_OVRD {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SPLL_OVRD {{ SPLLPWREN_OVRD: {=bool:?}, SPLLCLKEN_OVRD: {=bool:?}, SPLL_OVRD_EN: {=bool:?} }}" , self . SPLLPWREN_OVRD () , self . SPLLCLKEN_OVRD () , self . SPLL_OVRD_EN ())
        }
    }
    #[doc = "Trim Lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRIM_LOCK(pub u32);
    impl TRIM_LOCK {
        #[inline(always)]
        pub const fn TRIM_UNLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRIM_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IFR_DISABLE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IFR_DISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TRIM_LOCK_KEY(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_TRIM_LOCK_KEY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for TRIM_LOCK {
        #[inline(always)]
        fn default() -> TRIM_LOCK {
            TRIM_LOCK(0)
        }
    }
    impl core::fmt::Debug for TRIM_LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TRIM_LOCK")
                .field("TRIM_UNLOCK", &self.TRIM_UNLOCK())
                .field("IFR_DISABLE", &self.IFR_DISABLE())
                .field("TRIM_LOCK_KEY", &self.TRIM_LOCK_KEY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TRIM_LOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TRIM_LOCK {{ TRIM_UNLOCK: {=bool:?}, IFR_DISABLE: {=bool:?}, TRIM_LOCK_KEY: {=u16:?} }}" , self . TRIM_UNLOCK () , self . IFR_DISABLE () , self . TRIM_LOCK_KEY ())
        }
    }
    #[doc = "TRO Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TROCSR(pub u32);
    impl TROCSR {
        #[inline(always)]
        pub const fn TROCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn TROCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn TRO_REFCLK_SEL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_TRO_REFCLK_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
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
        pub const fn TROVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn TROSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn TROERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TROERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for TROCSR {
        #[inline(always)]
        fn default() -> TROCSR {
            TROCSR(0)
        }
    }
    impl core::fmt::Debug for TROCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TROCSR")
                .field("TROCM", &self.TROCM())
                .field("TROCMRE", &self.TROCMRE())
                .field("TRO_REFCLK_SEL", &self.TRO_REFCLK_SEL())
                .field("LK", &self.LK())
                .field("TROVLD", &self.TROVLD())
                .field("TROSEL", &self.TROSEL())
                .field("TROERR", &self.TROERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TROCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TROCSR {{ TROCM: {=bool:?}, TROCMRE: {=bool:?}, TRO_REFCLK_SEL: {=u8:?}, LK: {=bool:?}, TROVLD: {=bool:?}, TROSEL: {=bool:?}, TROERR: {=bool:?} }}" , self . TROCM () , self . TROCMRE () , self . TRO_REFCLK_SEL () , self . LK () , self . TROVLD () , self . TROSEL () , self . TROERR ())
        }
    }
    #[doc = "UPLL Control Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UPLLCSR(pub u32);
    impl UPLLCSR {
        #[inline(always)]
        pub const fn UPLLCM(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn UPLLCMRE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLCMRE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
        pub const fn UPLLVLD(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLVLD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn UPLLSEL(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn UPLLERR(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn UPLLVOR(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLVOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UPLLVORE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UPLLVORE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for UPLLCSR {
        #[inline(always)]
        fn default() -> UPLLCSR {
            UPLLCSR(0)
        }
    }
    impl core::fmt::Debug for UPLLCSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UPLLCSR")
                .field("UPLLCM", &self.UPLLCM())
                .field("UPLLCMRE", &self.UPLLCMRE())
                .field("LK", &self.LK())
                .field("UPLLVLD", &self.UPLLVLD())
                .field("UPLLSEL", &self.UPLLSEL())
                .field("UPLLERR", &self.UPLLERR())
                .field("UPLLVOR", &self.UPLLVOR())
                .field("UPLLVORE", &self.UPLLVORE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UPLLCSR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UPLLCSR {{ UPLLCM: {=bool:?}, UPLLCMRE: {=bool:?}, LK: {=bool:?}, UPLLVLD: {=bool:?}, UPLLSEL: {=bool:?}, UPLLERR: {=bool:?}, UPLLVOR: {=bool:?}, UPLLVORE: {=bool:?} }}" , self . UPLLCM () , self . UPLLCMRE () , self . LK () , self . UPLLVLD () , self . UPLLSEL () , self . UPLLERR () , self . UPLLVOR () , self . UPLLVORE ())
        }
    }
}
