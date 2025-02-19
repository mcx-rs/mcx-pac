#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMARTDMA {
    ptr: *mut u8,
}
unsafe impl Send for SMARTDMA {}
unsafe impl Sync for SMARTDMA {}
impl SMARTDMA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn BOOTADR(self) -> crate::common::Reg<regs::BOOTADR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn PC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn BREAK_ADDR(self) -> crate::common::Reg<regs::BREAK_ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn BREAK_VECT(self) -> crate::common::Reg<regs::BREAK_VECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn EMER_VECT(self) -> crate::common::Reg<regs::EMER_VECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn EMER_SEL(self) -> crate::common::Reg<regs::EMER_SEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn ARM2EZH(self) -> crate::common::Reg<regs::ARM2EZH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn EZH2ARM(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn PENDTRAP(self) -> crate::common::Reg<regs::PENDTRAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "ARM to EZH Interrupt Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ARM2EZH(pub u32);
    impl ARM2EZH {
        #[inline(always)]
        pub const fn IE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn GP(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_GP(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for ARM2EZH {
        #[inline(always)]
        fn default() -> ARM2EZH {
            ARM2EZH(0)
        }
    }
    impl core::fmt::Debug for ARM2EZH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ARM2EZH")
                .field("IE", &self.IE())
                .field("GP", &self.GP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ARM2EZH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ARM2EZH {{ IE: {=u8:?}, GP: {=u32:?} }}",
                self.IE(),
                self.GP()
            )
        }
    }
    #[doc = "Boot Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BOOTADR(pub u32);
    impl BOOTADR {
        #[inline(always)]
        pub const fn ADDR(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ADDR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for BOOTADR {
        #[inline(always)]
        fn default() -> BOOTADR {
            BOOTADR(0)
        }
    }
    impl core::fmt::Debug for BOOTADR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BOOTADR")
                .field("ADDR", &self.ADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BOOTADR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BOOTADR {{ ADDR: {=u32:?} }}", self.ADDR())
        }
    }
    #[doc = "Breakpoint Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BREAK_ADDR(pub u32);
    impl BREAK_ADDR {
        #[inline(always)]
        pub const fn ADDR(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ADDR(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for BREAK_ADDR {
        #[inline(always)]
        fn default() -> BREAK_ADDR {
            BREAK_ADDR(0)
        }
    }
    impl core::fmt::Debug for BREAK_ADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BREAK_ADDR")
                .field("ADDR", &self.ADDR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BREAK_ADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BREAK_ADDR {{ ADDR: {=u32:?} }}", self.ADDR())
        }
    }
    #[doc = "Breakpoint Vector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BREAK_VECT(pub u32);
    impl BREAK_VECT {
        #[inline(always)]
        pub const fn VEC(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_VEC(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for BREAK_VECT {
        #[inline(always)]
        fn default() -> BREAK_VECT {
            BREAK_VECT(0)
        }
    }
    impl core::fmt::Debug for BREAK_VECT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BREAK_VECT")
                .field("VEC", &self.VEC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BREAK_VECT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BREAK_VECT {{ VEC: {=u32:?} }}", self.VEC())
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTRL(pub u32);
    impl CTRL {
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn EXF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EXF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ERRDIS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERRDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn BUFEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SYNCEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SYNCEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn WKEY(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_WKEY(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("START", &self.START())
                .field("EXF", &self.EXF())
                .field("ERRDIS", &self.ERRDIS())
                .field("BUFEN", &self.BUFEN())
                .field("SYNCEN", &self.SYNCEN())
                .field("WKEY", &self.WKEY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTRL {{ START: {=bool:?}, EXF: {=bool:?}, ERRDIS: {=bool:?}, BUFEN: {=bool:?}, SYNCEN: {=bool:?}, WKEY: {=u16:?} }}" , self . START () , self . EXF () , self . ERRDIS () , self . BUFEN () , self . SYNCEN () , self . WKEY ())
        }
    }
    #[doc = "Emergency Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EMER_SEL(pub u32);
    impl EMER_SEL {
        #[inline(always)]
        pub const fn EN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RQ(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for EMER_SEL {
        #[inline(always)]
        fn default() -> EMER_SEL {
            EMER_SEL(0)
        }
    }
    impl core::fmt::Debug for EMER_SEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EMER_SEL")
                .field("EN", &self.EN())
                .field("RQ", &self.RQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EMER_SEL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EMER_SEL {{ EN: {=bool:?}, RQ: {=bool:?} }}",
                self.EN(),
                self.RQ()
            )
        }
    }
    #[doc = "Emergency Vector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EMER_VECT(pub u32);
    impl EMER_VECT {
        #[inline(always)]
        pub const fn VEC(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_VEC(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for EMER_VECT {
        #[inline(always)]
        fn default() -> EMER_VECT {
            EMER_VECT(0)
        }
    }
    impl core::fmt::Debug for EMER_VECT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EMER_VECT")
                .field("VEC", &self.VEC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EMER_VECT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EMER_VECT {{ VEC: {=u32:?} }}", self.VEC())
        }
    }
    #[doc = "Pending Trap Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PENDTRAP(pub u32);
    impl PENDTRAP {
        #[inline(always)]
        pub const fn STATUS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn POL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_POL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn EN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for PENDTRAP {
        #[inline(always)]
        fn default() -> PENDTRAP {
            PENDTRAP(0)
        }
    }
    impl core::fmt::Debug for PENDTRAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PENDTRAP")
                .field("STATUS", &self.STATUS())
                .field("POL", &self.POL())
                .field("EN", &self.EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PENDTRAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PENDTRAP {{ STATUS: {=u8:?}, POL: {=u8:?}, EN: {=u8:?} }}",
                self.STATUS(),
                self.POL(),
                self.EN()
            )
        }
    }
}
