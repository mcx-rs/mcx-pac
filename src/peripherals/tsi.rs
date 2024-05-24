#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "TSI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "TSI CONFIG (TSI_CONFIG) for Self-Capacitor"]
    pub CONFIG: crate::RWRegister<u32>,
    #[doc = "TSI CONFIG (TSI_CONFIG) for Mutual-Capacitor"]
    pub CONFIG_MUTUAL: crate::RWRegister<u32>,
    #[doc = "TSI Threshold"]
    pub TSHD: crate::RWRegister<u32>,
    #[doc = "TSI General Control and Status"]
    pub GENCS: crate::RWRegister<u32>,
    #[doc = "TSI Mutual-Capacitance"]
    pub MUL: crate::RWRegister<u32>,
    #[doc = "TSI SINC Filter"]
    pub SINC: crate::RWRegister<u32>,
    #[doc = "TSI SSC 0"]
    pub SSC0: crate::RWRegister<u32>,
    #[doc = "TSI SSC 1"]
    pub SSC1: crate::RWRegister<u32>,
    #[doc = "TSI SSC 2"]
    pub SSC2: crate::RWRegister<u32>,
    #[doc = "TSI Baseline"]
    pub BASELINE: crate::RWRegister<u32>,
    #[doc = "TSI Channel Merge"]
    pub CHMERGE: crate::RWRegister<u32>,
    #[doc = "TSI Shield"]
    pub SHIELD: crate::RWRegister<u32>,
    _reserved0: [u8; 0xd0],
    #[doc = "TSI Data and Status"]
    pub DATA: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "TSI Miscellaneous"]
    pub MISC: crate::RWRegister<u32>,
    #[doc = "TSI AUTO TRIG"]
    pub TRIG: crate::RWRegister<u32>,
}
#[doc = "TSI CONFIG (TSI_CONFIG) for Self-Capacitor"]
pub mod CONFIG {
    #[doc = "Mode"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Self capacitance"]
            pub const SELF_CP: u32 = 0;
            #[doc = "Mutual capacitance"]
            pub const MTL_CP: u32 = 1;
        }
    }
    #[doc = "TSI Channel"]
    pub mod TSICH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel 0"]
            pub const SELF_CP_0: u32 = 0;
            #[doc = "Channel 1"]
            pub const SELF_CP_1: u32 = 1;
            #[doc = "Channel 2"]
            pub const SELF_CP_2: u32 = 2;
            #[doc = "Channel 3"]
            pub const SELF_CP_3: u32 = 3;
            #[doc = "Channel 4"]
            pub const SELF_CP_4: u32 = 4;
            #[doc = "Channel 5"]
            pub const SELF_CP_5: u32 = 5;
            #[doc = "Channel 6"]
            pub const SELF_CP_6: u32 = 6;
            #[doc = "Channel 7"]
            pub const SELF_CP_7: u32 = 7;
            #[doc = "Channel 8"]
            pub const SELF_CP_8: u32 = 8;
            #[doc = "Channel 9"]
            pub const SELF_CP_9: u32 = 9;
            #[doc = "Channel 10"]
            pub const SELF_CP_10: u32 = 10;
            #[doc = "Channel 11"]
            pub const SELF_CP_11: u32 = 11;
            #[doc = "Channel 12"]
            pub const SELF_CP_12: u32 = 12;
            #[doc = "Channel 13"]
            pub const SELF_CP_13: u32 = 13;
            #[doc = "Channel 14"]
            pub const SELF_CP_14: u32 = 14;
            #[doc = "Channel 15"]
            pub const SELF_CP_15: u32 = 15;
            #[doc = "Channel 16"]
            pub const SELF_CP_16: u32 = 16;
            #[doc = "Channel 17"]
            pub const SELF_CP_17: u32 = 17;
            #[doc = "Channel 18"]
            pub const SELF_CP_18: u32 = 18;
            #[doc = "Channel 19"]
            pub const SELF_CP_19: u32 = 19;
            #[doc = "Channel 20"]
            pub const SELF_CP_20: u32 = 20;
            #[doc = "Channel 21"]
            pub const SELF_CP_21: u32 = 21;
            #[doc = "Channel 22"]
            pub const SELF_CP_22: u32 = 22;
            #[doc = "Channel 23"]
            pub const SELF_CP_23: u32 = 23;
            #[doc = "Channel 24"]
            pub const SELF_CP_24: u32 = 24;
        }
    }
    #[doc = "Self-Capacitance Noise Cancelation"]
    pub mod S_NOISE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Self-Capacitance Charge Current Multiple"]
    pub mod S_XCH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 / 16"]
            pub const DEF_16: u32 = 0;
            #[doc = "1 / 8"]
            pub const DEF_8: u32 = 1;
            #[doc = "1 / 4"]
            pub const DEF_4: u32 = 2;
            #[doc = "1 / 2"]
            pub const DEF_2: u32 = 3;
        }
    }
    #[doc = "Self-Capacitance Input Current Multiple"]
    pub mod S_XIN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 / 8"]
            pub const XIN_DEF_8: u32 = 0;
            #[doc = "1 / 4"]
            pub const XIN_DEF_4: u32 = 1;
        }
    }
    #[doc = "Capacitor Trim Setting"]
    pub mod S_CTRIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2.5 pF"]
            pub const CTRIM_25: u32 = 0;
            #[doc = "5.0 pF"]
            pub const CTRIM_5: u32 = 1;
            #[doc = "7.5 pF"]
            pub const CTRIM_75: u32 = 2;
            #[doc = "10 pF"]
            pub const CTRIM_10: u32 = 3;
            #[doc = "12.5 pF"]
            pub const CTRIM_125: u32 = 4;
            #[doc = "15.0 pF"]
            pub const CTRIM_15: u32 = 5;
            #[doc = "17.5 pF"]
            pub const CTRIM_175: u32 = 6;
            #[doc = "20 pF"]
            pub const CTRIM_20: u32 = 7;
        }
    }
    #[doc = "Self-Capacitance Sensitivity Boost"]
    pub mod S_SEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Self-Capacitance Discharge Current Multiple"]
    pub mod S_XDN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 / 16"]
            pub const XDN_DEF_16: u32 = 0;
            #[doc = "1 / 8"]
            pub const XDN_DEF_8: u32 = 1;
            #[doc = "1 / 4"]
            pub const XDN_DEF_4: u32 = 2;
            #[doc = "1 / 2"]
            pub const XDN_DEF_2: u32 = 3;
        }
    }
    #[doc = "S_XIN Adjust Ratio"]
    pub mod S_XIN_ADD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables; S_XIN = 0 for 1 / 4, S_XIN = 1 for 1 / 8"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables; S_XIN = 0 for 1 / 8, S_XIN = 1 for 1 / 16"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "TSI CONFIG (TSI_CONFIG) for Mutual-Capacitor"]
pub mod CONFIG_MUTUAL {
    #[doc = "Mode"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Self capacitance"]
            pub const CONFIG_SF_CP: u32 = 0;
            #[doc = "Mutual capacitance"]
            pub const CONFIG_MT_CP: u32 = 1;
        }
    }
    #[doc = "NMOS Current Mirror"]
    pub mod M_NMIRROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "m = 1"]
            pub const M1: u32 = 0;
            #[doc = "m = 2"]
            pub const M2: u32 = 1;
            #[doc = "m = 3"]
            pub const M3: u32 = 2;
            #[doc = "m = 4"]
            pub const M4: u32 = 3;
        }
    }
    #[doc = "PMOS Current Mirror on Right Side"]
    pub mod M_PMIRRORR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "m = 1"]
            pub const MP1: u32 = 0;
            #[doc = "m = 2"]
            pub const MP2: u32 = 1;
            #[doc = "m = 3"]
            pub const MP3: u32 = 2;
            #[doc = "m = 4"]
            pub const MP4: u32 = 3;
        }
    }
    #[doc = "PMOS Current Mirror on Left Side"]
    pub mod M_PMIRRORL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "m = 4"]
            pub const MPL4: u32 = 0;
            #[doc = "m = 8"]
            pub const MPL8: u32 = 1;
            #[doc = "m = 12"]
            pub const MPL12: u32 = 2;
            #[doc = "m = 16"]
            pub const MPL16: u32 = 3;
            #[doc = "m = 20"]
            pub const MPL20: u32 = 4;
            #[doc = "m = 24"]
            pub const MPL24: u32 = 5;
            #[doc = "m = 28"]
            pub const MPL28: u32 = 6;
            #[doc = "m = 32"]
            pub const MPL32: u32 = 7;
        }
    }
    #[doc = "Mutual-Capacitance RX Channel Selection"]
    pub mod M_SEL_RX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TSI[8]"]
            pub const TSI_8: u32 = 0;
            #[doc = "TSI[9]"]
            pub const TSI_9: u32 = 1;
            #[doc = "TSI[10]"]
            pub const TSI_10: u32 = 2;
            #[doc = "TSI[11]"]
            pub const TSI_11: u32 = 3;
            #[doc = "TSI[12]"]
            pub const TSI_12: u32 = 4;
            #[doc = "TSI[13]"]
            pub const TSI_13: u32 = 5;
            #[doc = "TSI[14]"]
            pub const TSI_14: u32 = 6;
            #[doc = "TSI[15]"]
            pub const TSI_15: u32 = 7;
            #[doc = "TSI[16]"]
            pub const TSI_16: u32 = 8;
            #[doc = "TSI[17]"]
            pub const TSI_17: u32 = 9;
            #[doc = "TSI[18]"]
            pub const TSI_18: u32 = 10;
            #[doc = "TSI[19]"]
            pub const TSI_19: u32 = 11;
            #[doc = "TSI[20]"]
            pub const TSI_20: u32 = 12;
            #[doc = "TSI[21]"]
            pub const TSI_21: u32 = 13;
            #[doc = "TSI[22]"]
            pub const TSI_22: u32 = 14;
            #[doc = "TSI[23]"]
            pub const TSI_23: u32 = 15;
            #[doc = "TSI[24]"]
            pub const TSI_24: u32 = 16;
        }
    }
    #[doc = "Mutual-Capacitance TX Channel Selection"]
    pub mod M_SEL_TX {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TSI[0]"]
            pub const TSI_TX_0: u32 = 0;
            #[doc = "TSI[1]"]
            pub const TSI_TX_1: u32 = 1;
            #[doc = "TSI[2]"]
            pub const TSI_TX_2: u32 = 2;
            #[doc = "TSI[3]"]
            pub const TSI_TX_3: u32 = 3;
            #[doc = "TSI[4]"]
            pub const TSI_TX_4: u32 = 4;
            #[doc = "TSI[5]"]
            pub const TSI_TX_5: u32 = 5;
            #[doc = "TSI[6]"]
            pub const TSI_TX_6: u32 = 6;
            #[doc = "TSI[7]"]
            pub const TSI_TX_7: u32 = 7;
        }
    }
    #[doc = "Mutual-Capacitance Counter Enable"]
    pub mod M_CNT_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance TX Pulldown Enable"]
    pub mod M_TX_PD_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance Sensitivity Boost"]
    pub mod M_SEN_BOOST {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 uA"]
            pub const BST_0: u32 = 0;
            #[doc = "2 uA"]
            pub const BST_2: u32 = 1;
            #[doc = "4 uA"]
            pub const BST_4: u32 = 2;
            #[doc = "6 uA"]
            pub const BST_6: u32 = 3;
            #[doc = "8 uA"]
            pub const BST_8: u32 = 4;
            #[doc = "10 uA"]
            pub const BST_10: u32 = 5;
            #[doc = "12 uA"]
            pub const BST_12: u32 = 6;
            #[doc = "14 uA"]
            pub const BST_14: u32 = 7;
            #[doc = "2 * n uA"]
            pub const BST_2N: u32 = 16;
        }
    }
    #[doc = "Mutual-Capacitance Precharge Resistor"]
    pub mod M_PRE_RES {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 k"]
            pub const RES_1: u32 = 0;
            #[doc = "2 k"]
            pub const RES_2: u32 = 1;
            #[doc = "3 k"]
            pub const RES_3: u32 = 2;
            #[doc = "4 k"]
            pub const RES_4: u32 = 3;
            #[doc = "5 k"]
            pub const RES_5: u32 = 4;
            #[doc = "6 k"]
            pub const RES_6: u32 = 5;
            #[doc = "7 k"]
            pub const RES_7: u32 = 6;
            #[doc = "8 k"]
            pub const RES_8: u32 = 7;
        }
    }
    #[doc = "Mutual-Capacitance Precharge Current"]
    pub mod M_PRE_CURRENT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 uA"]
            pub const CUR_1: u32 = 0;
            #[doc = "2 uA"]
            pub const CUR_2: u32 = 1;
            #[doc = "3 uA"]
            pub const CUR_3: u32 = 2;
            #[doc = "4 uA"]
            pub const CUR_4: u32 = 3;
            #[doc = "5 uA"]
            pub const CUR_5: u32 = 4;
            #[doc = "6 uA"]
            pub const CUR_6: u32 = 5;
            #[doc = "7 uA"]
            pub const CUR_7: u32 = 6;
            #[doc = "8 uA"]
            pub const CUR_8: u32 = 7;
        }
    }
}
#[doc = "TSI Threshold"]
pub mod TSHD {
    #[doc = "TSI Wakeup Channel Low Threshold"]
    pub mod THRESL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSI Wakeup Channel High Threshold"]
    pub mod THRESH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI General Control and Status"]
pub mod GENCS {
    #[doc = "In-Progress DMA Transfer Request Enable"]
    pub mod DMAEN_EOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Out-of-Range DMA Transfer Request Enable"]
    pub mod DMAEN_OUTRG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Scan Trigger Mode"]
    pub mod STM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software trigger scan"]
            pub const SWTRIG_SCN: u32 = 0;
            #[doc = "Hardware trigger scan"]
            pub const HWTRIG_SCN: u32 = 1;
        }
    }
    #[doc = "TSI Stop Enable"]
    pub mod STPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "TSI Enable"]
    pub mod TSIEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Trigger Start"]
    pub mod SWTS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Takes effect"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Capacitor Fine Trim"]
    pub mod CTRIM_FINE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.3125 pF"]
            pub const CTRIM_3125: u32 = 0;
            #[doc = "0.625 pF"]
            pub const CTRIM_625: u32 = 1;
            #[doc = "0.3125 * 3 pF"]
            pub const CTRIM_31253: u32 = 2;
            #[doc = "0.3125 * 4 pF"]
            pub const CTRIM_31254: u32 = 3;
            #[doc = "0.3125 * 5 pF"]
            pub const CTRIM_31255: u32 = 4;
            #[doc = "0.3125 * 6 pF"]
            pub const CTRIM_31256: u32 = 5;
            #[doc = "2.1875 pF"]
            pub const CTRIM_1875: u32 = 6;
            #[doc = "2.5 pF"]
            pub const CTRIM_25: u32 = 7;
        }
    }
    #[doc = "Delta Voltage"]
    pub mod DVOLT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vm = 0.6 V, Vp = 1.7 V"]
            pub const VOLT_17: u32 = 0;
            #[doc = "Vm = 0.6 V, Vp = 1.9 V"]
            pub const VOLT_19: u32 = 1;
            #[doc = "Vm = 0.6 V, Vp = 2.1 V"]
            pub const VOLT_21: u32 = 2;
            #[doc = "Vm = 0.6 V, Vp = 2.3 V"]
            pub const VOLT_23: u32 = 3;
            #[doc = "Vm = 0.6 V, Vp = 2.5 V"]
            pub const VOLT_25: u32 = 4;
            #[doc = "Vm = 0.6 V, Vp = 2.7 V"]
            pub const VOLT_27: u32 = 5;
        }
    }
    #[doc = "Debounce"]
    pub mod DEBOUNCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const INT_1: u32 = 0;
            #[doc = "2"]
            pub const INT_2: u32 = 1;
            #[doc = "n"]
            pub const INT_N: u32 = 16;
        }
    }
    #[doc = "Proximity Enable Signal"]
    pub mod S_PROX_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Set Clock"]
    pub mod SETCLK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "27.37 MHz"]
            pub const SETCLK_2737: u32 = 0;
            #[doc = "22.23 MHz"]
            pub const SETCLK_2223: u32 = 1;
            #[doc = "18.73 MHz"]
            pub const SETCLK_1873: u32 = 2;
            #[doc = "16.65 MHz"]
            pub const SETCLK_1616: u32 = 3;
            #[doc = "14.27 MHz"]
            pub const SETCLK_1427: u32 = 4;
            #[doc = "12.73 MHz"]
            pub const SETCLK_1273: u32 = 5;
            #[doc = "11.49 MHz"]
            pub const SETCLK_1149: u32 = 6;
            #[doc = "10.46 MHz"]
            pub const SETCLK_1046: u32 = 7;
        }
    }
    #[doc = "End-of-Scan Interrupt Enable"]
    pub mod ESOR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Out-of-Range Interrupt Enable"]
    pub mod OUTRG_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "TSI Mutual-Capacitance"]
pub mod MUL {
    #[doc = "Mutual-Capacitance Prevoltage"]
    pub mod M_VPRE_CHOOSE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal 1.2 V"]
            pub const INTERNAL: u32 = 0;
            #[doc = "External 1.2 V from PMC"]
            pub const EXTERNAL: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance Mode"]
    pub mod M_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "- 5 V ~ + 5 V"]
            pub const MODE_0: u32 = 0;
            #[doc = "0 V ~ + 5 V"]
            pub const MODE_5: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance Trim Cap"]
    pub mod M_TRIM_CAP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 pF"]
            pub const CP_0: u32 = 0;
            #[doc = "10 pF"]
            pub const CP_10: u32 = 1;
            #[doc = "10 pF"]
            pub const CP_10_1: u32 = 2;
            #[doc = "20 pF"]
            pub const CP_20: u32 = 3;
        }
    }
    #[doc = "Mutual-Capacitance TX Used"]
    pub mod M_TX_USED {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPIO"]
            pub const GPIO: u32 = 0;
            #[doc = "Mutual capacitance"]
            pub const MTCP: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance Trim"]
    pub mod M_TRIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI SINC Filter"]
pub mod SINC {
    #[doc = "SSC Output Control"]
    pub mod SSC_CONTROL_OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "0"]
            pub const DISABLED: u32 = 0;
            #[doc = "1"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SINC Valid"]
    pub mod SINC_VALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SINC Overflow Flag"]
    pub mod SINC_OVERFLOW_FLAG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No overflow"]
            pub const DISABLED: u32 = 0;
            #[doc = "Overflow"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Switch Enable"]
    pub mod SWITCH_ENABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decimation"]
    pub mod DECIMATION {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const DEC_1: u32 = 0;
            #[doc = "2"]
            pub const DEC_2: u32 = 1;
            #[doc = "3"]
            pub const DEC_3: u32 = 2;
            #[doc = "4"]
            pub const DEC_4: u32 = 3;
            #[doc = "5"]
            pub const DEC_5: u32 = 4;
            #[doc = "6"]
            pub const DEC_6: u32 = 5;
            #[doc = "7"]
            pub const DEC_7: u32 = 6;
            #[doc = "8"]
            pub const DEC_8: u32 = 7;
            #[doc = "9"]
            pub const DEC_9: u32 = 8;
            #[doc = "10"]
            pub const DEC_10: u32 = 9;
            #[doc = "11"]
            pub const DEC_11: u32 = 10;
            #[doc = "12"]
            pub const DEC_12: u32 = 11;
            #[doc = "13"]
            pub const DEC_13: u32 = 12;
            #[doc = "14"]
            pub const DEC_14: u32 = 13;
            #[doc = "15"]
            pub const DEC_15: u32 = 14;
            #[doc = "16"]
            pub const DEC_16: u32 = 15;
            #[doc = "17"]
            pub const DEC_17: u32 = 16;
            #[doc = "18"]
            pub const DEC_18: u32 = 17;
            #[doc = "19"]
            pub const DEC_19: u32 = 18;
            #[doc = "20"]
            pub const DEC_20: u32 = 19;
            #[doc = "21"]
            pub const DEC_21: u32 = 20;
            #[doc = "22"]
            pub const DEC_22: u32 = 21;
            #[doc = "23"]
            pub const DEC_23: u32 = 22;
            #[doc = "24"]
            pub const DEC_24: u32 = 23;
            #[doc = "25"]
            pub const DEC_25: u32 = 24;
            #[doc = "26"]
            pub const DEC_26: u32 = 25;
            #[doc = "27"]
            pub const DEC_27: u32 = 26;
            #[doc = "28"]
            pub const DEC_28: u32 = 27;
            #[doc = "29"]
            pub const DEC_29: u32 = 28;
            #[doc = "30"]
            pub const DEC_30: u32 = 29;
            #[doc = "31"]
            pub const DEC_31: u32 = 30;
            #[doc = "32"]
            pub const DEC_32: u32 = 31;
        }
    }
    #[doc = "Order"]
    pub mod ORDER {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Order 1"]
            pub const ORD_1: u32 = 0;
            #[doc = "Order 2"]
            pub const ORD_2: u32 = 1;
        }
    }
    #[doc = "Cutoff"]
    pub mod CUTOFF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "div = 1"]
            pub const DIV_1: u32 = 0;
            #[doc = "div = 2"]
            pub const DIV_2: u32 = 1;
            #[doc = "div = 4"]
            pub const DIV_4: u32 = 2;
            #[doc = "div = 8"]
            pub const DIV_8: u32 = 3;
            #[doc = "div = 16"]
            pub const DIV_16: u32 = 4;
            #[doc = "div = 32"]
            pub const DIV_32: u32 = 5;
            #[doc = "div = 64"]
            pub const DIV_64: u32 = 6;
            #[doc = "div = 128"]
            pub const DIV_128: u32 = 7;
            #[doc = "Do not use"]
            pub const DIV_NC1: u32 = 8;
            #[doc = "Do not use"]
            pub const DIV_NC2: u32 = 9;
            #[doc = "Do not use"]
            pub const DIV_NC3: u32 = 10;
            #[doc = "Do not use"]
            pub const DIV_NC4: u32 = 11;
            #[doc = "Do not use"]
            pub const DIV_NC5: u32 = 12;
            #[doc = "Do not use"]
            pub const DIV_NC6: u32 = 13;
            #[doc = "Do not use"]
            pub const DIV_NC7: u32 = 14;
            #[doc = "Do not use"]
            pub const DIV_NC8: u32 = 15;
        }
    }
}
#[doc = "TSI SSC 0"]
pub mod SSC0 {
    #[doc = "SSC Prescale Number"]
    pub mod SSC_PRESCALE_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "div = 1"]
            pub const DIV1: u32 = 0;
            #[doc = "div = 2"]
            pub const DIV2: u32 = 1;
            #[doc = "div = 4"]
            pub const DIV4: u32 = 3;
            #[doc = "div = 8"]
            pub const DIV8: u32 = 7;
            #[doc = "div = 16"]
            pub const DIV16: u32 = 15;
            #[doc = "div = 32"]
            pub const DIV32: u32 = 31;
            #[doc = "div = 64"]
            pub const DIV64: u32 = 63;
            #[doc = "div = 128"]
            pub const DIV128: u32 = 127;
            #[doc = "div = 256"]
            pub const DIV256: u32 = 255;
        }
    }
    #[doc = "Base Nocharge Number"]
    pub mod BASE_NOCHARGE_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const SSC_1: u32 = 0;
            #[doc = "2"]
            pub const SSC_2: u32 = 1;
            #[doc = "3"]
            pub const SSC_3: u32 = 2;
            #[doc = "4"]
            pub const SSC_4: u32 = 3;
            #[doc = "5"]
            pub const SSC_5: u32 = 4;
            #[doc = "6"]
            pub const SSC_6: u32 = 5;
            #[doc = "7"]
            pub const SSC_7: u32 = 6;
            #[doc = "8"]
            pub const SSC_8: u32 = 7;
            #[doc = "9"]
            pub const SSC_9: u32 = 8;
            #[doc = "10"]
            pub const SSC_10: u32 = 9;
            #[doc = "11"]
            pub const SSC_11: u32 = 10;
            #[doc = "12"]
            pub const SSC_12: u32 = 11;
            #[doc = "13"]
            pub const SSC_13: u32 = 12;
            #[doc = "14"]
            pub const SSC_14: u32 = 13;
            #[doc = "15"]
            pub const SSC_15: u32 = 14;
            #[doc = "16"]
            pub const SSC_16: u32 = 15;
        }
    }
    #[doc = "Charge Number"]
    pub mod CHARGE_NUM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const SSC_1: u32 = 0;
            #[doc = "2"]
            pub const SSC_2: u32 = 1;
            #[doc = "3"]
            pub const SSC_3: u32 = 2;
            #[doc = "4"]
            pub const SSC_4: u32 = 3;
            #[doc = "5"]
            pub const SSC_5: u32 = 4;
            #[doc = "6"]
            pub const SSC_6: u32 = 5;
            #[doc = "7"]
            pub const SSC_7: u32 = 6;
            #[doc = "8"]
            pub const SSC_8: u32 = 7;
            #[doc = "9"]
            pub const SSC_9: u32 = 8;
            #[doc = "10"]
            pub const SSC_10: u32 = 9;
            #[doc = "11"]
            pub const SSC_11: u32 = 10;
            #[doc = "12"]
            pub const SSC_12: u32 = 11;
            #[doc = "13"]
            pub const SSC_13: u32 = 12;
            #[doc = "14"]
            pub const SSC_14: u32 = 13;
            #[doc = "15"]
            pub const SSC_15: u32 = 14;
            #[doc = "16"]
            pub const SSC_16: u32 = 15;
        }
    }
    #[doc = "SSC Control Reverse"]
    pub mod SSC_CONTROL_REVERSE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Polarity retained"]
            pub const ENABLED: u32 = 0;
            #[doc = "Polarity reversed"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "SSC Mode"]
    pub mod SSC_MODE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PRBS mode"]
            pub const PRBS: u32 = 0;
            #[doc = "Up-Down Counter mode"]
            pub const UPDN: u32 = 1;
            #[doc = "Disables SSC function"]
            pub const DISABLED: u32 = 2;
            #[doc = "Do not use"]
            pub const NC: u32 = 3;
        }
    }
    #[doc = "PRBS Output Selection"]
    pub mod PRBS_OUTSEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not use"]
            pub const NC1: u32 = 0;
            #[doc = "Do not use"]
            pub const NC2: u32 = 1;
            #[doc = "2"]
            pub const PRBS_2: u32 = 2;
            #[doc = "3"]
            pub const PRBS_3: u32 = 3;
            #[doc = "4"]
            pub const PRBS_4: u32 = 4;
            #[doc = "5"]
            pub const PRBS_5: u32 = 5;
            #[doc = "6"]
            pub const PRBS_6: u32 = 6;
            #[doc = "7"]
            pub const PRBS_7: u32 = 7;
            #[doc = "8"]
            pub const PRBS_8: u32 = 8;
            #[doc = "9"]
            pub const PRBS_9: u32 = 9;
            #[doc = "10"]
            pub const PRBS_10: u32 = 10;
            #[doc = "11"]
            pub const PRBS_11: u32 = 11;
            #[doc = "12"]
            pub const PRBS_12: u32 = 12;
            #[doc = "13"]
            pub const PRBS_13: u32 = 13;
            #[doc = "14"]
            pub const PRBS_14: u32 = 14;
            #[doc = "15"]
            pub const PRBS_15: u32 = 15;
        }
    }
}
#[doc = "TSI SSC 1"]
pub mod SSC1 {
    #[doc = "PRBS Low Seed"]
    pub mod PRBS_SEED_LO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRBS High Seed"]
    pub mod PRBS_SEED_HI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRBS Low Weight"]
    pub mod PRBS_WEIGHT_LO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRBS High Weight"]
    pub mod PRBS_WEIGHT_HI {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI SSC 2"]
pub mod SSC2 {
    #[doc = "Move Repeat Number"]
    pub mod MOVE_REPEAT_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const UPDN_1: u32 = 0;
            #[doc = "2"]
            pub const UPDN_2: u32 = 1;
            #[doc = "3"]
            pub const UPDN_3: u32 = 2;
            #[doc = "4"]
            pub const UPDN_4: u32 = 3;
            #[doc = "5"]
            pub const UPDN_5: u32 = 4;
            #[doc = "6"]
            pub const UPDN_6: u32 = 5;
            #[doc = "7"]
            pub const UPDN_7: u32 = 6;
        }
    }
    #[doc = "Move Steps Number"]
    pub mod MOVE_STEPS_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const UPDN0: u32 = 0;
            #[doc = "1"]
            pub const UPDN1: u32 = 1;
            #[doc = "2"]
            pub const UPDN2: u32 = 2;
            #[doc = "3"]
            pub const UPDN3: u32 = 3;
            #[doc = "4"]
            pub const UPDN4: u32 = 4;
            #[doc = "5"]
            pub const UPDN5: u32 = 5;
            #[doc = "6"]
            pub const UPDN6: u32 = 6;
            #[doc = "7"]
            pub const UPDN7: u32 = 7;
        }
    }
    #[doc = "Move Nocharge Maximum"]
    pub mod MOVE_NOCHARGE_MAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Move Nocharge Minimum"]
    pub mod MOVE_NOCHARGE_MIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(1 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_1: u32 = 0;
            #[doc = "(2 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_2: u32 = 1;
            #[doc = "(3 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_3: u32 = 2;
            #[doc = "(4 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_4: u32 = 3;
            #[doc = "(5 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_5: u32 = 4;
            #[doc = "(6 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_6: u32 = 5;
            #[doc = "(7 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_7: u32 = 6;
            #[doc = "(8 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_8: u32 = 7;
            #[doc = "(9 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_9: u32 = 8;
            #[doc = "(10 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_10: u32 = 9;
            #[doc = "(11 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_11: u32 = 10;
            #[doc = "(12 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_12: u32 = 11;
            #[doc = "(13 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_13: u32 = 12;
            #[doc = "(14 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_14: u32 = 13;
            #[doc = "(15 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_15: u32 = 14;
            #[doc = "(16 + SSC0[BASE_NOCHARGE_NUM])"]
            pub const MV_16: u32 = 15;
        }
    }
}
#[doc = "TSI Baseline"]
pub mod BASELINE {
    #[doc = "Baseline"]
    pub mod BASELINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base Trace Debounce"]
    pub mod BASE_TRACE_DEBOUNCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0"]
            pub const CTR_0: u32 = 0;
            #[doc = "1 / 16"]
            pub const CTR_1: u32 = 1;
            #[doc = "2 / 16"]
            pub const CTR_2: u32 = 2;
            #[doc = "3 / 16"]
            pub const CTR_3: u32 = 3;
            #[doc = "n / 16"]
            pub const CTR_N: u32 = 8;
        }
    }
    #[doc = "Baseline Trace Enable"]
    pub mod BASE_TRACE_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Threshold Ratio"]
    pub mod THESHOLD_RATIO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "thresholdh = (baseline + counter) / 2 and thresholdl = (baseline - counter) / 2"]
            pub const TSHD_2: u32 = 0;
            #[doc = "thresholdh = (baseline + counter) / 4 and thresholdl = (baseline - counter) / 4"]
            pub const TSHD_4: u32 = 1;
            #[doc = "thresholdh = (baseline + counter) / 8 and thresholdl = (baseline - counter) / 8"]
            pub const TSHD_8: u32 = 2;
            #[doc = "thresholdh = (baseline + counter) / 16 and thresholdl = (baseline - counter) / 16"]
            pub const TSHD_16: u32 = 3;
            #[doc = "thresholdh = (baseline + counter) / 32 and thresholdl = (baseline - counter) / 32"]
            pub const TSHD_32: u32 = 4;
            #[doc = "thresholdh = (baseline + counter) / 64 and thresholdl = (baseline - counter) / 64"]
            pub const TSHD_64: u32 = 5;
            #[doc = "thresholdh = (baseline + counter) / 128 and thresholdl = (baseline - counter) / 128"]
            pub const TSHD_128: u32 = 6;
            #[doc = "thresholdh = (baseline + counter) / 256 and thresholdl = (baseline - counter) / 256"]
            pub const TSHD_256: u32 = 7;
        }
    }
    #[doc = "Threshold Trace Enable"]
    pub mod THRESHOLD_TRACE_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "TSI Channel Merge"]
pub mod CHMERGE {
    #[doc = "Channel Enable"]
    pub mod CHANNEL_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel not chosen for proximity pad"]
            pub const NOTCHOSEN: u32 = 0;
            #[doc = "Channel chosen for proximity pad"]
            pub const CHOSEN: u32 = 1;
        }
    }
}
#[doc = "TSI Shield"]
pub mod SHIELD {
    #[doc = "Shield Enable"]
    pub mod SHIELD_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Mutual-Capacitance Sensitivity Resistor"]
    pub mod M_SEN_RES {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "10 k"]
            pub const RES_10: u32 = 0;
            #[doc = "10 k + (2.5 / 3) k (just for auto-calibration)"]
            pub const RES_253: u32 = 1;
            #[doc = "12.5 k (default)"]
            pub const RES_125: u32 = 2;
            #[doc = "25 k"]
            pub const RES_25: u32 = 14;
        }
    }
}
#[doc = "TSI Data and Status"]
pub mod DATA {
    #[doc = "TSI Conversion Counter Value"]
    pub mod TSICNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End-of-Scan Flag"]
    pub mod EOSF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overrun Flag"]
    pub mod OVERRUNF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const DISABLED: u32 = 0;
            #[doc = "Yes"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Out-of-Range Flag"]
    pub mod OUTRGF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI Miscellaneous"]
pub mod MISC {
    #[doc = "Oscillator Clock Select"]
    pub mod OSC_CLK_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Analog oscillator"]
            pub const OSC_TSI: u32 = 0;
            #[doc = "Chip"]
            pub const OSC_SOC: u32 = 1;
        }
    }
    #[doc = "Test Finger"]
    pub mod TEST_FINGER {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Finger capacitor is 148 pF"]
            pub const FIN_148: u32 = 0;
            #[doc = "Finger capacitor is 296 pF"]
            pub const FIN_296: u32 = 1;
            #[doc = "Finger capacitor is 444 pF"]
            pub const FIN_444: u32 = 2;
            #[doc = "Finger capacitor is 592 pF"]
            pub const FIN_592: u32 = 3;
            #[doc = "Finger capacitor is 740 pF"]
            pub const FIN_740: u32 = 4;
            #[doc = "Finger capacitor is 888 pF"]
            pub const FIN_888: u32 = 5;
            #[doc = "Finger capacitor is 1036 pF"]
            pub const FIN_1036: u32 = 6;
            #[doc = "Finger capacitor is 1184 pF"]
            pub const FIN_1184: u32 = 7;
        }
    }
    #[doc = "Test Finger Function Enable Signals"]
    pub mod TEST_FINGER_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "TSI Clock Divider"]
    pub mod CLKDIVIDER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TSI AUTO TRIG"]
pub mod TRIG {
    #[doc = "Trigger Period Counter"]
    pub mod TRIG_PERIOD_COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Clock Divider"]
    pub mod TRIG_CLK_DIVIDER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No divider"]
            pub const DIV_NO: u32 = 0;
            #[doc = "Divided by 2"]
            pub const DIV_2: u32 = 1;
            #[doc = "Divided by 3"]
            pub const DIV_3: u32 = 2;
            #[doc = "Divided by 4"]
            pub const DIV_4: u32 = 3;
            #[doc = "Divided by n"]
            pub const DIV_N: u32 = 16;
        }
    }
    #[doc = "Trigger Enable"]
    pub mod TRIG_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Trigger Clock Select"]
    pub mod TRIG_CLK_SEL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "32 k clock"]
            pub const CLK_32: u32 = 0;
            #[doc = "clksoc"]
            pub const CLK_SOC: u32 = 1;
        }
    }
}
