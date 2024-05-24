#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "uSDHC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "DMA System Address"]
    pub DS_ADDR: crate::RWRegister<u32>,
    #[doc = "Block Attributes"]
    pub BLK_ATT: crate::RWRegister<u32>,
    #[doc = "Command Argument"]
    pub CMD_ARG: crate::RWRegister<u32>,
    #[doc = "Command Transfer Type"]
    pub CMD_XFR_TYP: crate::RWRegister<u32>,
    #[doc = "Command Response0"]
    pub CMD_RSP0: crate::RWRegister<u32>,
    #[doc = "Command Response1"]
    pub CMD_RSP1: crate::RWRegister<u32>,
    #[doc = "Command Response2"]
    pub CMD_RSP2: crate::RWRegister<u32>,
    #[doc = "Command Response3"]
    pub CMD_RSP3: crate::RWRegister<u32>,
    #[doc = "Data Buffer Access Port"]
    pub DATA_BUFF_ACC_PORT: crate::RWRegister<u32>,
    #[doc = "Present State"]
    pub PRES_STATE: crate::RWRegister<u32>,
    #[doc = "Protocol Control"]
    pub PROT_CTRL: crate::RWRegister<u32>,
    #[doc = "System Control"]
    pub SYS_CTRL: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Enable"]
    pub INT_STATUS_EN: crate::RWRegister<u32>,
    #[doc = "Interrupt Signal Enable"]
    pub INT_SIGNAL_EN: crate::RWRegister<u32>,
    #[doc = "Auto CMD12 Error Status"]
    pub AUTOCMD12_ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "Host Controller Capabilities"]
    pub HOST_CTRL_CAP: crate::RWRegister<u32>,
    #[doc = "Watermark Level"]
    pub WTMK_LVL: crate::RWRegister<u32>,
    #[doc = "Mixer Control"]
    pub MIX_CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Force Event"]
    pub FORCE_EVENT: crate::RWRegister<u32>,
    #[doc = "ADMA Error Status"]
    pub ADMA_ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "ADMA System Address"]
    pub ADMA_SYS_ADDR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "DLL (Delay Line) Control"]
    pub DLL_CTRL: crate::RWRegister<u32>,
    #[doc = "DLL Status"]
    pub DLL_STATUS: crate::RWRegister<u32>,
    #[doc = "CLK Tuning Control and Status"]
    pub CLK_TUNE_CTRL_STATUS: crate::RWRegister<u32>,
    _reserved2: [u8; 0x54],
    #[doc = "Vendor Specific Register"]
    pub VEND_SPEC: crate::RWRegister<u32>,
    #[doc = "eMMC Boot"]
    pub MMC_BOOT: crate::RWRegister<u32>,
    #[doc = "Vendor Specific 2 Register"]
    pub VEND_SPEC2: crate::RWRegister<u32>,
    #[doc = "Tuning Control"]
    pub TUNING_CTRL: crate::RWRegister<u32>,
}
#[doc = "DMA System Address"]
pub mod DS_ADDR {
    #[doc = "System address"]
    pub mod DS_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Attributes"]
pub mod BLK_ATT {
    #[doc = "Transfer block size"]
    pub mod BLKSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No data transfer"]
            pub const BLK_ATT_I: u32 = 0;
            #[doc = "1 byte"]
            pub const BLK_ATT_H: u32 = 1;
            #[doc = "2 bytes"]
            pub const BLK_ATT_G: u32 = 2;
            #[doc = "3 bytes"]
            pub const BLK_ATT_F: u32 = 3;
            #[doc = "4 bytes"]
            pub const BLK_ATT_E: u32 = 4;
            #[doc = "511 bytes"]
            pub const BLK_ATT_D: u32 = 511;
            #[doc = "512 bytes"]
            pub const BLK_ATT_C: u32 = 512;
            #[doc = "2048 bytes"]
            pub const BLK_ATT_B: u32 = 2048;
            #[doc = "4096 bytes"]
            pub const BLK_ATT_A: u32 = 4096;
        }
    }
    #[doc = "Blocks count for current transfer"]
    pub mod BLKCNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop count"]
            pub const BLKCNT_D: u32 = 0;
            #[doc = "1 block"]
            pub const BLKCNT_C: u32 = 1;
            #[doc = "2 blocks"]
            pub const BLKCNT_B: u32 = 2;
            #[doc = "65535 blocks"]
            pub const BLKCNT_A: u32 = 65535;
        }
    }
}
#[doc = "Command Argument"]
pub mod CMD_ARG {
    #[doc = "Command argument"]
    pub mod CMDARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Transfer Type"]
pub mod CMD_XFR_TYP {
    #[doc = "DMAEN"]
    pub mod DMAEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP_0B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP_0A: u32 = 1;
        }
    }
    #[doc = "BCEN"]
    pub mod BCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP1_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP1_A: u32 = 1;
        }
    }
    #[doc = "AC12EN"]
    pub mod AC12EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP2_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP2_A: u32 = 1;
        }
    }
    #[doc = "DDR_EN"]
    pub mod DDR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP3_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP3_A: u32 = 1;
        }
    }
    #[doc = "DTDSEL"]
    pub mod DTDSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP4_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP4_A: u32 = 1;
        }
    }
    #[doc = "MSBSEL"]
    pub mod MSBSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP5_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP5_A: u32 = 1;
        }
    }
    #[doc = "NIBBLE_POS"]
    pub mod NIBBLE_POS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP6_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP6_A: u32 = 1;
        }
    }
    #[doc = "AC23EN"]
    pub mod AC23EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CMD_XFR_TYP7_B: u32 = 0;
            #[doc = "Enable"]
            pub const CMD_XFR_TYP7_A: u32 = 1;
        }
    }
    #[doc = "Response type select"]
    pub mod RSPTYP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No response"]
            pub const RSPTYP_A: u32 = 0;
            #[doc = "Response length 136"]
            pub const RSPTYP_B: u32 = 1;
            #[doc = "Response length 48"]
            pub const RSPTYP_C: u32 = 2;
            #[doc = "Response length 48, check busy after response"]
            pub const RSPTYP_D: u32 = 3;
        }
    }
    #[doc = "Command CRC check enable"]
    pub mod CCCEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables command CRC check"]
            pub const CCCEN_B: u32 = 0;
            #[doc = "Enables command CRC check"]
            pub const CCCEN_A: u32 = 1;
        }
    }
    #[doc = "Command index check enable"]
    pub mod CICEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable command index check"]
            pub const CICEN_B: u32 = 0;
            #[doc = "Enables command index check"]
            pub const CICEN_A: u32 = 1;
        }
    }
    #[doc = "Data present select"]
    pub mod DPSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No data present"]
            pub const DPSEL_B: u32 = 0;
            #[doc = "Data present"]
            pub const DPSEL_A: u32 = 1;
        }
    }
    #[doc = "Command type"]
    pub mod CMDTYP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal other commands"]
            pub const CMDTYP_D: u32 = 0;
            #[doc = "Suspend CMD52 for writing bus suspend in CCCR"]
            pub const CMDTYP_C: u32 = 1;
            #[doc = "Resume CMD52 for writing function select in CCCR"]
            pub const CMDTYP_B: u32 = 2;
            #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
            pub const CMDTYP_A: u32 = 3;
        }
    }
    #[doc = "Command index"]
    pub mod CMDINX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response0"]
pub mod CMD_RSP0 {
    #[doc = "Command response 0"]
    pub mod CMDRSP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response1"]
pub mod CMD_RSP1 {
    #[doc = "Command response 1"]
    pub mod CMDRSP1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response2"]
pub mod CMD_RSP2 {
    #[doc = "Command response 2"]
    pub mod CMDRSP2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Response3"]
pub mod CMD_RSP3 {
    #[doc = "Command response 3"]
    pub mod CMDRSP3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Buffer Access Port"]
pub mod DATA_BUFF_ACC_PORT {
    #[doc = "Data content"]
    pub mod DATCONT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Present State"]
pub mod PRES_STATE {
    #[doc = "Command inhibit (CMD)"]
    pub mod CIHB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Can issue command using only CMD line"]
            pub const CIHB_A: u32 = 0;
            #[doc = "Cannot issue command"]
            pub const CIHB_B: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Inhibit Data (DATA)"]
    pub mod CDIHB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Can issue command that uses the DATA line"]
            pub const CDIHB_B: u32 = 0;
            #[doc = "Cannot issue command that uses the DATA line"]
            pub const CDIHB_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data line active"]
    pub mod DLA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DATA line inactive"]
            pub const DLA_A: u32 = 0;
            #[doc = "DATA line active"]
            pub const DLA_B: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SD clock stable"]
    pub mod SDSTB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clock is changing frequency and not stable."]
            pub const SDSTB_B: u32 = 0;
            #[doc = "Clock is stable."]
            pub const SDSTB_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write transfer active"]
    pub mod WTA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No valid data"]
            pub const WTA_B: u32 = 0;
            #[doc = "Transferring data"]
            pub const WTA_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read transfer active"]
    pub mod RTA {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No valid data"]
            pub const RTA_B: u32 = 0;
            #[doc = "Transferring data"]
            pub const RTA_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer write enable"]
    pub mod BWEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Write disable"]
            pub const BWEN_B: u32 = 0;
            #[doc = "Write enable"]
            pub const BWEN_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer read enable"]
    pub mod BREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Read disable"]
            pub const BREN_B: u32 = 0;
            #[doc = "Read enable"]
            pub const BREN_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode)"]
    pub mod RTR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Fixed or well tuned sampling clock"]
            pub const RTR_B: u32 = 0;
            #[doc = "Sampling clock needs re-tuning"]
            pub const RTR_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tap select change done"]
    pub mod TSCD {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Delay cell select change is not finished."]
            pub const TSCD_B: u32 = 0;
            #[doc = "Delay cell select change is finished."]
            pub const TSCD_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card inserted"]
    pub mod CINST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Power on reset or no card"]
            pub const CINST_A: u32 = 0;
            #[doc = "Card inserted"]
            pub const CINST_B: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMD line signal level"]
    pub mod CLSL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DATA[7:0] line signal level"]
    pub mod DLSL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "Data 0 line signal level"]
            pub const DATA0: u32 = 0;
            #[doc = "Data 1 line signal level"]
            pub const DATA1: u32 = 1;
            #[doc = "Data 2 line signal level"]
            pub const DATA2: u32 = 2;
            #[doc = "Data 3 line signal level"]
            pub const DATA3: u32 = 3;
            #[doc = "Data 4 line signal level"]
            pub const DATA4: u32 = 4;
            #[doc = "Data 5 line signal level"]
            pub const DATA5: u32 = 5;
            #[doc = "Data 6 line signal level"]
            pub const DATA6: u32 = 6;
            #[doc = "Data 7 line signal level"]
            pub const DATA7: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Protocol Control"]
pub mod PROT_CTRL {
    #[doc = "Data transfer width"]
    pub mod DTW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1-bit mode"]
            pub const DTW_C: u32 = 0;
            #[doc = "4-bit mode"]
            pub const DTW_B: u32 = 1;
            #[doc = "8-bit mode"]
            pub const DTW_A: u32 = 2;
        }
    }
    #[doc = "DATA3 as card detection pin"]
    pub mod D3CD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DATA3 does not monitor card insertion"]
            pub const D3CD_B: u32 = 0;
            #[doc = "DATA3 as card detection pin"]
            pub const D3CD_A: u32 = 1;
        }
    }
    #[doc = "Endian mode"]
    pub mod EMODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Big endian mode"]
            pub const EMODE_A: u32 = 0;
            #[doc = "Half word big endian mode"]
            pub const EMODE_B: u32 = 1;
            #[doc = "Little endian mode"]
            pub const EMODE_C: u32 = 2;
        }
    }
    #[doc = "DMA select"]
    pub mod DMASEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA or simple DMA is selected."]
            pub const DMASEL_A: u32 = 0;
            #[doc = "ADMA1 is selected."]
            pub const DMASEL_B: u32 = 1;
            #[doc = "ADMA2 is selected."]
            pub const DMASEL_C: u32 = 2;
        }
    }
    #[doc = "Stop at block gap request"]
    pub mod SABGREQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer"]
            pub const SABGREQ_B: u32 = 0;
            #[doc = "Stop"]
            pub const SABGREQ_A: u32 = 1;
        }
    }
    #[doc = "Continue request"]
    pub mod CREQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const CREQ_B: u32 = 0;
            #[doc = "Restart"]
            pub const CREQ_A: u32 = 1;
        }
    }
    #[doc = "Read wait control"]
    pub mod RWCTL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables read wait control and stop SD clock at block gap when SABGREQ field is set"]
            pub const RWCTL_B: u32 = 0;
            #[doc = "Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set"]
            pub const RWCTL_A: u32 = 1;
        }
    }
    #[doc = "Interrupt at block gap"]
    pub mod IABG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt at block gap"]
            pub const IABG_B: u32 = 0;
            #[doc = "Enables interrupt at block gap"]
            pub const IABG_A: u32 = 1;
        }
    }
    #[doc = "Read performed number 8 clock"]
    pub mod RD_DONE_NO_8CLK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup event enable on card interrupt"]
    pub mod WECINT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables wakeup event enable on card interrupt"]
            pub const WECINT_A: u32 = 0;
            #[doc = "Enables wakeup event enable on card interrupt"]
            pub const WECINT_B: u32 = 1;
        }
    }
    #[doc = "Wakeup event enable on SD card insertion"]
    pub mod WECINS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable wakeup event enable on SD card insertion"]
            pub const WECINS_B: u32 = 0;
            #[doc = "Enable wakeup event enable on SD card insertion"]
            pub const WECINS_A: u32 = 1;
        }
    }
    #[doc = "Wakeup event enable on SD card removal"]
    pub mod WECRM {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables wakeup event enable on SD card removal"]
            pub const WECRM_B: u32 = 0;
            #[doc = "Enables wakeup event enable on SD card removal"]
            pub const WECRM_A: u32 = 1;
        }
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    pub mod BURST_LEN_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Burst length is enabled for INCR."]
            pub const BURST_A: u32 = 1;
        }
    }
    #[doc = "Non-exact block read"]
    pub mod NON_EXACT_BLK_RD {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read."]
            pub const EXACT_B: u32 = 0;
            #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
            pub const EXACT_A: u32 = 1;
        }
    }
}
#[doc = "System Control"]
pub mod SYS_CTRL {
    #[doc = "Divisor"]
    pub mod DVS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide-by-1"]
            pub const DVS_A: u32 = 0;
            #[doc = "Divide-by-2"]
            pub const DVS_B: u32 = 1;
            #[doc = "Divide-by-15"]
            pub const DVS_C: u32 = 14;
            #[doc = "Divide-by-16"]
            pub const DVS_D: u32 = 15;
        }
    }
    #[doc = "SDCLK frequency select"]
    pub mod SDCLKFS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data timeout counter value"]
    pub mod DTOCV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDCLK x 2 32"]
            pub const DTOCV_X: u32 = 0;
            #[doc = "SDCLK x 2 33"]
            pub const DTOCV_W: u32 = 1;
            #[doc = "SDCLK x 2 18"]
            pub const DTOCV_V: u32 = 2;
            #[doc = "SDCLK x 2 19"]
            pub const DTOCV_U: u32 = 3;
            #[doc = "SDCLK x 2 29 recommend to use for other speed mode except SDR104 mode"]
            pub const DTOCV_T: u32 = 13;
            #[doc = "SDCLK x 2 30 recommend to use for SDR104 mode"]
            pub const DTOCV_S: u32 = 14;
        }
    }
    #[doc = "Hardware reset"]
    pub mod IPP_RST_N {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset for all"]
    pub mod RSTA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset"]
            pub const RSTA_B: u32 = 0;
            #[doc = "Reset"]
            pub const RSTA_A: u32 = 1;
        }
    }
    #[doc = "Software reset for CMD line"]
    pub mod RSTC {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset"]
            pub const RSTC_B: u32 = 0;
            #[doc = "Reset"]
            pub const RSTC_A: u32 = 1;
        }
    }
    #[doc = "Software reset for data line"]
    pub mod RSTD {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset"]
            pub const RSTD_B: u32 = 0;
            #[doc = "Reset"]
            pub const RSTD_A: u32 = 1;
        }
    }
    #[doc = "Initialization active"]
    pub mod INITA {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset tuning"]
    pub mod RSTT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod INT_STATUS {
    #[doc = "Command complete"]
    pub mod CC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command not complete"]
            pub const CC_B: u32 = 0;
            #[doc = "Command complete"]
            pub const CC_A: u32 = 1;
        }
    }
    #[doc = "Transfer complete"]
    pub mod TC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer does not complete"]
            pub const TC_B: u32 = 0;
            #[doc = "Transfer complete"]
            pub const TC_A: u32 = 1;
        }
    }
    #[doc = "Block gap event"]
    pub mod BGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No block gap event"]
            pub const BGE_B: u32 = 0;
            #[doc = "Transaction stopped at block gap"]
            pub const BGE_A: u32 = 1;
        }
    }
    #[doc = "DMA interrupt"]
    pub mod DINT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DMA interrupt"]
            pub const DINT_B: u32 = 0;
            #[doc = "DMA interrupt is generated."]
            pub const DINT_A: u32 = 1;
        }
    }
    #[doc = "Buffer write ready"]
    pub mod BWR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not ready to write buffer"]
            pub const BWR_B: u32 = 0;
            #[doc = "Ready to write buffer"]
            pub const BWR_A: u32 = 1;
        }
    }
    #[doc = "Buffer read ready"]
    pub mod BRR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not ready to read buffer"]
            pub const BRR_B: u32 = 0;
            #[doc = "Ready to read buffer"]
            pub const BRR_A: u32 = 1;
        }
    }
    #[doc = "Card insertion"]
    pub mod CINS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card state unstable or removed"]
            pub const BWR_B: u32 = 0;
            #[doc = "Card inserted"]
            pub const BWR_A: u32 = 1;
        }
    }
    #[doc = "Card removal"]
    pub mod CRM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Card state unstable or inserted"]
            pub const CRM_A: u32 = 0;
            #[doc = "Card removed"]
            pub const CRM_B: u32 = 1;
        }
    }
    #[doc = "Card interrupt"]
    pub mod CINT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No card interrupt"]
            pub const CINT_A: u32 = 0;
            #[doc = "Generate card interrupt"]
            pub const CINT_B: u32 = 1;
        }
    }
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode)"]
    pub mod RTE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Re-tuning is not required."]
            pub const RTE_A: u32 = 0;
            #[doc = "Re-tuning should be performed."]
            pub const RTE_B: u32 = 1;
        }
    }
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode)"]
    pub mod TP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interrupt Status"]
    pub mod ERR_INT_STATUS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command timeout error"]
    pub mod CTOE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CTOE_A: u32 = 0;
            #[doc = "Time out"]
            pub const CTOE_B: u32 = 1;
        }
    }
    #[doc = "Command CRC error"]
    pub mod CCE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CCE_A: u32 = 0;
            #[doc = "CRC error generated"]
            pub const CCE_B: u32 = 1;
        }
    }
    #[doc = "Command end bit error"]
    pub mod CEBE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CEBE_A: u32 = 0;
            #[doc = "End bit error generated"]
            pub const CEBE_B: u32 = 1;
        }
    }
    #[doc = "Command index error"]
    pub mod CIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const CIE_A: u32 = 0;
            #[doc = "Error"]
            pub const CIE_B: u32 = 1;
        }
    }
    #[doc = "Data timeout error"]
    pub mod DTOE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DTOE_A: u32 = 0;
            #[doc = "Time out"]
            pub const DTOE_B: u32 = 1;
        }
    }
    #[doc = "Data CRC error"]
    pub mod DCE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DCE_A: u32 = 0;
            #[doc = "Error"]
            pub const DCE_B: u32 = 1;
        }
    }
    #[doc = "Data end bit error"]
    pub mod DEBE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DEBE_A: u32 = 0;
            #[doc = "Error"]
            pub const DEBE_B: u32 = 1;
        }
    }
    #[doc = "Auto CMD12 error"]
    pub mod AC12E {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const AC12E_A: u32 = 0;
            #[doc = "Error"]
            pub const AC12E_B: u32 = 1;
        }
    }
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode)"]
    pub mod TNE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA error"]
    pub mod DMAE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DMAE_A: u32 = 0;
            #[doc = "Error"]
            pub const DMAE_B: u32 = 1;
        }
    }
}
#[doc = "Interrupt Status Enable"]
pub mod INT_STATUS_EN {
    #[doc = "Command complete status enable"]
    pub mod CCSEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CCSEN_B: u32 = 1;
        }
    }
    #[doc = "Transfer complete status enable"]
    pub mod TCSEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TCSEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const TCSEN_A: u32 = 1;
        }
    }
    #[doc = "Block gap event status enable"]
    pub mod BGESEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BGESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const BGESEN_B: u32 = 1;
        }
    }
    #[doc = "DMA interrupt status enable"]
    pub mod DINTSEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DINTSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const DINTSEN_B: u32 = 1;
        }
    }
    #[doc = "Buffer write ready status enable"]
    pub mod BWRSEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BWRSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const BWRSEN_B: u32 = 1;
        }
    }
    #[doc = "Buffer read ready status enable"]
    pub mod BRRSEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BRRSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const BRREN_B: u32 = 1;
        }
    }
    #[doc = "Card insertion status enable"]
    pub mod CINSSEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CINSEN_B: u32 = 1;
        }
    }
    #[doc = "Card removal status enable"]
    pub mod CRMSEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CRMSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CRMSEN_B: u32 = 1;
        }
    }
    #[doc = "Card interrupt status enable"]
    pub mod CINTSEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINTSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CINTSEN_B: u32 = 1;
        }
    }
    #[doc = "Re-tuning event status enable"]
    pub mod RTESEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const RTESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const RTESEN_B: u32 = 1;
        }
    }
    #[doc = "Tuning pass status enable"]
    pub mod TPSEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TPSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const TPSEN_B: u32 = 1;
        }
    }
    #[doc = "Command timeout error status enable"]
    pub mod CTOESEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CTOSEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CTOSEN_B: u32 = 1;
        }
    }
    #[doc = "Command CRC error status enable"]
    pub mod CCESEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CCESEN_B: u32 = 1;
        }
    }
    #[doc = "Command end bit error status enable"]
    pub mod CEBESEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CEBESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CEBESEN_B: u32 = 1;
        }
    }
    #[doc = "Command index error status enable"]
    pub mod CIESEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CIESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CIESEN_B: u32 = 1;
        }
    }
    #[doc = "Data timeout error status enable"]
    pub mod DTOESEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTOESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const DTOESEN_B: u32 = 1;
        }
    }
    #[doc = "Data CRC error status enable"]
    pub mod DCESEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DCESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const DCESEN_B: u32 = 1;
        }
    }
    #[doc = "Data end bit error status enable"]
    pub mod DEBESEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DBESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const DBESEN_B: u32 = 1;
        }
    }
    #[doc = "Auto CMD12 error status enable"]
    pub mod AC12ESEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const AC12ESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const AC12ESEN_B: u32 = 1;
        }
    }
    #[doc = "Tuning error status enable"]
    pub mod TNESEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TNESEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const TNESEN_B: u32 = 1;
        }
    }
    #[doc = "DMA error status enable"]
    pub mod DMAESEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DMASEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const DMASEN_A: u32 = 1;
        }
    }
}
#[doc = "Interrupt Signal Enable"]
pub mod INT_SIGNAL_EN {
    #[doc = "Command complete interrupt enable"]
    pub mod CCIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CCIEN_B: u32 = 1;
        }
    }
    #[doc = "Transfer complete interrupt enable"]
    pub mod TCIEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TCIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const TCIEN_B: u32 = 1;
        }
    }
    #[doc = "Block gap event interrupt enable"]
    pub mod BGEIEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BGIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const BGIEN_B: u32 = 1;
        }
    }
    #[doc = "DMA interrupt enable"]
    pub mod DINTIEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DINTIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const DINTIEN_A: u32 = 1;
        }
    }
    #[doc = "Buffer write ready interrupt enable"]
    pub mod BWRIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BWRIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const BWRIEN_B: u32 = 1;
        }
    }
    #[doc = "Buffer read ready interrupt enable"]
    pub mod BRRIEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const BRRIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const BRRIEN_A: u32 = 1;
        }
    }
    #[doc = "Card insertion interrupt enable"]
    pub mod CINSIEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINSIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CINSIEN_B: u32 = 1;
        }
    }
    #[doc = "Card removal interrupt enable"]
    pub mod CRMIEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CRMIEN_A: u32 = 0;
            #[doc = "Enabled"]
            pub const CRMIEN_B: u32 = 1;
        }
    }
    #[doc = "Card interrupt enable"]
    pub mod CINTIEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CINTIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const CINTIEN_A: u32 = 1;
        }
    }
    #[doc = "Re-tuning event interrupt enable"]
    pub mod RTEIEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const RTEIEN_O: u32 = 0;
            #[doc = "Enabled"]
            pub const RTEIEN_N: u32 = 1;
        }
    }
    #[doc = "Tuning Pass interrupt enable"]
    pub mod TPIEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TPIEN_R: u32 = 0;
            #[doc = "Enabled"]
            pub const TPIEN_Q: u32 = 1;
        }
    }
    #[doc = "Command timeout error interrupt enable"]
    pub mod CTOEIEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CTOEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const CTOEIEN_A: u32 = 1;
        }
    }
    #[doc = "Command CRC error interrupt enable"]
    pub mod CCEIEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CCEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const CCEIEN_A: u32 = 1;
        }
    }
    #[doc = "Command end bit error interrupt enable"]
    pub mod CEBEIEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CEBEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const CEBEIEN_A: u32 = 1;
        }
    }
    #[doc = "Command index error interrupt enable"]
    pub mod CIEIEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const CIEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const CIEIEN_A: u32 = 1;
        }
    }
    #[doc = "Data timeout error interrupt enable"]
    pub mod DTOEIEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DTOEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const DTOEIEN_A: u32 = 1;
        }
    }
    #[doc = "Data CRC error interrupt enable"]
    pub mod DCEIEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DCEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const DCEIEN_A: u32 = 1;
        }
    }
    #[doc = "Data end bit error interrupt enable"]
    pub mod DEBEIEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DEBEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const DEBEIEN_A: u32 = 1;
        }
    }
    #[doc = "Auto CMD12 error interrupt enable"]
    pub mod AC12EIEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const AC12EIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const AC12EIEN_A: u32 = 1;
        }
    }
    #[doc = "Tuning error interrupt enable"]
    pub mod TNEIEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const TNEIEN_B: u32 = 0;
            #[doc = "Enabled"]
            pub const TNEIEN_A: u32 = 1;
        }
    }
    #[doc = "DMA error interrupt enable"]
    pub mod DMAEIEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Masked"]
            pub const DMAEIEN_B: u32 = 0;
            #[doc = "Enable"]
            pub const DMAEIEN_A: u32 = 1;
        }
    }
}
#[doc = "Auto CMD12 Error Status"]
pub mod AUTOCMD12_ERR_STATUS {
    #[doc = "Auto CMD12 not executed"]
    pub mod AC12NE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Executed"]
            pub const AC12NE_B: u32 = 0;
            #[doc = "Not executed"]
            pub const AC12NE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD12 / 23 timeout error"]
    pub mod AC12TOE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const AC12TOE_B: u32 = 0;
            #[doc = "Time out"]
            pub const AC12TOE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD12 / 23 CRC error"]
    pub mod AC12CE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No CRC error"]
            pub const AC12CE_B: u32 = 0;
            #[doc = "CRC error met in Auto CMD12/23 response"]
            pub const AC12CE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD12 / 23 end bit error"]
    pub mod AC12EBE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const AC12EBE_B: u32 = 0;
            #[doc = "End bit error generated"]
            pub const AC12EBE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD12 / 23 index error"]
    pub mod AC12IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const AC12IE_B: u32 = 0;
            #[doc = "Error, the CMD index in response is not CMD12/23"]
            pub const AC12IE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command not issued by Auto CMD12 error"]
    pub mod CNIBAC12E {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const CNIBAC12E_B: u32 = 0;
            #[doc = "Not issued"]
            pub const CNIBAC12E_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Execute tuning"]
    pub mod EXECUTE_TUNING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tuning procedure is aborted"]
            pub const EX_TUN_B: u32 = 0;
            #[doc = "Start tuning procedure"]
            pub const EX_TUN_A: u32 = 1;
        }
    }
    #[doc = "Sample clock select"]
    pub mod SMP_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed clock is used to sample data"]
            pub const SMP_CLK_B: u32 = 0;
            #[doc = "Tuned clock is used to sample data"]
            pub const SMP_CLK_A: u32 = 1;
        }
    }
}
#[doc = "Host Controller Capabilities"]
pub mod HOST_CTRL_CAP {
    #[doc = "SDR50 support"]
    pub mod SDR50_SUPPORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDR104 support"]
    pub mod SDR104_SUPPORT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDR50 support"]
    pub mod DDR50_SUPPORT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use Tuning for SDR50"]
    pub mod USE_TUNING_SDR50 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDR50 does not support tuning"]
            pub const USE_TUNING_B: u32 = 0;
            #[doc = "SDR50 supports tuning"]
            pub const USE_TUNING_A: u32 = 1;
        }
    }
    #[doc = "Max block length"]
    pub mod MBL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "512 bytes"]
            pub const MBL_A: u32 = 0;
            #[doc = "1024 bytes"]
            pub const MBL_B: u32 = 1;
            #[doc = "2048 bytes"]
            pub const MBL_C: u32 = 2;
            #[doc = "4096 bytes"]
            pub const MBL_D: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA support"]
    pub mod ADMAS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Advanced DMA not supported"]
            pub const ADMAS_B: u32 = 0;
            #[doc = "Advanced DMA supported"]
            pub const ADMAS_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High speed support"]
    pub mod HSS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "High speed not supported"]
            pub const HSS_B: u32 = 0;
            #[doc = "High speed supported"]
            pub const HSS_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA support"]
    pub mod DMAS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DMA not supported"]
            pub const DMAS_B: u32 = 0;
            #[doc = "DMA supported"]
            pub const DMAS_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Suspend / resume support"]
    pub mod SRS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const SRS_B: u32 = 0;
            #[doc = "Supported"]
            pub const SRS_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage support 3.3 V"]
    pub mod VS33 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "3.3 V not supported"]
            pub const VS33_B: u32 = 0;
            #[doc = "3.3 V supported"]
            pub const VS33_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage support 3.0 V"]
    pub mod VS30 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "3.0 V not supported"]
            pub const VS30_B: u32 = 0;
            #[doc = "3.0 V supported"]
            pub const VS30_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage support 1.8 V"]
    pub mod VS18 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "1.8 V not supported"]
            pub const VS18_B: u32 = 0;
            #[doc = "1.8 V supported"]
            pub const VS18_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watermark Level"]
pub mod WTMK_LVL {
    #[doc = "Read watermark level"]
    pub mod RD_WML {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read burst length due to system restriction, the actual burst length might not exceed 16"]
    pub mod RD_BRST_LEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write watermark level"]
    pub mod WR_WML {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write burst length due to system restriction, the actual burst length might not exceed 16"]
    pub mod WR_BRST_LEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mixer Control"]
pub mod MIX_CTRL {
    #[doc = "DMA enable"]
    pub mod DMAEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DMAIN_B: u32 = 0;
            #[doc = "Enable"]
            pub const DMAIN_A: u32 = 1;
        }
    }
    #[doc = "Block count enable"]
    pub mod BCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const BCEN_B: u32 = 0;
            #[doc = "Enable"]
            pub const BCEN_A: u32 = 1;
        }
    }
    #[doc = "Auto CMD12 enable"]
    pub mod AC12EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const AC12EN_B: u32 = 0;
            #[doc = "Enable"]
            pub const AC12EN_A: u32 = 1;
        }
    }
    #[doc = "Dual data rate mode selection"]
    pub mod DDR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data transfer direction select"]
    pub mod DTDSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write (Host to card)"]
            pub const DTDSEL_B: u32 = 0;
            #[doc = "Read (Card to host)"]
            pub const DTDSEL_A: u32 = 1;
        }
    }
    #[doc = "Multi / Single block select"]
    pub mod MSBSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single block"]
            pub const MSBSEL_B: u32 = 0;
            #[doc = "Multiple blocks"]
            pub const MSBSEL_A: u32 = 1;
        }
    }
    #[doc = "Nibble position indication"]
    pub mod NIBBLE_POS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD23 enable"]
    pub mod AC23EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode)"]
    pub mod EXE_TUNE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not tuned or tuning completed"]
            pub const EXE_TUNE_D: u32 = 0;
            #[doc = "Execute tuning"]
            pub const EXE_TUNE_C: u32 = 1;
        }
    }
    #[doc = "Clock selection"]
    pub mod SMP_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed clock is used to sample data / cmd"]
            pub const SMPSEL_B: u32 = 0;
            #[doc = "Tuned clock is used to sample data / cmd"]
            pub const SMPSEL_A: u32 = 1;
        }
    }
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode)"]
    pub mod AUTO_TUNE_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable auto tuning"]
            pub const AUTOTUNE_B: u32 = 0;
            #[doc = "Enable auto tuning"]
            pub const AUTO_TUNE_A: u32 = 1;
        }
    }
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode)"]
    pub mod FBCLK_SEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback clock comes from the loopback CLK"]
            pub const FBCLK_B: u32 = 0;
            #[doc = "Feedback clock comes from the ipp_card_clk_out"]
            pub const FBCLK_A: u32 = 1;
        }
    }
}
#[doc = "Force Event"]
pub mod FORCE_EVENT {
    #[doc = "Force event auto command 12 not executed"]
    pub mod FEVTAC12NE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event auto command 12 time out error"]
    pub mod FEVTAC12TOE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event auto command 12 CRC error"]
    pub mod FEVTAC12CE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event Auto Command 12 end bit error"]
    pub mod FEVTAC12EBE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event Auto Command 12 index error"]
    pub mod FEVTAC12IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event command not executed by Auto Command 12 error"]
    pub mod FEVTCNIBAC12E {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event command time out error"]
    pub mod FEVTCTOE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event command CRC error"]
    pub mod FEVTCCE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event command end bit error"]
    pub mod FEVTCEBE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event command index error"]
    pub mod FEVTCIE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event data time out error"]
    pub mod FEVTDTOE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event data CRC error"]
    pub mod FEVTDCE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event data end bit error"]
    pub mod FEVTDEBE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event Auto Command 12 error"]
    pub mod FEVTAC12E {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force tuning error"]
    pub mod FEVTTNE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event DMA error"]
    pub mod FEVTDMAE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force event card interrupt"]
    pub mod FEVTCINT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADMA Error Status"]
pub mod ADMA_ERR_STATUS {
    #[doc = "ADMA error state (when ADMA error is occurred)"]
    pub mod ADMAES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA length mismatch error"]
    pub mod ADMALME {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const ADMAES_B: u32 = 0;
            #[doc = "Error"]
            pub const ADMAES_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA descriptor error"]
    pub mod ADMADCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const ADMADCE_B: u32 = 0;
            #[doc = "Error"]
            pub const ADMADCE_A: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADMA System Address"]
pub mod ADMA_SYS_ADDR {
    #[doc = "ADMA system address"]
    pub mod ADS_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL (Delay Line) Control"]
pub mod DLL_CTRL {
    #[doc = "DLL and delay chain"]
    pub mod DLL_CTRL_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL reset"]
    pub mod DLL_CTRL_RESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL slave delay line"]
    pub mod DLL_CTRL_SLV_FORCE_UPD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL slave delay target0"]
    pub mod DLL_CTRL_SLV_DLY_TARGET0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL gate update"]
    pub mod DLL_CTRL_GATE_UPDATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL slave override"]
    pub mod DLL_CTRL_SLV_OVERRIDE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL slave override val"]
    pub mod DLL_CTRL_SLV_OVERRIDE_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL slave delay target1"]
    pub mod DLL_CTRL_SLV_DLY_TARGET1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slave delay line update interval"]
    pub mod DLL_CTRL_SLV_UPDATE_INT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DLL control loop update interval"]
    pub mod DLL_CTRL_REF_UPDATE_INT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Status"]
pub mod DLL_STATUS {
    #[doc = "Slave delay-line lock status"]
    pub mod DLL_STS_SLV_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference DLL lock status"]
    pub mod DLL_STS_REF_LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slave delay line select status"]
    pub mod DLL_STS_SLV_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference delay line select taps"]
    pub mod DLL_STS_REF_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLK Tuning Control and Status"]
pub mod CLK_TUNE_CTRL_STATUS {
    #[doc = "Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
    pub mod DLY_CELL_SET_POST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
    pub mod DLY_CELL_SET_OUT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "delay cells on the feedback clock between the feedback clock and CLK_PRE"]
    pub mod DLY_CELL_SET_PRE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NXT error"]
    pub mod NXT_ERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay cells added on the feedback clock between CLK_OUT and CLK_POST"]
    pub mod TAP_SEL_POST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay cells added on the feedback clock between CLK_PRE and CLK_OUT"]
    pub mod TAP_SEL_OUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TAP_SEL_PRE"]
    pub mod TAP_SEL_PRE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRE error"]
    pub mod PRE_ERR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vendor Specific Register"]
pub mod VEND_SPEC {
    #[doc = "Check busy enable"]
    pub mod AC12_WR_CHKBUSY_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not check busy after auto CMD12 for write data packet"]
            pub const AC12_WR_CHKBUSY_EN_A: u32 = 0;
            #[doc = "Check busy after auto CMD12 for write data packet"]
            pub const AC12_WR_CHKBUSY_EN_B: u32 = 1;
        }
    }
    #[doc = "Force CLK"]
    pub mod FRC_SDCLK_ON {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CLK active or inactive is fully controlled by the hardware."]
            pub const FRC_SDCLK_ON_A: u32 = 0;
            #[doc = "Force CLK active"]
            pub const FRC_SDCLK_ON_B: u32 = 1;
        }
    }
    #[doc = "CRC Check Disable"]
    pub mod CRC_CHK_DIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check CRC16 for every read data packet and check CRC fields for every write data packet"]
            pub const CRC_CHK_DIS_A: u32 = 0;
            #[doc = "Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet"]
            pub const CRC_CHK_DIS_B: u32 = 1;
        }
    }
    #[doc = "Register byte access for CMD_XFR_TYP"]
    pub mod CMD_BYTE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable. MIX_CTRL[7:0] is read/write and CMD_XFR_TYP[7:0] is read-only."]
            pub const CMD_BYTE_EN_A: u32 = 0;
            #[doc = "Enable. MIX_CTRL[7:0] is read-only and CMD_XFR_TYP[7:0] is read/write."]
            pub const CMD_BYTE_EN_B: u32 = 1;
        }
    }
}
#[doc = "eMMC Boot"]
pub mod MMC_BOOT {
    #[doc = "Boot ACK time out"]
    pub mod DTOCV_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDCLK x 2^14"]
            pub const DTOCV_ACK_A: u32 = 0;
            #[doc = "SDCLK x 2^15"]
            pub const DTOCV_ACK_B: u32 = 1;
            #[doc = "SDCLK x 2^16"]
            pub const DTOCV_ACK_C: u32 = 2;
            #[doc = "SDCLK x 2^17"]
            pub const DTOCV_ACK_D: u32 = 3;
            #[doc = "SDCLK x 2^18"]
            pub const DTOCV_ACK_E: u32 = 4;
            #[doc = "SDCLK x 2^19"]
            pub const DTOCV_ACK_F: u32 = 5;
            #[doc = "SDCLK x 2^20"]
            pub const DTOCV_ACK_G: u32 = 6;
            #[doc = "SDCLK x 2^21"]
            pub const DTOCV_ACK_H: u32 = 7;
            #[doc = "SDCLK x 2^28"]
            pub const DTOCV_ACK_I: u32 = 14;
            #[doc = "SDCLK x 2^29"]
            pub const DTOCV_ACK_J: u32 = 15;
        }
    }
    #[doc = "BOOT ACK"]
    pub mod BOOT_ACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No ack"]
            pub const BOOT_ACK_A: u32 = 0;
            #[doc = "Ack"]
            pub const BOOT_ACK_B: u32 = 1;
        }
    }
    #[doc = "Boot mode"]
    pub mod BOOT_MODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal boot"]
            pub const BOOT_MODE_A: u32 = 0;
            #[doc = "Alternative boot"]
            pub const BOOT_MODE_B: u32 = 1;
        }
    }
    #[doc = "Boot enable"]
    pub mod BOOT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast boot disable"]
            pub const BOOT_EN_A: u32 = 0;
            #[doc = "Fast boot enable"]
            pub const BOOT_EN_B: u32 = 1;
        }
    }
    #[doc = "Auto stop at block gap"]
    pub mod AUTO_SABG_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time out"]
    pub mod DISABLE_TIME_OUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable time out"]
            pub const DISABLE_TIMEOUT_A: u32 = 0;
            #[doc = "Disable time out"]
            pub const DISABLE_TIMEOUT_B: u32 = 1;
        }
    }
    #[doc = "Stop At Block Gap value of automatic mode"]
    pub mod BOOT_BLK_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Vendor Specific 2 Register"]
pub mod VEND_SPEC2 {
    #[doc = "Card interrupt detection test"]
    pub mod CARD_INT_D3_TEST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Check the card interrupt only when DATA3 is high."]
            pub const CARD_INT_D3_A: u32 = 0;
            #[doc = "Check the card interrupt by ignoring the status of DATA3."]
            pub const CARD_INT_D3_B: u32 = 1;
        }
    }
    #[doc = "Tuning 8bit enable"]
    pub mod TUNING_8bit_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning 1bit enable"]
    pub mod TUNING_1bit_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning command enable"]
    pub mod TUNING_CMD_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Auto tuning circuit does not check the CMD line."]
            pub const TUNING_CMD_EN_A: u32 = 0;
            #[doc = "Auto tuning circuit checks the CMD line."]
            pub const TUNING_CMD_EN_B: u32 = 1;
        }
    }
    #[doc = "Argument2 register enable for ACMD23"]
    pub mod ACMD23_ARGU2_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const ACMD23_ARGU2_EN_B: u32 = 0;
            #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enabled."]
            pub const ACMD23_ARGU2_EN_A: u32 = 1;
        }
    }
}
#[doc = "Tuning Control"]
pub mod TUNING_CTRL {
    #[doc = "Tuning start"]
    pub mod TUNING_START_TAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable command check for standard tuning"]
    pub mod DIS_CMD_CHK_FOR_STD_TUNING {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning counter"]
    pub mod TUNING_COUNTER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TUNING_STEP"]
    pub mod TUNING_STEP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data window"]
    pub mod TUNING_WINDOW {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Standard tuning circuit and procedure enable"]
    pub mod STD_TUNING_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
