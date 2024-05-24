#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "USBPHY"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Power Down Register"]
    pub PWD: crate::RWRegister<u32>,
    #[doc = "Power Down Register"]
    pub PWD_SET: crate::RWRegister<u32>,
    #[doc = "Power Down Register"]
    pub PWD_CLR: crate::RWRegister<u32>,
    #[doc = "Power Down Register"]
    pub PWD_TOG: crate::RWRegister<u32>,
    #[doc = "Tx Control Register"]
    pub TX: crate::RWRegister<u32>,
    #[doc = "Tx Control Register"]
    pub TX_SET: crate::RWRegister<u32>,
    #[doc = "Tx Control Register"]
    pub TX_CLR: crate::RWRegister<u32>,
    #[doc = "Tx Control Register"]
    pub TX_TOG: crate::RWRegister<u32>,
    #[doc = "Rx Control Register"]
    pub RX: crate::RWRegister<u32>,
    #[doc = "Rx Control Register"]
    pub RX_SET: crate::RWRegister<u32>,
    #[doc = "Rx Control Register"]
    pub RX_CLR: crate::RWRegister<u32>,
    #[doc = "Rx Control Register"]
    pub RX_TOG: crate::RWRegister<u32>,
    #[doc = "General Purpose Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "General Purpose Control Register"]
    pub CTRL_SET: crate::RWRegister<u32>,
    #[doc = "General Purpose Control Register"]
    pub CTRL_CLR: crate::RWRegister<u32>,
    #[doc = "General Purpose Control Register"]
    pub CTRL_TOG: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STATUS: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "Debug Register 0"]
    pub DEBUG0: crate::RWRegister<u32>,
    #[doc = "Debug Register 0"]
    pub DEBUG0_SET: crate::RWRegister<u32>,
    #[doc = "Debug Register 0"]
    pub DEBUG0_CLR: crate::RWRegister<u32>,
    #[doc = "Debug Register 0"]
    pub DEBUG0_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x20],
    #[doc = "Version Register"]
    pub VERSION: crate::RWRegister<u32>,
    _reserved2: [u8; 0xc],
    #[doc = "IP Block Register"]
    pub IP: crate::RWRegister<u32>,
    #[doc = "IP Block Register"]
    pub IP_SET: crate::RWRegister<u32>,
    #[doc = "IP Block Register"]
    pub IP_CLR: crate::RWRegister<u32>,
    #[doc = "IP Block Register"]
    pub IP_TOG: crate::RWRegister<u32>,
    #[doc = "PLL SIC Register"]
    pub PLL_SIC: crate::RWRegister<u32>,
    #[doc = "PLL SIC Register"]
    pub PLL_SIC_SET: crate::RWRegister<u32>,
    #[doc = "PLL SIC Register"]
    pub PLL_SIC_CLR: crate::RWRegister<u32>,
    #[doc = "PLL SIC Register"]
    pub PLL_SIC_TOG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "VBUS Detect Register"]
    pub USB1_VBUS_DETECT: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Register"]
    pub USB1_VBUS_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Register"]
    pub USB1_VBUS_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Register"]
    pub USB1_VBUS_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Status Register"]
    pub USB1_VBUS_DET_STAT: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Status Register"]
    pub USB1_VBUS_DET_STAT_SET: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Status Register"]
    pub USB1_VBUS_DET_STAT_CLR: crate::RWRegister<u32>,
    #[doc = "VBUS Detect Status Register"]
    pub USB1_VBUS_DET_STAT_TOG: crate::RWRegister<u32>,
    #[doc = "Charger Detect Register"]
    pub USB1_CHRG_DETECT: crate::RWRegister<u32>,
    #[doc = "Charger Detect Register"]
    pub USB1_CHRG_DETECT_SET: crate::RWRegister<u32>,
    #[doc = "Charger Detect Register"]
    pub USB1_CHRG_DETECT_CLR: crate::RWRegister<u32>,
    #[doc = "Charger Detect Register"]
    pub USB1_CHRG_DETECT_TOG: crate::RWRegister<u32>,
    #[doc = "Charger Detect Status Register"]
    pub USB1_CHRG_DET_STAT: crate::RWRegister<u32>,
    #[doc = "Charger Detect Status Register"]
    pub USB1_CHRG_DET_STAT_SET: crate::RWRegister<u32>,
    #[doc = "Charger Detect Status Register"]
    pub USB1_CHRG_DET_STAT_CLR: crate::RWRegister<u32>,
    #[doc = "Charger Detect Status Register"]
    pub USB1_CHRG_DET_STAT_TOG: crate::RWRegister<u32>,
    #[doc = "Analog Control Register"]
    pub ANACTRL: crate::RWRegister<u32>,
    #[doc = "Analog Control Register"]
    pub ANACTRL_SET: crate::RWRegister<u32>,
    #[doc = "Analog Control Register"]
    pub ANACTRL_CLR: crate::RWRegister<u32>,
    #[doc = "Analog Control Register"]
    pub ANACTRL_TOG: crate::RWRegister<u32>,
    _reserved4: [u8; 0x20],
    #[doc = "Trim Register"]
    pub TRIM_OVERRIDE_EN: crate::RWRegister<u32>,
    #[doc = "Trim Register"]
    pub TRIM_OVERRIDE_EN_SET: crate::RWRegister<u32>,
    #[doc = "Trim Register"]
    pub TRIM_OVERRIDE_EN_CLR: crate::RWRegister<u32>,
    #[doc = "Trim Register"]
    pub TRIM_OVERRIDE_EN_TOG: crate::RWRegister<u32>,
    #[doc = "PFD Register A"]
    pub PFDA: crate::RWRegister<u32>,
    #[doc = "PFD Register A"]
    pub PFDA_SET: crate::RWRegister<u32>,
    #[doc = "PFD Register A"]
    pub PFDA_CLR: crate::RWRegister<u32>,
    #[doc = "PFD Register A"]
    pub PFDA_TOG: crate::RWRegister<u32>,
}
#[doc = "Power Down Register"]
pub mod PWD {
    #[doc = "Power down USB FS drivers"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Provide bias to allow enable of FS Tx drivers"]
            pub const FSTX_BIAS_ENABLE: u32 = 0;
            #[doc = "Power down FS Tx drivers and configure them to high impedance state"]
            pub const FSTX_BIAS_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USBPHY current bias block"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable current mirror circuits for general transceiver/detector bias"]
            pub const IBIAS_ENABLE: u32 = 0;
            #[doc = "Power down current bias mirrors"]
            pub const IBIAS_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USBPHY V-I converter and current mirror"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Tx V-to-I converter and current mirrors"]
            pub const V2I_BIAS_ENABLE: u32 = 0;
            #[doc = "Power down Tx V-to-I converter and current mirrors"]
            pub const V2I_BIAS_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USB HS receiver envelope detector and Host Disconnect comparator"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the HS Rx envelope detector and Host Disconnect comparator"]
            pub const RX_ENVHD_ENABLE: u32 = 0;
            #[doc = "Power down the HS Rx envelope detector and Host Disconnect comparator"]
            pub const RX_ENVHD_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USB FS differential receiver"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the FS differential receiver"]
            pub const FS_RXDIFF_ENABLE: u32 = 0;
            #[doc = "Power down the FS differential receiver"]
            pub const FS_RXDIFF_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USB HS differential receiver"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the HS differential receiver"]
            pub const HS_RXDIFF_ENABLE: u32 = 0;
            #[doc = "Power down the HS differential receiver"]
            pub const HS_RXDIFF_PWD: u32 = 1;
        }
    }
    #[doc = "Power down USBPHY receiver circuits except the FS differential comparator"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the PHY\'s receiver block except for the FS differential receiver"]
            pub const RX_BIAS_ENABLE: u32 = 0;
            #[doc = "Power down the PHY\'s receiver block except for the FS differential receiver"]
            pub const RX_BIAS_PWD: u32 = 1;
        }
    }
}
#[doc = "Power Down Register"]
pub mod PWD_SET {
    #[doc = "Power down USB FS drivers"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY current bias block"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY V-I converter and current mirror"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS receiver envelope detector and Host Disconnect comparator"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB FS differential receiver"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS differential receiver"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY receiver circuits except the FS differential comparator"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Down Register"]
pub mod PWD_CLR {
    #[doc = "Power down USB FS drivers"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY current bias block"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY V-I converter and current mirror"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS receiver envelope detector and Host Disconnect comparator"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB FS differential receiver"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS differential receiver"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY receiver circuits except the FS differential comparator"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Down Register"]
pub mod PWD_TOG {
    #[doc = "Power down USB FS drivers"]
    pub mod TXPWDFS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY current bias block"]
    pub mod TXPWDIBIAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY V-I converter and current mirror"]
    pub mod TXPWDV2I {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS receiver envelope detector and Host Disconnect comparator"]
    pub mod RXPWDENV {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB FS differential receiver"]
    pub mod RXPWD1PT1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USB HS differential receiver"]
    pub mod RXPWDDIFF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down USBPHY receiver circuits except the FS differential comparator"]
    pub mod RXPWDRX {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Control Register"]
pub mod TX {
    #[doc = "HS Tx output current trim"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Control Register"]
pub mod TX_SET {
    #[doc = "HS Tx output current trim"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Control Register"]
pub mod TX_CLR {
    #[doc = "HS Tx output current trim"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Control Register"]
pub mod TX_TOG {
    #[doc = "HS Tx output current trim"]
    pub mod D_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim"]
    pub mod TXCAL45DN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim"]
    pub mod TXCAL45DP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Control Register"]
pub mod RX {
    #[doc = "Envelope detector trip point"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trip-Level Voltage is 0.1000 V"]
            pub const ENV_TRIM_NOM: u32 = 0;
            #[doc = "Trip-Level Voltage is 0.1125 V"]
            pub const ENV_TRIM_MEDHI: u32 = 1;
            #[doc = "Trip-Level Voltage is 0.1250 V"]
            pub const ENV_TRIM_HI: u32 = 2;
            #[doc = "Trip-Level Voltage is 0.0875 V"]
            pub const ENV_TRIM_LO: u32 = 3;
        }
    }
    #[doc = "Disconnect detector trip point"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trip-Level Voltage is 0.56875 V"]
            pub const DISCON_TRIM_NOM: u32 = 0;
            #[doc = "Trip-Level Voltage is 0.55000 V"]
            pub const DISCON_TRIM_LO: u32 = 1;
            #[doc = "Trip-Level Voltage is 0.58125 V"]
            pub const DISCON_TRIM_MEDHI: u32 = 2;
            #[doc = "Trip-Level Voltage is 0.60000 V"]
            pub const DISCON_TRIM_HI: u32 = 3;
        }
    }
}
#[doc = "Rx Control Register"]
pub mod RX_SET {
    #[doc = "Envelope detector trip point"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detector trip point"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Control Register"]
pub mod RX_CLR {
    #[doc = "Envelope detector trip point"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detector trip point"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Control Register"]
pub mod RX_TOG {
    #[doc = "Envelope detector trip point"]
    pub mod ENVADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detector trip point"]
    pub mod DISCONADJ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Control Register"]
pub mod CTRL {
    #[doc = "EN ID change IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detect"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for Host disconnect"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device disconnect indication"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables 200 kohm pullup resistors on USB_DP and USB_DM pins"]
            pub const PLUGIN_DISABLE: u32 = 0;
            #[doc = "Enables 200 kohm pullup resistors on USB_DP and USB_DM pins"]
            pub const PLUGIN_ENABLE: u32 = 1;
        }
    }
    #[doc = "Device plugin polarity interrupt configuration for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable internal OTG ID detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the PHY\'s internal ID pin detection circuit"]
            pub const ID_DET_DISABLE: u32 = 0;
            #[doc = "Enable the PHY\'s internal ID pin detection circuit"]
            pub const ID_DET_ENABLE: u32 = 1;
        }
    }
    #[doc = "Resume IRQ"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ Resume detect"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for non-standard resistive plugged-in detection"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device connected indicator for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB clock switch option"]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 2 operation"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 3 operation"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Wakeup IRQ"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable autoresume"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear clock gate"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear PWD register bits"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "False, when ID resistance to ground is less than Ra_Plug_ID, indicating Host (A) side"]
            pub const ID_HOST: u32 = 0;
            #[doc = "True, when ID resistance is greater than Rb_Plug_ID, indicating Device (B) side"]
            pub const ID_DEVICE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI clock gate"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Control Register"]
pub mod CTRL_SET {
    #[doc = "EN ID change IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detect"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for Host disconnect"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device disconnect indication"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device plugin polarity interrupt configuration for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable internal OTG ID detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ Resume detect"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for non-standard resistive plugged-in detection"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device connected indicator for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB clock switch option"]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 2 operation"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 3 operation"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Wakeup IRQ"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable autoresume"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear clock gate"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear PWD register bits"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI clock gate"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Control Register"]
pub mod CTRL_CLR {
    #[doc = "EN ID change IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detect"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for Host disconnect"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device disconnect indication"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device plugin polarity interrupt configuration for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable internal OTG ID detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ Resume detect"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for non-standard resistive plugged-in detection"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device connected indicator for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB clock switch option"]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 2 operation"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 3 operation"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Wakeup IRQ"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable autoresume"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear clock gate"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear PWD register bits"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI clock gate"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Control Register"]
pub mod CTRL_TOG {
    #[doc = "EN ID change IRQ"]
    pub mod ENOTG_ID_CHG_IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disconnect detect"]
    pub mod ENHOSTDISCONDETECT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for Host disconnect"]
    pub mod ENIRQHOSTDISCON {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device disconnect indication"]
    pub mod HOSTDISCONDETECT_IRQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables non-standard resistive plugged-in detection"]
    pub mod ENDEVPLUGINDETECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device plugin polarity interrupt configuration for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_POLARITY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID change IRQ"]
    pub mod OTG_ID_CHG_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable internal OTG ID detector"]
    pub mod ENOTGIDDETECT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUMEIRQSTICKY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ Resume detect"]
    pub mod ENIRQRESUMEDETECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume IRQ"]
    pub mod RESUME_IRQ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable IRQ for non-standard resistive plugged-in detection"]
    pub mod ENIRQDEVPLUGIN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device connected indicator for non-standard resistive plugged-in detection"]
    pub mod DEVPLUGIN_IRQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB clock switch option"]
    pub mod DATA_ON_LRADC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 2 operation"]
    pub mod ENUTMILEVEL2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable level 3 operation"]
    pub mod ENUTMILEVEL3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Wakeup IRQ"]
    pub mod ENIRQWAKEUP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup IRQ"]
    pub mod WAKEUP_IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable autoresume"]
    pub mod AUTORESUME_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear clock gate"]
    pub mod ENAUTOCLR_CLKGATE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Autoclear PWD register bits"]
    pub mod ENAUTOCLR_PHY_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID value"]
    pub mod OTG_ID_VALUE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI Suspend"]
    pub mod UTMI_SUSPENDM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UTMI clock gate"]
    pub mod CLKGATE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset"]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STATUS {
    #[doc = "USB 3.3 V / 1.8 V supply status"]
    pub mod OK_STATUS_3V {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "One or both of the 1.8 V and 3.3 V supplies to the PHY are not powered"]
            pub const POWER_3_1P8_OK: u32 = 0;
            #[doc = "Both of the 1.8 V and 3.3 V supplies to the PHY are powered"]
            pub const POWER_3_1P8_BAD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host disconnect status"]
    pub mod HOSTDISCONDETECT_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "USB cable disconnect has not been detected at the local Host"]
            pub const NO_DISCONNECT: u32 = 0;
            #[doc = "USB cable disconnect has been detected at the local Host"]
            pub const DISCONNECT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status indicator for non-standard resistive plugged-in detection."]
    pub mod DEVPLUGIN_STATUS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No attachment to a USB Host is detected"]
            pub const NO_CABLE: u32 = 0;
            #[doc = "Cable attachment to a USB Host is detected"]
            pub const CABLE_ATTACH: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID status"]
    pub mod OTGID_STATUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "False, when ID resistance to ground is less than Ra_Plug_ID, indicating Host (A) side"]
            pub const ID_HOST: u32 = 0;
            #[doc = "True, when ID resistance is greater than Rb_Plug_ID, indicating Device (B) side"]
            pub const ID_DEVICE: u32 = 1;
        }
    }
    #[doc = "Resume status"]
    pub mod RESUME_STATUS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug Register 0"]
pub mod DEBUG0 {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP/DM pulldown resistors in Host pulldown overdrive mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host pulldown overdrive mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug Register 0"]
pub mod DEBUG0_SET {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP/DM pulldown resistors in Host pulldown overdrive mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host pulldown overdrive mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug Register 0"]
pub mod DEBUG0_CLR {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP/DM pulldown resistors in Host pulldown overdrive mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host pulldown overdrive mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug Register 0"]
pub mod DEBUG0_TOG {
    #[doc = "Hold OTG_ID"]
    pub mod OTGIDPIOLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select DP/DM pulldown resistors in Host pulldown overdrive mode"]
    pub mod HSTPULLDOWN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Host pulldown overdrive mode"]
    pub mod ENHSTPULLDOWN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version Register"]
pub mod VERSION {
    #[doc = "STEP"]
    pub mod STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MINOR"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAJOR"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Block Register"]
pub mod IP {
    #[doc = "Power control Suspend option"]
    pub mod POWER_CONTROL_SUSPEND_OPTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Block Register"]
pub mod IP_SET {
    #[doc = "Power control Suspend option"]
    pub mod POWER_CONTROL_SUSPEND_OPTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Block Register"]
pub mod IP_CLR {
    #[doc = "Power control Suspend option"]
    pub mod POWER_CONTROL_SUSPEND_OPTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Block Register"]
pub mod IP_TOG {
    #[doc = "Power control Suspend option"]
    pub mod POWER_CONTROL_SUSPEND_OPTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL SIC Register"]
pub mod PLL_SIC {
    #[doc = "Determines USB PLL operation status during Suspend"]
    pub mod MISC2_CONTROL0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power up PLL without regard to state of UTMI_SUSPENDM"]
            pub const PLL_ON_SUSPEND: u32 = 0;
            #[doc = "Power down PLL when in Suspend bus state"]
            pub const PLL_OFF_SUSPEND: u32 = 1;
        }
    }
    #[doc = "PLL multi-phase clock outputs enable"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable PLL multi-phase 480 MHz clock outputs"]
            pub const PLL_MP_DISABLE: u32 = 0;
            #[doc = "Enable PLL multi-phase 480 MHz clock outputs"]
            pub const PLL_MP_ENABLE: u32 = 1;
        }
    }
    #[doc = "USB PLL powerup control"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Powers down the USB PLL"]
            pub const PLL_FORCE_PWD: u32 = 0;
            #[doc = "Allows powerup of USB PLL"]
            pub const PLL_ALLOW_POWERUP: u32 = 1;
        }
    }
    #[doc = "PLL output clock enable"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable PLL single phase 480 MHz clock output"]
            pub const PLL_OUT_DISABLE: u32 = 0;
            #[doc = "Enable PLL single phase 480 MHz clock output"]
            pub const PLL_OUT_ENABLE: u32 = 1;
        }
    }
    #[doc = "Bypass USB PLL"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select PLL 480 MHz output clock for output mux"]
            pub const PLL_NO_BYPASS: u32 = 0;
            #[doc = "Select PLL input reference clock for output mux"]
            pub const PLL_BYPASS: u32 = 1;
        }
    }
    #[doc = "Reference bias power control"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control reference bias with PLL POWER internal state signal"]
            pub const BIAS_PLLPOWER: u32 = 0;
            #[doc = "Control reference bias with PLL_SIC[REFBIAS_PWD]"]
            pub const BIAS_REFBIAS_PWD: u32 = 1;
        }
    }
    #[doc = "Power down reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable master reference bias in direct power control mode"]
            pub const REFBIAS_ENABLED: u32 = 0;
            #[doc = "Power down master reference bias in direct power control mode"]
            pub const REFBIAS_PWD: u32 = 1;
        }
    }
    #[doc = "Enable PLL regulator"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the local regulator for the PLL"]
            pub const PLL_REG_DISABLE: u32 = 0;
            #[doc = "Enable the local regulator for the PLL"]
            pub const PLL_REG_ENABLE: u32 = 1;
        }
    }
    #[doc = "PLL divider value configuration"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configure for 32 MHz input clock (Divide by 15)"]
            pub const PLL_DIV_15: u32 = 0;
            #[doc = "Configure for 30 MHz input clock (Divide by 16)"]
            pub const PLL_DIV_16: u32 = 1;
            #[doc = "Configure for 24 MHz input clock (Divide by 20)"]
            pub const PLL_DIV_20: u32 = 2;
            #[doc = "Configure for 20 MHz input clock (Divide by 24)"]
            pub const PLL_DIV_24: u32 = 4;
            #[doc = "Configure for 19.2 MHz input clock (Divide by 25)"]
            pub const PLL_DIV_25: u32 = 5;
            #[doc = "Configure for 16 MHz input clock (Divide by 30)"]
            pub const PLL_DIV_30: u32 = 6;
            #[doc = "Configure for 12 MHz input clock (Divide by 40) May give marginal jitter results."]
            pub const PLL_DIV_32: u32 = 7;
        }
    }
    #[doc = "USB PLL lock status indicator"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The USB PLL is not currently locked"]
            pub const PLL_NOT_LOCKED: u32 = 0;
            #[doc = "The USB PLL is currently locked"]
            pub const PLL_LOCKED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL SIC Register"]
pub mod PLL_SIC_SET {
    #[doc = "Determines USB PLL operation status during Suspend"]
    pub mod MISC2_CONTROL0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL multi-phase clock outputs enable"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL powerup control"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output clock enable"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass USB PLL"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference bias power control"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL regulator"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL lock status indicator"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL SIC Register"]
pub mod PLL_SIC_CLR {
    #[doc = "Determines USB PLL operation status during Suspend"]
    pub mod MISC2_CONTROL0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL multi-phase clock outputs enable"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL powerup control"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output clock enable"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass USB PLL"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference bias power control"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL regulator"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL lock status indicator"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLL SIC Register"]
pub mod PLL_SIC_TOG {
    #[doc = "Determines USB PLL operation status during Suspend"]
    pub mod MISC2_CONTROL0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL multi-phase clock outputs enable"]
    pub mod PLL_EN_USB_CLKS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL powerup control"]
    pub mod PLL_POWER {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output clock enable"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass USB PLL"]
    pub mod PLL_BYPASS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference bias power control"]
    pub mod REFBIAS_PWD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power down reference bias"]
    pub mod REFBIAS_PWD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable PLL regulator"]
    pub mod PLL_REG_ENABLE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration"]
    pub mod PLL_DIV_SEL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB PLL lock status indicator"]
    pub mod PLL_LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Register"]
pub mod USB1_VBUS_DETECT {
    #[doc = "VBUS comparator threshold"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4.0 V"]
            pub const VBUS_VLD_4P0: u32 = 0;
            #[doc = "4.1 V"]
            pub const VBUS_VLD_4P1: u32 = 1;
            #[doc = "4.2 V"]
            pub const VBUS_VLD_4P2: u32 = 2;
            #[doc = "4.3 V"]
            pub const VBUS_VLD_4P3: u32 = 3;
            #[doc = "4.4 V (Default)"]
            pub const VBUS_VLD_4P4: u32 = 4;
            #[doc = "4.5 V"]
            pub const VBUS_VLD_4P5: u32 = 5;
            #[doc = "4.6 V"]
            pub const VBUS_VLD_4P6: u32 = 6;
            #[doc = "4.7 V"]
            pub const VBUS_VLD_4P7: u32 = 7;
        }
    }
    #[doc = "VBUS detect signal local override"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND"]
            pub const VBUS_NO_OVERRIDE: u32 = 0;
            #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
            pub const VBUS_OVERRIDE: u32 = 1;
        }
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to the USB subsystem"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB subsystem"]
            pub const VBUS_VLD_OUT: u32 = 0;
            #[doc = "Use the VBUS_VALID_3V comparator results for signal reported to the USB subsystem"]
            pub const VBUS_VLD_3V_OUT: u32 = 1;
        }
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB subsystem"]
            pub const USE_VBUS_VLD: u32 = 0;
            #[doc = "Use the Session Valid comparator results for signal reported to the USB subsystem"]
            pub const USE_ASESS_VLD: u32 = 1;
            #[doc = "Use the Session Valid comparator results for signal reported to the USB subsystem"]
            pub const USE_BSESS_VLD: u32 = 2;
        }
    }
    #[doc = "Enable local ID pin status override"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use PHY\'s ID pin detector or external override for ID status"]
            pub const NO_PHY_ID_OVERRIDE: u32 = 0;
            #[doc = "Allow local override of ID pin detection status"]
            pub const USE_PHY_ID_OVERRIDE: u32 = 1;
        }
    }
    #[doc = "ID pin status local override value"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external ID pin status override using value supplied from outside the PHY"]
    pub mod EXT_ID_OVERRIDE_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Determine the reported ID pin status results using the PHY\'s internal detector or local override"]
            pub const USE_PHY_ID: u32 = 0;
            #[doc = "Determine the reported ID pin status results using the external ID signal value"]
            pub const USE_EXT_ID: u32 = 1;
        }
    }
    #[doc = "Enable external VBUS override using value supplied from outside the PHY"]
    pub mod EXT_VBUS_OVERRIDE_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Determine the reported VBUS detection results using the PHY\'s internal comparators/detectors or local overrides"]
            pub const USE_PHY_VBUS: u32 = 0;
            #[doc = "Determine the reported VBUS detection results using the external VBUS_VALID value"]
            pub const USB_EXT_VBUS: u32 = 1;
        }
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_B {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
            pub const USE_VBUS_VLD: u32 = 0;
            #[doc = "Use the Session Valid detector for VBUS_VALID results. The Session Valid threshold is >= 0.8V and <= 4.0V."]
            pub const USE_SESS_VLD: u32 = 1;
        }
    }
    #[doc = "Individual enable controls for the VBUS detection comparators"]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power down the VBUS_VALID comparator"]
            pub const VBUS_VALID_DISABLE: u32 = 0;
            #[doc = "Enable the VBUS_VALID comparator"]
            pub const VBUS_VALID_ENABLE: u32 = 1;
        }
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VBUS discharge resistor is disabled"]
            pub const VBUS_DCHG_OFF: u32 = 0;
            #[doc = "VBUS discharge resistor is enabled"]
            pub const VBUS_DCHG_ON: u32 = 1;
        }
    }
}
#[doc = "VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_SET {
    #[doc = "VBUS comparator threshold"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal local override"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to the USB subsystem"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable local ID pin status override"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID pin status local override value"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external ID pin status override using value supplied from outside the PHY"]
    pub mod EXT_ID_OVERRIDE_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external VBUS override using value supplied from outside the PHY"]
    pub mod EXT_VBUS_OVERRIDE_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_B {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Individual enable controls for the VBUS detection comparators"]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_CLR {
    #[doc = "VBUS comparator threshold"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal local override"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to the USB subsystem"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable local ID pin status override"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID pin status local override value"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external ID pin status override using value supplied from outside the PHY"]
    pub mod EXT_ID_OVERRIDE_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external VBUS override using value supplied from outside the PHY"]
    pub mod EXT_VBUS_OVERRIDE_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_B {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Individual enable controls for the VBUS detection comparators"]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Register"]
pub mod USB1_VBUS_DETECT_TOG {
    #[doc = "VBUS comparator threshold"]
    pub mod VBUSVALID_THRESH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS detect signal local override"]
    pub mod VBUS_OVERRIDE_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for SESSEND"]
    pub mod SESSEND_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for B-Device Session Valid"]
    pub mod BVALID_OVERRIDE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for A-Device Session Valid"]
    pub mod AVALID_OVERRIDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override value for VBUS_VALID signal sent to the USB subsystem"]
    pub mod VBUSVALID_OVERRIDE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUSVALID_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB subsystem"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable local ID pin status override"]
    pub mod ID_OVERRIDE_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID pin status local override value"]
    pub mod ID_OVERRIDE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external ID pin status override using value supplied from outside the PHY"]
    pub mod EXT_ID_OVERRIDE_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable external VBUS override using value supplied from outside the PHY"]
    pub mod EXT_VBUS_OVERRIDE_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the comparator used for VBUS_VALID"]
    pub mod VBUSVALID_TO_B {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Individual enable controls for the VBUS detection comparators"]
    pub mod VBUSVALID_PWRUP_CMPS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controls VBUS discharge resistor"]
    pub mod DISCHARGE_VBUS {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Status Register"]
pub mod USB1_VBUS_DET_STAT {
    #[doc = "Session End indicator"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const SESSEND_LO: u32 = 0;
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const SESSEND_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B-Device Session Valid status"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const BVALID_LO: u32 = 0;
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const BVALID_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A-Device Session Valid status"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The VBUS voltage is below the Session Valid threshold"]
            pub const AVALID_LO: u32 = 0;
            #[doc = "The VBUS voltage is above the Session Valid threshold"]
            pub const AVALID_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS voltage status"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "VBUS is below the comparator threshold"]
            pub const VBUS_LO: u32 = 0;
            #[doc = "VBUS is above the comparator threshold"]
            pub const VBUS_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS_VALID_3V detector status"]
    pub mod VBUS_VALID_3V {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "VBUS voltage is below the VBUS_VALID_3V threshold"]
            pub const VBUS_VLD3V_LO: u32 = 0;
            #[doc = "VBUS voltage is above the VBUS_VALID_3V threshold"]
            pub const VBUS_VLD3V_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID external override status"]
    pub mod EXT_ID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Status Register"]
pub mod USB1_VBUS_DET_STAT_SET {
    #[doc = "Session End indicator"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B-Device Session Valid status"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A-Device Session Valid status"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS voltage status"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS_VALID_3V detector status"]
    pub mod VBUS_VALID_3V {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID external override status"]
    pub mod EXT_ID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Status Register"]
pub mod USB1_VBUS_DET_STAT_CLR {
    #[doc = "Session End indicator"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B-Device Session Valid status"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A-Device Session Valid status"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS voltage status"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS_VALID_3V detector status"]
    pub mod VBUS_VALID_3V {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID external override status"]
    pub mod EXT_ID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VBUS Detect Status Register"]
pub mod USB1_VBUS_DET_STAT_TOG {
    #[doc = "Session End indicator"]
    pub mod SESSEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "B-Device Session Valid status"]
    pub mod BVALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "A-Device Session Valid status"]
    pub mod AVALID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS voltage status"]
    pub mod VBUS_VALID {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBUS_VALID_3V detector status"]
    pub mod VBUS_VALID_3V {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OTG ID external override status"]
    pub mod EXT_ID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Register"]
pub mod USB1_CHRG_DETECT {
    #[doc = "Enables BC v1.2 Secondary Detection function for Charger Detect depending on other bit fields (active high)"]
    pub mod DETECT_SEC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables BC Secondary Detection function when controlled by this register"]
            pub const BC_SECDET_DISABLE: u32 = 0;
            #[doc = "Enables BC Secondary Detection function when controlled by this register"]
            pub const BC_SECDET_ENABLE: u32 = 1;
        }
    }
    #[doc = "DP pullup resistor enable override control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DP pullup resistor controlled only with UTMI bus signals"]
            pub const DP_PUE_NORMAL: u32 = 0;
            #[doc = "Force DP pullup resistor to be enabled"]
            pub const DP_PUE_OVERRIDE: u32 = 1;
        }
    }
    #[doc = "Enables BC v1.2 Charging Downstream Port (CDP) advertisement signaling for Charger Detect depending on other bit fields (active high)."]
    pub mod VDM_SRC_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables CDP advertisement signaling VDM_SRC function when controlled by this register."]
            pub const DCD_VDM_SRC_DISABLE: u32 = 0;
            #[doc = "Enables CDP advertisement signaling VDM_SRC function when controlled by this register."]
            pub const DCD_VDM_SRC_ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables BC v1.2 Data Contact Detect function for Charger Detect depending on other bit fields (active high)"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables BC Data Contact Detect function when controlled by this register"]
            pub const BC_DCD_DISABLE: u32 = 0;
            #[doc = "Enables BC Data Contact Detect function when controlled by this register"]
            pub const BC_DCD_ENABLE: u32 = 1;
        }
    }
    #[doc = "Enables BC v1.2 Primary Detection function for Charger Detect depending on other bit fields (active low)"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables BC Charger Detection function when controlled by this register"]
            pub const BC_CHRGDET_ENABLE: u32 = 0;
            #[doc = "Disables BC Charger Detection function when controlled by this register"]
            pub const BC_CHRGDET_DISABLE: u32 = 1;
        }
    }
    #[doc = "Enables selection of BC v1.2 functions depending on other bit fields (active low)"]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables BC v1.2 functions when controlled by this register"]
            pub const BC_ENABLE: u32 = 0;
            #[doc = "Disables BC v1.2 functions when controlled by this register"]
            pub const BC_DISABLE: u32 = 1;
        }
    }
    #[doc = "Selects control source for Battery Charging Detection/Advertisement circuits"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit fields in USB1_CHRG_DETECT control BC 1.2 functionality"]
            pub const CHRGDET_CTRL: u32 = 0;
            #[doc = "Bit fields and state machines in USBHSDCD module control BC 1.2 functionality"]
            pub const USBHSDCD_CTRL: u32 = 1;
        }
    }
}
#[doc = "Charger Detect Register"]
pub mod USB1_CHRG_DETECT_SET {
    #[doc = "Enables BC v1.2 Secondary Detection function for Charger Detect depending on other bit fields (active high)"]
    pub mod DETECT_SEC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP pullup resistor enable override control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Charging Downstream Port (CDP) advertisement signaling for Charger Detect depending on other bit fields (active high)."]
    pub mod VDM_SRC_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Data Contact Detect function for Charger Detect depending on other bit fields (active high)"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Primary Detection function for Charger Detect depending on other bit fields (active low)"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables selection of BC v1.2 functions depending on other bit fields (active low)"]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects control source for Battery Charging Detection/Advertisement circuits"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Register"]
pub mod USB1_CHRG_DETECT_CLR {
    #[doc = "Enables BC v1.2 Secondary Detection function for Charger Detect depending on other bit fields (active high)"]
    pub mod DETECT_SEC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP pullup resistor enable override control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Charging Downstream Port (CDP) advertisement signaling for Charger Detect depending on other bit fields (active high)."]
    pub mod VDM_SRC_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Data Contact Detect function for Charger Detect depending on other bit fields (active high)"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Primary Detection function for Charger Detect depending on other bit fields (active low)"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables selection of BC v1.2 functions depending on other bit fields (active low)"]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects control source for Battery Charging Detection/Advertisement circuits"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Register"]
pub mod USB1_CHRG_DETECT_TOG {
    #[doc = "Enables BC v1.2 Secondary Detection function for Charger Detect depending on other bit fields (active high)"]
    pub mod DETECT_SEC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP pullup resistor enable override control"]
    pub mod PULLUP_DP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Charging Downstream Port (CDP) advertisement signaling for Charger Detect depending on other bit fields (active high)."]
    pub mod VDM_SRC_ENABLE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Data Contact Detect function for Charger Detect depending on other bit fields (active high)"]
    pub mod CHK_CONTACT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables BC v1.2 Primary Detection function for Charger Detect depending on other bit fields (active low)"]
    pub mod CHK_CHRG_B {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enables selection of BC v1.2 functions depending on other bit fields (active low)"]
    pub mod EN_B {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects control source for Battery Charging Detection/Advertisement circuits"]
    pub mod DCDSEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Status Register"]
pub mod USB1_CHRG_DET_STAT {
    #[doc = "Battery Charging Data Contact Detection phase output"]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No USB cable attachment has been detected"]
            pub const NO_DC_DETECTED: u32 = 0;
            #[doc = "A USB cable attachment between the device and host has been detected"]
            pub const DC_DETECED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Primary Detection phase output"]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Standard Downstream Port (SDP) has been detected"]
            pub const SDP_DETECT: u32 = 0;
            #[doc = "Charging Port has been detected"]
            pub const CHRG_PORT_DETECT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM voltage"]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "USB_DM pin voltage is <= 0.8V"]
            pub const DM_SERX_LO: u32 = 0;
            #[doc = "USB_DM pin voltage is >= 2.0V"]
            pub const DM_SERX_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP voltage"]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "USB_DP pin voltage is <= 0.8V"]
            pub const DP_SERX_LO: u32 = 0;
            #[doc = "USB_DP pin voltage is >= 2.0V"]
            pub const DP_SERX_HI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Secondary Detection phase output"]
    pub mod SECDET_DCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Charging Downstream Port (CDP) has been detected"]
            pub const SECDET_CDP: u32 = 0;
            #[doc = "Downstream Charging Port (DCP) has been detected"]
            pub const SECDET_DCP: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Status Register"]
pub mod USB1_CHRG_DET_STAT_SET {
    #[doc = "Battery Charging Data Contact Detection phase output"]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Primary Detection phase output"]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM voltage"]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP voltage"]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Secondary Detection phase output"]
    pub mod SECDET_DCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Status Register"]
pub mod USB1_CHRG_DET_STAT_CLR {
    #[doc = "Battery Charging Data Contact Detection phase output"]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Primary Detection phase output"]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM voltage"]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP voltage"]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Secondary Detection phase output"]
    pub mod SECDET_DCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Charger Detect Status Register"]
pub mod USB1_CHRG_DET_STAT_TOG {
    #[doc = "Battery Charging Data Contact Detection phase output"]
    pub mod PLUG_CONTACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Primary Detection phase output"]
    pub mod CHRG_DETECTED {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM voltage"]
    pub mod DM_STATE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP voltage"]
    pub mod DP_STATE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Battery Charging Secondary Detection phase output"]
    pub mod SECDET_DCP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register"]
pub mod ANACTRL {
    #[doc = "LVI enable for USB 3.3 V monitor circuit"]
    pub mod LVI_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the internal low voltage detector for the USB 3.3 V and 1.8 V domain"]
            pub const LVI_3V_DISABLE: u32 = 0;
            #[doc = "Enable the internal low voltage detector for the USB 3.3 V and 1.8 V domain"]
            pub const LVI_3V_ENABLE: u32 = 1;
        }
    }
    #[doc = "PFD clock output mux control"]
    pub mod PFD_CLK_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB1PFDCLK is the same frequency as the USB PLL reference clock"]
            pub const PFD_CLK_BYPASS: u32 = 0;
            #[doc = "USB1PFDCLK frequency is pfd_clk divided by 4"]
            pub const PFD_CLK_DIV_4: u32 = 1;
            #[doc = "USB1PFDCLK frequency is pfd_clk divided by 2"]
            pub const PFD_CLK_DIV_2: u32 = 2;
            #[doc = "USB1PFDCLK is the same as the pfd_clk frequency"]
            pub const PFD_CLK_DIV_1: u32 = 3;
        }
    }
    #[doc = "Device pulldown enable"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The 15 kohm nominal pulldowns on the USB_DP and USB_DM pins are disabled for Device mode"]
            pub const DEV_PULLDOWN_DIS: u32 = 0;
            #[doc = "The 15 kohm nominal pulldowns on the USB_DP and USB_DM pins are enabled for Device mode"]
            pub const DEV_PULLDOWN_EN: u32 = 1;
        }
    }
}
#[doc = "Analog Control Register"]
pub mod ANACTRL_SET {
    #[doc = "LVI enable for USB 3.3 V monitor circuit"]
    pub mod LVI_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD clock output mux control"]
    pub mod PFD_CLK_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device pulldown enable"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register"]
pub mod ANACTRL_CLR {
    #[doc = "LVI enable for USB 3.3 V monitor circuit"]
    pub mod LVI_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD clock output mux control"]
    pub mod PFD_CLK_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device pulldown enable"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Analog Control Register"]
pub mod ANACTRL_TOG {
    #[doc = "LVI enable for USB 3.3 V monitor circuit"]
    pub mod LVI_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD clock output mux control"]
    pub mod PFD_CLK_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device pulldown enable"]
    pub mod DEV_PULLDOWN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trim Register"]
pub mod TRIM_OVERRIDE_EN {
    #[doc = "Override enable for PLL divider value"]
    pub mod DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the TRIM_OVERRIDE_EN register values for the PLL divider value"]
            pub const USE_TRIM0_PLLDIV: u32 = 0;
            #[doc = "Use the PLL_SIC register values for the PLL divider value"]
            pub const USE_PLL_SIC_PLLDIV: u32 = 1;
        }
    }
    #[doc = "Override enable for HS Tx output current trim"]
    pub mod TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the TRIM_OVERRIDE_EN register values for HS Tx output current trim"]
            pub const USE_TRIM0_DCAL: u32 = 0;
            #[doc = "Use the Tx register values for HS Tx output current trim"]
            pub const USE_TX_DCAL: u32 = 1;
        }
    }
    #[doc = "Override enable for USB_DP series termination trim"]
    pub mod TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the TRIM_OVERRIDE_EN register values for DP resistance trim"]
            pub const USE_TRIM0_CAL45DP: u32 = 0;
            #[doc = "Use the Tx register values for DP resistance trim"]
            pub const USE_TX_CAL45DP: u32 = 1;
        }
    }
    #[doc = "Override enable for USB_DM series termination trim"]
    pub mod TX_CAL45DM_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the TRIM_OVERRIDE_EN register values for DM resistance trim"]
            pub const USE_TRIM0_CAL45DN: u32 = 0;
            #[doc = "Use the Tx register values for DM resistance trim"]
            pub const USE_TX_CAL45DN: u32 = 1;
        }
    }
    #[doc = "PLL divider value configuration bits from outside USBPHY"]
    pub mod PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS Tx output current trim bits from outside USBPHY"]
    pub mod USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trim Register"]
pub mod TRIM_OVERRIDE_EN_SET {
    #[doc = "Override enable for PLL divider value"]
    pub mod DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for HS Tx output current trim"]
    pub mod TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DP series termination trim"]
    pub mod TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DM series termination trim"]
    pub mod TX_CAL45DM_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration bits from outside USBPHY"]
    pub mod PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS Tx output current trim bits from outside USBPHY"]
    pub mod USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trim Register"]
pub mod TRIM_OVERRIDE_EN_CLR {
    #[doc = "Override enable for PLL divider value"]
    pub mod DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for HS Tx output current trim"]
    pub mod TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DP series termination trim"]
    pub mod TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DM series termination trim"]
    pub mod TX_CAL45DM_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration bits from outside USBPHY"]
    pub mod PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS Tx output current trim bits from outside USBPHY"]
    pub mod USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trim Register"]
pub mod TRIM_OVERRIDE_EN_TOG {
    #[doc = "Override enable for PLL divider value"]
    pub mod DIV_SEL_OVERRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for HS Tx output current trim"]
    pub mod TX_D_CAL_OVERRIDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DP series termination trim"]
    pub mod TX_CAL45DP_OVERRIDE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override enable for USB_DM series termination trim"]
    pub mod TX_CAL45DM_OVERRIDE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL divider value configuration bits from outside USBPHY"]
    pub mod PLL_CTRL0_DIV_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HS Tx output current trim bits from outside USBPHY"]
    pub mod USBPHY_TX_D_CAL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DM series termination resistance trim bits from outside USBPHY."]
    pub mod USBPHY_TX_CAL45DN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PFD Register A"]
pub mod PFDA {
    #[doc = "PFD0 clock gate"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PFD0 clock output is enabled"]
            pub const PFD0_CLK_EN: u32 = 0;
            #[doc = "PFD0 clock output is gated (Default)"]
            pub const PFD0_CLK_DIS: u32 = 1;
        }
    }
    #[doc = "PFD0 fractional divider setting used to select the pfd_clk output frequency"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 stable signal from the Phase Fractional Divider"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PFD Register A"]
pub mod PFDA_SET {
    #[doc = "PFD0 clock gate"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 fractional divider setting used to select the pfd_clk output frequency"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 stable signal from the Phase Fractional Divider"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PFD Register A"]
pub mod PFDA_CLR {
    #[doc = "PFD0 clock gate"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 fractional divider setting used to select the pfd_clk output frequency"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 stable signal from the Phase Fractional Divider"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PFD Register A"]
pub mod PFDA_TOG {
    #[doc = "PFD0 clock gate"]
    pub mod PFD0_CLKGATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 fractional divider setting used to select the pfd_clk output frequency"]
    pub mod PFD0_FRAC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PFD0 stable signal from the Phase Fractional Divider"]
    pub mod PFD0_STABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
