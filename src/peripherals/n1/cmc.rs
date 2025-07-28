#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMC {
    ptr: *mut u8,
}
unsafe impl Send for CMC {}
unsafe impl Sync for CMC {}
impl CMC {
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
    pub const fn CKCTRL(self) -> crate::common::Reg<regs::CKCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CKSTAT(self) -> crate::common::Reg<regs::CKSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn PMPROT(self) -> crate::common::Reg<regs::PMPROT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn GPMCTRL(self) -> crate::common::Reg<regs::GPMCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PMCTRL(self, n: usize) -> crate::common::Reg<regs::PMCTRL, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRS(self) -> crate::common::Reg<regs::SRS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn RPC(self) -> crate::common::Reg<regs::RPC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn SSRS(self) -> crate::common::Reg<regs::SSRS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn SRIE(self) -> crate::common::Reg<regs::SRIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn SRIF(self) -> crate::common::Reg<regs::SRIF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn RSTCNT(self) -> crate::common::Reg<regs::RSTCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn MR(self, n: usize) -> crate::common::Reg<regs::MR, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FM(self, n: usize) -> crate::common::Reg<regs::FM, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMDIS(self, n: usize) -> crate::common::Reg<regs::SRAMDIS, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAMRET(self, n: usize) -> crate::common::Reg<regs::SRAMRET, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLASHCR(self) -> crate::common::Reg<regs::FLASHCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn BSR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn BLR(self) -> crate::common::Reg<regs::BLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn CORECTL(self) -> crate::common::Reg<regs::CORECTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn DBGCTL(self) -> crate::common::Reg<regs::DBGCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
}
pub mod regs {
    #[doc = "BootROM Lock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BLR(pub u32);
    impl BLR {
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for BLR {
        #[inline(always)]
        fn default() -> BLR {
            BLR(0)
        }
    }
    impl core::fmt::Debug for BLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BLR").field("LOCK", &self.LOCK()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BLR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BLR {{ LOCK: {=u8:?} }}", self.LOCK())
        }
    }
    #[doc = "Clock Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CKCTRL(pub u32);
    impl CKCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn CKMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CKMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CKCTRL {
        #[inline(always)]
        fn default() -> CKCTRL {
            CKCTRL(0)
        }
    }
    impl core::fmt::Debug for CKCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CKCTRL")
                .field("CKMODE", &self.CKMODE())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CKCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CKCTRL {{ CKMODE: {=u8:?}, LOCK: {=bool:?} }}",
                self.CKMODE(),
                self.LOCK()
            )
        }
    }
    #[doc = "Clock Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CKSTAT(pub u32);
    impl CKSTAT {
        #[must_use]
        #[inline(always)]
        pub const fn CKMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CKMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WAKEUP(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WAKEUP(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VALID(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VALID(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CKSTAT {
        #[inline(always)]
        fn default() -> CKSTAT {
            CKSTAT(0)
        }
    }
    impl core::fmt::Debug for CKSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CKSTAT")
                .field("CKMODE", &self.CKMODE())
                .field("WAKEUP", &self.WAKEUP())
                .field("VALID", &self.VALID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CKSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CKSTAT {{ CKMODE: {=u8:?}, WAKEUP: {=u8:?}, VALID: {=bool:?} }}",
                self.CKMODE(),
                self.WAKEUP(),
                self.VALID()
            )
        }
    }
    #[doc = "Core Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CORECTL(pub u32);
    impl CORECTL {
        #[must_use]
        #[inline(always)]
        pub const fn NPIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NPIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CORECTL {
        #[inline(always)]
        fn default() -> CORECTL {
            CORECTL(0)
        }
    }
    impl core::fmt::Debug for CORECTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CORECTL")
                .field("NPIE", &self.NPIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CORECTL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CORECTL {{ NPIE: {=bool:?} }}", self.NPIE())
        }
    }
    #[doc = "Debug Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DBGCTL(pub u32);
    impl DBGCTL {
        #[must_use]
        #[inline(always)]
        pub const fn SOD(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SOD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DBGCTL {
        #[inline(always)]
        fn default() -> DBGCTL {
            DBGCTL(0)
        }
    }
    impl core::fmt::Debug for DBGCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DBGCTL").field("SOD", &self.SOD()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DBGCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DBGCTL {{ SOD: {=bool:?} }}", self.SOD())
        }
    }
    #[doc = "Flash Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLASHCR(pub u32);
    impl FLASHCR {
        #[must_use]
        #[inline(always)]
        pub const fn FLASHDIS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLASHDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLASHDOZE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLASHDOZE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for FLASHCR {
        #[inline(always)]
        fn default() -> FLASHCR {
            FLASHCR(0)
        }
    }
    impl core::fmt::Debug for FLASHCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLASHCR")
                .field("FLASHDIS", &self.FLASHDIS())
                .field("FLASHDOZE", &self.FLASHDOZE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASHCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLASHCR {{ FLASHDIS: {=bool:?}, FLASHDOZE: {=bool:?} }}",
                self.FLASHDIS(),
                self.FLASHDOZE()
            )
        }
    }
    #[doc = "Force Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FM(pub u32);
    impl FM {
        #[must_use]
        #[inline(always)]
        pub const fn FORCECFG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FORCECFG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FM {
        #[inline(always)]
        fn default() -> FM {
            FM(0)
        }
    }
    impl core::fmt::Debug for FM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FM")
                .field("FORCECFG", &self.FORCECFG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FM {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FM {{ FORCECFG: {=bool:?} }}", self.FORCECFG())
        }
    }
    #[doc = "Global Power Mode Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GPMCTRL(pub u32);
    impl GPMCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn LPMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for GPMCTRL {
        #[inline(always)]
        fn default() -> GPMCTRL {
            GPMCTRL(0)
        }
    }
    impl core::fmt::Debug for GPMCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GPMCTRL")
                .field("LPMODE", &self.LPMODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GPMCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GPMCTRL {{ LPMODE: {=u8:?} }}", self.LPMODE())
        }
    }
    #[doc = "Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MR(pub u32);
    impl MR {
        #[must_use]
        #[inline(always)]
        pub const fn ISPMODE_n(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ISPMODE_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MR {
        #[inline(always)]
        fn default() -> MR {
            MR(0)
        }
    }
    impl core::fmt::Debug for MR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MR")
                .field("ISPMODE_n", &self.ISPMODE_n())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MR {{ ISPMODE_n: {=bool:?} }}", self.ISPMODE_n())
        }
    }
    #[doc = "Power Mode Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMCTRL(pub u32);
    impl PMCTRL {
        #[must_use]
        #[inline(always)]
        pub const fn LPMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PMCTRL {
        #[inline(always)]
        fn default() -> PMCTRL {
            PMCTRL(0)
        }
    }
    impl core::fmt::Debug for PMCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMCTRL")
                .field("LPMODE", &self.LPMODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PMCTRL {{ LPMODE: {=u8:?} }}", self.LPMODE())
        }
    }
    #[doc = "Power Mode Protection"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PMPROT(pub u32);
    impl PMPROT {
        #[must_use]
        #[inline(always)]
        pub const fn LPMODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LPMODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PMPROT {
        #[inline(always)]
        fn default() -> PMPROT {
            PMPROT(0)
        }
    }
    impl core::fmt::Debug for PMPROT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PMPROT")
                .field("LPMODE", &self.LPMODE())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PMPROT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PMPROT {{ LPMODE: {=u8:?}, LOCK: {=bool:?} }}",
                self.LPMODE(),
                self.LOCK()
            )
        }
    }
    #[doc = "Reset Pin Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RPC(pub u32);
    impl RPC {
        #[must_use]
        #[inline(always)]
        pub const fn FILTCFG(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FILTCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FILTEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FILTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPFEN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for RPC {
        #[inline(always)]
        fn default() -> RPC {
            RPC(0)
        }
    }
    impl core::fmt::Debug for RPC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RPC")
                .field("FILTCFG", &self.FILTCFG())
                .field("FILTEN", &self.FILTEN())
                .field("LPFEN", &self.LPFEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RPC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RPC {{ FILTCFG: {=u8:?}, FILTEN: {=bool:?}, LPFEN: {=bool:?} }}",
                self.FILTCFG(),
                self.FILTEN(),
                self.LPFEN()
            )
        }
    }
    #[doc = "Reset Count Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RSTCNT(pub u32);
    impl RSTCNT {
        #[must_use]
        #[inline(always)]
        pub const fn COUNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_COUNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for RSTCNT {
        #[inline(always)]
        fn default() -> RSTCNT {
            RSTCNT(0)
        }
    }
    impl core::fmt::Debug for RSTCNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RSTCNT")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RSTCNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RSTCNT {{ COUNT: {=u8:?} }}", self.COUNT())
        }
    }
    #[doc = "SRAM Disable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMDIS(pub u32);
    impl SRAMDIS {
        #[must_use]
        #[inline(always)]
        pub const fn DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SRAMDIS {
        #[inline(always)]
        fn default() -> SRAMDIS {
            SRAMDIS(0)
        }
    }
    impl core::fmt::Debug for SRAMDIS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAMDIS")
                .field("DIS0", &self.DIS0())
                .field("DIS1", &self.DIS1())
                .field("DIS2", &self.DIS2())
                .field("DIS3", &self.DIS3())
                .field("DIS4", &self.DIS4())
                .field("DIS5", &self.DIS5())
                .field("DIS6", &self.DIS6())
                .field("DIS7", &self.DIS7())
                .field("DIS8", &self.DIS8())
                .field("DIS9", &self.DIS9())
                .field("DIS10", &self.DIS10())
                .field("DIS11", &self.DIS11())
                .field("DIS12", &self.DIS12())
                .field("DIS13", &self.DIS13())
                .field("DIS14", &self.DIS14())
                .field("DIS15", &self.DIS15())
                .field("DIS16", &self.DIS16())
                .field("DIS17", &self.DIS17())
                .field("DIS18", &self.DIS18())
                .field("DIS19", &self.DIS19())
                .field("DIS20", &self.DIS20())
                .field("DIS21", &self.DIS21())
                .field("DIS22", &self.DIS22())
                .field("DIS23", &self.DIS23())
                .field("DIS24", &self.DIS24())
                .field("DIS25", &self.DIS25())
                .field("DIS26", &self.DIS26())
                .field("DIS27", &self.DIS27())
                .field("DIS28", &self.DIS28())
                .field("DIS29", &self.DIS29())
                .field("DIS30", &self.DIS30())
                .field("DIS31", &self.DIS31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAMDIS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRAMDIS {{ DIS0: {=bool:?}, DIS1: {=bool:?}, DIS2: {=bool:?}, DIS3: {=bool:?}, DIS4: {=bool:?}, DIS5: {=bool:?}, DIS6: {=bool:?}, DIS7: {=bool:?}, DIS8: {=bool:?}, DIS9: {=bool:?}, DIS10: {=bool:?}, DIS11: {=bool:?}, DIS12: {=bool:?}, DIS13: {=bool:?}, DIS14: {=bool:?}, DIS15: {=bool:?}, DIS16: {=bool:?}, DIS17: {=bool:?}, DIS18: {=bool:?}, DIS19: {=bool:?}, DIS20: {=bool:?}, DIS21: {=bool:?}, DIS22: {=bool:?}, DIS23: {=bool:?}, DIS24: {=bool:?}, DIS25: {=bool:?}, DIS26: {=bool:?}, DIS27: {=bool:?}, DIS28: {=bool:?}, DIS29: {=bool:?}, DIS30: {=bool:?}, DIS31: {=bool:?} }}" , self . DIS0 () , self . DIS1 () , self . DIS2 () , self . DIS3 () , self . DIS4 () , self . DIS5 () , self . DIS6 () , self . DIS7 () , self . DIS8 () , self . DIS9 () , self . DIS10 () , self . DIS11 () , self . DIS12 () , self . DIS13 () , self . DIS14 () , self . DIS15 () , self . DIS16 () , self . DIS17 () , self . DIS18 () , self . DIS19 () , self . DIS20 () , self . DIS21 () , self . DIS22 () , self . DIS23 () , self . DIS24 () , self . DIS25 () , self . DIS26 () , self . DIS27 () , self . DIS28 () , self . DIS29 () , self . DIS30 () , self . DIS31 ())
        }
    }
    #[doc = "SRAM Retention"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAMRET(pub u32);
    impl SRAMRET {
        #[must_use]
        #[inline(always)]
        pub const fn RET0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RET31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RET31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SRAMRET {
        #[inline(always)]
        fn default() -> SRAMRET {
            SRAMRET(0)
        }
    }
    impl core::fmt::Debug for SRAMRET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAMRET")
                .field("RET0", &self.RET0())
                .field("RET1", &self.RET1())
                .field("RET2", &self.RET2())
                .field("RET3", &self.RET3())
                .field("RET4", &self.RET4())
                .field("RET5", &self.RET5())
                .field("RET6", &self.RET6())
                .field("RET7", &self.RET7())
                .field("RET8", &self.RET8())
                .field("RET9", &self.RET9())
                .field("RET10", &self.RET10())
                .field("RET11", &self.RET11())
                .field("RET12", &self.RET12())
                .field("RET13", &self.RET13())
                .field("RET14", &self.RET14())
                .field("RET15", &self.RET15())
                .field("RET16", &self.RET16())
                .field("RET17", &self.RET17())
                .field("RET18", &self.RET18())
                .field("RET19", &self.RET19())
                .field("RET20", &self.RET20())
                .field("RET21", &self.RET21())
                .field("RET22", &self.RET22())
                .field("RET23", &self.RET23())
                .field("RET24", &self.RET24())
                .field("RET25", &self.RET25())
                .field("RET26", &self.RET26())
                .field("RET27", &self.RET27())
                .field("RET28", &self.RET28())
                .field("RET29", &self.RET29())
                .field("RET30", &self.RET30())
                .field("RET31", &self.RET31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAMRET {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRAMRET {{ RET0: {=bool:?}, RET1: {=bool:?}, RET2: {=bool:?}, RET3: {=bool:?}, RET4: {=bool:?}, RET5: {=bool:?}, RET6: {=bool:?}, RET7: {=bool:?}, RET8: {=bool:?}, RET9: {=bool:?}, RET10: {=bool:?}, RET11: {=bool:?}, RET12: {=bool:?}, RET13: {=bool:?}, RET14: {=bool:?}, RET15: {=bool:?}, RET16: {=bool:?}, RET17: {=bool:?}, RET18: {=bool:?}, RET19: {=bool:?}, RET20: {=bool:?}, RET21: {=bool:?}, RET22: {=bool:?}, RET23: {=bool:?}, RET24: {=bool:?}, RET25: {=bool:?}, RET26: {=bool:?}, RET27: {=bool:?}, RET28: {=bool:?}, RET29: {=bool:?}, RET30: {=bool:?}, RET31: {=bool:?} }}" , self . RET0 () , self . RET1 () , self . RET2 () , self . RET3 () , self . RET4 () , self . RET5 () , self . RET6 () , self . RET7 () , self . RET8 () , self . RET9 () , self . RET10 () , self . RET11 () , self . RET12 () , self . RET13 () , self . RET14 () , self . RET15 () , self . RET16 () , self . RET17 () , self . RET18 () , self . RET19 () , self . RET20 () , self . RET21 () , self . RET22 () , self . RET23 () , self . RET24 () , self . RET25 () , self . RET26 () , self . RET27 () , self . RET28 () , self . RET29 () , self . RET30 () , self . RET31 ())
        }
    }
    #[doc = "System Reset Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRIE(pub u32);
    impl SRIE {
        #[must_use]
        #[inline(always)]
        pub const fn PIN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPACK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCG(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCKUP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VBAT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VBAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SRIE {
        #[inline(always)]
        fn default() -> SRIE {
            SRIE(0)
        }
    }
    impl core::fmt::Debug for SRIE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRIE")
                .field("PIN", &self.PIN())
                .field("DAP", &self.DAP())
                .field("LPACK", &self.LPACK())
                .field("SCG", &self.SCG())
                .field("WWDT0", &self.WWDT0())
                .field("SW", &self.SW())
                .field("LOCKUP", &self.LOCKUP())
                .field("CPU1", &self.CPU1())
                .field("VBAT", &self.VBAT())
                .field("WWDT1", &self.WWDT1())
                .field("CDOG0", &self.CDOG0())
                .field("CDOG1", &self.CDOG1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRIE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRIE {{ PIN: {=bool:?}, DAP: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CPU1: {=bool:?}, VBAT: {=bool:?}, WWDT1: {=bool:?}, CDOG0: {=bool:?}, CDOG1: {=bool:?} }}" , self . PIN () , self . DAP () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CPU1 () , self . VBAT () , self . WWDT1 () , self . CDOG0 () , self . CDOG1 ())
        }
    }
    #[doc = "System Reset Interrupt Flag"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRIF(pub u32);
    impl SRIF {
        #[must_use]
        #[inline(always)]
        pub const fn PIN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPACK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCKUP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VBAT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VBAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SRIF {
        #[inline(always)]
        fn default() -> SRIF {
            SRIF(0)
        }
    }
    impl core::fmt::Debug for SRIF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRIF")
                .field("PIN", &self.PIN())
                .field("DAP", &self.DAP())
                .field("LPACK", &self.LPACK())
                .field("WWDT0", &self.WWDT0())
                .field("SW", &self.SW())
                .field("LOCKUP", &self.LOCKUP())
                .field("CPU1", &self.CPU1())
                .field("VBAT", &self.VBAT())
                .field("WWDT1", &self.WWDT1())
                .field("CDOG0", &self.CDOG0())
                .field("CDOG1", &self.CDOG1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRIF {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRIF {{ PIN: {=bool:?}, DAP: {=bool:?}, LPACK: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CPU1: {=bool:?}, VBAT: {=bool:?}, WWDT1: {=bool:?}, CDOG0: {=bool:?}, CDOG1: {=bool:?} }}" , self . PIN () , self . DAP () , self . LPACK () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CPU1 () , self . VBAT () , self . WWDT1 () , self . CDOG0 () , self . CDOG1 ())
        }
    }
    #[doc = "System Reset Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRS(pub u32);
    impl SRS {
        #[must_use]
        #[inline(always)]
        pub const fn WAKEUP(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WARM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WARM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FATAL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FATAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTACK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPACK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCG(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCKUP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VBAT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VBAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn JTAG(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JTAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SECVIO(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SECVIO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAMPER(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TAMPER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SRS {
        #[inline(always)]
        fn default() -> SRS {
            SRS(0)
        }
    }
    impl core::fmt::Debug for SRS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRS")
                .field("WAKEUP", &self.WAKEUP())
                .field("POR", &self.POR())
                .field("VD", &self.VD())
                .field("WARM", &self.WARM())
                .field("FATAL", &self.FATAL())
                .field("PIN", &self.PIN())
                .field("DAP", &self.DAP())
                .field("RSTACK", &self.RSTACK())
                .field("LPACK", &self.LPACK())
                .field("SCG", &self.SCG())
                .field("WWDT0", &self.WWDT0())
                .field("SW", &self.SW())
                .field("LOCKUP", &self.LOCKUP())
                .field("CPU1", &self.CPU1())
                .field("VBAT", &self.VBAT())
                .field("WWDT1", &self.WWDT1())
                .field("CDOG0", &self.CDOG0())
                .field("CDOG1", &self.CDOG1())
                .field("JTAG", &self.JTAG())
                .field("SECVIO", &self.SECVIO())
                .field("TAMPER", &self.TAMPER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRS {{ WAKEUP: {=bool:?}, POR: {=bool:?}, VD: {=bool:?}, WARM: {=bool:?}, FATAL: {=bool:?}, PIN: {=bool:?}, DAP: {=bool:?}, RSTACK: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CPU1: {=bool:?}, VBAT: {=bool:?}, WWDT1: {=bool:?}, CDOG0: {=bool:?}, CDOG1: {=bool:?}, JTAG: {=bool:?}, SECVIO: {=bool:?}, TAMPER: {=bool:?} }}" , self . WAKEUP () , self . POR () , self . VD () , self . WARM () , self . FATAL () , self . PIN () , self . DAP () , self . RSTACK () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CPU1 () , self . VBAT () , self . WWDT1 () , self . CDOG0 () , self . CDOG1 () , self . JTAG () , self . SECVIO () , self . TAMPER ())
        }
    }
    #[doc = "Sticky System Reset Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SSRS(pub u32);
    impl SSRS {
        #[must_use]
        #[inline(always)]
        pub const fn WAKEUP(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WAKEUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VD(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WARM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WARM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FATAL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FATAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PIN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTACK(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSTACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPACK(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCG(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SCG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SW(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCKUP(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CPU1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CPU1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VBAT(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VBAT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WWDT1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WWDT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDOG1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn JTAG(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JTAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SECVIO(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SECVIO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAMPER(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TAMPER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SSRS {
        #[inline(always)]
        fn default() -> SSRS {
            SSRS(0)
        }
    }
    impl core::fmt::Debug for SSRS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SSRS")
                .field("WAKEUP", &self.WAKEUP())
                .field("POR", &self.POR())
                .field("VD", &self.VD())
                .field("WARM", &self.WARM())
                .field("FATAL", &self.FATAL())
                .field("PIN", &self.PIN())
                .field("DAP", &self.DAP())
                .field("RSTACK", &self.RSTACK())
                .field("LPACK", &self.LPACK())
                .field("SCG", &self.SCG())
                .field("WWDT0", &self.WWDT0())
                .field("SW", &self.SW())
                .field("LOCKUP", &self.LOCKUP())
                .field("CPU1", &self.CPU1())
                .field("VBAT", &self.VBAT())
                .field("WWDT1", &self.WWDT1())
                .field("CDOG0", &self.CDOG0())
                .field("CDOG1", &self.CDOG1())
                .field("JTAG", &self.JTAG())
                .field("SECVIO", &self.SECVIO())
                .field("TAMPER", &self.TAMPER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSRS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSRS {{ WAKEUP: {=bool:?}, POR: {=bool:?}, VD: {=bool:?}, WARM: {=bool:?}, FATAL: {=bool:?}, PIN: {=bool:?}, DAP: {=bool:?}, RSTACK: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CPU1: {=bool:?}, VBAT: {=bool:?}, WWDT1: {=bool:?}, CDOG0: {=bool:?}, CDOG1: {=bool:?}, JTAG: {=bool:?}, SECVIO: {=bool:?}, TAMPER: {=bool:?} }}" , self . WAKEUP () , self . POR () , self . VD () , self . WARM () , self . FATAL () , self . PIN () , self . DAP () , self . RSTACK () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CPU1 () , self . VBAT () , self . WWDT1 () , self . CDOG0 () , self . CDOG1 () , self . JTAG () , self . SECVIO () , self . TAMPER ())
        }
    }
    #[doc = "Version ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VERID(pub u32);
    impl VERID {
        #[must_use]
        #[inline(always)]
        pub const fn FEATURE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_FEATURE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MINOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MAJOR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MAJOR(&mut self, val: u8) {
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
