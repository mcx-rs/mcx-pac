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
        assert!(n < 1usize);
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
    pub const fn FLASHCR(self) -> crate::common::Reg<regs::FLASHCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
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
        #[must_use]
        #[inline(always)]
        pub const fn FLASHWAKE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLASHWAKE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("FLASHWAKE", &self.FLASHWAKE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLASHCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLASHCR {{ FLASHDIS: {=bool:?}, FLASHDOZE: {=bool:?}, FLASHWAKE: {=bool:?} }}",
                self.FLASHDIS(),
                self.FLASHDOZE(),
                self.FLASHWAKE()
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
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("CDOG0", &self.CDOG0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRIE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRIE {{ PIN: {=bool:?}, DAP: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CDOG0: {=bool:?} }}" , self . PIN () , self . DAP () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CDOG0 ())
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
        pub const fn CDOG0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDOG0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("CDOG0", &self.CDOG0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRIF {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRIF {{ PIN: {=bool:?}, DAP: {=bool:?}, LPACK: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CDOG0: {=bool:?} }}" , self . PIN () , self . DAP () , self . LPACK () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CDOG0 ())
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
        pub const fn JTAG(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JTAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("CDOG0", &self.CDOG0())
                .field("JTAG", &self.JTAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRS {{ WAKEUP: {=bool:?}, POR: {=bool:?}, VD: {=bool:?}, WARM: {=bool:?}, FATAL: {=bool:?}, PIN: {=bool:?}, DAP: {=bool:?}, RSTACK: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CDOG0: {=bool:?}, JTAG: {=bool:?} }}" , self . WAKEUP () , self . POR () , self . VD () , self . WARM () , self . FATAL () , self . PIN () , self . DAP () , self . RSTACK () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CDOG0 () , self . JTAG ())
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
        pub const fn JTAG(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JTAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("CDOG0", &self.CDOG0())
                .field("JTAG", &self.JTAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SSRS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SSRS {{ WAKEUP: {=bool:?}, POR: {=bool:?}, VD: {=bool:?}, WARM: {=bool:?}, FATAL: {=bool:?}, PIN: {=bool:?}, DAP: {=bool:?}, RSTACK: {=bool:?}, LPACK: {=bool:?}, SCG: {=bool:?}, WWDT0: {=bool:?}, SW: {=bool:?}, LOCKUP: {=bool:?}, CDOG0: {=bool:?}, JTAG: {=bool:?} }}" , self . WAKEUP () , self . POR () , self . VD () , self . WARM () , self . FATAL () , self . PIN () , self . DAP () , self . RSTACK () , self . LPACK () , self . SCG () , self . WWDT0 () , self . SW () , self . LOCKUP () , self . CDOG0 () , self . JTAG ())
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
