#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn PROTLVL(self) -> crate::common::Reg<regs::PROTLVL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn SLOWCLKDIV(self) -> crate::common::Reg<regs::SLOWCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[inline(always)]
    pub const fn BUSCLKDIV(self) -> crate::common::Reg<regs::BUSCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCLKDIV(self) -> crate::common::Reg<regs::AHBCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn FROHFDIV(self) -> crate::common::Reg<regs::FROHFDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[inline(always)]
    pub const fn FROLFDIV(self) -> crate::common::Reg<regs::FROLFDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[inline(always)]
    pub const fn PLL1CLKDIV(self) -> crate::common::Reg<regs::PLL1CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
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
    pub const fn SMARTDMAINT(self) -> crate::common::Reg<regs::SMARTDMAINT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[inline(always)]
    pub const fn RAM_INTERLEAVE(
        self,
    ) -> crate::common::Reg<regs::RAM_INTERLEAVE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
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
    pub const fn ELS_UDF(self) -> crate::common::Reg<regs::ELS_UDF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e10usize) as _) }
    }
    #[inline(always)]
    pub const fn MSFCFG(self) -> crate::common::Reg<regs::MSFCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e1cusize) as _) }
    }
    #[inline(always)]
    pub const fn ELS_UID(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e20usize + n * 4usize) as _) }
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
            #[derive(defmt :: Format)]
            struct AHBCLKDIV {
                DIV: u8,
                UNSTAB: bool,
            }
            let proxy = AHBCLKDIV {
                DIV: self.DIV(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
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
        pub const fn CPU1_CBUS_SmartDMA_I(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU1_CBUS_SmartDMA_I(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn CPU1_SBUS_SmartDMA_D(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CPU1_SBUS_SmartDMA_D(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
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
        pub const fn PKC_ELS(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PKC_ELS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
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
                .field("CPU1_CBUS_SmartDMA_I", &self.CPU1_CBUS_SmartDMA_I())
                .field("CPU1_SBUS_SmartDMA_D", &self.CPU1_SBUS_SmartDMA_D())
                .field("DMA0", &self.DMA0())
                .field("PKC_ELS", &self.PKC_ELS())
                .field("USB_FS_ENET", &self.USB_FS_ENET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBMATPRIO {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AHBMATPRIO {
                CPU0_CBUS: u8,
                CPU0_SBUS: u8,
                CPU1_CBUS_SmartDMA_I: u8,
                CPU1_SBUS_SmartDMA_D: u8,
                DMA0: u8,
                PKC_ELS: u8,
                USB_FS_ENET: u8,
            }
            let proxy = AHBMATPRIO {
                CPU0_CBUS: self.CPU0_CBUS(),
                CPU0_SBUS: self.CPU0_SBUS(),
                CPU1_CBUS_SmartDMA_I: self.CPU1_CBUS_SmartDMA_I(),
                CPU1_SBUS_SmartDMA_D: self.CPU1_SBUS_SmartDMA_D(),
                DMA0: self.DMA0(),
                PKC_ELS: self.PKC_ELS(),
                USB_FS_ENET: self.USB_FS_ENET(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct BINARY_CODE_MSB {
                code_bin_41_32: u16,
            }
            let proxy = BINARY_CODE_MSB {
                code_bin_41_32: self.code_bin_41_32(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "BUS_CLK Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BUSCLKDIV(pub u32);
    impl BUSCLKDIV {
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
    impl Default for BUSCLKDIV {
        #[inline(always)]
        fn default() -> BUSCLKDIV {
            BUSCLKDIV(0)
        }
    }
    impl core::fmt::Debug for BUSCLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BUSCLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BUSCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct BUSCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = BUSCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct CLKUNLOCK {
                UNLOCK: bool,
            }
            let proxy = CLKUNLOCK {
                UNLOCK: self.UNLOCK(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct CPU0NSTCKCAL {
                TENMS: u32,
                SKEW: bool,
                NOREF: bool,
            }
            let proxy = CPU0NSTCKCAL {
                TENMS: self.TENMS(),
                SKEW: self.SKEW(),
                NOREF: self.NOREF(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct CPUSTAT {
                CPU0SLEEPING: bool,
                CPU0LOCKUP: bool,
            }
            let proxy = CPUSTAT {
                CPU0SLEEPING: self.CPU0SLEEPING(),
                CPU0LOCKUP: self.CPU0LOCKUP(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct CTIMERGLOBALSTARTEN {
                CTIMER0_CLK_EN: bool,
                CTIMER1_CLK_EN: bool,
                CTIMER2_CLK_EN: bool,
                CTIMER3_CLK_EN: bool,
                CTIMER4_CLK_EN: bool,
            }
            let proxy = CTIMERGLOBALSTARTEN {
                CTIMER0_CLK_EN: self.CTIMER0_CLK_EN(),
                CTIMER1_CLK_EN: self.CTIMER1_CLK_EN(),
                CTIMER2_CLK_EN: self.CTIMER2_CLK_EN(),
                CTIMER3_CLK_EN: self.CTIMER3_CLK_EN(),
                CTIMER4_CLK_EN: self.CTIMER4_CLK_EN(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DEBUG_FEATURES {
                CPU0_DBGEN: u8,
                CPU0_NIDEN: u8,
            }
            let proxy = DEBUG_FEATURES {
                CPU0_DBGEN: self.CPU0_DBGEN(),
                CPU0_NIDEN: self.CPU0_NIDEN(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DEBUG_FEATURES_DP {
                CPU0_DBGEN: u8,
                CPU0_NIDEN: u8,
            }
            let proxy = DEBUG_FEATURES_DP {
                CPU0_DBGEN: self.CPU0_DBGEN(),
                CPU0_NIDEN: self.CPU0_NIDEN(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DEBUG_LOCK_EN {
                LOCK_ALL: u8,
            }
            let proxy = DEBUG_LOCK_EN {
                LOCK_ALL: self.LOCK_ALL(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DEVICE_ID0 {
                RAM_SIZE: u8,
                FLASH_SIZE: u8,
                ROM_REV_MINOR: u8,
                SECURITY: u8,
            }
            let proxy = DEVICE_ID0 {
                RAM_SIZE: self.RAM_SIZE(),
                FLASH_SIZE: self.FLASH_SIZE(),
                ROM_REV_MINOR: self.ROM_REV_MINOR(),
                SECURITY: self.SECURITY(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DEVICE_TYPE {
                DEVICE_TYPE_NUM: u16,
                DEVICE_TYPE_SEC: bool,
                DEVICE_TYPE_PKG: u8,
                DEVICE_TYPE_PIN: u8,
            }
            let proxy = DEVICE_TYPE {
                DEVICE_TYPE_NUM: self.DEVICE_TYPE_NUM(),
                DEVICE_TYPE_SEC: self.DEVICE_TYPE_SEC(),
                DEVICE_TYPE_PKG: self.DEVICE_TYPE_PKG(),
                DEVICE_TYPE_PIN: self.DEVICE_TYPE_PIN(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct DIEID {
                MINOR_REVISION: u8,
                MAJOR_REVISION: u8,
                MCO_NUM_IN_DIE_ID: u32,
            }
            let proxy = DIEID {
                MINOR_REVISION: self.MINOR_REVISION(),
                MAJOR_REVISION: self.MAJOR_REVISION(),
                MCO_NUM_IN_DIE_ID: self.MCO_NUM_IN_DIE_ID(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct ELS_OTP_LC_STATE {
                OTP_LC_STATE: u8,
            }
            let proxy = ELS_OTP_LC_STATE {
                OTP_LC_STATE: self.OTP_LC_STATE(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct ELS_OTP_LC_STATE_DP {
                OTP_LC_STATE_DP: u8,
            }
            let proxy = ELS_OTP_LC_STATE_DP {
                OTP_LC_STATE_DP: self.OTP_LC_STATE_DP(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "UDF Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_UDF(pub u32);
    impl ELS_UDF {
        #[inline(always)]
        pub const fn KEY_SEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_KEY_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn UID_HIDDEN(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_UID_HIDDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[inline(always)]
        pub const fn UDF_HIDDEN(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_UDF_HIDDEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for ELS_UDF {
        #[inline(always)]
        fn default() -> ELS_UDF {
            ELS_UDF(0)
        }
    }
    impl core::fmt::Debug for ELS_UDF {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_UDF")
                .field("KEY_SEL", &self.KEY_SEL())
                .field("UID_HIDDEN", &self.UID_HIDDEN())
                .field("UDF_HIDDEN", &self.UDF_HIDDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_UDF {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_UDF {
                KEY_SEL: u8,
                UID_HIDDEN: u8,
                UDF_HIDDEN: u8,
            }
            let proxy = ELS_UDF {
                KEY_SEL: self.KEY_SEL(),
                UID_HIDDEN: self.UID_HIDDEN(),
                UDF_HIDDEN: self.UDF_HIDDEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Device UID 0..Device UID 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ELS_UID(pub u32);
    impl ELS_UID {
        #[inline(always)]
        pub const fn UID0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_UID0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn UID1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_UID1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn UID2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_UID2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn UID3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_UID3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ELS_UID {
        #[inline(always)]
        fn default() -> ELS_UID {
            ELS_UID(0)
        }
    }
    impl core::fmt::Debug for ELS_UID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ELS_UID")
                .field("UID0", &self.UID0())
                .field("UID1", &self.UID1())
                .field("UID2", &self.UID2())
                .field("UID3", &self.UID3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ELS_UID {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ELS_UID {
                UID0: u32,
                UID1: u32,
                UID2: u32,
                UID3: u32,
            }
            let proxy = ELS_UID {
                UID0: self.UID0(),
                UID1: self.UID1(),
                UID2: self.UID2(),
                UID3: self.UID3(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO_HF_DIV Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROHFDIV(pub u32);
    impl FROHFDIV {
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
    impl Default for FROHFDIV {
        #[inline(always)]
        fn default() -> FROHFDIV {
            FROHFDIV(0)
        }
    }
    impl core::fmt::Debug for FROHFDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROHFDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROHFDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FROHFDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FROHFDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FRO_LF_DIV Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FROLFDIV(pub u32);
    impl FROLFDIV {
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
    impl Default for FROLFDIV {
        #[inline(always)]
        fn default() -> FROLFDIV {
            FROLFDIV(0)
        }
    }
    impl core::fmt::Debug for FROLFDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FROLFDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FROLFDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct FROLFDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = FROLFDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct GRAY_CODE_MSB {
                code_gray_41_32: u16,
            }
            let proxy = GRAY_CODE_MSB {
                code_gray_41_32: self.code_gray_41_32(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct LPCAC_CTRL {
                DIS_LPCAC: bool,
                CLR_LPCAC: bool,
                FRC_NO_ALLOC: bool,
                DIS_LPCAC_WTBF: bool,
                LIM_LPCAC_WTBF: bool,
                LPCAC_XOM: bool,
                LPCAC_MEM_REQ: bool,
            }
            let proxy = LPCAC_CTRL {
                DIS_LPCAC: self.DIS_LPCAC(),
                CLR_LPCAC: self.CLR_LPCAC(),
                FRC_NO_ALLOC: self.FRC_NO_ALLOC(),
                DIS_LPCAC_WTBF: self.DIS_LPCAC_WTBF(),
                LIM_LPCAC_WTBF: self.LIM_LPCAC_WTBF(),
                LPCAC_XOM: self.LPCAC_XOM(),
                LPCAC_MEM_REQ: self.LPCAC_MEM_REQ(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "MSF Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MSFCFG(pub u32);
    impl MSFCFG {
        #[inline(always)]
        pub const fn IFR_ERASE_DIS0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IFR_ERASE_DIS0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IFR_ERASE_DIS1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IFR_ERASE_DIS1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IFR_ERASE_DIS2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IFR_ERASE_DIS2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IFR_ERASE_DIS3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IFR_ERASE_DIS3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn MASS_ERASE_DIS(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MASS_ERASE_DIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MSFCFG {
        #[inline(always)]
        fn default() -> MSFCFG {
            MSFCFG(0)
        }
    }
    impl core::fmt::Debug for MSFCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MSFCFG")
                .field("IFR_ERASE_DIS0", &self.IFR_ERASE_DIS0())
                .field("IFR_ERASE_DIS1", &self.IFR_ERASE_DIS1())
                .field("IFR_ERASE_DIS2", &self.IFR_ERASE_DIS2())
                .field("IFR_ERASE_DIS3", &self.IFR_ERASE_DIS3())
                .field("MASS_ERASE_DIS", &self.MASS_ERASE_DIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MSFCFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MSFCFG {
                IFR_ERASE_DIS0: bool,
                IFR_ERASE_DIS1: bool,
                IFR_ERASE_DIS2: bool,
                IFR_ERASE_DIS3: bool,
                MASS_ERASE_DIS: bool,
            }
            let proxy = MSFCFG {
                IFR_ERASE_DIS0: self.IFR_ERASE_DIS0(),
                IFR_ERASE_DIS1: self.IFR_ERASE_DIS1(),
                IFR_ERASE_DIS2: self.IFR_ERASE_DIS2(),
                IFR_ERASE_DIS3: self.IFR_ERASE_DIS3(),
                MASS_ERASE_DIS: self.MASS_ERASE_DIS(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct NMISRC {
                IRQCPU0: u8,
                NMIENCPU0: bool,
            }
            let proxy = NMISRC {
                IRQCPU0: self.IRQCPU0(),
                NMIENCPU0: self.NMIENCPU0(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct NVM_CTRL {
                DIS_FLASH_SPEC: bool,
                DIS_DATA_SPEC: bool,
                FLASH_STALL_EN: bool,
                DIS_MBECC_ERR_INST: bool,
                DIS_MBECC_ERR_DATA: bool,
            }
            let proxy = NVM_CTRL {
                DIS_FLASH_SPEC: self.DIS_FLASH_SPEC(),
                DIS_DATA_SPEC: self.DIS_DATA_SPEC(),
                FLASH_STALL_EN: self.FLASH_STALL_EN(),
                DIS_MBECC_ERR_INST: self.DIS_MBECC_ERR_INST(),
                DIS_MBECC_ERR_DATA: self.DIS_MBECC_ERR_DATA(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL1_CLK_DIV Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PLL1CLKDIV(pub u32);
    impl PLL1CLKDIV {
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
    impl Default for PLL1CLKDIV {
        #[inline(always)]
        fn default() -> PLL1CLKDIV {
            PLL1CLKDIV(0)
        }
    }
    impl core::fmt::Debug for PLL1CLKDIV {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PLL1CLKDIV")
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PLL1CLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PLL1CLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = PLL1CLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Protect Level Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PROTLVL(pub u32);
    impl PROTLVL {
        #[inline(always)]
        pub const fn PRIV(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PRIV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn LOCKNSMPU(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCKNSMPU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
    impl Default for PROTLVL {
        #[inline(always)]
        fn default() -> PROTLVL {
            PROTLVL(0)
        }
    }
    impl core::fmt::Debug for PROTLVL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PROTLVL")
                .field("PRIV", &self.PRIV())
                .field("LOCKNSMPU", &self.LOCKNSMPU())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PROTLVL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PROTLVL {
                PRIV: bool,
                LOCKNSMPU: bool,
                LOCK: bool,
            }
            let proxy = PROTLVL {
                PRIV: self.PRIV(),
                LOCKNSMPU: self.LOCKNSMPU(),
                LOCK: self.LOCK(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct PWM0SUBCTL {
                CLK0_EN: bool,
                CLK1_EN: bool,
                CLK2_EN: bool,
                CLK3_EN: bool,
            }
            let proxy = PWM0SUBCTL {
                CLK0_EN: self.CLK0_EN(),
                CLK1_EN: self.CLK1_EN(),
                CLK2_EN: self.CLK2_EN(),
                CLK3_EN: self.CLK3_EN(),
            };
            defmt::write!(f, "{}", proxy)
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
            #[derive(defmt :: Format)]
            struct PWM1SUBCTL {
                CLK0_EN: bool,
                CLK1_EN: bool,
                CLK2_EN: bool,
                CLK3_EN: bool,
            }
            let proxy = PWM1SUBCTL {
                CLK0_EN: self.CLK0_EN(),
                CLK1_EN: self.CLK1_EN(),
                CLK2_EN: self.CLK2_EN(),
                CLK3_EN: self.CLK3_EN(),
            };
            defmt::write!(f, "{}", proxy)
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
        #[inline(always)]
        pub const fn RAMC_CG_OVERRIDE(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC_CG_OVERRIDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("RAMC_CG_OVERRIDE", &self.RAMC_CG_OVERRIDE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAM_CTRL {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RAM_CTRL {
                RAMA_ECC_ENABLE: bool,
                RAMA_CG_OVERRIDE: bool,
                RAMX_CG_OVERRIDE: bool,
                RAMB_CG_OVERRIDE: bool,
                RAMC_CG_OVERRIDE: bool,
            }
            let proxy = RAM_CTRL {
                RAMA_ECC_ENABLE: self.RAMA_ECC_ENABLE(),
                RAMA_CG_OVERRIDE: self.RAMA_CG_OVERRIDE(),
                RAMX_CG_OVERRIDE: self.RAMX_CG_OVERRIDE(),
                RAMB_CG_OVERRIDE: self.RAMB_CG_OVERRIDE(),
                RAMC_CG_OVERRIDE: self.RAMC_CG_OVERRIDE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Controls RAM Interleave Integration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RAM_INTERLEAVE(pub u32);
    impl RAM_INTERLEAVE {
        #[inline(always)]
        pub const fn INTERLEAVE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INTERLEAVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for RAM_INTERLEAVE {
        #[inline(always)]
        fn default() -> RAM_INTERLEAVE {
            RAM_INTERLEAVE(0)
        }
    }
    impl core::fmt::Debug for RAM_INTERLEAVE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RAM_INTERLEAVE")
                .field("INTERLEAVE", &self.INTERLEAVE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RAM_INTERLEAVE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct RAM_INTERLEAVE {
                INTERLEAVE: bool,
            }
            let proxy = RAM_INTERLEAVE {
                INTERLEAVE: self.INTERLEAVE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB Matrix Remap Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct REMAP(pub u32);
    impl REMAP {
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
        pub const fn SmartDMA_D(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SmartDMA_D(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn SmartDMA_I(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SmartDMA_I(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
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
        pub const fn PKC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_PKC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn USB0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_USB0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
                .field("SmartDMA_D", &self.SmartDMA_D())
                .field("SmartDMA_I", &self.SmartDMA_I())
                .field("DMA0", &self.DMA0())
                .field("PKC", &self.PKC())
                .field("USB0", &self.USB0())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for REMAP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct REMAP {
                CPU0_SBUS: u8,
                SmartDMA_D: u8,
                SmartDMA_I: u8,
                DMA0: u8,
                PKC: u8,
                USB0: u8,
                LOCK: bool,
            }
            let proxy = REMAP {
                CPU0_SBUS: self.CPU0_SBUS(),
                SmartDMA_D: self.SmartDMA_D(),
                SmartDMA_I: self.SmartDMA_I(),
                DMA0: self.DMA0(),
                PKC: self.PKC(),
                USB0: self.USB0(),
                LOCK: self.LOCK(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SLOWCLKDIV(pub u32);
    impl SLOWCLKDIV {
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
                .field("DIV", &self.DIV())
                .field("RESET", &self.RESET())
                .field("HALT", &self.HALT())
                .field("UNSTAB", &self.UNSTAB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SLOWCLKDIV {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SLOWCLKDIV {
                DIV: u8,
                RESET: bool,
                HALT: bool,
                UNSTAB: bool,
            }
            let proxy = SLOWCLKDIV {
                DIV: self.DIV(),
                RESET: self.RESET(),
                HALT: self.HALT(),
                UNSTAB: self.UNSTAB(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SmartDMA Interrupt Hijack"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SMARTDMAINT(pub u32);
    impl SMARTDMAINT {
        #[inline(always)]
        pub const fn INT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn INT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn INT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn INT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn INT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn INT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn INT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn INT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn INT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn INT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn INT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn INT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn INT16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn INT17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn INT18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn INT19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn INT20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn INT21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn INT22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn INT23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SMARTDMAINT {
        #[inline(always)]
        fn default() -> SMARTDMAINT {
            SMARTDMAINT(0)
        }
    }
    impl core::fmt::Debug for SMARTDMAINT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SMARTDMAINT")
                .field("INT0", &self.INT0())
                .field("INT1", &self.INT1())
                .field("INT2", &self.INT2())
                .field("INT3", &self.INT3())
                .field("INT4", &self.INT4())
                .field("INT5", &self.INT5())
                .field("INT6", &self.INT6())
                .field("INT7", &self.INT7())
                .field("INT8", &self.INT8())
                .field("INT9", &self.INT9())
                .field("INT10", &self.INT10())
                .field("INT11", &self.INT11())
                .field("INT12", &self.INT12())
                .field("INT13", &self.INT13())
                .field("INT14", &self.INT14())
                .field("INT15", &self.INT15())
                .field("INT16", &self.INT16())
                .field("INT17", &self.INT17())
                .field("INT18", &self.INT18())
                .field("INT19", &self.INT19())
                .field("INT20", &self.INT20())
                .field("INT21", &self.INT21())
                .field("INT22", &self.INT22())
                .field("INT23", &self.INT23())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SMARTDMAINT {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SMARTDMAINT {
                INT0: bool,
                INT1: bool,
                INT2: bool,
                INT3: bool,
                INT4: bool,
                INT5: bool,
                INT6: bool,
                INT7: bool,
                INT8: bool,
                INT9: bool,
                INT10: bool,
                INT11: bool,
                INT12: bool,
                INT13: bool,
                INT14: bool,
                INT15: bool,
                INT16: bool,
                INT17: bool,
                INT18: bool,
                INT19: bool,
                INT20: bool,
                INT21: bool,
                INT22: bool,
                INT23: bool,
            }
            let proxy = SMARTDMAINT {
                INT0: self.INT0(),
                INT1: self.INT1(),
                INT2: self.INT2(),
                INT3: self.INT3(),
                INT4: self.INT4(),
                INT5: self.INT5(),
                INT6: self.INT6(),
                INT7: self.INT7(),
                INT8: self.INT8(),
                INT9: self.INT9(),
                INT10: self.INT10(),
                INT11: self.INT11(),
                INT12: self.INT12(),
                INT13: self.INT13(),
                INT14: self.INT14(),
                INT15: self.INT15(),
                INT16: self.INT16(),
                INT17: self.INT17(),
                INT18: self.INT18(),
                INT19: self.INT19(),
                INT20: self.INT20(),
                INT21: self.INT21(),
                INT22: self.INT22(),
                INT23: self.INT23(),
            };
            defmt::write!(f, "{}", proxy)
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
        pub const fn RAMC_XEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("RAMC_XEN", &self.RAMC_XEN())
                .field("LOCK", &self.LOCK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_XEN {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_XEN {
                RAMX0_XEN: bool,
                RAMX1_XEN: bool,
                RAMA0_XEN: bool,
                RAMA1_XEN: bool,
                RAMB_XEN: bool,
                RAMC_XEN: bool,
                LOCK: bool,
            }
            let proxy = SRAM_XEN {
                RAMX0_XEN: self.RAMX0_XEN(),
                RAMX1_XEN: self.RAMX1_XEN(),
                RAMA0_XEN: self.RAMA0_XEN(),
                RAMA1_XEN: self.RAMA1_XEN(),
                RAMB_XEN: self.RAMB_XEN(),
                RAMC_XEN: self.RAMC_XEN(),
                LOCK: self.LOCK(),
            };
            defmt::write!(f, "{}", proxy)
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
        #[inline(always)]
        pub const fn RAMC_XEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RAMC_XEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("RAMC_XEN", &self.RAMC_XEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_XEN_DP {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_XEN_DP {
                RAMX0_XEN: bool,
                RAMX1_XEN: bool,
                RAMA0_XEN: bool,
                RAMA1_XEN: bool,
                RAMB_XEN: bool,
                RAMC_XEN: bool,
            }
            let proxy = SRAM_XEN_DP {
                RAMX0_XEN: self.RAMX0_XEN(),
                RAMX1_XEN: self.RAMX1_XEN(),
                RAMA0_XEN: self.RAMA0_XEN(),
                RAMA1_XEN: self.RAMA1_XEN(),
                RAMB_XEN: self.RAMB_XEN(),
                RAMC_XEN: self.RAMC_XEN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
