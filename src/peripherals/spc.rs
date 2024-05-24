#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SPC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "Status Control"]
    pub SC: crate::RWRegister<u32>,
    #[doc = "SPC Regulator Control"]
    pub CNTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Low-Power Request Configuration"]
    pub LPREQ_CFG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x10],
    #[doc = "SPC Power Domain Mode Status"]
    pub PD_STATUS: [crate::RWRegister<u32>; 2usize],
    _reserved3: [u8; 0x8],
    #[doc = "SRAM Control"]
    pub SRAMCTL: crate::RWRegister<u32>,
    _reserved4: [u8; 0xbc],
    #[doc = "Active Power Mode Configuration"]
    pub ACTIVE_CFG: crate::RWRegister<u32>,
    #[doc = "Active Power Mode Configuration 1"]
    pub ACTIVE_CFG1: crate::RWRegister<u32>,
    #[doc = "Low-Power Mode Configuration"]
    pub LP_CFG: crate::RWRegister<u32>,
    #[doc = "Low Power Mode Configuration 1"]
    pub LP_CFG1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x10],
    #[doc = "Low Power Wake-Up Delay"]
    pub LPWKUP_DELAY: crate::RWRegister<u32>,
    #[doc = "Active Voltage Trim Delay"]
    pub ACTIVE_VDELAY: crate::RWRegister<u32>,
    _reserved6: [u8; 0x8],
    #[doc = "Voltage Detect Status"]
    pub VD_STAT: crate::RWRegister<u32>,
    #[doc = "Core Voltage Detect Configuration"]
    pub VD_CORE_CFG: crate::RWRegister<u32>,
    #[doc = "System Voltage Detect Configuration"]
    pub VD_SYS_CFG: crate::RWRegister<u32>,
    #[doc = "IO Voltage Detect Configuration"]
    pub VD_IO_CFG: crate::RWRegister<u32>,
    #[doc = "External Voltage Domain Configuration"]
    pub EVD_CFG: crate::RWRegister<u32>,
    #[doc = "VDD Core Glitch Detect Status Control"]
    pub VDD_CORE_GLITCH_DETECT_SC: crate::RWRegister<u32>,
    _reserved7: [u8; 0x1b8],
    #[doc = "LDO_CORE Configuration"]
    pub CORELDO_CFG: crate::RWRegister<u32>,
    _reserved8: [u8; 0xfc],
    #[doc = "LDO_SYS Configuration"]
    pub SYSLDO_CFG: crate::RWRegister<u32>,
    _reserved9: [u8; 0xfc],
    #[doc = "DCDC Configuration"]
    pub DCDC_CFG: crate::RWRegister<u32>,
    #[doc = "DCDC Burst Configuration"]
    pub DCDC_BURST_CFG: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard features"]
            pub const STANDARD: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Control"]
pub mod SC {
    #[doc = "SPC Busy Status Flag"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not busy"]
            pub const BUSY_NO: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPC Power Mode Configuration Status Flag"]
    pub mod SPC_LP_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPC is in Active mode; the ACTIVE_CFG register has control"]
            pub const ACTIVE: u32 = 0;
            #[doc = "All power domains requested low-power mode; SPC entered a low-power state; power-mode configuration based on the LP_CFG register"]
            pub const LOW_POWER: u32 = 1;
        }
    }
    #[doc = "Power Domain Low-Power Mode Request"]
    pub mod SPC_LP_MODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Sleep mode with system clock running"]
            pub const MODE0: u32 = 0;
            #[doc = "DSLEEP with system clock off"]
            pub const MODE1: u32 = 1;
            #[doc = "PDOWN with system clock off"]
            pub const MODE2: u32 = 2;
            #[doc = "DPDOWN with system clock off"]
            pub const MODE8: u32 = 8;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Isolation Clear Flags"]
    pub mod ISO_CLR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPC Regulator Control"]
pub mod CNTRL {
    #[doc = "LDO_CORE Regulator Enable"]
    pub mod CORELDO_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
    }
    #[doc = "LDO_SYS Regulator Enable"]
    pub mod SYSLDO_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
    }
    #[doc = "DCDC_CORE Regulator Enable"]
    pub mod DCDC_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
    }
}
#[doc = "Low-Power Request Configuration"]
pub mod LPREQ_CFG {
    #[doc = "Low-Power Request Output Enable"]
    pub mod LPREQOE {
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
    #[doc = "Low-Power Request Output Pin Polarity Control"]
    pub mod LPREQPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High"]
            pub const HIGH: u32 = 0;
            #[doc = "Low"]
            pub const LOW: u32 = 1;
        }
    }
    #[doc = "Low-Power Request Output Override"]
    pub mod LPREQOV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not forced"]
            pub const FORCE_NO: u32 = 0;
            #[doc = "Forced low (ignore LPREQPOL settings)"]
            pub const FORCE_LOW: u32 = 2;
            #[doc = "Forced high (ignore LPREQPOL settings)"]
            pub const FORCE_HIGH: u32 = 3;
        }
    }
}
#[doc = "SPC Power Domain Mode Status"]
pub mod PD_STATUS {
    #[doc = "Power Request Status Flag"]
    pub mod PWR_REQ_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Did not request"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Requested"]
            pub const REQ_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Domain Low Power Request Flag"]
    pub mod PD_LP_REQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not request"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Requested"]
            pub const REQ_YES: u32 = 1;
        }
    }
    #[doc = "Power Domain Low Power Mode Request"]
    pub mod LP_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "SLEEP with system clock running"]
            pub const MODE0: u32 = 0;
            #[doc = "DSLEEP with system clock off"]
            pub const MODE1: u32 = 1;
            #[doc = "PDOWN with system clock off"]
            pub const MODE2: u32 = 2;
            #[doc = "DPDOWN with system clock off"]
            pub const MODE8: u32 = 8;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control"]
pub mod SRAMCTL {
    #[doc = "Voltage Select Margin"]
    pub mod VSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1.0 V"]
            pub const VSM1: u32 = 1;
            #[doc = "1.1 V"]
            pub const VSM2: u32 = 2;
            #[doc = "SRAM configured for 1.2 V operation"]
            pub const VSM3: u32 = 3;
        }
    }
    #[doc = "SRAM Voltage Update Request"]
    pub mod REQ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not request"]
            pub const REQ_NO: u32 = 0;
            #[doc = "Request"]
            pub const REQ_YES: u32 = 1;
        }
    }
    #[doc = "SRAM Voltage Update Request Acknowledge"]
    pub mod ACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not acknowledged"]
            pub const ACK_NO: u32 = 0;
            #[doc = "Acknowledged"]
            pub const ACK_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Active Power Mode Configuration"]
pub mod ACTIVE_CFG {
    #[doc = "LDO_CORE VDD Drive Strength"]
    pub mod CORELDO_VDD_DS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const LOW: u32 = 0;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 1;
        }
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    pub mod CORELDO_VDD_LVL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Regulate to mid voltage (1.0 V)"]
            pub const MID: u32 = 1;
            #[doc = "Regulate to normal voltage (1.1 V)"]
            pub const NORMAL: u32 = 2;
            #[doc = "Regulate to overdrive voltage (1.2 V)"]
            pub const OVER: u32 = 3;
        }
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    pub mod SYSLDO_VDD_DS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const LOW: u32 = 0;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 1;
        }
    }
    #[doc = "LDO_SYS VDD Regulator Voltage Level"]
    pub mod SYSLDO_VDD_LVL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal voltage (1.8 V)"]
            pub const NORMAL: u32 = 0;
            #[doc = "Overdrive voltage (2.5 V)"]
            pub const OVER: u32 = 1;
        }
    }
    #[doc = "DCDC VDD Drive Strength"]
    pub mod DCDC_VDD_DS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const LOW: u32 = 1;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 2;
        }
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    pub mod DCDC_VDD_LVL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Midvoltage (1.0 V)"]
            pub const DCDC01: u32 = 1;
            #[doc = "Normal voltage (1.1 V)"]
            pub const DCDC10: u32 = 2;
            #[doc = "Overdrive voltage (1.2 V)"]
            pub const DCDC11: u32 = 3;
        }
    }
    #[doc = "VDD Core Glitch Detect Disable"]
    pub mod GLITCH_DETECT_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD Core Low Voltage Glitch Detect enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "VDD Core Low Voltage Glitch Detect disabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    pub mod LPBUFF_EN {
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
    #[doc = "Bandgap Mode"]
    pub mod BGMODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bandgap disabled"]
            pub const BGMODE0: u32 = 0;
            #[doc = "Bandgap enabled, buffer disabled"]
            pub const BGMODE01: u32 = 1;
            #[doc = "Bandgap enabled, buffer enabled"]
            pub const BGMODE10: u32 = 2;
        }
    }
    #[doc = "VDD Voltage Detect Disable"]
    pub mod VDD_VD_DISABLE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Core Low-Voltage Detection Enable"]
    pub mod CORE_LVDE {
        pub const offset: u32 = 24;
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
    #[doc = "System Low-Voltage Detection Enable"]
    pub mod SYS_LVDE {
        pub const offset: u32 = 25;
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
    #[doc = "IO Low-Voltage Detection Enable"]
    pub mod IO_LVDE {
        pub const offset: u32 = 26;
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
    #[doc = "Core High-Voltage Detection Enable"]
    pub mod CORE_HVDE {
        pub const offset: u32 = 27;
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
    #[doc = "System High-Voltage Detection Enable"]
    pub mod SYS_HVDE {
        pub const offset: u32 = 28;
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
    #[doc = "IO High-Voltage Detection Enable"]
    pub mod IO_HVDE {
        pub const offset: u32 = 29;
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
#[doc = "Active Power Mode Configuration 1"]
pub mod ACTIVE_CFG1 {
    #[doc = "Active Config Chip Control"]
    pub mod SOC_CNTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Low-Power Mode Configuration"]
pub mod LP_CFG {
    #[doc = "LDO_CORE VDD Drive Strength"]
    pub mod CORELDO_VDD_DS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const LOW: u32 = 0;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 1;
        }
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    pub mod CORELDO_VDD_LVL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Under voltage (0.95 V)"]
            pub const UNDER: u32 = 0;
            #[doc = "Mid voltage (1.0 V)"]
            pub const MID: u32 = 1;
            #[doc = "Normal voltage (1.1 V)"]
            pub const NORMAL: u32 = 2;
            #[doc = "Overdrive voltage (1.2 V)"]
            pub const OVER: u32 = 3;
        }
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    pub mod SYSLDO_VDD_DS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const LOW: u32 = 0;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 1;
        }
    }
    #[doc = "DCDC VDD Drive Strength"]
    pub mod DCDC_VDD_DS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pulse refresh"]
            pub const PULSE: u32 = 0;
            #[doc = "Low"]
            pub const LOW: u32 = 1;
            #[doc = "Normal"]
            pub const NORMAL: u32 = 2;
        }
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    pub mod DCDC_VDD_LVL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no description available"]
            pub const VDD00: u32 = 0;
            #[doc = "Mid voltage (1.0 V)"]
            pub const VDD01: u32 = 1;
            #[doc = "Normal voltage (1.1 V)"]
            pub const VDD10: u32 = 2;
            #[doc = "Overdrive voltage (1.2 V)"]
            pub const VDD11: u32 = 3;
        }
    }
    #[doc = "VDD Core Glitch Detect Disable"]
    pub mod GLITCH_DETECT_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "CORE VDD Internal Voltage Scaling (IVS) Enable"]
    pub mod COREVDD_IVS_EN {
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
    #[doc = "CMP Bandgap Buffer Enable"]
    pub mod LPBUFF_EN {
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
    #[doc = "Bandgap Mode"]
    pub mod BGMODE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bandgap disabled"]
            pub const BGMODE0: u32 = 0;
            #[doc = "Bandgap enabled, buffer disabled"]
            pub const BGMODE01: u32 = 1;
            #[doc = "Bandgap enabled, buffer enabled"]
            pub const BGMODE10: u32 = 2;
        }
    }
    #[doc = "Low-Power IREF Enable"]
    pub mod LP_IREFEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable for power saving in Deep Power Down mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Core Low Voltage Detect Enable"]
    pub mod CORE_LVDE {
        pub const offset: u32 = 24;
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
    #[doc = "System Low Voltage Detect Enable"]
    pub mod SYS_LVDE {
        pub const offset: u32 = 25;
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
    #[doc = "IO Low Voltage Detect Enable"]
    pub mod IO_LVDE {
        pub const offset: u32 = 26;
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
    #[doc = "Core High Voltage Detect Enable"]
    pub mod CORE_HVDE {
        pub const offset: u32 = 27;
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
    #[doc = "System High Voltage Detect Enable"]
    pub mod SYS_HVDE {
        pub const offset: u32 = 28;
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
    #[doc = "IO High Voltage Detect Enable"]
    pub mod IO_HVDE {
        pub const offset: u32 = 29;
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
#[doc = "Low Power Mode Configuration 1"]
pub mod LP_CFG1 {
    #[doc = "Low-Power Configuration Chip Control"]
    pub mod SOC_CNTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Low Power Wake-Up Delay"]
pub mod LPWKUP_DELAY {
    #[doc = "Low-Power Wake-Up Delay"]
    pub mod LPWKUP_DELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Active Voltage Trim Delay"]
pub mod ACTIVE_VDELAY {
    #[doc = "Active Voltage Delay"]
    pub mod ACTIVE_VDELAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Voltage Detect Status"]
pub mod VD_STAT {
    #[doc = "Core VDD Low-Voltage Detect Flag"]
    pub mod COREVDD_LVDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
    #[doc = "System VDD Low-Voltage Detect Flag"]
    pub mod SYSVDD_LVDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
    #[doc = "IO VDD LVD Flag"]
    pub mod IOVDD_LVDF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
    #[doc = "Core VDD HVD Flag"]
    pub mod COREVDD_HVDF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
    #[doc = "System VDD HVD Flag"]
    pub mod SYSVDD_HVDF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
    #[doc = "IO VDD HVD Flag"]
    pub mod IOVDD_HVDF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Event not detected"]
            pub const EVENT_NO: u32 = 0;
            #[doc = "Event detected"]
            pub const EVENT_YES: u32 = 1;
        }
    }
}
#[doc = "Core Voltage Detect Configuration"]
pub mod VD_CORE_CFG {
    #[doc = "Core VDD LVD Reset Enable"]
    pub mod LVDRE {
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
    #[doc = "Core VDD LVD Interrupt Enable"]
    pub mod LVDIE {
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
    #[doc = "Core VDD HVD Reset Enable"]
    pub mod HVDRE {
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
    #[doc = "Core VDD HVD Interrupt Enable"]
    pub mod HVDIE {
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
    #[doc = "Core Voltage Detect Reset Enable Lock"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0;
            #[doc = "Deny"]
            pub const DENY: u32 = 1;
        }
    }
}
#[doc = "System Voltage Detect Configuration"]
pub mod VD_SYS_CFG {
    #[doc = "System VDD LVD Reset Enable"]
    pub mod LVDRE {
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
    #[doc = "System VDD LVD Interrupt Enable"]
    pub mod LVDIE {
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
    #[doc = "System VDD HVD Reset Enable"]
    pub mod HVDRE {
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
    #[doc = "System VDD HVD Interrupt Enable"]
    pub mod HVDIE {
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
    #[doc = "System VDD Low-Voltage Level Select"]
    pub mod LVSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "Safe"]
            pub const SAFE: u32 = 1;
        }
    }
    #[doc = "System Voltage Detect Reset Enable Lock"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0;
            #[doc = "Deny"]
            pub const DENY: u32 = 1;
        }
    }
}
#[doc = "IO Voltage Detect Configuration"]
pub mod VD_IO_CFG {
    #[doc = "IO VDD LVD Reset Enable"]
    pub mod LVDRE {
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
    #[doc = "IO VDD LVD Interrupt Enable"]
    pub mod LVDIE {
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
    #[doc = "IO VDD HVD Reset Enable"]
    pub mod HVDRE {
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
    #[doc = "IO VDD HVD Interrupt Enable"]
    pub mod HVDIE {
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
    #[doc = "IO VDD Low-Voltage Level Select"]
    pub mod LVSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "Safe"]
            pub const SAFE: u32 = 1;
        }
    }
    #[doc = "IO Voltage Detect Reset Enable Lock"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow"]
            pub const ALLOW: u32 = 0;
            #[doc = "Deny"]
            pub const DENY: u32 = 1;
        }
    }
}
#[doc = "External Voltage Domain Configuration"]
pub mod EVD_CFG {
    #[doc = "External Voltage Domain Isolation"]
    pub mod EVDISO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Voltage Domain Low-Power Isolation"]
    pub mod EVDLPISO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Voltage Domain Status"]
    pub mod EVDSTAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VDD Core Glitch Detect Status Control"]
pub mod VDD_CORE_GLITCH_DETECT_SC {
    #[doc = "Counter Select"]
    pub mod CNT_SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const BIT0: u32 = 0;
            #[doc = "1"]
            pub const BIT1: u32 = 1;
            #[doc = "2"]
            pub const BIT2: u32 = 2;
            #[doc = "3"]
            pub const BIT3: u32 = 3;
        }
    }
    #[doc = "Timeout"]
    pub mod TIMEOUT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Core VDD Glitch Detect Reset Enable"]
    pub mod RE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GLITCH_DETECT_FLAG[CNT_SELECT] does not generate POR/LVD reset"]
            pub const DISABLED: u32 = 0;
            #[doc = "GLITCH_DETECT_FLAG[CNT_SELECT] does generate POR/LVD reset"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Core VDD Glitch Detect Interrupt Enable"]
    pub mod IE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GLITCH_DETECT_FLAG[CNT_SELECT] does not generate hardware interrupt (user polling)"]
            pub const DISABLED: u32 = 0;
            #[doc = "GLITCH_DETECT_FLAG[CNT_SELECT] does generate hardware interrupt"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    pub mod GLITCH_DETECT_FLAG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VDD Core Voltage Glitch Detect Reset Enable Lock Bit"]
    pub mod LOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Writes to RE are allowed."]
            pub const DISABLED: u32 = 0;
            #[doc = "Writes to RE are ignored."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "LDO_CORE Configuration"]
pub mod CORELDO_CFG {
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable"]
    pub mod DPDOWN_PULLDOWN_DISABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LDO_CORE pulldown in Deep Power Down not disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "LDO_CORE pulldown in Deep Power Down disabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "LDO_SYS Configuration"]
pub mod SYSLDO_CFG {
    #[doc = "Current Sink Enable"]
    pub mod ISINKEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "DCDC Configuration"]
pub mod DCDC_CFG {
    #[doc = "DCDC Burst Frequency Control Enable"]
    pub mod FREQ_CNTRL_ON {
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
    #[doc = "DCDC Burst Frequency Control"]
    pub mod FREQ_CNTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC Bleed Enable"]
    pub mod BLEED_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not add"]
            pub const ADD_NO: u32 = 0;
            #[doc = "Add"]
            pub const ADD_YES: u32 = 1;
        }
    }
}
#[doc = "DCDC Burst Configuration"]
pub mod DCDC_BURST_CFG {
    #[doc = "Software Burst Request"]
    pub mod BURST_REQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not generate"]
            pub const GEN_NO: u32 = 0;
            #[doc = "Generate"]
            pub const GEN_YES: u32 = 1;
        }
    }
    #[doc = "External Burst Request Enable"]
    pub mod EXT_BURST_EN {
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
    #[doc = "Burst Acknowledge Flag"]
    pub mod BURST_ACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not complete"]
            pub const COMPL_NO: u32 = 0;
            #[doc = "Completed"]
            pub const COMPL_YES: u32 = 1;
        }
    }
    #[doc = "Refresh Count Value"]
    pub mod PULSE_REFRESH_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
