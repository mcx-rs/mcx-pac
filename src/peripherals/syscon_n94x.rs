#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SYSCON"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "AHB Matrix Priority Control"]
    pub AHBMATPRIO: crate::RWRegister<u32>,
    _reserved1: [u8; 0x24],
    #[doc = "Secure CPU0 System Tick Calibration"]
    pub CPU0STCKCAL: crate::RWRegister<u32>,
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
    pub CPU0NSTCKCAL: crate::RWRegister<u32>,
    #[doc = "System tick calibration for CPU1"]
    pub CPU1STCKCAL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "NMI Source Select"]
    pub NMISRC: crate::RWRegister<u32>,
    _reserved3: [u8; 0xb4],
    #[doc = "Peripheral Reset Control 0"]
    pub PRESETCTRL0: crate::RWRegister<u32>,
    #[doc = "Peripheral Reset Control 1"]
    pub PRESETCTRL1: crate::RWRegister<u32>,
    #[doc = "Peripheral Reset Control 2"]
    pub PRESETCTRL2: crate::RWRegister<u32>,
    #[doc = "Peripheral Reset Control 3"]
    pub PRESETCTRL3: crate::RWRegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "Peripheral Reset Control Set"]
    pub PRESETCTRLSET: [crate::RWRegister<u32>; 4usize],
    _reserved5: [u8; 0x10],
    #[doc = "Peripheral Reset Control Clear"]
    pub PRESETCTRLCLR: [crate::RWRegister<u32>; 4usize],
    _reserved6: [u8; 0xb0],
    #[doc = "AHB Clock Control 0"]
    pub AHBCLKCTRL0: crate::RWRegister<u32>,
    #[doc = "AHB Clock Control 1"]
    pub AHBCLKCTRL1: crate::RWRegister<u32>,
    #[doc = "AHB Clock Control 2"]
    pub AHBCLKCTRL2: crate::RWRegister<u32>,
    #[doc = "AHB Clock Control 3"]
    pub AHBCLKCTRL3: crate::RWRegister<u32>,
    _reserved7: [u8; 0x10],
    #[doc = "AHB Clock Control Set"]
    pub AHBCLKCTRLSET: [crate::RWRegister<u32>; 4usize],
    _reserved8: [u8; 0x10],
    #[doc = "AHB Clock Control Clear"]
    pub AHBCLKCTRLCLR: [crate::RWRegister<u32>; 4usize],
    _reserved9: [u8; 0x10],
    #[doc = "CPU0 System Tick Timer Source Select"]
    pub SYSTICKCLKSEL0: crate::RWRegister<u32>,
    #[doc = "CPU1 System Tick Timer Source Select"]
    pub SYSTICKCLKSEL1: crate::RWRegister<u32>,
    #[doc = "Trace Clock Source Select"]
    pub TRACECLKSEL: crate::RWRegister<u32>,
    #[doc = "CTIMER Clock Source Select"]
    pub CTIMERCLKSEL: [crate::RWRegister<u32>; 5usize],
    _reserved10: [u8; 0x8],
    #[doc = "CLKOUT Clock Source Select"]
    pub CLKOUTSEL: crate::RWRegister<u32>,
    _reserved11: [u8; 0x18],
    #[doc = "ADC0 Clock Source Select"]
    pub ADC0CLKSEL: crate::RWRegister<u32>,
    #[doc = "USB-FS Clock Source Select"]
    pub USB0CLKSEL: crate::RWRegister<u32>,
    _reserved12: [u8; 0x4],
    #[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider"]
    pub FCCLKSEL: [crate::RWRegister<u32>; 10usize],
    _reserved13: [u8; 0x18],
    #[doc = "SCTimer/PWM Clock Source Select"]
    pub SCTCLKSEL: crate::RWRegister<u32>,
    _reserved14: [u8; 0xc],
    #[doc = "CPU0 System Tick Timer Divider"]
    pub SYSTICKCLKDIV0: crate::RWRegister<u32>,
    #[doc = "CPU1 System Tick Timer Divider"]
    pub SYSTICKCLKDIV1: crate::RWRegister<u32>,
    #[doc = "TRACE Clock Divider"]
    pub TRACECLKDIV: crate::RWRegister<u32>,
    _reserved15: [u8; 0x44],
    #[doc = "TSI Function Clock Source Select"]
    pub TSICLKSEL: crate::RWRegister<u32>,
    _reserved16: [u8; 0xc],
    #[doc = "SINC FILTER Function Clock Source Select"]
    pub SINCFILTCLKSEL: crate::RWRegister<u32>,
    _reserved17: [u8; 0x14],
    #[doc = "SLOW_CLK Clock Divider"]
    pub SLOWCLKDIV: crate::RWRegister<u32>,
    #[doc = "TSI Function Clock Divider"]
    pub TSICLKDIV: crate::RWRegister<u32>,
    #[doc = "System Clock Divider"]
    pub AHBCLKDIV: crate::RWRegister<u32>,
    #[doc = "CLKOUT Clock Divider"]
    pub CLKOUTDIV: crate::RWRegister<u32>,
    #[doc = "FRO_HF_DIV Clock Divider"]
    pub FROHFDIV: crate::RWRegister<u32>,
    #[doc = "WDT0 Clock Divider"]
    pub WDT0CLKDIV: crate::RWRegister<u32>,
    _reserved18: [u8; 0x4],
    #[doc = "ADC0 Clock Divider"]
    pub ADC0CLKDIV: crate::RWRegister<u32>,
    #[doc = "USB-FS Clock Divider"]
    pub USB0CLKDIV: crate::RWRegister<u32>,
    _reserved19: [u8; 0x18],
    #[doc = "SCT/PWM Clock Divider"]
    pub SCTCLKDIV: crate::RWRegister<u32>,
    _reserved20: [u8; 0xc],
    #[doc = "PLL Clock Divider"]
    pub PLLCLKDIV: crate::RWRegister<u32>,
    _reserved21: [u8; 0x8],
    #[doc = "CTimer Clock Divider"]
    pub CTIMERCLKDIV: [crate::RWRegister<u32>; 5usize],
    #[doc = "PLL1 Clock 0 Divider"]
    pub PLL1CLK0DIV: crate::RWRegister<u32>,
    #[doc = "PLL1 Clock 1 Divider"]
    pub PLL1CLK1DIV: crate::RWRegister<u32>,
    _reserved22: [u8; 0x4],
    #[doc = "UTICK Clock Divider"]
    pub UTICKCLKDIV: crate::RWRegister<u32>,
    #[doc = "CLKOUT FRG Control"]
    pub CLKOUT_FRGCTRL: crate::RWRegister<u32>,
    _reserved23: [u8; 0x4],
    #[doc = "Clock Configuration Unlock"]
    pub CLKUNLOCK: crate::RWRegister<u32>,
    #[doc = "NVM Control"]
    pub NVM_CTRL: crate::RWRegister<u32>,
    #[doc = "ROM Wait State"]
    pub ROMCR: crate::RWRegister<u32>,
    _reserved24: [u8; 0xc],
    #[doc = "SmartDMA Interrupt Hijack"]
    pub SmartDMAINT: crate::RWRegister<u32>,
    _reserved25: [u8; 0x4c],
    #[doc = "ADC1 Clock Source Select"]
    pub ADC1CLKSEL: crate::RWRegister<u32>,
    #[doc = "ADC1 Clock Divider"]
    pub ADC1CLKDIV: crate::RWRegister<u32>,
    _reserved26: [u8; 0x4],
    #[doc = "Control PKC RAM Interleave Access"]
    pub RAM_INTERLEAVE: crate::RWRegister<u32>,
    _reserved27: [u8; 0x1c],
    #[doc = "DAC0 Functional Clock Selection"]
    pub DAC0CLKSEL: crate::RWRegister<u32>,
    #[doc = "DAC0 functional clock divider"]
    pub DAC0CLKDIV: crate::RWRegister<u32>,
    #[doc = "DAC1 Functional Clock Selection"]
    pub DAC1CLKSEL: crate::RWRegister<u32>,
    #[doc = "DAC1 functional clock divider"]
    pub DAC1CLKDIV: crate::RWRegister<u32>,
    #[doc = "DAC2 Functional Clock Selection"]
    pub DAC2CLKSEL: crate::RWRegister<u32>,
    #[doc = "DAC2 functional clock divider"]
    pub DAC2CLKDIV: crate::RWRegister<u32>,
    #[doc = "FlexSPI Clock Selection"]
    pub FlexSPICLKSEL: crate::RWRegister<u32>,
    #[doc = "FlexSPI Clock Divider"]
    pub FlexSPICLKDIV: crate::RWRegister<u32>,
    _reserved28: [u8; 0x7c],
    #[doc = "PLL Clock Divider Clock Selection"]
    pub PLLCLKDIVSEL: crate::RWRegister<u32>,
    #[doc = "I3C0 Functional Clock Selection"]
    pub I3C0FCLKSEL: crate::RWRegister<u32>,
    #[doc = "I3C0 FCLK_STC Clock Selection"]
    pub I3C0FCLKSTCSEL: crate::RWRegister<u32>,
    #[doc = "I3C0 FCLK_STC Clock Divider"]
    pub I3C0FCLKSTCDIV: crate::RWRegister<u32>,
    #[doc = "I3C0 FCLK Slow Clock Divider"]
    pub I3C0FCLKSDIV: crate::RWRegister<u32>,
    #[doc = "I3C0 Functional Clock FCLK Divider"]
    pub I3C0FCLKDIV: crate::RWRegister<u32>,
    #[doc = "I3C0 FCLK Slow Selection"]
    pub I3C0FCLKSSEL: crate::RWRegister<u32>,
    #[doc = "MICFIL Clock Selection"]
    pub MICFILFCLKSEL: crate::RWRegister<u32>,
    #[doc = "MICFIL Clock Division"]
    pub MICFILFCLKDIV: crate::RWRegister<u32>,
    _reserved29: [u8; 0x8],
    #[doc = "uSDHC Clock Selection"]
    pub uSDHCCLKSEL: crate::RWRegister<u32>,
    #[doc = "uSDHC Function Clock Divider"]
    pub uSDHCCLKDIV: crate::RWRegister<u32>,
    #[doc = "FLEXIO Clock Selection"]
    pub FLEXIOCLKSEL: crate::RWRegister<u32>,
    #[doc = "FLEXIO Function Clock Divider"]
    pub FLEXIOCLKDIV: crate::RWRegister<u32>,
    _reserved30: [u8; 0x38],
    #[doc = "FLEXCAN0 Clock Selection"]
    pub FLEXCAN0CLKSEL: crate::RWRegister<u32>,
    #[doc = "FLEXCAN0 Function Clock Divider"]
    pub FLEXCAN0CLKDIV: crate::RWRegister<u32>,
    #[doc = "FLEXCAN1 Clock Selection"]
    pub FLEXCAN1CLKSEL: crate::RWRegister<u32>,
    #[doc = "FLEXCAN1 Function Clock Divider"]
    pub FLEXCAN1CLKDIV: crate::RWRegister<u32>,
    #[doc = "Ethernet RMII Clock Selection"]
    pub ENETRMIICLKSEL: crate::RWRegister<u32>,
    #[doc = "Ethernet RMII Function Clock Divider"]
    pub ENETRMIICLKDIV: crate::RWRegister<u32>,
    #[doc = "Ethernet PTP REF Clock Selection"]
    pub ENETPTPREFCLKSEL: crate::RWRegister<u32>,
    #[doc = "Ethernet PTP REF Function Clock Divider"]
    pub ENETPTPREFCLKDIV: crate::RWRegister<u32>,
    #[doc = "Ethernet PHY Interface Select"]
    pub ENET_PHY_INTF_SEL: crate::RWRegister<u32>,
    #[doc = "Sideband Flow Control"]
    pub ENET_SBD_FLOW_CTRL: crate::RWRegister<u32>,
    _reserved31: [u8; 0xc],
    #[doc = "EWM0 Clock Selection"]
    pub EWM0CLKSEL: crate::RWRegister<u32>,
    #[doc = "WDT1 Clock Selection"]
    pub WDT1CLKSEL: crate::RWRegister<u32>,
    #[doc = "WDT1 Function Clock Divider"]
    pub WDT1CLKDIV: crate::RWRegister<u32>,
    #[doc = "OSTIMER Clock Selection"]
    pub OSTIMERCLKSEL: crate::RWRegister<u32>,
    _reserved32: [u8; 0xc],
    #[doc = "CMP0 Function Clock Selection"]
    pub CMP0FCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP0 Function Clock Divider"]
    pub CMP0FCLKDIV: crate::RWRegister<u32>,
    #[doc = "CMP0 Round Robin Clock Selection"]
    pub CMP0RRCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP0 Round Robin Clock Divider"]
    pub CMP0RRCLKDIV: crate::RWRegister<u32>,
    #[doc = "CMP1 Function Clock Selection"]
    pub CMP1FCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP1 Function Clock Divider"]
    pub CMP1FCLKDIV: crate::RWRegister<u32>,
    #[doc = "CMP1 Round Robin Clock Source Select"]
    pub CMP1RRCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP1 Round Robin Clock Division"]
    pub CMP1RRCLKDIV: crate::RWRegister<u32>,
    #[doc = "CMP2 Function Clock Source Select"]
    pub CMP2FCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP2 Function Clock Division"]
    pub CMP2FCLKDIV: crate::RWRegister<u32>,
    #[doc = "CMP2 Round Robin Clock Source Select"]
    pub CMP2RRCLKSEL: crate::RWRegister<u32>,
    #[doc = "CMP2 Round Robin Clock Division"]
    pub CMP2RRCLKDIV: crate::RWRegister<u32>,
    _reserved33: [u8; 0x1e0],
    #[doc = "CPU Control for Multiple Processors"]
    pub CPUCTRL: crate::RWRegister<u32>,
    #[doc = "Coprocessor Boot Address"]
    pub CPBOOT: crate::RWRegister<u32>,
    _reserved34: [u8; 0x4],
    #[doc = "CPU Status"]
    pub CPUSTAT: crate::RWRegister<u32>,
    _reserved35: [u8; 0x14],
    #[doc = "LPCAC Control"]
    pub LPCAC_CTRL: crate::RWRegister<u32>,
    _reserved36: [u8; 0x28],
    #[doc = "LP_FLEXCOMM Clock Divider"]
    pub FLEXCOMMCLKDIV: [crate::RWRegister<u32>; 10usize],
    #[doc = "UTICK Function Clock Source Select"]
    pub UTICKCLKSEL: crate::RWRegister<u32>,
    _reserved37: [u8; 0x4],
    #[doc = "SAI0 Function Clock Source Select"]
    pub SAI0CLKSEL: crate::RWRegister<u32>,
    #[doc = "SAI1 Function Clock Source Select"]
    pub SAI1CLKSEL: crate::RWRegister<u32>,
    #[doc = "SAI0 Function Clock Division"]
    pub SAI0CLKDIV: crate::RWRegister<u32>,
    #[doc = "SAI1 Function Clock Division"]
    pub SAI1CLKDIV: crate::RWRegister<u32>,
    #[doc = "EMVSIM0 Clock Source Select"]
    pub EMVSIM0CLKSEL: crate::RWRegister<u32>,
    #[doc = "EMVSIM1 Clock Source Select"]
    pub EMVSIM1CLKSEL: crate::RWRegister<u32>,
    #[doc = "EMVSIM0 Function Clock Division"]
    pub EMVSIM0CLKDIV: crate::RWRegister<u32>,
    #[doc = "EMVSIM1 Function Clock Division"]
    pub EMVSIM1CLKDIV: crate::RWRegister<u32>,
    _reserved38: [u8; 0xb0],
    #[doc = "Key Retain Control"]
    pub KEY_RETAIN_CTRL: crate::RWRegister<u32>,
    _reserved39: [u8; 0xc],
    #[doc = "FRO 48MHz Reference Clock Control"]
    pub REF_CLK_CTRL: crate::RWRegister<u32>,
    #[doc = "FRO 48MHz Reference Clock Control Set"]
    pub REF_CLK_CTRL_SET: crate::RWRegister<u32>,
    #[doc = "FRO 48MHz Reference Clock Control Clear"]
    pub REF_CLK_CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "GDET Control Register"]
    pub GDET_CTRL: [crate::RWRegister<u32>; 2usize],
    _reserved40: [u8; 0xc],
    #[doc = "Life Cycle State Register"]
    pub ELS_OTP_LC_STATE: crate::RWRegister<u32>,
    #[doc = "Life Cycle State Register (Duplicate)"]
    pub ELS_OTP_LC_STATE_DP: crate::RWRegister<u32>,
    #[doc = "ELS Temporal State"]
    pub ELS_TEMPORAL_STATE: crate::RWRegister<u32>,
    #[doc = "Key Derivation Function Mask"]
    pub ELS_KDF_MASK: crate::RWRegister<u32>,
    _reserved41: [u8; 0x40],
    #[doc = "ELS AS Configuration"]
    pub ELS_AS_CFG0: crate::RWRegister<u32>,
    #[doc = "ELS AS Configuration1"]
    pub ELS_AS_CFG1: crate::RWRegister<u32>,
    #[doc = "ELS AS Configuration2"]
    pub ELS_AS_CFG2: crate::RWRegister<u32>,
    #[doc = "ELS AS Configuration3"]
    pub ELS_AS_CFG3: crate::RWRegister<u32>,
    #[doc = "ELS AS State Register"]
    pub ELS_AS_ST0: crate::RWRegister<u32>,
    #[doc = "ELS AS State1"]
    pub ELS_AS_ST1: crate::RWRegister<u32>,
    #[doc = "Boot state captured during boot: Main ROM log"]
    pub ELS_AS_BOOT_LOG0: crate::RWRegister<u32>,
    #[doc = "Boot state captured during boot: N-boot library log"]
    pub ELS_AS_BOOT_LOG1: crate::RWRegister<u32>,
    #[doc = "Boot state captured during boot: Hardware status signals log"]
    pub ELS_AS_BOOT_LOG2: crate::RWRegister<u32>,
    #[doc = "Boot state captured during boot: Security log"]
    pub ELS_AS_BOOT_LOG3: crate::RWRegister<u32>,
    #[doc = "ELS AS Flag0"]
    pub ELS_AS_FLAG0: crate::RWRegister<u32>,
    #[doc = "ELS AS Flag1"]
    pub ELS_AS_FLAG1: crate::RWRegister<u32>,
    _reserved42: [u8; 0x18],
    #[doc = "Clock Control"]
    pub CLOCK_CTRL: crate::RWRegister<u32>,
    _reserved43: [u8; 0x114],
    #[doc = "I3C1 Functional Clock Selection"]
    pub I3C1FCLKSEL: crate::RWRegister<u32>,
    #[doc = "Selects the I3C1 Time Control clock"]
    pub I3C1FCLKSTCSEL: crate::RWRegister<u32>,
    #[doc = "I3C1 FCLK_STC Clock Divider"]
    pub I3C1FCLKSTCDIV: crate::RWRegister<u32>,
    #[doc = "I3C1 FCLK Slow clock Divider"]
    pub I3C1FCLKSDIV: crate::RWRegister<u32>,
    #[doc = "I3C1 Functional Clock FCLK Divider"]
    pub I3C1FCLKDIV: crate::RWRegister<u32>,
    #[doc = "I3C1 FCLK Slow Selection"]
    pub I3C1FCLKSSEL: crate::RWRegister<u32>,
    _reserved44: [u8; 0x8],
    #[doc = "ETB Counter Status Register"]
    pub ETB_STATUS: crate::RWRegister<u32>,
    #[doc = "ETB Counter Control Register"]
    pub ETB_COUNTER_CTRL: crate::RWRegister<u32>,
    #[doc = "ETB Counter Reload Register"]
    pub ETB_COUNTER_RELOAD: crate::RWRegister<u32>,
    #[doc = "ETB Counter Value Register"]
    pub ETB_COUNTER_VALUE: crate::RWRegister<u32>,
    #[doc = "Gray to Binary Converter Gray code_gray[31:0]"]
    pub GRAY_CODE_LSB: crate::RWRegister<u32>,
    #[doc = "Gray to Binary Converter Gray code_gray[41:32]"]
    pub GRAY_CODE_MSB: crate::RWRegister<u32>,
    #[doc = "Gray to Binary Converter Binary Code [31:0]"]
    pub BINARY_CODE_LSB: crate::RWRegister<u32>,
    #[doc = "Gray to Binary Converter Binary Code [41:32]"]
    pub BINARY_CODE_MSB: crate::RWRegister<u32>,
    _reserved45: [u8; 0x294],
    #[doc = "Control Automatic Clock Gating"]
    pub AUTOCLKGATEOVERRIDE: crate::RWRegister<u32>,
    _reserved46: [u8; 0x24],
    #[doc = "Control Automatic Clock Gating C"]
    pub AUTOCLKGATEOVERRIDEC: crate::RWRegister<u32>,
    _reserved47: [u8; 0x8],
    #[doc = "PWM0 Submodule Control"]
    pub PWM0SUBCTL: crate::RWRegister<u32>,
    #[doc = "PWM1 Submodule Control"]
    pub PWM1SUBCTL: crate::RWRegister<u32>,
    #[doc = "CTIMER Global Start Enable"]
    pub CTIMERGLOBALSTARTEN: crate::RWRegister<u32>,
    #[doc = "RAM ECC Enable Control"]
    pub ECC_ENABLE_CTRL: crate::RWRegister<u32>,
    _reserved48: [u8; 0x158],
    #[doc = "Control Write Access to Security"]
    pub DEBUG_LOCK_EN: crate::RWRegister<u32>,
    #[doc = "Cortex Debug Features Control"]
    pub DEBUG_FEATURES: crate::RWRegister<u32>,
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    pub DEBUG_FEATURES_DP: crate::RWRegister<u32>,
    _reserved49: [u8; 0x8],
    #[doc = "CPU0 Software Debug Access"]
    pub SWD_ACCESS_CPU0: crate::RWRegister<u32>,
    #[doc = "CPU1 Software Debug Access"]
    pub SWD_ACCESS_CPU1: crate::RWRegister<u32>,
    _reserved50: [u8; 0x4],
    #[doc = "Debug Authentication BEACON"]
    pub DEBUG_AUTH_BEACON: crate::RWRegister<u32>,
    #[doc = "DSP Software Debug Access"]
    pub SWD_ACCESS_DSP: crate::RWRegister<u32>,
    _reserved51: [u8; 0x28],
    #[doc = "JTAG Chip ID"]
    pub JTAG_ID: crate::RWRegister<u32>,
    #[doc = "Device Type"]
    pub DEVICE_TYPE: crate::RWRegister<u32>,
    #[doc = "Device ID"]
    pub DEVICE_ID0: crate::RWRegister<u32>,
    #[doc = "Chip Revision ID and Number"]
    pub DIEID: crate::RWRegister<u32>,
}
#[doc = "AHB Matrix Priority Control"]
pub mod AHBMATPRIO {
    #[doc = "CPU0 C-AHB bus master priority level"]
    pub mod PRI_CPU0_CBUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    pub mod PRI_CPU0_SBUS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CPU1 S-AHB/SmartDMA-D bus master priority level"]
    pub mod PRI_CPU1_SBUS_SmartDMA_D {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CPU1 C-AHB/SmartDMA-I bus master priority level"]
    pub mod PRI_CPU1_CBUS_SmartDMA_I {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "DMA0 controller bus master priority level"]
    pub mod DMA0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "DMA1 controller bus master priority level"]
    pub mod DMA1 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "PKC and ELS bus master priority level"]
    pub mod PRI_PKC_ELS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "NPU O bus and Powerquad bus master priority level"]
    pub mod PRI_NPU_PQ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CoolFlux I bus master priority level"]
    pub mod PRI_COOLFLUX_I {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CoolFlux X bus master priority level"]
    pub mod PRI_COOLFLUX_X {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "CoolFlux Y bus master priority level"]
    pub mod PRI_COOLFLUX_Y_ESPI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "NPU D bus master priority level"]
    pub mod PRI_NPU_D {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "USB-FS and ENET bus master priority level"]
    pub mod PRI_USB_FS_ENET {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "USB-HS bus master priority level"]
    pub mod PRI_USB_HS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
    #[doc = "USDHC bus master priority level"]
    pub mod PRI_USDHC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "level 0"]
            pub const LEVEL0: u32 = 0;
            #[doc = "level 1"]
            pub const LEVEL1: u32 = 1;
            #[doc = "level 2"]
            pub const LEVEL2: u32 = 2;
            #[doc = "level 3"]
            pub const LEVEL3: u32 = 3;
        }
    }
}
#[doc = "Secure CPU0 System Tick Calibration"]
pub mod CPU0STCKCAL {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Whether the TENMS value is exact."]
    pub mod SKEW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TENMS value is exact"]
            pub const EXACT: u32 = 0;
            #[doc = "TENMS value is not exact or not given"]
            pub const INEXACT: u32 = 1;
        }
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    pub mod NOREF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference clock is provided"]
            pub const YES_REF: u32 = 0;
            #[doc = "No reference clock is provided"]
            pub const NO_REF: u32 = 1;
        }
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration"]
pub mod CPU0NSTCKCAL {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    pub mod SKEW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TENMS value is exact"]
            pub const EXACT: u32 = 0;
            #[doc = "TENMS value is not exact or not given"]
            pub const INEXACT: u32 = 1;
        }
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    pub mod NOREF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference clock is provided"]
            pub const YES_REF: u32 = 0;
            #[doc = "No reference clock is provided"]
            pub const NO_REF: u32 = 1;
        }
    }
}
#[doc = "System tick calibration for CPU1"]
pub mod CPU1STCKCAL {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    pub mod TENMS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is not exact or not given."]
    pub mod SKEW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor. 0 = reference clock is provided; 1 = no reference clock is provided."]
    pub mod NOREF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NMI Source Select"]
pub mod NMISRC {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    pub mod IRQCPU0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU1, if enabled by NMIENCPU1."]
    pub mod IRQCPU1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    pub mod NMIENCPU1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    pub mod NMIENCPU0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Peripheral Reset Control 0"]
pub mod PRESETCTRL0 {
    #[doc = "Flash management unit reset control"]
    pub mod FMU_RST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "FlexSPI reset control"]
    pub mod FLEXSPI_RST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "INPUTMUX reset control"]
    pub mod MUX_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PORT0 controller reset control"]
    pub mod PORT0_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PORT1 reset control"]
    pub mod PORT1_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PORT2 reset control"]
    pub mod PORT2_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PORT3 reset control"]
    pub mod PORT3_RST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PORT4 reset control"]
    pub mod PORT4_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "GPIO0 reset control"]
    pub mod GPIO0_RST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "GPIO1 reset control"]
    pub mod GPIO1_RST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "GPIO2 reset control"]
    pub mod GPIO2_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "GPIO3 reset control"]
    pub mod GPIO3_RST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "GPIO4 reset control"]
    pub mod GPIO4_RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PINT reset control"]
    pub mod PINT_RST {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "DMA0 reset control"]
    pub mod DMA0_RST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CRC reset control"]
    pub mod CRC_RST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Inter-CPU communication Mailbox reset control"]
    pub mod MAILBOX_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
}
#[doc = "Peripheral Reset Control 1"]
pub mod PRESETCTRL1 {
    #[doc = "MRT reset control"]
    pub mod MRT_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "OS Event Timer reset control"]
    pub mod OSTIMER_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SCT reset control"]
    pub mod SCT_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "ADC0 reset control"]
    pub mod ADC0_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "ADC1 reset control"]
    pub mod ADC1_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "DAC0 reset control"]
    pub mod DAC0_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "RTC reset control"]
    pub mod RTC_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "EVSIM0 reset control"]
    pub mod EVSIM0_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "EVSIM1 reset control"]
    pub mod EVSIM1_RST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "UTICK reset control"]
    pub mod UTICK_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM0 reset control"]
    pub mod FC0_RST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM1 reset control"]
    pub mod FC1_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM2 reset control"]
    pub mod FC2_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM3 reset control"]
    pub mod FC3_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM4 reset control"]
    pub mod FC4_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM5 reset control"]
    pub mod FC5_RST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM6 reset control"]
    pub mod FC6_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM7 reset control"]
    pub mod FC7_RST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM8 reset control"]
    pub mod FC8_RST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "LP_FLEXCOMM9 reset control"]
    pub mod FC9_RST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "MICFIL reset control"]
    pub mod MICFIL_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CTIMER2 reset control"]
    pub mod TIMER2_RST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "USB FS DCD reset control"]
    pub mod USB0_FS_DCD_RST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "USB FS reset control"]
    pub mod USB0_FS_RST {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CTIMER0 reset control"]
    pub mod TIMER0_RST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CTIMER1 reset control"]
    pub mod TIMER1_RST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SmartDMA reset control"]
    pub mod SmartDMA_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
}
#[doc = "Peripheral Reset Control 2"]
pub mod PRESETCTRL2 {
    #[doc = "DMA1 reset control"]
    pub mod DMA1_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Ethernet reset control"]
    pub mod ENET_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "uSDHC reset control"]
    pub mod USDHC_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "FLEXIO reset control"]
    pub mod FLEXIO_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SAI0 reset control"]
    pub mod SAI0_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SAI1 reset control"]
    pub mod SAI1_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "TRO reset control"]
    pub mod TRO_RST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "FREQME reset control"]
    pub mod FREQME_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "TRNG reset control"]
    pub mod TRNG_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CAN0 reset control"]
    pub mod FLEXCAN0_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CAN1 reset control"]
    pub mod FLEXCAN1_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "USB HS reset control"]
    pub mod USB_HS_RST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "USB HS PHY reset control"]
    pub mod USB_HS_PHY_RST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PowerQuad reset control"]
    pub mod PQ_RST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PLU reset control"]
    pub mod PLU_RST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CTIMER3 reset control"]
    pub mod TIMER3_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CTIMER4 reset control"]
    pub mod TIMER4_RST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PUF reset control"]
    pub mod PUF_RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PKC reset control"]
    pub mod PKC_RST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SM3 reset control"]
    pub mod SM3_RST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
}
#[doc = "Peripheral Reset Control 3"]
pub mod PRESETCTRL3 {
    #[doc = "I3C0 reset control"]
    pub mod I3C0_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "I3C1 reset control"]
    pub mod I3C1_RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "SINC reset control"]
    pub mod SINC_RST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CoolFlux reset control"]
    pub mod COOLFLUX_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "ENC0 reset control"]
    pub mod ENC0_RST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "ENC1 reset control"]
    pub mod ENC1_RST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PWM0 reset control"]
    pub mod PWM0_RST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "PWM1 reset control"]
    pub mod PWM1_RST {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "AOI0 reset control"]
    pub mod AOI0_RST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "DAC1 reset control"]
    pub mod DAC1_RST {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "DAC2 reset control"]
    pub mod DAC2_RST {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "OPAMP0 reset control"]
    pub mod OPAMP0_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "OPAMP1 reset control"]
    pub mod OPAMP1_RST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "OPAMP2 reset control"]
    pub mod OPAMP2_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CMP2 reset control"]
    pub mod CMP2_RST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "VREF reset control"]
    pub mod VREF_RST {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "CoolFlux APB reset control"]
    pub mod COOLFLUX_APB_RST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "NPU reset control"]
    pub mod NPU_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "TSI reset control"]
    pub mod TSI_RST {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "EWM reset control"]
    pub mod EWM_RST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "EIM reset control"]
    pub mod EIM_RST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Semaphore reset control"]
    pub mod SEMA42_RST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Block is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
}
#[doc = "Peripheral Reset Control Set"]
pub mod PRESETCTRLSET {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Peripheral Reset Control Clear"]
pub mod PRESETCTRLCLR {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Clock Control 0"]
pub mod AHBCLKCTRL0 {
    #[doc = "Enables the clock for the ROM"]
    pub mod ROM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMB Controller"]
    pub mod RAMB_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMC Controller"]
    pub mod RAMC_CTRL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMD Controller"]
    pub mod RAMD_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAME Controller"]
    pub mod RAME_CTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMF Controller"]
    pub mod RAMF_CTRL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMG Controller"]
    pub mod RAMG_CTRL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the RAMH Controller"]
    pub mod RAMH_CTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the Flash Management Unit"]
    pub mod FMU {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the Flash Memory Controller"]
    pub mod FMC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for FlexSPI"]
    pub mod FLEXSPI {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for INPUTMUX"]
    pub mod MUX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PORT0 controller"]
    pub mod PORT0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PORT1"]
    pub mod PORT1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PORT2"]
    pub mod PORT2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PORT3"]
    pub mod PORT3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PORT4"]
    pub mod PORT4 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GPIO0"]
    pub mod GPIO0 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GPIO1"]
    pub mod GPIO1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GPIO2"]
    pub mod GPIO2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GPIO3"]
    pub mod GPIO3 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GPIO4"]
    pub mod GPIO4 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PINT"]
    pub mod PINT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for DMA0"]
    pub mod DMA0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CRC"]
    pub mod CRC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for WWDT0"]
    pub mod WWDT0 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for WWDT1"]
    pub mod WWDT1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    pub mod MAILBOX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "AHB Clock Control 1"]
pub mod AHBCLKCTRL1 {
    #[doc = "Enables the clock for MRT"]
    pub mod MRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the OS Event Timer"]
    pub mod OSTIMER {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SCT"]
    pub mod SCT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ADC0"]
    pub mod ADC0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ADC1"]
    pub mod ADC1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for DAC0"]
    pub mod DAC0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for RTC"]
    pub mod RTC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for EVSIM0"]
    pub mod EVSIM0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for EVSIM1"]
    pub mod EVSIM1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for UTICK"]
    pub mod UTICK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM0"]
    pub mod FC0 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM1"]
    pub mod FC1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM2"]
    pub mod FC2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM3"]
    pub mod FC3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM4"]
    pub mod FC4 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM5"]
    pub mod FC5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM6"]
    pub mod FC6 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM7"]
    pub mod FC7 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM8"]
    pub mod FC8 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for LP_FLEXCOMM9"]
    pub mod FC9 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for MICFIL"]
    pub mod MICFIL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CTIMER2"]
    pub mod TIMER2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for USB-FS DCD"]
    pub mod USB0_FS_DCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for USB-FS"]
    pub mod USB0_FS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CTIMER0"]
    pub mod TIMER0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CTIMER1"]
    pub mod TIMER1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PKC RAM"]
    pub mod PKC_RAM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SmartDMA"]
    pub mod SmartDMA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "AHB Clock Control 2"]
pub mod AHBCLKCTRL2 {
    #[doc = "Enables the clock for DMA1"]
    pub mod DMA1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for Ethernet"]
    pub mod ENET {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for uSDHC"]
    pub mod uSDHC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for Flexio"]
    pub mod FLEXIO {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SAI0"]
    pub mod SAI0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SAI1"]
    pub mod SAI1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for TRO"]
    pub mod TRO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for the Frequency meter"]
    pub mod FREQME {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for TRNG"]
    pub mod TRNG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for FLEXCAN0"]
    pub mod FLEXCAN0 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for FLEXCAN1"]
    pub mod FLEXCAN1 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for USB HS"]
    pub mod USB_HS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for USB HS PHY"]
    pub mod USB_HS_PHY {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ELS"]
    pub mod ELS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for Powerquad"]
    pub mod PQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PLU_LUT"]
    pub mod PLU_LUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CTIMER3"]
    pub mod TIMER3 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CTIMER4"]
    pub mod TIMER4 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PUF"]
    pub mod PUF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PKC"]
    pub mod PKC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SCG"]
    pub mod SCG {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for GDET0 and GDET1"]
    pub mod GDET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SM3"]
    pub mod SM3 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "AHB Clock Control 3"]
pub mod AHBCLKCTRL3 {
    #[doc = "Enables the clock for I3C0"]
    pub mod I3C0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for I3C1"]
    pub mod I3C1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for SINC"]
    pub mod SINC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CoolFlux"]
    pub mod COOLFLUX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ENC0"]
    pub mod ENC0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ENC1"]
    pub mod ENC1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PWM0"]
    pub mod PWM0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for PWM1"]
    pub mod PWM1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for EVTG"]
    pub mod EVTG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for DAC1"]
    pub mod DAC1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for DAC2"]
    pub mod DAC2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for OPAMP0"]
    pub mod OPAMP0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for OPAMP1"]
    pub mod OPAMP1 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for OPAMP2"]
    pub mod OPAMP2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CMP2"]
    pub mod CMP2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for VREF"]
    pub mod VREF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for CoolFlux APB"]
    pub mod COOLFLUX_APB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock (CoolFlux needs to be properly programmed before the clock enabled.)"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for NPU"]
    pub mod NPU {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for TSI"]
    pub mod TSI {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for EWM"]
    pub mod EWM {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for EIM"]
    pub mod EIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for ERM"]
    pub mod ERM {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for INTM"]
    pub mod INTM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the clock for Semaphore"]
    pub mod SEMA42 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables clock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables clock"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "AHB Clock Control Set"]
pub mod AHBCLKCTRLSET {
    #[doc = "Data array value"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Clock Control Clear"]
pub mod AHBCLKCTRLCLR {
    #[doc = "Data array value"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPU0 System Tick Timer Source Select"]
pub mod SYSTICKCLKSEL0 {
    #[doc = "Selects the System Tick Timer for CPU0 source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SYSTICKCLKDIV0 output"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "Clk 1 MHz clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "LP Oscillator clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "CPU1 System Tick Timer Source Select"]
pub mod SYSTICKCLKSEL1 {
    #[doc = "Selects the System Tick Timer for CPU1 source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SYSTICKCLKDIV1 output"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "Clk 1 MHz clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "LP Oscillator clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "Trace Clock Source Select"]
pub mod TRACECLKSEL {
    #[doc = "Selects the trace clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRACECLKDIV output"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "Clk 1 MHz clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "LP Oscillator clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "CTIMER Clock Source Select"]
pub mod CTIMERCLKSEL {
    #[doc = "Selects the CTIMER clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO_1M clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO 12MHz clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "SAI0 MCLK IN clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "LP Oscillator clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
            #[doc = "SAI1 MCLK IN clock"]
            pub const ENUM_0X8: u32 = 8;
            #[doc = "SAI0 TX_BCLK clock"]
            pub const ENUM_0X9: u32 = 9;
            #[doc = "SAI0 RX_BCLK clock"]
            pub const ENUM_0XA: u32 = 10;
            #[doc = "SAI1 TX_BCLK clock"]
            pub const ENUM_0XB: u32 = 11;
            #[doc = "SAI1 RX_BCLK clock"]
            pub const ENUM_0XC: u32 = 12;
            #[doc = "No clock"]
            pub const ENUM_0XD: u32 = 13;
            #[doc = "No clock"]
            pub const ENUM_0XE: u32 = 14;
            #[doc = "No clock"]
            pub const ENUM_0XF: u32 = 15;
        }
    }
}
#[doc = "CLKOUT Clock Source Select"]
pub mod CLKOUTSEL {
    #[doc = "Selects the CLKOUT clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Main clock (main_clk)"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock (pll0_clk)"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "CLKIN clock (clk_in)"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF clock (fro_hf)"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO 12 MHz clock (fro_12m)"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock (pll1_clk)"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "LP Oscillator clock (lp_osc)"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "USB PLL clock (usb_pll_clk)"]
            pub const ENUM_0X7: u32 = 7;
            #[doc = "No clock"]
            pub const ENUM_0X8: u32 = 8;
            #[doc = "No clock"]
            pub const ENUM_0X9: u32 = 9;
            #[doc = "No clock"]
            pub const ENUM_0XA: u32 = 10;
            #[doc = "No clock"]
            pub const ENUM_0XB: u32 = 11;
            #[doc = "No clock"]
            pub const ENUM_0XC: u32 = 12;
            #[doc = "No clock"]
            pub const ENUM_0XD: u32 = 13;
            #[doc = "No clock"]
            pub const ENUM_0XE: u32 = 14;
            #[doc = "No clock"]
            pub const ENUM_0XF: u32 = 15;
        }
    }
}
#[doc = "ADC0 Clock Source Select"]
pub mod ADC0CLKSEL {
    #[doc = "Selects the ADC0 clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO 12 MHz clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "Clk_in"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "USB-FS Clock Source Select"]
pub mod USB0CLKSEL {
    #[doc = "Selects the USB-FS clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "Clk 48 MHz clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "Clk_in"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider"]
pub mod FCCLKSEL {
    #[doc = "Selects the LP_FLEXCOMM clock source for Fractional Rate Divider."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL divided clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "FRO 12 MHz clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "fro_hf_div clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "CLK_1MHz clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "LP Oscillator clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "SCTimer/PWM Clock Source Select"]
pub mod SCTCLKSEL {
    #[doc = "Selects the SCTimer/PWM clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "SAI0 MCLK_IN clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
            #[doc = "SAI1 MCLK_IN clock"]
            pub const ENUM_0X8: u32 = 8;
            #[doc = "No clock"]
            pub const ENUM_0X9: u32 = 9;
            #[doc = "No clock"]
            pub const ENUM_0XA: u32 = 10;
            #[doc = "No clock"]
            pub const ENUM_0XB: u32 = 11;
            #[doc = "No clock"]
            pub const ENUM_0XC: u32 = 12;
            #[doc = "No clock"]
            pub const ENUM_0XD: u32 = 13;
            #[doc = "No clock"]
            pub const ENUM_0XE: u32 = 14;
            #[doc = "No clock"]
            pub const ENUM_0XF: u32 = 15;
        }
    }
}
#[doc = "CPU0 System Tick Timer Divider"]
pub mod SYSTICKCLKDIV0 {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPU1 System Tick Timer Divider"]
pub mod SYSTICKCLKDIV1 {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRACE Clock Divider"]
pub mod TRACECLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI Function Clock Source Select"]
pub mod TSICLKSEL {
    #[doc = "Selects the TSI function clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "No clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "clk_in"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO_12Mhz clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "SINC FILTER Function Clock Source Select"]
pub mod SINCFILTCLKSEL {
    #[doc = "Selects the SINC FILTER function clock source."]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "clk_in"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO_12Mhz clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "SLOW_CLK Clock Divider"]
pub mod SLOWCLKDIV {
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI Function Clock Divider"]
pub mod TSICLKDIV {
    #[doc = "Clock divider value:"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Clock Divider"]
pub mod AHBCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLKOUT Clock Divider"]
pub mod CLKOUTDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FRO_HF_DIV Clock Divider"]
pub mod FROHFDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is running, this bit is set to 0 when the register is written."]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "WDT0 Clock Divider"]
pub mod WDT0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADC0 Clock Divider"]
pub mod ADC0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB-FS Clock Divider"]
pub mod USB0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SCT/PWM Clock Divider"]
pub mod SCTCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL Clock Divider"]
pub mod PLLCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CTimer Clock Divider"]
pub mod CTIMERCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const ENABLE: u32 = 0;
            #[doc = "Divider clock has stopped"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stable divider clock"]
            pub const ENABLE: u32 = 0;
            #[doc = "Unstable clock frequency"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "PLL1 Clock 0 Divider"]
pub mod PLL1CLK0DIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL1 Clock 1 Divider"]
pub mod PLL1CLK1DIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTICK Clock Divider"]
pub mod UTICKCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLKOUT FRG Control"]
pub mod CLKOUT_FRGCTRL {
    #[doc = "Divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Numerator value"]
    pub mod MULT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration Unlock"]
pub mod CLKUNLOCK {
    #[doc = "Controls clock configuration registers access (for example, xxxDIV, xxxSEL)"]
    pub mod UNLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Updates are allowed to all clock configuration registers"]
            pub const ENABLE: u32 = 0;
            #[doc = "Freezes all clock configuration registers update"]
            pub const FREEZE: u32 = 1;
        }
    }
}
#[doc = "NVM Control"]
pub mod NVM_CTRL {
    #[doc = "Flash speculation control"]
    pub mod DIS_FLASH_SPEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables flash speculation"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables flash speculation"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Flash data speculation control"]
    pub mod DIS_DATA_SPEC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables data speculation"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables data speculation"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Flash cache control"]
    pub mod DIS_FLASH_CACHE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables flash cache"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables flash cache"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Flash instruction cache control"]
    pub mod DIS_FLASH_INST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables flash instruction cache when DIS_FLASH_CACHE=0"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables flash instruction cache"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Flash data cache control"]
    pub mod DIS_FLASH_DATA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables flash data cache when DIS_FLASH_CACHE=0"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables flash data cache"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Clear flash cache control"]
    pub mod CLR_FLASH_CACHE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clear flash cache"]
            pub const ENABLE: u32 = 0;
            #[doc = "Clears flash cache"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "FLASH stall on busy control"]
    pub mod FLASH_STALL_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No stall on FLASH busy"]
            pub const ENABLE: u32 = 0;
            #[doc = "Stall on FLASH busy"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Bus error on instruction multi-bit ECC error control If DIS_MBECC_ERR_DATA and/or DIS_MBECC_INST are set, then speculation will not be enabled, even if this bit is cleared"]
    pub mod DIS_MBECC_ERR_INST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables bus error on multi-bit ECC error for instruction"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables bus error on multi-bit ECC error for instruction"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Bus error on data multi-bit ECC error control If DIS_MBECC_ERR_DATA and/or DIS_MBECC_INST are set, then speculation will not be enabled, even if this bit is cleared"]
    pub mod DIS_MBECC_ERR_DATA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables bus error on multi-bit ECC error for data"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables bus error on multi-bit ECC error for data"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "ROM Wait State"]
pub mod ROMCR {
    #[doc = "ROM waiting Arm core and other masters for one cycle"]
    pub mod ROM_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SmartDMA Interrupt Hijack"]
pub mod SmartDMAINT {
    #[doc = "SmartDMA hijack NVIC IRQ1"]
    pub mod INT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ17"]
    pub mod INT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ18"]
    pub mod INT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    pub mod INT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ30"]
    pub mod INT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    pub mod INT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    pub mod INT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    pub mod INT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    pub mod INT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ35"]
    pub mod INT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ36"]
    pub mod INT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ37"]
    pub mod INT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ38"]
    pub mod INT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    pub mod INT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    pub mod INT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    pub mod INT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ42"]
    pub mod INT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ45"]
    pub mod INT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ47"]
    pub mod INT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ50"]
    pub mod INT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ51"]
    pub mod INT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ66"]
    pub mod INT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ67"]
    pub mod INT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SmartDMA hijack NVIC IRQ77"]
    pub mod INT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "ADC1 Clock Source Select"]
pub mod ADC1CLKSEL {
    #[doc = "Selects the ADC1 clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO 12 MHz clock"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "Clk_in clock"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "ADC1 Clock Divider"]
pub mod ADC1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control PKC RAM Interleave Access"]
pub mod RAM_INTERLEAVE {
    #[doc = "Controls PKC RAM access for PKC RAM 0 and PKC RAM 1"]
    pub mod INTERLEAVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is consecutive."]
            pub const NORMAL: u32 = 0;
            #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is interleaved. This setting is need for PKC L0 memory access."]
            pub const INTERLEAVE: u32 = 1;
        }
    }
}
#[doc = "DAC0 Functional Clock Selection"]
pub mod DAC0CLKSEL {
    #[doc = "Selects the DAC clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "Clk_in"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO_12M"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "DAC0 functional clock divider"]
pub mod DAC0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC1 Functional Clock Selection"]
pub mod DAC1CLKSEL {
    #[doc = "Selects the DAC clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "Clk_in"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO_12M"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "DAC1 functional clock divider"]
pub mod DAC1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC2 Functional Clock Selection"]
pub mod DAC2CLKSEL {
    #[doc = "Selects the DAC clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM_0X0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM_0X1: u32 = 1;
            #[doc = "Clk_in"]
            pub const ENUM_0X2: u32 = 2;
            #[doc = "FRO_HF"]
            pub const ENUM_0X3: u32 = 3;
            #[doc = "FRO_12M"]
            pub const ENUM_0X4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM_0X5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM_0X6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM_0X7: u32 = 7;
        }
    }
}
#[doc = "DAC2 functional clock divider"]
pub mod DAC2CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FlexSPI Clock Selection"]
pub mod FlexSPICLKSEL {
    #[doc = "Selects the FlexSPI clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "pll1_clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
            #[doc = "No clock"]
            pub const ENUM8: u32 = 8;
            #[doc = "No clock"]
            pub const ENUM9: u32 = 9;
            #[doc = "No clock"]
            pub const ENUM10: u32 = 10;
            #[doc = "No clock"]
            pub const ENUM11: u32 = 11;
            #[doc = "No clock"]
            pub const ENUM12: u32 = 12;
            #[doc = "No clock"]
            pub const ENUM13: u32 = 13;
            #[doc = "No clock"]
            pub const ENUM14: u32 = 14;
            #[doc = "No clock"]
            pub const ENUM15: u32 = 15;
        }
    }
}
#[doc = "FlexSPI Clock Divider"]
pub mod FlexSPICLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL Clock Divider Clock Selection"]
pub mod PLLCLKDIVSEL {
    #[doc = "Selects the PLL Clock Divider source clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PLL0 clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "pll1_clk0"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "I3C0 Functional Clock Selection"]
pub mod I3C0FCLKSEL {
    #[doc = "Selects the I3C0 clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "I3C0 FCLK_STC Clock Selection"]
pub mod I3C0FCLKSTCSEL {
    #[doc = "Selects the I3C0 Time Control clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C0 functional clock I3C0FCLK"]
            pub const ENUM0: u32 = 0;
            #[doc = "FRO_1M clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "I3C0 FCLK_STC Clock Divider"]
pub mod I3C0FCLKSTCDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C0 FCLK Slow Clock Divider"]
pub mod I3C0FCLKSDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C0 Functional Clock FCLK Divider"]
pub mod I3C0FCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C0 FCLK Slow Selection"]
pub mod I3C0FCLKSSEL {
    #[doc = "Selects the I3C FCLK Slow clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO_1M clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "No clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "MICFIL Clock Selection"]
pub mod MICFILFCLKSEL {
    #[doc = "Selects the MICFIL clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO_12M clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "SAI0_MCLK clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
            #[doc = "SAI1_MCLK clock"]
            pub const ENUM8: u32 = 8;
            #[doc = "No clock"]
            pub const ENUM9: u32 = 9;
            #[doc = "No clock"]
            pub const ENUM10: u32 = 10;
            #[doc = "No clock"]
            pub const ENUM11: u32 = 11;
            #[doc = "No clock"]
            pub const ENUM12: u32 = 12;
            #[doc = "No clock"]
            pub const ENUM13: u32 = 13;
            #[doc = "No clock"]
            pub const ENUM14: u32 = 14;
            #[doc = "No clock"]
            pub const ENUM15: u32 = 15;
        }
    }
}
#[doc = "MICFIL Clock Division"]
pub mod MICFILFCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "uSDHC Clock Selection"]
pub mod uSDHCCLKSEL {
    #[doc = "Selects the uSDHC clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "FRO_12M clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "pll1_clk1 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "uSDHC Function Clock Divider"]
pub mod uSDHCCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FLEXIO Clock Selection"]
pub mod FLEXIOCLKSEL {
    #[doc = "Selects the FLEXIO clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "FRO_12M clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "FLEXIO Function Clock Divider"]
pub mod FLEXIOCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FLEXCAN0 Clock Selection"]
pub mod FLEXCAN0CLKSEL {
    #[doc = "Selects the FLEXCAN0 clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "FLEXCAN0 Function Clock Divider"]
pub mod FLEXCAN0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FLEXCAN1 Clock Selection"]
pub mod FLEXCAN1CLKSEL {
    #[doc = "Selects the FLEXCAN1 clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "FLEXCAN1 Function Clock Divider"]
pub mod FLEXCAN1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ethernet RMII Clock Selection"]
pub mod ENETRMIICLKSEL {
    #[doc = "Selects the Ethernet RMII clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "Ethernet RMII Function Clock Divider"]
pub mod ENETRMIICLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ethernet PTP REF Clock Selection"]
pub mod ENETPTPREFCLKSEL {
    #[doc = "Selects the Ethernet PTP REF clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "enet0_tx_clk clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "pll1_clk1 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "Ethernet PTP REF Function Clock Divider"]
pub mod ENETPTPREFCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ethernet PHY Interface Select"]
pub mod ENET_PHY_INTF_SEL {
    #[doc = "Selects the PHY interface"]
    pub mod PHY_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects MII PHY Interface"]
            pub const MII: u32 = 0;
            #[doc = "Selects RMII PHY Interface"]
            pub const RMII: u32 = 1;
        }
    }
}
#[doc = "Sideband Flow Control"]
pub mod ENET_SBD_FLOW_CTRL {
    #[doc = "Sideband Flow Control for channel0"]
    pub mod SEL_ch0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger flow control"]
            pub const DISABLE: u32 = 0;
            #[doc = "Trigger flow control"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Sideband Flow Control for channel1"]
    pub mod SEL_ch1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger flow control"]
            pub const DISABLE: u32 = 0;
            #[doc = "Trigger flow control"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "EWM0 Clock Selection"]
pub mod EWM0CLKSEL {
    #[doc = "Selects the EWM0 clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clk_16k[2]"]
            pub const ENUM0: u32 = 0;
            #[doc = "xtal32k[2]"]
            pub const ENUM1: u32 = 1;
        }
    }
}
#[doc = "WDT1 Clock Selection"]
pub mod WDT1CLKSEL {
    #[doc = "Selects the WDT1 clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO16K clock 2"]
            pub const ENUM0: u32 = 0;
            #[doc = "fro_hf_div clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "clk_1m clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "clk_1m clock"]
            pub const ENUM3: u32 = 3;
        }
    }
}
#[doc = "WDT1 Function Clock Divider"]
pub mod WDT1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OSTIMER Clock Selection"]
pub mod OSTIMERCLKSEL {
    #[doc = "Selects the OS Event Timer clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clk_16k[2]"]
            pub const ENUM0: u32 = 0;
            #[doc = "xtal32k[2]"]
            pub const ENUM1: u32 = 1;
            #[doc = "clk_1m clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
        }
    }
}
#[doc = "CMP0 Function Clock Selection"]
pub mod CMP0FCLKSEL {
    #[doc = "Selects the CMP0 function clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP0 Function Clock Divider"]
pub mod CMP0FCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP0 Round Robin Clock Selection"]
pub mod CMP0RRCLKSEL {
    #[doc = "Selects the CMP0 round robin clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP0 Round Robin Clock Divider"]
pub mod CMP0RRCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP1 Function Clock Selection"]
pub mod CMP1FCLKSEL {
    #[doc = "Selects the CMP1 function clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP1 Function Clock Divider"]
pub mod CMP1FCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP1 Round Robin Clock Source Select"]
pub mod CMP1RRCLKSEL {
    #[doc = "Selects the CMP1 round robin clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP1 Round Robin Clock Division"]
pub mod CMP1RRCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP2 Function Clock Source Select"]
pub mod CMP2FCLKSEL {
    #[doc = "Selects the CMP2 function clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP2 Function Clock Division"]
pub mod CMP2FCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP2 Round Robin Clock Source Select"]
pub mod CMP2RRCLKSEL {
    #[doc = "Selects the CMP2 round robin clock"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "FRO_HF clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_12M clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "CLKIN clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock0"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "CMP2 Round Robin Clock Division"]
pub mod CMP2RRCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPU Control for Multiple Processors"]
pub mod CPUCTRL {
    #[doc = "Enables the CPU1 clock"]
    pub mod CPU1CLKEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CPU1 clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "The CPU1 clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "CPU1 reset"]
    pub mod CPU1RSTEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CPU1 is not reset."]
            pub const RELEASED: u32 = 0;
            #[doc = "The CPU1 is reset."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Write Protect"]
    pub mod PROT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "For write operation to have an effect."]
            pub const PROT: u32 = 49348;
        }
        pub mod RW {}
    }
}
#[doc = "Coprocessor Boot Address"]
pub mod CPBOOT {
    #[doc = "Coprocessor Boot VTOR Address [31:7] for CPU1"]
    pub mod CPBOOT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CPU Status"]
pub mod CPUSTAT {
    #[doc = "CPU0 sleeping state"]
    pub mod CPU0SLEEPING {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "CPU is not sleeping"]
            pub const AWAKE: u32 = 0;
            #[doc = "CPU is sleeping"]
            pub const SLEEPING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU1 sleeping state"]
    pub mod CPU1SLEEPING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "CPU is not sleeping"]
            pub const AWAKE: u32 = 0;
            #[doc = "CPU is sleeping"]
            pub const SLEEPING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 lockup state"]
    pub mod CPU0LOCKUP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "CPU is not in lockup"]
            pub const AWAKE: u32 = 0;
            #[doc = "CPU is in lockup"]
            pub const SLEEPING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU1 lockup state"]
    pub mod CPU1LOCKUP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "CPU is not in lockup"]
            pub const AWAKE: u32 = 0;
            #[doc = "CPU is in lockup"]
            pub const SLEEPING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LPCAC Control"]
pub mod LPCAC_CTRL {
    #[doc = "Disables/enables the cache function."]
    pub mod DIS_LPCAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Clears the cache function."]
    pub mod CLR_LPCAC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unclears the cache"]
            pub const ENABLE: u32 = 0;
            #[doc = "Clears the cache"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Forces no allocation."]
    pub mod FRC_NO_ALLOC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces allocation"]
            pub const ENABLE: u32 = 0;
            #[doc = "Forces no allocation"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Enables parity miss."]
    pub mod PARITY_MISS_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables parity, miss on parity error"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    pub mod DIS_LPCAC_WTBF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables write through buffer"]
            pub const DISABLE: u32 = 0;
            #[doc = "Disables write through buffer"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    pub mod LIM_LPCAC_WTBF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write buffer enabled when transaction is bufferable."]
            pub const DISABLE: u32 = 0;
            #[doc = "Write buffer enabled when transaction is cacheable and bufferable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable parity error report."]
    pub mod PARITY_FAULT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables parity error report"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables parity error report"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    pub mod LPCAC_XOM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "LP_FLEXCOMM Clock Divider"]
pub mod FLEXCOMMCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UTICK Function Clock Source Select"]
pub mod UTICKCLKSEL {
    #[doc = "Selects the clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clk_in"]
            pub const ENUM0: u32 = 0;
            #[doc = "xtal32k[2]"]
            pub const ENUM1: u32 = 1;
            #[doc = "clk_1m clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
        }
    }
}
#[doc = "SAI0 Function Clock Source Select"]
pub mod SAI0CLKSEL {
    #[doc = "Selects the clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "PLL1_CLK0 clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "SAI1 Function Clock Source Select"]
pub mod SAI1CLKSEL {
    #[doc = "Selects the clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "PLL1_CLK0 clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "SAI0 Function Clock Division"]
pub mod SAI0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SAI1 Function Clock Division"]
pub mod SAI1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMVSIM0 Clock Source Select"]
pub mod EMVSIM0CLKSEL {
    #[doc = "Selects the EMVSIM0 function clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "FRO_12M clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock0"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "EMVSIM1 Clock Source Select"]
pub mod EMVSIM1CLKSEL {
    #[doc = "Selects the EMVSIM1 function clock source"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "FRO_12M clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock0"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "EMVSIM0 Function Clock Division"]
pub mod EMVSIM0CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EMVSIM1 Function Clock Division"]
pub mod EMVSIM1CLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key Retain Control"]
pub mod KEY_RETAIN_CTRL {
    #[doc = "Indicates if the PUF key has been retained in the VBAT domain and has not been reset or otherwise invalidated by software."]
    pub mod KEY_RETAIN_VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PUF key is not retained in VBAT domain."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "PUF key is retained in VBAT domain."]
            pub const VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the successful completion of the key_save or key_load routine. Once set, to clear the key_retain_done flag, both key_save and key_load should be cleared by software."]
    pub mod KEY_RETAIN_DONE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Key save / load sequence has not completed."]
            pub const NOT_DONE: u32 = 0;
            #[doc = "Key save / load sequence has completed."]
            pub const DONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    pub mod KEY_SAVE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key save sequence is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Key save sequence is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time"]
    pub mod KEY_LOAD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key load sequence is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Key load sequence is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "FRO 48MHz Reference Clock Control"]
pub mod REF_CLK_CTRL {
    #[doc = "GDET reference clock enable bit"]
    pub mod GDET_REFCLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ELS TRNG reference clock enable bit"]
    pub mod TRNG_REFCLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "FRO 48MHz Reference Clock Control Set"]
pub mod REF_CLK_CTRL_SET {
    #[doc = "GDET reference clock enable set bit"]
    pub mod GDET_REFCLK_EN_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Set to 1"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "ELS TRNG reference clock enable set bit"]
    pub mod TRNG_REFCLK_EN_SET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Set to 1"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "FRO 48MHz Reference Clock Control Clear"]
pub mod REF_CLK_CTRL_CLR {
    #[doc = "GDET reference clock enable clear bit"]
    pub mod GDET_REFCLK_EN_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Set to 0"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "ELS TRNG reference clock enable clear bit"]
    pub mod TRNG_REFCLK_EN_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Set to 0"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "GDET Control Register"]
pub mod GDET_CTRL {
    #[doc = "Controls the GDET clean event counter"]
    pub mod GDET_EVTCNT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event counter not cleared"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clears event counter"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Clears GDET error status"]
    pub mod GDET_ERR_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error status not cleared"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clears error status"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "GDET isolation control"]
    pub mod GDET_ISO_SW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Isolation is disabled"]
            pub const DISABLE0: u32 = 0;
            #[doc = "Isolation is disabled"]
            pub const DISABLE1: u32 = 1;
            #[doc = "Isolation is enabled. When both GDET0_CTRL/GDET1_CTRL GDET_ISO_SW are \"10\", isolation_on is asserted."]
            pub const ENABLE: u32 = 2;
            #[doc = "Isolation is disabled"]
            pub const DISABLE3: u32 = 3;
        }
    }
    #[doc = "Event count value"]
    pub mod EVENT_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive glitch detected"]
    pub mod POS_SYNC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Positive glitch not detected"]
            pub const DISABLE: u32 = 0;
            #[doc = "Positive glitch detected"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Negative glitch detected"]
    pub mod NEG_SYNC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Negative glitch not detected"]
            pub const DISABLE: u32 = 0;
            #[doc = "Negative glitch detected"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Event counter cleared"]
    pub mod EVENT_CLR_FLAG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Event counter not cleared"]
            pub const DISABLE: u32 = 0;
            #[doc = "Event counter cleared"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Life Cycle State Register"]
pub mod ELS_OTP_LC_STATE {
    #[doc = "OTP life cycle state"]
    pub mod OTP_LC_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Life Cycle State Register (Duplicate)"]
pub mod ELS_OTP_LC_STATE_DP {
    #[doc = "OTP life cycle state"]
    pub mod OTP_LC_STATE_DP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS Temporal State"]
pub mod ELS_TEMPORAL_STATE {
    #[doc = "Temporal state"]
    pub mod TEMPORAL_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key Derivation Function Mask"]
pub mod ELS_KDF_MASK {
    #[doc = "Key derivation function mask"]
    pub mod KDF_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Configuration"]
pub mod ELS_AS_CFG0 {
    #[doc = "LC state configuration bit"]
    pub mod CFG_LC_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_CORE_RESET_ENABLED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD IRQ are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_CORE_IRQ_ENABLED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When WatchDog Timer 0 is activated, this bit indicates state 1"]
    pub mod CFG_WDT0_ENABLED {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When Code WatchDog Timer 0 is activated, this bit indicates state 1"]
    pub mod CFG_CWDT0_ENABLED {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When either GDET is enabled, this bit indicates state 1."]
    pub mod CFG_ELS_GDET_ENABLED {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC analog glitch detect reset is enabled, this bit indicates state 1"]
    pub mod CFG_ANA_GDET_RESET_ENABLED {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC analog glitch detect IRQ is enabled, this bit indicates state 1"]
    pub mod CFG_ANA_GDET_IRQ_ENABLED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When tamper detector is enabled in TDET, this bit indicates state 1."]
    pub mod CFG_TAMPER_DET_ENABLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_VSYS_RESET_ENABLED {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_VDDIO_RESET_ENABLED {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD irq are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_VSYS_IRQ_ENABLED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD irq are enabled, this bit indicates state 1."]
    pub mod CFG_LVD_VDDIO_IRQ_ENABLED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When WatchDog Timer 1 is activated, this bit indicates state 1."]
    pub mod CFG_WDT1_ENABLED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When Code WatchDog Timer 1 is activated, this bit indicates state 1."]
    pub mod CFG_CWDT1_ENABLED {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When temperature tamper detector is enabled in VBAT, this bit indicates state 1."]
    pub mod CFG_TEMPTAMPER_DET_ENABLED {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When voltage tamper detector is enabled in VBAT, this bit indicates state 1."]
    pub mod CFG_VOLTAMPER_DET_ENABLED {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When light tamper detector is enabled in VBAT, this bit indicates state 1."]
    pub mod CFG_LHTTAMPER_DET_ENABLED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When clk tamper detector is enabled in VBAT, this bit indicates state 1."]
    pub mod CFG_CLKTAMPER_DET_ENABLED {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When QK PUF \"qk_disable_enroll\" input is driven 1, this bit indicates state 1"]
    pub mod CFG_QK_DISABLE_ENROLL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When QK PUF \"qk_disable_wrap\" input is driven 1, this bit indicates state 1"]
    pub mod CFG_QK_DISABLE_WRAP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Configuration1"]
pub mod ELS_AS_CFG1 {
    #[doc = "When CFG_SEC_ENA_SEC_CHK indicates state 0 or when DISABLE_STRICT_MODE bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    pub mod CFG_SEC_DIS_STRICT_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the DISABLE_VIOLATION_ABORT bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_DIS_VIOL_ABORT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the ENABLE_NS_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_ENA_NS_PRIV_CHK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the ENABLE_S_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_ENA_S_PRIV_CHK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the ENABLE_SECURE_CHECKING bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_ENA_SEC_CHK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the IDAU_ALL_NS bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1"]
    pub mod CFG_SEC_IDAU_ALLNS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the LOCK_NS_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_LOCK_NS_MPU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the LOCK_NS_VTOR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_LOCK_NS_VTOR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the LOCK_S_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_LOCK_S_MPU {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the LOCK_S_VTAIRCR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_LOCK_S_VTAIRCR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When the LOCK_SAU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1"]
    pub mod CFG_SEC_LOCK_SAU {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "metal version"]
    pub mod METAL_VERSION {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROM patch version"]
    pub mod ROM_PATCH_VERSION {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_CORE_RESET_ENABLED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD IRQ are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_CORE_IRQ_ENABLED {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_VSYS_RESET_ENABLED {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD reset are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_VDDIO_RESET_ENABLED {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD irq are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_VSYS_IRQ_ENABLED {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD irq are enabled, this bit indicates state 1."]
    pub mod CFG_HVD_VDDIO_IRQ_ENABLED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Configuration2"]
pub mod ELS_AS_CFG2 {
    #[doc = "ELS configuration command enable bit"]
    pub mod CFG_ELS_CMD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Configuration3"]
pub mod ELS_AS_CFG3 {
    #[doc = "Device type identification data"]
    pub mod DEVICE_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS State Register"]
pub mod ELS_AS_ST0 {
    #[doc = "TEMPORAL_STATE[3:0] in the ELS_TEMPORAL_STATE register reflects this register"]
    pub mod ST_TEMPORAL_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU0 (CM33) \"deben\" input is state 1, this bit indicates state 1"]
    pub mod ST_CPU0_DBGEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU0 (CM33) \"niden\" input is state 1, this bit indicates state 1"]
    pub mod ST_CPU0_NIDEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU0 (CM33) \"spiden\" input is state 1, this bit indicates state 1"]
    pub mod ST_CPU0_SPIDEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU0 (CM33) \"spniden\" input is state 1, this bit indicates state 1"]
    pub mod ST_CPU0_SPNIDEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU1 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    pub mod ST_CPU1_DBGEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When CPU1 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    pub mod ST_CPU1_NIDEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When DAP to AP0 for CPU0 (CM33) debug access is allowed, this bit indicates state 1"]
    pub mod ST_DAP_ENABLE_CPU0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When DAP to AP1 for CPU1 (CM33) debug access is allowed, this bit indicates state 1."]
    pub mod ST_DAP_ENABLE_CPU1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When DAP to AP3 for DSP (CoolFlux) debug access is allowed, this bit indicates state 1"]
    pub mod ST_DAP_ENABLE_DSP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When JTAG TAP access is allowed, this bit indicates state 1."]
    pub mod ST_ALLOW_TEST_ACCESS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When XO32K oscillation fail flag is state 1, this bit indicates state 1"]
    pub mod ST_XO32K_FAILED {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When XO40M oscillation fail flag is state 1, this bit indicates state 1"]
    pub mod ST_XO40M_FAILED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When IFR load fail flag is state 1, this bit indicates state 1"]
    pub mod ST_IFR_LOAD_FAILED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GLITCH_DETECT_FLAG is state of 4-bit Glitch Ripple Counter output."]
    pub mod ST_GLITCH_DETECT_FLAG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS State1"]
pub mod ELS_AS_ST1 {
    #[doc = "These register bits indicate the state of \"qk_puf_score[3:0]\" outputs from QK PUF block"]
    pub mod ST_QK_PUF_SCORE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This register bit indicates the state of \"qk_zeroized\" output from QK PUF block"]
    pub mod ST_QK_ZEROIZED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When MAIN_CLK is running from external clock source either XO32M, XO32K or GPIO CLKIN, this bit indicates state 1"]
    pub mod ST_MAIN_CLK_IS_EXT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOUT[1:0] setting on DCDC0 register in SPC block will reflect to this register. Default is 1.0V"]
    pub mod ST_DCDC_VOUT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC drive strength setting. Default is normal drive."]
    pub mod ST_DCDC_DS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ISP pin status during boot. By default ISP pin is pulled up. If want to enter ISP mode during boot, ISP pin should be pull down when out of reset."]
    pub mod ST_BOOT_MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BOOT_RETRY_CNT[3:0] in the ELS_BOOT_RETRY_CNT register reflects this register"]
    pub mod ST_BOOT_RETRY_CNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VOUT[1:0] setting on LDO Core register in SPC block will reflect to this register. Default is 1.0V"]
    pub mod ST_LDO_CORE_VOUT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LDO_CORE drive strength setting. Default is normal drive."]
    pub mod ST_LDO_CORE_DS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Boot state captured during boot: Main ROM log"]
pub mod ELS_AS_BOOT_LOG0 {
    #[doc = "Boot image source used during this boot."]
    pub mod BOOT_IMAGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Internal flash image 0"]
            pub const ENUM0: u32 = 0;
            #[doc = "Internal flash image 1"]
            pub const ENUM1: u32 = 1;
            #[doc = "FlexSPI flash image 0"]
            pub const ENUM2: u32 = 2;
            #[doc = "FlexSPI flash image 1"]
            pub const ENUM3: u32 = 3;
            #[doc = "Recovery SPI flash image"]
            pub const ENUM4: u32 = 4;
            #[doc = "Serial boot image (write-memory and execute ISP command used)"]
            pub const ENUM5: u32 = 5;
            #[doc = "Receive SB3 containing SB_JUMP command is used."]
            pub const ENUM6: u32 = 6;
            #[doc = "Customer SBL/recovery image (Bank1 IFR0)."]
            pub const ENUM7: u32 = 7;
            #[doc = "NXP MAD recovery image (Bank1 IFR0)."]
            pub const ENUM8: u32 = 8;
            #[doc = "NXP ROM extension (NMPA - Bank0 IFR0)."]
            pub const ENUM9: u32 = 9;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC verify is used instead of ECDSA verify on this boot."]
    pub mod CMAC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ECDSA P-384 verification is done on this boot."]
    pub mod ECDSA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Off-chip Prince is enabled during boot."]
    pub mod OFF_CHIP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "On-chip Prince is enabled during boot."]
    pub mod ON_CHIP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CDI based device keys are derived for CSR harvesting on this boot."]
    pub mod CDI_CSR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CDI per DICE specification is computed on this boot."]
    pub mod CDI_DICE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TrustZone preset data is loaded during this boot."]
    pub mod TRUSTZONE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Debug authentication done in this session prior to boot."]
    pub mod DEBUG_AUTH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITRC zeroize event is handled in this session of boot."]
    pub mod ITRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Digital glitch detector is enabled during boot."]
    pub mod DIG_GDET {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog glitch detector is enabled during boot."]
    pub mod ANA_GDET {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot from deep-power down state."]
    pub mod DEEP_PD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last low-power mode value. ROM copies SPC_LP_MODE field from SPC->SC[7:4]."]
    pub mod LOW_POWER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ISP pin state at boot time. ROM copies CMC->MR0[0]."]
    pub mod ISP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Boot state captured during boot: N-boot library log"]
pub mod ELS_AS_BOOT_LOG1 {
    #[doc = "RoTK index used for this boot."]
    pub mod RoTK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIPS self-test is executed and PASS during this boot. When a bit is set, means self-test is executed and it FAILS. When a bit is clear, means corresponding self-test is executed and PASS or it is not executed."]
    pub mod FIPS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SB3 type (valid after nboot_sb3_load_manifest())."]
    pub mod SB3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "customer fw load/update file."]
            pub const CUSTOMER: u32 = 0;
            #[doc = "NXP Provisioning FW."]
            pub const NXP: u32 = 1;
            #[doc = "ELS signed OEM Provisioning FW."]
            pub const OEM: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Boot state captured during boot: Hardware status signals log"]
pub mod ELS_AS_BOOT_LOG2 {
    #[doc = "CMC->SRS[5:0]"]
    pub mod CMC_SRS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBAT->STATUSA[1:0] | ~VBAT->STATUSB[1:0]"]
    pub mod VBAT_STATUS0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMC->SRS[16:8]"]
    pub mod CMC_SRS1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBAT->STATUSA[11:6] | ~VBAT->STATUSB[11:6]"]
    pub mod VBAT_STATUS1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMC->SRS[31:24]"]
    pub mod CMC_SRS2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Boot state captured during boot: Security log"]
pub mod ELS_AS_BOOT_LOG3 {
    #[doc = "CFPA->ERR_AUTH_FAIL_COUNT[7:0]"]
    pub mod ERR_AUTH_FAIL_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CFPA->ERR_ITRC_COUNT[7:0]"]
    pub mod ERR_ITRC_COUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CFPA->ERR_CFG_FAIL_COUNT[7:0]"]
    pub mod ERR_CFG_FAIL_COUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CFPA->ERR_COUNT3[7:0]"]
    pub mod ERR_COUNT3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Flag0"]
pub mod ELS_AS_FLAG0 {
    #[doc = "This flag bit is set as 1 when DAP enables AP0 for CPU0 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    pub mod FLAG_AP_ENABLE_CPU0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP1 for CPU1 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    pub mod FLAG_AP_ENABLE_CPU1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP3 for DSP (CoolFlux) debug access. The register is cleared 0 by PMC reset event."]
    pub mod FLAG_AP_ENABLE_DSP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTPC can output attack_detect signal when it detects attack when load shadow registers. The output will be cleared by reset. ELS_AS_FLAG is reset by PoR, so the status can be recorded."]
    pub mod EFUSE_ATTACK_DETECT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag register is set 1 when VDD_CORE LVD event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_LVD_CORE_OCCURED {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    pub mod FLAG_WDT0_RESET_OCCURED {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    pub mod FLAG_CWDT0_RESET_OCCURED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_WDT0_IRQ_OCCURED {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_CWDT0_IRQ_OCCURED {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when QK_ERROR is flagged from QK PUF block. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_QK_ERROR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when GDET error is flagged. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_ELS_GLITCH_DETECTED {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when ANALOG GDET error is flagged in SYSCON block. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_ANA_GLITCH_DETECTED {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when tamper event is flagged from TDET. This register is cleared 0 by AO domain POR or by PMC reset event, if tamper detection event is cleared by software."]
    pub mod FLAG_TAMPER_EVENT_DETECTED {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when FLASH controller indicates ECC error. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_FLASH_ECC_INVALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when security violation is indicated from FLASH sub-system or AHB bus matrix."]
    pub mod FLAG_SEC_VIOL_IRQ_OCURRED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure code transactions. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_CPU0_NS_C_ACC_OCCURED {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure data transactions. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_CPU0_NS_D_ACC_OCCURED {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag register is set 1 when VDD_SYS LVD event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_LVD_VSYS_OCCURED {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag register is set 1 when VDD LVD event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_LVD_VDDIO_OCCURED {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    pub mod FLAG_WDT1_RESET_OCCURED {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    pub mod FLAG_CWDT1_RESET_OCCURED {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_WDT1_IRQ_OCCURED {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_CWDT1_IRQ_OCCURED {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when temperature temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_TEMPTAMPER_DET_IRQ_OCCURED {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when voltage temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_VOLTAMPER_DET_IRQ_OCCURED {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when light temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_LHTTAMPER_DET_IRQ_OCCURED {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when clock temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    pub mod FLAG_CLKTAMPER_DET_IRQ_OCCURED {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ELS AS Flag1"]
pub mod ELS_AS_FLAG1 {
    #[doc = "This flag bit is set as 1 when HVD from VDD_CORE power domain is triggered."]
    pub mod FLAG_HVD_CORE_OCCURED {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_SYS power domain is triggered"]
    pub mod FLAG_HVD_VSYS_OCCURED {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD power domain is triggered"]
    pub mod FLAG_HVD_VDDIO_OCCURED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not Triggered"]
            pub const DISABLE: u32 = 0;
            #[doc = "Triggered"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Control"]
pub mod CLOCK_CTRL {
    #[doc = "Enables the clk_in clock for the Frequency Measurement, USB HS and LPTIMER0/1 modules."]
    pub mod CLKIN_ENA_FM_USBH_LPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the FRO_1MHz clock for RTC module and for UTICK"]
    pub mod FRO1MHZ_ENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the FRO_12MHz clock for the Flash, LPTIMER0/1, and Frequency Measurement modules"]
    pub mod FRO12MHZ_ENA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables FRO HF clock for the Frequency Measure module"]
    pub mod FRO_HF_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables clk_in clock for MICFIL, EMVSIM0/1, CAN0/1, I3C0/1, SAI0/1, SINC Filter (SINC), TSI, USBFS, SCT, uSDHC, clkout."]
    pub mod CLKIN_ENA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables FRO_1MHz clock for clock muxing in clock gen"]
    pub mod FRO1MHZ_CLK_ENA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    pub mod PLU_DEGLITCH_CLK_ENA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "I3C1 Functional Clock Selection"]
pub mod I3C1FCLKSEL {
    #[doc = "I3C1 clock select"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "PLL0 clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "CLKIN clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "FRO_HF clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "PLL1_clk0 clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "USB PLL clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "Selects the I3C1 Time Control clock"]
pub mod I3C1FCLKSTCSEL {
    #[doc = "I3C1 FCLK_STC clock select"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C1 functional clock I3C1FCLK"]
            pub const ENUM0: u32 = 0;
            #[doc = "FRO_1M clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "I3C1 FCLK_STC Clock Divider"]
pub mod I3C1FCLKSTCDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C1 FCLK Slow clock Divider"]
pub mod I3C1FCLKSDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C1 Functional Clock FCLK Divider"]
pub mod I3C1FCLKDIV {
    #[doc = "Clock divider value"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resets the divider counter"]
    pub mod RESET {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider is not reset"]
            pub const RELEASED: u32 = 0;
            #[doc = "Divider is reset"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Halts the divider counter"]
    pub mod HALT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divider clock is running"]
            pub const RUN: u32 = 0;
            #[doc = "Divider clock is stopped"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Divider status flag"]
    pub mod UNSTAB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Divider clock is stable"]
            pub const STABLE: u32 = 0;
            #[doc = "Clock frequency is not stable"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "I3C1 FCLK Slow Selection"]
pub mod I3C1FCLKSSEL {
    #[doc = "I3C1 FCLK Slow Clock Select"]
    pub mod SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO_1M clock"]
            pub const ENUM0: u32 = 0;
            #[doc = "No clock"]
            pub const ENUM1: u32 = 1;
            #[doc = "No clock"]
            pub const ENUM2: u32 = 2;
            #[doc = "No clock"]
            pub const ENUM3: u32 = 3;
            #[doc = "No clock"]
            pub const ENUM4: u32 = 4;
            #[doc = "No clock"]
            pub const ENUM5: u32 = 5;
            #[doc = "No clock"]
            pub const ENUM6: u32 = 6;
            #[doc = "No clock"]
            pub const ENUM7: u32 = 7;
        }
    }
}
#[doc = "ETB Counter Status Register"]
pub mod ETB_STATUS {
    #[doc = "ETB Interrupt"]
    pub mod IRQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ETB interrupt is not asserted"]
            pub const DISABLE: u32 = 0;
            #[doc = "ETB interrupt is asserted when ETB count expires. Write 1 to clear it."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ETB NMI"]
    pub mod NMI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ETB NMI is not asserted"]
            pub const DISABLE: u32 = 0;
            #[doc = "ETB NMI is asserted. Write 1 to clear it."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Debug halt request"]
    pub mod DBG_HALT_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The debug halt request signal is not asserted"]
            pub const DISABLE: u32 = 0;
            #[doc = "The debug halt request signal is asserted when the ETB count expires"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETB Counter Control Register"]
pub mod ETB_COUNTER_CTRL {
    #[doc = "Enables the ETB counter"]
    pub mod CNTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ETB counter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ETB counter is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Response Type"]
    pub mod RSPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No response when the ETB count expires"]
            pub const NO_RESP: u32 = 0;
            #[doc = "Generates a normal interrupt when the ETB count expires"]
            pub const INTERRUPT: u32 = 1;
            #[doc = "Generates an NMI interrupt when the ETB count expires"]
            pub const NMI: u32 = 2;
            #[doc = "Generates a debug halt when the ETB count expires via CPU0 CTICHIN[2]"]
            pub const DEBUG_HALT: u32 = 3;
        }
    }
    #[doc = "Reload request"]
    pub mod RLRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "ETB Counter Reload Register"]
pub mod ETB_COUNTER_RELOAD {
    #[doc = "Byte count reload value"]
    pub mod RELOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ETB Counter Value Register"]
pub mod ETB_COUNTER_VALUE {
    #[doc = "Byte count counter value"]
    pub mod COUNTER_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gray to Binary Converter Gray code_gray[31:0]"]
pub mod GRAY_CODE_LSB {
    #[doc = "Gray code [31:0]"]
    pub mod code_gray_31_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gray to Binary Converter Gray code_gray[41:32]"]
pub mod GRAY_CODE_MSB {
    #[doc = "Gray code [41:32]"]
    pub mod code_gray_41_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gray to Binary Converter Binary Code [31:0]"]
pub mod BINARY_CODE_LSB {
    #[doc = "Binary code [31:0]"]
    pub mod code_bin_31_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gray to Binary Converter Binary Code [41:32]"]
pub mod BINARY_CODE_MSB {
    #[doc = "Binary code [41:32]"]
    pub mod code_bin_41_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Automatic Clock Gating"]
pub mod AUTOCLKGATEOVERRIDE {
    #[doc = "Controls automatic clock gating for the RAMB Controller"]
    pub mod RAMB_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMC Controller"]
    pub mod RAMC_CTRL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller"]
    pub mod RAMD_CTRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    pub mod RAME_CTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMF Controller"]
    pub mod RAMF_CTRL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    pub mod RAMG_CTRL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller"]
    pub mod RAMH_CTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Control Automatic Clock Gating C"]
pub mod AUTOCLKGATEOVERRIDEC {
    #[doc = "Controls automatic clock gating of the RAMX controller"]
    pub mod RAMX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls automatic clock gating of the RAMA controller"]
    pub mod RAMA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic clock gating is not overridden"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic clock gating is overridden (Automatic clock gating is disabled)."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "PWM0 Submodule Control"]
pub mod PWM0SUBCTL {
    #[doc = "Enables PWM0 SUB Clock0"]
    pub mod CLK0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM0 SUB Clock1"]
    pub mod CLK1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM0 SUB Clock2"]
    pub mod CLK2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM0 SUB Clock3"]
    pub mod CLK3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM0 submodule 0 DMA compare value done mask"]
    pub mod DMAVALM0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM0 submodule 1 DMA compare value done mask"]
    pub mod DMAVALM1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM0 submodule 2 DMA compare value done mask"]
    pub mod DMAVALM2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM0 submodule 3 DMA compare value done mask"]
    pub mod DMAVALM3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM1 Submodule Control"]
pub mod PWM1SUBCTL {
    #[doc = "Enables PWM1 SUB Clock0"]
    pub mod CLK0_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM1 SUB Clock1"]
    pub mod CLK1_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM1 SUB Clock2"]
    pub mod CLK2_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables PWM1 SUB Clock3"]
    pub mod CLK3_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM1 submodule 0 DMA compare value done mask"]
    pub mod DMAVALM0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM1 submodule 1 DMA compare value done mask"]
    pub mod DMAVALM1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM1 submodule 2 DMA compare value done mask"]
    pub mod DMAVALM2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM1 submodule 3 DMA compare value done mask"]
    pub mod DMAVALM3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CTIMER Global Start Enable"]
pub mod CTIMERGLOBALSTARTEN {
    #[doc = "Enables the CTIMER0 function clock"]
    pub mod CTIMER0_CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the CTIMER1 function clock"]
    pub mod CTIMER1_CLK_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the CTIMER2 function clock"]
    pub mod CTIMER2_CLK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the CTIMER3 function clock"]
    pub mod CTIMER3_CLK_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables the CTIMER4 function clock"]
    pub mod CTIMER4_CLK_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "RAM ECC Enable Control"]
pub mod ECC_ENABLE_CTRL {
    #[doc = "RAMA ECC enable"]
    pub mod RAMA_ECC_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "RAMB and RAMX ECC enable"]
    pub mod RAMB_RAMX_ECC_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "RAMD and RAMC ECC enable"]
    pub mod RAMD_RAMC_ECC_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "RAMF and RAME ECC enable"]
    pub mod RAMF_RAME_ECC_ENABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Control Write Access to Security"]
pub mod DEBUG_LOCK_EN {
    #[doc = "Controls write access to the security registers"]
    pub mod LOCK_ALL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any other value than b1010: disables write access to all registers"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables write access to all registers"]
            pub const ENABLE: u32 = 10;
        }
    }
}
#[doc = "Cortex Debug Features Control"]
pub mod DEBUG_FEATURES {
    #[doc = "CPU0 invasive debug control"]
    pub mod CPU0_DBGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 non-invasive debug control"]
    pub mod CPU0_NIDEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    pub mod CPU0_SPIDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    pub mod CPU0_SPNIDEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU1 invasive debug control"]
    pub mod CPU1_DBGEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU1 non-invasive debug control"]
    pub mod CPU1_NIDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "DSP invasive debug control"]
    pub mod DSP_DBGDEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)"]
pub mod DEBUG_FEATURES_DP {
    #[doc = "CPU0 invasive debug control"]
    pub mod CPU0_DBGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 non-invasive debug control"]
    pub mod CPU0_NIDEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    pub mod CPU0_SPIDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    pub mod CPU0_SPNIDEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU1 invasive debug control"]
    pub mod CPU1_DBGEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "CPU1 non-invasive debug control"]
    pub mod CPU1_NIDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "DSP invasive debug control"]
    pub mod DSP_DBGEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables debug"]
            pub const DISABLE: u32 = 1;
            #[doc = "Enables debug"]
            pub const ENABLE: u32 = 2;
        }
    }
}
#[doc = "CPU0 Software Debug Access"]
pub mod SWD_ACCESS_CPU0 {
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    pub mod SEC_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
            pub const DISABLE: u32 = 0;
            #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
            pub const ENABLE: u32 = 305419896;
        }
    }
}
#[doc = "CPU1 Software Debug Access"]
pub mod SWD_ACCESS_CPU1 {
    #[doc = "Security code to allow CPU1 DAP: 0x12345678"]
    pub mod SEC_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "CPU1 DAP is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Security code to allow CPU1 DAP"]
            pub const ENABLE: u32 = 305419896;
        }
        pub mod RW {}
    }
}
#[doc = "Debug Authentication BEACON"]
pub mod DEBUG_AUTH_BEACON {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    pub mod BEACON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DSP Software Debug Access"]
pub mod SWD_ACCESS_DSP {
    #[doc = "DSP SWD-AP: 0x12345678"]
    pub mod SEC_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DSP DAP is not allowed. Reading back register is read as 0x5."]
            pub const DISABLE: u32 = 0;
            #[doc = "Value to write to enable DSP SWD access. Reading back register is read as 0xA."]
            pub const ENABLE: u32 = 305419896;
        }
    }
}
#[doc = "JTAG Chip ID"]
pub mod JTAG_ID {
    #[doc = "Indicates the device ID"]
    pub mod JTAG_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Device Type"]
pub mod DEVICE_TYPE {
    #[doc = "Indicates DEVICE TYPE."]
    pub mod DEVICE_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Device ID"]
pub mod DEVICE_ID0 {
    #[doc = "ROM revision."]
    pub mod ROM_REV_MINOR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Chip Revision ID and Number"]
pub mod DIEID {
    #[doc = "Chip minor revision"]
    pub mod MINOR_REVISION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip major revision"]
    pub mod MAJOR_REVISION {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip number"]
    pub mod MCO_NUM_IN_DIE_ID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
