#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSCON {
    ptr: *mut u8,
}
unsafe impl Send for SYSCON {}
unsafe impl Sync for SYSCON {}
impl SYSCON {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn REMAP(self) -> crate::common::Reg<regs::REMAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBMATPRIO(self) -> crate::common::Reg<regs::AHBMATPRIO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn CPU0NSTCKCAL(self) -> crate::common::Reg<regs::CPU0NSTCKCAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn NMISRC(self) -> crate::common::Reg<regs::NMISRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn SLOWCLKDIV(self) -> crate::common::Reg<regs::SLOWCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKDIV(self) -> crate::common::Reg<regs::AHBCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn CLKUNLOCK(self) -> crate::common::Reg<regs::CLKUNLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
    #[inline(always)]
    pub const fn NVM_CTRL(self) -> crate::common::Reg<regs::NVM_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn ROMCR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn CPUSTAT(self) -> crate::common::Reg<regs::CPUSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
    #[inline(always)]
    pub const fn LPCAC_CTRL(self) -> crate::common::Reg<regs::LPCAC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0824usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM0SUBCTL(self) -> crate::common::Reg<regs::PWM0SUBCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0938usize) as _) }
    }
    #[inline(always)]
    pub const fn PWM1SUBCTL(self) -> crate::common::Reg<regs::PWM1SUBCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x093cusize) as _) }
    }
    #[inline(always)]
    pub const fn CTIMERGLOBALSTARTEN(
        self,
    ) -> crate::common::Reg<regs::CTIMERGLOBALSTARTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0940usize) as _) }
    }
    #[inline(always)]
    pub const fn RAM_CTRL(self) -> crate::common::Reg<regs::RAM_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0944usize) as _) }
    }
    #[inline(always)]
    pub const fn GRAY_CODE_LSB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b60usize) as _) }
    }
    #[inline(always)]
    pub const fn GRAY_CODE_MSB(self) -> crate::common::Reg<regs::GRAY_CODE_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b64usize) as _) }
    }
    #[inline(always)]
    pub const fn BINARY_CODE_LSB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b68usize) as _) }
    }
    #[inline(always)]
    pub const fn BINARY_CODE_MSB(
        self,
    ) -> crate::common::Reg<regs::BINARY_CODE_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b6cusize) as _) }
    }
    #[inline(always)]
    pub const fn ROP_STATE(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e3cusize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_XEN(self) -> crate::common::Reg<regs::SRAM_XEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e58usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_XEN_DP(self) -> crate::common::Reg<regs::SRAM_XEN_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e5cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_OTP_LC_STATE(
        self,
    ) -> crate::common::Reg<regs::ELS_OTP_LC_STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e80usize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_OTP_LC_STATE_DP(
        self,
    ) -> crate::common::Reg<regs::ELS_OTP_LC_STATE_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e84usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_LOCK_EN(self) -> crate::common::Reg<regs::DEBUG_LOCK_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa0usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_FEATURES(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_FEATURES_DP(
        self,
    ) -> crate::common::Reg<regs::DEBUG_FEATURES_DP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fa8usize) as _) }
    }
    #[inline(always)]
    pub const fn SWD_ACCESS_CPU0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fb4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEBUG_AUTH_BEACON(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn JTAG_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[inline(always)]
    pub const fn DEVICE_TYPE(self) -> crate::common::Reg<regs::DEVICE_TYPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[inline(always)]
    pub const fn DEVICE_ID0(self) -> crate::common::Reg<regs::DEVICE_ID0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[inline(always)]
    pub const fn DIEID(self) -> crate::common::Reg<regs::DIEID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "System Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCLKDIV(pub u32);
    impl AHBCLKDIV {
        #[inline(always)]
        pub const fn DIV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AHBCLKDIV {
        #[inline(always)]
        fn default() -> AHBCLKDIV {
            AHBCLKDIV(0)
        }
    }
    impl core::fmt::Debug for AHBCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCLKDIV")
                .field("DIV", &self.DIV())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBCLKDIV {{ DIV: {=u8:?}, UNSTAB: {=bool:?} }}",
                self.DIV(),
                self.UNSTAB()
            )
        }
    }
    #[doc = "AHB Matrix Priority Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBMATPRIO(pub u32);
    impl AHBMATPRIO {
        #[inline(always)]
        pub const fn CPU0_CBUS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_CBUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0_SBUS(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SBUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn USB_FS_ENET(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_USB_FS_ENET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for AHBMATPRIO {
        #[inline(always)]
        fn default() -> AHBMATPRIO {
            AHBMATPRIO(0)
        }
    }
    impl core::fmt::Debug for AHBMATPRIO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBMATPRIO")
                .field("CPU0_CBUS", &self.CPU0_CBUS())
                .field("CPU0_SBUS", &self.CPU0_SBUS())
                .field("DMA0", &self.DMA0())
                .field("USB_FS_ENET", &self.USB_FS_ENET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBMATPRIO {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHBMATPRIO {{ CPU0_CBUS: {=u8:?}, CPU0_SBUS: {=u8:?}, DMA0: {=u8:?}, USB_FS_ENET: {=u8:?} }}" , self . CPU0_CBUS () , self . CPU0_SBUS () , self . DMA0 () , self . USB_FS_ENET ())
        }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BINARY_CODE_MSB(pub u32);
    impl BINARY_CODE_MSB {
        #[inline(always)]
        pub const fn code_bin_41_32(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_code_bin_41_32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for BINARY_CODE_MSB {
        #[inline(always)]
        fn default() -> BINARY_CODE_MSB {
            BINARY_CODE_MSB(0)
        }
    }
    impl core::fmt::Debug for BINARY_CODE_MSB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BINARY_CODE_MSB")
                .field("code_bin_41_32", &self.code_bin_41_32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BINARY_CODE_MSB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BINARY_CODE_MSB {{ code_bin_41_32: {=u16:?} }}",
                self.code_bin_41_32()
            )
        }
    }
    #[doc = "Clock Configuration Unlock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CLKUNLOCK(pub u32);
    impl CLKUNLOCK {
        #[inline(always)]
        pub const fn UNLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CLKUNLOCK {
        #[inline(always)]
        fn default() -> CLKUNLOCK {
            CLKUNLOCK(0)
        }
    }
    impl core::fmt::Debug for CLKUNLOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CLKUNLOCK")
                .field("UNLOCK", &self.UNLOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CLKUNLOCK {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CLKUNLOCK {{ UNLOCK: {=bool:?} }}", self.UNLOCK())
        }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPU0NSTCKCAL(pub u32);
    impl CPU0NSTCKCAL {
        #[inline(always)]
        pub const fn TENMS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TENMS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn SKEW(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SKEW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn NOREF(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NOREF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for CPU0NSTCKCAL {
        #[inline(always)]
        fn default() -> CPU0NSTCKCAL {
            CPU0NSTCKCAL(0)
        }
    }
    impl core::fmt::Debug for CPU0NSTCKCAL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPU0NSTCKCAL")
                .field("TENMS", &self.TENMS())
                .field("SKEW", &self.SKEW())
                .field("NOREF", &self.NOREF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPU0NSTCKCAL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CPU0NSTCKCAL {{ TENMS: {=u32:?}, SKEW: {=bool:?}, NOREF: {=bool:?} }}",
                self.TENMS(),
                self.SKEW(),
                self.NOREF()
            )
        }
    }
    #[doc = "CPU Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CPUSTAT(pub u32);
    impl CPUSTAT {
        #[inline(always)]
        pub const fn CPU0SLEEPING(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPU0SLEEPING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0LOCKUP(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPU0LOCKUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CPUSTAT {
        #[inline(always)]
        fn default() -> CPUSTAT {
            CPUSTAT(0)
        }
    }
    impl core::fmt::Debug for CPUSTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CPUSTAT")
                .field("CPU0SLEEPING", &self.CPU0SLEEPING())
                .field("CPU0LOCKUP", &self.CPU0LOCKUP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CPUSTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CPUSTAT {{ CPU0SLEEPING: {=bool:?}, CPU0LOCKUP: {=bool:?} }}",
                self.CPU0SLEEPING(),
                self.CPU0LOCKUP()
            )
        }
    }
    #[doc = "CTIMER Global Start Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CTIMERGLOBALSTARTEN(pub u32);
    impl CTIMERGLOBALSTARTEN {
        #[inline(always)]
        pub const fn CTIMER0_CLK_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER0_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CTIMER1_CLK_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER1_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CTIMER2_CLK_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER2_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CTIMER3_CLK_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER3_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn CTIMER4_CLK_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CTIMER4_CLK_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for CTIMERGLOBALSTARTEN {
        #[inline(always)]
        fn default() -> CTIMERGLOBALSTARTEN {
            CTIMERGLOBALSTARTEN(0)
        }
    }
    impl core::fmt::Debug for CTIMERGLOBALSTARTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CTIMERGLOBALSTARTEN")
                .field("CTIMER0_CLK_EN", &self.CTIMER0_CLK_EN())
                .field("CTIMER1_CLK_EN", &self.CTIMER1_CLK_EN())
                .field("CTIMER2_CLK_EN", &self.CTIMER2_CLK_EN())
                .field("CTIMER3_CLK_EN", &self.CTIMER3_CLK_EN())
                .field("CTIMER4_CLK_EN", &self.CTIMER4_CLK_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CTIMERGLOBALSTARTEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CTIMERGLOBALSTARTEN {{ CTIMER0_CLK_EN: {=bool:?}, CTIMER1_CLK_EN: {=bool:?}, CTIMER2_CLK_EN: {=bool:?}, CTIMER3_CLK_EN: {=bool:?}, CTIMER4_CLK_EN: {=bool:?} }}" , self . CTIMER0_CLK_EN () , self . CTIMER1_CLK_EN () , self . CTIMER2_CLK_EN () , self . CTIMER3_CLK_EN () , self . CTIMER4_CLK_EN ())
        }
    }
    #[doc = "Cortex Debug Features Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_FEATURES(pub u32);
    impl DEBUG_FEATURES {
        #[inline(always)]
        pub const fn CPU0_DBGEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_DBGEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0_NIDEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_NIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for DEBUG_FEATURES {
        #[inline(always)]
        fn default() -> DEBUG_FEATURES {
            DEBUG_FEATURES(0)
        }
    }
    impl core::fmt::Debug for DEBUG_FEATURES {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_FEATURES")
                .field("CPU0_DBGEN", &self.CPU0_DBGEN())
                .field("CPU0_NIDEN", &self.CPU0_NIDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_FEATURES {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DEBUG_FEATURES {{ CPU0_DBGEN: {=u8:?}, CPU0_NIDEN: {=u8:?} }}",
                self.CPU0_DBGEN(),
                self.CPU0_NIDEN()
            )
        }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_FEATURES_DP(pub u32);
    impl DEBUG_FEATURES_DP {
        #[inline(always)]
        pub const fn CPU0_DBGEN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_DBGEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CPU0_NIDEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_NIDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for DEBUG_FEATURES_DP {
        #[inline(always)]
        fn default() -> DEBUG_FEATURES_DP {
            DEBUG_FEATURES_DP(0)
        }
    }
    impl core::fmt::Debug for DEBUG_FEATURES_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_FEATURES_DP")
                .field("CPU0_DBGEN", &self.CPU0_DBGEN())
                .field("CPU0_NIDEN", &self.CPU0_NIDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_FEATURES_DP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DEBUG_FEATURES_DP {{ CPU0_DBGEN: {=u8:?}, CPU0_NIDEN: {=u8:?} }}",
                self.CPU0_DBGEN(),
                self.CPU0_NIDEN()
            )
        }
    }
    #[doc = "Control Write Access to Security"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEBUG_LOCK_EN(pub u32);
    impl DEBUG_LOCK_EN {
        #[inline(always)]
        pub const fn LOCK_ALL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_LOCK_ALL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for DEBUG_LOCK_EN {
        #[inline(always)]
        fn default() -> DEBUG_LOCK_EN {
            DEBUG_LOCK_EN(0)
        }
    }
    impl core::fmt::Debug for DEBUG_LOCK_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEBUG_LOCK_EN")
                .field("LOCK_ALL", &self.LOCK_ALL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEBUG_LOCK_EN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DEBUG_LOCK_EN {{ LOCK_ALL: {=u8:?} }}", self.LOCK_ALL())
        }
    }
    #[doc = "Device ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEVICE_ID0(pub u32);
    impl DEVICE_ID0 {
        #[inline(always)]
        pub const fn RAM_SIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RAM_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn FLASH_SIZE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_FLASH_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn ROM_REV_MINOR(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ROM_REV_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn SECURITY(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SECURITY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for DEVICE_ID0 {
        #[inline(always)]
        fn default() -> DEVICE_ID0 {
            DEVICE_ID0(0)
        }
    }
    impl core::fmt::Debug for DEVICE_ID0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEVICE_ID0")
                .field("RAM_SIZE", &self.RAM_SIZE())
                .field("FLASH_SIZE", &self.FLASH_SIZE())
                .field("ROM_REV_MINOR", &self.ROM_REV_MINOR())
                .field("SECURITY", &self.SECURITY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEVICE_ID0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DEVICE_ID0 {{ RAM_SIZE: {=u8:?}, FLASH_SIZE: {=u8:?}, ROM_REV_MINOR: {=u8:?}, SECURITY: {=u8:?} }}" , self . RAM_SIZE () , self . FLASH_SIZE () , self . ROM_REV_MINOR () , self . SECURITY ())
        }
    }
    #[doc = "Device Type"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DEVICE_TYPE(pub u32);
    impl DEVICE_TYPE {
        #[inline(always)]
        pub const fn DEVICE_TYPE_NUM(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DEVICE_TYPE_NUM(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn DEVICE_TYPE_SEC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEVICE_TYPE_SEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn DEVICE_TYPE_PKG(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DEVICE_TYPE_PKG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[inline(always)]
        pub const fn DEVICE_TYPE_PIN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_DEVICE_TYPE_PIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DEVICE_TYPE {
        #[inline(always)]
        fn default() -> DEVICE_TYPE {
            DEVICE_TYPE(0)
        }
    }
    impl core::fmt::Debug for DEVICE_TYPE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DEVICE_TYPE")
                .field("DEVICE_TYPE_NUM", &self.DEVICE_TYPE_NUM())
                .field("DEVICE_TYPE_SEC", &self.DEVICE_TYPE_SEC())
                .field("DEVICE_TYPE_PKG", &self.DEVICE_TYPE_PKG())
                .field("DEVICE_TYPE_PIN", &self.DEVICE_TYPE_PIN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DEVICE_TYPE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DEVICE_TYPE {{ DEVICE_TYPE_NUM: {=u16:?}, DEVICE_TYPE_SEC: {=bool:?}, DEVICE_TYPE_PKG: {=u8:?}, DEVICE_TYPE_PIN: {=u8:?} }}" , self . DEVICE_TYPE_NUM () , self . DEVICE_TYPE_SEC () , self . DEVICE_TYPE_PKG () , self . DEVICE_TYPE_PIN ())
        }
    }
    #[doc = "Chip Revision ID and Number"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DIEID(pub u32);
    impl DIEID {
        #[inline(always)]
        pub const fn MINOR_REVISION(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MINOR_REVISION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn MAJOR_REVISION(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MAJOR_REVISION(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn MCO_NUM_IN_DIE_ID(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_MCO_NUM_IN_DIE_ID(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
        }
    }
    impl Default for DIEID {
        #[inline(always)]
        fn default() -> DIEID {
            DIEID(0)
        }
    }
    impl core::fmt::Debug for DIEID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DIEID")
                .field("MINOR_REVISION", &self.MINOR_REVISION())
                .field("MAJOR_REVISION", &self.MAJOR_REVISION())
                .field("MCO_NUM_IN_DIE_ID", &self.MCO_NUM_IN_DIE_ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DIEID {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DIEID {{ MINOR_REVISION: {=u8:?}, MAJOR_REVISION: {=u8:?}, MCO_NUM_IN_DIE_ID: {=u32:?} }}" , self . MINOR_REVISION () , self . MAJOR_REVISION () , self . MCO_NUM_IN_DIE_ID ())
        }
    }
    #[doc = "Life Cycle State Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_OTP_LC_STATE(pub u32);
    impl ELS_OTP_LC_STATE {
        #[inline(always)]
        pub const fn OTP_LC_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OTP_LC_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ELS_OTP_LC_STATE {
        #[inline(always)]
        fn default() -> ELS_OTP_LC_STATE {
            ELS_OTP_LC_STATE(0)
        }
    }
    impl core::fmt::Debug for ELS_OTP_LC_STATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_OTP_LC_STATE")
                .field("OTP_LC_STATE", &self.OTP_LC_STATE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_OTP_LC_STATE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ELS_OTP_LC_STATE {{ OTP_LC_STATE: {=u8:?} }}",
                self.OTP_LC_STATE()
            )
        }
    }
    #[doc = "Life Cycle State Register (Duplicate)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_OTP_LC_STATE_DP(pub u32);
    impl ELS_OTP_LC_STATE_DP {
        #[inline(always)]
        pub const fn OTP_LC_STATE_DP(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OTP_LC_STATE_DP(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for ELS_OTP_LC_STATE_DP {
        #[inline(always)]
        fn default() -> ELS_OTP_LC_STATE_DP {
            ELS_OTP_LC_STATE_DP(0)
        }
    }
    impl core::fmt::Debug for ELS_OTP_LC_STATE_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_OTP_LC_STATE_DP")
                .field("OTP_LC_STATE_DP", &self.OTP_LC_STATE_DP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_OTP_LC_STATE_DP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ELS_OTP_LC_STATE_DP {{ OTP_LC_STATE_DP: {=u8:?} }}",
                self.OTP_LC_STATE_DP()
            )
        }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[41:32\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GRAY_CODE_MSB(pub u32);
    impl GRAY_CODE_MSB {
        #[inline(always)]
        pub const fn code_gray_41_32(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_code_gray_41_32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for GRAY_CODE_MSB {
        #[inline(always)]
        fn default() -> GRAY_CODE_MSB {
            GRAY_CODE_MSB(0)
        }
    }
    impl core::fmt::Debug for GRAY_CODE_MSB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GRAY_CODE_MSB")
                .field("code_gray_41_32", &self.code_gray_41_32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GRAY_CODE_MSB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GRAY_CODE_MSB {{ code_gray_41_32: {=u16:?} }}",
                self.code_gray_41_32()
            )
        }
    }
    #[doc = "LPCAC Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LPCAC_CTRL(pub u32);
    impl LPCAC_CTRL {
        #[inline(always)]
        pub const fn DIS_LPCAC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_LPCAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLR_LPCAC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLR_LPCAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FRC_NO_ALLOC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FRC_NO_ALLOC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIS_LPCAC_WTBF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_LPCAC_WTBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn LIM_LPCAC_WTBF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LIM_LPCAC_WTBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn LPCAC_XOM(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPCAC_XOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn LPCAC_MEM_REQ(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LPCAC_MEM_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for LPCAC_CTRL {
        #[inline(always)]
        fn default() -> LPCAC_CTRL {
            LPCAC_CTRL(0)
        }
    }
    impl core::fmt::Debug for LPCAC_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LPCAC_CTRL")
                .field("DIS_LPCAC", &self.DIS_LPCAC())
                .field("CLR_LPCAC", &self.CLR_LPCAC())
                .field("FRC_NO_ALLOC", &self.FRC_NO_ALLOC())
                .field("DIS_LPCAC_WTBF", &self.DIS_LPCAC_WTBF())
                .field("LIM_LPCAC_WTBF", &self.LIM_LPCAC_WTBF())
                .field("LPCAC_XOM", &self.LPCAC_XOM())
                .field("LPCAC_MEM_REQ", &self.LPCAC_MEM_REQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LPCAC_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LPCAC_CTRL {{ DIS_LPCAC: {=bool:?}, CLR_LPCAC: {=bool:?}, FRC_NO_ALLOC: {=bool:?}, DIS_LPCAC_WTBF: {=bool:?}, LIM_LPCAC_WTBF: {=bool:?}, LPCAC_XOM: {=bool:?}, LPCAC_MEM_REQ: {=bool:?} }}" , self . DIS_LPCAC () , self . CLR_LPCAC () , self . FRC_NO_ALLOC () , self . DIS_LPCAC_WTBF () , self . LIM_LPCAC_WTBF () , self . LPCAC_XOM () , self . LPCAC_MEM_REQ ())
        }
    }
    #[doc = "NMI Source Select"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NMISRC(pub u32);
    impl NMISRC {
        #[inline(always)]
        pub const fn IRQCPU0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_IRQCPU0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NMIENCPU0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_NMIENCPU0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for NMISRC {
        #[inline(always)]
        fn default() -> NMISRC {
            NMISRC(0)
        }
    }
    impl core::fmt::Debug for NMISRC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NMISRC")
                .field("IRQCPU0", &self.IRQCPU0())
                .field("NMIENCPU0", &self.NMIENCPU0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NMISRC {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "NMISRC {{ IRQCPU0: {=u8:?}, NMIENCPU0: {=bool:?} }}",
                self.IRQCPU0(),
                self.NMIENCPU0()
            )
        }
    }
    #[doc = "NVM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NVM_CTRL(pub u32);
    impl NVM_CTRL {
        #[inline(always)]
        pub const fn DIS_FLASH_SPEC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_FLASH_SPEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DIS_DATA_SPEC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_DATA_SPEC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn FLASH_STALL_EN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_FLASH_STALL_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn DIS_MBECC_ERR_INST(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_MBECC_ERR_INST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn DIS_MBECC_ERR_DATA(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_MBECC_ERR_DATA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for NVM_CTRL {
        #[inline(always)]
        fn default() -> NVM_CTRL {
            NVM_CTRL(0)
        }
    }
    impl core::fmt::Debug for NVM_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NVM_CTRL")
                .field("DIS_FLASH_SPEC", &self.DIS_FLASH_SPEC())
                .field("DIS_DATA_SPEC", &self.DIS_DATA_SPEC())
                .field("FLASH_STALL_EN", &self.FLASH_STALL_EN())
                .field("DIS_MBECC_ERR_INST", &self.DIS_MBECC_ERR_INST())
                .field("DIS_MBECC_ERR_DATA", &self.DIS_MBECC_ERR_DATA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NVM_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "NVM_CTRL {{ DIS_FLASH_SPEC: {=bool:?}, DIS_DATA_SPEC: {=bool:?}, FLASH_STALL_EN: {=bool:?}, DIS_MBECC_ERR_INST: {=bool:?}, DIS_MBECC_ERR_DATA: {=bool:?} }}" , self . DIS_FLASH_SPEC () , self . DIS_DATA_SPEC () , self . FLASH_STALL_EN () , self . DIS_MBECC_ERR_INST () , self . DIS_MBECC_ERR_DATA ())
        }
    }
    #[doc = "PWM0 Submodule Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM0SUBCTL(pub u32);
    impl PWM0SUBCTL {
        #[inline(always)]
        pub const fn CLK0_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK0_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLK1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLK2_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLK3_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK3_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PWM0SUBCTL {
        #[inline(always)]
        fn default() -> PWM0SUBCTL {
            PWM0SUBCTL(0)
        }
    }
    impl core::fmt::Debug for PWM0SUBCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM0SUBCTL")
                .field("CLK0_EN", &self.CLK0_EN())
                .field("CLK1_EN", &self.CLK1_EN())
                .field("CLK2_EN", &self.CLK2_EN())
                .field("CLK3_EN", &self.CLK3_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM0SUBCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PWM0SUBCTL {{ CLK0_EN: {=bool:?}, CLK1_EN: {=bool:?}, CLK2_EN: {=bool:?}, CLK3_EN: {=bool:?} }}" , self . CLK0_EN () , self . CLK1_EN () , self . CLK2_EN () , self . CLK3_EN ())
        }
    }
    #[doc = "PWM1 Submodule Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PWM1SUBCTL(pub u32);
    impl PWM1SUBCTL {
        #[inline(always)]
        pub const fn CLK0_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK0_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLK1_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK1_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLK2_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CLK3_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLK3_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PWM1SUBCTL {
        #[inline(always)]
        fn default() -> PWM1SUBCTL {
            PWM1SUBCTL(0)
        }
    }
    impl core::fmt::Debug for PWM1SUBCTL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PWM1SUBCTL")
                .field("CLK0_EN", &self.CLK0_EN())
                .field("CLK1_EN", &self.CLK1_EN())
                .field("CLK2_EN", &self.CLK2_EN())
                .field("CLK3_EN", &self.CLK3_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PWM1SUBCTL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PWM1SUBCTL {{ CLK0_EN: {=bool:?}, CLK1_EN: {=bool:?}, CLK2_EN: {=bool:?}, CLK3_EN: {=bool:?} }}" , self . CLK0_EN () , self . CLK1_EN () , self . CLK2_EN () , self . CLK3_EN ())
        }
    }
    #[doc = "RAM Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAM_CTRL(pub u32);
    impl RAM_CTRL {
        #[inline(always)]
        pub const fn RAMA_ECC_ENABLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA_ECC_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RAMA_CG_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA_CG_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn RAMX_CG_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX_CG_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn RAMB_CG_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_CG_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for RAM_CTRL {
        #[inline(always)]
        fn default() -> RAM_CTRL {
            RAM_CTRL(0)
        }
    }
    impl core::fmt::Debug for RAM_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAM_CTRL")
                .field("RAMA_ECC_ENABLE", &self.RAMA_ECC_ENABLE())
                .field("RAMA_CG_OVERRIDE", &self.RAMA_CG_OVERRIDE())
                .field("RAMX_CG_OVERRIDE", &self.RAMX_CG_OVERRIDE())
                .field("RAMB_CG_OVERRIDE", &self.RAMB_CG_OVERRIDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAM_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RAM_CTRL {{ RAMA_ECC_ENABLE: {=bool:?}, RAMA_CG_OVERRIDE: {=bool:?}, RAMX_CG_OVERRIDE: {=bool:?}, RAMB_CG_OVERRIDE: {=bool:?} }}" , self . RAMA_ECC_ENABLE () , self . RAMA_CG_OVERRIDE () , self . RAMX_CG_OVERRIDE () , self . RAMB_CG_OVERRIDE ())
        }
    }
    #[doc = "AHB Matrix Remap Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REMAP(pub u32);
    impl REMAP {
        #[inline(always)]
        pub const fn CPU0_SBUS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU0_SBUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn DMA0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_DMA0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn USB0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_USB0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for REMAP {
        #[inline(always)]
        fn default() -> REMAP {
            REMAP(0)
        }
    }
    impl core::fmt::Debug for REMAP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("REMAP")
                .field("CPU0_SBUS", &self.CPU0_SBUS())
                .field("DMA0", &self.DMA0())
                .field("USB0", &self.USB0())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REMAP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "REMAP {{ CPU0_SBUS: {=u8:?}, DMA0: {=u8:?}, USB0: {=u8:?}, LOCK: {=bool:?} }}",
                self.CPU0_SBUS(),
                self.DMA0(),
                self.USB0(),
                self.LOCK()
            )
        }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SLOWCLKDIV(pub u32);
    impl SLOWCLKDIV {
        #[inline(always)]
        pub const fn RESET(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn HALT(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HALT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn UNSTAB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNSTAB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SLOWCLKDIV {
        #[inline(always)]
        fn default() -> SLOWCLKDIV {
            SLOWCLKDIV(0)
        }
    }
    impl core::fmt::Debug for SLOWCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SLOWCLKDIV")
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SLOWCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SLOWCLKDIV {{ RESET: {=bool:?}, HALT: {=bool:?}, UNSTAB: {=bool:?} }}",
                self.RESET(),
                self.HALT(),
                self.UNSTAB()
            )
        }
    }
    #[doc = "RAM XEN Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_XEN(pub u32);
    impl SRAM_XEN {
        #[inline(always)]
        pub const fn RAMX0_XEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX0_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RAMX1_XEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX1_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMA0_XEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA0_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMA1_XEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA1_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RAMB_XEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SRAM_XEN {
        #[inline(always)]
        fn default() -> SRAM_XEN {
            SRAM_XEN(0)
        }
    }
    impl core::fmt::Debug for SRAM_XEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_XEN")
                .field("RAMX0_XEN", &self.RAMX0_XEN())
                .field("RAMX1_XEN", &self.RAMX1_XEN())
                .field("RAMA0_XEN", &self.RAMA0_XEN())
                .field("RAMA1_XEN", &self.RAMA1_XEN())
                .field("RAMB_XEN", &self.RAMB_XEN())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_XEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRAM_XEN {{ RAMX0_XEN: {=bool:?}, RAMX1_XEN: {=bool:?}, RAMA0_XEN: {=bool:?}, RAMA1_XEN: {=bool:?}, RAMB_XEN: {=bool:?}, LOCK: {=bool:?} }}" , self . RAMX0_XEN () , self . RAMX1_XEN () , self . RAMA0_XEN () , self . RAMA1_XEN () , self . RAMB_XEN () , self . LOCK ())
        }
    }
    #[doc = "RAM XEN Control (Duplicate)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_XEN_DP(pub u32);
    impl SRAM_XEN_DP {
        #[inline(always)]
        pub const fn RAMX0_XEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX0_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RAMX1_XEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMX1_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RAMA0_XEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA0_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RAMA1_XEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMA1_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn RAMB_XEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMB_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for SRAM_XEN_DP {
        #[inline(always)]
        fn default() -> SRAM_XEN_DP {
            SRAM_XEN_DP(0)
        }
    }
    impl core::fmt::Debug for SRAM_XEN_DP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_XEN_DP")
                .field("RAMX0_XEN", &self.RAMX0_XEN())
                .field("RAMX1_XEN", &self.RAMX1_XEN())
                .field("RAMA0_XEN", &self.RAMA0_XEN())
                .field("RAMA1_XEN", &self.RAMA1_XEN())
                .field("RAMB_XEN", &self.RAMB_XEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_XEN_DP {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SRAM_XEN_DP {{ RAMX0_XEN: {=bool:?}, RAMX1_XEN: {=bool:?}, RAMA0_XEN: {=bool:?}, RAMA1_XEN: {=bool:?}, RAMB_XEN: {=bool:?} }}" , self . RAMX0_XEN () , self . RAMX1_XEN () , self . RAMA0_XEN () , self . RAMA1_XEN () , self . RAMB_XEN ())
        }
    }
}
