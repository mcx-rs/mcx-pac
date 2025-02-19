#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VBAT {
    ptr: *mut u8,
}
unsafe impl Send for VBAT {}
unsafe impl Sync for VBAT {}
impl VBAT {
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
    pub const fn STATUSA(self) -> crate::common::Reg<regs::STATUSA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUSB(self) -> crate::common::Reg<regs::STATUSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQENA(self) -> crate::common::Reg<regs::IRQENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IRQENB(self) -> crate::common::Reg<regs::IRQENB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn WAKENA(self) -> crate::common::Reg<regs::WAKENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKENB(self) -> crate::common::Reg<regs::WAKENB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TAMPERA(self) -> crate::common::Reg<regs::TAMPERA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn TAMPERB(self) -> crate::common::Reg<regs::TAMPERB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn LOCKA(self) -> crate::common::Reg<regs::LOCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn LOCKB(self) -> crate::common::Reg<regs::LOCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKECFG(self) -> crate::common::Reg<regs::WAKECFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn OSCCTLA(self) -> crate::common::Reg<regs::OSCCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn OSCCTLB(self) -> crate::common::Reg<regs::OSCCTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn OSCCFGA(self) -> crate::common::Reg<regs::OSCCFGA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn OSCCFGB(self) -> crate::common::Reg<regs::OSCCFGB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn OSCLCKA(self) -> crate::common::Reg<regs::OSCLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn OSCLCKB(self) -> crate::common::Reg<regs::OSCLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn OSCCLKE(self) -> crate::common::Reg<regs::OSCCLKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn FROCTLA(self) -> crate::common::Reg<regs::FROCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn FROCTLB(self) -> crate::common::Reg<regs::FROCTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn FROLCKA(self) -> crate::common::Reg<regs::FROLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn FROLCKB(self) -> crate::common::Reg<regs::FROLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[inline(always)]
    pub const fn FROCLKE(self) -> crate::common::Reg<regs::FROCLKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOCTLA(self) -> crate::common::Reg<regs::LDOCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOCTLB(self) -> crate::common::Reg<regs::LDOCTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOLCKA(self) -> crate::common::Reg<regs::LDOLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOLCKB(self) -> crate::common::Reg<regs::LDOLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[inline(always)]
    pub const fn LDORAMC(self) -> crate::common::Reg<regs::LDORAMC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOTIMER0(self) -> crate::common::Reg<regs::LDOTIMER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[inline(always)]
    pub const fn LDOTIMER1(self) -> crate::common::Reg<regs::LDOTIMER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[inline(always)]
    pub const fn MONCTLA(self) -> crate::common::Reg<regs::MONCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn MONCTLB(self) -> crate::common::Reg<regs::MONCTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn MONCFGA(self) -> crate::common::Reg<regs::MONCFGA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[inline(always)]
    pub const fn MONCFGB(self) -> crate::common::Reg<regs::MONCFGB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[inline(always)]
    pub const fn MONLCKA(self) -> crate::common::Reg<regs::MONLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[inline(always)]
    pub const fn MONLCKB(self) -> crate::common::Reg<regs::MONLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[inline(always)]
    pub const fn TAMCTLA(self) -> crate::common::Reg<regs::TAMCTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn TAMCTLB(self) -> crate::common::Reg<regs::TAMCTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[inline(always)]
    pub const fn TAMLCKA(self) -> crate::common::Reg<regs::TAMLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[inline(always)]
    pub const fn TAMLCKB(self) -> crate::common::Reg<regs::TAMLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[inline(always)]
    pub const fn SWICTLA(self) -> crate::common::Reg<regs::SWICTLA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn SWICTLB(self) -> crate::common::Reg<regs::SWICTLB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[inline(always)]
    pub const fn SWILCKA(self) -> crate::common::Reg<regs::SWILCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[inline(always)]
    pub const fn SWILCKB(self) -> crate::common::Reg<regs::SWILCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[inline(always)]
    pub const fn WAKEUP(self, n: usize) -> WAKEUP {
        assert!(n < 2usize);
        unsafe { WAKEUP::from_ptr(self.ptr.add(0x0700usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKLCKA(self) -> crate::common::Reg<regs::WAKLCKA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKLCKB(self) -> crate::common::Reg<regs::WAKLCKB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07fcusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAKEUP {
    ptr: *mut u8,
}
unsafe impl Send for WAKEUP {}
unsafe impl Sync for WAKEUP {}
impl WAKEUP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn WAKEUPA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn WAKEUPB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "FRO16K Clock Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCLKE(pub u32);
    impl FROCLKE {
        #[inline(always)]
        pub const fn CLKE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for FROCLKE {
        #[inline(always)]
        fn default() -> FROCLKE {
            FROCLKE(0)
        }
    }
    impl core::fmt::Debug for FROCLKE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROCLKE")
                .field("CLKE", &self.CLKE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROCLKE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROCLKE {{ CLKE: {=u8:?} }}", self.CLKE())
        }
    }
    #[doc = "FRO16K Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCTLA(pub u32);
    impl FROCTLA {
        #[inline(always)]
        pub const fn FRO_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROCTLA {
        #[inline(always)]
        fn default() -> FROCTLA {
            FROCTLA(0)
        }
    }
    impl core::fmt::Debug for FROCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROCTLA")
                .field("FRO_EN", &self.FRO_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROCTLA {{ FRO_EN: {=bool:?} }}", self.FRO_EN())
        }
    }
    #[doc = "FRO16K Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROCTLB(pub u32);
    impl FROCTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROCTLB {
        #[inline(always)]
        fn default() -> FROCTLB {
            FROCTLB(0)
        }
    }
    impl core::fmt::Debug for FROCTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROCTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROCTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROCTLB {{ INVERSE: {=bool:?} }}", self.INVERSE())
        }
    }
    #[doc = "FRO16K Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROLCKA(pub u32);
    impl FROLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROLCKA {
        #[inline(always)]
        fn default() -> FROLCKA {
            FROLCKA(0)
        }
    }
    impl core::fmt::Debug for FROLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "FRO16K Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROLCKB(pub u32);
    impl FROLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for FROLCKB {
        #[inline(always)]
        fn default() -> FROLCKB {
            FROLCKB(0)
        }
    }
    impl core::fmt::Debug for FROLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FROLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Interrupt Enable A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRQENA(pub u32);
    impl IRQENA {
        #[inline(always)]
        pub const fn POR_DET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POR_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TIMER0_FLAG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER0_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TIMER1_FLAG(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER1_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn LDO_RDY(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDO_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OSC_RDY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CLOCK_DET(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLOCK_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CONFIG_DET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VOLT_DET(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOLT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TEMP_DET(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEMP_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LIGHT_DET(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIGHT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEC0_DET(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEC0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IRQ0_DET(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn IRQ1_DET(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ1_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn IRQ2_DET(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ2_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn IRQ3_DET(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ3_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for IRQENA {
        #[inline(always)]
        fn default() -> IRQENA {
            IRQENA(0)
        }
    }
    impl core::fmt::Debug for IRQENA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IRQENA")
                .field("POR_DET", &self.POR_DET())
                .field("WAKEUP_FLAG", &self.WAKEUP_FLAG())
                .field("TIMER0_FLAG", &self.TIMER0_FLAG())
                .field("TIMER1_FLAG", &self.TIMER1_FLAG())
                .field("LDO_RDY", &self.LDO_RDY())
                .field("OSC_RDY", &self.OSC_RDY())
                .field("CLOCK_DET", &self.CLOCK_DET())
                .field("CONFIG_DET", &self.CONFIG_DET())
                .field("VOLT_DET", &self.VOLT_DET())
                .field("TEMP_DET", &self.TEMP_DET())
                .field("LIGHT_DET", &self.LIGHT_DET())
                .field("SEC0_DET", &self.SEC0_DET())
                .field("IRQ0_DET", &self.IRQ0_DET())
                .field("IRQ1_DET", &self.IRQ1_DET())
                .field("IRQ2_DET", &self.IRQ2_DET())
                .field("IRQ3_DET", &self.IRQ3_DET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IRQENA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IRQENA {{ POR_DET: {=bool:?}, WAKEUP_FLAG: {=bool:?}, TIMER0_FLAG: {=bool:?}, TIMER1_FLAG: {=bool:?}, LDO_RDY: {=bool:?}, OSC_RDY: {=bool:?}, CLOCK_DET: {=bool:?}, CONFIG_DET: {=bool:?}, VOLT_DET: {=bool:?}, TEMP_DET: {=bool:?}, LIGHT_DET: {=bool:?}, SEC0_DET: {=bool:?}, IRQ0_DET: {=bool:?}, IRQ1_DET: {=bool:?}, IRQ2_DET: {=bool:?}, IRQ3_DET: {=bool:?} }}" , self . POR_DET () , self . WAKEUP_FLAG () , self . TIMER0_FLAG () , self . TIMER1_FLAG () , self . LDO_RDY () , self . OSC_RDY () , self . CLOCK_DET () , self . CONFIG_DET () , self . VOLT_DET () , self . TEMP_DET () , self . LIGHT_DET () , self . SEC0_DET () , self . IRQ0_DET () , self . IRQ1_DET () , self . IRQ2_DET () , self . IRQ3_DET ())
        }
    }
    #[doc = "Interrupt Enable B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRQENB(pub u32);
    impl IRQENB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for IRQENB {
        #[inline(always)]
        fn default() -> IRQENB {
            IRQENB(0)
        }
    }
    impl core::fmt::Debug for IRQENB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IRQENB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IRQENB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IRQENB {{ INVERSE: {=u32:?} }}", self.INVERSE())
        }
    }
    #[doc = "LDO_RAM Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOCTLA(pub u32);
    impl LDOCTLA {
        #[inline(always)]
        pub const fn BG_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LDO_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDO_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn REFRESH_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REFRESH_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for LDOCTLA {
        #[inline(always)]
        fn default() -> LDOCTLA {
            LDOCTLA(0)
        }
    }
    impl core::fmt::Debug for LDOCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOCTLA")
                .field("BG_EN", &self.BG_EN())
                .field("LDO_EN", &self.LDO_EN())
                .field("REFRESH_EN", &self.REFRESH_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LDOCTLA {{ BG_EN: {=bool:?}, LDO_EN: {=bool:?}, REFRESH_EN: {=bool:?} }}",
                self.BG_EN(),
                self.LDO_EN(),
                self.REFRESH_EN()
            )
        }
    }
    #[doc = "LDO_RAM Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOCTLB(pub u32);
    impl LDOCTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for LDOCTLB {
        #[inline(always)]
        fn default() -> LDOCTLB {
            LDOCTLB(0)
        }
    }
    impl core::fmt::Debug for LDOCTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOCTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOCTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LDOCTLB {{ INVERSE: {=u8:?} }}", self.INVERSE())
        }
    }
    #[doc = "LDO_RAM Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOLCKA(pub u32);
    impl LDOLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LDOLCKA {
        #[inline(always)]
        fn default() -> LDOLCKA {
            LDOLCKA(0)
        }
    }
    impl core::fmt::Debug for LDOLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LDOLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "LDO_RAM Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOLCKB(pub u32);
    impl LDOLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LDOLCKB {
        #[inline(always)]
        fn default() -> LDOLCKB {
            LDOLCKB(0)
        }
    }
    impl core::fmt::Debug for LDOLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LDOLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "RAM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDORAMC(pub u32);
    impl LDORAMC {
        #[inline(always)]
        pub const fn ISO(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ISO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SWI(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RET0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RET0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn RET1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RET1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn RET2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RET2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn RET3(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RET3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for LDORAMC {
        #[inline(always)]
        fn default() -> LDORAMC {
            LDORAMC(0)
        }
    }
    impl core::fmt::Debug for LDORAMC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDORAMC")
                .field("ISO", &self.ISO())
                .field("SWI", &self.SWI())
                .field("RET0", &self.RET0())
                .field("RET1", &self.RET1())
                .field("RET2", &self.RET2())
                .field("RET3", &self.RET3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDORAMC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LDORAMC {{ ISO: {=bool:?}, SWI: {=bool:?}, RET0: {=bool:?}, RET1: {=bool:?}, RET2: {=bool:?}, RET3: {=bool:?} }}" , self . ISO () , self . SWI () , self . RET0 () , self . RET1 () , self . RET2 () , self . RET3 ())
        }
    }
    #[doc = "Bandgap Timer 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOTIMER0(pub u32);
    impl LDOTIMER0 {
        #[inline(always)]
        pub const fn TIMCFG(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_TIMCFG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[inline(always)]
        pub const fn TIMEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LDOTIMER0 {
        #[inline(always)]
        fn default() -> LDOTIMER0 {
            LDOTIMER0(0)
        }
    }
    impl core::fmt::Debug for LDOTIMER0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOTIMER0")
                .field("TIMCFG", &self.TIMCFG())
                .field("TIMEN", &self.TIMEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOTIMER0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LDOTIMER0 {{ TIMCFG: {=u8:?}, TIMEN: {=bool:?} }}",
                self.TIMCFG(),
                self.TIMEN()
            )
        }
    }
    #[doc = "Bandgap Timer 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LDOTIMER1(pub u32);
    impl LDOTIMER1 {
        #[inline(always)]
        pub const fn TIMCFG(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TIMCFG(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn TIMEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for LDOTIMER1 {
        #[inline(always)]
        fn default() -> LDOTIMER1 {
            LDOTIMER1(0)
        }
    }
    impl core::fmt::Debug for LDOTIMER1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LDOTIMER1")
                .field("TIMCFG", &self.TIMCFG())
                .field("TIMEN", &self.TIMEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LDOTIMER1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LDOTIMER1 {{ TIMCFG: {=u32:?}, TIMEN: {=bool:?} }}",
                self.TIMCFG(),
                self.TIMEN()
            )
        }
    }
    #[doc = "Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LOCKA(pub u32);
    impl LOCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LOCKA {
        #[inline(always)]
        fn default() -> LOCKA {
            LOCKA(0)
        }
    }
    impl core::fmt::Debug for LOCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LOCKA").field("LOCK", &self.LOCK()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LOCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LOCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LOCKB(pub u32);
    impl LOCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LOCKB {
        #[inline(always)]
        fn default() -> LOCKB {
            LOCKB(0)
        }
    }
    impl core::fmt::Debug for LOCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LOCKB").field("LOCK", &self.LOCK()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LOCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LOCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "CLKMON Configuration A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONCFGA(pub u32);
    impl MONCFGA {
        #[inline(always)]
        pub const fn FREQ_TRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_FREQ_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn DIVIDE_TRIM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIVIDE_TRIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RSVD_TRIM(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RSVD_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
    }
    impl Default for MONCFGA {
        #[inline(always)]
        fn default() -> MONCFGA {
            MONCFGA(0)
        }
    }
    impl core::fmt::Debug for MONCFGA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONCFGA")
                .field("FREQ_TRIM", &self.FREQ_TRIM())
                .field("DIVIDE_TRIM", &self.DIVIDE_TRIM())
                .field("RSVD_TRIM", &self.RSVD_TRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONCFGA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MONCFGA {{ FREQ_TRIM: {=u8:?}, DIVIDE_TRIM: {=bool:?}, RSVD_TRIM: {=u8:?} }}",
                self.FREQ_TRIM(),
                self.DIVIDE_TRIM(),
                self.RSVD_TRIM()
            )
        }
    }
    #[doc = "CLKMON Configuration B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONCFGB(pub u32);
    impl MONCFGB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for MONCFGB {
        #[inline(always)]
        fn default() -> MONCFGB {
            MONCFGB(0)
        }
    }
    impl core::fmt::Debug for MONCFGB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONCFGB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONCFGB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MONCFGB {{ INVERSE: {=u8:?} }}", self.INVERSE())
        }
    }
    #[doc = "CLKMON Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONCTLA(pub u32);
    impl MONCTLA {
        #[inline(always)]
        pub const fn MON_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MON_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MONCTLA {
        #[inline(always)]
        fn default() -> MONCTLA {
            MONCTLA(0)
        }
    }
    impl core::fmt::Debug for MONCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONCTLA")
                .field("MON_EN", &self.MON_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MONCTLA {{ MON_EN: {=bool:?} }}", self.MON_EN())
        }
    }
    #[doc = "CLKMON Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONCTLB(pub u32);
    impl MONCTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MONCTLB {
        #[inline(always)]
        fn default() -> MONCTLB {
            MONCTLB(0)
        }
    }
    impl core::fmt::Debug for MONCTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONCTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONCTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MONCTLB {{ INVERSE: {=bool:?} }}", self.INVERSE())
        }
    }
    #[doc = "CLKMON Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONLCKA(pub u32);
    impl MONLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MONLCKA {
        #[inline(always)]
        fn default() -> MONLCKA {
            MONLCKA(0)
        }
    }
    impl core::fmt::Debug for MONLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MONLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "CLKMON Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MONLCKB(pub u32);
    impl MONLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MONLCKB {
        #[inline(always)]
        fn default() -> MONLCKB {
            MONLCKB(0)
        }
    }
    impl core::fmt::Debug for MONLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MONLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MONLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MONLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Oscillator Configuration A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCCFGA(pub u32);
    impl OSCCFGA {
        #[inline(always)]
        pub const fn CMP_TRIM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CMP_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CAP2_TRIM(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAP2_TRIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DLY_TRIM(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DLY_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[inline(always)]
        pub const fn CAP_TRIM(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAP_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[inline(always)]
        pub const fn INIT_TRIM(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_INIT_TRIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
    }
    impl Default for OSCCFGA {
        #[inline(always)]
        fn default() -> OSCCFGA {
            OSCCFGA(0)
        }
    }
    impl core::fmt::Debug for OSCCFGA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCCFGA")
                .field("CMP_TRIM", &self.CMP_TRIM())
                .field("CAP2_TRIM", &self.CAP2_TRIM())
                .field("DLY_TRIM", &self.DLY_TRIM())
                .field("CAP_TRIM", &self.CAP_TRIM())
                .field("INIT_TRIM", &self.INIT_TRIM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCCFGA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OSCCFGA {{ CMP_TRIM: {=u8:?}, CAP2_TRIM: {=bool:?}, DLY_TRIM: {=u8:?}, CAP_TRIM: {=u8:?}, INIT_TRIM: {=u8:?} }}" , self . CMP_TRIM () , self . CAP2_TRIM () , self . DLY_TRIM () , self . CAP_TRIM () , self . INIT_TRIM ())
        }
    }
    #[doc = "Oscillator Configuration B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCCFGB(pub u32);
    impl OSCCFGB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for OSCCFGB {
        #[inline(always)]
        fn default() -> OSCCFGB {
            OSCCFGB(0)
        }
    }
    impl core::fmt::Debug for OSCCFGB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCCFGB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCCFGB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OSCCFGB {{ INVERSE: {=u16:?} }}", self.INVERSE())
        }
    }
    #[doc = "Oscillator Clock Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCCLKE(pub u32);
    impl OSCCLKE {
        #[inline(always)]
        pub const fn CLKE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CLKE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for OSCCLKE {
        #[inline(always)]
        fn default() -> OSCCLKE {
            OSCCLKE(0)
        }
    }
    impl core::fmt::Debug for OSCCLKE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCCLKE")
                .field("CLKE", &self.CLKE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCCLKE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OSCCLKE {{ CLKE: {=u8:?} }}", self.CLKE())
        }
    }
    #[doc = "Oscillator Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCCTLA(pub u32);
    impl OSCCTLA {
        #[inline(always)]
        pub const fn OSC_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OSC_BYP_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_BYP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn COARSE_AMP_GAIN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_COARSE_AMP_GAIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn CAP_SEL_EN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CAP_SEL_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn EXTAL_CAP_SEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_EXTAL_CAP_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn XTAL_CAP_SEL(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_XTAL_CAP_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn MODE_EN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_MODE_EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn SUPPLY_DET(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SUPPLY_DET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for OSCCTLA {
        #[inline(always)]
        fn default() -> OSCCTLA {
            OSCCTLA(0)
        }
    }
    impl core::fmt::Debug for OSCCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCCTLA")
                .field("OSC_EN", &self.OSC_EN())
                .field("OSC_BYP_EN", &self.OSC_BYP_EN())
                .field("COARSE_AMP_GAIN", &self.COARSE_AMP_GAIN())
                .field("CAP_SEL_EN", &self.CAP_SEL_EN())
                .field("EXTAL_CAP_SEL", &self.EXTAL_CAP_SEL())
                .field("XTAL_CAP_SEL", &self.XTAL_CAP_SEL())
                .field("MODE_EN", &self.MODE_EN())
                .field("SUPPLY_DET", &self.SUPPLY_DET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OSCCTLA {{ OSC_EN: {=bool:?}, OSC_BYP_EN: {=bool:?}, COARSE_AMP_GAIN: {=u8:?}, CAP_SEL_EN: {=bool:?}, EXTAL_CAP_SEL: {=u8:?}, XTAL_CAP_SEL: {=u8:?}, MODE_EN: {=u8:?}, SUPPLY_DET: {=u8:?} }}" , self . OSC_EN () , self . OSC_BYP_EN () , self . COARSE_AMP_GAIN () , self . CAP_SEL_EN () , self . EXTAL_CAP_SEL () , self . XTAL_CAP_SEL () , self . MODE_EN () , self . SUPPLY_DET ())
        }
    }
    #[doc = "Oscillator Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCCTLB(pub u32);
    impl OSCCTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for OSCCTLB {
        #[inline(always)]
        fn default() -> OSCCTLB {
            OSCCTLB(0)
        }
    }
    impl core::fmt::Debug for OSCCTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCCTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCCTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OSCCTLB {{ INVERSE: {=u32:?} }}", self.INVERSE())
        }
    }
    #[doc = "Oscillator Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCLCKA(pub u32);
    impl OSCLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OSCLCKA {
        #[inline(always)]
        fn default() -> OSCLCKA {
            OSCLCKA(0)
        }
    }
    impl core::fmt::Debug for OSCLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OSCLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Oscillator Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OSCLCKB(pub u32);
    impl OSCLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OSCLCKB {
        #[inline(always)]
        fn default() -> OSCLCKB {
            OSCLCKB(0)
        }
    }
    impl core::fmt::Debug for OSCLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OSCLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OSCLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OSCLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Status A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUSA(pub u32);
    impl STATUSA {
        #[inline(always)]
        pub const fn POR_DET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POR_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TIMER0_FLAG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER0_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TIMER1_FLAG(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER1_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn LDO_RDY(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDO_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OSC_RDY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CLOCK_DET(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLOCK_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CONFIG_DET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VOLT_DET(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOLT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TEMP_DET(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEMP_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LIGHT_DET(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIGHT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEC0_DET(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEC0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IRQ0_DET(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn IRQ1_DET(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ1_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn IRQ2_DET(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ2_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn IRQ3_DET(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ3_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for STATUSA {
        #[inline(always)]
        fn default() -> STATUSA {
            STATUSA(0)
        }
    }
    impl core::fmt::Debug for STATUSA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUSA")
                .field("POR_DET", &self.POR_DET())
                .field("WAKEUP_FLAG", &self.WAKEUP_FLAG())
                .field("TIMER0_FLAG", &self.TIMER0_FLAG())
                .field("TIMER1_FLAG", &self.TIMER1_FLAG())
                .field("LDO_RDY", &self.LDO_RDY())
                .field("OSC_RDY", &self.OSC_RDY())
                .field("CLOCK_DET", &self.CLOCK_DET())
                .field("CONFIG_DET", &self.CONFIG_DET())
                .field("VOLT_DET", &self.VOLT_DET())
                .field("TEMP_DET", &self.TEMP_DET())
                .field("LIGHT_DET", &self.LIGHT_DET())
                .field("SEC0_DET", &self.SEC0_DET())
                .field("IRQ0_DET", &self.IRQ0_DET())
                .field("IRQ1_DET", &self.IRQ1_DET())
                .field("IRQ2_DET", &self.IRQ2_DET())
                .field("IRQ3_DET", &self.IRQ3_DET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUSA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUSA {{ POR_DET: {=bool:?}, WAKEUP_FLAG: {=bool:?}, TIMER0_FLAG: {=bool:?}, TIMER1_FLAG: {=bool:?}, LDO_RDY: {=bool:?}, OSC_RDY: {=bool:?}, CLOCK_DET: {=bool:?}, CONFIG_DET: {=bool:?}, VOLT_DET: {=bool:?}, TEMP_DET: {=bool:?}, LIGHT_DET: {=bool:?}, SEC0_DET: {=bool:?}, IRQ0_DET: {=bool:?}, IRQ1_DET: {=bool:?}, IRQ2_DET: {=bool:?}, IRQ3_DET: {=bool:?} }}" , self . POR_DET () , self . WAKEUP_FLAG () , self . TIMER0_FLAG () , self . TIMER1_FLAG () , self . LDO_RDY () , self . OSC_RDY () , self . CLOCK_DET () , self . CONFIG_DET () , self . VOLT_DET () , self . TEMP_DET () , self . LIGHT_DET () , self . SEC0_DET () , self . IRQ0_DET () , self . IRQ1_DET () , self . IRQ2_DET () , self . IRQ3_DET ())
        }
    }
    #[doc = "Status B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUSB(pub u32);
    impl STATUSB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for STATUSB {
        #[inline(always)]
        fn default() -> STATUSB {
            STATUSB(0)
        }
    }
    impl core::fmt::Debug for STATUSB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUSB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUSB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "STATUSB {{ INVERSE: {=u32:?} }}", self.INVERSE())
        }
    }
    #[doc = "Switch Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWICTLA(pub u32);
    impl SWICTLA {
        #[inline(always)]
        pub const fn SWI_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWI_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LP_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SWICTLA {
        #[inline(always)]
        fn default() -> SWICTLA {
            SWICTLA(0)
        }
    }
    impl core::fmt::Debug for SWICTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWICTLA")
                .field("SWI_EN", &self.SWI_EN())
                .field("LP_EN", &self.LP_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWICTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SWICTLA {{ SWI_EN: {=bool:?}, LP_EN: {=bool:?} }}",
                self.SWI_EN(),
                self.LP_EN()
            )
        }
    }
    #[doc = "Switch Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWICTLB(pub u32);
    impl SWICTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for SWICTLB {
        #[inline(always)]
        fn default() -> SWICTLB {
            SWICTLB(0)
        }
    }
    impl core::fmt::Debug for SWICTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWICTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWICTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWICTLB {{ INVERSE: {=u8:?} }}", self.INVERSE())
        }
    }
    #[doc = "Switch Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWILCKA(pub u32);
    impl SWILCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SWILCKA {
        #[inline(always)]
        fn default() -> SWILCKA {
            SWILCKA(0)
        }
    }
    impl core::fmt::Debug for SWILCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWILCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWILCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWILCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Switch Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SWILCKB(pub u32);
    impl SWILCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SWILCKB {
        #[inline(always)]
        fn default() -> SWILCKB {
            SWILCKB(0)
        }
    }
    impl core::fmt::Debug for SWILCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SWILCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SWILCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SWILCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "TAMPER Control A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMCTLA(pub u32);
    impl TAMCTLA {
        #[inline(always)]
        pub const fn VOLT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOLT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TEMP_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEMP_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn LIGHT_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIGHT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for TAMCTLA {
        #[inline(always)]
        fn default() -> TAMCTLA {
            TAMCTLA(0)
        }
    }
    impl core::fmt::Debug for TAMCTLA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMCTLA")
                .field("VOLT_EN", &self.VOLT_EN())
                .field("TEMP_EN", &self.TEMP_EN())
                .field("LIGHT_EN", &self.LIGHT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMCTLA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TAMCTLA {{ VOLT_EN: {=bool:?}, TEMP_EN: {=bool:?}, LIGHT_EN: {=bool:?} }}",
                self.VOLT_EN(),
                self.TEMP_EN(),
                self.LIGHT_EN()
            )
        }
    }
    #[doc = "TAMPER Control B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMCTLB(pub u32);
    impl TAMCTLB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TAMCTLB {
        #[inline(always)]
        fn default() -> TAMCTLB {
            TAMCTLB(0)
        }
    }
    impl core::fmt::Debug for TAMCTLB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMCTLB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMCTLB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TAMCTLB {{ INVERSE: {=u8:?} }}", self.INVERSE())
        }
    }
    #[doc = "TAMPER Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMLCKA(pub u32);
    impl TAMLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TAMLCKA {
        #[inline(always)]
        fn default() -> TAMLCKA {
            TAMLCKA(0)
        }
    }
    impl core::fmt::Debug for TAMLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TAMLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "TAMPER Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMLCKB(pub u32);
    impl TAMLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TAMLCKB {
        #[inline(always)]
        fn default() -> TAMLCKB {
            TAMLCKB(0)
        }
    }
    impl core::fmt::Debug for TAMLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TAMLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Tamper Enable A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMPERA(pub u32);
    impl TAMPERA {
        #[inline(always)]
        pub const fn POR_DET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POR_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLOCK_DET(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLOCK_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CONFIG_DET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VOLT_DET(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOLT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TEMP_DET(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEMP_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LIGHT_DET(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIGHT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEC0_DET(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEC0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for TAMPERA {
        #[inline(always)]
        fn default() -> TAMPERA {
            TAMPERA(0)
        }
    }
    impl core::fmt::Debug for TAMPERA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMPERA")
                .field("POR_DET", &self.POR_DET())
                .field("CLOCK_DET", &self.CLOCK_DET())
                .field("CONFIG_DET", &self.CONFIG_DET())
                .field("VOLT_DET", &self.VOLT_DET())
                .field("TEMP_DET", &self.TEMP_DET())
                .field("LIGHT_DET", &self.LIGHT_DET())
                .field("SEC0_DET", &self.SEC0_DET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMPERA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TAMPERA {{ POR_DET: {=bool:?}, CLOCK_DET: {=bool:?}, CONFIG_DET: {=bool:?}, VOLT_DET: {=bool:?}, TEMP_DET: {=bool:?}, LIGHT_DET: {=bool:?}, SEC0_DET: {=bool:?} }}" , self . POR_DET () , self . CLOCK_DET () , self . CONFIG_DET () , self . VOLT_DET () , self . TEMP_DET () , self . LIGHT_DET () , self . SEC0_DET ())
        }
    }
    #[doc = "Tamper Enable B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TAMPERB(pub u32);
    impl TAMPERB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TAMPERB {
        #[inline(always)]
        fn default() -> TAMPERB {
            TAMPERB(0)
        }
    }
    impl core::fmt::Debug for TAMPERB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TAMPERB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TAMPERB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TAMPERB {{ INVERSE: {=u16:?} }}", self.INVERSE())
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
    #[doc = "Wake-up Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKECFG(pub u32);
    impl WAKECFG {
        #[inline(always)]
        pub const fn OUT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WAKECFG {
        #[inline(always)]
        fn default() -> WAKECFG {
            WAKECFG(0)
        }
    }
    impl core::fmt::Debug for WAKECFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKECFG").field("OUT", &self.OUT()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKECFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WAKECFG {{ OUT: {=bool:?} }}", self.OUT())
        }
    }
    #[doc = "Wake-up Enable A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKENA(pub u32);
    impl WAKENA {
        #[inline(always)]
        pub const fn POR_DET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_POR_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WAKEUP_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WAKEUP_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TIMER0_FLAG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER0_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn TIMER1_FLAG(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TIMER1_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn LDO_RDY(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LDO_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn OSC_RDY(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OSC_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn CLOCK_DET(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLOCK_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn CONFIG_DET(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn VOLT_DET(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_VOLT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn TEMP_DET(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEMP_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn LIGHT_DET(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIGHT_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEC0_DET(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEC0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IRQ0_DET(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ0_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn IRQ1_DET(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ1_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn IRQ2_DET(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ2_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn IRQ3_DET(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IRQ3_DET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for WAKENA {
        #[inline(always)]
        fn default() -> WAKENA {
            WAKENA(0)
        }
    }
    impl core::fmt::Debug for WAKENA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKENA")
                .field("POR_DET", &self.POR_DET())
                .field("WAKEUP_FLAG", &self.WAKEUP_FLAG())
                .field("TIMER0_FLAG", &self.TIMER0_FLAG())
                .field("TIMER1_FLAG", &self.TIMER1_FLAG())
                .field("LDO_RDY", &self.LDO_RDY())
                .field("OSC_RDY", &self.OSC_RDY())
                .field("CLOCK_DET", &self.CLOCK_DET())
                .field("CONFIG_DET", &self.CONFIG_DET())
                .field("VOLT_DET", &self.VOLT_DET())
                .field("TEMP_DET", &self.TEMP_DET())
                .field("LIGHT_DET", &self.LIGHT_DET())
                .field("SEC0_DET", &self.SEC0_DET())
                .field("IRQ0_DET", &self.IRQ0_DET())
                .field("IRQ1_DET", &self.IRQ1_DET())
                .field("IRQ2_DET", &self.IRQ2_DET())
                .field("IRQ3_DET", &self.IRQ3_DET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKENA {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WAKENA {{ POR_DET: {=bool:?}, WAKEUP_FLAG: {=bool:?}, TIMER0_FLAG: {=bool:?}, TIMER1_FLAG: {=bool:?}, LDO_RDY: {=bool:?}, OSC_RDY: {=bool:?}, CLOCK_DET: {=bool:?}, CONFIG_DET: {=bool:?}, VOLT_DET: {=bool:?}, TEMP_DET: {=bool:?}, LIGHT_DET: {=bool:?}, SEC0_DET: {=bool:?}, IRQ0_DET: {=bool:?}, IRQ1_DET: {=bool:?}, IRQ2_DET: {=bool:?}, IRQ3_DET: {=bool:?} }}" , self . POR_DET () , self . WAKEUP_FLAG () , self . TIMER0_FLAG () , self . TIMER1_FLAG () , self . LDO_RDY () , self . OSC_RDY () , self . CLOCK_DET () , self . CONFIG_DET () , self . VOLT_DET () , self . TEMP_DET () , self . LIGHT_DET () , self . SEC0_DET () , self . IRQ0_DET () , self . IRQ1_DET () , self . IRQ2_DET () , self . IRQ3_DET ())
        }
    }
    #[doc = "Wake-up Enable B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKENB(pub u32);
    impl WAKENB {
        #[inline(always)]
        pub const fn INVERSE(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_INVERSE(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for WAKENB {
        #[inline(always)]
        fn default() -> WAKENB {
            WAKENB(0)
        }
    }
    impl core::fmt::Debug for WAKENB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKENB")
                .field("INVERSE", &self.INVERSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKENB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WAKENB {{ INVERSE: {=u32:?} }}", self.INVERSE())
        }
    }
    #[doc = "Wakeup Lock A"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKLCKA(pub u32);
    impl WAKLCKA {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WAKLCKA {
        #[inline(always)]
        fn default() -> WAKLCKA {
            WAKLCKA(0)
        }
    }
    impl core::fmt::Debug for WAKLCKA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKLCKA")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKLCKA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WAKLCKA {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
    #[doc = "Wakeup Lock B"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WAKLCKB(pub u32);
    impl WAKLCKB {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WAKLCKB {
        #[inline(always)]
        fn default() -> WAKLCKB {
            WAKLCKB(0)
        }
    }
    impl core::fmt::Debug for WAKLCKB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WAKLCKB")
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WAKLCKB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WAKLCKB {{ LOCK: {=bool:?} }}", self.LOCK())
        }
    }
}
